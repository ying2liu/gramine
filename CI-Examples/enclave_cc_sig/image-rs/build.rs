// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

fn main() -> shadow_rs::SdResult<()> {
    let deps_dir="/home/yliu79/zhiwei/image-rs/deps";
    println!("cargo:rustc-link-search=/home/yliu79/zhiwei/image-rs/deps");
    println!("cargo:rustc-link-lib=dylib=sgx_util");
    println!("cargo:rustc-link-lib=dylib=pal");
 
    tonic_build::compile_protos("./protos/getresource.proto")?;

    shadow_rs::new()
}
