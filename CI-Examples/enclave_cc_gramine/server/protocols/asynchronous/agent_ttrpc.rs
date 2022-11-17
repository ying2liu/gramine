// This file is generated by ttrpc-compiler 0.4.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct AgentServiceClient {
    client: ::ttrpc::r#async::Client,
}

impl AgentServiceClient {
    pub fn new(client: ::ttrpc::r#async::Client) -> Self {
        AgentServiceClient {
            client: client,
        }
    }

    pub async fn create_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::CreateContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "CreateContainer", cres);
    }

    pub async fn start_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::StartContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "StartContainer", cres);
    }

    pub async fn remove_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::RemoveContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "RemoveContainer", cres);
    }

    pub async fn exec_process(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ExecProcess", cres);
    }

    pub async fn signal_process(&mut self, ctx: ttrpc::context::Context, req: &super::agent::SignalProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "SignalProcess", cres);
    }

    pub async fn wait_process(&mut self, ctx: ttrpc::context::Context, req: &super::agent::WaitProcessRequest) -> ::ttrpc::Result<super::agent::WaitProcessResponse> {
        let mut cres = super::agent::WaitProcessResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "WaitProcess", cres);
    }

    pub async fn list_processes(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ListProcessesRequest) -> ::ttrpc::Result<super::agent::ListProcessesResponse> {
        let mut cres = super::agent::ListProcessesResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ListProcesses", cres);
    }

    pub async fn update_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::UpdateContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "UpdateContainer", cres);
    }

    pub async fn stats_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::StatsContainerRequest) -> ::ttrpc::Result<super::agent::StatsContainerResponse> {
        let mut cres = super::agent::StatsContainerResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "StatsContainer", cres);
    }

    pub async fn pause_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::PauseContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "PauseContainer", cres);
    }

    pub async fn resume_container(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ResumeContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ResumeContainer", cres);
    }

    pub async fn write_stdin(&mut self, ctx: ttrpc::context::Context, req: &super::agent::WriteStreamRequest) -> ::ttrpc::Result<super::agent::WriteStreamResponse> {
        let mut cres = super::agent::WriteStreamResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "WriteStdin", cres);
    }

    pub async fn read_stdout(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ReadStreamRequest) -> ::ttrpc::Result<super::agent::ReadStreamResponse> {
        let mut cres = super::agent::ReadStreamResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ReadStdout", cres);
    }

    pub async fn read_stderr(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ReadStreamRequest) -> ::ttrpc::Result<super::agent::ReadStreamResponse> {
        let mut cres = super::agent::ReadStreamResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ReadStderr", cres);
    }

    pub async fn close_stdin(&mut self, ctx: ttrpc::context::Context, req: &super::agent::CloseStdinRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "CloseStdin", cres);
    }

    pub async fn tty_win_resize(&mut self, ctx: ttrpc::context::Context, req: &super::agent::TtyWinResizeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "TtyWinResize", cres);
    }

    pub async fn update_interface(&mut self, ctx: ttrpc::context::Context, req: &super::agent::UpdateInterfaceRequest) -> ::ttrpc::Result<super::types::Interface> {
        let mut cres = super::types::Interface::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "UpdateInterface", cres);
    }

    pub async fn update_routes(&mut self, ctx: ttrpc::context::Context, req: &super::agent::UpdateRoutesRequest) -> ::ttrpc::Result<super::agent::Routes> {
        let mut cres = super::agent::Routes::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "UpdateRoutes", cres);
    }

    pub async fn list_interfaces(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ListInterfacesRequest) -> ::ttrpc::Result<super::agent::Interfaces> {
        let mut cres = super::agent::Interfaces::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ListInterfaces", cres);
    }

    pub async fn list_routes(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ListRoutesRequest) -> ::ttrpc::Result<super::agent::Routes> {
        let mut cres = super::agent::Routes::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ListRoutes", cres);
    }

    pub async fn start_tracing(&mut self, ctx: ttrpc::context::Context, req: &super::agent::StartTracingRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "StartTracing", cres);
    }

    pub async fn stop_tracing(&mut self, ctx: ttrpc::context::Context, req: &super::agent::StopTracingRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "StopTracing", cres);
    }

    pub async fn create_sandbox(&mut self, ctx: ttrpc::context::Context, req: &super::agent::CreateSandboxRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "CreateSandbox", cres);
    }

    pub async fn destroy_sandbox(&mut self, ctx: ttrpc::context::Context, req: &super::agent::DestroySandboxRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "DestroySandbox", cres);
    }

    pub async fn online_cpu_mem(&mut self, ctx: ttrpc::context::Context, req: &super::agent::OnlineCPUMemRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "OnlineCPUMem", cres);
    }

    pub async fn reseed_random_dev(&mut self, ctx: ttrpc::context::Context, req: &super::agent::ReseedRandomDevRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "ReseedRandomDev", cres);
    }

    pub async fn get_guest_details(&mut self, ctx: ttrpc::context::Context, req: &super::agent::GuestDetailsRequest) -> ::ttrpc::Result<super::agent::GuestDetailsResponse> {
        let mut cres = super::agent::GuestDetailsResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "GetGuestDetails", cres);
    }

    pub async fn mem_hotplug_by_probe(&mut self, ctx: ttrpc::context::Context, req: &super::agent::MemHotplugByProbeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "MemHotplugByProbe", cres);
    }

    pub async fn set_guest_date_time(&mut self, ctx: ttrpc::context::Context, req: &super::agent::SetGuestDateTimeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "SetGuestDateTime", cres);
    }

    pub async fn copy_file(&mut self, ctx: ttrpc::context::Context, req: &super::agent::CopyFileRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.AgentService", "CopyFile", cres);
    }
}

struct CreateContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CreateContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, CreateContainerRequest, create_container);
    }
}

struct StartContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StartContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, StartContainerRequest, start_container);
    }
}

struct RemoveContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for RemoveContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, RemoveContainerRequest, remove_container);
    }
}

struct ExecProcessMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ExecProcessMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ExecProcessRequest, exec_process);
    }
}

struct SignalProcessMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for SignalProcessMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, SignalProcessRequest, signal_process);
    }
}

struct WaitProcessMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for WaitProcessMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, WaitProcessRequest, wait_process);
    }
}

struct ListProcessesMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ListProcessesMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ListProcessesRequest, list_processes);
    }
}

struct UpdateContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for UpdateContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, UpdateContainerRequest, update_container);
    }
}

struct StatsContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StatsContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, StatsContainerRequest, stats_container);
    }
}

struct PauseContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for PauseContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, PauseContainerRequest, pause_container);
    }
}

struct ResumeContainerMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ResumeContainerMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ResumeContainerRequest, resume_container);
    }
}

struct WriteStdinMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for WriteStdinMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, WriteStreamRequest, write_stdin);
    }
}

struct ReadStdoutMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ReadStdoutMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ReadStreamRequest, read_stdout);
    }
}

struct ReadStderrMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ReadStderrMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ReadStreamRequest, read_stderr);
    }
}

struct CloseStdinMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CloseStdinMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, CloseStdinRequest, close_stdin);
    }
}

struct TtyWinResizeMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for TtyWinResizeMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, TtyWinResizeRequest, tty_win_resize);
    }
}

struct UpdateInterfaceMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for UpdateInterfaceMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, UpdateInterfaceRequest, update_interface);
    }
}

struct UpdateRoutesMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for UpdateRoutesMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, UpdateRoutesRequest, update_routes);
    }
}

struct ListInterfacesMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ListInterfacesMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ListInterfacesRequest, list_interfaces);
    }
}

struct ListRoutesMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ListRoutesMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ListRoutesRequest, list_routes);
    }
}

struct StartTracingMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StartTracingMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, StartTracingRequest, start_tracing);
    }
}

struct StopTracingMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StopTracingMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, StopTracingRequest, stop_tracing);
    }
}

struct CreateSandboxMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CreateSandboxMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, CreateSandboxRequest, create_sandbox);
    }
}

struct DestroySandboxMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for DestroySandboxMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, DestroySandboxRequest, destroy_sandbox);
    }
}

struct OnlineCpuMemMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for OnlineCpuMemMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, OnlineCPUMemRequest, online_cpu_mem);
    }
}

struct ReseedRandomDevMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ReseedRandomDevMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, ReseedRandomDevRequest, reseed_random_dev);
    }
}

struct GetGuestDetailsMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for GetGuestDetailsMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, GuestDetailsRequest, get_guest_details);
    }
}

struct MemHotplugByProbeMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for MemHotplugByProbeMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, MemHotplugByProbeRequest, mem_hotplug_by_probe);
    }
}

struct SetGuestDateTimeMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for SetGuestDateTimeMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, SetGuestDateTimeRequest, set_guest_date_time);
    }
}

struct CopyFileMethod {
    service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CopyFileMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, agent, CopyFileRequest, copy_file);
    }
}

#[async_trait]
pub trait AgentService: Sync {
    async fn create_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::CreateContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/CreateContainer is not supported".to_string())))
    }
    async fn start_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::StartContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/StartContainer is not supported".to_string())))
    }
    async fn remove_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::RemoveContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/RemoveContainer is not supported".to_string())))
    }
    async fn exec_process(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ExecProcess is not supported".to_string())))
    }
    async fn signal_process(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::SignalProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/SignalProcess is not supported".to_string())))
    }
    async fn wait_process(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::WaitProcessRequest) -> ::ttrpc::Result<super::agent::WaitProcessResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/WaitProcess is not supported".to_string())))
    }
    async fn list_processes(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ListProcessesRequest) -> ::ttrpc::Result<super::agent::ListProcessesResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ListProcesses is not supported".to_string())))
    }
    async fn update_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::UpdateContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/UpdateContainer is not supported".to_string())))
    }
    async fn stats_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::StatsContainerRequest) -> ::ttrpc::Result<super::agent::StatsContainerResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/StatsContainer is not supported".to_string())))
    }
    async fn pause_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::PauseContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/PauseContainer is not supported".to_string())))
    }
    async fn resume_container(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ResumeContainerRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ResumeContainer is not supported".to_string())))
    }
    async fn write_stdin(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::WriteStreamRequest) -> ::ttrpc::Result<super::agent::WriteStreamResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/WriteStdin is not supported".to_string())))
    }
    async fn read_stdout(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ReadStreamRequest) -> ::ttrpc::Result<super::agent::ReadStreamResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ReadStdout is not supported".to_string())))
    }
    async fn read_stderr(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ReadStreamRequest) -> ::ttrpc::Result<super::agent::ReadStreamResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ReadStderr is not supported".to_string())))
    }
    async fn close_stdin(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::CloseStdinRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/CloseStdin is not supported".to_string())))
    }
    async fn tty_win_resize(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::TtyWinResizeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/TtyWinResize is not supported".to_string())))
    }
    async fn update_interface(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::UpdateInterfaceRequest) -> ::ttrpc::Result<super::types::Interface> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/UpdateInterface is not supported".to_string())))
    }
    async fn update_routes(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::UpdateRoutesRequest) -> ::ttrpc::Result<super::agent::Routes> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/UpdateRoutes is not supported".to_string())))
    }
    async fn list_interfaces(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ListInterfacesRequest) -> ::ttrpc::Result<super::agent::Interfaces> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ListInterfaces is not supported".to_string())))
    }
    async fn list_routes(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ListRoutesRequest) -> ::ttrpc::Result<super::agent::Routes> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ListRoutes is not supported".to_string())))
    }
    async fn start_tracing(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::StartTracingRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/StartTracing is not supported".to_string())))
    }
    async fn stop_tracing(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::StopTracingRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/StopTracing is not supported".to_string())))
    }
    async fn create_sandbox(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::CreateSandboxRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/CreateSandbox is not supported".to_string())))
    }
    async fn destroy_sandbox(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::DestroySandboxRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/DestroySandbox is not supported".to_string())))
    }
    async fn online_cpu_mem(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::OnlineCPUMemRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/OnlineCPUMem is not supported".to_string())))
    }
    async fn reseed_random_dev(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::ReseedRandomDevRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/ReseedRandomDev is not supported".to_string())))
    }
    async fn get_guest_details(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::GuestDetailsRequest) -> ::ttrpc::Result<super::agent::GuestDetailsResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/GetGuestDetails is not supported".to_string())))
    }
    async fn mem_hotplug_by_probe(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::MemHotplugByProbeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/MemHotplugByProbe is not supported".to_string())))
    }
    async fn set_guest_date_time(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::SetGuestDateTimeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/SetGuestDateTime is not supported".to_string())))
    }
    async fn copy_file(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::agent::CopyFileRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.AgentService/CopyFile is not supported".to_string())))
    }
}

pub fn create_agent_service(service: Arc<std::boxed::Box<dyn AgentService + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/grpc.AgentService/CreateContainer".to_string(),
                    std::boxed::Box::new(CreateContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/StartContainer".to_string(),
                    std::boxed::Box::new(StartContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/RemoveContainer".to_string(),
                    std::boxed::Box::new(RemoveContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ExecProcess".to_string(),
                    std::boxed::Box::new(ExecProcessMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/SignalProcess".to_string(),
                    std::boxed::Box::new(SignalProcessMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/WaitProcess".to_string(),
                    std::boxed::Box::new(WaitProcessMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ListProcesses".to_string(),
                    std::boxed::Box::new(ListProcessesMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/UpdateContainer".to_string(),
                    std::boxed::Box::new(UpdateContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/StatsContainer".to_string(),
                    std::boxed::Box::new(StatsContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/PauseContainer".to_string(),
                    std::boxed::Box::new(PauseContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ResumeContainer".to_string(),
                    std::boxed::Box::new(ResumeContainerMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/WriteStdin".to_string(),
                    std::boxed::Box::new(WriteStdinMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ReadStdout".to_string(),
                    std::boxed::Box::new(ReadStdoutMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ReadStderr".to_string(),
                    std::boxed::Box::new(ReadStderrMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/CloseStdin".to_string(),
                    std::boxed::Box::new(CloseStdinMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/TtyWinResize".to_string(),
                    std::boxed::Box::new(TtyWinResizeMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/UpdateInterface".to_string(),
                    std::boxed::Box::new(UpdateInterfaceMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/UpdateRoutes".to_string(),
                    std::boxed::Box::new(UpdateRoutesMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ListInterfaces".to_string(),
                    std::boxed::Box::new(ListInterfacesMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ListRoutes".to_string(),
                    std::boxed::Box::new(ListRoutesMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/StartTracing".to_string(),
                    std::boxed::Box::new(StartTracingMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/StopTracing".to_string(),
                    std::boxed::Box::new(StopTracingMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/CreateSandbox".to_string(),
                    std::boxed::Box::new(CreateSandboxMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/DestroySandbox".to_string(),
                    std::boxed::Box::new(DestroySandboxMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/OnlineCPUMem".to_string(),
                    std::boxed::Box::new(OnlineCpuMemMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/ReseedRandomDev".to_string(),
                    std::boxed::Box::new(ReseedRandomDevMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/GetGuestDetails".to_string(),
                    std::boxed::Box::new(GetGuestDetailsMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/MemHotplugByProbe".to_string(),
                    std::boxed::Box::new(MemHotplugByProbeMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/SetGuestDateTime".to_string(),
                    std::boxed::Box::new(SetGuestDateTimeMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/grpc.AgentService/CopyFile".to_string(),
                    std::boxed::Box::new(CopyFileMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods
}
