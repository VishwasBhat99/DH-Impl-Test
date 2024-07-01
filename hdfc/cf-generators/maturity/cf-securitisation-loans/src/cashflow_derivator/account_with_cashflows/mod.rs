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
    pub acc_id: ::std::string::String,
    pub cust_name: ::std::string::String,
    pub pout_bal: f64,
    pub acc_int: f64,
    pub st_dt: i64,
    pub c_dt: i64,
    pub gl_cd: ::std::string::String,
    pub int_rt: f64,
    pub int_typ: ::std::string::String,
    pub int_bmark: ::std::string::String,
    pub rt_flag: ::std::string::String,
    pub prod_cd: ::std::string::String,
    pub nxt_pay_dt: i64,
    pub mis2: i64,
    pub ccy: ::std::string::String,
    pub ratings: ::std::string::String,
    pub rating_agency: ::std::string::String,
    pub asset_class: ::std::string::String,
    pub div: ::std::string::String,
    pub typ: ::std::string::String,
    pub originator: ::std::string::String,
    pub as_on_dt: i64,
    pub rep_freq: ::std::string::String,
    pub nxt_rep_dt: i64,
    pub alm_line: ::std::string::String,
    pub yeild: f64,
    pub deal_name: ::std::string::String,
    pub org_tenor: i64,
    pub resid_tenor: i64,
    pub total_interest_amount: f64,
    pub total_principal_amount: f64,
    pub mis1: i64,
    pub ia_line: ::std::string::String,
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

    // string acc_id = 1;


    pub fn get_acc_id(&self) -> &str {
        &self.acc_id
    }
    pub fn clear_acc_id(&mut self) {
        self.acc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_acc_id(&mut self, v: ::std::string::String) {
        self.acc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acc_id(&mut self) -> &mut ::std::string::String {
        &mut self.acc_id
    }

    // Take field
    pub fn take_acc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.acc_id, ::std::string::String::new())
    }

    // string cust_name = 2;


    pub fn get_cust_name(&self) -> &str {
        &self.cust_name
    }
    pub fn clear_cust_name(&mut self) {
        self.cust_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_name(&mut self, v: ::std::string::String) {
        self.cust_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_name(&mut self) -> &mut ::std::string::String {
        &mut self.cust_name
    }

    // Take field
    pub fn take_cust_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_name, ::std::string::String::new())
    }

    // double pout_bal = 3;


    pub fn get_pout_bal(&self) -> f64 {
        self.pout_bal
    }
    pub fn clear_pout_bal(&mut self) {
        self.pout_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_pout_bal(&mut self, v: f64) {
        self.pout_bal = v;
    }

    // double acc_int = 4;


    pub fn get_acc_int(&self) -> f64 {
        self.acc_int
    }
    pub fn clear_acc_int(&mut self) {
        self.acc_int = 0.;
    }

    // Param is passed by value, moved
    pub fn set_acc_int(&mut self, v: f64) {
        self.acc_int = v;
    }

    // int64 st_dt = 5;


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

    // int64 c_dt = 6;


    pub fn get_c_dt(&self) -> i64 {
        self.c_dt
    }
    pub fn clear_c_dt(&mut self) {
        self.c_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_c_dt(&mut self, v: i64) {
        self.c_dt = v;
    }

    // string gl_cd = 7;


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

    // double int_rt = 8;


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

    // string int_typ = 9;


    pub fn get_int_typ(&self) -> &str {
        &self.int_typ
    }
    pub fn clear_int_typ(&mut self) {
        self.int_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_typ(&mut self, v: ::std::string::String) {
        self.int_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int_typ(&mut self) -> &mut ::std::string::String {
        &mut self.int_typ
    }

    // Take field
    pub fn take_int_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int_typ, ::std::string::String::new())
    }

    // string int_bmark = 10;


    pub fn get_int_bmark(&self) -> &str {
        &self.int_bmark
    }
    pub fn clear_int_bmark(&mut self) {
        self.int_bmark.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_bmark(&mut self, v: ::std::string::String) {
        self.int_bmark = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int_bmark(&mut self) -> &mut ::std::string::String {
        &mut self.int_bmark
    }

    // Take field
    pub fn take_int_bmark(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int_bmark, ::std::string::String::new())
    }

    // string rt_flag = 11;


    pub fn get_rt_flag(&self) -> &str {
        &self.rt_flag
    }
    pub fn clear_rt_flag(&mut self) {
        self.rt_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_rt_flag(&mut self, v: ::std::string::String) {
        self.rt_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rt_flag(&mut self) -> &mut ::std::string::String {
        &mut self.rt_flag
    }

    // Take field
    pub fn take_rt_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rt_flag, ::std::string::String::new())
    }

    // string prod_cd = 12;


    pub fn get_prod_cd(&self) -> &str {
        &self.prod_cd
    }
    pub fn clear_prod_cd(&mut self) {
        self.prod_cd.clear();
    }

    // Param is passed by value, moved
    pub fn set_prod_cd(&mut self, v: ::std::string::String) {
        self.prod_cd = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prod_cd(&mut self) -> &mut ::std::string::String {
        &mut self.prod_cd
    }

    // Take field
    pub fn take_prod_cd(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.prod_cd, ::std::string::String::new())
    }

    // int64 nxt_pay_dt = 13;


    pub fn get_nxt_pay_dt(&self) -> i64 {
        self.nxt_pay_dt
    }
    pub fn clear_nxt_pay_dt(&mut self) {
        self.nxt_pay_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_pay_dt(&mut self, v: i64) {
        self.nxt_pay_dt = v;
    }

    // int64 mis2 = 14;


    pub fn get_mis2(&self) -> i64 {
        self.mis2
    }
    pub fn clear_mis2(&mut self) {
        self.mis2 = 0;
    }

    // Param is passed by value, moved
    pub fn set_mis2(&mut self, v: i64) {
        self.mis2 = v;
    }

    // string ccy = 15;


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

    // string ratings = 16;


    pub fn get_ratings(&self) -> &str {
        &self.ratings
    }
    pub fn clear_ratings(&mut self) {
        self.ratings.clear();
    }

    // Param is passed by value, moved
    pub fn set_ratings(&mut self, v: ::std::string::String) {
        self.ratings = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ratings(&mut self) -> &mut ::std::string::String {
        &mut self.ratings
    }

    // Take field
    pub fn take_ratings(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ratings, ::std::string::String::new())
    }

    // string rating_agency = 17;


    pub fn get_rating_agency(&self) -> &str {
        &self.rating_agency
    }
    pub fn clear_rating_agency(&mut self) {
        self.rating_agency.clear();
    }

    // Param is passed by value, moved
    pub fn set_rating_agency(&mut self, v: ::std::string::String) {
        self.rating_agency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rating_agency(&mut self) -> &mut ::std::string::String {
        &mut self.rating_agency
    }

    // Take field
    pub fn take_rating_agency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rating_agency, ::std::string::String::new())
    }

    // string asset_class = 18;


    pub fn get_asset_class(&self) -> &str {
        &self.asset_class
    }
    pub fn clear_asset_class(&mut self) {
        self.asset_class.clear();
    }

    // Param is passed by value, moved
    pub fn set_asset_class(&mut self, v: ::std::string::String) {
        self.asset_class = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_asset_class(&mut self) -> &mut ::std::string::String {
        &mut self.asset_class
    }

    // Take field
    pub fn take_asset_class(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.asset_class, ::std::string::String::new())
    }

    // string div = 19;


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

    // string typ = 20;


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

    // string originator = 21;


    pub fn get_originator(&self) -> &str {
        &self.originator
    }
    pub fn clear_originator(&mut self) {
        self.originator.clear();
    }

    // Param is passed by value, moved
    pub fn set_originator(&mut self, v: ::std::string::String) {
        self.originator = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_originator(&mut self) -> &mut ::std::string::String {
        &mut self.originator
    }

    // Take field
    pub fn take_originator(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.originator, ::std::string::String::new())
    }

    // int64 as_on_dt = 22;


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

    // string rep_freq = 23;


    pub fn get_rep_freq(&self) -> &str {
        &self.rep_freq
    }
    pub fn clear_rep_freq(&mut self) {
        self.rep_freq.clear();
    }

    // Param is passed by value, moved
    pub fn set_rep_freq(&mut self, v: ::std::string::String) {
        self.rep_freq = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rep_freq(&mut self) -> &mut ::std::string::String {
        &mut self.rep_freq
    }

    // Take field
    pub fn take_rep_freq(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rep_freq, ::std::string::String::new())
    }

    // int64 nxt_rep_dt = 24;


    pub fn get_nxt_rep_dt(&self) -> i64 {
        self.nxt_rep_dt
    }
    pub fn clear_nxt_rep_dt(&mut self) {
        self.nxt_rep_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_rep_dt(&mut self, v: i64) {
        self.nxt_rep_dt = v;
    }

    // string alm_line = 25;


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

    // double yeild = 26;


    pub fn get_yeild(&self) -> f64 {
        self.yeild
    }
    pub fn clear_yeild(&mut self) {
        self.yeild = 0.;
    }

    // Param is passed by value, moved
    pub fn set_yeild(&mut self, v: f64) {
        self.yeild = v;
    }

    // string deal_name = 27;


    pub fn get_deal_name(&self) -> &str {
        &self.deal_name
    }
    pub fn clear_deal_name(&mut self) {
        self.deal_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_deal_name(&mut self, v: ::std::string::String) {
        self.deal_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deal_name(&mut self) -> &mut ::std::string::String {
        &mut self.deal_name
    }

    // Take field
    pub fn take_deal_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.deal_name, ::std::string::String::new())
    }

    // int64 org_tenor = 28;


    pub fn get_org_tenor(&self) -> i64 {
        self.org_tenor
    }
    pub fn clear_org_tenor(&mut self) {
        self.org_tenor = 0;
    }

    // Param is passed by value, moved
    pub fn set_org_tenor(&mut self, v: i64) {
        self.org_tenor = v;
    }

    // int64 resid_tenor = 29;


    pub fn get_resid_tenor(&self) -> i64 {
        self.resid_tenor
    }
    pub fn clear_resid_tenor(&mut self) {
        self.resid_tenor = 0;
    }

    // Param is passed by value, moved
    pub fn set_resid_tenor(&mut self, v: i64) {
        self.resid_tenor = v;
    }

    // double total_interest_amount = 30;


    pub fn get_total_interest_amount(&self) -> f64 {
        self.total_interest_amount
    }
    pub fn clear_total_interest_amount(&mut self) {
        self.total_interest_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_interest_amount(&mut self, v: f64) {
        self.total_interest_amount = v;
    }

    // double total_principal_amount = 31;


    pub fn get_total_principal_amount(&self) -> f64 {
        self.total_principal_amount
    }
    pub fn clear_total_principal_amount(&mut self) {
        self.total_principal_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_principal_amount(&mut self, v: f64) {
        self.total_principal_amount = v;
    }

    // int64 mis1 = 32;


    pub fn get_mis1(&self) -> i64 {
        self.mis1
    }
    pub fn clear_mis1(&mut self) {
        self.mis1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_mis1(&mut self, v: i64) {
        self.mis1 = v;
    }

    // string ia_line = 33;


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

    // string sma_flag = 34;


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

    // repeated .Cashflow cashflows = 35;


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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.acc_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.pout_bal = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.acc_int = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.st_dt = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.c_dt = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_cd)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_rt = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int_typ)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int_bmark)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rt_flag)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prod_cd)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_pay_dt = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mis2 = tmp;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ratings)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rating_agency)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.asset_class)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.div)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.typ)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.originator)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.as_on_dt = tmp;
                },
                23 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rep_freq)?;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_rep_dt = tmp;
                },
                25 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.yeild = tmp;
                },
                27 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.deal_name)?;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.org_tenor = tmp;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.resid_tenor = tmp;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_interest_amount = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_principal_amount = tmp;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mis1 = tmp;
                },
                33 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ia_line)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                35 => {
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
        if !self.acc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.acc_id);
        }
        if !self.cust_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cust_name);
        }
        if self.pout_bal != 0. {
            my_size += 9;
        }
        if self.acc_int != 0. {
            my_size += 9;
        }
        if self.st_dt != 0 {
            my_size += ::protobuf::rt::value_size(5, self.st_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.c_dt != 0 {
            my_size += ::protobuf::rt::value_size(6, self.c_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.gl_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.gl_cd);
        }
        if self.int_rt != 0. {
            my_size += 9;
        }
        if !self.int_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.int_typ);
        }
        if !self.int_bmark.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.int_bmark);
        }
        if !self.rt_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.rt_flag);
        }
        if !self.prod_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.prod_cd);
        }
        if self.nxt_pay_dt != 0 {
            my_size += ::protobuf::rt::value_size(13, self.nxt_pay_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mis2 != 0 {
            my_size += ::protobuf::rt::value_size(14, self.mis2, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.ccy);
        }
        if !self.ratings.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.ratings);
        }
        if !self.rating_agency.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.rating_agency);
        }
        if !self.asset_class.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.asset_class);
        }
        if !self.div.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.div);
        }
        if !self.typ.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.typ);
        }
        if !self.originator.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.originator);
        }
        if self.as_on_dt != 0 {
            my_size += ::protobuf::rt::value_size(22, self.as_on_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.rep_freq.is_empty() {
            my_size += ::protobuf::rt::string_size(23, &self.rep_freq);
        }
        if self.nxt_rep_dt != 0 {
            my_size += ::protobuf::rt::value_size(24, self.nxt_rep_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(25, &self.alm_line);
        }
        if self.yeild != 0. {
            my_size += 10;
        }
        if !self.deal_name.is_empty() {
            my_size += ::protobuf::rt::string_size(27, &self.deal_name);
        }
        if self.org_tenor != 0 {
            my_size += ::protobuf::rt::value_size(28, self.org_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resid_tenor != 0 {
            my_size += ::protobuf::rt::value_size(29, self.resid_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_interest_amount != 0. {
            my_size += 10;
        }
        if self.total_principal_amount != 0. {
            my_size += 10;
        }
        if self.mis1 != 0 {
            my_size += ::protobuf::rt::value_size(32, self.mis1, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ia_line.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.ia_line);
        }
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.sma_flag);
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
        if !self.acc_id.is_empty() {
            os.write_string(1, &self.acc_id)?;
        }
        if !self.cust_name.is_empty() {
            os.write_string(2, &self.cust_name)?;
        }
        if self.pout_bal != 0. {
            os.write_double(3, self.pout_bal)?;
        }
        if self.acc_int != 0. {
            os.write_double(4, self.acc_int)?;
        }
        if self.st_dt != 0 {
            os.write_int64(5, self.st_dt)?;
        }
        if self.c_dt != 0 {
            os.write_int64(6, self.c_dt)?;
        }
        if !self.gl_cd.is_empty() {
            os.write_string(7, &self.gl_cd)?;
        }
        if self.int_rt != 0. {
            os.write_double(8, self.int_rt)?;
        }
        if !self.int_typ.is_empty() {
            os.write_string(9, &self.int_typ)?;
        }
        if !self.int_bmark.is_empty() {
            os.write_string(10, &self.int_bmark)?;
        }
        if !self.rt_flag.is_empty() {
            os.write_string(11, &self.rt_flag)?;
        }
        if !self.prod_cd.is_empty() {
            os.write_string(12, &self.prod_cd)?;
        }
        if self.nxt_pay_dt != 0 {
            os.write_int64(13, self.nxt_pay_dt)?;
        }
        if self.mis2 != 0 {
            os.write_int64(14, self.mis2)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(15, &self.ccy)?;
        }
        if !self.ratings.is_empty() {
            os.write_string(16, &self.ratings)?;
        }
        if !self.rating_agency.is_empty() {
            os.write_string(17, &self.rating_agency)?;
        }
        if !self.asset_class.is_empty() {
            os.write_string(18, &self.asset_class)?;
        }
        if !self.div.is_empty() {
            os.write_string(19, &self.div)?;
        }
        if !self.typ.is_empty() {
            os.write_string(20, &self.typ)?;
        }
        if !self.originator.is_empty() {
            os.write_string(21, &self.originator)?;
        }
        if self.as_on_dt != 0 {
            os.write_int64(22, self.as_on_dt)?;
        }
        if !self.rep_freq.is_empty() {
            os.write_string(23, &self.rep_freq)?;
        }
        if self.nxt_rep_dt != 0 {
            os.write_int64(24, self.nxt_rep_dt)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(25, &self.alm_line)?;
        }
        if self.yeild != 0. {
            os.write_double(26, self.yeild)?;
        }
        if !self.deal_name.is_empty() {
            os.write_string(27, &self.deal_name)?;
        }
        if self.org_tenor != 0 {
            os.write_int64(28, self.org_tenor)?;
        }
        if self.resid_tenor != 0 {
            os.write_int64(29, self.resid_tenor)?;
        }
        if self.total_interest_amount != 0. {
            os.write_double(30, self.total_interest_amount)?;
        }
        if self.total_principal_amount != 0. {
            os.write_double(31, self.total_principal_amount)?;
        }
        if self.mis1 != 0 {
            os.write_int64(32, self.mis1)?;
        }
        if !self.ia_line.is_empty() {
            os.write_string(33, &self.ia_line)?;
        }
        if !self.sma_flag.is_empty() {
            os.write_string(34, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(35, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                "acc_id",
                |m: &AccountWithCashflows| { &m.acc_id },
                |m: &mut AccountWithCashflows| { &mut m.acc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_name",
                |m: &AccountWithCashflows| { &m.cust_name },
                |m: &mut AccountWithCashflows| { &mut m.cust_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "pout_bal",
                |m: &AccountWithCashflows| { &m.pout_bal },
                |m: &mut AccountWithCashflows| { &mut m.pout_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "acc_int",
                |m: &AccountWithCashflows| { &m.acc_int },
                |m: &mut AccountWithCashflows| { &mut m.acc_int },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "st_dt",
                |m: &AccountWithCashflows| { &m.st_dt },
                |m: &mut AccountWithCashflows| { &mut m.st_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "c_dt",
                |m: &AccountWithCashflows| { &m.c_dt },
                |m: &mut AccountWithCashflows| { &mut m.c_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "gl_cd",
                |m: &AccountWithCashflows| { &m.gl_cd },
                |m: &mut AccountWithCashflows| { &mut m.gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_rt",
                |m: &AccountWithCashflows| { &m.int_rt },
                |m: &mut AccountWithCashflows| { &mut m.int_rt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int_typ",
                |m: &AccountWithCashflows| { &m.int_typ },
                |m: &mut AccountWithCashflows| { &mut m.int_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int_bmark",
                |m: &AccountWithCashflows| { &m.int_bmark },
                |m: &mut AccountWithCashflows| { &mut m.int_bmark },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rt_flag",
                |m: &AccountWithCashflows| { &m.rt_flag },
                |m: &mut AccountWithCashflows| { &mut m.rt_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "prod_cd",
                |m: &AccountWithCashflows| { &m.prod_cd },
                |m: &mut AccountWithCashflows| { &mut m.prod_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_pay_dt",
                |m: &AccountWithCashflows| { &m.nxt_pay_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_pay_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mis2",
                |m: &AccountWithCashflows| { &m.mis2 },
                |m: &mut AccountWithCashflows| { &mut m.mis2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ccy",
                |m: &AccountWithCashflows| { &m.ccy },
                |m: &mut AccountWithCashflows| { &mut m.ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ratings",
                |m: &AccountWithCashflows| { &m.ratings },
                |m: &mut AccountWithCashflows| { &mut m.ratings },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rating_agency",
                |m: &AccountWithCashflows| { &m.rating_agency },
                |m: &mut AccountWithCashflows| { &mut m.rating_agency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "asset_class",
                |m: &AccountWithCashflows| { &m.asset_class },
                |m: &mut AccountWithCashflows| { &mut m.asset_class },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "div",
                |m: &AccountWithCashflows| { &m.div },
                |m: &mut AccountWithCashflows| { &mut m.div },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "typ",
                |m: &AccountWithCashflows| { &m.typ },
                |m: &mut AccountWithCashflows| { &mut m.typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "originator",
                |m: &AccountWithCashflows| { &m.originator },
                |m: &mut AccountWithCashflows| { &mut m.originator },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "as_on_dt",
                |m: &AccountWithCashflows| { &m.as_on_dt },
                |m: &mut AccountWithCashflows| { &mut m.as_on_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rep_freq",
                |m: &AccountWithCashflows| { &m.rep_freq },
                |m: &mut AccountWithCashflows| { &mut m.rep_freq },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_rep_dt",
                |m: &AccountWithCashflows| { &m.nxt_rep_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_rep_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &AccountWithCashflows| { &m.alm_line },
                |m: &mut AccountWithCashflows| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "yeild",
                |m: &AccountWithCashflows| { &m.yeild },
                |m: &mut AccountWithCashflows| { &mut m.yeild },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "deal_name",
                |m: &AccountWithCashflows| { &m.deal_name },
                |m: &mut AccountWithCashflows| { &mut m.deal_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "org_tenor",
                |m: &AccountWithCashflows| { &m.org_tenor },
                |m: &mut AccountWithCashflows| { &mut m.org_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "resid_tenor",
                |m: &AccountWithCashflows| { &m.resid_tenor },
                |m: &mut AccountWithCashflows| { &mut m.resid_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "total_interest_amount",
                |m: &AccountWithCashflows| { &m.total_interest_amount },
                |m: &mut AccountWithCashflows| { &mut m.total_interest_amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "total_principal_amount",
                |m: &AccountWithCashflows| { &m.total_principal_amount },
                |m: &mut AccountWithCashflows| { &mut m.total_principal_amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mis1",
                |m: &AccountWithCashflows| { &m.mis1 },
                |m: &mut AccountWithCashflows| { &mut m.mis1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ia_line",
                |m: &AccountWithCashflows| { &m.ia_line },
                |m: &mut AccountWithCashflows| { &mut m.ia_line },
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
        self.acc_id.clear();
        self.cust_name.clear();
        self.pout_bal = 0.;
        self.acc_int = 0.;
        self.st_dt = 0;
        self.c_dt = 0;
        self.gl_cd.clear();
        self.int_rt = 0.;
        self.int_typ.clear();
        self.int_bmark.clear();
        self.rt_flag.clear();
        self.prod_cd.clear();
        self.nxt_pay_dt = 0;
        self.mis2 = 0;
        self.ccy.clear();
        self.ratings.clear();
        self.rating_agency.clear();
        self.asset_class.clear();
        self.div.clear();
        self.typ.clear();
        self.originator.clear();
        self.as_on_dt = 0;
        self.rep_freq.clear();
        self.nxt_rep_dt = 0;
        self.alm_line.clear();
        self.yeild = 0.;
        self.deal_name.clear();
        self.org_tenor = 0;
        self.resid_tenor = 0;
        self.total_interest_amount = 0.;
        self.total_principal_amount = 0.;
        self.mis1 = 0;
        self.ia_line.clear();
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
    \n\x0fsec_loans.proto\"R\n\x08Cashflow\x12\x17\n\x07int_amt\x18\x01\x20\
    \x01(\x01R\x06intAmt\x12\x19\n\x08prin_amt\x18\x02\x20\x01(\x01R\x07prin\
    Amt\x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\"\xdc\x07\n\x14Accou\
    ntWithCashflows\x12\x15\n\x06acc_id\x18\x01\x20\x01(\tR\x05accId\x12\x1b\
    \n\tcust_name\x18\x02\x20\x01(\tR\x08custName\x12\x19\n\x08pout_bal\x18\
    \x03\x20\x01(\x01R\x07poutBal\x12\x17\n\x07acc_int\x18\x04\x20\x01(\x01R\
    \x06accInt\x12\x13\n\x05st_dt\x18\x05\x20\x01(\x03R\x04stDt\x12\x11\n\
    \x04c_dt\x18\x06\x20\x01(\x03R\x03cDt\x12\x13\n\x05gl_cd\x18\x07\x20\x01\
    (\tR\x04glCd\x12\x15\n\x06int_rt\x18\x08\x20\x01(\x01R\x05intRt\x12\x17\
    \n\x07int_typ\x18\t\x20\x01(\tR\x06intTyp\x12\x1b\n\tint_bmark\x18\n\x20\
    \x01(\tR\x08intBmark\x12\x17\n\x07rt_flag\x18\x0b\x20\x01(\tR\x06rtFlag\
    \x12\x17\n\x07prod_cd\x18\x0c\x20\x01(\tR\x06prodCd\x12\x1c\n\nnxt_pay_d\
    t\x18\r\x20\x01(\x03R\x08nxtPayDt\x12\x12\n\x04mis2\x18\x0e\x20\x01(\x03\
    R\x04mis2\x12\x10\n\x03ccy\x18\x0f\x20\x01(\tR\x03ccy\x12\x18\n\x07ratin\
    gs\x18\x10\x20\x01(\tR\x07ratings\x12#\n\rrating_agency\x18\x11\x20\x01(\
    \tR\x0cratingAgency\x12\x1f\n\x0basset_class\x18\x12\x20\x01(\tR\nassetC\
    lass\x12\x10\n\x03div\x18\x13\x20\x01(\tR\x03div\x12\x10\n\x03typ\x18\
    \x14\x20\x01(\tR\x03typ\x12\x1e\n\noriginator\x18\x15\x20\x01(\tR\norigi\
    nator\x12\x18\n\x08as_on_dt\x18\x16\x20\x01(\x03R\x06asOnDt\x12\x19\n\
    \x08rep_freq\x18\x17\x20\x01(\tR\x07repFreq\x12\x1c\n\nnxt_rep_dt\x18\
    \x18\x20\x01(\x03R\x08nxtRepDt\x12\x19\n\x08alm_line\x18\x19\x20\x01(\tR\
    \x07almLine\x12\x14\n\x05yeild\x18\x1a\x20\x01(\x01R\x05yeild\x12\x1b\n\
    \tdeal_name\x18\x1b\x20\x01(\tR\x08dealName\x12\x1b\n\torg_tenor\x18\x1c\
    \x20\x01(\x03R\x08orgTenor\x12\x1f\n\x0bresid_tenor\x18\x1d\x20\x01(\x03\
    R\nresidTenor\x122\n\x15total_interest_amount\x18\x1e\x20\x01(\x01R\x13t\
    otalInterestAmount\x124\n\x16total_principal_amount\x18\x1f\x20\x01(\x01\
    R\x14totalPrincipalAmount\x12\x12\n\x04mis1\x18\x20\x20\x01(\x03R\x04mis\
    1\x12\x17\n\x07ia_line\x18!\x20\x01(\tR\x06iaLine\x12\x19\n\x08sma_flag\
    \x18\"\x20\x01(\tR\x07smaFlag\x12'\n\tcashflows\x18#\x20\x03(\x0b2\t.Cas\
    hflowR\tcashflowsb\x06proto3\
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
