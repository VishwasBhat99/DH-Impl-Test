// This file is generated by rust-protobuf 2.27.1. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `ndtl_1.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct OutputAccount {
    // message fields
    pub as_on_date: ::std::string::String,
    pub ndtl_val: f64,
    pub total_gsec_bv: f64,
    pub total_gsec_mv: f64,
    pub ex_slr_bv: f64,
    pub msf_bv: f64,
    pub slr_bv: f64,
    pub ex_slr_pct_gsec: f64,
    pub msf_pct_gsec: f64,
    pub slr_pct_gsec: f64,
    pub ex_slr_mv: f64,
    pub msf_mv: f64,
    pub slr_mv: f64,
    pub final_excess_slr: f64,
    pub final_msf: f64,
    pub final_slr: f64,
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

    // string as_on_date = 1;


    pub fn get_as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn clear_as_on_date(&mut self) {
        self.as_on_date.clear();
    }

    // Param is passed by value, moved
    pub fn set_as_on_date(&mut self, v: ::std::string::String) {
        self.as_on_date = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_as_on_date(&mut self) -> &mut ::std::string::String {
        &mut self.as_on_date
    }

    // Take field
    pub fn take_as_on_date(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.as_on_date, ::std::string::String::new())
    }

    // double ndtl_val = 2;


    pub fn get_ndtl_val(&self) -> f64 {
        self.ndtl_val
    }
    pub fn clear_ndtl_val(&mut self) {
        self.ndtl_val = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ndtl_val(&mut self, v: f64) {
        self.ndtl_val = v;
    }

    // double total_gsec_bv = 3;


    pub fn get_total_gsec_bv(&self) -> f64 {
        self.total_gsec_bv
    }
    pub fn clear_total_gsec_bv(&mut self) {
        self.total_gsec_bv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_gsec_bv(&mut self, v: f64) {
        self.total_gsec_bv = v;
    }

    // double total_gsec_mv = 4;


    pub fn get_total_gsec_mv(&self) -> f64 {
        self.total_gsec_mv
    }
    pub fn clear_total_gsec_mv(&mut self) {
        self.total_gsec_mv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_gsec_mv(&mut self, v: f64) {
        self.total_gsec_mv = v;
    }

    // double ex_slr_bv = 5;


    pub fn get_ex_slr_bv(&self) -> f64 {
        self.ex_slr_bv
    }
    pub fn clear_ex_slr_bv(&mut self) {
        self.ex_slr_bv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ex_slr_bv(&mut self, v: f64) {
        self.ex_slr_bv = v;
    }

    // double msf_bv = 6;


    pub fn get_msf_bv(&self) -> f64 {
        self.msf_bv
    }
    pub fn clear_msf_bv(&mut self) {
        self.msf_bv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_msf_bv(&mut self, v: f64) {
        self.msf_bv = v;
    }

    // double slr_bv = 7;


    pub fn get_slr_bv(&self) -> f64 {
        self.slr_bv
    }
    pub fn clear_slr_bv(&mut self) {
        self.slr_bv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_slr_bv(&mut self, v: f64) {
        self.slr_bv = v;
    }

    // double ex_slr_pct_gsec = 8;


    pub fn get_ex_slr_pct_gsec(&self) -> f64 {
        self.ex_slr_pct_gsec
    }
    pub fn clear_ex_slr_pct_gsec(&mut self) {
        self.ex_slr_pct_gsec = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ex_slr_pct_gsec(&mut self, v: f64) {
        self.ex_slr_pct_gsec = v;
    }

    // double msf_pct_gsec = 9;


    pub fn get_msf_pct_gsec(&self) -> f64 {
        self.msf_pct_gsec
    }
    pub fn clear_msf_pct_gsec(&mut self) {
        self.msf_pct_gsec = 0.;
    }

    // Param is passed by value, moved
    pub fn set_msf_pct_gsec(&mut self, v: f64) {
        self.msf_pct_gsec = v;
    }

    // double slr_pct_gsec = 10;


    pub fn get_slr_pct_gsec(&self) -> f64 {
        self.slr_pct_gsec
    }
    pub fn clear_slr_pct_gsec(&mut self) {
        self.slr_pct_gsec = 0.;
    }

    // Param is passed by value, moved
    pub fn set_slr_pct_gsec(&mut self, v: f64) {
        self.slr_pct_gsec = v;
    }

    // double ex_slr_mv = 11;


    pub fn get_ex_slr_mv(&self) -> f64 {
        self.ex_slr_mv
    }
    pub fn clear_ex_slr_mv(&mut self) {
        self.ex_slr_mv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ex_slr_mv(&mut self, v: f64) {
        self.ex_slr_mv = v;
    }

    // double msf_mv = 12;


    pub fn get_msf_mv(&self) -> f64 {
        self.msf_mv
    }
    pub fn clear_msf_mv(&mut self) {
        self.msf_mv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_msf_mv(&mut self, v: f64) {
        self.msf_mv = v;
    }

    // double slr_mv = 13;


    pub fn get_slr_mv(&self) -> f64 {
        self.slr_mv
    }
    pub fn clear_slr_mv(&mut self) {
        self.slr_mv = 0.;
    }

    // Param is passed by value, moved
    pub fn set_slr_mv(&mut self, v: f64) {
        self.slr_mv = v;
    }

    // double final_excess_slr = 14;


    pub fn get_final_excess_slr(&self) -> f64 {
        self.final_excess_slr
    }
    pub fn clear_final_excess_slr(&mut self) {
        self.final_excess_slr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_final_excess_slr(&mut self, v: f64) {
        self.final_excess_slr = v;
    }

    // double final_msf = 15;


    pub fn get_final_msf(&self) -> f64 {
        self.final_msf
    }
    pub fn clear_final_msf(&mut self) {
        self.final_msf = 0.;
    }

    // Param is passed by value, moved
    pub fn set_final_msf(&mut self, v: f64) {
        self.final_msf = v;
    }

    // double final_slr = 16;


    pub fn get_final_slr(&self) -> f64 {
        self.final_slr
    }
    pub fn clear_final_slr(&mut self) {
        self.final_slr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_final_slr(&mut self, v: f64) {
        self.final_slr = v;
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.as_on_date)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ndtl_val = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_gsec_bv = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_gsec_mv = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ex_slr_bv = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.msf_bv = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.slr_bv = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ex_slr_pct_gsec = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.msf_pct_gsec = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.slr_pct_gsec = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ex_slr_mv = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.msf_mv = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.slr_mv = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.final_excess_slr = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.final_msf = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.final_slr = tmp;
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
        if !self.as_on_date.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.as_on_date);
        }
        if self.ndtl_val != 0. {
            my_size += 9;
        }
        if self.total_gsec_bv != 0. {
            my_size += 9;
        }
        if self.total_gsec_mv != 0. {
            my_size += 9;
        }
        if self.ex_slr_bv != 0. {
            my_size += 9;
        }
        if self.msf_bv != 0. {
            my_size += 9;
        }
        if self.slr_bv != 0. {
            my_size += 9;
        }
        if self.ex_slr_pct_gsec != 0. {
            my_size += 9;
        }
        if self.msf_pct_gsec != 0. {
            my_size += 9;
        }
        if self.slr_pct_gsec != 0. {
            my_size += 9;
        }
        if self.ex_slr_mv != 0. {
            my_size += 9;
        }
        if self.msf_mv != 0. {
            my_size += 9;
        }
        if self.slr_mv != 0. {
            my_size += 9;
        }
        if self.final_excess_slr != 0. {
            my_size += 9;
        }
        if self.final_msf != 0. {
            my_size += 9;
        }
        if self.final_slr != 0. {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.as_on_date.is_empty() {
            os.write_string(1, &self.as_on_date)?;
        }
        if self.ndtl_val != 0. {
            os.write_double(2, self.ndtl_val)?;
        }
        if self.total_gsec_bv != 0. {
            os.write_double(3, self.total_gsec_bv)?;
        }
        if self.total_gsec_mv != 0. {
            os.write_double(4, self.total_gsec_mv)?;
        }
        if self.ex_slr_bv != 0. {
            os.write_double(5, self.ex_slr_bv)?;
        }
        if self.msf_bv != 0. {
            os.write_double(6, self.msf_bv)?;
        }
        if self.slr_bv != 0. {
            os.write_double(7, self.slr_bv)?;
        }
        if self.ex_slr_pct_gsec != 0. {
            os.write_double(8, self.ex_slr_pct_gsec)?;
        }
        if self.msf_pct_gsec != 0. {
            os.write_double(9, self.msf_pct_gsec)?;
        }
        if self.slr_pct_gsec != 0. {
            os.write_double(10, self.slr_pct_gsec)?;
        }
        if self.ex_slr_mv != 0. {
            os.write_double(11, self.ex_slr_mv)?;
        }
        if self.msf_mv != 0. {
            os.write_double(12, self.msf_mv)?;
        }
        if self.slr_mv != 0. {
            os.write_double(13, self.slr_mv)?;
        }
        if self.final_excess_slr != 0. {
            os.write_double(14, self.final_excess_slr)?;
        }
        if self.final_msf != 0. {
            os.write_double(15, self.final_msf)?;
        }
        if self.final_slr != 0. {
            os.write_double(16, self.final_slr)?;
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

    fn new() -> OutputAccount {
        OutputAccount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "as_on_date",
                |m: &OutputAccount| { &m.as_on_date },
                |m: &mut OutputAccount| { &mut m.as_on_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ndtl_val",
                |m: &OutputAccount| { &m.ndtl_val },
                |m: &mut OutputAccount| { &mut m.ndtl_val },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "total_gsec_bv",
                |m: &OutputAccount| { &m.total_gsec_bv },
                |m: &mut OutputAccount| { &mut m.total_gsec_bv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "total_gsec_mv",
                |m: &OutputAccount| { &m.total_gsec_mv },
                |m: &mut OutputAccount| { &mut m.total_gsec_mv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ex_slr_bv",
                |m: &OutputAccount| { &m.ex_slr_bv },
                |m: &mut OutputAccount| { &mut m.ex_slr_bv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "msf_bv",
                |m: &OutputAccount| { &m.msf_bv },
                |m: &mut OutputAccount| { &mut m.msf_bv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "slr_bv",
                |m: &OutputAccount| { &m.slr_bv },
                |m: &mut OutputAccount| { &mut m.slr_bv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ex_slr_pct_gsec",
                |m: &OutputAccount| { &m.ex_slr_pct_gsec },
                |m: &mut OutputAccount| { &mut m.ex_slr_pct_gsec },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "msf_pct_gsec",
                |m: &OutputAccount| { &m.msf_pct_gsec },
                |m: &mut OutputAccount| { &mut m.msf_pct_gsec },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "slr_pct_gsec",
                |m: &OutputAccount| { &m.slr_pct_gsec },
                |m: &mut OutputAccount| { &mut m.slr_pct_gsec },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ex_slr_mv",
                |m: &OutputAccount| { &m.ex_slr_mv },
                |m: &mut OutputAccount| { &mut m.ex_slr_mv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "msf_mv",
                |m: &OutputAccount| { &m.msf_mv },
                |m: &mut OutputAccount| { &mut m.msf_mv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "slr_mv",
                |m: &OutputAccount| { &m.slr_mv },
                |m: &mut OutputAccount| { &mut m.slr_mv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "final_excess_slr",
                |m: &OutputAccount| { &m.final_excess_slr },
                |m: &mut OutputAccount| { &mut m.final_excess_slr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "final_msf",
                |m: &OutputAccount| { &m.final_msf },
                |m: &mut OutputAccount| { &mut m.final_msf },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "final_slr",
                |m: &OutputAccount| { &m.final_slr },
                |m: &mut OutputAccount| { &mut m.final_slr },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OutputAccount>(
                "OutputAccount",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OutputAccount {
        static instance: ::protobuf::rt::LazyV2<OutputAccount> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OutputAccount::new)
    }
}

impl ::protobuf::Clear for OutputAccount {
    fn clear(&mut self) {
        self.as_on_date.clear();
        self.ndtl_val = 0.;
        self.total_gsec_bv = 0.;
        self.total_gsec_mv = 0.;
        self.ex_slr_bv = 0.;
        self.msf_bv = 0.;
        self.slr_bv = 0.;
        self.ex_slr_pct_gsec = 0.;
        self.msf_pct_gsec = 0.;
        self.slr_pct_gsec = 0.;
        self.ex_slr_mv = 0.;
        self.msf_mv = 0.;
        self.slr_mv = 0.;
        self.final_excess_slr = 0.;
        self.final_msf = 0.;
        self.final_slr = 0.;
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
    \n\x0cndtl_1.proto\"\xf3\x03\n\rOutputAccount\x12\x1c\n\nas_on_date\x18\
    \x01\x20\x01(\tR\x08asOnDate\x12\x19\n\x08ndtl_val\x18\x02\x20\x01(\x01R\
    \x07ndtlVal\x12\"\n\rtotal_gsec_bv\x18\x03\x20\x01(\x01R\x0btotalGsecBv\
    \x12\"\n\rtotal_gsec_mv\x18\x04\x20\x01(\x01R\x0btotalGsecMv\x12\x1a\n\t\
    ex_slr_bv\x18\x05\x20\x01(\x01R\x07exSlrBv\x12\x15\n\x06msf_bv\x18\x06\
    \x20\x01(\x01R\x05msfBv\x12\x15\n\x06slr_bv\x18\x07\x20\x01(\x01R\x05slr\
    Bv\x12%\n\x0fex_slr_pct_gsec\x18\x08\x20\x01(\x01R\x0cexSlrPctGsec\x12\
    \x20\n\x0cmsf_pct_gsec\x18\t\x20\x01(\x01R\nmsfPctGsec\x12\x20\n\x0cslr_\
    pct_gsec\x18\n\x20\x01(\x01R\nslrPctGsec\x12\x1a\n\tex_slr_mv\x18\x0b\
    \x20\x01(\x01R\x07exSlrMv\x12\x15\n\x06msf_mv\x18\x0c\x20\x01(\x01R\x05m\
    sfMv\x12\x15\n\x06slr_mv\x18\r\x20\x01(\x01R\x05slrMv\x12(\n\x10final_ex\
    cess_slr\x18\x0e\x20\x01(\x01R\x0efinalExcessSlr\x12\x1b\n\tfinal_msf\
    \x18\x0f\x20\x01(\x01R\x08finalMsf\x12\x1b\n\tfinal_slr\x18\x10\x20\x01(\
    \x01R\x08finalSlrb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}