// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Output {
    // message fields
    pub cust_id: ::std::string::String,
    pub count: i64,
    pub tot_bal: f64,
    pub exp_status: ::std::string::String,
    pub limit_bal: f64,
    pub limit_status: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Output {
    pub fn new() -> Output {
        ::std::default::Default::default()
    }

    // string cust_id = 1;

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

    pub fn get_cust_id(&self) -> &str {
        &self.cust_id
    }

    // int64 count = 2;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = v;
    }

    pub fn get_count(&self) -> i64 {
        self.count
    }

    // double tot_bal = 3;

    pub fn clear_tot_bal(&mut self) {
        self.tot_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tot_bal(&mut self, v: f64) {
        self.tot_bal = v;
    }

    pub fn get_tot_bal(&self) -> f64 {
        self.tot_bal
    }

    // string exp_status = 4;

    pub fn clear_exp_status(&mut self) {
        self.exp_status.clear();
    }

    // Param is passed by value, moved
    pub fn set_exp_status(&mut self, v: ::std::string::String) {
        self.exp_status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exp_status(&mut self) -> &mut ::std::string::String {
        &mut self.exp_status
    }

    // Take field
    pub fn take_exp_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.exp_status, ::std::string::String::new())
    }

    pub fn get_exp_status(&self) -> &str {
        &self.exp_status
    }

    // double limit_bal = 5;

    pub fn clear_limit_bal(&mut self) {
        self.limit_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_limit_bal(&mut self, v: f64) {
        self.limit_bal = v;
    }

    pub fn get_limit_bal(&self) -> f64 {
        self.limit_bal
    }

    // string limit_status = 6;

    pub fn clear_limit_status(&mut self) {
        self.limit_status.clear();
    }

    // Param is passed by value, moved
    pub fn set_limit_status(&mut self, v: ::std::string::String) {
        self.limit_status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_limit_status(&mut self) -> &mut ::std::string::String {
        &mut self.limit_status
    }

    // Take field
    pub fn take_limit_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.limit_status, ::std::string::String::new())
    }

    pub fn get_limit_status(&self) -> &str {
        &self.limit_status
    }
}

impl ::protobuf::Message for Output {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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
                    self.tot_bal = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.exp_status)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.limit_bal = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.limit_status)?;
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
        if self.tot_bal != 0. {
            my_size += 9;
        }
        if !self.exp_status.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.exp_status);
        }
        if self.limit_bal != 0. {
            my_size += 9;
        }
        if !self.limit_status.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.limit_status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cust_id.is_empty() {
            os.write_string(1, &self.cust_id)?;
        }
        if self.count != 0 {
            os.write_int64(2, self.count)?;
        }
        if self.tot_bal != 0. {
            os.write_double(3, self.tot_bal)?;
        }
        if !self.exp_status.is_empty() {
            os.write_string(4, &self.exp_status)?;
        }
        if self.limit_bal != 0. {
            os.write_double(5, self.limit_bal)?;
        }
        if !self.limit_status.is_empty() {
            os.write_string(6, &self.limit_status)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Output {
        Output::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
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
                    "tot_bal",
                    |m: &Output| { &m.tot_bal },
                    |m: &mut Output| { &mut m.tot_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "exp_status",
                    |m: &Output| { &m.exp_status },
                    |m: &mut Output| { &mut m.exp_status },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "limit_bal",
                    |m: &Output| { &m.limit_bal },
                    |m: &mut Output| { &mut m.limit_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "limit_status",
                    |m: &Output| { &m.limit_status },
                    |m: &mut Output| { &mut m.limit_status },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Output>(
                    "Output",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Output {
        static mut instance: ::protobuf::lazy::Lazy<Output> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Output,
        };
        unsafe {
            instance.get(Output::new)
        }
    }
}

impl ::protobuf::Clear for Output {
    fn clear(&mut self) {
        self.clear_cust_id();
        self.clear_count();
        self.clear_tot_bal();
        self.clear_exp_status();
        self.clear_limit_bal();
        self.clear_limit_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Output {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Output {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17proto/tot_bal_gen.proto\"\xaf\x01\n\x06Output\x12\x17\n\x07cust_id\
    \x18\x01\x20\x01(\tR\x06custId\x12\x14\n\x05count\x18\x02\x20\x01(\x03R\
    \x05count\x12\x17\n\x07tot_bal\x18\x03\x20\x01(\x01R\x06totBal\x12\x1d\n\
    \nexp_status\x18\x04\x20\x01(\tR\texpStatus\x12\x1b\n\tlimit_bal\x18\x05\
    \x20\x01(\x01R\x08limitBal\x12!\n\x0climit_status\x18\x06\x20\x01(\tR\
    \x0blimitStatusb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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
