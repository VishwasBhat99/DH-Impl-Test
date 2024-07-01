// This file is generated by rust-protobuf 2.25.1. Do not edit
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
//! Generated file from `hdfc_rd.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct Cashflow {
    // message fields
    pub int_amt: f64,
    pub prin_amt: f64,
    pub date: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Cashflow {
    fn default() -> &'a Cashflow {
        <Cashflow as ::protobuf::Message>::default_instance()
    }
}

impl Cashflow {
    pub fn new() -> Cashflow {
        ::std::default::Default::default()
    }

    // double int_amt = 1;


    pub fn get_int_amt(&self) -> f64 {
        self.int_amt
    }
    pub fn clear_int_amt(&mut self) {
        self.int_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_int_amt(&mut self, v: f64) {
        self.int_amt = v;
    }

    // double prin_amt = 2;


    pub fn get_prin_amt(&self) -> f64 {
        self.prin_amt
    }
    pub fn clear_prin_amt(&mut self) {
        self.prin_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_prin_amt(&mut self, v: f64) {
        self.prin_amt = v;
    }

    // int64 date = 3;


    pub fn get_date(&self) -> i64 {
        self.date
    }
    pub fn clear_date(&mut self) {
        self.date = 0;
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: i64) {
        self.date = v;
    }
}

impl ::protobuf::Message for Cashflow {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_amt = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.prin_amt = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.date = tmp;
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
        if self.int_amt != 0. {
            my_size += 9;
        }
        if self.prin_amt != 0. {
            my_size += 9;
        }
        if self.date != 0 {
            my_size += ::protobuf::rt::value_size(3, self.date, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.int_amt != 0. {
            os.write_double(1, self.int_amt)?;
        }
        if self.prin_amt != 0. {
            os.write_double(2, self.prin_amt)?;
        }
        if self.date != 0 {
            os.write_int64(3, self.date)?;
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

    fn new() -> Cashflow {
        Cashflow::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_amt",
                |m: &Cashflow| { &m.int_amt },
                |m: &mut Cashflow| { &mut m.int_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "prin_amt",
                |m: &Cashflow| { &m.prin_amt },
                |m: &mut Cashflow| { &mut m.prin_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "date",
                |m: &Cashflow| { &m.date },
                |m: &mut Cashflow| { &mut m.date },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Cashflow>(
                "Cashflow",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Cashflow {
        static instance: ::protobuf::rt::LazyV2<Cashflow> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Cashflow::new)
    }
}

impl ::protobuf::Clear for Cashflow {
    fn clear(&mut self) {
        self.int_amt = 0.;
        self.prin_amt = 0.;
        self.date = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cashflow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cashflow {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountWithCashflows {
    // message fields
    pub acc_no: ::std::string::String,
    pub cntr_party: ::std::string::String,
    pub clients_name: ::std::string::String,
    pub ccy: ::std::string::String,
    pub gl_no: i64,
    pub amt: f64,
    pub int_rt: f64,
    pub st_dt: i64,
    pub mat_dt: i64,
    pub alm_line: ::std::string::String,
    pub div: ::std::string::String,
    pub cust_id: i64,
    pub prod_code: ::std::string::String,
    pub cod_mis_comp_1: ::std::string::String,
    pub dat_val: i64,
    pub alm_concat: ::std::string::String,
    pub tot_int_amt: f64,
    pub tot_prin_amt: f64,
    pub cashflows: ::protobuf::RepeatedField<Cashflow>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AccountWithCashflows {
    fn default() -> &'a AccountWithCashflows {
        <AccountWithCashflows as ::protobuf::Message>::default_instance()
    }
}

impl AccountWithCashflows {
    pub fn new() -> AccountWithCashflows {
        ::std::default::Default::default()
    }

    // string acc_no = 1;


    pub fn get_acc_no(&self) -> &str {
        &self.acc_no
    }
    pub fn clear_acc_no(&mut self) {
        self.acc_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_acc_no(&mut self, v: ::std::string::String) {
        self.acc_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acc_no(&mut self) -> &mut ::std::string::String {
        &mut self.acc_no
    }

    // Take field
    pub fn take_acc_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.acc_no, ::std::string::String::new())
    }

    // string cntr_party = 2;


    pub fn get_cntr_party(&self) -> &str {
        &self.cntr_party
    }
    pub fn clear_cntr_party(&mut self) {
        self.cntr_party.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntr_party(&mut self, v: ::std::string::String) {
        self.cntr_party = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntr_party(&mut self) -> &mut ::std::string::String {
        &mut self.cntr_party
    }

    // Take field
    pub fn take_cntr_party(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntr_party, ::std::string::String::new())
    }

    // string clients_name = 3;


    pub fn get_clients_name(&self) -> &str {
        &self.clients_name
    }
    pub fn clear_clients_name(&mut self) {
        self.clients_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_clients_name(&mut self, v: ::std::string::String) {
        self.clients_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clients_name(&mut self) -> &mut ::std::string::String {
        &mut self.clients_name
    }

    // Take field
    pub fn take_clients_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clients_name, ::std::string::String::new())
    }

    // string ccy = 4;


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

    // int64 gl_no = 5;


    pub fn get_gl_no(&self) -> i64 {
        self.gl_no
    }
    pub fn clear_gl_no(&mut self) {
        self.gl_no = 0;
    }

    // Param is passed by value, moved
    pub fn set_gl_no(&mut self, v: i64) {
        self.gl_no = v;
    }

    // double amt = 6;


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

    // double int_rt = 7;


    pub fn get_int_rt(&self) -> f64 {
        self.int_rt
    }
    pub fn clear_int_rt(&mut self) {
        self.int_rt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_int_rt(&mut self, v: f64) {
        self.int_rt = v;
    }

    // int64 st_dt = 8;


    pub fn get_st_dt(&self) -> i64 {
        self.st_dt
    }
    pub fn clear_st_dt(&mut self) {
        self.st_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_st_dt(&mut self, v: i64) {
        self.st_dt = v;
    }

    // int64 mat_dt = 9;


    pub fn get_mat_dt(&self) -> i64 {
        self.mat_dt
    }
    pub fn clear_mat_dt(&mut self) {
        self.mat_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_mat_dt(&mut self, v: i64) {
        self.mat_dt = v;
    }

    // string alm_line = 10;


    pub fn get_alm_line(&self) -> &str {
        &self.alm_line
    }
    pub fn clear_alm_line(&mut self) {
        self.alm_line.clear();
    }

    // Param is passed by value, moved
    pub fn set_alm_line(&mut self, v: ::std::string::String) {
        self.alm_line = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alm_line(&mut self) -> &mut ::std::string::String {
        &mut self.alm_line
    }

    // Take field
    pub fn take_alm_line(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alm_line, ::std::string::String::new())
    }

    // string div = 11;


    pub fn get_div(&self) -> &str {
        &self.div
    }
    pub fn clear_div(&mut self) {
        self.div.clear();
    }

    // Param is passed by value, moved
    pub fn set_div(&mut self, v: ::std::string::String) {
        self.div = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_div(&mut self) -> &mut ::std::string::String {
        &mut self.div
    }

    // Take field
    pub fn take_div(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.div, ::std::string::String::new())
    }

    // int64 cust_id = 12;


    pub fn get_cust_id(&self) -> i64 {
        self.cust_id
    }
    pub fn clear_cust_id(&mut self) {
        self.cust_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cust_id(&mut self, v: i64) {
        self.cust_id = v;
    }

    // string prod_code = 13;


    pub fn get_prod_code(&self) -> &str {
        &self.prod_code
    }
    pub fn clear_prod_code(&mut self) {
        self.prod_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_prod_code(&mut self, v: ::std::string::String) {
        self.prod_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prod_code(&mut self) -> &mut ::std::string::String {
        &mut self.prod_code
    }

    // Take field
    pub fn take_prod_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.prod_code, ::std::string::String::new())
    }

    // string cod_mis_comp_1 = 14;


    pub fn get_cod_mis_comp_1(&self) -> &str {
        &self.cod_mis_comp_1
    }
    pub fn clear_cod_mis_comp_1(&mut self) {
        self.cod_mis_comp_1.clear();
    }

    // Param is passed by value, moved
    pub fn set_cod_mis_comp_1(&mut self, v: ::std::string::String) {
        self.cod_mis_comp_1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cod_mis_comp_1(&mut self) -> &mut ::std::string::String {
        &mut self.cod_mis_comp_1
    }

    // Take field
    pub fn take_cod_mis_comp_1(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cod_mis_comp_1, ::std::string::String::new())
    }

    // int64 dat_val = 15;


    pub fn get_dat_val(&self) -> i64 {
        self.dat_val
    }
    pub fn clear_dat_val(&mut self) {
        self.dat_val = 0;
    }

    // Param is passed by value, moved
    pub fn set_dat_val(&mut self, v: i64) {
        self.dat_val = v;
    }

    // string alm_concat = 16;


    pub fn get_alm_concat(&self) -> &str {
        &self.alm_concat
    }
    pub fn clear_alm_concat(&mut self) {
        self.alm_concat.clear();
    }

    // Param is passed by value, moved
    pub fn set_alm_concat(&mut self, v: ::std::string::String) {
        self.alm_concat = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alm_concat(&mut self) -> &mut ::std::string::String {
        &mut self.alm_concat
    }

    // Take field
    pub fn take_alm_concat(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alm_concat, ::std::string::String::new())
    }

    // double tot_int_amt = 17;


    pub fn get_tot_int_amt(&self) -> f64 {
        self.tot_int_amt
    }
    pub fn clear_tot_int_amt(&mut self) {
        self.tot_int_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tot_int_amt(&mut self, v: f64) {
        self.tot_int_amt = v;
    }

    // double tot_prin_amt = 18;


    pub fn get_tot_prin_amt(&self) -> f64 {
        self.tot_prin_amt
    }
    pub fn clear_tot_prin_amt(&mut self) {
        self.tot_prin_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tot_prin_amt(&mut self, v: f64) {
        self.tot_prin_amt = v;
    }

    // repeated .Cashflow cashflows = 19;


    pub fn get_cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }
    pub fn clear_cashflows(&mut self) {
        self.cashflows.clear();
    }

    // Param is passed by value, moved
    pub fn set_cashflows(&mut self, v: ::protobuf::RepeatedField<Cashflow>) {
        self.cashflows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cashflows(&mut self) -> &mut ::protobuf::RepeatedField<Cashflow> {
        &mut self.cashflows
    }

    // Take field
    pub fn take_cashflows(&mut self) -> ::protobuf::RepeatedField<Cashflow> {
        ::std::mem::replace(&mut self.cashflows, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for AccountWithCashflows {
    fn is_initialized(&self) -> bool {
        for v in &self.cashflows {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.acc_no)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntr_party)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clients_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gl_no = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_rt = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.st_dt = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_dt = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.div)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.cust_id = tmp;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prod_code)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cod_mis_comp_1)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.dat_val = tmp;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_concat)?;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_int_amt = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_prin_amt = tmp;
                },
                19 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cashflows)?;
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
        if !self.acc_no.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.acc_no);
        }
        if !self.cntr_party.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cntr_party);
        }
        if !self.clients_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.clients_name);
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.ccy);
        }
        if self.gl_no != 0 {
            my_size += ::protobuf::rt::value_size(5, self.gl_no, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.amt != 0. {
            my_size += 9;
        }
        if self.int_rt != 0. {
            my_size += 9;
        }
        if self.st_dt != 0 {
            my_size += ::protobuf::rt::value_size(8, self.st_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mat_dt != 0 {
            my_size += ::protobuf::rt::value_size(9, self.mat_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.alm_line);
        }
        if !self.div.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.div);
        }
        if self.cust_id != 0 {
            my_size += ::protobuf::rt::value_size(12, self.cust_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.prod_code.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.prod_code);
        }
        if !self.cod_mis_comp_1.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.cod_mis_comp_1);
        }
        if self.dat_val != 0 {
            my_size += ::protobuf::rt::value_size(15, self.dat_val, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.alm_concat.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.alm_concat);
        }
        if self.tot_int_amt != 0. {
            my_size += 10;
        }
        if self.tot_prin_amt != 0. {
            my_size += 10;
        }
        for value in &self.cashflows {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.acc_no.is_empty() {
            os.write_string(1, &self.acc_no)?;
        }
        if !self.cntr_party.is_empty() {
            os.write_string(2, &self.cntr_party)?;
        }
        if !self.clients_name.is_empty() {
            os.write_string(3, &self.clients_name)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(4, &self.ccy)?;
        }
        if self.gl_no != 0 {
            os.write_int64(5, self.gl_no)?;
        }
        if self.amt != 0. {
            os.write_double(6, self.amt)?;
        }
        if self.int_rt != 0. {
            os.write_double(7, self.int_rt)?;
        }
        if self.st_dt != 0 {
            os.write_int64(8, self.st_dt)?;
        }
        if self.mat_dt != 0 {
            os.write_int64(9, self.mat_dt)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(10, &self.alm_line)?;
        }
        if !self.div.is_empty() {
            os.write_string(11, &self.div)?;
        }
        if self.cust_id != 0 {
            os.write_int64(12, self.cust_id)?;
        }
        if !self.prod_code.is_empty() {
            os.write_string(13, &self.prod_code)?;
        }
        if !self.cod_mis_comp_1.is_empty() {
            os.write_string(14, &self.cod_mis_comp_1)?;
        }
        if self.dat_val != 0 {
            os.write_int64(15, self.dat_val)?;
        }
        if !self.alm_concat.is_empty() {
            os.write_string(16, &self.alm_concat)?;
        }
        if self.tot_int_amt != 0. {
            os.write_double(17, self.tot_int_amt)?;
        }
        if self.tot_prin_amt != 0. {
            os.write_double(18, self.tot_prin_amt)?;
        }
        for v in &self.cashflows {
            os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> AccountWithCashflows {
        AccountWithCashflows::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "acc_no",
                |m: &AccountWithCashflows| { &m.acc_no },
                |m: &mut AccountWithCashflows| { &mut m.acc_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntr_party",
                |m: &AccountWithCashflows| { &m.cntr_party },
                |m: &mut AccountWithCashflows| { &mut m.cntr_party },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "clients_name",
                |m: &AccountWithCashflows| { &m.clients_name },
                |m: &mut AccountWithCashflows| { &mut m.clients_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ccy",
                |m: &AccountWithCashflows| { &m.ccy },
                |m: &mut AccountWithCashflows| { &mut m.ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "gl_no",
                |m: &AccountWithCashflows| { &m.gl_no },
                |m: &mut AccountWithCashflows| { &mut m.gl_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "amt",
                |m: &AccountWithCashflows| { &m.amt },
                |m: &mut AccountWithCashflows| { &mut m.amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_rt",
                |m: &AccountWithCashflows| { &m.int_rt },
                |m: &mut AccountWithCashflows| { &mut m.int_rt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "st_dt",
                |m: &AccountWithCashflows| { &m.st_dt },
                |m: &mut AccountWithCashflows| { &mut m.st_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mat_dt",
                |m: &AccountWithCashflows| { &m.mat_dt },
                |m: &mut AccountWithCashflows| { &mut m.mat_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &AccountWithCashflows| { &m.alm_line },
                |m: &mut AccountWithCashflows| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "div",
                |m: &AccountWithCashflows| { &m.div },
                |m: &mut AccountWithCashflows| { &mut m.div },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "cust_id",
                |m: &AccountWithCashflows| { &m.cust_id },
                |m: &mut AccountWithCashflows| { &mut m.cust_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "prod_code",
                |m: &AccountWithCashflows| { &m.prod_code },
                |m: &mut AccountWithCashflows| { &mut m.prod_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cod_mis_comp_1",
                |m: &AccountWithCashflows| { &m.cod_mis_comp_1 },
                |m: &mut AccountWithCashflows| { &mut m.cod_mis_comp_1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "dat_val",
                |m: &AccountWithCashflows| { &m.dat_val },
                |m: &mut AccountWithCashflows| { &mut m.dat_val },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_concat",
                |m: &AccountWithCashflows| { &m.alm_concat },
                |m: &mut AccountWithCashflows| { &mut m.alm_concat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_int_amt",
                |m: &AccountWithCashflows| { &m.tot_int_amt },
                |m: &mut AccountWithCashflows| { &mut m.tot_int_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_prin_amt",
                |m: &AccountWithCashflows| { &m.tot_prin_amt },
                |m: &mut AccountWithCashflows| { &mut m.tot_prin_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cashflow>>(
                "cashflows",
                |m: &AccountWithCashflows| { &m.cashflows },
                |m: &mut AccountWithCashflows| { &mut m.cashflows },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AccountWithCashflows>(
                "AccountWithCashflows",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AccountWithCashflows {
        static instance: ::protobuf::rt::LazyV2<AccountWithCashflows> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AccountWithCashflows::new)
    }
}

impl ::protobuf::Clear for AccountWithCashflows {
    fn clear(&mut self) {
        self.acc_no.clear();
        self.cntr_party.clear();
        self.clients_name.clear();
        self.ccy.clear();
        self.gl_no = 0;
        self.amt = 0.;
        self.int_rt = 0.;
        self.st_dt = 0;
        self.mat_dt = 0;
        self.alm_line.clear();
        self.div.clear();
        self.cust_id = 0;
        self.prod_code.clear();
        self.cod_mis_comp_1.clear();
        self.dat_val = 0;
        self.alm_concat.clear();
        self.tot_int_amt = 0.;
        self.tot_prin_amt = 0.;
        self.cashflows.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountWithCashflows {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountWithCashflows {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rhdfc_rd.proto\"R\n\x08Cashflow\x12\x17\n\x07int_amt\x18\x01\x20\x01(\
    \x01R\x06intAmt\x12\x19\n\x08prin_amt\x18\x02\x20\x01(\x01R\x07prinAmt\
    \x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\"\x96\x04\n\x14AccountW\
    ithCashflows\x12\x15\n\x06acc_no\x18\x01\x20\x01(\tR\x05accNo\x12\x1d\n\
    \ncntr_party\x18\x02\x20\x01(\tR\tcntrParty\x12!\n\x0cclients_name\x18\
    \x03\x20\x01(\tR\x0bclientsName\x12\x10\n\x03ccy\x18\x04\x20\x01(\tR\x03\
    ccy\x12\x13\n\x05gl_no\x18\x05\x20\x01(\x03R\x04glNo\x12\x10\n\x03amt\
    \x18\x06\x20\x01(\x01R\x03amt\x12\x15\n\x06int_rt\x18\x07\x20\x01(\x01R\
    \x05intRt\x12\x13\n\x05st_dt\x18\x08\x20\x01(\x03R\x04stDt\x12\x15\n\x06\
    mat_dt\x18\t\x20\x01(\x03R\x05matDt\x12\x19\n\x08alm_line\x18\n\x20\x01(\
    \tR\x07almLine\x12\x10\n\x03div\x18\x0b\x20\x01(\tR\x03div\x12\x17\n\x07\
    cust_id\x18\x0c\x20\x01(\x03R\x06custId\x12\x1b\n\tprod_code\x18\r\x20\
    \x01(\tR\x08prodCode\x12#\n\x0ecod_mis_comp_1\x18\x0e\x20\x01(\tR\x0bcod\
    MisComp1\x12\x17\n\x07dat_val\x18\x0f\x20\x01(\x03R\x06datVal\x12\x1d\n\
    \nalm_concat\x18\x10\x20\x01(\tR\talmConcat\x12\x1e\n\x0btot_int_amt\x18\
    \x11\x20\x01(\x01R\ttotIntAmt\x12\x20\n\x0ctot_prin_amt\x18\x12\x20\x01(\
    \x01R\ntotPrinAmt\x12'\n\tcashflows\x18\x13\x20\x03(\x0b2\t.CashflowR\tc\
    ashflowsb\x06proto3\
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
