// This file is generated by rust-protobuf 2.0.6. Do not edit
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
pub struct Data {
    // message fields
    pub lmr_field: ::std::string::String,
    pub ccy: ::std::string::String,
    pub amt: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Data {
    pub fn new() -> Data {
        ::std::default::Default::default()
    }

    // string lmr_field = 1;

    pub fn clear_lmr_field(&mut self) {
        self.lmr_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_lmr_field(&mut self, v: ::std::string::String) {
        self.lmr_field = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lmr_field(&mut self) -> &mut ::std::string::String {
        &mut self.lmr_field
    }

    // Take field
    pub fn take_lmr_field(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lmr_field, ::std::string::String::new())
    }

    pub fn get_lmr_field(&self) -> &str {
        &self.lmr_field
    }

    // string ccy = 2;

    pub fn clear_ccy(&mut self) {
        self.ccy.clear();
    }

    // Param is passed by value, moved
    pub fn set_ccy(&mut self, v: ::std::string::String) {
        self.ccy = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ccy(&mut self) -> &mut ::std::string::String {
        &mut self.ccy
    }

    // Take field
    pub fn take_ccy(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ccy, ::std::string::String::new())
    }

    pub fn get_ccy(&self) -> &str {
        &self.ccy
    }

    // double amt = 3;

    pub fn clear_amt(&mut self) {
        self.amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt(&mut self, v: f64) {
        self.amt = v;
    }

    pub fn get_amt(&self) -> f64 {
        self.amt
    }
}

impl ::protobuf::Message for Data {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lmr_field)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt = tmp;
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
        if !self.lmr_field.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.lmr_field);
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ccy);
        }
        if self.amt != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.lmr_field.is_empty() {
            os.write_string(1, &self.lmr_field)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(2, &self.ccy)?;
        }
        if self.amt != 0. {
            os.write_double(3, self.amt)?;
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

    fn new() -> Data {
        Data::new()
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
                    "lmr_field",
                    |m: &Data| { &m.lmr_field },
                    |m: &mut Data| { &mut m.lmr_field },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ccy",
                    |m: &Data| { &m.ccy },
                    |m: &mut Data| { &mut m.ccy },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "amt",
                    |m: &Data| { &m.amt },
                    |m: &mut Data| { &mut m.amt },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Data>(
                    "Data",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Data {
        static mut instance: ::protobuf::lazy::Lazy<Data> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Data,
        };
        unsafe {
            instance.get(Data::new)
        }
    }
}

impl ::protobuf::Clear for Data {
    fn clear(&mut self) {
        self.clear_lmr_field();
        self.clear_ccy();
        self.clear_amt();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Data {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ebasel_t1.proto\"G\n\x04Data\x12\x1b\n\tlmr_field\x18\x01\x20\x01(\
    \tR\x08lmrField\x12\x10\n\x03ccy\x18\x02\x20\x01(\tR\x03ccy\x12\x10\n\
    \x03amt\x18\x03\x20\x01(\x01R\x03amtb\x06proto3\
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