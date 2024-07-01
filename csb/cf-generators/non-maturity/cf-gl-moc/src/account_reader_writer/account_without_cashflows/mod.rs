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
//! Generated file from `csb_gl_moc.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_11_0;

#[derive(PartialEq,Clone,Default)]
pub struct OutputAccount {
    // message fields
    pub gl_cd: ::std::string::String,
    pub dr_bal: f64,
    pub cr_bal: f64,
    pub amt: f64,
    pub ccy: ::std::string::String,
    pub br_cd: ::std::string::String,
    pub typ: ::std::string::String,
    pub gl_desc: ::std::string::String,
    pub w4b_cd: ::std::string::String,
    pub w4b_desc: ::std::string::String,
    pub balm_llg: ::std::string::String,
    pub care_llg: ::std::string::String,
    pub ba_llg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OutputAccount {
    fn default() -> &'a OutputAccount {
        <OutputAccount as ::protobuf::Message>::default_instance()
    }
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // string gl_cd = 1;


    pub fn get_gl_cd(&self) -> &str {
        &self.gl_cd
    }
    pub fn clear_gl_cd(&mut self) {
        self.gl_cd.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl_cd(&mut self, v: ::std::string::String) {
        self.gl_cd = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl_cd(&mut self) -> &mut ::std::string::String {
        &mut self.gl_cd
    }

    // Take field
    pub fn take_gl_cd(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl_cd, ::std::string::String::new())
    }

    // double dr_bal = 2;


    pub fn get_dr_bal(&self) -> f64 {
        self.dr_bal
    }
    pub fn clear_dr_bal(&mut self) {
        self.dr_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_dr_bal(&mut self, v: f64) {
        self.dr_bal = v;
    }

    // double cr_bal = 3;


    pub fn get_cr_bal(&self) -> f64 {
        self.cr_bal
    }
    pub fn clear_cr_bal(&mut self) {
        self.cr_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_cr_bal(&mut self, v: f64) {
        self.cr_bal = v;
    }

    // double amt = 4;


    pub fn get_amt(&self) -> f64 {
        self.amt
    }
    pub fn clear_amt(&mut self) {
        self.amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt(&mut self, v: f64) {
        self.amt = v;
    }

    // string ccy = 5;


    pub fn get_ccy(&self) -> &str {
        &self.ccy
    }
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

    // string br_cd = 6;


    pub fn get_br_cd(&self) -> &str {
        &self.br_cd
    }
    pub fn clear_br_cd(&mut self) {
        self.br_cd.clear();
    }

    // Param is passed by value, moved
    pub fn set_br_cd(&mut self, v: ::std::string::String) {
        self.br_cd = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_br_cd(&mut self) -> &mut ::std::string::String {
        &mut self.br_cd
    }

    // Take field
    pub fn take_br_cd(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.br_cd, ::std::string::String::new())
    }

    // string typ = 7;


    pub fn get_typ(&self) -> &str {
        &self.typ
    }
    pub fn clear_typ(&mut self) {
        self.typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: ::std::string::String) {
        self.typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typ(&mut self) -> &mut ::std::string::String {
        &mut self.typ
    }

    // Take field
    pub fn take_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.typ, ::std::string::String::new())
    }

    // string gl_desc = 8;


    pub fn get_gl_desc(&self) -> &str {
        &self.gl_desc
    }
    pub fn clear_gl_desc(&mut self) {
        self.gl_desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl_desc(&mut self, v: ::std::string::String) {
        self.gl_desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl_desc(&mut self) -> &mut ::std::string::String {
        &mut self.gl_desc
    }

    // Take field
    pub fn take_gl_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl_desc, ::std::string::String::new())
    }

    // string w4b_cd = 9;


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

    // string w4b_desc = 10;


    pub fn get_w4b_desc(&self) -> &str {
        &self.w4b_desc
    }
    pub fn clear_w4b_desc(&mut self) {
        self.w4b_desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_w4b_desc(&mut self, v: ::std::string::String) {
        self.w4b_desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_w4b_desc(&mut self) -> &mut ::std::string::String {
        &mut self.w4b_desc
    }

    // Take field
    pub fn take_w4b_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.w4b_desc, ::std::string::String::new())
    }

    // string balm_llg = 11;


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

    // string care_llg = 12;


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

    // string ba_llg = 13;


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

impl ::protobuf::Message for OutputAccount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_cd)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.dr_bal = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.cr_bal = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.br_cd)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.typ)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_desc)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.w4b_cd)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.w4b_desc)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balm_llg)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.care_llg)?;
                },
                13 => {
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
        if !self.gl_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.gl_cd);
        }
        if self.dr_bal != 0. {
            my_size += 9;
        }
        if self.cr_bal != 0. {
            my_size += 9;
        }
        if self.amt != 0. {
            my_size += 9;
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.ccy);
        }
        if !self.br_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.br_cd);
        }
        if !self.typ.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.typ);
        }
        if !self.gl_desc.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.gl_desc);
        }
        if !self.w4b_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.w4b_cd);
        }
        if !self.w4b_desc.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.w4b_desc);
        }
        if !self.balm_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.balm_llg);
        }
        if !self.care_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.care_llg);
        }
        if !self.ba_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.ba_llg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.gl_cd.is_empty() {
            os.write_string(1, &self.gl_cd)?;
        }
        if self.dr_bal != 0. {
            os.write_double(2, self.dr_bal)?;
        }
        if self.cr_bal != 0. {
            os.write_double(3, self.cr_bal)?;
        }
        if self.amt != 0. {
            os.write_double(4, self.amt)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(5, &self.ccy)?;
        }
        if !self.br_cd.is_empty() {
            os.write_string(6, &self.br_cd)?;
        }
        if !self.typ.is_empty() {
            os.write_string(7, &self.typ)?;
        }
        if !self.gl_desc.is_empty() {
            os.write_string(8, &self.gl_desc)?;
        }
        if !self.w4b_cd.is_empty() {
            os.write_string(9, &self.w4b_cd)?;
        }
        if !self.w4b_desc.is_empty() {
            os.write_string(10, &self.w4b_desc)?;
        }
        if !self.balm_llg.is_empty() {
            os.write_string(11, &self.balm_llg)?;
        }
        if !self.care_llg.is_empty() {
            os.write_string(12, &self.care_llg)?;
        }
        if !self.ba_llg.is_empty() {
            os.write_string(13, &self.ba_llg)?;
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

    fn new() -> OutputAccount {
        OutputAccount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gl_cd",
                    |m: &OutputAccount| { &m.gl_cd },
                    |m: &mut OutputAccount| { &mut m.gl_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "dr_bal",
                    |m: &OutputAccount| { &m.dr_bal },
                    |m: &mut OutputAccount| { &mut m.dr_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "cr_bal",
                    |m: &OutputAccount| { &m.cr_bal },
                    |m: &mut OutputAccount| { &mut m.cr_bal },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "amt",
                    |m: &OutputAccount| { &m.amt },
                    |m: &mut OutputAccount| { &mut m.amt },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ccy",
                    |m: &OutputAccount| { &m.ccy },
                    |m: &mut OutputAccount| { &mut m.ccy },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "br_cd",
                    |m: &OutputAccount| { &m.br_cd },
                    |m: &mut OutputAccount| { &mut m.br_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "typ",
                    |m: &OutputAccount| { &m.typ },
                    |m: &mut OutputAccount| { &mut m.typ },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gl_desc",
                    |m: &OutputAccount| { &m.gl_desc },
                    |m: &mut OutputAccount| { &mut m.gl_desc },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "w4b_cd",
                    |m: &OutputAccount| { &m.w4b_cd },
                    |m: &mut OutputAccount| { &mut m.w4b_cd },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "w4b_desc",
                    |m: &OutputAccount| { &m.w4b_desc },
                    |m: &mut OutputAccount| { &mut m.w4b_desc },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "balm_llg",
                    |m: &OutputAccount| { &m.balm_llg },
                    |m: &mut OutputAccount| { &mut m.balm_llg },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "care_llg",
                    |m: &OutputAccount| { &m.care_llg },
                    |m: &mut OutputAccount| { &mut m.care_llg },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ba_llg",
                    |m: &OutputAccount| { &m.ba_llg },
                    |m: &mut OutputAccount| { &mut m.ba_llg },
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
        static mut instance: ::protobuf::lazy::Lazy<OutputAccount> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(OutputAccount::new)
        }
    }
}

impl ::protobuf::Clear for OutputAccount {
    fn clear(&mut self) {
        self.gl_cd.clear();
        self.dr_bal = 0.;
        self.cr_bal = 0.;
        self.amt = 0.;
        self.ccy.clear();
        self.br_cd.clear();
        self.typ.clear();
        self.gl_desc.clear();
        self.w4b_cd.clear();
        self.w4b_desc.clear();
        self.balm_llg.clear();
        self.care_llg.clear();
        self.ba_llg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputAccount {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10csb_gl_moc.proto\"\xb5\x02\n\rOutputAccount\x12\x13\n\x05gl_cd\x18\
    \x01\x20\x01(\tR\x04glCd\x12\x15\n\x06dr_bal\x18\x02\x20\x01(\x01R\x05dr\
    Bal\x12\x15\n\x06cr_bal\x18\x03\x20\x01(\x01R\x05crBal\x12\x10\n\x03amt\
    \x18\x04\x20\x01(\x01R\x03amt\x12\x10\n\x03ccy\x18\x05\x20\x01(\tR\x03cc\
    y\x12\x13\n\x05br_cd\x18\x06\x20\x01(\tR\x04brCd\x12\x10\n\x03typ\x18\
    \x07\x20\x01(\tR\x03typ\x12\x17\n\x07gl_desc\x18\x08\x20\x01(\tR\x06glDe\
    sc\x12\x15\n\x06w4b_cd\x18\t\x20\x01(\tR\x05w4bCd\x12\x19\n\x08w4b_desc\
    \x18\n\x20\x01(\tR\x07w4bDesc\x12\x19\n\x08balm_llg\x18\x0b\x20\x01(\tR\
    \x07balmLlg\x12\x19\n\x08care_llg\x18\x0c\x20\x01(\tR\x07careLlg\x12\x15\
    \n\x06ba_llg\x18\r\x20\x01(\tR\x05baLlgb\x06proto3\
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