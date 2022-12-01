// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

// use std::fs::File;
// use std::io::{Read, Write};
use ttrpc_codegen::Codegen;
use ttrpc_codegen::Customize;

fn main() {
    let deps_dir="/home/yliu79/zhiwei/image-rs/deps";
     println!("cargo:rustc-link-search=/home/yliu79/zhiwei/image-rs/deps");
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

