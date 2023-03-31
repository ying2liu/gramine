// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

use anyhow::{anyhow, Result};
use dircpy::CopyBuilder;
use fs_extra;
use crate::snapshots::{MountPoint, Snapshotter};
use std::str;
use std::os::raw::c_char;
use std::ffi::CString;
use std::os::unix::fs::PermissionsExt;
const GRAMINE_ROOTFS: &str = "/yingtmp/rootfs";
const GRAMINE_ENC: &str = "/enc";
const KEY_PATH: &str = "/dev/attestation/keys/_sgx_mrsigner";
 
#[derive(Debug)]
pub struct Graminefs {
    pub data_dir: PathBuf,
    pub index: AtomicUsize,
}

fn clear_path(mount_path: &Path) -> Result<()> {
    let mut from_paths = Vec::new();
    let paths = fs::read_dir(
        mount_path
            .to_str()
            .ok_or(anyhow!("mount_path does not exist"))?,
    )?;
    for path in paths {
        from_paths.push(path?.path());
    }
    fs_extra::remove_items(&from_paths)?;

    Ok(())
}

/*fn create_dir(create_path: &PathBuf) -> Result<()> {
    if !create_path.exists() {
        fs::create_dir_all(create_path.as_path())?;
    }

    Ok(())
}*/


impl Snapshotter for Graminefs {
    fn mount(&mut self, layer_path: &[&str], mount_path: &Path) -> Result<MountPoint> {
        // From the description of https://github.com/occlum/occlum/blob/master/docs/runtime_mount.md#1-mount-trusted-unionfs-consisting-of-sefss ,
        // the source type of runtime mount is "unionfs".
        let fs_type = String::from("unionfs");

        if !mount_path.exists() {
            fs::create_dir_all(mount_path)?;
        }
 
        let gramine_rootfs = Path::new(GRAMINE_ROOTFS);
        //println!("{}", gramine_rootfs);
        if !gramine_rootfs.exists() {
            fs::create_dir_all(gramine_rootfs)?;
        }
 
        let gramine_enc = Path::new(GRAMINE_ENC);
        //println!("{}", gramine_enc);
        if !gramine_enc.exists() {
            fs::create_dir_all(gramine_enc)?;
        }

        let _key_path = Path::new(KEY_PATH);
        // store the rootfs in different places according to the cid
 /*       let cid = mount_path
            .parent()
            .ok_or(anyhow!("parent do not exist"))?
            .file_name()
            .ok_or(anyhow!("Unknown error: file name parse fail"))?;
        let sefs_base = Path::new("/images").join(cid).join("sefs");
        let unionfs_lowerdir = sefs_base.join("lower");
        let unionfs_upperdir = sefs_base.join("upper");

        // For mounting trusted UnionFS at runtime of occlum,
        // you can refer to https://github.com/occlum/occlum/blob/master/docs/runtime_mount.md#1-mount-trusted-unionfs-consisting-of-sefss.
        // "c7-32-b3-ed-44-df-ec-7b-25-2d-9a-32-38-8d-58-61" is a hardcode key used to encrypt or decrypt the FS currently,
        // and it will be replaced with dynamic key in the near future.
        let options = format!(
            "lowerdir={},upperdir={},key={}",
            unionfs_lowerdir.display(),
            unionfs_upperdir.display(),
            "c7-32-b3-ed-44-df-ec-7b-25-2d-9a-32-38-8d-58-61"
        );

        let flags = MsFlags::empty();

        nix::mount::mount(
            Some(source),
            mount_path,
            Some(fs_type.as_str()),
            flags,
            Some(options.as_str()),
        )
        .map_err(|e| {
            anyhow!(
                "failed to mount {:?} to {:?}, with error: {}",
                source,
                mount_path,
                e
            )
        })?;
*/
        // clear the mount_path if there is something
        clear_path(mount_path)?;

	let gramine_rootfs = "/tmp/roofs";
	let gramine_enc = "/enc";

        #[link(name = "sgx_util")]
        extern {
            fn pf_encrypt_files(input_dir: *const c_char, output_dir: *const c_char, wrap_key_path: *const c_char)->u32;
            fn pf_init()->u32;
        }

        // copy dirs to the specified mount directory
        let mut layer_path_vec = layer_path.to_vec();
        let len = layer_path_vec.len();
        for _i in 0..len {
            let layer = layer_path_vec
                .pop()
                .ok_or(anyhow!("Pop() failed from Vec"))?;
            CopyBuilder::new(layer, /*&mount_path*/ gramine_rootfs).overwrite(true).run()?;
        }
        fs::set_permissions(gramine_rootfs, fs::Permissions::from_mode(0o777))?;
        //as_ref().as_os_str().as_bytes()).unwrap()    
        let key_path = "/dev/attestation/keys/_sgx_mrsigner";
        unsafe {
            let orig_path = CString::new(gramine_rootfs).expect("orignal folder failed");
            let dest_path = CString::new(gramine_enc).expect("destination folder failed");
            let key = CString::new(key_path).expect("key failed");
            pf_init();
            pf_encrypt_files (orig_path.as_ptr(), dest_path.as_ptr(), key.as_ptr());
        }

        Ok(MountPoint {
            r#type: fs_type,
            mount_path: mount_path.to_path_buf(),
            work_dir: self.data_dir.to_path_buf(),
        })
   }

    fn unmount(&self, mount_point: &MountPoint) -> Result<()> {
        nix::mount::umount(mount_point.mount_path.as_path())?;

        Ok(())
    }
}
