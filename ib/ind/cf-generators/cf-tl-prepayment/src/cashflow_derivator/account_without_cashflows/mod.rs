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
//! Generated file from `ib-cf-td-mov-prepayment.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct AccountWithoutCashflows {
    // message fields
    pub customer_no: ::std::string::String,
    pub cust_acct_no: ::std::string::String,
    pub apprv_date: i64,
    pub lst_fin_date: i64,
    pub actl_mat_date: i64,
    pub closure_amount: f64,
    pub int_rate: f64,
    pub gl_class_code: ::std::string::String,
    pub currency_ind: ::std::string::String,
    pub accnt_live_days: i64,
    pub preclosure_bkt_id: ::std::string::String,
    pub actual_days_mat: i64,
    pub contractual_bkt_id: ::std::string::String,
    pub llg_type: ::std::string::String,
    pub add_dim1: ::std::string::String,
    pub add_dim2: ::std::string::String,
    pub add_dim3: ::std::string::String,
    pub add_dim4: ::std::string::String,
    pub add_dim5: ::std::string::String,
    pub add_dim6: ::std::string::String,
    pub add_dim7: ::std::string::String,
    pub add_dim8: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AccountWithoutCashflows {
    fn default() -> &'a AccountWithoutCashflows {
        <AccountWithoutCashflows as ::protobuf::Message>::default_instance()
    }
}

impl AccountWithoutCashflows {
    pub fn new() -> AccountWithoutCashflows {
        ::std::default::Default::default()
    }

    // string customer_no = 1;


    pub fn get_customer_no(&self) -> &str {
        &self.customer_no
    }
    pub fn clear_customer_no(&mut self) {
        self.customer_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_customer_no(&mut self, v: ::std::string::String) {
        self.customer_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_customer_no(&mut self) -> &mut ::std::string::String {
        &mut self.customer_no
    }

    // Take field
    pub fn take_customer_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.customer_no, ::std::string::String::new())
    }

    // string cust_acct_no = 2;


    pub fn get_cust_acct_no(&self) -> &str {
        &self.cust_acct_no
    }
    pub fn clear_cust_acct_no(&mut self) {
        self.cust_acct_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_acct_no(&mut self, v: ::std::string::String) {
        self.cust_acct_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_acct_no(&mut self) -> &mut ::std::string::String {
        &mut self.cust_acct_no
    }

    // Take field
    pub fn take_cust_acct_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_acct_no, ::std::string::String::new())
    }

    // int64 apprv_date = 3;


    pub fn get_apprv_date(&self) -> i64 {
        self.apprv_date
    }
    pub fn clear_apprv_date(&mut self) {
        self.apprv_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_apprv_date(&mut self, v: i64) {
        self.apprv_date = v;
    }

    // int64 lst_fin_date = 4;


    pub fn get_lst_fin_date(&self) -> i64 {
        self.lst_fin_date
    }
    pub fn clear_lst_fin_date(&mut self) {
        self.lst_fin_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_lst_fin_date(&mut self, v: i64) {
        self.lst_fin_date = v;
    }

    // int64 actl_mat_date = 5;


    pub fn get_actl_mat_date(&self) -> i64 {
        self.actl_mat_date
    }
    pub fn clear_actl_mat_date(&mut self) {
        self.actl_mat_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_actl_mat_date(&mut self, v: i64) {
        self.actl_mat_date = v;
    }

    // double closure_amount = 6;


    pub fn get_closure_amount(&self) -> f64 {
        self.closure_amount
    }
    pub fn clear_closure_amount(&mut self) {
        self.closure_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_closure_amount(&mut self, v: f64) {
        self.closure_amount = v;
    }

    // double int_rate = 7;


    pub fn get_int_rate(&self) -> f64 {
        self.int_rate
    }
    pub fn clear_int_rate(&mut self) {
        self.int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_int_rate(&mut self, v: f64) {
        self.int_rate = v;
    }

    // string gl_class_code = 8;


    pub fn get_gl_class_code(&self) -> &str {
        &self.gl_class_code
    }
    pub fn clear_gl_class_code(&mut self) {
        self.gl_class_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl_class_code(&mut self, v: ::std::string::String) {
        self.gl_class_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl_class_code(&mut self) -> &mut ::std::string::String {
        &mut self.gl_class_code
    }

    // Take field
    pub fn take_gl_class_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl_class_code, ::std::string::String::new())
    }

    // string currency_ind = 9;


    pub fn get_currency_ind(&self) -> &str {
        &self.currency_ind
    }
    pub fn clear_currency_ind(&mut self) {
        self.currency_ind.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency_ind(&mut self, v: ::std::string::String) {
        self.currency_ind = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency_ind(&mut self) -> &mut ::std::string::String {
        &mut self.currency_ind
    }

    // Take field
    pub fn take_currency_ind(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency_ind, ::std::string::String::new())
    }

    // int64 accnt_live_days = 10;


    pub fn get_accnt_live_days(&self) -> i64 {
        self.accnt_live_days
    }
    pub fn clear_accnt_live_days(&mut self) {
        self.accnt_live_days = 0;
    }

    // Param is passed by value, moved
    pub fn set_accnt_live_days(&mut self, v: i64) {
        self.accnt_live_days = v;
    }

    // string preclosure_bkt_id = 11;


    pub fn get_preclosure_bkt_id(&self) -> &str {
        &self.preclosure_bkt_id
    }
    pub fn clear_preclosure_bkt_id(&mut self) {
        self.preclosure_bkt_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_preclosure_bkt_id(&mut self, v: ::std::string::String) {
        self.preclosure_bkt_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preclosure_bkt_id(&mut self) -> &mut ::std::string::String {
        &mut self.preclosure_bkt_id
    }

    // Take field
    pub fn take_preclosure_bkt_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.preclosure_bkt_id, ::std::string::String::new())
    }

    // int64 actual_days_mat = 12;


    pub fn get_actual_days_mat(&self) -> i64 {
        self.actual_days_mat
    }
    pub fn clear_actual_days_mat(&mut self) {
        self.actual_days_mat = 0;
    }

    // Param is passed by value, moved
    pub fn set_actual_days_mat(&mut self, v: i64) {
        self.actual_days_mat = v;
    }

    // string contractual_bkt_id = 13;


    pub fn get_contractual_bkt_id(&self) -> &str {
        &self.contractual_bkt_id
    }
    pub fn clear_contractual_bkt_id(&mut self) {
        self.contractual_bkt_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_contractual_bkt_id(&mut self, v: ::std::string::String) {
        self.contractual_bkt_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contractual_bkt_id(&mut self) -> &mut ::std::string::String {
        &mut self.contractual_bkt_id
    }

    // Take field
    pub fn take_contractual_bkt_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.contractual_bkt_id, ::std::string::String::new())
    }

    // string llg_type = 14;


    pub fn get_llg_type(&self) -> &str {
        &self.llg_type
    }
    pub fn clear_llg_type(&mut self) {
        self.llg_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_llg_type(&mut self, v: ::std::string::String) {
        self.llg_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_llg_type(&mut self) -> &mut ::std::string::String {
        &mut self.llg_type
    }

    // Take field
    pub fn take_llg_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.llg_type, ::std::string::String::new())
    }

    // string add_dim1 = 15;


    pub fn get_add_dim1(&self) -> &str {
        &self.add_dim1
    }
    pub fn clear_add_dim1(&mut self) {
        self.add_dim1.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim1(&mut self, v: ::std::string::String) {
        self.add_dim1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim1(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim1
    }

    // Take field
    pub fn take_add_dim1(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim1, ::std::string::String::new())
    }

    // string add_dim2 = 16;


    pub fn get_add_dim2(&self) -> &str {
        &self.add_dim2
    }
    pub fn clear_add_dim2(&mut self) {
        self.add_dim2.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim2(&mut self, v: ::std::string::String) {
        self.add_dim2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim2(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim2
    }

    // Take field
    pub fn take_add_dim2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim2, ::std::string::String::new())
    }

    // string add_dim3 = 17;


    pub fn get_add_dim3(&self) -> &str {
        &self.add_dim3
    }
    pub fn clear_add_dim3(&mut self) {
        self.add_dim3.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim3(&mut self, v: ::std::string::String) {
        self.add_dim3 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim3(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim3
    }

    // Take field
    pub fn take_add_dim3(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim3, ::std::string::String::new())
    }

    // string add_dim4 = 18;


    pub fn get_add_dim4(&self) -> &str {
        &self.add_dim4
    }
    pub fn clear_add_dim4(&mut self) {
        self.add_dim4.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim4(&mut self, v: ::std::string::String) {
        self.add_dim4 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim4(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim4
    }

    // Take field
    pub fn take_add_dim4(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim4, ::std::string::String::new())
    }

    // string add_dim5 = 19;


    pub fn get_add_dim5(&self) -> &str {
        &self.add_dim5
    }
    pub fn clear_add_dim5(&mut self) {
        self.add_dim5.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim5(&mut self, v: ::std::string::String) {
        self.add_dim5 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim5(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim5
    }

    // Take field
    pub fn take_add_dim5(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim5, ::std::string::String::new())
    }

    // string add_dim6 = 20;


    pub fn get_add_dim6(&self) -> &str {
        &self.add_dim6
    }
    pub fn clear_add_dim6(&mut self) {
        self.add_dim6.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim6(&mut self, v: ::std::string::String) {
        self.add_dim6 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim6(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim6
    }

    // Take field
    pub fn take_add_dim6(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim6, ::std::string::String::new())
    }

    // string add_dim7 = 21;


    pub fn get_add_dim7(&self) -> &str {
        &self.add_dim7
    }
    pub fn clear_add_dim7(&mut self) {
        self.add_dim7.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim7(&mut self, v: ::std::string::String) {
        self.add_dim7 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim7(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim7
    }

    // Take field
    pub fn take_add_dim7(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim7, ::std::string::String::new())
    }

    // string add_dim8 = 22;


    pub fn get_add_dim8(&self) -> &str {
        &self.add_dim8
    }
    pub fn clear_add_dim8(&mut self) {
        self.add_dim8.clear();
    }

    // Param is passed by value, moved
    pub fn set_add_dim8(&mut self, v: ::std::string::String) {
        self.add_dim8 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_dim8(&mut self) -> &mut ::std::string::String {
        &mut self.add_dim8
    }

    // Take field
    pub fn take_add_dim8(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.add_dim8, ::std::string::String::new())
    }
}

impl ::protobuf::Message for AccountWithoutCashflows {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.customer_no)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_acct_no)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.apprv_date = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lst_fin_date = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.actl_mat_date = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.closure_amount = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_rate = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_class_code)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency_ind)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.accnt_live_days = tmp;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.preclosure_bkt_id)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.actual_days_mat = tmp;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.contractual_bkt_id)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.llg_type)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim1)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim2)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim3)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim4)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim5)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim6)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim7)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.add_dim8)?;
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
        if !self.customer_no.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.customer_no);
        }
        if !self.cust_acct_no.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cust_acct_no);
        }
        if self.apprv_date != 0 {
            my_size += ::protobuf::rt::value_size(3, self.apprv_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lst_fin_date != 0 {
            my_size += ::protobuf::rt::value_size(4, self.lst_fin_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.actl_mat_date != 0 {
            my_size += ::protobuf::rt::value_size(5, self.actl_mat_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.closure_amount != 0. {
            my_size += 9;
        }
        if self.int_rate != 0. {
            my_size += 9;
        }
        if !self.gl_class_code.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.gl_class_code);
        }
        if !self.currency_ind.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.currency_ind);
        }
        if self.accnt_live_days != 0 {
            my_size += ::protobuf::rt::value_size(10, self.accnt_live_days, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.preclosure_bkt_id.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.preclosure_bkt_id);
        }
        if self.actual_days_mat != 0 {
            my_size += ::protobuf::rt::value_size(12, self.actual_days_mat, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.contractual_bkt_id.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.contractual_bkt_id);
        }
        if !self.llg_type.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.llg_type);
        }
        if !self.add_dim1.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.add_dim1);
        }
        if !self.add_dim2.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.add_dim2);
        }
        if !self.add_dim3.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.add_dim3);
        }
        if !self.add_dim4.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.add_dim4);
        }
        if !self.add_dim5.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.add_dim5);
        }
        if !self.add_dim6.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.add_dim6);
        }
        if !self.add_dim7.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.add_dim7);
        }
        if !self.add_dim8.is_empty() {
            my_size += ::protobuf::rt::string_size(22, &self.add_dim8);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.customer_no.is_empty() {
            os.write_string(1, &self.customer_no)?;
        }
        if !self.cust_acct_no.is_empty() {
            os.write_string(2, &self.cust_acct_no)?;
        }
        if self.apprv_date != 0 {
            os.write_int64(3, self.apprv_date)?;
        }
        if self.lst_fin_date != 0 {
            os.write_int64(4, self.lst_fin_date)?;
        }
        if self.actl_mat_date != 0 {
            os.write_int64(5, self.actl_mat_date)?;
        }
        if self.closure_amount != 0. {
            os.write_double(6, self.closure_amount)?;
        }
        if self.int_rate != 0. {
            os.write_double(7, self.int_rate)?;
        }
        if !self.gl_class_code.is_empty() {
            os.write_string(8, &self.gl_class_code)?;
        }
        if !self.currency_ind.is_empty() {
            os.write_string(9, &self.currency_ind)?;
        }
        if self.accnt_live_days != 0 {
            os.write_int64(10, self.accnt_live_days)?;
        }
        if !self.preclosure_bkt_id.is_empty() {
            os.write_string(11, &self.preclosure_bkt_id)?;
        }
        if self.actual_days_mat != 0 {
            os.write_int64(12, self.actual_days_mat)?;
        }
        if !self.contractual_bkt_id.is_empty() {
            os.write_string(13, &self.contractual_bkt_id)?;
        }
        if !self.llg_type.is_empty() {
            os.write_string(14, &self.llg_type)?;
        }
        if !self.add_dim1.is_empty() {
            os.write_string(15, &self.add_dim1)?;
        }
        if !self.add_dim2.is_empty() {
            os.write_string(16, &self.add_dim2)?;
        }
        if !self.add_dim3.is_empty() {
            os.write_string(17, &self.add_dim3)?;
        }
        if !self.add_dim4.is_empty() {
            os.write_string(18, &self.add_dim4)?;
        }
        if !self.add_dim5.is_empty() {
            os.write_string(19, &self.add_dim5)?;
        }
        if !self.add_dim6.is_empty() {
            os.write_string(20, &self.add_dim6)?;
        }
        if !self.add_dim7.is_empty() {
            os.write_string(21, &self.add_dim7)?;
        }
        if !self.add_dim8.is_empty() {
            os.write_string(22, &self.add_dim8)?;
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

    fn new() -> AccountWithoutCashflows {
        AccountWithoutCashflows::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "customer_no",
                |m: &AccountWithoutCashflows| { &m.customer_no },
                |m: &mut AccountWithoutCashflows| { &mut m.customer_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_acct_no",
                |m: &AccountWithoutCashflows| { &m.cust_acct_no },
                |m: &mut AccountWithoutCashflows| { &mut m.cust_acct_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "apprv_date",
                |m: &AccountWithoutCashflows| { &m.apprv_date },
                |m: &mut AccountWithoutCashflows| { &mut m.apprv_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "lst_fin_date",
                |m: &AccountWithoutCashflows| { &m.lst_fin_date },
                |m: &mut AccountWithoutCashflows| { &mut m.lst_fin_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "actl_mat_date",
                |m: &AccountWithoutCashflows| { &m.actl_mat_date },
                |m: &mut AccountWithoutCashflows| { &mut m.actl_mat_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "closure_amount",
                |m: &AccountWithoutCashflows| { &m.closure_amount },
                |m: &mut AccountWithoutCashflows| { &mut m.closure_amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_rate",
                |m: &AccountWithoutCashflows| { &m.int_rate },
                |m: &mut AccountWithoutCashflows| { &mut m.int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "gl_class_code",
                |m: &AccountWithoutCashflows| { &m.gl_class_code },
                |m: &mut AccountWithoutCashflows| { &mut m.gl_class_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency_ind",
                |m: &AccountWithoutCashflows| { &m.currency_ind },
                |m: &mut AccountWithoutCashflows| { &mut m.currency_ind },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "accnt_live_days",
                |m: &AccountWithoutCashflows| { &m.accnt_live_days },
                |m: &mut AccountWithoutCashflows| { &mut m.accnt_live_days },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "preclosure_bkt_id",
                |m: &AccountWithoutCashflows| { &m.preclosure_bkt_id },
                |m: &mut AccountWithoutCashflows| { &mut m.preclosure_bkt_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "actual_days_mat",
                |m: &AccountWithoutCashflows| { &m.actual_days_mat },
                |m: &mut AccountWithoutCashflows| { &mut m.actual_days_mat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "contractual_bkt_id",
                |m: &AccountWithoutCashflows| { &m.contractual_bkt_id },
                |m: &mut AccountWithoutCashflows| { &mut m.contractual_bkt_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "llg_type",
                |m: &AccountWithoutCashflows| { &m.llg_type },
                |m: &mut AccountWithoutCashflows| { &mut m.llg_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim1",
                |m: &AccountWithoutCashflows| { &m.add_dim1 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim2",
                |m: &AccountWithoutCashflows| { &m.add_dim2 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim3",
                |m: &AccountWithoutCashflows| { &m.add_dim3 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim4",
                |m: &AccountWithoutCashflows| { &m.add_dim4 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim4 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim5",
                |m: &AccountWithoutCashflows| { &m.add_dim5 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim5 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim6",
                |m: &AccountWithoutCashflows| { &m.add_dim6 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim6 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim7",
                |m: &AccountWithoutCashflows| { &m.add_dim7 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim7 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "add_dim8",
                |m: &AccountWithoutCashflows| { &m.add_dim8 },
                |m: &mut AccountWithoutCashflows| { &mut m.add_dim8 },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AccountWithoutCashflows>(
                "AccountWithoutCashflows",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AccountWithoutCashflows {
        static instance: ::protobuf::rt::LazyV2<AccountWithoutCashflows> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AccountWithoutCashflows::new)
    }
}

impl ::protobuf::Clear for AccountWithoutCashflows {
    fn clear(&mut self) {
        self.customer_no.clear();
        self.cust_acct_no.clear();
        self.apprv_date = 0;
        self.lst_fin_date = 0;
        self.actl_mat_date = 0;
        self.closure_amount = 0.;
        self.int_rate = 0.;
        self.gl_class_code.clear();
        self.currency_ind.clear();
        self.accnt_live_days = 0;
        self.preclosure_bkt_id.clear();
        self.actual_days_mat = 0;
        self.contractual_bkt_id.clear();
        self.llg_type.clear();
        self.add_dim1.clear();
        self.add_dim2.clear();
        self.add_dim3.clear();
        self.add_dim4.clear();
        self.add_dim5.clear();
        self.add_dim6.clear();
        self.add_dim7.clear();
        self.add_dim8.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountWithoutCashflows {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountWithoutCashflows {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dib-cf-td-mov-prepayment.proto\"\xe7\x05\n\x17AccountWithoutCashflo\
    ws\x12\x1f\n\x0bcustomer_no\x18\x01\x20\x01(\tR\ncustomerNo\x12\x20\n\
    \x0ccust_acct_no\x18\x02\x20\x01(\tR\ncustAcctNo\x12\x1d\n\napprv_date\
    \x18\x03\x20\x01(\x03R\tapprvDate\x12\x20\n\x0clst_fin_date\x18\x04\x20\
    \x01(\x03R\nlstFinDate\x12\"\n\ractl_mat_date\x18\x05\x20\x01(\x03R\x0ba\
    ctlMatDate\x12%\n\x0eclosure_amount\x18\x06\x20\x01(\x01R\rclosureAmount\
    \x12\x19\n\x08int_rate\x18\x07\x20\x01(\x01R\x07intRate\x12\"\n\rgl_clas\
    s_code\x18\x08\x20\x01(\tR\x0bglClassCode\x12!\n\x0ccurrency_ind\x18\t\
    \x20\x01(\tR\x0bcurrencyInd\x12&\n\x0faccnt_live_days\x18\n\x20\x01(\x03\
    R\raccntLiveDays\x12*\n\x11preclosure_bkt_id\x18\x0b\x20\x01(\tR\x0fprec\
    losureBktId\x12&\n\x0factual_days_mat\x18\x0c\x20\x01(\x03R\ractualDaysM\
    at\x12,\n\x12contractual_bkt_id\x18\r\x20\x01(\tR\x10contractualBktId\
    \x12\x19\n\x08llg_type\x18\x0e\x20\x01(\tR\x07llgType\x12\x19\n\x08add_d\
    im1\x18\x0f\x20\x01(\tR\x07addDim1\x12\x19\n\x08add_dim2\x18\x10\x20\x01\
    (\tR\x07addDim2\x12\x19\n\x08add_dim3\x18\x11\x20\x01(\tR\x07addDim3\x12\
    \x19\n\x08add_dim4\x18\x12\x20\x01(\tR\x07addDim4\x12\x19\n\x08add_dim5\
    \x18\x13\x20\x01(\tR\x07addDim5\x12\x19\n\x08add_dim6\x18\x14\x20\x01(\t\
    R\x07addDim6\x12\x19\n\x08add_dim7\x18\x15\x20\x01(\tR\x07addDim7\x12\
    \x19\n\x08add_dim8\x18\x16\x20\x01(\tR\x07addDim8b\x06proto3\
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