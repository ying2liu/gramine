// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

mod protocols;
mod utils;

use protocols::r#async::{image, image_ttrpc};
use ttrpc::context::{self, Context};
use ttrpc::r#async::Client;
use clap::{Arg, App};
use std::str;
use std::os::raw::{c_char, c_uint};
use std::ffi::CString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;
use std::thread::sleep;
use std::process::{Command, Stdio};


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut sockaddr = utils::SOCK_ADDR;

    let matches = App::new("enclave-agent")
                    .author("Enclave-cc Team")
                    .arg(Arg::with_name("connect")
                        .short("c")
                        .long("connect")
                        .value_name("sockaddr")
                        .help("connect to server,  tcp://ip_addr:port")
                        .takes_value(true)
                    )
                    .get_matches();

    if matches.is_present("connect") {
        sockaddr = matches.value_of("connect").unwrap();
    }

    println!("Client connect to {}", sockaddr);
    let c = Client::connect(sockaddr).unwrap();
    let ic = image_ttrpc::ImageClient::new(c);

    let mut tic = ic.clone();

    let now = std::time::Instant::now();

    let t1 = tokio::spawn(async move {
        let mut req = image::PullImageRequest::new();
        println!(
            "Green Thread 1 - {} started: {:?}",
            "image.pull_image()",
            now.elapsed(),
        );

//        let image_tag = "docker.io/huaijin20191223/ubuntu-base:HelloWorld_v1.1"; //multi-layer
        let image_tag = "docker.io/huaijin20191223/ubuntu-base-enc"; //encrypted
	    //let image_tag = "docker.io/lyingying/yingtest:v1";
        //let image_tag = "docker.io/lyingying/yingtest:v1";
//        let image_tag = "docker.io/huaijin20191223/scratch-base:v1.8"; //one level works
        //let image_tag = "docker.io/huaijin20191223/scratch-base:v1.14";
        //let image_tag = "docker.io/huaijin20191223/ubuntu-base:hello_world_v1.3";
        // image_tag = "docker.io/huaijin20191223/ubuntu-base:hello_world_v1.5";
        //let image_tag = "docker.io/huaijin20191223/ubuntu-base:hello_world_v1.6";
        //let image_tag = "docker.io/huaijin20191223/ubuntu-base:hello_world_v1.8";
        //let image_tag = "docker.io/huaijin20191223/centos-base:hello_world_v1.2";
        //let image_tag = "docker.io/ubuntu";
        //let image_tag = "docker.io/busybox";
        //let cid = "scratch-base_v1.8";
        req.set_image(image_tag.to_string());
        //req.set_container_id(cid.to_string()); 
        println!(
            "Green Thread 1 - {} -> {:?} ended: {:?}",
            "pull_image",
            tic.pull_image(default_ctx(), &req)
                .await,
            now.elapsed(),
        );
        println!("pull_image - {}", image_tag);
   });
   let _ = tokio::join!(t1);

    #[link(name = "sgx_util")]
    extern {
        fn pf_encrypt_files(input_dir: *const c_char, output_dir: *const c_char, wrap_key_path: *const c_char)->u32;
        fn pf_decrypt_files(input_dir: *const c_char, output_dir: *const c_char, Verify_path:bool, wrap_key_path: *const c_char)->u32;
        fn pf_init()->u32;
    }
    #[link(name = "pal")]
    extern {
        fn PalGetSpecialKey(name: *const c_char, key: &mut [u8; 16], key_size: & u128)->u32;
    }

    let mut seal_key: [u8; 16] = [0; 16];
    let KEY_PATH = "/dev/attestation/keys/default";
    unsafe {
        let cstring0 = CString::new("_sgx_mrsigner").expect("orignal folder failed");
        PalGetSpecialKey(cstring0.as_ptr(), &mut seal_key, &128);
        fs::write(KEY_PATH, seal_key).expect("Unable to write key");
        println!("{:?}", seal_key);
    }

    unsafe {
        let cstring1 = CString::new("/enc").expect("enccrypted folder failed");
        let cstring2 = CString::new(KEY_PATH).expect("key failed");
        let cstring3 = CString::new("/dec").expect("decrypted folder failed");
        pf_init();
        pf_decrypt_files (cstring1.as_ptr(), cstring3.as_ptr(), false, cstring2.as_ptr()); 
    }
 
    const APP: &str = "/dec/usr/bin/hello_world";
    fs::set_permissions(APP:&str, fs::Permissions::from_mode(0o777)).unwrap();

    let output = Command::new("/dec/usr/bin/hello_world")
        // Tell the OS to record the command's output
        .stdout(Stdio::inherit())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}

fn default_ctx() -> Context {
    let mut ctx = context::with_timeout(0);
    ctx.add("key-1".to_string(), "value-1-1".to_string());
    ctx.add("key-1".to_string(), "value-1-2".to_string());
    ctx.set("key-2".to_string(), vec!["value-2".to_string()]);

    ctx
}
