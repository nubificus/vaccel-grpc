// This file is generated by rust-protobuf 3.3.0. Do not edit
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

//! Generated file from `torch.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:vaccel.TorchTensor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TorchTensor {
    // message fields
    // @@protoc_insertion_point(field:vaccel.TorchTensor.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:vaccel.TorchTensor.dims)
    pub dims: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:vaccel.TorchTensor.type)
    pub type_: ::protobuf::EnumOrUnknown<TorchDataType>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.TorchTensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TorchTensor {
    fn default() -> &'a TorchTensor {
        <TorchTensor as ::protobuf::Message>::default_instance()
    }
}

impl TorchTensor {
    pub fn new() -> TorchTensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &TorchTensor| { &m.data },
            |m: &mut TorchTensor| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "dims",
            |m: &TorchTensor| { &m.dims },
            |m: &mut TorchTensor| { &mut m.dims },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &TorchTensor| { &m.type_ },
            |m: &mut TorchTensor| { &mut m.type_ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TorchTensor>(
            "TorchTensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TorchTensor {
    const NAME: &'static str = "TorchTensor";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.data = is.read_bytes()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.dims)?;
                },
                16 => {
                    self.dims.push(is.read_uint32()?);
                },
                24 => {
                    self.type_ = is.read_enum_or_unknown()?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        for value in &self.dims {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.type_ != ::protobuf::EnumOrUnknown::new(TorchDataType::UNUSED) {
            my_size += ::protobuf::rt::int32_size(3, self.type_.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        }
        for v in &self.dims {
            os.write_uint32(2, *v)?;
        };
        if self.type_ != ::protobuf::EnumOrUnknown::new(TorchDataType::UNUSED) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.type_))?;
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

    fn new() -> TorchTensor {
        TorchTensor::new()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.dims.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(TorchDataType::UNUSED);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TorchTensor {
        static instance: TorchTensor = TorchTensor {
            data: ::std::vec::Vec::new(),
            dims: ::std::vec::Vec::new(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TorchTensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TorchTensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TorchTensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TorchTensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.TorchJitloadForwardRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TorchJitloadForwardRequest {
    // message fields
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardRequest.session_id)
    pub session_id: u32,
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardRequest.model_id)
    pub model_id: i64,
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardRequest.run_options)
    pub run_options: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardRequest.in_tensors)
    pub in_tensors: ::std::vec::Vec<TorchTensor>,
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardRequest.nr_outputs)
    pub nr_outputs: i32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.TorchJitloadForwardRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TorchJitloadForwardRequest {
    fn default() -> &'a TorchJitloadForwardRequest {
        <TorchJitloadForwardRequest as ::protobuf::Message>::default_instance()
    }
}

impl TorchJitloadForwardRequest {
    pub fn new() -> TorchJitloadForwardRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "session_id",
            |m: &TorchJitloadForwardRequest| { &m.session_id },
            |m: &mut TorchJitloadForwardRequest| { &mut m.session_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "model_id",
            |m: &TorchJitloadForwardRequest| { &m.model_id },
            |m: &mut TorchJitloadForwardRequest| { &mut m.model_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "run_options",
            |m: &TorchJitloadForwardRequest| { &m.run_options },
            |m: &mut TorchJitloadForwardRequest| { &mut m.run_options },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "in_tensors",
            |m: &TorchJitloadForwardRequest| { &m.in_tensors },
            |m: &mut TorchJitloadForwardRequest| { &mut m.in_tensors },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nr_outputs",
            |m: &TorchJitloadForwardRequest| { &m.nr_outputs },
            |m: &mut TorchJitloadForwardRequest| { &mut m.nr_outputs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TorchJitloadForwardRequest>(
            "TorchJitloadForwardRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TorchJitloadForwardRequest {
    const NAME: &'static str = "TorchJitloadForwardRequest";

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
                    self.model_id = is.read_int64()?;
                },
                26 => {
                    self.run_options = is.read_bytes()?;
                },
                34 => {
                    self.in_tensors.push(is.read_message()?);
                },
                40 => {
                    self.nr_outputs = is.read_int32()?;
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
        if self.model_id != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.model_id);
        }
        if !self.run_options.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.run_options);
        }
        for value in &self.in_tensors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.nr_outputs != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.nr_outputs);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.session_id != 0 {
            os.write_uint32(1, self.session_id)?;
        }
        if self.model_id != 0 {
            os.write_int64(2, self.model_id)?;
        }
        if !self.run_options.is_empty() {
            os.write_bytes(3, &self.run_options)?;
        }
        for v in &self.in_tensors {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.nr_outputs != 0 {
            os.write_int32(5, self.nr_outputs)?;
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

    fn new() -> TorchJitloadForwardRequest {
        TorchJitloadForwardRequest::new()
    }

    fn clear(&mut self) {
        self.session_id = 0;
        self.model_id = 0;
        self.run_options.clear();
        self.in_tensors.clear();
        self.nr_outputs = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TorchJitloadForwardRequest {
        static instance: TorchJitloadForwardRequest = TorchJitloadForwardRequest {
            session_id: 0,
            model_id: 0,
            run_options: ::std::vec::Vec::new(),
            in_tensors: ::std::vec::Vec::new(),
            nr_outputs: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TorchJitloadForwardRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TorchJitloadForwardRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TorchJitloadForwardRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TorchJitloadForwardRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.TorchJitloadForwardResult)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TorchJitloadForwardResult {
    // message fields
    // @@protoc_insertion_point(field:vaccel.TorchJitloadForwardResult.out_tensors)
    pub out_tensors: ::std::vec::Vec<TorchTensor>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.TorchJitloadForwardResult.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TorchJitloadForwardResult {
    fn default() -> &'a TorchJitloadForwardResult {
        <TorchJitloadForwardResult as ::protobuf::Message>::default_instance()
    }
}

impl TorchJitloadForwardResult {
    pub fn new() -> TorchJitloadForwardResult {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "out_tensors",
            |m: &TorchJitloadForwardResult| { &m.out_tensors },
            |m: &mut TorchJitloadForwardResult| { &mut m.out_tensors },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TorchJitloadForwardResult>(
            "TorchJitloadForwardResult",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TorchJitloadForwardResult {
    const NAME: &'static str = "TorchJitloadForwardResult";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.out_tensors.push(is.read_message()?);
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
        for value in &self.out_tensors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.out_tensors {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TorchJitloadForwardResult {
        TorchJitloadForwardResult::new()
    }

    fn clear(&mut self) {
        self.out_tensors.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TorchJitloadForwardResult {
        static instance: TorchJitloadForwardResult = TorchJitloadForwardResult {
            out_tensors: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TorchJitloadForwardResult {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TorchJitloadForwardResult").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TorchJitloadForwardResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TorchJitloadForwardResult {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.TorchJitloadForwardResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TorchJitloadForwardResponse {
    // message oneof groups
    pub result: ::std::option::Option<torch_jitload_forward_response::Result>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.TorchJitloadForwardResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TorchJitloadForwardResponse {
    fn default() -> &'a TorchJitloadForwardResponse {
        <TorchJitloadForwardResponse as ::protobuf::Message>::default_instance()
    }
}

impl TorchJitloadForwardResponse {
    pub fn new() -> TorchJitloadForwardResponse {
        ::std::default::Default::default()
    }

    // .vaccel.VaccelError error = 1;

    pub fn error(&self) -> &super::error::VaccelError {
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(ref v)) => v,
            _ => <super::error::VaccelError as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_error(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::error::VaccelError) {
        self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut super::error::VaccelError {
        if let ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(super::error::VaccelError::new()));
        }
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error(&mut self) -> super::error::VaccelError {
        if self.has_error() {
            match self.result.take() {
                ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(v)) => v,
                _ => panic!(),
            }
        } else {
            super::error::VaccelError::new()
        }
    }

    // .vaccel.TorchJitloadForwardResult result = 2;

    pub fn result(&self) -> &TorchJitloadForwardResult {
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(ref v)) => v,
            _ => <TorchJitloadForwardResult as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: TorchJitloadForwardResult) {
        self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(v))
    }

    // Mutable pointer to the field.
    pub fn mut_result(&mut self) -> &mut TorchJitloadForwardResult {
        if let ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(_)) = self.result {
        } else {
            self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(TorchJitloadForwardResult::new()));
        }
        match self.result {
            ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_result(&mut self) -> TorchJitloadForwardResult {
        if self.has_result() {
            match self.result.take() {
                ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(v)) => v,
                _ => panic!(),
            }
        } else {
            TorchJitloadForwardResult::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::error::VaccelError>(
            "error",
            TorchJitloadForwardResponse::has_error,
            TorchJitloadForwardResponse::error,
            TorchJitloadForwardResponse::mut_error,
            TorchJitloadForwardResponse::set_error,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, TorchJitloadForwardResult>(
            "result",
            TorchJitloadForwardResponse::has_result,
            TorchJitloadForwardResponse::result,
            TorchJitloadForwardResponse::mut_result,
            TorchJitloadForwardResponse::set_result,
        ));
        oneofs.push(torch_jitload_forward_response::Result::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TorchJitloadForwardResponse>(
            "TorchJitloadForwardResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TorchJitloadForwardResponse {
    const NAME: &'static str = "TorchJitloadForwardResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Error(is.read_message()?));
                },
                18 => {
                    self.result = ::std::option::Option::Some(torch_jitload_forward_response::Result::Result(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &torch_jitload_forward_response::Result::Error(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &torch_jitload_forward_response::Result::Result(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.result {
            match v {
                &torch_jitload_forward_response::Result::Error(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &torch_jitload_forward_response::Result::Result(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
            };
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

    fn new() -> TorchJitloadForwardResponse {
        TorchJitloadForwardResponse::new()
    }

    fn clear(&mut self) {
        self.result = ::std::option::Option::None;
        self.result = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TorchJitloadForwardResponse {
        static instance: TorchJitloadForwardResponse = TorchJitloadForwardResponse {
            result: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TorchJitloadForwardResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TorchJitloadForwardResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TorchJitloadForwardResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TorchJitloadForwardResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TorchJitloadForwardResponse`
pub mod torch_jitload_forward_response {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:vaccel.TorchJitloadForwardResponse.result)
    pub enum Result {
        // @@protoc_insertion_point(oneof_field:vaccel.TorchJitloadForwardResponse.error)
        Error(super::super::error::VaccelError),
        // @@protoc_insertion_point(oneof_field:vaccel.TorchJitloadForwardResponse.result)
        Result(super::TorchJitloadForwardResult),
    }

    impl ::protobuf::Oneof for Result {
    }

    impl ::protobuf::OneofFull for Result {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::TorchJitloadForwardResponse as ::protobuf::MessageFull>::descriptor().oneof_by_name("result").unwrap()).clone()
        }
    }

    impl Result {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Result>("result")
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:vaccel.TorchDataType)
pub enum TorchDataType {
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.UNUSED)
    UNUSED = 0,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.UInt8)
    UInt8 = 1,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.Int8)
    Int8 = 2,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.Int16)
    Int16 = 3,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.Int32)
    Int32 = 4,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.Int64)
    Int64 = 5,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.Half)
    Half = 6,
    // @@protoc_insertion_point(enum_value:vaccel.TorchDataType.FLOAT)
    FLOAT = 7,
}

impl ::protobuf::Enum for TorchDataType {
    const NAME: &'static str = "TorchDataType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TorchDataType> {
        match value {
            0 => ::std::option::Option::Some(TorchDataType::UNUSED),
            1 => ::std::option::Option::Some(TorchDataType::UInt8),
            2 => ::std::option::Option::Some(TorchDataType::Int8),
            3 => ::std::option::Option::Some(TorchDataType::Int16),
            4 => ::std::option::Option::Some(TorchDataType::Int32),
            5 => ::std::option::Option::Some(TorchDataType::Int64),
            6 => ::std::option::Option::Some(TorchDataType::Half),
            7 => ::std::option::Option::Some(TorchDataType::FLOAT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<TorchDataType> {
        match str {
            "UNUSED" => ::std::option::Option::Some(TorchDataType::UNUSED),
            "UInt8" => ::std::option::Option::Some(TorchDataType::UInt8),
            "Int8" => ::std::option::Option::Some(TorchDataType::Int8),
            "Int16" => ::std::option::Option::Some(TorchDataType::Int16),
            "Int32" => ::std::option::Option::Some(TorchDataType::Int32),
            "Int64" => ::std::option::Option::Some(TorchDataType::Int64),
            "Half" => ::std::option::Option::Some(TorchDataType::Half),
            "FLOAT" => ::std::option::Option::Some(TorchDataType::FLOAT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [TorchDataType] = &[
        TorchDataType::UNUSED,
        TorchDataType::UInt8,
        TorchDataType::Int8,
        TorchDataType::Int16,
        TorchDataType::Int32,
        TorchDataType::Int64,
        TorchDataType::Half,
        TorchDataType::FLOAT,
    ];
}

impl ::protobuf::EnumFull for TorchDataType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("TorchDataType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for TorchDataType {
    fn default() -> Self {
        TorchDataType::UNUSED
    }
}

impl TorchDataType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<TorchDataType>("TorchDataType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0btorch.proto\x12\x06vaccel\x1a\x0berror.proto\"`\n\x0bTorchTensor\
    \x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\x12\x12\n\x04dims\x18\
    \x02\x20\x03(\rR\x04dims\x12)\n\x04type\x18\x03\x20\x01(\x0e2\x15.vaccel\
    .TorchDataTypeR\x04type\"\xca\x01\n\x1aTorchJitloadForwardRequest\x12\
    \x1d\n\nsession_id\x18\x01\x20\x01(\rR\tsessionId\x12\x19\n\x08model_id\
    \x18\x02\x20\x01(\x03R\x07modelId\x12\x1f\n\x0brun_options\x18\x03\x20\
    \x01(\x0cR\nrunOptions\x122\n\nin_tensors\x18\x04\x20\x03(\x0b2\x13.vacc\
    el.TorchTensorR\tinTensors\x12\x1d\n\nnr_outputs\x18\x05\x20\x01(\x05R\t\
    nrOutputs\"Q\n\x19TorchJitloadForwardResult\x124\n\x0bout_tensors\x18\
    \x01\x20\x03(\x0b2\x13.vaccel.TorchTensorR\noutTensors\"\x91\x01\n\x1bTo\
    rchJitloadForwardResponse\x12+\n\x05error\x18\x01\x20\x01(\x0b2\x13.vacc\
    el.VaccelErrorH\0R\x05error\x12;\n\x06result\x18\x02\x20\x01(\x0b2!.vacc\
    el.TorchJitloadForwardResultH\0R\x06resultB\x08\n\x06result*f\n\rTorchDa\
    taType\x12\n\n\x06UNUSED\x10\0\x12\t\n\x05UInt8\x10\x01\x12\x08\n\x04Int\
    8\x10\x02\x12\t\n\x05Int16\x10\x03\x12\t\n\x05Int32\x10\x04\x12\t\n\x05I\
    nt64\x10\x05\x12\x08\n\x04Half\x10\x06\x12\t\n\x05FLOAT\x10\x07b\x06prot\
    o3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::error::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(TorchTensor::generated_message_descriptor_data());
            messages.push(TorchJitloadForwardRequest::generated_message_descriptor_data());
            messages.push(TorchJitloadForwardResult::generated_message_descriptor_data());
            messages.push(TorchJitloadForwardResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(TorchDataType::generated_enum_descriptor_data());
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
