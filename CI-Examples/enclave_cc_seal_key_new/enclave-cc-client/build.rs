// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//
use std::env;
use std::path::Path;
use ttrpc_codegen::Codegen;
use ttrpc_codegen::Customize;

fn main() {
    let _dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&_dir).join("deps").display());
    println!("cargo:rustc-link-lib=dylib=sgx_util");
 
    let protos = vec![
        "protocols/protos/image.proto",
    ];

    Codegen::new()
        .out_dir("protocols/asynchronous")
        .inputs(&protos)
        .include("protocols/protos")
        .rust_protobuf()
        .customize(Customize {
            async_all: true,
            ..Default::default()
        })
        .run()
        .expect("Gen async code failed.");
}
