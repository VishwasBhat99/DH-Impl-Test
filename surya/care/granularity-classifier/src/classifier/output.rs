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
#![allow(unused_imports)]
#![allow(unused_results)]

#[derive(PartialEq,Clone,Default)]
pub struct Output {
    // message fields
    pub cust_id: ::std::string::String,
    pub count: i64,
    pub out_bal: f64,
    pub limit_bal: f64,
    pub amt_considered: f64,
    pub status: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Output {
    fn default() -> &'a Output {
        <Output as ::protobuf::Message>::default_instance()
    }
}

impl Output {
    pub fn new() -> Output {
        ::std::default::Default::default()
    }

    // string cust_id = 1;


    pub fn get_cust_id(&self) -> &str {
        &self.cust_id
    }
    pub fn clear_cust_id(&mut self) {
        self.cust_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_id(&mut self, v: ::std::string::String) {
        self.cust_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_id(&mut self) -> &mut ::std::string::String {
        &mut self.cust_id
    }

    // Take field
    pub fn take_cust_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_id, ::std::string::String::new())
    }

    // int64 count = 2;


    pub fn get_count(&self) -> i64 {
        self.count
    }
    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = v;
    }

    // double out_bal = 3;


    pub fn get_out_bal(&self) -> f64 {
        self.out_bal
    }
    pub fn clear_out_bal(&mut self) {
        self.out_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_out_bal(&mut self, v: f64) {
        self.out_bal = v;
    }

    // double limit_bal = 4;


    pub fn get_limit_bal(&self) -> f64 {
        self.limit_bal
    }
    pub fn clear_limit_bal(&mut self) {
        self.limit_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_limit_bal(&mut self, v: f64) {
        self.limit_bal = v;
    }

    // double amt_considered = 5;


    pub fn get_amt_considered(&self) -> f64 {
        self.amt_considered
    }
    pub fn clear_amt_considered(&mut self) {
        self.amt_considered = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt_considered(&mut self, v: f64) {
        self.amt_considered = v;
    }

    // string status = 6;


    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ::std::string::String {
        &mut self.status
    }

    // Take field
    pub fn take_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.status, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Output {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.count = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.out_bal = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.limit_bal = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt_considered = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.status)?;
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
        if !self.cust_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cust_id);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.out_bal != 0. {
            my_size += 9;
        }
        if self.limit_bal != 0. {
            my_size += 9;
        }
        if self.amt_considered != 0. {
            my_size += 9;
        }
        if !self.status.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.cust_id.is_empty() {
            os.write_string(1, &self.cust_id)?;
        }
        if self.count != 0 {
            os.write_int64(2, self.count)?;
        }
        if self.out_bal != 0. {
            os.write_double(3, self.out_bal)?;
        }
        if self.limit_bal != 0. {
            os.write_double(4, self.limit_bal)?;
        }
        if self.amt_considered != 0. {
            os.write_double(5, self.amt_considered)?;
        }
        if !self.status.is_empty() {
            os.write_string(6, &self.status)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Output {
        Output::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_id",
                |m: &Output| { &m.cust_id },
                |m: &mut Output| { &mut m.cust_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "count",
                |m: &Output| { &m.count },
                |m: &mut Output| { &mut m.count },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "out_bal",
                |m: &Output| { &m.out_bal },
                |m: &mut Output| { &mut m.out_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "limit_bal",
                |m: &Output| { &m.limit_bal },
                |m: &mut Output| { &mut m.limit_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "amt_considered",
                |m: &Output| { &m.amt_considered },
                |m: &mut Output| { &mut m.amt_considered },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "status",
                |m: &Output| { &m.status },
                |m: &mut Output| { &mut m.status },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Output>(
                "Output",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Output {
        static instance: ::protobuf::rt::LazyV2<Output> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Output::new)
    }
}

impl ::protobuf::Clear for Output {
    fn clear(&mut self) {
        self.cust_id.clear();
        self.count = 0;
        self.out_bal = 0.;
        self.limit_bal = 0.;
        self.amt_considered = 0.;
        self.status.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Output {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Output {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fcare_gran.proto\"\xac\x01\n\x06Output\x12\x17\n\x07cust_id\x18\x01\
    \x20\x01(\tR\x06custId\x12\x14\n\x05count\x18\x02\x20\x01(\x03R\x05count\
    \x12\x17\n\x07out_bal\x18\x03\x20\x01(\x01R\x06outBal\x12\x1b\n\tlimit_b\
    al\x18\x04\x20\x01(\x01R\x08limitBal\x12%\n\x0eamt_considered\x18\x05\
    \x20\x01(\x01R\ramtConsidered\x12\x16\n\x06status\x18\x06\x20\x01(\tR\
    \x06statusb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).expect("Unable to parse ProtoFile Descriptor")
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
