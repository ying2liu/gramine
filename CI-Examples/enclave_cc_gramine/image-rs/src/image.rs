// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use oci_spec::image::{ImageConfiguration, Os};
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::bundle::{create_runtime_config, BUNDLE_ROOTFS};
use crate::config::ImageConfig;
use crate::decoder::Compression;
use crate::meta_store::{MetaStore, METAFILE};
use crate::pull::PullClient;

#[cfg(feature = "overlay_feature")]
use crate::snapshots::overlay::OverLay;

#[cfg(feature = "occlum_feature")]
use crate::snapshots::occlum::unionfs::Unionfs;

#[cfg(feature = "gramine_feature")]
use crate::snapshots::gramine::graminefs::Graminefs;


use crate::snapshots::{SnapshotType, Snapshotter};
use crate::validate::security_validate;

/// The metadata info for container image layer.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct LayerMeta {
    /// Image layer compression algorithm type.
    pub decoder: Compression,

    /// Whether image layer is encrypted.
    pub encrypted: bool,

    /// The compressed digest of image layer.
    pub compressed_digest: String,

    /// The uncompressed digest of image layer.
    pub uncompressed_digest: String,

    /// The image layer storage path.
    pub store_path: String,
}

/// The metadata info for container image.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ImageMeta {
    /// The digest of the image configuration.
    pub id: String,

    /// The digest of the image.
    pub digest: String,

    /// The reference string for the image
    pub reference: String,

    /// The image configuration.
    pub image_config: ImageConfiguration,

    /// Whether image is signed.
    pub signed: bool,

    /// The metadata of image layers.
    pub layer_metas: Vec<LayerMeta>,
}

/// The`image-rs` client will support OCI image
/// pulling, image signing verfication, image layer
/// decryption/unpack/store and management.
pub struct ImageClient {
    /// The config for `image-rs` client.
    pub config: ImageConfig,

    /// The metadata database for `image-rs` client.
    pub meta_store: Arc<Mutex<MetaStore>>,

    /// The supported snapshots for `image-rs` client.
    pub snapshots: HashMap<SnapshotType, Box<dyn Snapshotter>>,
}

impl Default for ImageClient {
    // construct a default instance of `ImageClient`
    fn default() -> ImageClient {
        let config = ImageConfig::default();
        let meta_store = MetaStore::try_from(Path::new(METAFILE)).unwrap_or_default();

        let mut snapshots = HashMap::new();

        #[cfg(feature = "overlay_feature")]
        {
            println!("YINGYING overlay_feature");
            let overlay_index = meta_store
                .snapshot_db
                .get(&SnapshotType::Overlay.to_string())
                .unwrap_or(&0);
            let overlay = OverLay {
                data_dir: config.work_dir.join(SnapshotType::Overlay.to_string()),
                index: AtomicUsize::new(*overlay_index),
            };
            snapshots.insert(
                SnapshotType::Overlay,
                Box::new(overlay) as Box<dyn Snapshotter>,
            );
        }

        #[cfg(feature = "occlum_feature")]
        {
             println!("YINGYING occlum_feature");
            let occlum_unionfs_index = meta_store
                .snapshot_db
                .get(&SnapshotType::OcclumUnionfs.to_string())
                .unwrap_or(&0);
            let occlum_unionfs = Unionfs {
                data_dir: config
                    .work_dir
                    .join(SnapshotType::OcclumUnionfs.to_string()),
                index: AtomicUsize::new(*occlum_unionfs_index),
            };
            snapshots.insert(
                SnapshotType::OcclumUnionfs,
                Box::new(occlum_unionfs) as Box<dyn Snapshotter>,
            );
        }

        #[cfg(feature = "gramine_feature")]
        {
             println!("YINGYING gramine_feature");
            let graminefs_index = meta_store
                .snapshot_db
                .get(&SnapshotType::Graminefs.to_string())
                .unwrap_or(&0);
            let graminefs = Graminefs {
                data_dir: config
                    .work_dir
                    .join(SnapshotType::Graminefs.to_string()),
                index: AtomicUsize::new(*graminefs_index),
            };
            snapshots.insert(
                SnapshotType::Graminefs,
                Box::new(graminefs) as Box<dyn Snapshotter>,
            );
        }

 
        ImageClient {
            config,
            meta_store: Arc::new(Mutex::new(meta_store)),
            snapshots,
        }
    }
}

impl ImageClient {
    /// pull_image pulls an image with optional auth info and decrypt config
    /// and store the pulled data under user defined work_dir/layers.
    /// It will return the image ID with prepeared bundle: a rootfs directory,
    /// and config.json will be ready in the bundle_dir passed by user.
    pub async fn pull_image(
        &mut self,
        image_url: &str,
        bundle_dir: &Path,
        auth_info: &Option<&str>,
        decrypt_config: &Option<&str>,
    ) -> Result<String> {
        let mut client =
            PullClient::new(image_url, &self.config.work_dir.join("layers"), auth_info)?;
        let (image_manifest, image_digest, image_config) = client.pull_manifest().await?;

        let id = image_manifest.config.digest.clone();
        if self.meta_store.lock().await.image_db.contains_key(&id) {
            return Ok(id);
        }

        if self.config.security_validate {
            if let Some(wrapped_aa_kbc_params) = decrypt_config {
                let wrapped_aa_kbc_params = wrapped_aa_kbc_params.to_string();
                let aa_kbc_params =
                    wrapped_aa_kbc_params.trim_start_matches("provider:attestation-agent:");

                security_validate(image_url, &image_digest, aa_kbc_params)
                    .await
                    .map_err(|e| anyhow!("Security validate failed: {:?}", e))?;
            } else {
                return Err(anyhow!("Security validation need aa_kbc_params."));
            }
        }

        let mut image_data = ImageMeta {
            id,
            digest: image_digest,
            reference: image_url.to_string(),
            image_config: ImageConfiguration::from_reader(image_config.as_bytes())?,
            ..Default::default()
        };

        let diff_ids = image_data.image_config.rootfs().diff_ids();
        if diff_ids.len() != image_manifest.layers.len() {
            return Err(anyhow!(
                "Pulled number of layers mismatch with image config diff_ids"
            ));
        }

        let layer_metas = client
            .pull_layers(
                image_manifest.layers.clone(),
                diff_ids,
                decrypt_config,
                self.meta_store.clone(),
            )
            .await?;

        image_data.layer_metas = layer_metas;
        let layer_db: HashMap<String, LayerMeta> = image_data
            .layer_metas
            .iter()
            .map(|layer| (layer.compressed_digest.clone(), layer.clone()))
            .collect();

        self.meta_store.lock().await.layer_db.extend(layer_db);

        let layer_path = image_data
            .layer_metas
            .iter()
            .rev()
            .map(|l| l.store_path.as_str())
            .collect::<Vec<&str>>();
             println!("YINGYING client pull_image9");
/*
//YINGYING add encrpt file
        #[link(name = "sgx_util")]
        extern {
            fn pf_encrypt_files(input_dir: *const c_char, output_dir: *const c_char, wrap_key_path: *const c_char)->u32;
            fn pf_init()->u32;
        }

        let mut seal_key: [u8; 16] = [0; 16];
        #[link(name = "pal")]
        extern {
	    fn PalGetSpecialKey(name: *const c_char, key: &mut [u8; 16], key_size: & u128)->u32;	
        }

        let new_rootfs = "/tmp/rootfs";*/

        if let Some(snapshot) = self.snapshots.get_mut(&self.config.default_snapshot) {
            println!("YINGYING client pull_image default_snapshot");
            println!("{}", self.config.default_snapshot);           
 /*            // copy dirs to the specified mount directory
            let mut layer_path_vec = layer_path.to_vec();
            let len = layer_path_vec.len();
            for _i in 0..len {
                println!("YINGYING pull_image10.1\n");
                let layer = layer_path_vec.pop().ok_or(anyhow!("Pop() failed from Vec"))?;
                println!("YINGYING pull_image10.2\n");
                CopyBuilder::new(layer, new_rootfs)
                    .overwrite(true)
                    .run()?;
                }
            fs::set_permissions(new_rootfs, fs::Permissions::from_mode(0o777))?;

            let KEY_PATH = "/dev/attestation/keys/default";
    	    let seal = 1;
	    if seal == 0 {
                let NEW_KEY = "0011223344556677";
               fs::write(KEY_PATH, NEW_KEY).expect("Unable to write key");
            } else {
		unsafe {
		   let cstring0 = CString::new("_sgx_mrsigner").expect("orignal folder failed");
		    PalGetSpecialKey(cstring0.as_ptr(), &mut seal_key, &128);
                    fs::write(KEY_PATH, seal_key).expect("Unable to write key");
		    println!("{:?}", seal_key);
		}

            }

            unsafe {
                let cstring1 = CString::new(new_rootfs).expect("orignal folder failed");
                let cstring2 = CString::new("/enc").expect("destination folder failed");
                let cstring3 = CString::new(KEY_PATH).expect("key failed");
                pf_init();
                pf_encrypt_files (cstring1.as_ptr(), cstring2.as_ptr(), cstring3.as_ptr());
            }
*/
            snapshot.mount(&layer_path, &bundle_dir.join(BUNDLE_ROOTFS))?;
        } else {
            return Err(anyhow!(
                "default snapshot {} not found",
                &self.config.default_snapshot
            ));
        }
        println!("YINGYING client pull_image11");
        let image_config = image_data.image_config.clone();
        println!("YINGYING client pull_image12");
        if image_config.os() != &Os::Linux {
            println!("YINGYING client pull_image13");
            return Err(anyhow!("unsupport OS image {:?}", image_config.os()));
        }
        create_runtime_config(&image_config, bundle_dir)?;
        println!("YINGYING client pull_image14");

        let image_id = image_data.id.clone();
        self.meta_store
            .lock()
            .await
            .image_db
            .insert(image_data.id.clone(), image_data);
        println!("YINGYING client pull_image15");
        Ok(image_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[tokio::test]
    async fn test_pull_image() {
        let work_dir = tempfile::tempdir().unwrap();
        std::env::set_var("CC_IMAGE_WORK_DIR", &work_dir.path());

        // TODO test with more OCI image registries and fix broken registries.
        let oci_images = vec![
            // Alibaba Container Registry
            "registry.cn-hangzhou.aliyuncs.com/acs/busybox:v1.29.2",
            // Amazon Elastic Container Registry
            // "public.ecr.aws/docker/library/hello-world:linux"

            // Azure Container Registry
            "mcr.microsoft.com/hello-world",
            // Docker container Registry
            "docker.io/i386/busybox",
            // Google Container Registry
            "gcr.io/google-containers/busybox:1.27.2",
            // JFrog Container Registry
            // "releases-docker.jfrog.io/reg2/busybox:1.33.1"
        ];

        let mut image_client = ImageClient::default();
        for image in oci_images.iter() {
            let bundle_dir = tempfile::tempdir().unwrap();

            assert!(image_client
                .pull_image(image, bundle_dir.path(), &None, &None)
                .await
                .is_ok());
        }

        assert_eq!(image_client.meta_store.lock().await.image_db.len(), 4);
    }

    #[tokio::test]
    async fn test_pull_signed_image() {
        let work_dir = tempfile::tempdir().unwrap();
        std::env::set_var("CC_IMAGE_WORK_DIR", &work_dir.path());
        let signature_script = format!(
            "{}/scripts/install_test_signatures.sh",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        Command::new(&signature_script)
            .arg("install")
            .output()
            .unwrap();

        let images_can_be_pulled = vec![
            // Test can pull a unencrypted signed image from a protected registry.
            "quay.io/kata-containers/confidential-containers:signed",
            // Test can pull an unencrypted unsigned image from an unprotected registry.
            "quay.io/prometheus/busybox:latest",
        ];

        let images_cannot_be_pulled = vec![
            // Test cannot pull an unencrypted unsigned image from a protected registry.
            "quay.io/kata-containers/confidential-containers:unsigned",
            // Test unencrypted signed image with unknown signature is rejected.
            "quay.io/kata-containers/confidential-containers:other_signed",
        ];

        let mut image_client = ImageClient::default();
        image_client.config.security_validate = true;

        let bundle_dir = tempfile::tempdir().unwrap();

        for image in images_cannot_be_pulled.iter() {
            assert!(image_client
                .pull_image(
                    image,
                    bundle_dir.path(),
                    &None,
                    &Some("provider:attestation-agent:null_kbc::null")
                )
                .await
                .is_err());
        }

        for image in images_can_be_pulled.iter() {
            assert!(image_client
                .pull_image(
                    image,
                    bundle_dir.path(),
                    &None,
                    &Some("provider:attestation-agent:null_kbc::null")
                )
                .await
                .is_ok());
        }

        assert_eq!(image_client.meta_store.lock().await.image_db.len(), 2);

        Command::new(&signature_script)
            .arg("clean")
            .output()
            .unwrap();
    }
}
