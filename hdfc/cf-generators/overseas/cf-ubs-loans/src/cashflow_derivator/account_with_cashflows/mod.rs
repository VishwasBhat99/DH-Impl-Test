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
    pub cust_no: ::std::string::String,
    pub reference: ::std::string::String,
    pub cust_name: ::std::string::String,
    pub branch_cd: ::std::string::String,
    pub norm_int_rt: f64,
    pub acurl_freq: ::std::string::String,
    pub book_dt: i64,
    pub val_dt: i64,
    pub mat_dt: i64,
    pub user_def_stats: ::std::string::String,
    pub prod_cd: ::std::string::String,
    pub gl: ::std::string::String,
    pub curr: ::std::string::String,
    pub prin_ost_bal: f64,
    pub spread: f64,
    pub compmis2: i64,
    pub rt_flag_new: ::std::string::String,
    pub rt_cd_new: ::std::string::String,
    pub division: ::std::string::String,
    pub alm_line: ::std::string::String,
    pub ia_llg: ::std::string::String,
    pub balm_llg: ::std::string::String,
    pub repricing_freq: ::std::string::String,
    pub lst_repricing_dt: i64,
    pub nxt_repricing_dt: i64,
    pub int_basis: ::std::string::String,
    pub cust_typ: ::std::string::String,
    pub npa_typ: ::std::string::String,
    pub bmid: ::std::string::String,
    pub compmis1: i64,
    pub compmis3: i64,
    pub org_tenor: i64,
    pub resid_tenor: i64,
    pub def_ftp_flag: ::std::string::String,
    pub concat: ::std::string::String,
    pub cntr_party: ::std::string::String,
    pub lcy_amount: f64,
    pub raw_benchmark: ::std::string::String,
    pub total_interest_amount: f64,
    pub total_principal_amount: f64,
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

    // string cust_no = 1;


    pub fn get_cust_no(&self) -> &str {
        &self.cust_no
    }
    pub fn clear_cust_no(&mut self) {
        self.cust_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_no(&mut self, v: ::std::string::String) {
        self.cust_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_no(&mut self) -> &mut ::std::string::String {
        &mut self.cust_no
    }

    // Take field
    pub fn take_cust_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_no, ::std::string::String::new())
    }

    // string reference = 2;


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

    // string cust_name = 3;


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

    // string branch_cd = 4;


    pub fn get_branch_cd(&self) -> &str {
        &self.branch_cd
    }
    pub fn clear_branch_cd(&mut self) {
        self.branch_cd.clear();
    }

    // Param is passed by value, moved
    pub fn set_branch_cd(&mut self, v: ::std::string::String) {
        self.branch_cd = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_branch_cd(&mut self) -> &mut ::std::string::String {
        &mut self.branch_cd
    }

    // Take field
    pub fn take_branch_cd(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.branch_cd, ::std::string::String::new())
    }

    // double norm_int_rt = 5;


    pub fn get_norm_int_rt(&self) -> f64 {
        self.norm_int_rt
    }
    pub fn clear_norm_int_rt(&mut self) {
        self.norm_int_rt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_norm_int_rt(&mut self, v: f64) {
        self.norm_int_rt = v;
    }

    // string acurl_freq = 6;


    pub fn get_acurl_freq(&self) -> &str {
        &self.acurl_freq
    }
    pub fn clear_acurl_freq(&mut self) {
        self.acurl_freq.clear();
    }

    // Param is passed by value, moved
    pub fn set_acurl_freq(&mut self, v: ::std::string::String) {
        self.acurl_freq = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acurl_freq(&mut self) -> &mut ::std::string::String {
        &mut self.acurl_freq
    }

    // Take field
    pub fn take_acurl_freq(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.acurl_freq, ::std::string::String::new())
    }

    // int64 book_dt = 7;


    pub fn get_book_dt(&self) -> i64 {
        self.book_dt
    }
    pub fn clear_book_dt(&mut self) {
        self.book_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_book_dt(&mut self, v: i64) {
        self.book_dt = v;
    }

    // int64 val_dt = 8;


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

    // string user_def_stats = 10;


    pub fn get_user_def_stats(&self) -> &str {
        &self.user_def_stats
    }
    pub fn clear_user_def_stats(&mut self) {
        self.user_def_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_def_stats(&mut self, v: ::std::string::String) {
        self.user_def_stats = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_def_stats(&mut self) -> &mut ::std::string::String {
        &mut self.user_def_stats
    }

    // Take field
    pub fn take_user_def_stats(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_def_stats, ::std::string::String::new())
    }

    // string prod_cd = 11;


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

    // string gl = 12;


    pub fn get_gl(&self) -> &str {
        &self.gl
    }
    pub fn clear_gl(&mut self) {
        self.gl.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl(&mut self, v: ::std::string::String) {
        self.gl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl(&mut self) -> &mut ::std::string::String {
        &mut self.gl
    }

    // Take field
    pub fn take_gl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl, ::std::string::String::new())
    }

    // string curr = 13;


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

    // double prin_ost_bal = 14;


    pub fn get_prin_ost_bal(&self) -> f64 {
        self.prin_ost_bal
    }
    pub fn clear_prin_ost_bal(&mut self) {
        self.prin_ost_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_prin_ost_bal(&mut self, v: f64) {
        self.prin_ost_bal = v;
    }

    // double spread = 15;


    pub fn get_spread(&self) -> f64 {
        self.spread
    }
    pub fn clear_spread(&mut self) {
        self.spread = 0.;
    }

    // Param is passed by value, moved
    pub fn set_spread(&mut self, v: f64) {
        self.spread = v;
    }

    // int64 compmis2 = 16;


    pub fn get_compmis2(&self) -> i64 {
        self.compmis2
    }
    pub fn clear_compmis2(&mut self) {
        self.compmis2 = 0;
    }

    // Param is passed by value, moved
    pub fn set_compmis2(&mut self, v: i64) {
        self.compmis2 = v;
    }

    // string rt_flag_new = 17;


    pub fn get_rt_flag_new(&self) -> &str {
        &self.rt_flag_new
    }
    pub fn clear_rt_flag_new(&mut self) {
        self.rt_flag_new.clear();
    }

    // Param is passed by value, moved
    pub fn set_rt_flag_new(&mut self, v: ::std::string::String) {
        self.rt_flag_new = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rt_flag_new(&mut self) -> &mut ::std::string::String {
        &mut self.rt_flag_new
    }

    // Take field
    pub fn take_rt_flag_new(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rt_flag_new, ::std::string::String::new())
    }

    // string rt_cd_new = 18;


    pub fn get_rt_cd_new(&self) -> &str {
        &self.rt_cd_new
    }
    pub fn clear_rt_cd_new(&mut self) {
        self.rt_cd_new.clear();
    }

    // Param is passed by value, moved
    pub fn set_rt_cd_new(&mut self, v: ::std::string::String) {
        self.rt_cd_new = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rt_cd_new(&mut self) -> &mut ::std::string::String {
        &mut self.rt_cd_new
    }

    // Take field
    pub fn take_rt_cd_new(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rt_cd_new, ::std::string::String::new())
    }

    // string division = 19;


    pub fn get_division(&self) -> &str {
        &self.division
    }
    pub fn clear_division(&mut self) {
        self.division.clear();
    }

    // Param is passed by value, moved
    pub fn set_division(&mut self, v: ::std::string::String) {
        self.division = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_division(&mut self) -> &mut ::std::string::String {
        &mut self.division
    }

    // Take field
    pub fn take_division(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.division, ::std::string::String::new())
    }

    // string alm_line = 20;


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

    // string ia_llg = 21;


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

    // string balm_llg = 22;


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

    // string repricing_freq = 23;


    pub fn get_repricing_freq(&self) -> &str {
        &self.repricing_freq
    }
    pub fn clear_repricing_freq(&mut self) {
        self.repricing_freq.clear();
    }

    // Param is passed by value, moved
    pub fn set_repricing_freq(&mut self, v: ::std::string::String) {
        self.repricing_freq = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repricing_freq(&mut self) -> &mut ::std::string::String {
        &mut self.repricing_freq
    }

    // Take field
    pub fn take_repricing_freq(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.repricing_freq, ::std::string::String::new())
    }

    // int64 lst_repricing_dt = 24;


    pub fn get_lst_repricing_dt(&self) -> i64 {
        self.lst_repricing_dt
    }
    pub fn clear_lst_repricing_dt(&mut self) {
        self.lst_repricing_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_lst_repricing_dt(&mut self, v: i64) {
        self.lst_repricing_dt = v;
    }

    // int64 nxt_repricing_dt = 25;


    pub fn get_nxt_repricing_dt(&self) -> i64 {
        self.nxt_repricing_dt
    }
    pub fn clear_nxt_repricing_dt(&mut self) {
        self.nxt_repricing_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_nxt_repricing_dt(&mut self, v: i64) {
        self.nxt_repricing_dt = v;
    }

    // string int_basis = 26;


    pub fn get_int_basis(&self) -> &str {
        &self.int_basis
    }
    pub fn clear_int_basis(&mut self) {
        self.int_basis.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_basis(&mut self, v: ::std::string::String) {
        self.int_basis = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int_basis(&mut self) -> &mut ::std::string::String {
        &mut self.int_basis
    }

    // Take field
    pub fn take_int_basis(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int_basis, ::std::string::String::new())
    }

    // string cust_typ = 27;


    pub fn get_cust_typ(&self) -> &str {
        &self.cust_typ
    }
    pub fn clear_cust_typ(&mut self) {
        self.cust_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_typ(&mut self, v: ::std::string::String) {
        self.cust_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_typ(&mut self) -> &mut ::std::string::String {
        &mut self.cust_typ
    }

    // Take field
    pub fn take_cust_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_typ, ::std::string::String::new())
    }

    // string npa_typ = 28;


    pub fn get_npa_typ(&self) -> &str {
        &self.npa_typ
    }
    pub fn clear_npa_typ(&mut self) {
        self.npa_typ.clear();
    }

    // Param is passed by value, moved
    pub fn set_npa_typ(&mut self, v: ::std::string::String) {
        self.npa_typ = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_npa_typ(&mut self) -> &mut ::std::string::String {
        &mut self.npa_typ
    }

    // Take field
    pub fn take_npa_typ(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.npa_typ, ::std::string::String::new())
    }

    // string bmid = 29;


    pub fn get_bmid(&self) -> &str {
        &self.bmid
    }
    pub fn clear_bmid(&mut self) {
        self.bmid.clear();
    }

    // Param is passed by value, moved
    pub fn set_bmid(&mut self, v: ::std::string::String) {
        self.bmid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bmid(&mut self) -> &mut ::std::string::String {
        &mut self.bmid
    }

    // Take field
    pub fn take_bmid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.bmid, ::std::string::String::new())
    }

    // int64 compmis1 = 30;


    pub fn get_compmis1(&self) -> i64 {
        self.compmis1
    }
    pub fn clear_compmis1(&mut self) {
        self.compmis1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_compmis1(&mut self, v: i64) {
        self.compmis1 = v;
    }

    // int64 compmis3 = 31;


    pub fn get_compmis3(&self) -> i64 {
        self.compmis3
    }
    pub fn clear_compmis3(&mut self) {
        self.compmis3 = 0;
    }

    // Param is passed by value, moved
    pub fn set_compmis3(&mut self, v: i64) {
        self.compmis3 = v;
    }

    // int64 org_tenor = 32;


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

    // int64 resid_tenor = 33;


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

    // string def_ftp_flag = 34;


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

    // string concat = 35;


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

    // string cntr_party = 36;


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

    // double lcy_amount = 37;


    pub fn get_lcy_amount(&self) -> f64 {
        self.lcy_amount
    }
    pub fn clear_lcy_amount(&mut self) {
        self.lcy_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_lcy_amount(&mut self, v: f64) {
        self.lcy_amount = v;
    }

    // string raw_benchmark = 38;


    pub fn get_raw_benchmark(&self) -> &str {
        &self.raw_benchmark
    }
    pub fn clear_raw_benchmark(&mut self) {
        self.raw_benchmark.clear();
    }

    // Param is passed by value, moved
    pub fn set_raw_benchmark(&mut self, v: ::std::string::String) {
        self.raw_benchmark = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_benchmark(&mut self) -> &mut ::std::string::String {
        &mut self.raw_benchmark
    }

    // Take field
    pub fn take_raw_benchmark(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.raw_benchmark, ::std::string::String::new())
    }

    // double total_interest_amount = 39;


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

    // double total_principal_amount = 40;


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

    // string sma_flag = 41;


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

    // repeated .Cashflow cashflows = 42;


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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_no)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.reference)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.branch_cd)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.norm_int_rt = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.acurl_freq)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.book_dt = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.val_dt = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_dt = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_def_stats)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prod_cd)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.curr)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.prin_ost_bal = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.spread = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compmis2 = tmp;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rt_flag_new)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rt_cd_new)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.division)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ia_llg)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balm_llg)?;
                },
                23 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.repricing_freq)?;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lst_repricing_dt = tmp;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.nxt_repricing_dt = tmp;
                },
                26 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int_basis)?;
                },
                27 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_typ)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.npa_typ)?;
                },
                29 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.bmid)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compmis1 = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compmis3 = tmp;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.org_tenor = tmp;
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.resid_tenor = tmp;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.def_ftp_flag)?;
                },
                35 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.concat)?;
                },
                36 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cntr_party)?;
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.lcy_amount = tmp;
                },
                38 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.raw_benchmark)?;
                },
                39 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_interest_amount = tmp;
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_principal_amount = tmp;
                },
                41 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                42 => {
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
        if !self.cust_no.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cust_no);
        }
        if !self.reference.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.reference);
        }
        if !self.cust_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.cust_name);
        }
        if !self.branch_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.branch_cd);
        }
        if self.norm_int_rt != 0. {
            my_size += 9;
        }
        if !self.acurl_freq.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.acurl_freq);
        }
        if self.book_dt != 0 {
            my_size += ::protobuf::rt::value_size(7, self.book_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.val_dt != 0 {
            my_size += ::protobuf::rt::value_size(8, self.val_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mat_dt != 0 {
            my_size += ::protobuf::rt::value_size(9, self.mat_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.user_def_stats.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.user_def_stats);
        }
        if !self.prod_cd.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.prod_cd);
        }
        if !self.gl.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.gl);
        }
        if !self.curr.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.curr);
        }
        if self.prin_ost_bal != 0. {
            my_size += 9;
        }
        if self.spread != 0. {
            my_size += 9;
        }
        if self.compmis2 != 0 {
            my_size += ::protobuf::rt::value_size(16, self.compmis2, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.rt_flag_new.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.rt_flag_new);
        }
        if !self.rt_cd_new.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.rt_cd_new);
        }
        if !self.division.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.division);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.alm_line);
        }
        if !self.ia_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.ia_llg);
        }
        if !self.balm_llg.is_empty() {
            my_size += ::protobuf::rt::string_size(22, &self.balm_llg);
        }
        if !self.repricing_freq.is_empty() {
            my_size += ::protobuf::rt::string_size(23, &self.repricing_freq);
        }
        if self.lst_repricing_dt != 0 {
            my_size += ::protobuf::rt::value_size(24, self.lst_repricing_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nxt_repricing_dt != 0 {
            my_size += ::protobuf::rt::value_size(25, self.nxt_repricing_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.int_basis.is_empty() {
            my_size += ::protobuf::rt::string_size(26, &self.int_basis);
        }
        if !self.cust_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(27, &self.cust_typ);
        }
        if !self.npa_typ.is_empty() {
            my_size += ::protobuf::rt::string_size(28, &self.npa_typ);
        }
        if !self.bmid.is_empty() {
            my_size += ::protobuf::rt::string_size(29, &self.bmid);
        }
        if self.compmis1 != 0 {
            my_size += ::protobuf::rt::value_size(30, self.compmis1, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.compmis3 != 0 {
            my_size += ::protobuf::rt::value_size(31, self.compmis3, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.org_tenor != 0 {
            my_size += ::protobuf::rt::value_size(32, self.org_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resid_tenor != 0 {
            my_size += ::protobuf::rt::value_size(33, self.resid_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.def_ftp_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.def_ftp_flag);
        }
        if !self.concat.is_empty() {
            my_size += ::protobuf::rt::string_size(35, &self.concat);
        }
        if !self.cntr_party.is_empty() {
            my_size += ::protobuf::rt::string_size(36, &self.cntr_party);
        }
        if self.lcy_amount != 0. {
            my_size += 10;
        }
        if !self.raw_benchmark.is_empty() {
            my_size += ::protobuf::rt::string_size(38, &self.raw_benchmark);
        }
        if self.total_interest_amount != 0. {
            my_size += 10;
        }
        if self.total_principal_amount != 0. {
            my_size += 10;
        }
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(41, &self.sma_flag);
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
        if !self.cust_no.is_empty() {
            os.write_string(1, &self.cust_no)?;
        }
        if !self.reference.is_empty() {
            os.write_string(2, &self.reference)?;
        }
        if !self.cust_name.is_empty() {
            os.write_string(3, &self.cust_name)?;
        }
        if !self.branch_cd.is_empty() {
            os.write_string(4, &self.branch_cd)?;
        }
        if self.norm_int_rt != 0. {
            os.write_double(5, self.norm_int_rt)?;
        }
        if !self.acurl_freq.is_empty() {
            os.write_string(6, &self.acurl_freq)?;
        }
        if self.book_dt != 0 {
            os.write_int64(7, self.book_dt)?;
        }
        if self.val_dt != 0 {
            os.write_int64(8, self.val_dt)?;
        }
        if self.mat_dt != 0 {
            os.write_int64(9, self.mat_dt)?;
        }
        if !self.user_def_stats.is_empty() {
            os.write_string(10, &self.user_def_stats)?;
        }
        if !self.prod_cd.is_empty() {
            os.write_string(11, &self.prod_cd)?;
        }
        if !self.gl.is_empty() {
            os.write_string(12, &self.gl)?;
        }
        if !self.curr.is_empty() {
            os.write_string(13, &self.curr)?;
        }
        if self.prin_ost_bal != 0. {
            os.write_double(14, self.prin_ost_bal)?;
        }
        if self.spread != 0. {
            os.write_double(15, self.spread)?;
        }
        if self.compmis2 != 0 {
            os.write_int64(16, self.compmis2)?;
        }
        if !self.rt_flag_new.is_empty() {
            os.write_string(17, &self.rt_flag_new)?;
        }
        if !self.rt_cd_new.is_empty() {
            os.write_string(18, &self.rt_cd_new)?;
        }
        if !self.division.is_empty() {
            os.write_string(19, &self.division)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(20, &self.alm_line)?;
        }
        if !self.ia_llg.is_empty() {
            os.write_string(21, &self.ia_llg)?;
        }
        if !self.balm_llg.is_empty() {
            os.write_string(22, &self.balm_llg)?;
        }
        if !self.repricing_freq.is_empty() {
            os.write_string(23, &self.repricing_freq)?;
        }
        if self.lst_repricing_dt != 0 {
            os.write_int64(24, self.lst_repricing_dt)?;
        }
        if self.nxt_repricing_dt != 0 {
            os.write_int64(25, self.nxt_repricing_dt)?;
        }
        if !self.int_basis.is_empty() {
            os.write_string(26, &self.int_basis)?;
        }
        if !self.cust_typ.is_empty() {
            os.write_string(27, &self.cust_typ)?;
        }
        if !self.npa_typ.is_empty() {
            os.write_string(28, &self.npa_typ)?;
        }
        if !self.bmid.is_empty() {
            os.write_string(29, &self.bmid)?;
        }
        if self.compmis1 != 0 {
            os.write_int64(30, self.compmis1)?;
        }
        if self.compmis3 != 0 {
            os.write_int64(31, self.compmis3)?;
        }
        if self.org_tenor != 0 {
            os.write_int64(32, self.org_tenor)?;
        }
        if self.resid_tenor != 0 {
            os.write_int64(33, self.resid_tenor)?;
        }
        if !self.def_ftp_flag.is_empty() {
            os.write_string(34, &self.def_ftp_flag)?;
        }
        if !self.concat.is_empty() {
            os.write_string(35, &self.concat)?;
        }
        if !self.cntr_party.is_empty() {
            os.write_string(36, &self.cntr_party)?;
        }
        if self.lcy_amount != 0. {
            os.write_double(37, self.lcy_amount)?;
        }
        if !self.raw_benchmark.is_empty() {
            os.write_string(38, &self.raw_benchmark)?;
        }
        if self.total_interest_amount != 0. {
            os.write_double(39, self.total_interest_amount)?;
        }
        if self.total_principal_amount != 0. {
            os.write_double(40, self.total_principal_amount)?;
        }
        if !self.sma_flag.is_empty() {
            os.write_string(41, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(42, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                "cust_no",
                |m: &AccountWithCashflows| { &m.cust_no },
                |m: &mut AccountWithCashflows| { &mut m.cust_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "reference",
                |m: &AccountWithCashflows| { &m.reference },
                |m: &mut AccountWithCashflows| { &mut m.reference },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_name",
                |m: &AccountWithCashflows| { &m.cust_name },
                |m: &mut AccountWithCashflows| { &mut m.cust_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "branch_cd",
                |m: &AccountWithCashflows| { &m.branch_cd },
                |m: &mut AccountWithCashflows| { &mut m.branch_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "norm_int_rt",
                |m: &AccountWithCashflows| { &m.norm_int_rt },
                |m: &mut AccountWithCashflows| { &mut m.norm_int_rt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "acurl_freq",
                |m: &AccountWithCashflows| { &m.acurl_freq },
                |m: &mut AccountWithCashflows| { &mut m.acurl_freq },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "book_dt",
                |m: &AccountWithCashflows| { &m.book_dt },
                |m: &mut AccountWithCashflows| { &mut m.book_dt },
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
                "user_def_stats",
                |m: &AccountWithCashflows| { &m.user_def_stats },
                |m: &mut AccountWithCashflows| { &mut m.user_def_stats },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "prod_cd",
                |m: &AccountWithCashflows| { &m.prod_cd },
                |m: &mut AccountWithCashflows| { &mut m.prod_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "gl",
                |m: &AccountWithCashflows| { &m.gl },
                |m: &mut AccountWithCashflows| { &mut m.gl },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "curr",
                |m: &AccountWithCashflows| { &m.curr },
                |m: &mut AccountWithCashflows| { &mut m.curr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "prin_ost_bal",
                |m: &AccountWithCashflows| { &m.prin_ost_bal },
                |m: &mut AccountWithCashflows| { &mut m.prin_ost_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "spread",
                |m: &AccountWithCashflows| { &m.spread },
                |m: &mut AccountWithCashflows| { &mut m.spread },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "compmis2",
                |m: &AccountWithCashflows| { &m.compmis2 },
                |m: &mut AccountWithCashflows| { &mut m.compmis2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rt_flag_new",
                |m: &AccountWithCashflows| { &m.rt_flag_new },
                |m: &mut AccountWithCashflows| { &mut m.rt_flag_new },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rt_cd_new",
                |m: &AccountWithCashflows| { &m.rt_cd_new },
                |m: &mut AccountWithCashflows| { &mut m.rt_cd_new },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "division",
                |m: &AccountWithCashflows| { &m.division },
                |m: &mut AccountWithCashflows| { &mut m.division },
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "repricing_freq",
                |m: &AccountWithCashflows| { &m.repricing_freq },
                |m: &mut AccountWithCashflows| { &mut m.repricing_freq },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "lst_repricing_dt",
                |m: &AccountWithCashflows| { &m.lst_repricing_dt },
                |m: &mut AccountWithCashflows| { &mut m.lst_repricing_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "nxt_repricing_dt",
                |m: &AccountWithCashflows| { &m.nxt_repricing_dt },
                |m: &mut AccountWithCashflows| { &mut m.nxt_repricing_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int_basis",
                |m: &AccountWithCashflows| { &m.int_basis },
                |m: &mut AccountWithCashflows| { &mut m.int_basis },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_typ",
                |m: &AccountWithCashflows| { &m.cust_typ },
                |m: &mut AccountWithCashflows| { &mut m.cust_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "npa_typ",
                |m: &AccountWithCashflows| { &m.npa_typ },
                |m: &mut AccountWithCashflows| { &mut m.npa_typ },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "bmid",
                |m: &AccountWithCashflows| { &m.bmid },
                |m: &mut AccountWithCashflows| { &mut m.bmid },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "compmis1",
                |m: &AccountWithCashflows| { &m.compmis1 },
                |m: &mut AccountWithCashflows| { &mut m.compmis1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "compmis3",
                |m: &AccountWithCashflows| { &m.compmis3 },
                |m: &mut AccountWithCashflows| { &mut m.compmis3 },
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "def_ftp_flag",
                |m: &AccountWithCashflows| { &m.def_ftp_flag },
                |m: &mut AccountWithCashflows| { &mut m.def_ftp_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "concat",
                |m: &AccountWithCashflows| { &m.concat },
                |m: &mut AccountWithCashflows| { &mut m.concat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cntr_party",
                |m: &AccountWithCashflows| { &m.cntr_party },
                |m: &mut AccountWithCashflows| { &mut m.cntr_party },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "lcy_amount",
                |m: &AccountWithCashflows| { &m.lcy_amount },
                |m: &mut AccountWithCashflows| { &mut m.lcy_amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "raw_benchmark",
                |m: &AccountWithCashflows| { &m.raw_benchmark },
                |m: &mut AccountWithCashflows| { &mut m.raw_benchmark },
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
        self.cust_no.clear();
        self.reference.clear();
        self.cust_name.clear();
        self.branch_cd.clear();
        self.norm_int_rt = 0.;
        self.acurl_freq.clear();
        self.book_dt = 0;
        self.val_dt = 0;
        self.mat_dt = 0;
        self.user_def_stats.clear();
        self.prod_cd.clear();
        self.gl.clear();
        self.curr.clear();
        self.prin_ost_bal = 0.;
        self.spread = 0.;
        self.compmis2 = 0;
        self.rt_flag_new.clear();
        self.rt_cd_new.clear();
        self.division.clear();
        self.alm_line.clear();
        self.ia_llg.clear();
        self.balm_llg.clear();
        self.repricing_freq.clear();
        self.lst_repricing_dt = 0;
        self.nxt_repricing_dt = 0;
        self.int_basis.clear();
        self.cust_typ.clear();
        self.npa_typ.clear();
        self.bmid.clear();
        self.compmis1 = 0;
        self.compmis3 = 0;
        self.org_tenor = 0;
        self.resid_tenor = 0;
        self.def_ftp_flag.clear();
        self.concat.clear();
        self.cntr_party.clear();
        self.lcy_amount = 0.;
        self.raw_benchmark.clear();
        self.total_interest_amount = 0.;
        self.total_principal_amount = 0.;
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
    \n\x14loans_ovs_hdfc.proto\"R\n\x08Cashflow\x12\x17\n\x07int_amt\x18\x01\
    \x20\x01(\x01R\x06intAmt\x12\x19\n\x08prin_amt\x18\x02\x20\x01(\x01R\x07\
    prinAmt\x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\"\x8c\n\n\x14Acc\
    ountWithCashflows\x12\x17\n\x07cust_no\x18\x01\x20\x01(\tR\x06custNo\x12\
    \x1c\n\treference\x18\x02\x20\x01(\tR\treference\x12\x1b\n\tcust_name\
    \x18\x03\x20\x01(\tR\x08custName\x12\x1b\n\tbranch_cd\x18\x04\x20\x01(\t\
    R\x08branchCd\x12\x1e\n\x0bnorm_int_rt\x18\x05\x20\x01(\x01R\tnormIntRt\
    \x12\x1d\n\nacurl_freq\x18\x06\x20\x01(\tR\tacurlFreq\x12\x17\n\x07book_\
    dt\x18\x07\x20\x01(\x03R\x06bookDt\x12\x15\n\x06val_dt\x18\x08\x20\x01(\
    \x03R\x05valDt\x12\x15\n\x06mat_dt\x18\t\x20\x01(\x03R\x05matDt\x12$\n\
    \x0euser_def_stats\x18\n\x20\x01(\tR\x0cuserDefStats\x12\x17\n\x07prod_c\
    d\x18\x0b\x20\x01(\tR\x06prodCd\x12\x0e\n\x02gl\x18\x0c\x20\x01(\tR\x02g\
    l\x12\x12\n\x04curr\x18\r\x20\x01(\tR\x04curr\x12\x20\n\x0cprin_ost_bal\
    \x18\x0e\x20\x01(\x01R\nprinOstBal\x12\x16\n\x06spread\x18\x0f\x20\x01(\
    \x01R\x06spread\x12\x1a\n\x08compmis2\x18\x10\x20\x01(\x03R\x08compmis2\
    \x12\x1e\n\x0brt_flag_new\x18\x11\x20\x01(\tR\trtFlagNew\x12\x1a\n\trt_c\
    d_new\x18\x12\x20\x01(\tR\x07rtCdNew\x12\x1a\n\x08division\x18\x13\x20\
    \x01(\tR\x08division\x12\x19\n\x08alm_line\x18\x14\x20\x01(\tR\x07almLin\
    e\x12\x15\n\x06ia_llg\x18\x15\x20\x01(\tR\x05iaLlg\x12\x19\n\x08balm_llg\
    \x18\x16\x20\x01(\tR\x07balmLlg\x12%\n\x0erepricing_freq\x18\x17\x20\x01\
    (\tR\rrepricingFreq\x12(\n\x10lst_repricing_dt\x18\x18\x20\x01(\x03R\x0e\
    lstRepricingDt\x12(\n\x10nxt_repricing_dt\x18\x19\x20\x01(\x03R\x0enxtRe\
    pricingDt\x12\x1b\n\tint_basis\x18\x1a\x20\x01(\tR\x08intBasis\x12\x19\n\
    \x08cust_typ\x18\x1b\x20\x01(\tR\x07custTyp\x12\x17\n\x07npa_typ\x18\x1c\
    \x20\x01(\tR\x06npaTyp\x12\x12\n\x04bmid\x18\x1d\x20\x01(\tR\x04bmid\x12\
    \x1a\n\x08compmis1\x18\x1e\x20\x01(\x03R\x08compmis1\x12\x1a\n\x08compmi\
    s3\x18\x1f\x20\x01(\x03R\x08compmis3\x12\x1b\n\torg_tenor\x18\x20\x20\
    \x01(\x03R\x08orgTenor\x12\x1f\n\x0bresid_tenor\x18!\x20\x01(\x03R\nresi\
    dTenor\x12\x20\n\x0cdef_ftp_flag\x18\"\x20\x01(\tR\ndefFtpFlag\x12\x16\n\
    \x06concat\x18#\x20\x01(\tR\x06concat\x12\x1d\n\ncntr_party\x18$\x20\x01\
    (\tR\tcntrParty\x12\x1d\n\nlcy_amount\x18%\x20\x01(\x01R\tlcyAmount\x12#\
    \n\rraw_benchmark\x18&\x20\x01(\tR\x0crawBenchmark\x122\n\x15total_inter\
    est_amount\x18'\x20\x01(\x01R\x13totalInterestAmount\x124\n\x16total_pri\
    ncipal_amount\x18(\x20\x01(\x01R\x14totalPrincipalAmount\x12\x19\n\x08sm\
    a_flag\x18)\x20\x01(\tR\x07smaFlag\x12'\n\tcashflows\x18*\x20\x03(\x0b2\
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
