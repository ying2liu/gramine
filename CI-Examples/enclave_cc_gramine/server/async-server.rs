// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

mod protocols;
mod utils;

extern crate log;

use std::sync::Arc;

use log::LevelFilter;
use clap::{Arg, App};
use protocols::r#async::{image, image_ttrpc};
use ttrpc::asynchronous::Server;

use async_trait::async_trait;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

//use tokio::runtime::Runtime;
use image_rs::image::ImageClient;
use std::path::Path;
use ttrpc::{
    self,
    error::get_rpc_status,
};
use anyhow::Result;


struct ImageService;

impl ImageService {
    async fn pull_image(&self, req: &image::PullImageRequest) -> Result<String> {
        let image = req.get_image();
        //println!("image: {}", image);

        let work_dir = Path::new("/tmp/mnt_target");
        std::env::set_var("CC_IMAGE_WORK_DIR", work_dir);

        let mut image_client = ImageClient::default();
        //let image = "docker.io/huaijin20191223/scratch-base:v1.6";
        //let image = "registry.cn-hangzhou.aliyuncs.com/acs/busybox:v1.29.2";
        let bundle_dir = Path::new("/mnt_image");
        println!("Pullimg image {:?}, bundle path {}", image, bundle_dir.display());

        let image_id = image_client.pull_image(image, bundle_dir, &None, &None).await?;
        //let image_id = "fd244ge34tgfe35g".to_string();

        Ok(image_id)
    }
}
#[async_trait]
impl image_ttrpc::Image for ImageService {
    async fn pull_image(
        &self,
        _ctx: &::ttrpc::r#async::TtrpcContext,
        req: image::PullImageRequest,
    ) -> ttrpc::Result<image::PullImageResponse> {
        match self.pull_image(&req).await {
            Ok(r) => {
                let mut resp = image::PullImageResponse::new();
                resp.image_ref = r;
                return Ok(resp);
            }
            Err(e) => {
                return Err(get_rpc_status(ttrpc::Code::INTERNAL, e.to_string()));
            }
        }
    }
}

//#[tokio::main(flavor = "current_thread")]
#[tokio::main(worker_threads = 1)]
async fn main() {
    simple_logging::log_to_stderr(LevelFilter::Trace);

    let i = Box::new(ImageService {}) as Box<dyn image_ttrpc::Image + Send + Sync>;
    let i = Arc::new(i);
    let iservice = image_ttrpc::create_image(i);

    let mut sockaddr = utils::SOCK_ADDR;

    let matches = App::new("enclave-agent")
                    .author("Enclave-cc Team")
                    .arg(Arg::with_name("listen")
                        .short("l")
                        .long("listen")
                        .value_name("sockaddr")
                        .help("Work in listen mode, tcp://ip_addr:port")
                        .takes_value(true)
                    )
                    .get_matches();

    if matches.is_present("listen") {
        sockaddr = matches.value_of("listen").unwrap();
    }

    println!("Server listen to {}", sockaddr);
    //utils::remove_if_sock_exist(utils::SOCK_ADDR).unwrap();
    let mut server = Server::new()
        .bind(sockaddr)
        .unwrap()
        .register_service(iservice);

    let mut hangup = signal(SignalKind::hangup()).unwrap();
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    server.start().await.unwrap();

    tokio::select! {
        _ = hangup.recv() => {
            // test stop_listen -> start
            println!("stop listen");
            server.stop_listen().await;
            println!("start listen");
            server.start().await.unwrap();

            // hold some time for the new test connection.
            sleep(std::time::Duration::from_secs(100)).await;
        }
        _ = interrupt.recv() => {
            // test graceful shutdown
            println!("graceful shutdown");
            server.shutdown().await.unwrap();
        }
    };
}
