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
pub struct OutputAccount {
    // message fields
    pub date: i64,
    pub segment: ::std::string::String,
    pub sub_segment: ::std::string::String,
    pub member_id: ::std::string::String,
    pub member_name: ::std::string::String,
    pub isin: ::std::string::String,
    pub security_desc: ::std::string::String,
    pub mat_date: i64,
    pub face_value: i64,
    pub face_val_treps: ::std::string::String,
    pub balance: ::std::string::String,
    pub isin_cred_lend: ::std::string::String,
    pub security_des: ::std::string::String,
    pub face_val_rec: ::std::string::String,
    pub mar_val: f64,
    pub book_val: f64,
    pub os_amt: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // int64 date = 1;

    pub fn clear_date(&mut self) {
        self.date = 0;
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: i64) {
        self.date = v;
    }

    pub fn get_date(&self) -> i64 {
        self.date
    }

    // string segment = 2;

    pub fn clear_segment(&mut self) {
        self.segment.clear();
    }

    // Param is passed by value, moved
    pub fn set_segment(&mut self, v: ::std::string::String) {
        self.segment = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_segment(&mut self) -> &mut ::std::string::String {
        &mut self.segment
    }

    // Take field
    pub fn take_segment(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.segment, ::std::string::String::new())
    }

    pub fn get_segment(&self) -> &str {
        &self.segment
    }

    // string sub_segment = 3;

    pub fn clear_sub_segment(&mut self) {
        self.sub_segment.clear();
    }

    // Param is passed by value, moved
    pub fn set_sub_segment(&mut self, v: ::std::string::String) {
        self.sub_segment = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_segment(&mut self) -> &mut ::std::string::String {
        &mut self.sub_segment
    }

    // Take field
    pub fn take_sub_segment(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sub_segment, ::std::string::String::new())
    }

    pub fn get_sub_segment(&self) -> &str {
        &self.sub_segment
    }

    // string member_id = 4;

    pub fn clear_member_id(&mut self) {
        self.member_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: ::std::string::String) {
        self.member_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member_id(&mut self) -> &mut ::std::string::String {
        &mut self.member_id
    }

    // Take field
    pub fn take_member_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.member_id, ::std::string::String::new())
    }

    pub fn get_member_id(&self) -> &str {
        &self.member_id
    }

    // string member_name = 5;

    pub fn clear_member_name(&mut self) {
        self.member_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_member_name(&mut self, v: ::std::string::String) {
        self.member_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member_name(&mut self) -> &mut ::std::string::String {
        &mut self.member_name
    }

    // Take field
    pub fn take_member_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.member_name, ::std::string::String::new())
    }

    pub fn get_member_name(&self) -> &str {
        &self.member_name
    }

    // string isin = 6;

    pub fn clear_isin(&mut self) {
        self.isin.clear();
    }

    // Param is passed by value, moved
    pub fn set_isin(&mut self, v: ::std::string::String) {
        self.isin = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_isin(&mut self) -> &mut ::std::string::String {
        &mut self.isin
    }

    // Take field
    pub fn take_isin(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.isin, ::std::string::String::new())
    }

    pub fn get_isin(&self) -> &str {
        &self.isin
    }

    // string security_desc = 7;

    pub fn clear_security_desc(&mut self) {
        self.security_desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_security_desc(&mut self, v: ::std::string::String) {
        self.security_desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_security_desc(&mut self) -> &mut ::std::string::String {
        &mut self.security_desc
    }

    // Take field
    pub fn take_security_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.security_desc, ::std::string::String::new())
    }

    pub fn get_security_desc(&self) -> &str {
        &self.security_desc
    }

    // int64 mat_date = 8;

    pub fn clear_mat_date(&mut self) {
        self.mat_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_mat_date(&mut self, v: i64) {
        self.mat_date = v;
    }

    pub fn get_mat_date(&self) -> i64 {
        self.mat_date
    }

    // int64 face_value = 9;

    pub fn clear_face_value(&mut self) {
        self.face_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_face_value(&mut self, v: i64) {
        self.face_value = v;
    }

    pub fn get_face_value(&self) -> i64 {
        self.face_value
    }

    // string face_val_treps = 10;

    pub fn clear_face_val_treps(&mut self) {
        self.face_val_treps.clear();
    }

    // Param is passed by value, moved
    pub fn set_face_val_treps(&mut self, v: ::std::string::String) {
        self.face_val_treps = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_face_val_treps(&mut self) -> &mut ::std::string::String {
        &mut self.face_val_treps
    }

    // Take field
    pub fn take_face_val_treps(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.face_val_treps, ::std::string::String::new())
    }

    pub fn get_face_val_treps(&self) -> &str {
        &self.face_val_treps
    }

    // string balance = 11;

    pub fn clear_balance(&mut self) {
        self.balance.clear();
    }

    // Param is passed by value, moved
    pub fn set_balance(&mut self, v: ::std::string::String) {
        self.balance = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_balance(&mut self) -> &mut ::std::string::String {
        &mut self.balance
    }

    // Take field
    pub fn take_balance(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.balance, ::std::string::String::new())
    }

    pub fn get_balance(&self) -> &str {
        &self.balance
    }

    // string isin_cred_lend = 12;

    pub fn clear_isin_cred_lend(&mut self) {
        self.isin_cred_lend.clear();
    }

    // Param is passed by value, moved
    pub fn set_isin_cred_lend(&mut self, v: ::std::string::String) {
        self.isin_cred_lend = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_isin_cred_lend(&mut self) -> &mut ::std::string::String {
        &mut self.isin_cred_lend
    }

    // Take field
    pub fn take_isin_cred_lend(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.isin_cred_lend, ::std::string::String::new())
    }

    pub fn get_isin_cred_lend(&self) -> &str {
        &self.isin_cred_lend
    }

    // string security_des = 13;

    pub fn clear_security_des(&mut self) {
        self.security_des.clear();
    }

    // Param is passed by value, moved
    pub fn set_security_des(&mut self, v: ::std::string::String) {
        self.security_des = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_security_des(&mut self) -> &mut ::std::string::String {
        &mut self.security_des
    }

    // Take field
    pub fn take_security_des(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.security_des, ::std::string::String::new())
    }

    pub fn get_security_des(&self) -> &str {
        &self.security_des
    }

    // string face_val_rec = 14;

    pub fn clear_face_val_rec(&mut self) {
        self.face_val_rec.clear();
    }

    // Param is passed by value, moved
    pub fn set_face_val_rec(&mut self, v: ::std::string::String) {
        self.face_val_rec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_face_val_rec(&mut self) -> &mut ::std::string::String {
        &mut self.face_val_rec
    }

    // Take field
    pub fn take_face_val_rec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.face_val_rec, ::std::string::String::new())
    }

    pub fn get_face_val_rec(&self) -> &str {
        &self.face_val_rec
    }

    // double mar_val = 15;

    pub fn clear_mar_val(&mut self) {
        self.mar_val = 0.;
    }

    // Param is passed by value, moved
    pub fn set_mar_val(&mut self, v: f64) {
        self.mar_val = v;
    }

    pub fn get_mar_val(&self) -> f64 {
        self.mar_val
    }

    // double book_val = 16;

    pub fn clear_book_val(&mut self) {
        self.book_val = 0.;
    }

    // Param is passed by value, moved
    pub fn set_book_val(&mut self, v: f64) {
        self.book_val = v;
    }

    pub fn get_book_val(&self) -> f64 {
        self.book_val
    }

    // double os_amt = 17;

    pub fn clear_os_amt(&mut self) {
        self.os_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_amt(&mut self, v: f64) {
        self.os_amt = v;
    }

    pub fn get_os_amt(&self) -> f64 {
        self.os_amt
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
                    self.date = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.segment)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sub_segment)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.member_id)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.member_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.isin)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.security_desc)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_date = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.face_value = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.face_val_treps)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balance)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.isin_cred_lend)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.security_des)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.face_val_rec)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.mar_val = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.book_val = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_amt = tmp;
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
        if self.date != 0 {
            my_size += ::protobuf::rt::value_size(1, self.date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.segment.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.segment);
        }
        if !self.sub_segment.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.sub_segment);
        }
        if !self.member_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.member_id);
        }
        if !self.member_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.member_name);
        }
        if !self.isin.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.isin);
        }
        if !self.security_desc.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.security_desc);
        }
        if self.mat_date != 0 {
            my_size += ::protobuf::rt::value_size(8, self.mat_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.face_value != 0 {
            my_size += ::protobuf::rt::value_size(9, self.face_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.face_val_treps.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.face_val_treps);
        }
        if !self.balance.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.balance);
        }
        if !self.isin_cred_lend.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.isin_cred_lend);
        }
        if !self.security_des.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.security_des);
        }
        if !self.face_val_rec.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.face_val_rec);
        }
        if self.mar_val != 0. {
            my_size += 9;
        }
        if self.book_val != 0. {
            my_size += 10;
        }
        if self.os_amt != 0. {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.date != 0 {
            os.write_int64(1, self.date)?;
        }
        if !self.segment.is_empty() {
            os.write_string(2, &self.segment)?;
        }
        if !self.sub_segment.is_empty() {
            os.write_string(3, &self.sub_segment)?;
        }
        if !self.member_id.is_empty() {
            os.write_string(4, &self.member_id)?;
        }
        if !self.member_name.is_empty() {
            os.write_string(5, &self.member_name)?;
        }
        if !self.isin.is_empty() {
            os.write_string(6, &self.isin)?;
        }
        if !self.security_desc.is_empty() {
            os.write_string(7, &self.security_desc)?;
        }
        if self.mat_date != 0 {
            os.write_int64(8, self.mat_date)?;
        }
        if self.face_value != 0 {
            os.write_int64(9, self.face_value)?;
        }
        if !self.face_val_treps.is_empty() {
            os.write_string(10, &self.face_val_treps)?;
        }
        if !self.balance.is_empty() {
            os.write_string(11, &self.balance)?;
        }
        if !self.isin_cred_lend.is_empty() {
            os.write_string(12, &self.isin_cred_lend)?;
        }
        if !self.security_des.is_empty() {
            os.write_string(13, &self.security_des)?;
        }
        if !self.face_val_rec.is_empty() {
            os.write_string(14, &self.face_val_rec)?;
        }
        if self.mar_val != 0. {
            os.write_double(15, self.mar_val)?;
        }
        if self.book_val != 0. {
            os.write_double(16, self.book_val)?;
        }
        if self.os_amt != 0. {
            os.write_double(17, self.os_amt)?;
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
                    "date",
                    |m: &OutputAccount| { &m.date },
                    |m: &mut OutputAccount| { &mut m.date },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "segment",
                    |m: &OutputAccount| { &m.segment },
                    |m: &mut OutputAccount| { &mut m.segment },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sub_segment",
                    |m: &OutputAccount| { &m.sub_segment },
                    |m: &mut OutputAccount| { &mut m.sub_segment },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_id",
                    |m: &OutputAccount| { &m.member_id },
                    |m: &mut OutputAccount| { &mut m.member_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_name",
                    |m: &OutputAccount| { &m.member_name },
                    |m: &mut OutputAccount| { &mut m.member_name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "isin",
                    |m: &OutputAccount| { &m.isin },
                    |m: &mut OutputAccount| { &mut m.isin },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "security_desc",
                    |m: &OutputAccount| { &m.security_desc },
                    |m: &mut OutputAccount| { &mut m.security_desc },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "mat_date",
                    |m: &OutputAccount| { &m.mat_date },
                    |m: &mut OutputAccount| { &mut m.mat_date },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "face_value",
                    |m: &OutputAccount| { &m.face_value },
                    |m: &mut OutputAccount| { &mut m.face_value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "face_val_treps",
                    |m: &OutputAccount| { &m.face_val_treps },
                    |m: &mut OutputAccount| { &mut m.face_val_treps },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "balance",
                    |m: &OutputAccount| { &m.balance },
                    |m: &mut OutputAccount| { &mut m.balance },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "isin_cred_lend",
                    |m: &OutputAccount| { &m.isin_cred_lend },
                    |m: &mut OutputAccount| { &mut m.isin_cred_lend },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "security_des",
                    |m: &OutputAccount| { &m.security_des },
                    |m: &mut OutputAccount| { &mut m.security_des },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "face_val_rec",
                    |m: &OutputAccount| { &m.face_val_rec },
                    |m: &mut OutputAccount| { &mut m.face_val_rec },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "mar_val",
                    |m: &OutputAccount| { &m.mar_val },
                    |m: &mut OutputAccount| { &mut m.mar_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "book_val",
                    |m: &OutputAccount| { &m.book_val },
                    |m: &mut OutputAccount| { &mut m.book_val },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "os_amt",
                    |m: &OutputAccount| { &m.os_amt },
                    |m: &mut OutputAccount| { &mut m.os_amt },
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
        self.clear_date();
        self.clear_segment();
        self.clear_sub_segment();
        self.clear_member_id();
        self.clear_member_name();
        self.clear_isin();
        self.clear_security_desc();
        self.clear_mat_date();
        self.clear_face_value();
        self.clear_face_val_treps();
        self.clear_balance();
        self.clear_isin_cred_lend();
        self.clear_security_des();
        self.clear_face_val_rec();
        self.clear_mar_val();
        self.clear_book_val();
        self.clear_os_amt();
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
    \n\x12nsfr-holding.proto\"\x85\x04\n\rOutputAccount\x12\x12\n\x04date\
    \x18\x01\x20\x01(\x03R\x04date\x12\x18\n\x07segment\x18\x02\x20\x01(\tR\
    \x07segment\x12\x1f\n\x0bsub_segment\x18\x03\x20\x01(\tR\nsubSegment\x12\
    \x1b\n\tmember_id\x18\x04\x20\x01(\tR\x08memberId\x12\x1f\n\x0bmember_na\
    me\x18\x05\x20\x01(\tR\nmemberName\x12\x12\n\x04isin\x18\x06\x20\x01(\tR\
    \x04isin\x12#\n\rsecurity_desc\x18\x07\x20\x01(\tR\x0csecurityDesc\x12\
    \x19\n\x08mat_date\x18\x08\x20\x01(\x03R\x07matDate\x12\x1d\n\nface_valu\
    e\x18\t\x20\x01(\x03R\tfaceValue\x12$\n\x0eface_val_treps\x18\n\x20\x01(\
    \tR\x0cfaceValTreps\x12\x18\n\x07balance\x18\x0b\x20\x01(\tR\x07balance\
    \x12$\n\x0eisin_cred_lend\x18\x0c\x20\x01(\tR\x0cisinCredLend\x12!\n\x0c\
    security_des\x18\r\x20\x01(\tR\x0bsecurityDes\x12\x20\n\x0cface_val_rec\
    \x18\x0e\x20\x01(\tR\nfaceValRec\x12\x17\n\x07mar_val\x18\x0f\x20\x01(\
    \x01R\x06marVal\x12\x19\n\x08book_val\x18\x10\x20\x01(\x01R\x07bookVal\
    \x12\x15\n\x06os_amt\x18\x11\x20\x01(\x01R\x05osAmtb\x06proto3\
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