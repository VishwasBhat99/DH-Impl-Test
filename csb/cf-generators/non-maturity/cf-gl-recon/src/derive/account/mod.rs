// This file is generated by rust-protobuf 2.11.0. Do not edit
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
//! Generated file from `csb_cf_gl_recon.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq,Clone,Default)]
pub struct Account {
    // message fields
    pub as_on_dt: i64,
    pub src_file_name: ::std::string::String,
    pub src_gl_cd: i64,
    pub gl_typ: ::std::string::String,
    pub src_ccy: ::std::string::String,
    pub src_gl_bal: f64,
    pub ora_gl_cd: i64,
    pub ora_gl_bal: f64,
    pub ora_ccy: ::std::string::String,
    pub gl_diff_amt: f64,
    pub w4b_cd: ::std::string::String,
    pub balm_llg: ::std::string::String,
    pub care_llg: ::std::string::String,
    pub ba_llg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Account {
    fn default() -> &'a Account {
        <Account as ::protobuf::Message>::default_instance()
    }
}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    // int64 as_on_dt = 1;


    pub fn get_as_on_dt(&self) -> i64 {
        self.as_on_dt
    }
    pub fn clear_as_on_dt(&mut self) {
        self.as_on_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_as_on_dt(&mut self, v: i64) {
        self.as_on_dt = v;
    }

    // string src_file_name = 2;


    pub fn get_src_file_name(&self) -> &str {
        &self.src_file_name
    }
    pub fn clear_src_file_name(&mut self) {
        self.src_file_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_src_file_name(&mut self, v: ::std::string::String) {
        self.src_file_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src_file_name(&mut self) -> &mut ::std::string::String {
        &mut self.src_file_name
    }

    // Take field
    pub fn take_src_file_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.src_file_name, ::std::string::String::new())
    }

    // int64 src_gl_cd = 3;


    pub fn get_src_gl_cd(&self) -> i64 {
        self.src_gl_cd
    }
    pub fn clear_src_gl_cd(&mut self) {
        self.src_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_src_gl_cd(&mut self, v: i64) {
        self.src_gl_cd = v;
    }

    // string gl_typ = 4;


    pub fn get_gl_typ(&self) -> &str {
        &self.gl_typ
    }
    pub fn clear_gl_typ(&mut self) {
        self.gl_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl_typ(&mut self, v: ::std::string::String) {
        self.gl_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl_typ(&mut self) -> &mut ::std::string::String {
        &mut self.gl_typ
    }

    // Take field
    pub fn take_gl_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl_typ, ::std::string::String::new())
    }

    // string src_ccy = 5;


    pub fn get_src_ccy(&self) -> &str {
        &self.src_ccy
    }
    pub fn clear_src_ccy(&mut self) {
        self.src_ccy.clear();
    }

    // Param is passed by value, moved
    pub fn set_src_ccy(&mut self, v: ::std::string::String) {
        self.src_ccy = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src_ccy(&mut self) -> &mut ::std::string::String {
        &mut self.src_ccy
    }

    // Take field
    pub fn take_src_ccy(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.src_ccy, ::std::string::String::new())
    }

    // double src_gl_bal = 6;


    pub fn get_src_gl_bal(&self) -> f64 {
        self.src_gl_bal
    }
    pub fn clear_src_gl_bal(&mut self) {
        self.src_gl_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_src_gl_bal(&mut self, v: f64) {
        self.src_gl_bal = v;
    }

    // int64 ora_gl_cd = 7;


    pub fn get_ora_gl_cd(&self) -> i64 {
        self.ora_gl_cd
    }
    pub fn clear_ora_gl_cd(&mut self) {
        self.ora_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_ora_gl_cd(&mut self, v: i64) {
        self.ora_gl_cd = v;
    }

    // double ora_gl_bal = 8;


    pub fn get_ora_gl_bal(&self) -> f64 {
        self.ora_gl_bal
    }
    pub fn clear_ora_gl_bal(&mut self) {
        self.ora_gl_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ora_gl_bal(&mut self, v: f64) {
        self.ora_gl_bal = v;
    }

    // string ora_ccy = 9;


    pub fn get_ora_ccy(&self) -> &str {
        &self.ora_ccy
    }
    pub fn clear_ora_ccy(&mut self) {
        self.ora_ccy.clear();
    }

    // Param is passed by value, moved
    pub fn set_ora_ccy(&mut self, v: ::std::string::String) {
        self.ora_ccy = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ora_ccy(&mut self) -> &mut ::std::string::String {
        &mut self.ora_ccy
    }

    // Take field
    pub fn take_ora_ccy(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ora_ccy, ::std::string::String::new())
    }

    // double gl_diff_amt = 10;


    pub fn get_gl_diff_amt(&self) -> f64 {
        self.gl_diff_amt
    }
    pub fn clear_gl_diff_amt(&mut self) {
        self.gl_diff_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_gl_diff_amt(&mut self, v: f64) {
        self.gl_diff_amt = v;
    }

    // string w4b_cd = 11;


    pub fn get_w4b_cd(&self) -> &str {
        &self.w4b_cd
    }
    pub fn clear_w4b_cd(&mut self) {
        self.w4b_cd.clear();
    }

    // Param is passed by value, moved
    pub fn set_w4b_cd(&mut self, v: ::std::string::String) {
        self.w4b_cd = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_w4b_cd(&mut self) -> &mut ::std::string::String {
        &mut self.w4b_cd
    }

    // Take field
    pub fn take_w4b_cd(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.w4b_cd, ::std::string::String::new())
    }

    // string balm_llg = 12;


    pub fn get_balm_llg(&self) -> &str {
        &self.balm_llg
    }
    pub fn clear_balm_llg(&mut self) {
        self.balm_llg.clear();
    }

    // Param is passed by value, moved
    pub fn set_balm_llg(&mut self, v: ::std::string::String) {
        self.balm_llg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_balm_llg(&mut self) -> &mut ::std::string::String {
        &mut self.balm_llg
    }

    // Take field
    pub fn take_balm_llg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.balm_llg, ::std::string::String::new())
    }

    // string care_llg = 13;


    pub fn get_care_llg(&self) -> &str {
        &self.care_llg
    }
    pub fn clear_care_llg(&mut self) {
        self.care_llg.clear();
    }

    // Param is passed by value, moved
    pub fn set_care_llg(&mut self, v: ::std::string::String) {
        self.care_llg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_care_llg(&mut self) -> &mut ::std::string::String {
        &mut self.care_llg
    }

    // Take field
    pub fn take_care_llg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.care_llg, ::std::string::String::new())
    }

    // string ba_llg = 14;


    pub fn get_ba_llg(&self) -> &str {
        &self.ba_llg
    }
    pub fn clear_ba_llg(&mut self) {
        self.ba_llg.clear();
    }

    // Param is passed by value, moved
    pub fn set_ba_llg(&mut self, v: ::std::string::String) {
        self.ba_llg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ba_llg(&mut self) -> &mut ::std::string::String {
        &mut self.ba_llg
    }

    // Take field
    pub fn take_ba_llg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ba_llg, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.as_on_dt = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.src_file_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.src_gl_cd = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_typ)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.src_ccy)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.src_gl_bal = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ora_gl_cd = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ora_gl_bal = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ora_ccy)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.gl_diff_amt = tmp;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.w4b_cd)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balm_llg)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.care_llg)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ba_llg)?;
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
        if self.as_on_dt != 0 {
            my_size += ::protobuf::rt::value_size(1, self.as_on_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.src_file_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.src_file_name);
        }
        if self.src_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(3, self.src_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.gl_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.gl_typ);
        }
        if !self.src_ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.src_ccy);
        }
        if self.src_gl_bal != 0. {
            my_size += 9;
        }
        if self.ora_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(7, self.ora_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ora_gl_bal != 0. {
            my_size += 9;
        }
        if !self.ora_ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.ora_ccy);
        }
        if self.gl_diff_amt != 0. {
            my_size += 9;
        }
        if !self.w4b_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.w4b_cd);
        }
        if !self.balm_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.balm_llg);
        }
        if !self.care_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.care_llg);
        }
        if !self.ba_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.ba_llg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.as_on_dt != 0 {
            os.write_int64(1, self.as_on_dt)?;
        }
        if !self.src_file_name.is_empty() {
            os.write_string(2, &self.src_file_name)?;
        }
        if self.src_gl_cd != 0 {
            os.write_int64(3, self.src_gl_cd)?;
        }
        if !self.gl_typ.is_empty() {
            os.write_string(4, &self.gl_typ)?;
        }
        if !self.src_ccy.is_empty() {
            os.write_string(5, &self.src_ccy)?;
        }
        if self.src_gl_bal != 0. {
            os.write_double(6, self.src_gl_bal)?;
        }
        if self.ora_gl_cd != 0 {
            os.write_int64(7, self.ora_gl_cd)?;
        }
        if self.ora_gl_bal != 0. {
            os.write_double(8, self.ora_gl_bal)?;
        }
        if !self.ora_ccy.is_empty() {
            os.write_string(9, &self.ora_ccy)?;
        }
        if self.gl_diff_amt != 0. {
            os.write_double(10, self.gl_diff_amt)?;
        }
        if !self.w4b_cd.is_empty() {
            os.write_string(11, &self.w4b_cd)?;
        }
        if !self.balm_llg.is_empty() {
            os.write_string(12, &self.balm_llg)?;
        }
        if !self.care_llg.is_empty() {
            os.write_string(13, &self.care_llg)?;
        }
        if !self.ba_llg.is_empty() {
            os.write_string(14, &self.ba_llg)?;
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

    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "as_on_dt",
                    |m: &Account| { &m.as_on_dt },
                    |m: &mut Account| { &mut m.as_on_dt },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src_file_name",
                    |m: &Account| { &m.src_file_name },
                    |m: &mut Account| { &mut m.src_file_name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "src_gl_cd",
                    |m: &Account| { &m.src_gl_cd },
                    |m: &mut Account| { &mut m.src_gl_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gl_typ",
                    |m: &Account| { &m.gl_typ },
                    |m: &mut Account| { &mut m.gl_typ },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src_ccy",
                    |m: &Account| { &m.src_ccy },
                    |m: &mut Account| { &mut m.src_ccy },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "src_gl_bal",
                    |m: &Account| { &m.src_gl_bal },
                    |m: &mut Account| { &mut m.src_gl_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ora_gl_cd",
                    |m: &Account| { &m.ora_gl_cd },
                    |m: &mut Account| { &mut m.ora_gl_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "ora_gl_bal",
                    |m: &Account| { &m.ora_gl_bal },
                    |m: &mut Account| { &mut m.ora_gl_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ora_ccy",
                    |m: &Account| { &m.ora_ccy },
                    |m: &mut Account| { &mut m.ora_ccy },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "gl_diff_amt",
                    |m: &Account| { &m.gl_diff_amt },
                    |m: &mut Account| { &mut m.gl_diff_amt },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "w4b_cd",
                    |m: &Account| { &m.w4b_cd },
                    |m: &mut Account| { &mut m.w4b_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "balm_llg",
                    |m: &Account| { &m.balm_llg },
                    |m: &mut Account| { &mut m.balm_llg },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "care_llg",
                    |m: &Account| { &m.care_llg },
                    |m: &mut Account| { &mut m.care_llg },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ba_llg",
                    |m: &Account| { &m.ba_llg },
                    |m: &mut Account| { &mut m.ba_llg },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Account>(
                    "Account",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Account {
        static mut instance: ::protobuf::lazy::Lazy<Account> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Account::new)
        }
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.as_on_dt = 0;
        self.src_file_name.clear();
        self.src_gl_cd = 0;
        self.gl_typ.clear();
        self.src_ccy.clear();
        self.src_gl_bal = 0.;
        self.ora_gl_cd = 0;
        self.ora_gl_bal = 0.;
        self.ora_ccy.clear();
        self.gl_diff_amt = 0.;
        self.w4b_cd.clear();
        self.balm_llg.clear();
        self.care_llg.clear();
        self.ba_llg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15csb_cf_gl_recon.proto\"\x88\x03\n\x07Account\x12\x18\n\x08as_on_dt\
    \x18\x01\x20\x01(\x03R\x06asOnDt\x12\"\n\rsrc_file_name\x18\x02\x20\x01(\
    \tR\x0bsrcFileName\x12\x1a\n\tsrc_gl_cd\x18\x03\x20\x01(\x03R\x07srcGlCd\
    \x12\x15\n\x06gl_typ\x18\x04\x20\x01(\tR\x05glTyp\x12\x17\n\x07src_ccy\
    \x18\x05\x20\x01(\tR\x06srcCcy\x12\x1c\n\nsrc_gl_bal\x18\x06\x20\x01(\
    \x01R\x08srcGlBal\x12\x1a\n\tora_gl_cd\x18\x07\x20\x01(\x03R\x07oraGlCd\
    \x12\x1c\n\nora_gl_bal\x18\x08\x20\x01(\x01R\x08oraGlBal\x12\x17\n\x07or\
    a_ccy\x18\t\x20\x01(\tR\x06oraCcy\x12\x1e\n\x0bgl_diff_amt\x18\n\x20\x01\
    (\x01R\tglDiffAmt\x12\x15\n\x06w4b_cd\x18\x0b\x20\x01(\tR\x05w4bCd\x12\
    \x19\n\x08balm_llg\x18\x0c\x20\x01(\tR\x07balmLlg\x12\x19\n\x08care_llg\
    \x18\r\x20\x01(\tR\x07careLlg\x12\x15\n\x06ba_llg\x18\x0e\x20\x01(\tR\
    \x05baLlgb\x06proto3\
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
