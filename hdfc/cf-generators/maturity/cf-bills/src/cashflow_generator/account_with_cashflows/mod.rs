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
    pub reference: ::std::string::String,
    pub cust: ::std::string::String,
    pub curr: ::std::string::String,
    pub val_dt: i64,
    pub mat_dt: i64,
    pub npa_stats: ::std::string::String,
    pub gl: i64,
    pub int_rt: f64,
    pub cust_name: ::std::string::String,
    pub comp_mis1: i64,
    pub comp_mis2: i64,
    pub loan_type: ::std::string::String,
    pub acurl_basis: ::std::string::String,
    pub div: ::std::string::String,
    pub alm_line: ::std::string::String,
    pub ia_llg: ::std::string::String,
    pub balm_llg: ::std::string::String,
    pub as_on_dt: i64,
    pub nxt_rep_dt: i64,
    pub exchange_rt: f64,
    pub asset_class: ::std::string::String,
    pub org_tenor: i64,
    pub tot_int_amt: f64,
    pub tot_prin_amt: f64,
    pub int_st_dt: i64,
    pub def_ftp_flag: ::std::string::String,
    pub bal_os_amt_lcy: f64,
    pub txn_dt: i64,
    pub bill_amt: f64,
    pub concat: ::std::string::String,
    pub rate_flag: ::std::string::String,
    pub comp_mis3: i64,
    pub is_acc_weaker: ::std::string::String,
    pub ews_weaker_value: ::std::string::String,
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

    // string reference = 1;


    pub fn get_reference(&self) -> &str {
        &self.reference
    }
    pub fn clear_reference(&mut self) {
        self.reference.clear();
    }

    // Param is passed by value, moved
    pub fn set_reference(&mut self, v: ::std::string::String) {
        self.reference = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reference(&mut self) -> &mut ::std::string::String {
        &mut self.reference
    }

    // Take field
    pub fn take_reference(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.reference, ::std::string::String::new())
    }

    // string cust = 2;


    pub fn get_cust(&self) -> &str {
        &self.cust
    }
    pub fn clear_cust(&mut self) {
        self.cust.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust(&mut self, v: ::std::string::String) {
        self.cust = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust(&mut self) -> &mut ::std::string::String {
        &mut self.cust
    }

    // Take field
    pub fn take_cust(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust, ::std::string::String::new())
    }

    // string curr = 3;


    pub fn get_curr(&self) -> &str {
        &self.curr
    }
    pub fn clear_curr(&mut self) {
        self.curr.clear();
    }

    // Param is passed by value, moved
    pub fn set_curr(&mut self, v: ::std::string::String) {
        self.curr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_curr(&mut self) -> &mut ::std::string::String {
        &mut self.curr
    }

    // Take field
    pub fn take_curr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.curr, ::std::string::String::new())
    }

    // int64 val_dt = 4;


    pub fn get_val_dt(&self) -> i64 {
        self.val_dt
    }
    pub fn clear_val_dt(&mut self) {
        self.val_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_val_dt(&mut self, v: i64) {
        self.val_dt = v;
    }

    // int64 mat_dt = 5;


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

    // string npa_stats = 6;


    pub fn get_npa_stats(&self) -> &str {
        &self.npa_stats
    }
    pub fn clear_npa_stats(&mut self) {
        self.npa_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_npa_stats(&mut self, v: ::std::string::String) {
        self.npa_stats = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_npa_stats(&mut self) -> &mut ::std::string::String {
        &mut self.npa_stats
    }

    // Take field
    pub fn take_npa_stats(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.npa_stats, ::std::string::String::new())
    }

    // int64 gl = 7;


    pub fn get_gl(&self) -> i64 {
        self.gl
    }
    pub fn clear_gl(&mut self) {
        self.gl = 0;
    }

    // Param is passed by value, moved
    pub fn set_gl(&mut self, v: i64) {
        self.gl = v;
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

    // string cust_name = 9;


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

    // int64 comp_mis1 = 10;


    pub fn get_comp_mis1(&self) -> i64 {
        self.comp_mis1
    }
    pub fn clear_comp_mis1(&mut self) {
        self.comp_mis1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_comp_mis1(&mut self, v: i64) {
        self.comp_mis1 = v;
    }

    // int64 comp_mis2 = 11;


    pub fn get_comp_mis2(&self) -> i64 {
        self.comp_mis2
    }
    pub fn clear_comp_mis2(&mut self) {
        self.comp_mis2 = 0;
    }

    // Param is passed by value, moved
    pub fn set_comp_mis2(&mut self, v: i64) {
        self.comp_mis2 = v;
    }

    // string loan_type = 12;


    pub fn get_loan_type(&self) -> &str {
        &self.loan_type
    }
    pub fn clear_loan_type(&mut self) {
        self.loan_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_loan_type(&mut self, v: ::std::string::String) {
        self.loan_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loan_type(&mut self) -> &mut ::std::string::String {
        &mut self.loan_type
    }

    // Take field
    pub fn take_loan_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.loan_type, ::std::string::String::new())
    }

    // string acurl_basis = 13;


    pub fn get_acurl_basis(&self) -> &str {
        &self.acurl_basis
    }
    pub fn clear_acurl_basis(&mut self) {
        self.acurl_basis.clear();
    }

    // Param is passed by value, moved
    pub fn set_acurl_basis(&mut self, v: ::std::string::String) {
        self.acurl_basis = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acurl_basis(&mut self) -> &mut ::std::string::String {
        &mut self.acurl_basis
    }

    // Take field
    pub fn take_acurl_basis(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.acurl_basis, ::std::string::String::new())
    }

    // string div = 14;


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

    // string alm_line = 15;


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

    // string ia_llg = 16;


    pub fn get_ia_llg(&self) -> &str {
        &self.ia_llg
    }
    pub fn clear_ia_llg(&mut self) {
        self.ia_llg.clear();
    }

    // Param is passed by value, moved
    pub fn set_ia_llg(&mut self, v: ::std::string::String) {
        self.ia_llg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ia_llg(&mut self) -> &mut ::std::string::String {
        &mut self.ia_llg
    }

    // Take field
    pub fn take_ia_llg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ia_llg, ::std::string::String::new())
    }

    // string balm_llg = 17;


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

    // int64 as_on_dt = 18;


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

    // int64 nxt_rep_dt = 19;


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

    // double exchange_rt = 20;


    pub fn get_exchange_rt(&self) -> f64 {
        self.exchange_rt
    }
    pub fn clear_exchange_rt(&mut self) {
        self.exchange_rt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_exchange_rt(&mut self, v: f64) {
        self.exchange_rt = v;
    }

    // string asset_class = 21;


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

    // int64 org_tenor = 22;


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

    // double tot_int_amt = 23;


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

    // double tot_prin_amt = 24;


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

    // int64 int_st_dt = 25;


    pub fn get_int_st_dt(&self) -> i64 {
        self.int_st_dt
    }
    pub fn clear_int_st_dt(&mut self) {
        self.int_st_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_int_st_dt(&mut self, v: i64) {
        self.int_st_dt = v;
    }

    // string def_ftp_flag = 26;


    pub fn get_def_ftp_flag(&self) -> &str {
        &self.def_ftp_flag
    }
    pub fn clear_def_ftp_flag(&mut self) {
        self.def_ftp_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_def_ftp_flag(&mut self, v: ::std::string::String) {
        self.def_ftp_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_def_ftp_flag(&mut self) -> &mut ::std::string::String {
        &mut self.def_ftp_flag
    }

    // Take field
    pub fn take_def_ftp_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.def_ftp_flag, ::std::string::String::new())
    }

    // double bal_os_amt_lcy = 27;


    pub fn get_bal_os_amt_lcy(&self) -> f64 {
        self.bal_os_amt_lcy
    }
    pub fn clear_bal_os_amt_lcy(&mut self) {
        self.bal_os_amt_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bal_os_amt_lcy(&mut self, v: f64) {
        self.bal_os_amt_lcy = v;
    }

    // int64 txn_dt = 28;


    pub fn get_txn_dt(&self) -> i64 {
        self.txn_dt
    }
    pub fn clear_txn_dt(&mut self) {
        self.txn_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_txn_dt(&mut self, v: i64) {
        self.txn_dt = v;
    }

    // double bill_amt = 29;


    pub fn get_bill_amt(&self) -> f64 {
        self.bill_amt
    }
    pub fn clear_bill_amt(&mut self) {
        self.bill_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bill_amt(&mut self, v: f64) {
        self.bill_amt = v;
    }

    // string concat = 30;


    pub fn get_concat(&self) -> &str {
        &self.concat
    }
    pub fn clear_concat(&mut self) {
        self.concat.clear();
    }

    // Param is passed by value, moved
    pub fn set_concat(&mut self, v: ::std::string::String) {
        self.concat = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_concat(&mut self) -> &mut ::std::string::String {
        &mut self.concat
    }

    // Take field
    pub fn take_concat(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.concat, ::std::string::String::new())
    }

    // string rate_flag = 31;


    pub fn get_rate_flag(&self) -> &str {
        &self.rate_flag
    }
    pub fn clear_rate_flag(&mut self) {
        self.rate_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_rate_flag(&mut self, v: ::std::string::String) {
        self.rate_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rate_flag(&mut self) -> &mut ::std::string::String {
        &mut self.rate_flag
    }

    // Take field
    pub fn take_rate_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rate_flag, ::std::string::String::new())
    }

    // int64 comp_mis3 = 32;


    pub fn get_comp_mis3(&self) -> i64 {
        self.comp_mis3
    }
    pub fn clear_comp_mis3(&mut self) {
        self.comp_mis3 = 0;
    }

    // Param is passed by value, moved
    pub fn set_comp_mis3(&mut self, v: i64) {
        self.comp_mis3 = v;
    }

    // string is_acc_weaker = 33;


    pub fn get_is_acc_weaker(&self) -> &str {
        &self.is_acc_weaker
    }
    pub fn clear_is_acc_weaker(&mut self) {
        self.is_acc_weaker.clear();
    }

    // Param is passed by value, moved
    pub fn set_is_acc_weaker(&mut self, v: ::std::string::String) {
        self.is_acc_weaker = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_acc_weaker(&mut self) -> &mut ::std::string::String {
        &mut self.is_acc_weaker
    }

    // Take field
    pub fn take_is_acc_weaker(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.is_acc_weaker, ::std::string::String::new())
    }

    // string ews_weaker_value = 34;


    pub fn get_ews_weaker_value(&self) -> &str {
        &self.ews_weaker_value
    }
    pub fn clear_ews_weaker_value(&mut self) {
        self.ews_weaker_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_ews_weaker_value(&mut self, v: ::std::string::String) {
        self.ews_weaker_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ews_weaker_value(&mut self) -> &mut ::std::string::String {
        &mut self.ews_weaker_value
    }

    // Take field
    pub fn take_ews_weaker_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ews_weaker_value, ::std::string::String::new())
    }

    // string sma_flag = 35;


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

    // repeated .Cashflow cashflows = 36;


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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reference)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.curr)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.val_dt = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_dt = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.npa_stats)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gl = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_rt = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_name)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.comp_mis1 = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.comp_mis2 = tmp;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loan_type)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.acurl_basis)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.div)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ia_llg)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balm_llg)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.as_on_dt = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_rep_dt = tmp;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.exchange_rt = tmp;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.asset_class)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.org_tenor = tmp;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_int_amt = tmp;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_prin_amt = tmp;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.int_st_dt = tmp;
                },
                26 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.def_ftp_flag)?;
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.bal_os_amt_lcy = tmp;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.txn_dt = tmp;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.bill_amt = tmp;
                },
                30 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.concat)?;
                },
                31 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rate_flag)?;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.comp_mis3 = tmp;
                },
                33 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.is_acc_weaker)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ews_weaker_value)?;
                },
                35 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                36 => {
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
        if !self.reference.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.reference);
        }
        if !self.cust.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cust);
        }
        if !self.curr.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.curr);
        }
        if self.val_dt != 0 {
            my_size += ::protobuf::rt::value_size(4, self.val_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mat_dt != 0 {
            my_size += ::protobuf::rt::value_size(5, self.mat_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.npa_stats.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.npa_stats);
        }
        if self.gl != 0 {
            my_size += ::protobuf::rt::value_size(7, self.gl, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.int_rt != 0. {
            my_size += 9;
        }
        if !self.cust_name.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.cust_name);
        }
        if self.comp_mis1 != 0 {
            my_size += ::protobuf::rt::value_size(10, self.comp_mis1, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.comp_mis2 != 0 {
            my_size += ::protobuf::rt::value_size(11, self.comp_mis2, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.loan_type.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.loan_type);
        }
        if !self.acurl_basis.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.acurl_basis);
        }
        if !self.div.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.div);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.alm_line);
        }
        if !self.ia_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.ia_llg);
        }
        if !self.balm_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.balm_llg);
        }
        if self.as_on_dt != 0 {
            my_size += ::protobuf::rt::value_size(18, self.as_on_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nxt_rep_dt != 0 {
            my_size += ::protobuf::rt::value_size(19, self.nxt_rep_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.exchange_rt != 0. {
            my_size += 10;
        }
        if !self.asset_class.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.asset_class);
        }
        if self.org_tenor != 0 {
            my_size += ::protobuf::rt::value_size(22, self.org_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.tot_int_amt != 0. {
            my_size += 10;
        }
        if self.tot_prin_amt != 0. {
            my_size += 10;
        }
        if self.int_st_dt != 0 {
            my_size += ::protobuf::rt::value_size(25, self.int_st_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.def_ftp_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(26, &self.def_ftp_flag);
        }
        if self.bal_os_amt_lcy != 0. {
            my_size += 10;
        }
        if self.txn_dt != 0 {
            my_size += ::protobuf::rt::value_size(28, self.txn_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bill_amt != 0. {
            my_size += 10;
        }
        if !self.concat.is_empty() {
            my_size += ::protobuf::rt::string_size(30, &self.concat);
        }
        if !self.rate_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(31, &self.rate_flag);
        }
        if self.comp_mis3 != 0 {
            my_size += ::protobuf::rt::value_size(32, self.comp_mis3, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.is_acc_weaker.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.is_acc_weaker);
        }
        if !self.ews_weaker_value.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.ews_weaker_value);
        }
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(35, &self.sma_flag);
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
        if !self.reference.is_empty() {
            os.write_string(1, &self.reference)?;
        }
        if !self.cust.is_empty() {
            os.write_string(2, &self.cust)?;
        }
        if !self.curr.is_empty() {
            os.write_string(3, &self.curr)?;
        }
        if self.val_dt != 0 {
            os.write_int64(4, self.val_dt)?;
        }
        if self.mat_dt != 0 {
            os.write_int64(5, self.mat_dt)?;
        }
        if !self.npa_stats.is_empty() {
            os.write_string(6, &self.npa_stats)?;
        }
        if self.gl != 0 {
            os.write_int64(7, self.gl)?;
        }
        if self.int_rt != 0. {
            os.write_double(8, self.int_rt)?;
        }
        if !self.cust_name.is_empty() {
            os.write_string(9, &self.cust_name)?;
        }
        if self.comp_mis1 != 0 {
            os.write_int64(10, self.comp_mis1)?;
        }
        if self.comp_mis2 != 0 {
            os.write_int64(11, self.comp_mis2)?;
        }
        if !self.loan_type.is_empty() {
            os.write_string(12, &self.loan_type)?;
        }
        if !self.acurl_basis.is_empty() {
            os.write_string(13, &self.acurl_basis)?;
        }
        if !self.div.is_empty() {
            os.write_string(14, &self.div)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(15, &self.alm_line)?;
        }
        if !self.ia_llg.is_empty() {
            os.write_string(16, &self.ia_llg)?;
        }
        if !self.balm_llg.is_empty() {
            os.write_string(17, &self.balm_llg)?;
        }
        if self.as_on_dt != 0 {
            os.write_int64(18, self.as_on_dt)?;
        }
        if self.nxt_rep_dt != 0 {
            os.write_int64(19, self.nxt_rep_dt)?;
        }
        if self.exchange_rt != 0. {
            os.write_double(20, self.exchange_rt)?;
        }
        if !self.asset_class.is_empty() {
            os.write_string(21, &self.asset_class)?;
        }
        if self.org_tenor != 0 {
            os.write_int64(22, self.org_tenor)?;
        }
        if self.tot_int_amt != 0. {
            os.write_double(23, self.tot_int_amt)?;
        }
        if self.tot_prin_amt != 0. {
            os.write_double(24, self.tot_prin_amt)?;
        }
        if self.int_st_dt != 0 {
            os.write_int64(25, self.int_st_dt)?;
        }
        if !self.def_ftp_flag.is_empty() {
            os.write_string(26, &self.def_ftp_flag)?;
        }
        if self.bal_os_amt_lcy != 0. {
            os.write_double(27, self.bal_os_amt_lcy)?;
        }
        if self.txn_dt != 0 {
            os.write_int64(28, self.txn_dt)?;
        }
        if self.bill_amt != 0. {
            os.write_double(29, self.bill_amt)?;
        }
        if !self.concat.is_empty() {
            os.write_string(30, &self.concat)?;
        }
        if !self.rate_flag.is_empty() {
            os.write_string(31, &self.rate_flag)?;
        }
        if self.comp_mis3 != 0 {
            os.write_int64(32, self.comp_mis3)?;
        }
        if !self.is_acc_weaker.is_empty() {
            os.write_string(33, &self.is_acc_weaker)?;
        }
        if !self.ews_weaker_value.is_empty() {
            os.write_string(34, &self.ews_weaker_value)?;
        }
        if !self.sma_flag.is_empty() {
            os.write_string(35, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(36, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                "reference",
                |m: &AccountWithCashflows| { &m.reference },
                |m: &mut AccountWithCashflows| { &mut m.reference },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust",
                |m: &AccountWithCashflows| { &m.cust },
                |m: &mut AccountWithCashflows| { &mut m.cust },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "curr",
                |m: &AccountWithCashflows| { &m.curr },
                |m: &mut AccountWithCashflows| { &mut m.curr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "val_dt",
                |m: &AccountWithCashflows| { &m.val_dt },
                |m: &mut AccountWithCashflows| { &mut m.val_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mat_dt",
                |m: &AccountWithCashflows| { &m.mat_dt },
                |m: &mut AccountWithCashflows| { &mut m.mat_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "npa_stats",
                |m: &AccountWithCashflows| { &m.npa_stats },
                |m: &mut AccountWithCashflows| { &mut m.npa_stats },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "gl",
                |m: &AccountWithCashflows| { &m.gl },
                |m: &mut AccountWithCashflows| { &mut m.gl },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_rt",
                |m: &AccountWithCashflows| { &m.int_rt },
                |m: &mut AccountWithCashflows| { &mut m.int_rt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_name",
                |m: &AccountWithCashflows| { &m.cust_name },
                |m: &mut AccountWithCashflows| { &mut m.cust_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "comp_mis1",
                |m: &AccountWithCashflows| { &m.comp_mis1 },
                |m: &mut AccountWithCashflows| { &mut m.comp_mis1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "comp_mis2",
                |m: &AccountWithCashflows| { &m.comp_mis2 },
                |m: &mut AccountWithCashflows| { &mut m.comp_mis2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "loan_type",
                |m: &AccountWithCashflows| { &m.loan_type },
                |m: &mut AccountWithCashflows| { &mut m.loan_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "acurl_basis",
                |m: &AccountWithCashflows| { &m.acurl_basis },
                |m: &mut AccountWithCashflows| { &mut m.acurl_basis },
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
                "ia_llg",
                |m: &AccountWithCashflows| { &m.ia_llg },
                |m: &mut AccountWithCashflows| { &mut m.ia_llg },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "balm_llg",
                |m: &AccountWithCashflows| { &m.balm_llg },
                |m: &mut AccountWithCashflows| { &mut m.balm_llg },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "as_on_dt",
                |m: &AccountWithCashflows| { &m.as_on_dt },
                |m: &mut AccountWithCashflows| { &mut m.as_on_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_rep_dt",
                |m: &AccountWithCashflows| { &m.nxt_rep_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_rep_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "exchange_rt",
                |m: &AccountWithCashflows| { &m.exchange_rt },
                |m: &mut AccountWithCashflows| { &mut m.exchange_rt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "asset_class",
                |m: &AccountWithCashflows| { &m.asset_class },
                |m: &mut AccountWithCashflows| { &mut m.asset_class },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "org_tenor",
                |m: &AccountWithCashflows| { &m.org_tenor },
                |m: &mut AccountWithCashflows| { &mut m.org_tenor },
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "int_st_dt",
                |m: &AccountWithCashflows| { &m.int_st_dt },
                |m: &mut AccountWithCashflows| { &mut m.int_st_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "def_ftp_flag",
                |m: &AccountWithCashflows| { &m.def_ftp_flag },
                |m: &mut AccountWithCashflows| { &mut m.def_ftp_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "bal_os_amt_lcy",
                |m: &AccountWithCashflows| { &m.bal_os_amt_lcy },
                |m: &mut AccountWithCashflows| { &mut m.bal_os_amt_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "txn_dt",
                |m: &AccountWithCashflows| { &m.txn_dt },
                |m: &mut AccountWithCashflows| { &mut m.txn_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "bill_amt",
                |m: &AccountWithCashflows| { &m.bill_amt },
                |m: &mut AccountWithCashflows| { &mut m.bill_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "concat",
                |m: &AccountWithCashflows| { &m.concat },
                |m: &mut AccountWithCashflows| { &mut m.concat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rate_flag",
                |m: &AccountWithCashflows| { &m.rate_flag },
                |m: &mut AccountWithCashflows| { &mut m.rate_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "comp_mis3",
                |m: &AccountWithCashflows| { &m.comp_mis3 },
                |m: &mut AccountWithCashflows| { &mut m.comp_mis3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "is_acc_weaker",
                |m: &AccountWithCashflows| { &m.is_acc_weaker },
                |m: &mut AccountWithCashflows| { &mut m.is_acc_weaker },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ews_weaker_value",
                |m: &AccountWithCashflows| { &m.ews_weaker_value },
                |m: &mut AccountWithCashflows| { &mut m.ews_weaker_value },
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
        self.reference.clear();
        self.cust.clear();
        self.curr.clear();
        self.val_dt = 0;
        self.mat_dt = 0;
        self.npa_stats.clear();
        self.gl = 0;
        self.int_rt = 0.;
        self.cust_name.clear();
        self.comp_mis1 = 0;
        self.comp_mis2 = 0;
        self.loan_type.clear();
        self.acurl_basis.clear();
        self.div.clear();
        self.alm_line.clear();
        self.ia_llg.clear();
        self.balm_llg.clear();
        self.as_on_dt = 0;
        self.nxt_rep_dt = 0;
        self.exchange_rt = 0.;
        self.asset_class.clear();
        self.org_tenor = 0;
        self.tot_int_amt = 0.;
        self.tot_prin_amt = 0.;
        self.int_st_dt = 0;
        self.def_ftp_flag.clear();
        self.bal_os_amt_lcy = 0.;
        self.txn_dt = 0;
        self.bill_amt = 0.;
        self.concat.clear();
        self.rate_flag.clear();
        self.comp_mis3 = 0;
        self.is_acc_weaker.clear();
        self.ews_weaker_value.clear();
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
    \n\x13ubs_bills_dom.proto\"R\n\x08Cashflow\x12\x17\n\x07int_amt\x18\x01\
    \x20\x01(\x01R\x06intAmt\x12\x19\n\x08prin_amt\x18\x02\x20\x01(\x01R\x07\
    prinAmt\x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\"\x94\x08\n\x14A\
    ccountWithCashflows\x12\x1c\n\treference\x18\x01\x20\x01(\tR\treference\
    \x12\x12\n\x04cust\x18\x02\x20\x01(\tR\x04cust\x12\x12\n\x04curr\x18\x03\
    \x20\x01(\tR\x04curr\x12\x15\n\x06val_dt\x18\x04\x20\x01(\x03R\x05valDt\
    \x12\x15\n\x06mat_dt\x18\x05\x20\x01(\x03R\x05matDt\x12\x1b\n\tnpa_stats\
    \x18\x06\x20\x01(\tR\x08npaStats\x12\x0e\n\x02gl\x18\x07\x20\x01(\x03R\
    \x02gl\x12\x15\n\x06int_rt\x18\x08\x20\x01(\x01R\x05intRt\x12\x1b\n\tcus\
    t_name\x18\t\x20\x01(\tR\x08custName\x12\x1b\n\tcomp_mis1\x18\n\x20\x01(\
    \x03R\x08compMis1\x12\x1b\n\tcomp_mis2\x18\x0b\x20\x01(\x03R\x08compMis2\
    \x12\x1b\n\tloan_type\x18\x0c\x20\x01(\tR\x08loanType\x12\x1f\n\x0bacurl\
    _basis\x18\r\x20\x01(\tR\nacurlBasis\x12\x10\n\x03div\x18\x0e\x20\x01(\t\
    R\x03div\x12\x19\n\x08alm_line\x18\x0f\x20\x01(\tR\x07almLine\x12\x15\n\
    \x06ia_llg\x18\x10\x20\x01(\tR\x05iaLlg\x12\x19\n\x08balm_llg\x18\x11\
    \x20\x01(\tR\x07balmLlg\x12\x18\n\x08as_on_dt\x18\x12\x20\x01(\x03R\x06a\
    sOnDt\x12\x1c\n\nnxt_rep_dt\x18\x13\x20\x01(\x03R\x08nxtRepDt\x12\x1f\n\
    \x0bexchange_rt\x18\x14\x20\x01(\x01R\nexchangeRt\x12\x1f\n\x0basset_cla\
    ss\x18\x15\x20\x01(\tR\nassetClass\x12\x1b\n\torg_tenor\x18\x16\x20\x01(\
    \x03R\x08orgTenor\x12\x1e\n\x0btot_int_amt\x18\x17\x20\x01(\x01R\ttotInt\
    Amt\x12\x20\n\x0ctot_prin_amt\x18\x18\x20\x01(\x01R\ntotPrinAmt\x12\x1a\
    \n\tint_st_dt\x18\x19\x20\x01(\x03R\x07intStDt\x12\x20\n\x0cdef_ftp_flag\
    \x18\x1a\x20\x01(\tR\ndefFtpFlag\x12#\n\x0ebal_os_amt_lcy\x18\x1b\x20\
    \x01(\x01R\x0bbalOsAmtLcy\x12\x15\n\x06txn_dt\x18\x1c\x20\x01(\x03R\x05t\
    xnDt\x12\x19\n\x08bill_amt\x18\x1d\x20\x01(\x01R\x07billAmt\x12\x16\n\
    \x06concat\x18\x1e\x20\x01(\tR\x06concat\x12\x1b\n\trate_flag\x18\x1f\
    \x20\x01(\tR\x08rateFlag\x12\x1b\n\tcomp_mis3\x18\x20\x20\x01(\x03R\x08c\
    ompMis3\x12\"\n\ris_acc_weaker\x18!\x20\x01(\tR\x0bisAccWeaker\x12(\n\
    \x10ews_weaker_value\x18\"\x20\x01(\tR\x0eewsWeakerValue\x12\x19\n\x08sm\
    a_flag\x18#\x20\x01(\tR\x07smaFlag\x12'\n\tcashflows\x18$\x20\x03(\x0b2\
    \t.CashflowR\tcashflowsb\x06proto3\
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
