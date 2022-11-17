// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `health.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct CheckRequest {
    // message fields
    pub service: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CheckRequest {
    fn default() -> &'a CheckRequest {
        <CheckRequest as ::protobuf::Message>::default_instance()
    }
}

impl CheckRequest {
    pub fn new() -> CheckRequest {
        ::std::default::Default::default()
    }

    // string service = 1;


    pub fn get_service(&self) -> &str {
        &self.service
    }
    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CheckRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.service.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.service.is_empty() {
            os.write_string(1, &self.service)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CheckRequest {
        CheckRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    |m: &CheckRequest| { &m.service },
                    |m: &mut CheckRequest| { &mut m.service },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<CheckRequest>(
                    "CheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<CheckRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(CheckRequest::new)
        }
    }
}

impl ::protobuf::Clear for CheckRequest {
    fn clear(&mut self) {
        self.service.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckResponse {
    // message fields
    pub status: HealthCheckResponse_ServingStatus,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HealthCheckResponse {
    fn default() -> &'a HealthCheckResponse {
        <HealthCheckResponse as ::protobuf::Message>::default_instance()
    }
}

impl HealthCheckResponse {
    pub fn new() -> HealthCheckResponse {
        ::std::default::Default::default()
    }

    // .grpc.HealthCheckResponse.ServingStatus status = 1;


    pub fn get_status(&self) -> HealthCheckResponse_ServingStatus {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = HealthCheckResponse_ServingStatus::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: HealthCheckResponse_ServingStatus) {
        self.status = v;
    }
}

impl ::protobuf::Message for HealthCheckResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != HealthCheckResponse_ServingStatus::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.status != HealthCheckResponse_ServingStatus::UNKNOWN {
            os.write_enum(1, self.status.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HealthCheckResponse {
        HealthCheckResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HealthCheckResponse_ServingStatus>>(
                    "status",
                    |m: &HealthCheckResponse| { &m.status },
                    |m: &mut HealthCheckResponse| { &mut m.status },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<HealthCheckResponse>(
                    "HealthCheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HealthCheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(HealthCheckResponse::new)
        }
    }
}

impl ::protobuf::Clear for HealthCheckResponse {
    fn clear(&mut self) {
        self.status = HealthCheckResponse_ServingStatus::UNKNOWN;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HealthCheckResponse_ServingStatus {
    UNKNOWN = 0,
    SERVING = 1,
    NOT_SERVING = 2,
}

impl ::protobuf::ProtobufEnum for HealthCheckResponse_ServingStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HealthCheckResponse_ServingStatus> {
        match value {
            0 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::UNKNOWN),
            1 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::SERVING),
            2 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::NOT_SERVING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HealthCheckResponse_ServingStatus] = &[
            HealthCheckResponse_ServingStatus::UNKNOWN,
            HealthCheckResponse_ServingStatus::SERVING,
            HealthCheckResponse_ServingStatus::NOT_SERVING,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<HealthCheckResponse_ServingStatus>("HealthCheckResponse.ServingStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HealthCheckResponse_ServingStatus {
}

impl ::std::default::Default for HealthCheckResponse_ServingStatus {
    fn default() -> Self {
        HealthCheckResponse_ServingStatus::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckResponse_ServingStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VersionCheckResponse {
    // message fields
    pub grpc_version: ::std::string::String,
    pub agent_version: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VersionCheckResponse {
    fn default() -> &'a VersionCheckResponse {
        <VersionCheckResponse as ::protobuf::Message>::default_instance()
    }
}

impl VersionCheckResponse {
    pub fn new() -> VersionCheckResponse {
        ::std::default::Default::default()
    }

    // string grpc_version = 1;


    pub fn get_grpc_version(&self) -> &str {
        &self.grpc_version
    }
    pub fn clear_grpc_version(&mut self) {
        self.grpc_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_grpc_version(&mut self, v: ::std::string::String) {
        self.grpc_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grpc_version(&mut self) -> &mut ::std::string::String {
        &mut self.grpc_version
    }

    // Take field
    pub fn take_grpc_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.grpc_version, ::std::string::String::new())
    }

    // string agent_version = 2;


    pub fn get_agent_version(&self) -> &str {
        &self.agent_version
    }
    pub fn clear_agent_version(&mut self) {
        self.agent_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_agent_version(&mut self, v: ::std::string::String) {
        self.agent_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_version(&mut self) -> &mut ::std::string::String {
        &mut self.agent_version
    }

    // Take field
    pub fn take_agent_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.agent_version, ::std::string::String::new())
    }
}

impl ::protobuf::Message for VersionCheckResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.grpc_version)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.agent_version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.grpc_version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.grpc_version);
        }
        if !self.agent_version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.agent_version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.grpc_version.is_empty() {
            os.write_string(1, &self.grpc_version)?;
        }
        if !self.agent_version.is_empty() {
            os.write_string(2, &self.agent_version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> VersionCheckResponse {
        VersionCheckResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "grpc_version",
                    |m: &VersionCheckResponse| { &m.grpc_version },
                    |m: &mut VersionCheckResponse| { &mut m.grpc_version },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "agent_version",
                    |m: &VersionCheckResponse| { &m.agent_version },
                    |m: &mut VersionCheckResponse| { &mut m.agent_version },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<VersionCheckResponse>(
                    "VersionCheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static VersionCheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<VersionCheckResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(VersionCheckResponse::new)
        }
    }
}

impl ::protobuf::Clear for VersionCheckResponse {
    fn clear(&mut self) {
        self.grpc_version.clear();
        self.agent_version.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VersionCheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionCheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0chealth.proto\x12\x04grpc\"#\n\x0cCheckRequest\x12\x11\n\x07service\
    \x18\x01\x20\x01(\tB\0:\0\"\x90\x01\n\x13HealthCheckResponse\x129\n\x06s\
    tatus\x18\x01\x20\x01(\x0e2'.grpc.HealthCheckResponse.ServingStatusB\0\"\
    <\n\rServingStatus\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0b\n\x07SERVING\x10\
    \x01\x12\x0f\n\x0bNOT_SERVING\x10\x02\x1a\0:\0\"I\n\x14VersionCheckRespo\
    nse\x12\x16\n\x0cgrpc_version\x18\x01\x20\x01(\tB\0\x12\x17\n\ragent_ver\
    sion\x18\x02\x20\x01(\tB\0:\0B\x10\xf8\xe1\x1e\x01\xc0\xe2\x1e\x01\xa8\
    \xe2\x1e\x01\xb8\xe2\x1e\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
