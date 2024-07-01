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
pub struct OutputAccount {
    // message fields
    pub gl_item: i64,
    pub branch: i64,
    pub basic: i64,
    pub suffix: i64,
    pub currency: ::std::string::String,
    pub cf_amount: f64,
    pub balance_in_omr: f64,
    pub rl1: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // int64 gl_item = 1;

    pub fn clear_gl_item(&mut self) {
        self.gl_item = 0;
    }

    // Param is passed by value, moved
    pub fn set_gl_item(&mut self, v: i64) {
        self.gl_item = v;
    }

    pub fn get_gl_item(&self) -> i64 {
        self.gl_item
    }

    // int64 branch = 2;

    pub fn clear_branch(&mut self) {
        self.branch = 0;
    }

    // Param is passed by value, moved
    pub fn set_branch(&mut self, v: i64) {
        self.branch = v;
    }

    pub fn get_branch(&self) -> i64 {
        self.branch
    }

    // int64 basic = 3;

    pub fn clear_basic(&mut self) {
        self.basic = 0;
    }

    // Param is passed by value, moved
    pub fn set_basic(&mut self, v: i64) {
        self.basic = v;
    }

    pub fn get_basic(&self) -> i64 {
        self.basic
    }

    // int64 suffix = 4;

    pub fn clear_suffix(&mut self) {
        self.suffix = 0;
    }

    // Param is passed by value, moved
    pub fn set_suffix(&mut self, v: i64) {
        self.suffix = v;
    }

    pub fn get_suffix(&self) -> i64 {
        self.suffix
    }

    // string currency = 5;

    pub fn clear_currency(&mut self) {
        self.currency.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency(&mut self) -> &mut ::std::string::String {
        &mut self.currency
    }

    // Take field
    pub fn take_currency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency, ::std::string::String::new())
    }

    pub fn get_currency(&self) -> &str {
        &self.currency
    }

    // double cf_amount = 6;

    pub fn clear_cf_amount(&mut self) {
        self.cf_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_cf_amount(&mut self, v: f64) {
        self.cf_amount = v;
    }

    pub fn get_cf_amount(&self) -> f64 {
        self.cf_amount
    }

    // double balance_in_omr = 7;

    pub fn clear_balance_in_omr(&mut self) {
        self.balance_in_omr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_balance_in_omr(&mut self, v: f64) {
        self.balance_in_omr = v;
    }

    pub fn get_balance_in_omr(&self) -> f64 {
        self.balance_in_omr
    }

    // int32 rl1 = 8;

    pub fn clear_rl1(&mut self) {
        self.rl1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_rl1(&mut self, v: i32) {
        self.rl1 = v;
    }

    pub fn get_rl1(&self) -> i32 {
        self.rl1
    }
}

impl ::protobuf::Message for OutputAccount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gl_item = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.branch = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.basic = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.suffix = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.cf_amount = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.balance_in_omr = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.rl1 = tmp;
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
        if self.gl_item != 0 {
            my_size += ::protobuf::rt::value_size(1, self.gl_item, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.branch != 0 {
            my_size += ::protobuf::rt::value_size(2, self.branch, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.basic != 0 {
            my_size += ::protobuf::rt::value_size(3, self.basic, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.suffix != 0 {
            my_size += ::protobuf::rt::value_size(4, self.suffix, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.currency);
        }
        if self.cf_amount != 0. {
            my_size += 9;
        }
        if self.balance_in_omr != 0. {
            my_size += 9;
        }
        if self.rl1 != 0 {
            my_size += ::protobuf::rt::value_size(8, self.rl1, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.gl_item != 0 {
            os.write_int64(1, self.gl_item)?;
        }
        if self.branch != 0 {
            os.write_int64(2, self.branch)?;
        }
        if self.basic != 0 {
            os.write_int64(3, self.basic)?;
        }
        if self.suffix != 0 {
            os.write_int64(4, self.suffix)?;
        }
        if !self.currency.is_empty() {
            os.write_string(5, &self.currency)?;
        }
        if self.cf_amount != 0. {
            os.write_double(6, self.cf_amount)?;
        }
        if self.balance_in_omr != 0. {
            os.write_double(7, self.balance_in_omr)?;
        }
        if self.rl1 != 0 {
            os.write_int32(8, self.rl1)?;
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

    fn new() -> OutputAccount {
        OutputAccount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "gl_item",
                    |m: &OutputAccount| { &m.gl_item },
                    |m: &mut OutputAccount| { &mut m.gl_item },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "branch",
                    |m: &OutputAccount| { &m.branch },
                    |m: &mut OutputAccount| { &mut m.branch },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "basic",
                    |m: &OutputAccount| { &m.basic },
                    |m: &mut OutputAccount| { &mut m.basic },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "suffix",
                    |m: &OutputAccount| { &m.suffix },
                    |m: &mut OutputAccount| { &mut m.suffix },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currency",
                    |m: &OutputAccount| { &m.currency },
                    |m: &mut OutputAccount| { &mut m.currency },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "cf_amount",
                    |m: &OutputAccount| { &m.cf_amount },
                    |m: &mut OutputAccount| { &mut m.cf_amount },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "balance_in_omr",
                    |m: &OutputAccount| { &m.balance_in_omr },
                    |m: &mut OutputAccount| { &mut m.balance_in_omr },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "rl1",
                    |m: &OutputAccount| { &m.rl1 },
                    |m: &mut OutputAccount| { &mut m.rl1 },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputAccount>(
                    "OutputAccount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OutputAccount {
        static mut instance: ::protobuf::lazy::Lazy<OutputAccount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputAccount,
        };
        unsafe {
            instance.get(OutputAccount::new)
        }
    }
}

impl ::protobuf::Clear for OutputAccount {
    fn clear(&mut self) {
        self.clear_gl_item();
        self.clear_branch();
        self.clear_basic();
        self.clear_suffix();
        self.clear_currency();
        self.clear_cf_amount();
        self.clear_balance_in_omr();
        self.clear_rl1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputAccount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\"\xdf\x01\n\rOutputAccount\x12\x17\n\x07gl_item\x18\
    \x01\x20\x01(\x03R\x06glItem\x12\x16\n\x06branch\x18\x02\x20\x01(\x03R\
    \x06branch\x12\x14\n\x05basic\x18\x03\x20\x01(\x03R\x05basic\x12\x16\n\
    \x06suffix\x18\x04\x20\x01(\x03R\x06suffix\x12\x1a\n\x08currency\x18\x05\
    \x20\x01(\tR\x08currency\x12\x1b\n\tcf_amount\x18\x06\x20\x01(\x01R\x08c\
    fAmount\x12$\n\x0ebalance_in_omr\x18\x07\x20\x01(\x01R\x0cbalanceInOmr\
    \x12\x10\n\x03rl1\x18\x08\x20\x01(\x05R\x03rl1b\x06proto3\
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