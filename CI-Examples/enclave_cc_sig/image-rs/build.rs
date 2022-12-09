// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//
use std::env;
use std::path::Path;

fn main() -> shadow_rs::SdResult<()> {
    let _dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&_dir).join("deps").display());
    println!("cargo:rustc-link-lib=dylib=sgx_util");
    println!("cargo:rustc-link-lib=dylib=pal");
 
    tonic_build::compile_protos("./protos/getresource.proto")?;

    shadow_rs::new()
}
