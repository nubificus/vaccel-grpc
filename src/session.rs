// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `session.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:vaccel.CreateSessionRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CreateSessionRequest {
    // message fields
    // @@protoc_insertion_point(field:vaccel.CreateSessionRequest.flags)
    pub flags: u32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.CreateSessionRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateSessionRequest {
    fn default() -> &'a CreateSessionRequest {
        <CreateSessionRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateSessionRequest {
    pub fn new() -> CreateSessionRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "flags",
            |m: &CreateSessionRequest| { &m.flags },
            |m: &mut CreateSessionRequest| { &mut m.flags },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateSessionRequest>(
            "CreateSessionRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateSessionRequest {
    const NAME: &'static str = "CreateSessionRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.flags = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.flags != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.flags);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.flags != 0 {
            os.write_uint32(1, self.flags)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CreateSessionRequest {
        CreateSessionRequest::new()
    }

    fn clear(&mut self) {
        self.flags = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateSessionRequest {
        static instance: CreateSessionRequest = CreateSessionRequest {
            flags: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateSessionRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateSessionRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateSessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateSessionRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.UpdateSessionRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateSessionRequest {
    // message fields
    // @@protoc_insertion_point(field:vaccel.UpdateSessionRequest.session_id)
    pub session_id: u32,
    // @@protoc_insertion_point(field:vaccel.UpdateSessionRequest.flags)
    pub flags: u32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.UpdateSessionRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateSessionRequest {
    fn default() -> &'a UpdateSessionRequest {
        <UpdateSessionRequest as ::protobuf::Message>::default_instance()
    }
}

impl UpdateSessionRequest {
    pub fn new() -> UpdateSessionRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "session_id",
            |m: &UpdateSessionRequest| { &m.session_id },
            |m: &mut UpdateSessionRequest| { &mut m.session_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "flags",
            |m: &UpdateSessionRequest| { &m.flags },
            |m: &mut UpdateSessionRequest| { &mut m.flags },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateSessionRequest>(
            "UpdateSessionRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateSessionRequest {
    const NAME: &'static str = "UpdateSessionRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.session_id = is.read_uint32()?;
                },
                16 => {
                    self.flags = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.session_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.session_id);
        }
        if self.flags != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.flags);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.session_id != 0 {
            os.write_uint32(1, self.session_id)?;
        }
        if self.flags != 0 {
            os.write_uint32(2, self.flags)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UpdateSessionRequest {
        UpdateSessionRequest::new()
    }

    fn clear(&mut self) {
        self.session_id = 0;
        self.flags = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateSessionRequest {
        static instance: UpdateSessionRequest = UpdateSessionRequest {
            session_id: 0,
            flags: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateSessionRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateSessionRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateSessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateSessionRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.DestroySessionRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DestroySessionRequest {
    // message fields
    // @@protoc_insertion_point(field:vaccel.DestroySessionRequest.session_id)
    pub session_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.DestroySessionRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DestroySessionRequest {
    fn default() -> &'a DestroySessionRequest {
        <DestroySessionRequest as ::protobuf::Message>::default_instance()
    }
}

impl DestroySessionRequest {
    pub fn new() -> DestroySessionRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "session_id",
            |m: &DestroySessionRequest| { &m.session_id },
            |m: &mut DestroySessionRequest| { &mut m.session_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DestroySessionRequest>(
            "DestroySessionRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DestroySessionRequest {
    const NAME: &'static str = "DestroySessionRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.session_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.session_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.session_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.session_id != 0 {
            os.write_uint32(1, self.session_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DestroySessionRequest {
        DestroySessionRequest::new()
    }

    fn clear(&mut self) {
        self.session_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DestroySessionRequest {
        static instance: DestroySessionRequest = DestroySessionRequest {
            session_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DestroySessionRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DestroySessionRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DestroySessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DestroySessionRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.CreateSessionResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CreateSessionResponse {
    // message fields
    // @@protoc_insertion_point(field:vaccel.CreateSessionResponse.session_id)
    pub session_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.CreateSessionResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateSessionResponse {
    fn default() -> &'a CreateSessionResponse {
        <CreateSessionResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateSessionResponse {
    pub fn new() -> CreateSessionResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "session_id",
            |m: &CreateSessionResponse| { &m.session_id },
            |m: &mut CreateSessionResponse| { &mut m.session_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateSessionResponse>(
            "CreateSessionResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateSessionResponse {
    const NAME: &'static str = "CreateSessionResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.session_id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.session_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.session_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.session_id != 0 {
            os.write_uint32(1, self.session_id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CreateSessionResponse {
        CreateSessionResponse::new()
    }

    fn clear(&mut self) {
        self.session_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateSessionResponse {
        static instance: CreateSessionResponse = CreateSessionResponse {
            session_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateSessionResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateSessionResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateSessionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateSessionResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsession.proto\x12\x06vaccel\",\n\x14CreateSessionRequest\x12\x14\n\
    \x05flags\x18\x01\x20\x01(\rR\x05flags\"K\n\x14UpdateSessionRequest\x12\
    \x1d\n\nsession_id\x18\x01\x20\x01(\rR\tsessionId\x12\x14\n\x05flags\x18\
    \x02\x20\x01(\rR\x05flags\"6\n\x15DestroySessionRequest\x12\x1d\n\nsessi\
    on_id\x18\x01\x20\x01(\rR\tsessionId\"6\n\x15CreateSessionResponse\x12\
    \x1d\n\nsession_id\x18\x01\x20\x01(\rR\tsessionIdb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(CreateSessionRequest::generated_message_descriptor_data());
            messages.push(UpdateSessionRequest::generated_message_descriptor_data());
            messages.push(DestroySessionRequest::generated_message_descriptor_data());
            messages.push(CreateSessionResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
