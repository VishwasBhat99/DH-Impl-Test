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
    pub account_id: ::std::string::String,
    pub cf_date: i64,
    pub cf_amount: f64,
    pub currency: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // string account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: ::std::string::String) {
        self.account_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_id(&mut self) -> &mut ::std::string::String {
        &mut self.account_id
    }

    // Take field
    pub fn take_account_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.account_id, ::std::string::String::new())
    }

    pub fn get_account_id(&self) -> &str {
        &self.account_id
    }

    // int64 cf_date = 2;

    pub fn clear_cf_date(&mut self) {
        self.cf_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_cf_date(&mut self, v: i64) {
        self.cf_date = v;
    }

    pub fn get_cf_date(&self) -> i64 {
        self.cf_date
    }

    // double cf_amount = 3;

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

    // string currency = 4;

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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.account_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.cf_date = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.cf_amount = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
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
        if !self.account_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.account_id);
        }
        if self.cf_date != 0 {
            my_size += ::protobuf::rt::value_size(2, self.cf_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cf_amount != 0. {
            my_size += 9;
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.currency);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.account_id.is_empty() {
            os.write_string(1, &self.account_id)?;
        }
        if self.cf_date != 0 {
            os.write_int64(2, self.cf_date)?;
        }
        if self.cf_amount != 0. {
            os.write_double(3, self.cf_amount)?;
        }
        if !self.currency.is_empty() {
            os.write_string(4, &self.currency)?;
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

    fn as_any(&self) -> &dyn(::std::any::Any) {
        self as &dyn(::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn(::std::any::Any) {
        self as &mut dyn(::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn(::std::any::Any)> {
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_id",
                    |m: &OutputAccount| { &m.account_id },
                    |m: &mut OutputAccount| { &mut m.account_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "cf_date",
                    |m: &OutputAccount| { &m.cf_date },
                    |m: &mut OutputAccount| { &mut m.cf_date },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "cf_amount",
                    |m: &OutputAccount| { &m.cf_amount },
                    |m: &mut OutputAccount| { &mut m.cf_amount },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currency",
                    |m: &OutputAccount| { &m.currency },
                    |m: &mut OutputAccount| { &mut m.currency },
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
        self.clear_account_id();
        self.clear_cf_date();
        self.clear_cf_amount();
        self.clear_currency();
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
    \n\rmessage.proto\"\x80\x01\n\rOutputAccount\x12\x1d\n\naccount_id\x18\
    \x01\x20\x01(\tR\taccountId\x12\x17\n\x07cf_date\x18\x02\x20\x01(\x03R\
    \x06cfDate\x12\x1b\n\tcf_amount\x18\x03\x20\x01(\x01R\x08cfAmount\x12\
    \x1a\n\x08currency\x18\x04\x20\x01(\tR\x08currencyb\x06proto3\
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