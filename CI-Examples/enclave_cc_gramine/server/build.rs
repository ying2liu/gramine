// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

use std::fs::File;
use std::io::{Read, Write};
use ttrpc_codegen::Codegen;
use ttrpc_codegen::Customize;

fn main() {
    let deps_dir="/home/yliu79/zhiwei/image-rs/deps";
     println!("cargo:rustc-link-search=/home/yliu79/zhiwei/server/deps");
     println!("cargo:rustc-link-lib=dylib=sgx_util");

    let protos = vec![
        "protocols/protos/github.com/kata-containers/agent/pkg/types/types.proto",
        "protocols/protos/agent.proto",
        "protocols/protos/health.proto",
        "protocols/protos/google/protobuf/empty.proto",
        "protocols/protos/oci.proto",
        "protocols/protos/image.proto",
    ];

    Codegen::new()
        .out_dir("protocols/sync")
        .inputs(&protos)
        .include("protocols/protos")
        .rust_protobuf()
        .customize(Customize {
            ..Default::default()
        })
        .run()
        .expect("Gen sync code failed.");

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

    // There is a message named 'Box' in oci.proto
    // so there is a struct named 'Box', we should replace Box<Self> to ::std::boxed::Box<Self>
    // to avoid the conflict.
    replace_text_in_file(
        "protocols/sync/oci.rs",
        "self: Box<Self>",
        "self: ::std::boxed::Box<Self>",
    )
    .unwrap();

    replace_text_in_file(
        "protocols/asynchronous/oci.rs",
        "self: Box<Self>",
        "self: ::std::boxed::Box<Self>",
    )
    .unwrap();
}

fn replace_text_in_file(file_name: &str, from: &str, to: &str) -> Result<(), std::io::Error> {
    let mut src = File::open(file_name)?;
    let mut contents = String::new();
    src.read_to_string(&mut contents).unwrap();
    drop(src);

    let new_contents = contents.replace(from, to);

    let mut dst = File::create(&file_name)?;
    dst.write(new_contents.as_bytes())?;

    Ok(())
}
