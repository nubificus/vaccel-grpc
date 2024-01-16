// This file is generated by ttrpc-compiler 0.6.2. Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]
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
#![allow(clippy::all)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct VaccelAgentClient {
    client: ::ttrpc::Client,
}

impl VaccelAgentClient {
    pub fn new(client: ::ttrpc::Client) -> Self {
        VaccelAgentClient {
            client: client,
        }
    }

    pub fn create_session(&self, ctx: ttrpc::context::Context, req: &super::session::CreateSessionRequest) -> ::ttrpc::Result<super::session::CreateSessionResponse> {
        let mut cres = super::session::CreateSessionResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "CreateSession", cres);
        Ok(cres)
    }

    pub fn destroy_session(&self, ctx: ttrpc::context::Context, req: &super::session::DestroySessionRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        let mut cres = super::agent::VaccelEmpty::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "DestroySession", cres);
        Ok(cres)
    }

    pub fn create_resource(&self, ctx: ttrpc::context::Context, req: &super::resources::CreateResourceRequest) -> ::ttrpc::Result<super::resources::CreateResourceResponse> {
        let mut cres = super::resources::CreateResourceResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "CreateResource", cres);
        Ok(cres)
    }

    pub fn register_resource(&self, ctx: ttrpc::context::Context, req: &super::resources::RegisterResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        let mut cres = super::agent::VaccelEmpty::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "RegisterResource", cres);
        Ok(cres)
    }

    pub fn unregister_resource(&self, ctx: ttrpc::context::Context, req: &super::resources::UnregisterResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        let mut cres = super::agent::VaccelEmpty::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "UnregisterResource", cres);
        Ok(cres)
    }

    pub fn destroy_resource(&self, ctx: ttrpc::context::Context, req: &super::resources::DestroyResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        let mut cres = super::agent::VaccelEmpty::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "DestroyResource", cres);
        Ok(cres)
    }

    pub fn image_classification(&self, ctx: ttrpc::context::Context, req: &super::image::ImageClassificationRequest) -> ::ttrpc::Result<super::image::ImageClassificationResponse> {
        let mut cres = super::image::ImageClassificationResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "ImageClassification", cres);
        Ok(cres)
    }

    pub fn tensorflow_model_load(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowModelLoadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelLoadResponse> {
        let mut cres = super::tensorflow::TensorflowModelLoadResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowModelLoad", cres);
        Ok(cres)
    }

    pub fn tensorflow_model_unload(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowModelUnloadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelUnloadResponse> {
        let mut cres = super::tensorflow::TensorflowModelUnloadResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowModelUnload", cres);
        Ok(cres)
    }

    pub fn tensorflow_model_run(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowModelRunRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelRunResponse> {
        let mut cres = super::tensorflow::TensorflowModelRunResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowModelRun", cres);
        Ok(cres)
    }

    pub fn tensorflow_lite_model_load(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowLiteModelLoadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelLoadResponse> {
        let mut cres = super::tensorflow::TensorflowLiteModelLoadResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowLiteModelLoad", cres);
        Ok(cres)
    }

    pub fn tensorflow_lite_model_unload(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowLiteModelUnloadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelUnloadResponse> {
        let mut cres = super::tensorflow::TensorflowLiteModelUnloadResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowLiteModelUnload", cres);
        Ok(cres)
    }

    pub fn tensorflow_lite_model_run(&self, ctx: ttrpc::context::Context, req: &super::tensorflow::TensorflowLiteModelRunRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelRunResponse> {
        let mut cres = super::tensorflow::TensorflowLiteModelRunResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TensorflowLiteModelRun", cres);
        Ok(cres)
    }

    pub fn torch_jitload_forward(&self, ctx: ttrpc::context::Context, req: &super::torch::TorchJitloadForwardRequest) -> ::ttrpc::Result<super::torch::TorchJitloadForwardResponse> {
        let mut cres = super::torch::TorchJitloadForwardResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "TorchJitloadForward", cres);
        Ok(cres)
    }

    pub fn genop(&self, ctx: ttrpc::context::Context, req: &super::genop::GenopRequest) -> ::ttrpc::Result<super::genop::GenopResponse> {
        let mut cres = super::genop::GenopResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "Genop", cres);
        Ok(cres)
    }

    pub fn get_timers(&self, ctx: ttrpc::context::Context, req: &super::profiling::ProfilingRequest) -> ::ttrpc::Result<super::profiling::ProfilingResponse> {
        let mut cres = super::profiling::ProfilingResponse::new();
        ::ttrpc::client_request!(self, ctx, req, "vaccel.VaccelAgent", "GetTimers", cres);
        Ok(cres)
    }
}

struct CreateSessionMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CreateSessionMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, session, CreateSessionRequest, create_session);
        Ok(())
    }
}

struct DestroySessionMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for DestroySessionMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, session, DestroySessionRequest, destroy_session);
        Ok(())
    }
}

struct CreateResourceMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for CreateResourceMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, resources, CreateResourceRequest, create_resource);
        Ok(())
    }
}

struct RegisterResourceMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for RegisterResourceMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, resources, RegisterResourceRequest, register_resource);
        Ok(())
    }
}

struct UnregisterResourceMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for UnregisterResourceMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, resources, UnregisterResourceRequest, unregister_resource);
        Ok(())
    }
}

struct DestroyResourceMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for DestroyResourceMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, resources, DestroyResourceRequest, destroy_resource);
        Ok(())
    }
}

struct ImageClassificationMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ImageClassificationMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, image, ImageClassificationRequest, image_classification);
        Ok(())
    }
}

struct TensorflowModelLoadMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowModelLoadMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowModelLoadRequest, tensorflow_model_load);
        Ok(())
    }
}

struct TensorflowModelUnloadMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowModelUnloadMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowModelUnloadRequest, tensorflow_model_unload);
        Ok(())
    }
}

struct TensorflowModelRunMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowModelRunMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowModelRunRequest, tensorflow_model_run);
        Ok(())
    }
}

struct TensorflowLiteModelLoadMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowLiteModelLoadMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowLiteModelLoadRequest, tensorflow_lite_model_load);
        Ok(())
    }
}

struct TensorflowLiteModelUnloadMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowLiteModelUnloadMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowLiteModelUnloadRequest, tensorflow_lite_model_unload);
        Ok(())
    }
}

struct TensorflowLiteModelRunMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TensorflowLiteModelRunMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, tensorflow, TensorflowLiteModelRunRequest, tensorflow_lite_model_run);
        Ok(())
    }
}

struct TorchJitloadForwardMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for TorchJitloadForwardMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, torch, TorchJitloadForwardRequest, torch_jitload_forward);
        Ok(())
    }
}

struct GenopMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for GenopMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, genop, GenopRequest, genop);
        Ok(())
    }
}

struct GetTimersMethod {
    service: Arc<Box<dyn VaccelAgent + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for GetTimersMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, profiling, ProfilingRequest, get_timers);
        Ok(())
    }
}

pub trait VaccelAgent {
    fn create_session(&self, _ctx: &::ttrpc::TtrpcContext, _: super::session::CreateSessionRequest) -> ::ttrpc::Result<super::session::CreateSessionResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/CreateSession is not supported".to_string())))
    }
    fn destroy_session(&self, _ctx: &::ttrpc::TtrpcContext, _: super::session::DestroySessionRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/DestroySession is not supported".to_string())))
    }
    fn create_resource(&self, _ctx: &::ttrpc::TtrpcContext, _: super::resources::CreateResourceRequest) -> ::ttrpc::Result<super::resources::CreateResourceResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/CreateResource is not supported".to_string())))
    }
    fn register_resource(&self, _ctx: &::ttrpc::TtrpcContext, _: super::resources::RegisterResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/RegisterResource is not supported".to_string())))
    }
    fn unregister_resource(&self, _ctx: &::ttrpc::TtrpcContext, _: super::resources::UnregisterResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/UnregisterResource is not supported".to_string())))
    }
    fn destroy_resource(&self, _ctx: &::ttrpc::TtrpcContext, _: super::resources::DestroyResourceRequest) -> ::ttrpc::Result<super::agent::VaccelEmpty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/DestroyResource is not supported".to_string())))
    }
    fn image_classification(&self, _ctx: &::ttrpc::TtrpcContext, _: super::image::ImageClassificationRequest) -> ::ttrpc::Result<super::image::ImageClassificationResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/ImageClassification is not supported".to_string())))
    }
    fn tensorflow_model_load(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowModelLoadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelLoadResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowModelLoad is not supported".to_string())))
    }
    fn tensorflow_model_unload(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowModelUnloadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelUnloadResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowModelUnload is not supported".to_string())))
    }
    fn tensorflow_model_run(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowModelRunRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowModelRunResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowModelRun is not supported".to_string())))
    }
    fn tensorflow_lite_model_load(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowLiteModelLoadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelLoadResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowLiteModelLoad is not supported".to_string())))
    }
    fn tensorflow_lite_model_unload(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowLiteModelUnloadRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelUnloadResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowLiteModelUnload is not supported".to_string())))
    }
    fn tensorflow_lite_model_run(&self, _ctx: &::ttrpc::TtrpcContext, _: super::tensorflow::TensorflowLiteModelRunRequest) -> ::ttrpc::Result<super::tensorflow::TensorflowLiteModelRunResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TensorflowLiteModelRun is not supported".to_string())))
    }
    fn torch_jitload_forward(&self, _ctx: &::ttrpc::TtrpcContext, _: super::torch::TorchJitloadForwardRequest) -> ::ttrpc::Result<super::torch::TorchJitloadForwardResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/TorchJitloadForward is not supported".to_string())))
    }
    fn genop(&self, _ctx: &::ttrpc::TtrpcContext, _: super::genop::GenopRequest) -> ::ttrpc::Result<super::genop::GenopResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/Genop is not supported".to_string())))
    }
    fn get_timers(&self, _ctx: &::ttrpc::TtrpcContext, _: super::profiling::ProfilingRequest) -> ::ttrpc::Result<super::profiling::ProfilingResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/vaccel.VaccelAgent/GetTimers is not supported".to_string())))
    }
}

pub fn create_vaccel_agent(service: Arc<Box<dyn VaccelAgent + Send + Sync>>) -> HashMap<String, Box<dyn ::ttrpc::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/vaccel.VaccelAgent/CreateSession".to_string(),
                    Box::new(CreateSessionMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/DestroySession".to_string(),
                    Box::new(DestroySessionMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/CreateResource".to_string(),
                    Box::new(CreateResourceMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/RegisterResource".to_string(),
                    Box::new(RegisterResourceMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/UnregisterResource".to_string(),
                    Box::new(UnregisterResourceMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/DestroyResource".to_string(),
                    Box::new(DestroyResourceMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/ImageClassification".to_string(),
                    Box::new(ImageClassificationMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowModelLoad".to_string(),
                    Box::new(TensorflowModelLoadMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowModelUnload".to_string(),
                    Box::new(TensorflowModelUnloadMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowModelRun".to_string(),
                    Box::new(TensorflowModelRunMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowLiteModelLoad".to_string(),
                    Box::new(TensorflowLiteModelLoadMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowLiteModelUnload".to_string(),
                    Box::new(TensorflowLiteModelUnloadMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TensorflowLiteModelRun".to_string(),
                    Box::new(TensorflowLiteModelRunMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/TorchJitloadForward".to_string(),
                    Box::new(TorchJitloadForwardMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/Genop".to_string(),
                    Box::new(GenopMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/vaccel.VaccelAgent/GetTimers".to_string(),
                    Box::new(GetTimersMethod{service: service.clone()}) as Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods
}
