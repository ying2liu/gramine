// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

use std::str;
//use std::{env, error::Error, fs};
//use std::os::unix::fs::PermissionsExt;
use std::process::Command;
//use execute::Execute;

fn main() {
   println!("entrying");
/*    //YINGYING add encrpt file
    #[link(name = "sgx_util")]
        extern {
           fn pf_encrypt_files(input_dir: *const c_char, output_dir: *const c_char, wrap_key_path: *const c_char)->u32;
            fn pf_decrypt_files(input_dir: *const c_char, output_dir: *const c_char, Verify_path:bool, wrap_key_path: *const c_char)->u32;
            fn pf_init()->u32;
        }

//                 fs::set_permissions(new_rootfs, fs::Permissions::from_mode(0o777))?;

            let NEW_KEY = "0011223344556677";
            let KEY_PATH = "/dev/attestation/keys/default";
            fs::write(KEY_PATH, NEW_KEY).expect("Unable to write key");

            unsafe {
                let cstring1 = CString::new("/enc").expect("enccrypted folder failed");
                let cstring2 = CString::new(KEY_PATH).expect("key failed");
                let cstring3 = CString::new("/dec").expect("decrypted folder failed");
                pf_init();
                pf_decrypt_files (cstring1.as_ptr(), cstring3.as_ptr(), false, cstring2.as_ptr()); 
            }
*/
//     let input_path = "/dec/bin/hello_world";
/*     let input = fs::read(&input_path)?;

    println!("Analyzing {:?}...", input_path);

/*    let file = match delf::File::parse_or_print_error(&input[..]) {
        Some(f) => f,
        None => std::process::exit(1),
    };*/
    //println!("{:#?}", file);
    println!("Executing {:?}...", input_path);
    use std::process::Command;
    let status = Command::new(input_path).status()?;
    if !status.success() {
 //       return Err("process did not exit successfully".into());
    } else {
        println!("running hello_world successfully");
    } 
 */
    const FFMPEG_PATH: &str = "./hello_world";

    let mut cmd = Command::new(FFMPEG_PATH);
    match cmd.output() {
        Ok(o)  => {
            unsafe {
                println! ("Output:  {}", String::from_utf8_unchecked(o.stdout));
            }
        }

        Err(e) => {
            println!("There was an error {}", e);
        }
    }

   // command.arg("-i");
   // command.arg("/path/to/media-file");
    //command.arg("/path/to/output-file");

    /*
    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
         }
    } else {
        eprintln!("Interrupted!");
    }
*/
}


