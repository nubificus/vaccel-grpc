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

//! Generated file from `profiling.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:vaccel.ProfilingRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProfilingRequest {
    // message fields
    // @@protoc_insertion_point(field:vaccel.ProfilingRequest.session_id)
    pub session_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.ProfilingRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProfilingRequest {
    fn default() -> &'a ProfilingRequest {
        <ProfilingRequest as ::protobuf::Message>::default_instance()
    }
}

impl ProfilingRequest {
    pub fn new() -> ProfilingRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "session_id",
            |m: &ProfilingRequest| { &m.session_id },
            |m: &mut ProfilingRequest| { &mut m.session_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProfilingRequest>(
            "ProfilingRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProfilingRequest {
    const NAME: &'static str = "ProfilingRequest";

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

    fn new() -> ProfilingRequest {
        ProfilingRequest::new()
    }

    fn clear(&mut self) {
        self.session_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProfilingRequest {
        static instance: ProfilingRequest = ProfilingRequest {
            session_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProfilingRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProfilingRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProfilingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProfilingRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.ProfRegion)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProfRegion {
    // message fields
    // @@protoc_insertion_point(field:vaccel.ProfRegion.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:vaccel.ProfRegion.samples)
    pub samples: ::std::vec::Vec<prof_region::Sample>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.ProfRegion.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProfRegion {
    fn default() -> &'a ProfRegion {
        <ProfRegion as ::protobuf::Message>::default_instance()
    }
}

impl ProfRegion {
    pub fn new() -> ProfRegion {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &ProfRegion| { &m.name },
            |m: &mut ProfRegion| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "samples",
            |m: &ProfRegion| { &m.samples },
            |m: &mut ProfRegion| { &mut m.samples },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProfRegion>(
            "ProfRegion",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProfRegion {
    const NAME: &'static str = "ProfRegion";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    self.samples.push(is.read_message()?);
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.samples {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.samples {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> ProfRegion {
        ProfRegion::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.samples.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProfRegion {
        static instance: ProfRegion = ProfRegion {
            name: ::std::string::String::new(),
            samples: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProfRegion {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProfRegion").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProfRegion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProfRegion {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ProfRegion`
pub mod prof_region {
    // @@protoc_insertion_point(message:vaccel.ProfRegion.Sample)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Sample {
        // message fields
        // @@protoc_insertion_point(field:vaccel.ProfRegion.Sample.start)
        pub start: u64,
        // @@protoc_insertion_point(field:vaccel.ProfRegion.Sample.time)
        pub time: u64,
        // special fields
        // @@protoc_insertion_point(special_field:vaccel.ProfRegion.Sample.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Sample {
        fn default() -> &'a Sample {
            <Sample as ::protobuf::Message>::default_instance()
        }
    }

    impl Sample {
        pub fn new() -> Sample {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "start",
                |m: &Sample| { &m.start },
                |m: &mut Sample| { &mut m.start },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "time",
                |m: &Sample| { &m.time },
                |m: &mut Sample| { &mut m.time },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Sample>(
                "ProfRegion.Sample",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Sample {
        const NAME: &'static str = "Sample";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.start = is.read_uint64()?;
                    },
                    16 => {
                        self.time = is.read_uint64()?;
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
            if self.start != 0 {
                my_size += ::protobuf::rt::uint64_size(1, self.start);
            }
            if self.time != 0 {
                my_size += ::protobuf::rt::uint64_size(2, self.time);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.start != 0 {
                os.write_uint64(1, self.start)?;
            }
            if self.time != 0 {
                os.write_uint64(2, self.time)?;
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

        fn new() -> Sample {
            Sample::new()
        }

        fn clear(&mut self) {
            self.start = 0;
            self.time = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Sample {
            static instance: Sample = Sample {
                start: 0,
                time: 0,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Sample {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("ProfRegion.Sample").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Sample {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Sample {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

// @@protoc_insertion_point(message:vaccel.ProfRegions)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProfRegions {
    // message fields
    // @@protoc_insertion_point(field:vaccel.ProfRegions.timer)
    pub timer: ::std::vec::Vec<ProfRegion>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.ProfRegions.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProfRegions {
    fn default() -> &'a ProfRegions {
        <ProfRegions as ::protobuf::Message>::default_instance()
    }
}

impl ProfRegions {
    pub fn new() -> ProfRegions {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "timer",
            |m: &ProfRegions| { &m.timer },
            |m: &mut ProfRegions| { &mut m.timer },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProfRegions>(
            "ProfRegions",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProfRegions {
    const NAME: &'static str = "ProfRegions";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.timer.push(is.read_message()?);
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
        for value in &self.timer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.timer {
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

    fn new() -> ProfRegions {
        ProfRegions::new()
    }

    fn clear(&mut self) {
        self.timer.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProfRegions {
        static instance: ProfRegions = ProfRegions {
            timer: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProfRegions {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProfRegions").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProfRegions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProfRegions {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:vaccel.ProfilingResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProfilingResponse {
    // message fields
    // @@protoc_insertion_point(field:vaccel.ProfilingResponse.result)
    pub result: ::protobuf::MessageField<ProfRegions>,
    // special fields
    // @@protoc_insertion_point(special_field:vaccel.ProfilingResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProfilingResponse {
    fn default() -> &'a ProfilingResponse {
        <ProfilingResponse as ::protobuf::Message>::default_instance()
    }
}

impl ProfilingResponse {
    pub fn new() -> ProfilingResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ProfRegions>(
            "result",
            |m: &ProfilingResponse| { &m.result },
            |m: &mut ProfilingResponse| { &mut m.result },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProfilingResponse>(
            "ProfilingResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProfilingResponse {
    const NAME: &'static str = "ProfilingResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.result)?;
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
        if let Some(v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.result.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> ProfilingResponse {
        ProfilingResponse::new()
    }

    fn clear(&mut self) {
        self.result.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProfilingResponse {
        static instance: ProfilingResponse = ProfilingResponse {
            result: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProfilingResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProfilingResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProfilingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProfilingResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fprofiling.proto\x12\x06vaccel\x1a\x0berror.proto\"1\n\x10Profiling\
    Request\x12\x1d\n\nsession_id\x18\x01\x20\x01(\rR\tsessionId\"\x89\x01\n\
    \nProfRegion\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x123\n\x07sam\
    ples\x18\x02\x20\x03(\x0b2\x19.vaccel.ProfRegion.SampleR\x07samples\x1a2\
    \n\x06Sample\x12\x14\n\x05start\x18\x01\x20\x01(\x04R\x05start\x12\x12\n\
    \x04time\x18\x02\x20\x01(\x04R\x04time\"7\n\x0bProfRegions\x12(\n\x05tim\
    er\x18\x01\x20\x03(\x0b2\x12.vaccel.ProfRegionR\x05timer\"@\n\x11Profili\
    ngResponse\x12+\n\x06result\x18\x01\x20\x01(\x0b2\x13.vaccel.ProfRegions\
    R\x06resultb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(ProfilingRequest::generated_message_descriptor_data());
            messages.push(ProfRegion::generated_message_descriptor_data());
            messages.push(ProfRegions::generated_message_descriptor_data());
            messages.push(ProfilingResponse::generated_message_descriptor_data());
            messages.push(prof_region::Sample::generated_message_descriptor_data());
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
