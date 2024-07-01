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

#[derive(PartialEq,Clone,Default)]
pub struct Cashflow {
    // message fields
    pub interest_amount: f64,
    pub principal_amount: f64,
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

    // double interest_amount = 1;


    pub fn get_interest_amount(&self) -> f64 {
        self.interest_amount
    }
    pub fn clear_interest_amount(&mut self) {
        self.interest_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_interest_amount(&mut self, v: f64) {
        self.interest_amount = v;
    }

    // double principal_amount = 2;


    pub fn get_principal_amount(&self) -> f64 {
        self.principal_amount
    }
    pub fn clear_principal_amount(&mut self) {
        self.principal_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_principal_amount(&mut self, v: f64) {
        self.principal_amount = v;
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
                    self.interest_amount = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.principal_amount = tmp;
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
        if self.interest_amount != 0. {
            my_size += 9;
        }
        if self.principal_amount != 0. {
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
        if self.interest_amount != 0. {
            os.write_double(1, self.interest_amount)?;
        }
        if self.principal_amount != 0. {
            os.write_double(2, self.principal_amount)?;
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
                "interest_amount",
                |m: &Cashflow| { &m.interest_amount },
                |m: &mut Cashflow| { &mut m.interest_amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "principal_amount",
                |m: &Cashflow| { &m.principal_amount },
                |m: &mut Cashflow| { &mut m.principal_amount },
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
        self.interest_amount = 0.;
        self.principal_amount = 0.;
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
    pub deal_id: ::std::string::String,
    pub branch: ::std::string::String,
    pub inst_name: ::std::string::String,
    pub lend_borr_typ: ::std::string::String,
    pub typology: ::std::string::String,
    pub usage: ::std::string::String,
    pub sub_typ_borr_lend: ::std::string::String,
    pub cntrprty: ::std::string::String,
    pub crtn_dt: i64,
    pub val_date: i64,
    pub deal_date: i64,
    pub ccy: ::std::string::String,
    pub crnt_deal_amt: f64,
    pub crnt_conv_rt_lcy: f64,
    pub crnt_deal_amt_lcy: f64,
    pub roi: f64,
    pub tenor_days: i64,
    pub mat_dt: i64,
    pub prin_amt: f64,
    pub int_amt: f64,
    pub cf_typ: ::std::string::String,
    pub flow_typ: ::std::string::String,
    pub mat_amt: f64,
    pub dealer_name: ::std::string::String,
    pub nds_ref_no: ::std::string::String,
    pub nxt_fix_dt: i64,
    pub residual_tenor: i64,
    pub nxt_put_dt: i64,
    pub nxt_call_dt: i64,
    pub nxt_int_pay_dt: i64,
    pub int_pay_tenor: i64,
    pub aip_air: f64,
    pub downgrade_clause: ::std::string::String,
    pub avg_monthly_bal: ::std::string::String,
    pub glcode: ::std::string::String,
    pub cntrprty_ctgry_1: ::std::string::String,
    pub cntrprty_ctgry_2: ::std::string::String,
    pub cntrprty_ctgry_3: ::std::string::String,
    pub cntrprty_ctgry_4: ::std::string::String,
    pub int_pay_rec: ::std::string::String,
    pub bckt_days: i64,
    pub cntrprty_ctgry_5: ::std::string::String,
    pub ind_outside_ind: ::std::string::String,
    pub system_gl: ::std::string::String,
    pub alm_concat: ::std::string::String,
    pub div: ::std::string::String,
    pub alm_line: ::std::string::String,
    pub ia_line: ::std::string::String,
    pub tot_int_amt: f64,
    pub tot_prin_amt: f64,
    pub sma_flag: ::std::string::String,
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

    // string deal_id = 1;


    pub fn get_deal_id(&self) -> &str {
        &self.deal_id
    }
    pub fn clear_deal_id(&mut self) {
        self.deal_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_deal_id(&mut self, v: ::std::string::String) {
        self.deal_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deal_id(&mut self) -> &mut ::std::string::String {
        &mut self.deal_id
    }

    // Take field
    pub fn take_deal_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.deal_id, ::std::string::String::new())
    }

    // string branch = 2;


    pub fn get_branch(&self) -> &str {
        &self.branch
    }
    pub fn clear_branch(&mut self) {
        self.branch.clear();
    }

    // Param is passed by value, moved
    pub fn set_branch(&mut self, v: ::std::string::String) {
        self.branch = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_branch(&mut self) -> &mut ::std::string::String {
        &mut self.branch
    }

    // Take field
    pub fn take_branch(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.branch, ::std::string::String::new())
    }

    // string inst_name = 3;


    pub fn get_inst_name(&self) -> &str {
        &self.inst_name
    }
    pub fn clear_inst_name(&mut self) {
        self.inst_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_inst_name(&mut self, v: ::std::string::String) {
        self.inst_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inst_name(&mut self) -> &mut ::std::string::String {
        &mut self.inst_name
    }

    // Take field
    pub fn take_inst_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.inst_name, ::std::string::String::new())
    }

    // string lend_borr_typ = 4;


    pub fn get_lend_borr_typ(&self) -> &str {
        &self.lend_borr_typ
    }
    pub fn clear_lend_borr_typ(&mut self) {
        self.lend_borr_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_lend_borr_typ(&mut self, v: ::std::string::String) {
        self.lend_borr_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lend_borr_typ(&mut self) -> &mut ::std::string::String {
        &mut self.lend_borr_typ
    }

    // Take field
    pub fn take_lend_borr_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lend_borr_typ, ::std::string::String::new())
    }

    // string typology = 5;


    pub fn get_typology(&self) -> &str {
        &self.typology
    }
    pub fn clear_typology(&mut self) {
        self.typology.clear();
    }

    // Param is passed by value, moved
    pub fn set_typology(&mut self, v: ::std::string::String) {
        self.typology = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typology(&mut self) -> &mut ::std::string::String {
        &mut self.typology
    }

    // Take field
    pub fn take_typology(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.typology, ::std::string::String::new())
    }

    // string usage = 6;


    pub fn get_usage(&self) -> &str {
        &self.usage
    }
    pub fn clear_usage(&mut self) {
        self.usage.clear();
    }

    // Param is passed by value, moved
    pub fn set_usage(&mut self, v: ::std::string::String) {
        self.usage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_usage(&mut self) -> &mut ::std::string::String {
        &mut self.usage
    }

    // Take field
    pub fn take_usage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.usage, ::std::string::String::new())
    }

    // string sub_typ_borr_lend = 7;


    pub fn get_sub_typ_borr_lend(&self) -> &str {
        &self.sub_typ_borr_lend
    }
    pub fn clear_sub_typ_borr_lend(&mut self) {
        self.sub_typ_borr_lend.clear();
    }

    // Param is passed by value, moved
    pub fn set_sub_typ_borr_lend(&mut self, v: ::std::string::String) {
        self.sub_typ_borr_lend = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_typ_borr_lend(&mut self) -> &mut ::std::string::String {
        &mut self.sub_typ_borr_lend
    }

    // Take field
    pub fn take_sub_typ_borr_lend(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sub_typ_borr_lend, ::std::string::String::new())
    }

    // string cntrprty = 8;


    pub fn get_cntrprty(&self) -> &str {
        &self.cntrprty
    }
    pub fn clear_cntrprty(&mut self) {
        self.cntrprty.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty(&mut self, v: ::std::string::String) {
        self.cntrprty = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty
    }

    // Take field
    pub fn take_cntrprty(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty, ::std::string::String::new())
    }

    // int64 crtn_dt = 9;


    pub fn get_crtn_dt(&self) -> i64 {
        self.crtn_dt
    }
    pub fn clear_crtn_dt(&mut self) {
        self.crtn_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_crtn_dt(&mut self, v: i64) {
        self.crtn_dt = v;
    }

    // int64 val_date = 10;


    pub fn get_val_date(&self) -> i64 {
        self.val_date
    }
    pub fn clear_val_date(&mut self) {
        self.val_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_val_date(&mut self, v: i64) {
        self.val_date = v;
    }

    // int64 deal_date = 11;


    pub fn get_deal_date(&self) -> i64 {
        self.deal_date
    }
    pub fn clear_deal_date(&mut self) {
        self.deal_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_deal_date(&mut self, v: i64) {
        self.deal_date = v;
    }

    // string ccy = 12;


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

    // double crnt_deal_amt = 13;


    pub fn get_crnt_deal_amt(&self) -> f64 {
        self.crnt_deal_amt
    }
    pub fn clear_crnt_deal_amt(&mut self) {
        self.crnt_deal_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_crnt_deal_amt(&mut self, v: f64) {
        self.crnt_deal_amt = v;
    }

    // double crnt_conv_rt_lcy = 14;


    pub fn get_crnt_conv_rt_lcy(&self) -> f64 {
        self.crnt_conv_rt_lcy
    }
    pub fn clear_crnt_conv_rt_lcy(&mut self) {
        self.crnt_conv_rt_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_crnt_conv_rt_lcy(&mut self, v: f64) {
        self.crnt_conv_rt_lcy = v;
    }

    // double crnt_deal_amt_lcy = 15;


    pub fn get_crnt_deal_amt_lcy(&self) -> f64 {
        self.crnt_deal_amt_lcy
    }
    pub fn clear_crnt_deal_amt_lcy(&mut self) {
        self.crnt_deal_amt_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_crnt_deal_amt_lcy(&mut self, v: f64) {
        self.crnt_deal_amt_lcy = v;
    }

    // double roi = 16;


    pub fn get_roi(&self) -> f64 {
        self.roi
    }
    pub fn clear_roi(&mut self) {
        self.roi = 0.;
    }

    // Param is passed by value, moved
    pub fn set_roi(&mut self, v: f64) {
        self.roi = v;
    }

    // int64 tenor_days = 17;


    pub fn get_tenor_days(&self) -> i64 {
        self.tenor_days
    }
    pub fn clear_tenor_days(&mut self) {
        self.tenor_days = 0;
    }

    // Param is passed by value, moved
    pub fn set_tenor_days(&mut self, v: i64) {
        self.tenor_days = v;
    }

    // int64 mat_dt = 18;


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

    // double prin_amt = 19;


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

    // double int_amt = 20;


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

    // string cf_typ = 21;


    pub fn get_cf_typ(&self) -> &str {
        &self.cf_typ
    }
    pub fn clear_cf_typ(&mut self) {
        self.cf_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf_typ(&mut self, v: ::std::string::String) {
        self.cf_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf_typ(&mut self) -> &mut ::std::string::String {
        &mut self.cf_typ
    }

    // Take field
    pub fn take_cf_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf_typ, ::std::string::String::new())
    }

    // string flow_typ = 22;


    pub fn get_flow_typ(&self) -> &str {
        &self.flow_typ
    }
    pub fn clear_flow_typ(&mut self) {
        self.flow_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_flow_typ(&mut self, v: ::std::string::String) {
        self.flow_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_flow_typ(&mut self) -> &mut ::std::string::String {
        &mut self.flow_typ
    }

    // Take field
    pub fn take_flow_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.flow_typ, ::std::string::String::new())
    }

    // double mat_amt = 23;


    pub fn get_mat_amt(&self) -> f64 {
        self.mat_amt
    }
    pub fn clear_mat_amt(&mut self) {
        self.mat_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_mat_amt(&mut self, v: f64) {
        self.mat_amt = v;
    }

    // string dealer_name = 24;


    pub fn get_dealer_name(&self) -> &str {
        &self.dealer_name
    }
    pub fn clear_dealer_name(&mut self) {
        self.dealer_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_dealer_name(&mut self, v: ::std::string::String) {
        self.dealer_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dealer_name(&mut self) -> &mut ::std::string::String {
        &mut self.dealer_name
    }

    // Take field
    pub fn take_dealer_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.dealer_name, ::std::string::String::new())
    }

    // string nds_ref_no = 25;


    pub fn get_nds_ref_no(&self) -> &str {
        &self.nds_ref_no
    }
    pub fn clear_nds_ref_no(&mut self) {
        self.nds_ref_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_nds_ref_no(&mut self, v: ::std::string::String) {
        self.nds_ref_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nds_ref_no(&mut self) -> &mut ::std::string::String {
        &mut self.nds_ref_no
    }

    // Take field
    pub fn take_nds_ref_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nds_ref_no, ::std::string::String::new())
    }

    // int64 nxt_fix_dt = 26;


    pub fn get_nxt_fix_dt(&self) -> i64 {
        self.nxt_fix_dt
    }
    pub fn clear_nxt_fix_dt(&mut self) {
        self.nxt_fix_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_fix_dt(&mut self, v: i64) {
        self.nxt_fix_dt = v;
    }

    // int64 residual_tenor = 27;


    pub fn get_residual_tenor(&self) -> i64 {
        self.residual_tenor
    }
    pub fn clear_residual_tenor(&mut self) {
        self.residual_tenor = 0;
    }

    // Param is passed by value, moved
    pub fn set_residual_tenor(&mut self, v: i64) {
        self.residual_tenor = v;
    }

    // int64 nxt_put_dt = 28;


    pub fn get_nxt_put_dt(&self) -> i64 {
        self.nxt_put_dt
    }
    pub fn clear_nxt_put_dt(&mut self) {
        self.nxt_put_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_put_dt(&mut self, v: i64) {
        self.nxt_put_dt = v;
    }

    // int64 nxt_call_dt = 29;


    pub fn get_nxt_call_dt(&self) -> i64 {
        self.nxt_call_dt
    }
    pub fn clear_nxt_call_dt(&mut self) {
        self.nxt_call_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_call_dt(&mut self, v: i64) {
        self.nxt_call_dt = v;
    }

    // int64 nxt_int_pay_dt = 30;


    pub fn get_nxt_int_pay_dt(&self) -> i64 {
        self.nxt_int_pay_dt
    }
    pub fn clear_nxt_int_pay_dt(&mut self) {
        self.nxt_int_pay_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_int_pay_dt(&mut self, v: i64) {
        self.nxt_int_pay_dt = v;
    }

    // int64 int_pay_tenor = 31;


    pub fn get_int_pay_tenor(&self) -> i64 {
        self.int_pay_tenor
    }
    pub fn clear_int_pay_tenor(&mut self) {
        self.int_pay_tenor = 0;
    }

    // Param is passed by value, moved
    pub fn set_int_pay_tenor(&mut self, v: i64) {
        self.int_pay_tenor = v;
    }

    // double aip_air = 32;


    pub fn get_aip_air(&self) -> f64 {
        self.aip_air
    }
    pub fn clear_aip_air(&mut self) {
        self.aip_air = 0.;
    }

    // Param is passed by value, moved
    pub fn set_aip_air(&mut self, v: f64) {
        self.aip_air = v;
    }

    // string downgrade_clause = 33;


    pub fn get_downgrade_clause(&self) -> &str {
        &self.downgrade_clause
    }
    pub fn clear_downgrade_clause(&mut self) {
        self.downgrade_clause.clear();
    }

    // Param is passed by value, moved
    pub fn set_downgrade_clause(&mut self, v: ::std::string::String) {
        self.downgrade_clause = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_downgrade_clause(&mut self) -> &mut ::std::string::String {
        &mut self.downgrade_clause
    }

    // Take field
    pub fn take_downgrade_clause(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.downgrade_clause, ::std::string::String::new())
    }

    // string avg_monthly_bal = 34;


    pub fn get_avg_monthly_bal(&self) -> &str {
        &self.avg_monthly_bal
    }
    pub fn clear_avg_monthly_bal(&mut self) {
        self.avg_monthly_bal.clear();
    }

    // Param is passed by value, moved
    pub fn set_avg_monthly_bal(&mut self, v: ::std::string::String) {
        self.avg_monthly_bal = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avg_monthly_bal(&mut self) -> &mut ::std::string::String {
        &mut self.avg_monthly_bal
    }

    // Take field
    pub fn take_avg_monthly_bal(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.avg_monthly_bal, ::std::string::String::new())
    }

    // string glcode = 35;


    pub fn get_glcode(&self) -> &str {
        &self.glcode
    }
    pub fn clear_glcode(&mut self) {
        self.glcode.clear();
    }

    // Param is passed by value, moved
    pub fn set_glcode(&mut self, v: ::std::string::String) {
        self.glcode = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_glcode(&mut self) -> &mut ::std::string::String {
        &mut self.glcode
    }

    // Take field
    pub fn take_glcode(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.glcode, ::std::string::String::new())
    }

    // string cntrprty_ctgry_1 = 36;


    pub fn get_cntrprty_ctgry_1(&self) -> &str {
        &self.cntrprty_ctgry_1
    }
    pub fn clear_cntrprty_ctgry_1(&mut self) {
        self.cntrprty_ctgry_1.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty_ctgry_1(&mut self, v: ::std::string::String) {
        self.cntrprty_ctgry_1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty_ctgry_1(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty_ctgry_1
    }

    // Take field
    pub fn take_cntrprty_ctgry_1(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty_ctgry_1, ::std::string::String::new())
    }

    // string cntrprty_ctgry_2 = 37;


    pub fn get_cntrprty_ctgry_2(&self) -> &str {
        &self.cntrprty_ctgry_2
    }
    pub fn clear_cntrprty_ctgry_2(&mut self) {
        self.cntrprty_ctgry_2.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty_ctgry_2(&mut self, v: ::std::string::String) {
        self.cntrprty_ctgry_2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty_ctgry_2(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty_ctgry_2
    }

    // Take field
    pub fn take_cntrprty_ctgry_2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty_ctgry_2, ::std::string::String::new())
    }

    // string cntrprty_ctgry_3 = 38;


    pub fn get_cntrprty_ctgry_3(&self) -> &str {
        &self.cntrprty_ctgry_3
    }
    pub fn clear_cntrprty_ctgry_3(&mut self) {
        self.cntrprty_ctgry_3.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty_ctgry_3(&mut self, v: ::std::string::String) {
        self.cntrprty_ctgry_3 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty_ctgry_3(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty_ctgry_3
    }

    // Take field
    pub fn take_cntrprty_ctgry_3(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty_ctgry_3, ::std::string::String::new())
    }

    // string cntrprty_ctgry_4 = 39;


    pub fn get_cntrprty_ctgry_4(&self) -> &str {
        &self.cntrprty_ctgry_4
    }
    pub fn clear_cntrprty_ctgry_4(&mut self) {
        self.cntrprty_ctgry_4.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty_ctgry_4(&mut self, v: ::std::string::String) {
        self.cntrprty_ctgry_4 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty_ctgry_4(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty_ctgry_4
    }

    // Take field
    pub fn take_cntrprty_ctgry_4(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty_ctgry_4, ::std::string::String::new())
    }

    // string int_pay_rec = 40;


    pub fn get_int_pay_rec(&self) -> &str {
        &self.int_pay_rec
    }
    pub fn clear_int_pay_rec(&mut self) {
        self.int_pay_rec.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_pay_rec(&mut self, v: ::std::string::String) {
        self.int_pay_rec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int_pay_rec(&mut self) -> &mut ::std::string::String {
        &mut self.int_pay_rec
    }

    // Take field
    pub fn take_int_pay_rec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int_pay_rec, ::std::string::String::new())
    }

    // int64 bckt_days = 41;


    pub fn get_bckt_days(&self) -> i64 {
        self.bckt_days
    }
    pub fn clear_bckt_days(&mut self) {
        self.bckt_days = 0;
    }

    // Param is passed by value, moved
    pub fn set_bckt_days(&mut self, v: i64) {
        self.bckt_days = v;
    }

    // string cntrprty_ctgry_5 = 42;


    pub fn get_cntrprty_ctgry_5(&self) -> &str {
        &self.cntrprty_ctgry_5
    }
    pub fn clear_cntrprty_ctgry_5(&mut self) {
        self.cntrprty_ctgry_5.clear();
    }

    // Param is passed by value, moved
    pub fn set_cntrprty_ctgry_5(&mut self, v: ::std::string::String) {
        self.cntrprty_ctgry_5 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cntrprty_ctgry_5(&mut self) -> &mut ::std::string::String {
        &mut self.cntrprty_ctgry_5
    }

    // Take field
    pub fn take_cntrprty_ctgry_5(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cntrprty_ctgry_5, ::std::string::String::new())
    }

    // string ind_outside_ind = 43;


    pub fn get_ind_outside_ind(&self) -> &str {
        &self.ind_outside_ind
    }
    pub fn clear_ind_outside_ind(&mut self) {
        self.ind_outside_ind.clear();
    }

    // Param is passed by value, moved
    pub fn set_ind_outside_ind(&mut self, v: ::std::string::String) {
        self.ind_outside_ind = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ind_outside_ind(&mut self) -> &mut ::std::string::String {
        &mut self.ind_outside_ind
    }

    // Take field
    pub fn take_ind_outside_ind(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ind_outside_ind, ::std::string::String::new())
    }

    // string system_gl = 44;


    pub fn get_system_gl(&self) -> &str {
        &self.system_gl
    }
    pub fn clear_system_gl(&mut self) {
        self.system_gl.clear();
    }

    // Param is passed by value, moved
    pub fn set_system_gl(&mut self, v: ::std::string::String) {
        self.system_gl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_system_gl(&mut self) -> &mut ::std::string::String {
        &mut self.system_gl
    }

    // Take field
    pub fn take_system_gl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.system_gl, ::std::string::String::new())
    }

    // string alm_concat = 45;


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

    // string div = 46;


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

    // string alm_line = 47;


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

    // string ia_line = 48;


    pub fn get_ia_line(&self) -> &str {
        &self.ia_line
    }
    pub fn clear_ia_line(&mut self) {
        self.ia_line.clear();
    }

    // Param is passed by value, moved
    pub fn set_ia_line(&mut self, v: ::std::string::String) {
        self.ia_line = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ia_line(&mut self) -> &mut ::std::string::String {
        &mut self.ia_line
    }

    // Take field
    pub fn take_ia_line(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ia_line, ::std::string::String::new())
    }

    // double tot_int_amt = 49;


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

    // double tot_prin_amt = 50;


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

    // string sma_flag = 51;


    pub fn get_sma_flag(&self) -> &str {
        &self.sma_flag
    }
    pub fn clear_sma_flag(&mut self) {
        self.sma_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_sma_flag(&mut self, v: ::std::string::String) {
        self.sma_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sma_flag(&mut self) -> &mut ::std::string::String {
        &mut self.sma_flag
    }

    // Take field
    pub fn take_sma_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sma_flag, ::std::string::String::new())
    }

    // repeated .Cashflow cashflows = 52;


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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.deal_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.branch)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.inst_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lend_borr_typ)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.typology)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.usage)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sub_typ_borr_lend)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.crtn_dt = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.val_date = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.deal_date = tmp;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.crnt_deal_amt = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.crnt_conv_rt_lcy = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.crnt_deal_amt_lcy = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.roi = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.tenor_days = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_dt = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.prin_amt = tmp;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_amt = tmp;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf_typ)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.flow_typ)?;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.mat_amt = tmp;
                },
                24 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.dealer_name)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nds_ref_no)?;
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_fix_dt = tmp;
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.residual_tenor = tmp;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_put_dt = tmp;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_call_dt = tmp;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_int_pay_dt = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.int_pay_tenor = tmp;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.aip_air = tmp;
                },
                33 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.downgrade_clause)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.avg_monthly_bal)?;
                },
                35 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.glcode)?;
                },
                36 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty_ctgry_1)?;
                },
                37 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty_ctgry_2)?;
                },
                38 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty_ctgry_3)?;
                },
                39 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty_ctgry_4)?;
                },
                40 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int_pay_rec)?;
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.bckt_days = tmp;
                },
                42 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntrprty_ctgry_5)?;
                },
                43 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ind_outside_ind)?;
                },
                44 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.system_gl)?;
                },
                45 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_concat)?;
                },
                46 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.div)?;
                },
                47 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                48 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ia_line)?;
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_int_amt = tmp;
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_prin_amt = tmp;
                },
                51 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                52 => {
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
        if !self.deal_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.deal_id);
        }
        if !self.branch.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.branch);
        }
        if !self.inst_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.inst_name);
        }
        if !self.lend_borr_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.lend_borr_typ);
        }
        if !self.typology.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.typology);
        }
        if !self.usage.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.usage);
        }
        if !self.sub_typ_borr_lend.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.sub_typ_borr_lend);
        }
        if !self.cntrprty.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.cntrprty);
        }
        if self.crtn_dt != 0 {
            my_size += ::protobuf::rt::value_size(9, self.crtn_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.val_date != 0 {
            my_size += ::protobuf::rt::value_size(10, self.val_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.deal_date != 0 {
            my_size += ::protobuf::rt::value_size(11, self.deal_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.ccy);
        }
        if self.crnt_deal_amt != 0. {
            my_size += 9;
        }
        if self.crnt_conv_rt_lcy != 0. {
            my_size += 9;
        }
        if self.crnt_deal_amt_lcy != 0. {
            my_size += 9;
        }
        if self.roi != 0. {
            my_size += 10;
        }
        if self.tenor_days != 0 {
            my_size += ::protobuf::rt::value_size(17, self.tenor_days, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mat_dt != 0 {
            my_size += ::protobuf::rt::value_size(18, self.mat_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prin_amt != 0. {
            my_size += 10;
        }
        if self.int_amt != 0. {
            my_size += 10;
        }
        if !self.cf_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.cf_typ);
        }
        if !self.flow_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(22, &self.flow_typ);
        }
        if self.mat_amt != 0. {
            my_size += 10;
        }
        if !self.dealer_name.is_empty() {
            my_size += ::protobuf::rt::string_size(24, &self.dealer_name);
        }
        if !self.nds_ref_no.is_empty() {
            my_size += ::protobuf::rt::string_size(25, &self.nds_ref_no);
        }
        if self.nxt_fix_dt != 0 {
            my_size += ::protobuf::rt::value_size(26, self.nxt_fix_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.residual_tenor != 0 {
            my_size += ::protobuf::rt::value_size(27, self.residual_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nxt_put_dt != 0 {
            my_size += ::protobuf::rt::value_size(28, self.nxt_put_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nxt_call_dt != 0 {
            my_size += ::protobuf::rt::value_size(29, self.nxt_call_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nxt_int_pay_dt != 0 {
            my_size += ::protobuf::rt::value_size(30, self.nxt_int_pay_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.int_pay_tenor != 0 {
            my_size += ::protobuf::rt::value_size(31, self.int_pay_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.aip_air != 0. {
            my_size += 10;
        }
        if !self.downgrade_clause.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.downgrade_clause);
        }
        if !self.avg_monthly_bal.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.avg_monthly_bal);
        }
        if !self.glcode.is_empty() {
            my_size += ::protobuf::rt::string_size(35, &self.glcode);
        }
        if !self.cntrprty_ctgry_1.is_empty() {
            my_size += ::protobuf::rt::string_size(36, &self.cntrprty_ctgry_1);
        }
        if !self.cntrprty_ctgry_2.is_empty() {
            my_size += ::protobuf::rt::string_size(37, &self.cntrprty_ctgry_2);
        }
        if !self.cntrprty_ctgry_3.is_empty() {
            my_size += ::protobuf::rt::string_size(38, &self.cntrprty_ctgry_3);
        }
        if !self.cntrprty_ctgry_4.is_empty() {
            my_size += ::protobuf::rt::string_size(39, &self.cntrprty_ctgry_4);
        }
        if !self.int_pay_rec.is_empty() {
            my_size += ::protobuf::rt::string_size(40, &self.int_pay_rec);
        }
        if self.bckt_days != 0 {
            my_size += ::protobuf::rt::value_size(41, self.bckt_days, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.cntrprty_ctgry_5.is_empty() {
            my_size += ::protobuf::rt::string_size(42, &self.cntrprty_ctgry_5);
        }
        if !self.ind_outside_ind.is_empty() {
            my_size += ::protobuf::rt::string_size(43, &self.ind_outside_ind);
        }
        if !self.system_gl.is_empty() {
            my_size += ::protobuf::rt::string_size(44, &self.system_gl);
        }
        if !self.alm_concat.is_empty() {
            my_size += ::protobuf::rt::string_size(45, &self.alm_concat);
        }
        if !self.div.is_empty() {
            my_size += ::protobuf::rt::string_size(46, &self.div);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(47, &self.alm_line);
        }
        if !self.ia_line.is_empty() {
            my_size += ::protobuf::rt::string_size(48, &self.ia_line);
        }
        if self.tot_int_amt != 0. {
            my_size += 10;
        }
        if self.tot_prin_amt != 0. {
            my_size += 10;
        }
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(51, &self.sma_flag);
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
        if !self.deal_id.is_empty() {
            os.write_string(1, &self.deal_id)?;
        }
        if !self.branch.is_empty() {
            os.write_string(2, &self.branch)?;
        }
        if !self.inst_name.is_empty() {
            os.write_string(3, &self.inst_name)?;
        }
        if !self.lend_borr_typ.is_empty() {
            os.write_string(4, &self.lend_borr_typ)?;
        }
        if !self.typology.is_empty() {
            os.write_string(5, &self.typology)?;
        }
        if !self.usage.is_empty() {
            os.write_string(6, &self.usage)?;
        }
        if !self.sub_typ_borr_lend.is_empty() {
            os.write_string(7, &self.sub_typ_borr_lend)?;
        }
        if !self.cntrprty.is_empty() {
            os.write_string(8, &self.cntrprty)?;
        }
        if self.crtn_dt != 0 {
            os.write_int64(9, self.crtn_dt)?;
        }
        if self.val_date != 0 {
            os.write_int64(10, self.val_date)?;
        }
        if self.deal_date != 0 {
            os.write_int64(11, self.deal_date)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(12, &self.ccy)?;
        }
        if self.crnt_deal_amt != 0. {
            os.write_double(13, self.crnt_deal_amt)?;
        }
        if self.crnt_conv_rt_lcy != 0. {
            os.write_double(14, self.crnt_conv_rt_lcy)?;
        }
        if self.crnt_deal_amt_lcy != 0. {
            os.write_double(15, self.crnt_deal_amt_lcy)?;
        }
        if self.roi != 0. {
            os.write_double(16, self.roi)?;
        }
        if self.tenor_days != 0 {
            os.write_int64(17, self.tenor_days)?;
        }
        if self.mat_dt != 0 {
            os.write_int64(18, self.mat_dt)?;
        }
        if self.prin_amt != 0. {
            os.write_double(19, self.prin_amt)?;
        }
        if self.int_amt != 0. {
            os.write_double(20, self.int_amt)?;
        }
        if !self.cf_typ.is_empty() {
            os.write_string(21, &self.cf_typ)?;
        }
        if !self.flow_typ.is_empty() {
            os.write_string(22, &self.flow_typ)?;
        }
        if self.mat_amt != 0. {
            os.write_double(23, self.mat_amt)?;
        }
        if !self.dealer_name.is_empty() {
            os.write_string(24, &self.dealer_name)?;
        }
        if !self.nds_ref_no.is_empty() {
            os.write_string(25, &self.nds_ref_no)?;
        }
        if self.nxt_fix_dt != 0 {
            os.write_int64(26, self.nxt_fix_dt)?;
        }
        if self.residual_tenor != 0 {
            os.write_int64(27, self.residual_tenor)?;
        }
        if self.nxt_put_dt != 0 {
            os.write_int64(28, self.nxt_put_dt)?;
        }
        if self.nxt_call_dt != 0 {
            os.write_int64(29, self.nxt_call_dt)?;
        }
        if self.nxt_int_pay_dt != 0 {
            os.write_int64(30, self.nxt_int_pay_dt)?;
        }
        if self.int_pay_tenor != 0 {
            os.write_int64(31, self.int_pay_tenor)?;
        }
        if self.aip_air != 0. {
            os.write_double(32, self.aip_air)?;
        }
        if !self.downgrade_clause.is_empty() {
            os.write_string(33, &self.downgrade_clause)?;
        }
        if !self.avg_monthly_bal.is_empty() {
            os.write_string(34, &self.avg_monthly_bal)?;
        }
        if !self.glcode.is_empty() {
            os.write_string(35, &self.glcode)?;
        }
        if !self.cntrprty_ctgry_1.is_empty() {
            os.write_string(36, &self.cntrprty_ctgry_1)?;
        }
        if !self.cntrprty_ctgry_2.is_empty() {
            os.write_string(37, &self.cntrprty_ctgry_2)?;
        }
        if !self.cntrprty_ctgry_3.is_empty() {
            os.write_string(38, &self.cntrprty_ctgry_3)?;
        }
        if !self.cntrprty_ctgry_4.is_empty() {
            os.write_string(39, &self.cntrprty_ctgry_4)?;
        }
        if !self.int_pay_rec.is_empty() {
            os.write_string(40, &self.int_pay_rec)?;
        }
        if self.bckt_days != 0 {
            os.write_int64(41, self.bckt_days)?;
        }
        if !self.cntrprty_ctgry_5.is_empty() {
            os.write_string(42, &self.cntrprty_ctgry_5)?;
        }
        if !self.ind_outside_ind.is_empty() {
            os.write_string(43, &self.ind_outside_ind)?;
        }
        if !self.system_gl.is_empty() {
            os.write_string(44, &self.system_gl)?;
        }
        if !self.alm_concat.is_empty() {
            os.write_string(45, &self.alm_concat)?;
        }
        if !self.div.is_empty() {
            os.write_string(46, &self.div)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(47, &self.alm_line)?;
        }
        if !self.ia_line.is_empty() {
            os.write_string(48, &self.ia_line)?;
        }
        if self.tot_int_amt != 0. {
            os.write_double(49, self.tot_int_amt)?;
        }
        if self.tot_prin_amt != 0. {
            os.write_double(50, self.tot_prin_amt)?;
        }
        if !self.sma_flag.is_empty() {
            os.write_string(51, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(52, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                "deal_id",
                |m: &AccountWithCashflows| { &m.deal_id },
                |m: &mut AccountWithCashflows| { &mut m.deal_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "branch",
                |m: &AccountWithCashflows| { &m.branch },
                |m: &mut AccountWithCashflows| { &mut m.branch },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "inst_name",
                |m: &AccountWithCashflows| { &m.inst_name },
                |m: &mut AccountWithCashflows| { &mut m.inst_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "lend_borr_typ",
                |m: &AccountWithCashflows| { &m.lend_borr_typ },
                |m: &mut AccountWithCashflows| { &mut m.lend_borr_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "typology",
                |m: &AccountWithCashflows| { &m.typology },
                |m: &mut AccountWithCashflows| { &mut m.typology },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "usage",
                |m: &AccountWithCashflows| { &m.usage },
                |m: &mut AccountWithCashflows| { &mut m.usage },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sub_typ_borr_lend",
                |m: &AccountWithCashflows| { &m.sub_typ_borr_lend },
                |m: &mut AccountWithCashflows| { &mut m.sub_typ_borr_lend },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty",
                |m: &AccountWithCashflows| { &m.cntrprty },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "crtn_dt",
                |m: &AccountWithCashflows| { &m.crtn_dt },
                |m: &mut AccountWithCashflows| { &mut m.crtn_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "val_date",
                |m: &AccountWithCashflows| { &m.val_date },
                |m: &mut AccountWithCashflows| { &mut m.val_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "deal_date",
                |m: &AccountWithCashflows| { &m.deal_date },
                |m: &mut AccountWithCashflows| { &mut m.deal_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ccy",
                |m: &AccountWithCashflows| { &m.ccy },
                |m: &mut AccountWithCashflows| { &mut m.ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "crnt_deal_amt",
                |m: &AccountWithCashflows| { &m.crnt_deal_amt },
                |m: &mut AccountWithCashflows| { &mut m.crnt_deal_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "crnt_conv_rt_lcy",
                |m: &AccountWithCashflows| { &m.crnt_conv_rt_lcy },
                |m: &mut AccountWithCashflows| { &mut m.crnt_conv_rt_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "crnt_deal_amt_lcy",
                |m: &AccountWithCashflows| { &m.crnt_deal_amt_lcy },
                |m: &mut AccountWithCashflows| { &mut m.crnt_deal_amt_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "roi",
                |m: &AccountWithCashflows| { &m.roi },
                |m: &mut AccountWithCashflows| { &mut m.roi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "tenor_days",
                |m: &AccountWithCashflows| { &m.tenor_days },
                |m: &mut AccountWithCashflows| { &mut m.tenor_days },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mat_dt",
                |m: &AccountWithCashflows| { &m.mat_dt },
                |m: &mut AccountWithCashflows| { &mut m.mat_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "prin_amt",
                |m: &AccountWithCashflows| { &m.prin_amt },
                |m: &mut AccountWithCashflows| { &mut m.prin_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_amt",
                |m: &AccountWithCashflows| { &m.int_amt },
                |m: &mut AccountWithCashflows| { &mut m.int_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cf_typ",
                |m: &AccountWithCashflows| { &m.cf_typ },
                |m: &mut AccountWithCashflows| { &mut m.cf_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "flow_typ",
                |m: &AccountWithCashflows| { &m.flow_typ },
                |m: &mut AccountWithCashflows| { &mut m.flow_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "mat_amt",
                |m: &AccountWithCashflows| { &m.mat_amt },
                |m: &mut AccountWithCashflows| { &mut m.mat_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "dealer_name",
                |m: &AccountWithCashflows| { &m.dealer_name },
                |m: &mut AccountWithCashflows| { &mut m.dealer_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "nds_ref_no",
                |m: &AccountWithCashflows| { &m.nds_ref_no },
                |m: &mut AccountWithCashflows| { &mut m.nds_ref_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_fix_dt",
                |m: &AccountWithCashflows| { &m.nxt_fix_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_fix_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "residual_tenor",
                |m: &AccountWithCashflows| { &m.residual_tenor },
                |m: &mut AccountWithCashflows| { &mut m.residual_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_put_dt",
                |m: &AccountWithCashflows| { &m.nxt_put_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_put_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_call_dt",
                |m: &AccountWithCashflows| { &m.nxt_call_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_call_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_int_pay_dt",
                |m: &AccountWithCashflows| { &m.nxt_int_pay_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_int_pay_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "int_pay_tenor",
                |m: &AccountWithCashflows| { &m.int_pay_tenor },
                |m: &mut AccountWithCashflows| { &mut m.int_pay_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "aip_air",
                |m: &AccountWithCashflows| { &m.aip_air },
                |m: &mut AccountWithCashflows| { &mut m.aip_air },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "downgrade_clause",
                |m: &AccountWithCashflows| { &m.downgrade_clause },
                |m: &mut AccountWithCashflows| { &mut m.downgrade_clause },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "avg_monthly_bal",
                |m: &AccountWithCashflows| { &m.avg_monthly_bal },
                |m: &mut AccountWithCashflows| { &mut m.avg_monthly_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "glcode",
                |m: &AccountWithCashflows| { &m.glcode },
                |m: &mut AccountWithCashflows| { &mut m.glcode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty_ctgry_1",
                |m: &AccountWithCashflows| { &m.cntrprty_ctgry_1 },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty_ctgry_1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty_ctgry_2",
                |m: &AccountWithCashflows| { &m.cntrprty_ctgry_2 },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty_ctgry_2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty_ctgry_3",
                |m: &AccountWithCashflows| { &m.cntrprty_ctgry_3 },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty_ctgry_3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty_ctgry_4",
                |m: &AccountWithCashflows| { &m.cntrprty_ctgry_4 },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty_ctgry_4 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int_pay_rec",
                |m: &AccountWithCashflows| { &m.int_pay_rec },
                |m: &mut AccountWithCashflows| { &mut m.int_pay_rec },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "bckt_days",
                |m: &AccountWithCashflows| { &m.bckt_days },
                |m: &mut AccountWithCashflows| { &mut m.bckt_days },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntrprty_ctgry_5",
                |m: &AccountWithCashflows| { &m.cntrprty_ctgry_5 },
                |m: &mut AccountWithCashflows| { &mut m.cntrprty_ctgry_5 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ind_outside_ind",
                |m: &AccountWithCashflows| { &m.ind_outside_ind },
                |m: &mut AccountWithCashflows| { &mut m.ind_outside_ind },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "system_gl",
                |m: &AccountWithCashflows| { &m.system_gl },
                |m: &mut AccountWithCashflows| { &mut m.system_gl },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_concat",
                |m: &AccountWithCashflows| { &m.alm_concat },
                |m: &mut AccountWithCashflows| { &mut m.alm_concat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "div",
                |m: &AccountWithCashflows| { &m.div },
                |m: &mut AccountWithCashflows| { &mut m.div },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &AccountWithCashflows| { &m.alm_line },
                |m: &mut AccountWithCashflows| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ia_line",
                |m: &AccountWithCashflows| { &m.ia_line },
                |m: &mut AccountWithCashflows| { &mut m.ia_line },
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sma_flag",
                |m: &AccountWithCashflows| { &m.sma_flag },
                |m: &mut AccountWithCashflows| { &mut m.sma_flag },
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
        self.deal_id.clear();
        self.branch.clear();
        self.inst_name.clear();
        self.lend_borr_typ.clear();
        self.typology.clear();
        self.usage.clear();
        self.sub_typ_borr_lend.clear();
        self.cntrprty.clear();
        self.crtn_dt = 0;
        self.val_date = 0;
        self.deal_date = 0;
        self.ccy.clear();
        self.crnt_deal_amt = 0.;
        self.crnt_conv_rt_lcy = 0.;
        self.crnt_deal_amt_lcy = 0.;
        self.roi = 0.;
        self.tenor_days = 0;
        self.mat_dt = 0;
        self.prin_amt = 0.;
        self.int_amt = 0.;
        self.cf_typ.clear();
        self.flow_typ.clear();
        self.mat_amt = 0.;
        self.dealer_name.clear();
        self.nds_ref_no.clear();
        self.nxt_fix_dt = 0;
        self.residual_tenor = 0;
        self.nxt_put_dt = 0;
        self.nxt_call_dt = 0;
        self.nxt_int_pay_dt = 0;
        self.int_pay_tenor = 0;
        self.aip_air = 0.;
        self.downgrade_clause.clear();
        self.avg_monthly_bal.clear();
        self.glcode.clear();
        self.cntrprty_ctgry_1.clear();
        self.cntrprty_ctgry_2.clear();
        self.cntrprty_ctgry_3.clear();
        self.cntrprty_ctgry_4.clear();
        self.int_pay_rec.clear();
        self.bckt_days = 0;
        self.cntrprty_ctgry_5.clear();
        self.ind_outside_ind.clear();
        self.system_gl.clear();
        self.alm_concat.clear();
        self.div.clear();
        self.alm_line.clear();
        self.ia_line.clear();
        self.tot_int_amt = 0.;
        self.tot_prin_amt = 0.;
        self.sma_flag.clear();
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
    \n\x0fborr_lend.proto\"r\n\x08Cashflow\x12'\n\x0finterest_amount\x18\x01\
    \x20\x01(\x01R\x0einterestAmount\x12)\n\x10principal_amount\x18\x02\x20\
    \x01(\x01R\x0fprincipalAmount\x12\x12\n\x04date\x18\x03\x20\x01(\x03R\
    \x04date\"\xf1\x0c\n\x14AccountWithCashflows\x12\x17\n\x07deal_id\x18\
    \x01\x20\x01(\tR\x06dealId\x12\x16\n\x06branch\x18\x02\x20\x01(\tR\x06br\
    anch\x12\x1b\n\tinst_name\x18\x03\x20\x01(\tR\x08instName\x12\"\n\rlend_\
    borr_typ\x18\x04\x20\x01(\tR\x0blendBorrTyp\x12\x1a\n\x08typology\x18\
    \x05\x20\x01(\tR\x08typology\x12\x14\n\x05usage\x18\x06\x20\x01(\tR\x05u\
    sage\x12)\n\x11sub_typ_borr_lend\x18\x07\x20\x01(\tR\x0esubTypBorrLend\
    \x12\x1a\n\x08cntrprty\x18\x08\x20\x01(\tR\x08cntrprty\x12\x17\n\x07crtn\
    _dt\x18\t\x20\x01(\x03R\x06crtnDt\x12\x19\n\x08val_date\x18\n\x20\x01(\
    \x03R\x07valDate\x12\x1b\n\tdeal_date\x18\x0b\x20\x01(\x03R\x08dealDate\
    \x12\x10\n\x03ccy\x18\x0c\x20\x01(\tR\x03ccy\x12\"\n\rcrnt_deal_amt\x18\
    \r\x20\x01(\x01R\x0bcrntDealAmt\x12'\n\x10crnt_conv_rt_lcy\x18\x0e\x20\
    \x01(\x01R\rcrntConvRtLcy\x12)\n\x11crnt_deal_amt_lcy\x18\x0f\x20\x01(\
    \x01R\x0ecrntDealAmtLcy\x12\x10\n\x03roi\x18\x10\x20\x01(\x01R\x03roi\
    \x12\x1d\n\ntenor_days\x18\x11\x20\x01(\x03R\ttenorDays\x12\x15\n\x06mat\
    _dt\x18\x12\x20\x01(\x03R\x05matDt\x12\x19\n\x08prin_amt\x18\x13\x20\x01\
    (\x01R\x07prinAmt\x12\x17\n\x07int_amt\x18\x14\x20\x01(\x01R\x06intAmt\
    \x12\x15\n\x06cf_typ\x18\x15\x20\x01(\tR\x05cfTyp\x12\x19\n\x08flow_typ\
    \x18\x16\x20\x01(\tR\x07flowTyp\x12\x17\n\x07mat_amt\x18\x17\x20\x01(\
    \x01R\x06matAmt\x12\x1f\n\x0bdealer_name\x18\x18\x20\x01(\tR\ndealerName\
    \x12\x1c\n\nnds_ref_no\x18\x19\x20\x01(\tR\x08ndsRefNo\x12\x1c\n\nnxt_fi\
    x_dt\x18\x1a\x20\x01(\x03R\x08nxtFixDt\x12%\n\x0eresidual_tenor\x18\x1b\
    \x20\x01(\x03R\rresidualTenor\x12\x1c\n\nnxt_put_dt\x18\x1c\x20\x01(\x03\
    R\x08nxtPutDt\x12\x1e\n\x0bnxt_call_dt\x18\x1d\x20\x01(\x03R\tnxtCallDt\
    \x12#\n\x0enxt_int_pay_dt\x18\x1e\x20\x01(\x03R\x0bnxtIntPayDt\x12\"\n\r\
    int_pay_tenor\x18\x1f\x20\x01(\x03R\x0bintPayTenor\x12\x17\n\x07aip_air\
    \x18\x20\x20\x01(\x01R\x06aipAir\x12)\n\x10downgrade_clause\x18!\x20\x01\
    (\tR\x0fdowngradeClause\x12&\n\x0favg_monthly_bal\x18\"\x20\x01(\tR\ravg\
    MonthlyBal\x12\x16\n\x06glcode\x18#\x20\x01(\tR\x06glcode\x12(\n\x10cntr\
    prty_ctgry_1\x18$\x20\x01(\tR\x0ecntrprtyCtgry1\x12(\n\x10cntrprty_ctgry\
    _2\x18%\x20\x01(\tR\x0ecntrprtyCtgry2\x12(\n\x10cntrprty_ctgry_3\x18&\
    \x20\x01(\tR\x0ecntrprtyCtgry3\x12(\n\x10cntrprty_ctgry_4\x18'\x20\x01(\
    \tR\x0ecntrprtyCtgry4\x12\x1e\n\x0bint_pay_rec\x18(\x20\x01(\tR\tintPayR\
    ec\x12\x1b\n\tbckt_days\x18)\x20\x01(\x03R\x08bcktDays\x12(\n\x10cntrprt\
    y_ctgry_5\x18*\x20\x01(\tR\x0ecntrprtyCtgry5\x12&\n\x0find_outside_ind\
    \x18+\x20\x01(\tR\rindOutsideInd\x12\x1b\n\tsystem_gl\x18,\x20\x01(\tR\
    \x08systemGl\x12\x1d\n\nalm_concat\x18-\x20\x01(\tR\talmConcat\x12\x10\n\
    \x03div\x18.\x20\x01(\tR\x03div\x12\x19\n\x08alm_line\x18/\x20\x01(\tR\
    \x07almLine\x12\x17\n\x07ia_line\x180\x20\x01(\tR\x06iaLine\x12\x1e\n\
    \x0btot_int_amt\x181\x20\x01(\x01R\ttotIntAmt\x12\x20\n\x0ctot_prin_amt\
    \x182\x20\x01(\x01R\ntotPrinAmt\x12\x19\n\x08sma_flag\x183\x20\x01(\tR\
    \x07smaFlag\x12'\n\tcashflows\x184\x20\x03(\x0b2\t.CashflowR\tcashflowsb\
    \x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).expect("Error parsing proto data")
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
