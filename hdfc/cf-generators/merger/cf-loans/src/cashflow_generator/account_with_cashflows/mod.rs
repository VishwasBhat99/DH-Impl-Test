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
    pub acc_no: ::std::string::String,
    pub disbursed_amt: f64,
    pub os_loan_bal_lcy: f64,
    pub int_rate: f64,
    pub ei_amt_crnt: f64,
    pub int_type: ::std::string::String,
    pub os_p_bal_due_local_ccy: f64,
    pub os_i_bal_due_local_ccy: f64,
    pub ei_amt_paid_adv_lcy: f64,
    pub pre_ei_bal_lcy: f64,
    pub acc_open_value_date: i64,
    pub maturity_date: i64,
    pub ei_start_date_crnt: i64,
    pub ei_end_date_crnt: i64,
    pub ei_pay_freq_crnt: ::std::string::String,
    pub emi_last_paid_date_crnt: i64,
    pub ei_pay_day: i64,
    pub ei_orginal_term: i64,
    pub ei_bal_term: i64,
    pub rep_bm: ::std::string::String,
    pub spread: ::std::string::String,
    pub last_rep_date: i64,
    pub next_rep_date: i64,
    pub rep_freq: i64,
    pub no_ei_structures: i64,
    pub npa_class: ::std::string::String,
    pub remark: ::std::string::String,
    pub months_os_comb: ::std::string::String,
    pub mor_type: ::std::string::String,
    pub from_mor_date: i64,
    pub to_mor_date: i64,
    pub recalc_ei_amt_flag: ::std::string::String,
    pub mor_int_calc: ::std::string::String,
    pub bullet_pay_flag: ::std::string::String,
    pub restrct_flag: ::std::string::String,
    pub residential_mortgage: ::std::string::String,
    pub risk_weight: ::std::string::String,
    pub internal_rating: ::std::string::String,
    pub external_rating: ::std::string::String,
    pub contractual_tenor: i64,
    pub residual_tenor: i64,
    pub cust_constitution_code: ::std::string::String,
    pub prod_code: ::std::string::String,
    pub p_gl_code: ::std::string::String,
    pub m_npaclass: ::std::string::String,
    pub acrd_int: f64,
    pub cust_id: ::std::string::String,
    pub cust_name: ::std::string::String,
    pub group_id: ::std::string::String,
    pub group_name: ::std::string::String,
    pub branch_code: ::std::string::String,
    pub sector: ::std::string::String,
    pub industry: ::std::string::String,
    pub ltv: ::std::string::String,
    pub overdue_acc: ::std::string::String,
    pub excess_acc: ::std::string::String,
    pub loan_type: ::std::string::String,
    pub resid_int: f64,
    pub ccy: ::std::string::String,
    pub hdfc_ltd_percent: f64,
    pub sec_percent: f64,
    pub overdue_type: ::std::string::String,
    pub alm_line: ::std::string::String,
    pub structure_number: ::std::string::String,
    pub memi: f64,
    pub ost_bal: f64,
    pub roi: f64,
    pub asondate: i64,
    pub emi_overdue_gl_cd: i64,
    pub pre_emi_overdue_gl_cd: i64,
    pub excess_emi_gl_cd: i64,
    pub excess_pre_emi_gl_cd: i64,
    pub tot_prin_amt: f64,
    pub tot_int_amt: f64,
    pub balm_l2: ::std::string::String,
    pub derived_npa_class: ::std::string::String,
    pub common_cust_id: ::std::string::String,
    pub derived_next_reprice_date: i64,
    pub derived_risk_weight: ::std::string::String,
    pub restructure_flag: ::std::string::String,
    pub resid: ::std::string::String,
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

    // double disbursed_amt = 2;


    pub fn get_disbursed_amt(&self) -> f64 {
        self.disbursed_amt
    }
    pub fn clear_disbursed_amt(&mut self) {
        self.disbursed_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_disbursed_amt(&mut self, v: f64) {
        self.disbursed_amt = v;
    }

    // double os_loan_bal_lcy = 3;


    pub fn get_os_loan_bal_lcy(&self) -> f64 {
        self.os_loan_bal_lcy
    }
    pub fn clear_os_loan_bal_lcy(&mut self) {
        self.os_loan_bal_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_loan_bal_lcy(&mut self, v: f64) {
        self.os_loan_bal_lcy = v;
    }

    // double int_rate = 4;


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

    // double ei_amt_crnt = 5;


    pub fn get_ei_amt_crnt(&self) -> f64 {
        self.ei_amt_crnt
    }
    pub fn clear_ei_amt_crnt(&mut self) {
        self.ei_amt_crnt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ei_amt_crnt(&mut self, v: f64) {
        self.ei_amt_crnt = v;
    }

    // string int_type = 6;


    pub fn get_int_type(&self) -> &str {
        &self.int_type
    }
    pub fn clear_int_type(&mut self) {
        self.int_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_type(&mut self, v: ::std::string::String) {
        self.int_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_int_type(&mut self) -> &mut ::std::string::String {
        &mut self.int_type
    }

    // Take field
    pub fn take_int_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.int_type, ::std::string::String::new())
    }

    // double os_p_bal_due_local_ccy = 7;


    pub fn get_os_p_bal_due_local_ccy(&self) -> f64 {
        self.os_p_bal_due_local_ccy
    }
    pub fn clear_os_p_bal_due_local_ccy(&mut self) {
        self.os_p_bal_due_local_ccy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_p_bal_due_local_ccy(&mut self, v: f64) {
        self.os_p_bal_due_local_ccy = v;
    }

    // double os_i_bal_due_local_ccy = 8;


    pub fn get_os_i_bal_due_local_ccy(&self) -> f64 {
        self.os_i_bal_due_local_ccy
    }
    pub fn clear_os_i_bal_due_local_ccy(&mut self) {
        self.os_i_bal_due_local_ccy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_i_bal_due_local_ccy(&mut self, v: f64) {
        self.os_i_bal_due_local_ccy = v;
    }

    // double ei_amt_paid_adv_lcy = 9;


    pub fn get_ei_amt_paid_adv_lcy(&self) -> f64 {
        self.ei_amt_paid_adv_lcy
    }
    pub fn clear_ei_amt_paid_adv_lcy(&mut self) {
        self.ei_amt_paid_adv_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ei_amt_paid_adv_lcy(&mut self, v: f64) {
        self.ei_amt_paid_adv_lcy = v;
    }

    // double pre_ei_bal_lcy = 10;


    pub fn get_pre_ei_bal_lcy(&self) -> f64 {
        self.pre_ei_bal_lcy
    }
    pub fn clear_pre_ei_bal_lcy(&mut self) {
        self.pre_ei_bal_lcy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_pre_ei_bal_lcy(&mut self, v: f64) {
        self.pre_ei_bal_lcy = v;
    }

    // int64 acc_open_value_date = 11;


    pub fn get_acc_open_value_date(&self) -> i64 {
        self.acc_open_value_date
    }
    pub fn clear_acc_open_value_date(&mut self) {
        self.acc_open_value_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_acc_open_value_date(&mut self, v: i64) {
        self.acc_open_value_date = v;
    }

    // int64 maturity_date = 12;


    pub fn get_maturity_date(&self) -> i64 {
        self.maturity_date
    }
    pub fn clear_maturity_date(&mut self) {
        self.maturity_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_maturity_date(&mut self, v: i64) {
        self.maturity_date = v;
    }

    // int64 ei_start_date_crnt = 13;


    pub fn get_ei_start_date_crnt(&self) -> i64 {
        self.ei_start_date_crnt
    }
    pub fn clear_ei_start_date_crnt(&mut self) {
        self.ei_start_date_crnt = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_start_date_crnt(&mut self, v: i64) {
        self.ei_start_date_crnt = v;
    }

    // int64 ei_end_date_crnt = 14;


    pub fn get_ei_end_date_crnt(&self) -> i64 {
        self.ei_end_date_crnt
    }
    pub fn clear_ei_end_date_crnt(&mut self) {
        self.ei_end_date_crnt = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_end_date_crnt(&mut self, v: i64) {
        self.ei_end_date_crnt = v;
    }

    // string ei_pay_freq_crnt = 15;


    pub fn get_ei_pay_freq_crnt(&self) -> &str {
        &self.ei_pay_freq_crnt
    }
    pub fn clear_ei_pay_freq_crnt(&mut self) {
        self.ei_pay_freq_crnt.clear();
    }

    // Param is passed by value, moved
    pub fn set_ei_pay_freq_crnt(&mut self, v: ::std::string::String) {
        self.ei_pay_freq_crnt = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ei_pay_freq_crnt(&mut self) -> &mut ::std::string::String {
        &mut self.ei_pay_freq_crnt
    }

    // Take field
    pub fn take_ei_pay_freq_crnt(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ei_pay_freq_crnt, ::std::string::String::new())
    }

    // int64 emi_last_paid_date_crnt = 16;


    pub fn get_emi_last_paid_date_crnt(&self) -> i64 {
        self.emi_last_paid_date_crnt
    }
    pub fn clear_emi_last_paid_date_crnt(&mut self) {
        self.emi_last_paid_date_crnt = 0;
    }

    // Param is passed by value, moved
    pub fn set_emi_last_paid_date_crnt(&mut self, v: i64) {
        self.emi_last_paid_date_crnt = v;
    }

    // int64 ei_pay_day = 17;


    pub fn get_ei_pay_day(&self) -> i64 {
        self.ei_pay_day
    }
    pub fn clear_ei_pay_day(&mut self) {
        self.ei_pay_day = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_pay_day(&mut self, v: i64) {
        self.ei_pay_day = v;
    }

    // int64 ei_orginal_term = 18;


    pub fn get_ei_orginal_term(&self) -> i64 {
        self.ei_orginal_term
    }
    pub fn clear_ei_orginal_term(&mut self) {
        self.ei_orginal_term = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_orginal_term(&mut self, v: i64) {
        self.ei_orginal_term = v;
    }

    // int64 ei_bal_term = 19;


    pub fn get_ei_bal_term(&self) -> i64 {
        self.ei_bal_term
    }
    pub fn clear_ei_bal_term(&mut self) {
        self.ei_bal_term = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_bal_term(&mut self, v: i64) {
        self.ei_bal_term = v;
    }

    // string rep_bm = 20;


    pub fn get_rep_bm(&self) -> &str {
        &self.rep_bm
    }
    pub fn clear_rep_bm(&mut self) {
        self.rep_bm.clear();
    }

    // Param is passed by value, moved
    pub fn set_rep_bm(&mut self, v: ::std::string::String) {
        self.rep_bm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rep_bm(&mut self) -> &mut ::std::string::String {
        &mut self.rep_bm
    }

    // Take field
    pub fn take_rep_bm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rep_bm, ::std::string::String::new())
    }

    // string spread = 21;


    pub fn get_spread(&self) -> &str {
        &self.spread
    }
    pub fn clear_spread(&mut self) {
        self.spread.clear();
    }

    // Param is passed by value, moved
    pub fn set_spread(&mut self, v: ::std::string::String) {
        self.spread = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spread(&mut self) -> &mut ::std::string::String {
        &mut self.spread
    }

    // Take field
    pub fn take_spread(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.spread, ::std::string::String::new())
    }

    // int64 last_rep_date = 22;


    pub fn get_last_rep_date(&self) -> i64 {
        self.last_rep_date
    }
    pub fn clear_last_rep_date(&mut self) {
        self.last_rep_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_rep_date(&mut self, v: i64) {
        self.last_rep_date = v;
    }

    // int64 next_rep_date = 23;


    pub fn get_next_rep_date(&self) -> i64 {
        self.next_rep_date
    }
    pub fn clear_next_rep_date(&mut self) {
        self.next_rep_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_next_rep_date(&mut self, v: i64) {
        self.next_rep_date = v;
    }

    // int64 rep_freq = 24;


    pub fn get_rep_freq(&self) -> i64 {
        self.rep_freq
    }
    pub fn clear_rep_freq(&mut self) {
        self.rep_freq = 0;
    }

    // Param is passed by value, moved
    pub fn set_rep_freq(&mut self, v: i64) {
        self.rep_freq = v;
    }

    // int64 no_ei_structures = 25;


    pub fn get_no_ei_structures(&self) -> i64 {
        self.no_ei_structures
    }
    pub fn clear_no_ei_structures(&mut self) {
        self.no_ei_structures = 0;
    }

    // Param is passed by value, moved
    pub fn set_no_ei_structures(&mut self, v: i64) {
        self.no_ei_structures = v;
    }

    // string npa_class = 26;


    pub fn get_npa_class(&self) -> &str {
        &self.npa_class
    }
    pub fn clear_npa_class(&mut self) {
        self.npa_class.clear();
    }

    // Param is passed by value, moved
    pub fn set_npa_class(&mut self, v: ::std::string::String) {
        self.npa_class = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_npa_class(&mut self) -> &mut ::std::string::String {
        &mut self.npa_class
    }

    // Take field
    pub fn take_npa_class(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.npa_class, ::std::string::String::new())
    }

    // string remark = 27;


    pub fn get_remark(&self) -> &str {
        &self.remark
    }
    pub fn clear_remark(&mut self) {
        self.remark.clear();
    }

    // Param is passed by value, moved
    pub fn set_remark(&mut self, v: ::std::string::String) {
        self.remark = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remark(&mut self) -> &mut ::std::string::String {
        &mut self.remark
    }

    // Take field
    pub fn take_remark(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remark, ::std::string::String::new())
    }

    // string months_os_comb = 28;


    pub fn get_months_os_comb(&self) -> &str {
        &self.months_os_comb
    }
    pub fn clear_months_os_comb(&mut self) {
        self.months_os_comb.clear();
    }

    // Param is passed by value, moved
    pub fn set_months_os_comb(&mut self, v: ::std::string::String) {
        self.months_os_comb = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_months_os_comb(&mut self) -> &mut ::std::string::String {
        &mut self.months_os_comb
    }

    // Take field
    pub fn take_months_os_comb(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.months_os_comb, ::std::string::String::new())
    }

    // string mor_type = 29;


    pub fn get_mor_type(&self) -> &str {
        &self.mor_type
    }
    pub fn clear_mor_type(&mut self) {
        self.mor_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_mor_type(&mut self, v: ::std::string::String) {
        self.mor_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mor_type(&mut self) -> &mut ::std::string::String {
        &mut self.mor_type
    }

    // Take field
    pub fn take_mor_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mor_type, ::std::string::String::new())
    }

    // int64 from_mor_date = 30;


    pub fn get_from_mor_date(&self) -> i64 {
        self.from_mor_date
    }
    pub fn clear_from_mor_date(&mut self) {
        self.from_mor_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_from_mor_date(&mut self, v: i64) {
        self.from_mor_date = v;
    }

    // int64 to_mor_date = 31;


    pub fn get_to_mor_date(&self) -> i64 {
        self.to_mor_date
    }
    pub fn clear_to_mor_date(&mut self) {
        self.to_mor_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_to_mor_date(&mut self, v: i64) {
        self.to_mor_date = v;
    }

    // string recalc_ei_amt_flag = 32;


    pub fn get_recalc_ei_amt_flag(&self) -> &str {
        &self.recalc_ei_amt_flag
    }
    pub fn clear_recalc_ei_amt_flag(&mut self) {
        self.recalc_ei_amt_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_recalc_ei_amt_flag(&mut self, v: ::std::string::String) {
        self.recalc_ei_amt_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_recalc_ei_amt_flag(&mut self) -> &mut ::std::string::String {
        &mut self.recalc_ei_amt_flag
    }

    // Take field
    pub fn take_recalc_ei_amt_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.recalc_ei_amt_flag, ::std::string::String::new())
    }

    // string mor_int_calc = 33;


    pub fn get_mor_int_calc(&self) -> &str {
        &self.mor_int_calc
    }
    pub fn clear_mor_int_calc(&mut self) {
        self.mor_int_calc.clear();
    }

    // Param is passed by value, moved
    pub fn set_mor_int_calc(&mut self, v: ::std::string::String) {
        self.mor_int_calc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mor_int_calc(&mut self) -> &mut ::std::string::String {
        &mut self.mor_int_calc
    }

    // Take field
    pub fn take_mor_int_calc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mor_int_calc, ::std::string::String::new())
    }

    // string bullet_pay_flag = 34;


    pub fn get_bullet_pay_flag(&self) -> &str {
        &self.bullet_pay_flag
    }
    pub fn clear_bullet_pay_flag(&mut self) {
        self.bullet_pay_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_bullet_pay_flag(&mut self, v: ::std::string::String) {
        self.bullet_pay_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bullet_pay_flag(&mut self) -> &mut ::std::string::String {
        &mut self.bullet_pay_flag
    }

    // Take field
    pub fn take_bullet_pay_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.bullet_pay_flag, ::std::string::String::new())
    }

    // string restrct_flag = 35;


    pub fn get_restrct_flag(&self) -> &str {
        &self.restrct_flag
    }
    pub fn clear_restrct_flag(&mut self) {
        self.restrct_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_restrct_flag(&mut self, v: ::std::string::String) {
        self.restrct_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_restrct_flag(&mut self) -> &mut ::std::string::String {
        &mut self.restrct_flag
    }

    // Take field
    pub fn take_restrct_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.restrct_flag, ::std::string::String::new())
    }

    // string residential_mortgage = 36;


    pub fn get_residential_mortgage(&self) -> &str {
        &self.residential_mortgage
    }
    pub fn clear_residential_mortgage(&mut self) {
        self.residential_mortgage.clear();
    }

    // Param is passed by value, moved
    pub fn set_residential_mortgage(&mut self, v: ::std::string::String) {
        self.residential_mortgage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_residential_mortgage(&mut self) -> &mut ::std::string::String {
        &mut self.residential_mortgage
    }

    // Take field
    pub fn take_residential_mortgage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.residential_mortgage, ::std::string::String::new())
    }

    // string risk_weight = 37;


    pub fn get_risk_weight(&self) -> &str {
        &self.risk_weight
    }
    pub fn clear_risk_weight(&mut self) {
        self.risk_weight.clear();
    }

    // Param is passed by value, moved
    pub fn set_risk_weight(&mut self, v: ::std::string::String) {
        self.risk_weight = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_risk_weight(&mut self) -> &mut ::std::string::String {
        &mut self.risk_weight
    }

    // Take field
    pub fn take_risk_weight(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.risk_weight, ::std::string::String::new())
    }

    // string internal_rating = 38;


    pub fn get_internal_rating(&self) -> &str {
        &self.internal_rating
    }
    pub fn clear_internal_rating(&mut self) {
        self.internal_rating.clear();
    }

    // Param is passed by value, moved
    pub fn set_internal_rating(&mut self, v: ::std::string::String) {
        self.internal_rating = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_rating(&mut self) -> &mut ::std::string::String {
        &mut self.internal_rating
    }

    // Take field
    pub fn take_internal_rating(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.internal_rating, ::std::string::String::new())
    }

    // string external_rating = 39;


    pub fn get_external_rating(&self) -> &str {
        &self.external_rating
    }
    pub fn clear_external_rating(&mut self) {
        self.external_rating.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_rating(&mut self, v: ::std::string::String) {
        self.external_rating = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_external_rating(&mut self) -> &mut ::std::string::String {
        &mut self.external_rating
    }

    // Take field
    pub fn take_external_rating(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.external_rating, ::std::string::String::new())
    }

    // int64 contractual_tenor = 40;


    pub fn get_contractual_tenor(&self) -> i64 {
        self.contractual_tenor
    }
    pub fn clear_contractual_tenor(&mut self) {
        self.contractual_tenor = 0;
    }

    // Param is passed by value, moved
    pub fn set_contractual_tenor(&mut self, v: i64) {
        self.contractual_tenor = v;
    }

    // int64 residual_tenor = 41;


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

    // string cust_constitution_code = 42;


    pub fn get_cust_constitution_code(&self) -> &str {
        &self.cust_constitution_code
    }
    pub fn clear_cust_constitution_code(&mut self) {
        self.cust_constitution_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_constitution_code(&mut self, v: ::std::string::String) {
        self.cust_constitution_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_constitution_code(&mut self) -> &mut ::std::string::String {
        &mut self.cust_constitution_code
    }

    // Take field
    pub fn take_cust_constitution_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_constitution_code, ::std::string::String::new())
    }

    // string prod_code = 43;


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

    // string p_gl_code = 44;


    pub fn get_p_gl_code(&self) -> &str {
        &self.p_gl_code
    }
    pub fn clear_p_gl_code(&mut self) {
        self.p_gl_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_p_gl_code(&mut self, v: ::std::string::String) {
        self.p_gl_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p_gl_code(&mut self) -> &mut ::std::string::String {
        &mut self.p_gl_code
    }

    // Take field
    pub fn take_p_gl_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.p_gl_code, ::std::string::String::new())
    }

    // string m_npaclass = 45;


    pub fn get_m_npaclass(&self) -> &str {
        &self.m_npaclass
    }
    pub fn clear_m_npaclass(&mut self) {
        self.m_npaclass.clear();
    }

    // Param is passed by value, moved
    pub fn set_m_npaclass(&mut self, v: ::std::string::String) {
        self.m_npaclass = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m_npaclass(&mut self) -> &mut ::std::string::String {
        &mut self.m_npaclass
    }

    // Take field
    pub fn take_m_npaclass(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.m_npaclass, ::std::string::String::new())
    }

    // double acrd_int = 46;


    pub fn get_acrd_int(&self) -> f64 {
        self.acrd_int
    }
    pub fn clear_acrd_int(&mut self) {
        self.acrd_int = 0.;
    }

    // Param is passed by value, moved
    pub fn set_acrd_int(&mut self, v: f64) {
        self.acrd_int = v;
    }

    // string cust_id = 47;


    pub fn get_cust_id(&self) -> &str {
        &self.cust_id
    }
    pub fn clear_cust_id(&mut self) {
        self.cust_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_cust_id(&mut self, v: ::std::string::String) {
        self.cust_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cust_id(&mut self) -> &mut ::std::string::String {
        &mut self.cust_id
    }

    // Take field
    pub fn take_cust_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cust_id, ::std::string::String::new())
    }

    // string cust_name = 48;


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

    // string group_id = 49;


    pub fn get_group_id(&self) -> &str {
        &self.group_id
    }
    pub fn clear_group_id(&mut self) {
        self.group_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: ::std::string::String) {
        self.group_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group_id(&mut self) -> &mut ::std::string::String {
        &mut self.group_id
    }

    // Take field
    pub fn take_group_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.group_id, ::std::string::String::new())
    }

    // string group_name = 50;


    pub fn get_group_name(&self) -> &str {
        &self.group_name
    }
    pub fn clear_group_name(&mut self) {
        self.group_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_name(&mut self, v: ::std::string::String) {
        self.group_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group_name(&mut self) -> &mut ::std::string::String {
        &mut self.group_name
    }

    // Take field
    pub fn take_group_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.group_name, ::std::string::String::new())
    }

    // string branch_code = 51;


    pub fn get_branch_code(&self) -> &str {
        &self.branch_code
    }
    pub fn clear_branch_code(&mut self) {
        self.branch_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_branch_code(&mut self, v: ::std::string::String) {
        self.branch_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_branch_code(&mut self) -> &mut ::std::string::String {
        &mut self.branch_code
    }

    // Take field
    pub fn take_branch_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.branch_code, ::std::string::String::new())
    }

    // string sector = 52;


    pub fn get_sector(&self) -> &str {
        &self.sector
    }
    pub fn clear_sector(&mut self) {
        self.sector.clear();
    }

    // Param is passed by value, moved
    pub fn set_sector(&mut self, v: ::std::string::String) {
        self.sector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sector(&mut self) -> &mut ::std::string::String {
        &mut self.sector
    }

    // Take field
    pub fn take_sector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sector, ::std::string::String::new())
    }

    // string industry = 53;


    pub fn get_industry(&self) -> &str {
        &self.industry
    }
    pub fn clear_industry(&mut self) {
        self.industry.clear();
    }

    // Param is passed by value, moved
    pub fn set_industry(&mut self, v: ::std::string::String) {
        self.industry = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_industry(&mut self) -> &mut ::std::string::String {
        &mut self.industry
    }

    // Take field
    pub fn take_industry(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.industry, ::std::string::String::new())
    }

    // string ltv = 54;


    pub fn get_ltv(&self) -> &str {
        &self.ltv
    }
    pub fn clear_ltv(&mut self) {
        self.ltv.clear();
    }

    // Param is passed by value, moved
    pub fn set_ltv(&mut self, v: ::std::string::String) {
        self.ltv = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ltv(&mut self) -> &mut ::std::string::String {
        &mut self.ltv
    }

    // Take field
    pub fn take_ltv(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ltv, ::std::string::String::new())
    }

    // string overdue_acc = 55;


    pub fn get_overdue_acc(&self) -> &str {
        &self.overdue_acc
    }
    pub fn clear_overdue_acc(&mut self) {
        self.overdue_acc.clear();
    }

    // Param is passed by value, moved
    pub fn set_overdue_acc(&mut self, v: ::std::string::String) {
        self.overdue_acc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_overdue_acc(&mut self) -> &mut ::std::string::String {
        &mut self.overdue_acc
    }

    // Take field
    pub fn take_overdue_acc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.overdue_acc, ::std::string::String::new())
    }

    // string excess_acc = 56;


    pub fn get_excess_acc(&self) -> &str {
        &self.excess_acc
    }
    pub fn clear_excess_acc(&mut self) {
        self.excess_acc.clear();
    }

    // Param is passed by value, moved
    pub fn set_excess_acc(&mut self, v: ::std::string::String) {
        self.excess_acc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_excess_acc(&mut self) -> &mut ::std::string::String {
        &mut self.excess_acc
    }

    // Take field
    pub fn take_excess_acc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.excess_acc, ::std::string::String::new())
    }

    // string loan_type = 57;


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

    // double resid_int = 58;


    pub fn get_resid_int(&self) -> f64 {
        self.resid_int
    }
    pub fn clear_resid_int(&mut self) {
        self.resid_int = 0.;
    }

    // Param is passed by value, moved
    pub fn set_resid_int(&mut self, v: f64) {
        self.resid_int = v;
    }

    // string ccy = 59;


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

    // double hdfc_ltd_percent = 60;


    pub fn get_hdfc_ltd_percent(&self) -> f64 {
        self.hdfc_ltd_percent
    }
    pub fn clear_hdfc_ltd_percent(&mut self) {
        self.hdfc_ltd_percent = 0.;
    }

    // Param is passed by value, moved
    pub fn set_hdfc_ltd_percent(&mut self, v: f64) {
        self.hdfc_ltd_percent = v;
    }

    // double sec_percent = 61;


    pub fn get_sec_percent(&self) -> f64 {
        self.sec_percent
    }
    pub fn clear_sec_percent(&mut self) {
        self.sec_percent = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sec_percent(&mut self, v: f64) {
        self.sec_percent = v;
    }

    // string overdue_type = 62;


    pub fn get_overdue_type(&self) -> &str {
        &self.overdue_type
    }
    pub fn clear_overdue_type(&mut self) {
        self.overdue_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_overdue_type(&mut self, v: ::std::string::String) {
        self.overdue_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_overdue_type(&mut self) -> &mut ::std::string::String {
        &mut self.overdue_type
    }

    // Take field
    pub fn take_overdue_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.overdue_type, ::std::string::String::new())
    }

    // string alm_line = 63;


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

    // string structure_number = 64;


    pub fn get_structure_number(&self) -> &str {
        &self.structure_number
    }
    pub fn clear_structure_number(&mut self) {
        self.structure_number.clear();
    }

    // Param is passed by value, moved
    pub fn set_structure_number(&mut self, v: ::std::string::String) {
        self.structure_number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_structure_number(&mut self) -> &mut ::std::string::String {
        &mut self.structure_number
    }

    // Take field
    pub fn take_structure_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.structure_number, ::std::string::String::new())
    }

    // double memi = 65;


    pub fn get_memi(&self) -> f64 {
        self.memi
    }
    pub fn clear_memi(&mut self) {
        self.memi = 0.;
    }

    // Param is passed by value, moved
    pub fn set_memi(&mut self, v: f64) {
        self.memi = v;
    }

    // double ost_bal = 66;


    pub fn get_ost_bal(&self) -> f64 {
        self.ost_bal
    }
    pub fn clear_ost_bal(&mut self) {
        self.ost_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ost_bal(&mut self, v: f64) {
        self.ost_bal = v;
    }

    // double roi = 67;


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

    // int64 asondate = 68;


    pub fn get_asondate(&self) -> i64 {
        self.asondate
    }
    pub fn clear_asondate(&mut self) {
        self.asondate = 0;
    }

    // Param is passed by value, moved
    pub fn set_asondate(&mut self, v: i64) {
        self.asondate = v;
    }

    // int64 emi_overdue_gl_cd = 69;


    pub fn get_emi_overdue_gl_cd(&self) -> i64 {
        self.emi_overdue_gl_cd
    }
    pub fn clear_emi_overdue_gl_cd(&mut self) {
        self.emi_overdue_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_emi_overdue_gl_cd(&mut self, v: i64) {
        self.emi_overdue_gl_cd = v;
    }

    // int64 pre_emi_overdue_gl_cd = 70;


    pub fn get_pre_emi_overdue_gl_cd(&self) -> i64 {
        self.pre_emi_overdue_gl_cd
    }
    pub fn clear_pre_emi_overdue_gl_cd(&mut self) {
        self.pre_emi_overdue_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_pre_emi_overdue_gl_cd(&mut self, v: i64) {
        self.pre_emi_overdue_gl_cd = v;
    }

    // int64 excess_emi_gl_cd = 71;


    pub fn get_excess_emi_gl_cd(&self) -> i64 {
        self.excess_emi_gl_cd
    }
    pub fn clear_excess_emi_gl_cd(&mut self) {
        self.excess_emi_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_excess_emi_gl_cd(&mut self, v: i64) {
        self.excess_emi_gl_cd = v;
    }

    // int64 excess_pre_emi_gl_cd = 72;


    pub fn get_excess_pre_emi_gl_cd(&self) -> i64 {
        self.excess_pre_emi_gl_cd
    }
    pub fn clear_excess_pre_emi_gl_cd(&mut self) {
        self.excess_pre_emi_gl_cd = 0;
    }

    // Param is passed by value, moved
    pub fn set_excess_pre_emi_gl_cd(&mut self, v: i64) {
        self.excess_pre_emi_gl_cd = v;
    }

    // double tot_prin_amt = 73;


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

    // double tot_int_amt = 74;


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

    // string balm_l2 = 75;


    pub fn get_balm_l2(&self) -> &str {
        &self.balm_l2
    }
    pub fn clear_balm_l2(&mut self) {
        self.balm_l2.clear();
    }

    // Param is passed by value, moved
    pub fn set_balm_l2(&mut self, v: ::std::string::String) {
        self.balm_l2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_balm_l2(&mut self) -> &mut ::std::string::String {
        &mut self.balm_l2
    }

    // Take field
    pub fn take_balm_l2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.balm_l2, ::std::string::String::new())
    }

    // string derived_npa_class = 76;


    pub fn get_derived_npa_class(&self) -> &str {
        &self.derived_npa_class
    }
    pub fn clear_derived_npa_class(&mut self) {
        self.derived_npa_class.clear();
    }

    // Param is passed by value, moved
    pub fn set_derived_npa_class(&mut self, v: ::std::string::String) {
        self.derived_npa_class = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_derived_npa_class(&mut self) -> &mut ::std::string::String {
        &mut self.derived_npa_class
    }

    // Take field
    pub fn take_derived_npa_class(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.derived_npa_class, ::std::string::String::new())
    }

    // string common_cust_id = 77;


    pub fn get_common_cust_id(&self) -> &str {
        &self.common_cust_id
    }
    pub fn clear_common_cust_id(&mut self) {
        self.common_cust_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_common_cust_id(&mut self, v: ::std::string::String) {
        self.common_cust_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_cust_id(&mut self) -> &mut ::std::string::String {
        &mut self.common_cust_id
    }

    // Take field
    pub fn take_common_cust_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.common_cust_id, ::std::string::String::new())
    }

    // int64 derived_next_reprice_date = 78;


    pub fn get_derived_next_reprice_date(&self) -> i64 {
        self.derived_next_reprice_date
    }
    pub fn clear_derived_next_reprice_date(&mut self) {
        self.derived_next_reprice_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_derived_next_reprice_date(&mut self, v: i64) {
        self.derived_next_reprice_date = v;
    }

    // string derived_risk_weight = 79;


    pub fn get_derived_risk_weight(&self) -> &str {
        &self.derived_risk_weight
    }
    pub fn clear_derived_risk_weight(&mut self) {
        self.derived_risk_weight.clear();
    }

    // Param is passed by value, moved
    pub fn set_derived_risk_weight(&mut self, v: ::std::string::String) {
        self.derived_risk_weight = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_derived_risk_weight(&mut self) -> &mut ::std::string::String {
        &mut self.derived_risk_weight
    }

    // Take field
    pub fn take_derived_risk_weight(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.derived_risk_weight, ::std::string::String::new())
    }

    // string restructure_flag = 80;


    pub fn get_restructure_flag(&self) -> &str {
        &self.restructure_flag
    }
    pub fn clear_restructure_flag(&mut self) {
        self.restructure_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_restructure_flag(&mut self, v: ::std::string::String) {
        self.restructure_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_restructure_flag(&mut self) -> &mut ::std::string::String {
        &mut self.restructure_flag
    }

    // Take field
    pub fn take_restructure_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.restructure_flag, ::std::string::String::new())
    }

    // string resid = 81;


    pub fn get_resid(&self) -> &str {
        &self.resid
    }
    pub fn clear_resid(&mut self) {
        self.resid.clear();
    }

    // Param is passed by value, moved
    pub fn set_resid(&mut self, v: ::std::string::String) {
        self.resid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resid(&mut self) -> &mut ::std::string::String {
        &mut self.resid
    }

    // Take field
    pub fn take_resid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resid, ::std::string::String::new())
    }

    // string ia_line = 82;


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

    // string sma_flag = 83;


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

    // repeated .Cashflow cashflows = 84;


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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.disbursed_amt = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_loan_bal_lcy = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.int_rate = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ei_amt_crnt = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.int_type)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_p_bal_due_local_ccy = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_i_bal_due_local_ccy = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ei_amt_paid_adv_lcy = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.pre_ei_bal_lcy = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.acc_open_value_date = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.maturity_date = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_start_date_crnt = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_end_date_crnt = tmp;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ei_pay_freq_crnt)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.emi_last_paid_date_crnt = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_pay_day = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_orginal_term = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_bal_term = tmp;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rep_bm)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.spread)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_rep_date = tmp;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.next_rep_date = tmp;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.rep_freq = tmp;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.no_ei_structures = tmp;
                },
                26 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.npa_class)?;
                },
                27 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remark)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.months_os_comb)?;
                },
                29 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mor_type)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.from_mor_date = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.to_mor_date = tmp;
                },
                32 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.recalc_ei_amt_flag)?;
                },
                33 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mor_int_calc)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.bullet_pay_flag)?;
                },
                35 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.restrct_flag)?;
                },
                36 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.residential_mortgage)?;
                },
                37 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.risk_weight)?;
                },
                38 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.internal_rating)?;
                },
                39 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.external_rating)?;
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.contractual_tenor = tmp;
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.residual_tenor = tmp;
                },
                42 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_constitution_code)?;
                },
                43 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prod_code)?;
                },
                44 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.p_gl_code)?;
                },
                45 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.m_npaclass)?;
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.acrd_int = tmp;
                },
                47 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_id)?;
                },
                48 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cust_name)?;
                },
                49 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.group_id)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.group_name)?;
                },
                51 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.branch_code)?;
                },
                52 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sector)?;
                },
                53 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.industry)?;
                },
                54 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ltv)?;
                },
                55 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.overdue_acc)?;
                },
                56 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.excess_acc)?;
                },
                57 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loan_type)?;
                },
                58 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.resid_int = tmp;
                },
                59 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ccy)?;
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.hdfc_ltd_percent = tmp;
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sec_percent = tmp;
                },
                62 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.overdue_type)?;
                },
                63 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                64 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.structure_number)?;
                },
                65 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.memi = tmp;
                },
                66 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ost_bal = tmp;
                },
                67 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.roi = tmp;
                },
                68 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.asondate = tmp;
                },
                69 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.emi_overdue_gl_cd = tmp;
                },
                70 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.pre_emi_overdue_gl_cd = tmp;
                },
                71 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.excess_emi_gl_cd = tmp;
                },
                72 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.excess_pre_emi_gl_cd = tmp;
                },
                73 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_prin_amt = tmp;
                },
                74 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_int_amt = tmp;
                },
                75 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.balm_l2)?;
                },
                76 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.derived_npa_class)?;
                },
                77 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.common_cust_id)?;
                },
                78 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.derived_next_reprice_date = tmp;
                },
                79 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.derived_risk_weight)?;
                },
                80 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.restructure_flag)?;
                },
                81 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resid)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ia_line)?;
                },
                83 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                84 => {
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
        if self.disbursed_amt != 0. {
            my_size += 9;
        }
        if self.os_loan_bal_lcy != 0. {
            my_size += 9;
        }
        if self.int_rate != 0. {
            my_size += 9;
        }
        if self.ei_amt_crnt != 0. {
            my_size += 9;
        }
        if !self.int_type.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.int_type);
        }
        if self.os_p_bal_due_local_ccy != 0. {
            my_size += 9;
        }
        if self.os_i_bal_due_local_ccy != 0. {
            my_size += 9;
        }
        if self.ei_amt_paid_adv_lcy != 0. {
            my_size += 9;
        }
        if self.pre_ei_bal_lcy != 0. {
            my_size += 9;
        }
        if self.acc_open_value_date != 0 {
            my_size += ::protobuf::rt::value_size(11, self.acc_open_value_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.maturity_date != 0 {
            my_size += ::protobuf::rt::value_size(12, self.maturity_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_start_date_crnt != 0 {
            my_size += ::protobuf::rt::value_size(13, self.ei_start_date_crnt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_end_date_crnt != 0 {
            my_size += ::protobuf::rt::value_size(14, self.ei_end_date_crnt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ei_pay_freq_crnt.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.ei_pay_freq_crnt);
        }
        if self.emi_last_paid_date_crnt != 0 {
            my_size += ::protobuf::rt::value_size(16, self.emi_last_paid_date_crnt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_pay_day != 0 {
            my_size += ::protobuf::rt::value_size(17, self.ei_pay_day, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_orginal_term != 0 {
            my_size += ::protobuf::rt::value_size(18, self.ei_orginal_term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_bal_term != 0 {
            my_size += ::protobuf::rt::value_size(19, self.ei_bal_term, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.rep_bm.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.rep_bm);
        }
        if !self.spread.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.spread);
        }
        if self.last_rep_date != 0 {
            my_size += ::protobuf::rt::value_size(22, self.last_rep_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.next_rep_date != 0 {
            my_size += ::protobuf::rt::value_size(23, self.next_rep_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.rep_freq != 0 {
            my_size += ::protobuf::rt::value_size(24, self.rep_freq, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.no_ei_structures != 0 {
            my_size += ::protobuf::rt::value_size(25, self.no_ei_structures, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.npa_class.is_empty() {
            my_size += ::protobuf::rt::string_size(26, &self.npa_class);
        }
        if !self.remark.is_empty() {
            my_size += ::protobuf::rt::string_size(27, &self.remark);
        }
        if !self.months_os_comb.is_empty() {
            my_size += ::protobuf::rt::string_size(28, &self.months_os_comb);
        }
        if !self.mor_type.is_empty() {
            my_size += ::protobuf::rt::string_size(29, &self.mor_type);
        }
        if self.from_mor_date != 0 {
            my_size += ::protobuf::rt::value_size(30, self.from_mor_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.to_mor_date != 0 {
            my_size += ::protobuf::rt::value_size(31, self.to_mor_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.recalc_ei_amt_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(32, &self.recalc_ei_amt_flag);
        }
        if !self.mor_int_calc.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.mor_int_calc);
        }
        if !self.bullet_pay_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.bullet_pay_flag);
        }
        if !self.restrct_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(35, &self.restrct_flag);
        }
        if !self.residential_mortgage.is_empty() {
            my_size += ::protobuf::rt::string_size(36, &self.residential_mortgage);
        }
        if !self.risk_weight.is_empty() {
            my_size += ::protobuf::rt::string_size(37, &self.risk_weight);
        }
        if !self.internal_rating.is_empty() {
            my_size += ::protobuf::rt::string_size(38, &self.internal_rating);
        }
        if !self.external_rating.is_empty() {
            my_size += ::protobuf::rt::string_size(39, &self.external_rating);
        }
        if self.contractual_tenor != 0 {
            my_size += ::protobuf::rt::value_size(40, self.contractual_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.residual_tenor != 0 {
            my_size += ::protobuf::rt::value_size(41, self.residual_tenor, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.cust_constitution_code.is_empty() {
            my_size += ::protobuf::rt::string_size(42, &self.cust_constitution_code);
        }
        if !self.prod_code.is_empty() {
            my_size += ::protobuf::rt::string_size(43, &self.prod_code);
        }
        if !self.p_gl_code.is_empty() {
            my_size += ::protobuf::rt::string_size(44, &self.p_gl_code);
        }
        if !self.m_npaclass.is_empty() {
            my_size += ::protobuf::rt::string_size(45, &self.m_npaclass);
        }
        if self.acrd_int != 0. {
            my_size += 10;
        }
        if !self.cust_id.is_empty() {
            my_size += ::protobuf::rt::string_size(47, &self.cust_id);
        }
        if !self.cust_name.is_empty() {
            my_size += ::protobuf::rt::string_size(48, &self.cust_name);
        }
        if !self.group_id.is_empty() {
            my_size += ::protobuf::rt::string_size(49, &self.group_id);
        }
        if !self.group_name.is_empty() {
            my_size += ::protobuf::rt::string_size(50, &self.group_name);
        }
        if !self.branch_code.is_empty() {
            my_size += ::protobuf::rt::string_size(51, &self.branch_code);
        }
        if !self.sector.is_empty() {
            my_size += ::protobuf::rt::string_size(52, &self.sector);
        }
        if !self.industry.is_empty() {
            my_size += ::protobuf::rt::string_size(53, &self.industry);
        }
        if !self.ltv.is_empty() {
            my_size += ::protobuf::rt::string_size(54, &self.ltv);
        }
        if !self.overdue_acc.is_empty() {
            my_size += ::protobuf::rt::string_size(55, &self.overdue_acc);
        }
        if !self.excess_acc.is_empty() {
            my_size += ::protobuf::rt::string_size(56, &self.excess_acc);
        }
        if !self.loan_type.is_empty() {
            my_size += ::protobuf::rt::string_size(57, &self.loan_type);
        }
        if self.resid_int != 0. {
            my_size += 10;
        }
        if !self.ccy.is_empty() {
            my_size += ::protobuf::rt::string_size(59, &self.ccy);
        }
        if self.hdfc_ltd_percent != 0. {
            my_size += 10;
        }
        if self.sec_percent != 0. {
            my_size += 10;
        }
        if !self.overdue_type.is_empty() {
            my_size += ::protobuf::rt::string_size(62, &self.overdue_type);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(63, &self.alm_line);
        }
        if !self.structure_number.is_empty() {
            my_size += ::protobuf::rt::string_size(64, &self.structure_number);
        }
        if self.memi != 0. {
            my_size += 10;
        }
        if self.ost_bal != 0. {
            my_size += 10;
        }
        if self.roi != 0. {
            my_size += 10;
        }
        if self.asondate != 0 {
            my_size += ::protobuf::rt::value_size(68, self.asondate, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.emi_overdue_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(69, self.emi_overdue_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.pre_emi_overdue_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(70, self.pre_emi_overdue_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.excess_emi_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(71, self.excess_emi_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.excess_pre_emi_gl_cd != 0 {
            my_size += ::protobuf::rt::value_size(72, self.excess_pre_emi_gl_cd, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.tot_prin_amt != 0. {
            my_size += 10;
        }
        if self.tot_int_amt != 0. {
            my_size += 10;
        }
        if !self.balm_l2.is_empty() {
            my_size += ::protobuf::rt::string_size(75, &self.balm_l2);
        }
        if !self.derived_npa_class.is_empty() {
            my_size += ::protobuf::rt::string_size(76, &self.derived_npa_class);
        }
        if !self.common_cust_id.is_empty() {
            my_size += ::protobuf::rt::string_size(77, &self.common_cust_id);
        }
        if self.derived_next_reprice_date != 0 {
            my_size += ::protobuf::rt::value_size(78, self.derived_next_reprice_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.derived_risk_weight.is_empty() {
            my_size += ::protobuf::rt::string_size(79, &self.derived_risk_weight);
        }
        if !self.restructure_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(80, &self.restructure_flag);
        }
        if !self.resid.is_empty() {
            my_size += ::protobuf::rt::string_size(81, &self.resid);
        }
        if !self.ia_line.is_empty() {
            my_size += ::protobuf::rt::string_size(82, &self.ia_line);
        }
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(83, &self.sma_flag);
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
        if self.disbursed_amt != 0. {
            os.write_double(2, self.disbursed_amt)?;
        }
        if self.os_loan_bal_lcy != 0. {
            os.write_double(3, self.os_loan_bal_lcy)?;
        }
        if self.int_rate != 0. {
            os.write_double(4, self.int_rate)?;
        }
        if self.ei_amt_crnt != 0. {
            os.write_double(5, self.ei_amt_crnt)?;
        }
        if !self.int_type.is_empty() {
            os.write_string(6, &self.int_type)?;
        }
        if self.os_p_bal_due_local_ccy != 0. {
            os.write_double(7, self.os_p_bal_due_local_ccy)?;
        }
        if self.os_i_bal_due_local_ccy != 0. {
            os.write_double(8, self.os_i_bal_due_local_ccy)?;
        }
        if self.ei_amt_paid_adv_lcy != 0. {
            os.write_double(9, self.ei_amt_paid_adv_lcy)?;
        }
        if self.pre_ei_bal_lcy != 0. {
            os.write_double(10, self.pre_ei_bal_lcy)?;
        }
        if self.acc_open_value_date != 0 {
            os.write_int64(11, self.acc_open_value_date)?;
        }
        if self.maturity_date != 0 {
            os.write_int64(12, self.maturity_date)?;
        }
        if self.ei_start_date_crnt != 0 {
            os.write_int64(13, self.ei_start_date_crnt)?;
        }
        if self.ei_end_date_crnt != 0 {
            os.write_int64(14, self.ei_end_date_crnt)?;
        }
        if !self.ei_pay_freq_crnt.is_empty() {
            os.write_string(15, &self.ei_pay_freq_crnt)?;
        }
        if self.emi_last_paid_date_crnt != 0 {
            os.write_int64(16, self.emi_last_paid_date_crnt)?;
        }
        if self.ei_pay_day != 0 {
            os.write_int64(17, self.ei_pay_day)?;
        }
        if self.ei_orginal_term != 0 {
            os.write_int64(18, self.ei_orginal_term)?;
        }
        if self.ei_bal_term != 0 {
            os.write_int64(19, self.ei_bal_term)?;
        }
        if !self.rep_bm.is_empty() {
            os.write_string(20, &self.rep_bm)?;
        }
        if !self.spread.is_empty() {
            os.write_string(21, &self.spread)?;
        }
        if self.last_rep_date != 0 {
            os.write_int64(22, self.last_rep_date)?;
        }
        if self.next_rep_date != 0 {
            os.write_int64(23, self.next_rep_date)?;
        }
        if self.rep_freq != 0 {
            os.write_int64(24, self.rep_freq)?;
        }
        if self.no_ei_structures != 0 {
            os.write_int64(25, self.no_ei_structures)?;
        }
        if !self.npa_class.is_empty() {
            os.write_string(26, &self.npa_class)?;
        }
        if !self.remark.is_empty() {
            os.write_string(27, &self.remark)?;
        }
        if !self.months_os_comb.is_empty() {
            os.write_string(28, &self.months_os_comb)?;
        }
        if !self.mor_type.is_empty() {
            os.write_string(29, &self.mor_type)?;
        }
        if self.from_mor_date != 0 {
            os.write_int64(30, self.from_mor_date)?;
        }
        if self.to_mor_date != 0 {
            os.write_int64(31, self.to_mor_date)?;
        }
        if !self.recalc_ei_amt_flag.is_empty() {
            os.write_string(32, &self.recalc_ei_amt_flag)?;
        }
        if !self.mor_int_calc.is_empty() {
            os.write_string(33, &self.mor_int_calc)?;
        }
        if !self.bullet_pay_flag.is_empty() {
            os.write_string(34, &self.bullet_pay_flag)?;
        }
        if !self.restrct_flag.is_empty() {
            os.write_string(35, &self.restrct_flag)?;
        }
        if !self.residential_mortgage.is_empty() {
            os.write_string(36, &self.residential_mortgage)?;
        }
        if !self.risk_weight.is_empty() {
            os.write_string(37, &self.risk_weight)?;
        }
        if !self.internal_rating.is_empty() {
            os.write_string(38, &self.internal_rating)?;
        }
        if !self.external_rating.is_empty() {
            os.write_string(39, &self.external_rating)?;
        }
        if self.contractual_tenor != 0 {
            os.write_int64(40, self.contractual_tenor)?;
        }
        if self.residual_tenor != 0 {
            os.write_int64(41, self.residual_tenor)?;
        }
        if !self.cust_constitution_code.is_empty() {
            os.write_string(42, &self.cust_constitution_code)?;
        }
        if !self.prod_code.is_empty() {
            os.write_string(43, &self.prod_code)?;
        }
        if !self.p_gl_code.is_empty() {
            os.write_string(44, &self.p_gl_code)?;
        }
        if !self.m_npaclass.is_empty() {
            os.write_string(45, &self.m_npaclass)?;
        }
        if self.acrd_int != 0. {
            os.write_double(46, self.acrd_int)?;
        }
        if !self.cust_id.is_empty() {
            os.write_string(47, &self.cust_id)?;
        }
        if !self.cust_name.is_empty() {
            os.write_string(48, &self.cust_name)?;
        }
        if !self.group_id.is_empty() {
            os.write_string(49, &self.group_id)?;
        }
        if !self.group_name.is_empty() {
            os.write_string(50, &self.group_name)?;
        }
        if !self.branch_code.is_empty() {
            os.write_string(51, &self.branch_code)?;
        }
        if !self.sector.is_empty() {
            os.write_string(52, &self.sector)?;
        }
        if !self.industry.is_empty() {
            os.write_string(53, &self.industry)?;
        }
        if !self.ltv.is_empty() {
            os.write_string(54, &self.ltv)?;
        }
        if !self.overdue_acc.is_empty() {
            os.write_string(55, &self.overdue_acc)?;
        }
        if !self.excess_acc.is_empty() {
            os.write_string(56, &self.excess_acc)?;
        }
        if !self.loan_type.is_empty() {
            os.write_string(57, &self.loan_type)?;
        }
        if self.resid_int != 0. {
            os.write_double(58, self.resid_int)?;
        }
        if !self.ccy.is_empty() {
            os.write_string(59, &self.ccy)?;
        }
        if self.hdfc_ltd_percent != 0. {
            os.write_double(60, self.hdfc_ltd_percent)?;
        }
        if self.sec_percent != 0. {
            os.write_double(61, self.sec_percent)?;
        }
        if !self.overdue_type.is_empty() {
            os.write_string(62, &self.overdue_type)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(63, &self.alm_line)?;
        }
        if !self.structure_number.is_empty() {
            os.write_string(64, &self.structure_number)?;
        }
        if self.memi != 0. {
            os.write_double(65, self.memi)?;
        }
        if self.ost_bal != 0. {
            os.write_double(66, self.ost_bal)?;
        }
        if self.roi != 0. {
            os.write_double(67, self.roi)?;
        }
        if self.asondate != 0 {
            os.write_int64(68, self.asondate)?;
        }
        if self.emi_overdue_gl_cd != 0 {
            os.write_int64(69, self.emi_overdue_gl_cd)?;
        }
        if self.pre_emi_overdue_gl_cd != 0 {
            os.write_int64(70, self.pre_emi_overdue_gl_cd)?;
        }
        if self.excess_emi_gl_cd != 0 {
            os.write_int64(71, self.excess_emi_gl_cd)?;
        }
        if self.excess_pre_emi_gl_cd != 0 {
            os.write_int64(72, self.excess_pre_emi_gl_cd)?;
        }
        if self.tot_prin_amt != 0. {
            os.write_double(73, self.tot_prin_amt)?;
        }
        if self.tot_int_amt != 0. {
            os.write_double(74, self.tot_int_amt)?;
        }
        if !self.balm_l2.is_empty() {
            os.write_string(75, &self.balm_l2)?;
        }
        if !self.derived_npa_class.is_empty() {
            os.write_string(76, &self.derived_npa_class)?;
        }
        if !self.common_cust_id.is_empty() {
            os.write_string(77, &self.common_cust_id)?;
        }
        if self.derived_next_reprice_date != 0 {
            os.write_int64(78, self.derived_next_reprice_date)?;
        }
        if !self.derived_risk_weight.is_empty() {
            os.write_string(79, &self.derived_risk_weight)?;
        }
        if !self.restructure_flag.is_empty() {
            os.write_string(80, &self.restructure_flag)?;
        }
        if !self.resid.is_empty() {
            os.write_string(81, &self.resid)?;
        }
        if !self.ia_line.is_empty() {
            os.write_string(82, &self.ia_line)?;
        }
        if !self.sma_flag.is_empty() {
            os.write_string(83, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(84, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "disbursed_amt",
                |m: &AccountWithCashflows| { &m.disbursed_amt },
                |m: &mut AccountWithCashflows| { &mut m.disbursed_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_loan_bal_lcy",
                |m: &AccountWithCashflows| { &m.os_loan_bal_lcy },
                |m: &mut AccountWithCashflows| { &mut m.os_loan_bal_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "int_rate",
                |m: &AccountWithCashflows| { &m.int_rate },
                |m: &mut AccountWithCashflows| { &mut m.int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ei_amt_crnt",
                |m: &AccountWithCashflows| { &m.ei_amt_crnt },
                |m: &mut AccountWithCashflows| { &mut m.ei_amt_crnt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "int_type",
                |m: &AccountWithCashflows| { &m.int_type },
                |m: &mut AccountWithCashflows| { &mut m.int_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_p_bal_due_local_ccy",
                |m: &AccountWithCashflows| { &m.os_p_bal_due_local_ccy },
                |m: &mut AccountWithCashflows| { &mut m.os_p_bal_due_local_ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_i_bal_due_local_ccy",
                |m: &AccountWithCashflows| { &m.os_i_bal_due_local_ccy },
                |m: &mut AccountWithCashflows| { &mut m.os_i_bal_due_local_ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ei_amt_paid_adv_lcy",
                |m: &AccountWithCashflows| { &m.ei_amt_paid_adv_lcy },
                |m: &mut AccountWithCashflows| { &mut m.ei_amt_paid_adv_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "pre_ei_bal_lcy",
                |m: &AccountWithCashflows| { &m.pre_ei_bal_lcy },
                |m: &mut AccountWithCashflows| { &mut m.pre_ei_bal_lcy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "acc_open_value_date",
                |m: &AccountWithCashflows| { &m.acc_open_value_date },
                |m: &mut AccountWithCashflows| { &mut m.acc_open_value_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "maturity_date",
                |m: &AccountWithCashflows| { &m.maturity_date },
                |m: &mut AccountWithCashflows| { &mut m.maturity_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_start_date_crnt",
                |m: &AccountWithCashflows| { &m.ei_start_date_crnt },
                |m: &mut AccountWithCashflows| { &mut m.ei_start_date_crnt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_end_date_crnt",
                |m: &AccountWithCashflows| { &m.ei_end_date_crnt },
                |m: &mut AccountWithCashflows| { &mut m.ei_end_date_crnt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ei_pay_freq_crnt",
                |m: &AccountWithCashflows| { &m.ei_pay_freq_crnt },
                |m: &mut AccountWithCashflows| { &mut m.ei_pay_freq_crnt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "emi_last_paid_date_crnt",
                |m: &AccountWithCashflows| { &m.emi_last_paid_date_crnt },
                |m: &mut AccountWithCashflows| { &mut m.emi_last_paid_date_crnt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_pay_day",
                |m: &AccountWithCashflows| { &m.ei_pay_day },
                |m: &mut AccountWithCashflows| { &mut m.ei_pay_day },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_orginal_term",
                |m: &AccountWithCashflows| { &m.ei_orginal_term },
                |m: &mut AccountWithCashflows| { &mut m.ei_orginal_term },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_bal_term",
                |m: &AccountWithCashflows| { &m.ei_bal_term },
                |m: &mut AccountWithCashflows| { &mut m.ei_bal_term },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "rep_bm",
                |m: &AccountWithCashflows| { &m.rep_bm },
                |m: &mut AccountWithCashflows| { &mut m.rep_bm },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "spread",
                |m: &AccountWithCashflows| { &m.spread },
                |m: &mut AccountWithCashflows| { &mut m.spread },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "last_rep_date",
                |m: &AccountWithCashflows| { &m.last_rep_date },
                |m: &mut AccountWithCashflows| { &mut m.last_rep_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "next_rep_date",
                |m: &AccountWithCashflows| { &m.next_rep_date },
                |m: &mut AccountWithCashflows| { &mut m.next_rep_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "rep_freq",
                |m: &AccountWithCashflows| { &m.rep_freq },
                |m: &mut AccountWithCashflows| { &mut m.rep_freq },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "no_ei_structures",
                |m: &AccountWithCashflows| { &m.no_ei_structures },
                |m: &mut AccountWithCashflows| { &mut m.no_ei_structures },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "npa_class",
                |m: &AccountWithCashflows| { &m.npa_class },
                |m: &mut AccountWithCashflows| { &mut m.npa_class },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "remark",
                |m: &AccountWithCashflows| { &m.remark },
                |m: &mut AccountWithCashflows| { &mut m.remark },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "months_os_comb",
                |m: &AccountWithCashflows| { &m.months_os_comb },
                |m: &mut AccountWithCashflows| { &mut m.months_os_comb },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "mor_type",
                |m: &AccountWithCashflows| { &m.mor_type },
                |m: &mut AccountWithCashflows| { &mut m.mor_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "from_mor_date",
                |m: &AccountWithCashflows| { &m.from_mor_date },
                |m: &mut AccountWithCashflows| { &mut m.from_mor_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "to_mor_date",
                |m: &AccountWithCashflows| { &m.to_mor_date },
                |m: &mut AccountWithCashflows| { &mut m.to_mor_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "recalc_ei_amt_flag",
                |m: &AccountWithCashflows| { &m.recalc_ei_amt_flag },
                |m: &mut AccountWithCashflows| { &mut m.recalc_ei_amt_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "mor_int_calc",
                |m: &AccountWithCashflows| { &m.mor_int_calc },
                |m: &mut AccountWithCashflows| { &mut m.mor_int_calc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "bullet_pay_flag",
                |m: &AccountWithCashflows| { &m.bullet_pay_flag },
                |m: &mut AccountWithCashflows| { &mut m.bullet_pay_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "restrct_flag",
                |m: &AccountWithCashflows| { &m.restrct_flag },
                |m: &mut AccountWithCashflows| { &mut m.restrct_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "residential_mortgage",
                |m: &AccountWithCashflows| { &m.residential_mortgage },
                |m: &mut AccountWithCashflows| { &mut m.residential_mortgage },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "risk_weight",
                |m: &AccountWithCashflows| { &m.risk_weight },
                |m: &mut AccountWithCashflows| { &mut m.risk_weight },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "internal_rating",
                |m: &AccountWithCashflows| { &m.internal_rating },
                |m: &mut AccountWithCashflows| { &mut m.internal_rating },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "external_rating",
                |m: &AccountWithCashflows| { &m.external_rating },
                |m: &mut AccountWithCashflows| { &mut m.external_rating },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "contractual_tenor",
                |m: &AccountWithCashflows| { &m.contractual_tenor },
                |m: &mut AccountWithCashflows| { &mut m.contractual_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "residual_tenor",
                |m: &AccountWithCashflows| { &m.residual_tenor },
                |m: &mut AccountWithCashflows| { &mut m.residual_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_constitution_code",
                |m: &AccountWithCashflows| { &m.cust_constitution_code },
                |m: &mut AccountWithCashflows| { &mut m.cust_constitution_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "prod_code",
                |m: &AccountWithCashflows| { &m.prod_code },
                |m: &mut AccountWithCashflows| { &mut m.prod_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "p_gl_code",
                |m: &AccountWithCashflows| { &m.p_gl_code },
                |m: &mut AccountWithCashflows| { &mut m.p_gl_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "m_npaclass",
                |m: &AccountWithCashflows| { &m.m_npaclass },
                |m: &mut AccountWithCashflows| { &mut m.m_npaclass },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "acrd_int",
                |m: &AccountWithCashflows| { &m.acrd_int },
                |m: &mut AccountWithCashflows| { &mut m.acrd_int },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_id",
                |m: &AccountWithCashflows| { &m.cust_id },
                |m: &mut AccountWithCashflows| { &mut m.cust_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cust_name",
                |m: &AccountWithCashflows| { &m.cust_name },
                |m: &mut AccountWithCashflows| { &mut m.cust_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_id",
                |m: &AccountWithCashflows| { &m.group_id },
                |m: &mut AccountWithCashflows| { &mut m.group_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_name",
                |m: &AccountWithCashflows| { &m.group_name },
                |m: &mut AccountWithCashflows| { &mut m.group_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "branch_code",
                |m: &AccountWithCashflows| { &m.branch_code },
                |m: &mut AccountWithCashflows| { &mut m.branch_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sector",
                |m: &AccountWithCashflows| { &m.sector },
                |m: &mut AccountWithCashflows| { &mut m.sector },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "industry",
                |m: &AccountWithCashflows| { &m.industry },
                |m: &mut AccountWithCashflows| { &mut m.industry },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ltv",
                |m: &AccountWithCashflows| { &m.ltv },
                |m: &mut AccountWithCashflows| { &mut m.ltv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "overdue_acc",
                |m: &AccountWithCashflows| { &m.overdue_acc },
                |m: &mut AccountWithCashflows| { &mut m.overdue_acc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "excess_acc",
                |m: &AccountWithCashflows| { &m.excess_acc },
                |m: &mut AccountWithCashflows| { &mut m.excess_acc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "loan_type",
                |m: &AccountWithCashflows| { &m.loan_type },
                |m: &mut AccountWithCashflows| { &mut m.loan_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "resid_int",
                |m: &AccountWithCashflows| { &m.resid_int },
                |m: &mut AccountWithCashflows| { &mut m.resid_int },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ccy",
                |m: &AccountWithCashflows| { &m.ccy },
                |m: &mut AccountWithCashflows| { &mut m.ccy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "hdfc_ltd_percent",
                |m: &AccountWithCashflows| { &m.hdfc_ltd_percent },
                |m: &mut AccountWithCashflows| { &mut m.hdfc_ltd_percent },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sec_percent",
                |m: &AccountWithCashflows| { &m.sec_percent },
                |m: &mut AccountWithCashflows| { &mut m.sec_percent },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "overdue_type",
                |m: &AccountWithCashflows| { &m.overdue_type },
                |m: &mut AccountWithCashflows| { &mut m.overdue_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &AccountWithCashflows| { &m.alm_line },
                |m: &mut AccountWithCashflows| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "structure_number",
                |m: &AccountWithCashflows| { &m.structure_number },
                |m: &mut AccountWithCashflows| { &mut m.structure_number },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "memi",
                |m: &AccountWithCashflows| { &m.memi },
                |m: &mut AccountWithCashflows| { &mut m.memi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ost_bal",
                |m: &AccountWithCashflows| { &m.ost_bal },
                |m: &mut AccountWithCashflows| { &mut m.ost_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "roi",
                |m: &AccountWithCashflows| { &m.roi },
                |m: &mut AccountWithCashflows| { &mut m.roi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "asondate",
                |m: &AccountWithCashflows| { &m.asondate },
                |m: &mut AccountWithCashflows| { &mut m.asondate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "emi_overdue_gl_cd",
                |m: &AccountWithCashflows| { &m.emi_overdue_gl_cd },
                |m: &mut AccountWithCashflows| { &mut m.emi_overdue_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "pre_emi_overdue_gl_cd",
                |m: &AccountWithCashflows| { &m.pre_emi_overdue_gl_cd },
                |m: &mut AccountWithCashflows| { &mut m.pre_emi_overdue_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "excess_emi_gl_cd",
                |m: &AccountWithCashflows| { &m.excess_emi_gl_cd },
                |m: &mut AccountWithCashflows| { &mut m.excess_emi_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "excess_pre_emi_gl_cd",
                |m: &AccountWithCashflows| { &m.excess_pre_emi_gl_cd },
                |m: &mut AccountWithCashflows| { &mut m.excess_pre_emi_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_prin_amt",
                |m: &AccountWithCashflows| { &m.tot_prin_amt },
                |m: &mut AccountWithCashflows| { &mut m.tot_prin_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_int_amt",
                |m: &AccountWithCashflows| { &m.tot_int_amt },
                |m: &mut AccountWithCashflows| { &mut m.tot_int_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "balm_l2",
                |m: &AccountWithCashflows| { &m.balm_l2 },
                |m: &mut AccountWithCashflows| { &mut m.balm_l2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "derived_npa_class",
                |m: &AccountWithCashflows| { &m.derived_npa_class },
                |m: &mut AccountWithCashflows| { &mut m.derived_npa_class },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "common_cust_id",
                |m: &AccountWithCashflows| { &m.common_cust_id },
                |m: &mut AccountWithCashflows| { &mut m.common_cust_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "derived_next_reprice_date",
                |m: &AccountWithCashflows| { &m.derived_next_reprice_date },
                |m: &mut AccountWithCashflows| { &mut m.derived_next_reprice_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "derived_risk_weight",
                |m: &AccountWithCashflows| { &m.derived_risk_weight },
                |m: &mut AccountWithCashflows| { &mut m.derived_risk_weight },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "restructure_flag",
                |m: &AccountWithCashflows| { &m.restructure_flag },
                |m: &mut AccountWithCashflows| { &mut m.restructure_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "resid",
                |m: &AccountWithCashflows| { &m.resid },
                |m: &mut AccountWithCashflows| { &mut m.resid },
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
        self.acc_no.clear();
        self.disbursed_amt = 0.;
        self.os_loan_bal_lcy = 0.;
        self.int_rate = 0.;
        self.ei_amt_crnt = 0.;
        self.int_type.clear();
        self.os_p_bal_due_local_ccy = 0.;
        self.os_i_bal_due_local_ccy = 0.;
        self.ei_amt_paid_adv_lcy = 0.;
        self.pre_ei_bal_lcy = 0.;
        self.acc_open_value_date = 0;
        self.maturity_date = 0;
        self.ei_start_date_crnt = 0;
        self.ei_end_date_crnt = 0;
        self.ei_pay_freq_crnt.clear();
        self.emi_last_paid_date_crnt = 0;
        self.ei_pay_day = 0;
        self.ei_orginal_term = 0;
        self.ei_bal_term = 0;
        self.rep_bm.clear();
        self.spread.clear();
        self.last_rep_date = 0;
        self.next_rep_date = 0;
        self.rep_freq = 0;
        self.no_ei_structures = 0;
        self.npa_class.clear();
        self.remark.clear();
        self.months_os_comb.clear();
        self.mor_type.clear();
        self.from_mor_date = 0;
        self.to_mor_date = 0;
        self.recalc_ei_amt_flag.clear();
        self.mor_int_calc.clear();
        self.bullet_pay_flag.clear();
        self.restrct_flag.clear();
        self.residential_mortgage.clear();
        self.risk_weight.clear();
        self.internal_rating.clear();
        self.external_rating.clear();
        self.contractual_tenor = 0;
        self.residual_tenor = 0;
        self.cust_constitution_code.clear();
        self.prod_code.clear();
        self.p_gl_code.clear();
        self.m_npaclass.clear();
        self.acrd_int = 0.;
        self.cust_id.clear();
        self.cust_name.clear();
        self.group_id.clear();
        self.group_name.clear();
        self.branch_code.clear();
        self.sector.clear();
        self.industry.clear();
        self.ltv.clear();
        self.overdue_acc.clear();
        self.excess_acc.clear();
        self.loan_type.clear();
        self.resid_int = 0.;
        self.ccy.clear();
        self.hdfc_ltd_percent = 0.;
        self.sec_percent = 0.;
        self.overdue_type.clear();
        self.alm_line.clear();
        self.structure_number.clear();
        self.memi = 0.;
        self.ost_bal = 0.;
        self.roi = 0.;
        self.asondate = 0;
        self.emi_overdue_gl_cd = 0;
        self.pre_emi_overdue_gl_cd = 0;
        self.excess_emi_gl_cd = 0;
        self.excess_pre_emi_gl_cd = 0;
        self.tot_prin_amt = 0.;
        self.tot_int_amt = 0.;
        self.balm_l2.clear();
        self.derived_npa_class.clear();
        self.common_cust_id.clear();
        self.derived_next_reprice_date = 0;
        self.derived_risk_weight.clear();
        self.restructure_flag.clear();
        self.resid.clear();
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
    \n\x12loans_merger.proto\"r\n\x08Cashflow\x12'\n\x0finterest_amount\x18\
    \x01\x20\x01(\x01R\x0einterestAmount\x12)\n\x10principal_amount\x18\x02\
    \x20\x01(\x01R\x0fprincipalAmount\x12\x12\n\x04date\x18\x03\x20\x01(\x03\
    R\x04date\"\x88\x17\n\x14AccountWithCashflows\x12\x15\n\x06acc_no\x18\
    \x01\x20\x01(\tR\x05accNo\x12#\n\rdisbursed_amt\x18\x02\x20\x01(\x01R\
    \x0cdisbursedAmt\x12%\n\x0fos_loan_bal_lcy\x18\x03\x20\x01(\x01R\x0cosLo\
    anBalLcy\x12\x19\n\x08int_rate\x18\x04\x20\x01(\x01R\x07intRate\x12\x1e\
    \n\x0bei_amt_crnt\x18\x05\x20\x01(\x01R\teiAmtCrnt\x12\x19\n\x08int_type\
    \x18\x06\x20\x01(\tR\x07intType\x121\n\x16os_p_bal_due_local_ccy\x18\x07\
    \x20\x01(\x01R\x11osPBalDueLocalCcy\x121\n\x16os_i_bal_due_local_ccy\x18\
    \x08\x20\x01(\x01R\x11osIBalDueLocalCcy\x12,\n\x13ei_amt_paid_adv_lcy\
    \x18\t\x20\x01(\x01R\x0feiAmtPaidAdvLcy\x12#\n\x0epre_ei_bal_lcy\x18\n\
    \x20\x01(\x01R\x0bpreEiBalLcy\x12-\n\x13acc_open_value_date\x18\x0b\x20\
    \x01(\x03R\x10accOpenValueDate\x12#\n\rmaturity_date\x18\x0c\x20\x01(\
    \x03R\x0cmaturityDate\x12+\n\x12ei_start_date_crnt\x18\r\x20\x01(\x03R\
    \x0feiStartDateCrnt\x12'\n\x10ei_end_date_crnt\x18\x0e\x20\x01(\x03R\rei\
    EndDateCrnt\x12'\n\x10ei_pay_freq_crnt\x18\x0f\x20\x01(\tR\reiPayFreqCrn\
    t\x124\n\x17emi_last_paid_date_crnt\x18\x10\x20\x01(\x03R\x13emiLastPaid\
    DateCrnt\x12\x1c\n\nei_pay_day\x18\x11\x20\x01(\x03R\x08eiPayDay\x12&\n\
    \x0fei_orginal_term\x18\x12\x20\x01(\x03R\reiOrginalTerm\x12\x1e\n\x0bei\
    _bal_term\x18\x13\x20\x01(\x03R\teiBalTerm\x12\x15\n\x06rep_bm\x18\x14\
    \x20\x01(\tR\x05repBm\x12\x16\n\x06spread\x18\x15\x20\x01(\tR\x06spread\
    \x12\"\n\rlast_rep_date\x18\x16\x20\x01(\x03R\x0blastRepDate\x12\"\n\rne\
    xt_rep_date\x18\x17\x20\x01(\x03R\x0bnextRepDate\x12\x19\n\x08rep_freq\
    \x18\x18\x20\x01(\x03R\x07repFreq\x12(\n\x10no_ei_structures\x18\x19\x20\
    \x01(\x03R\x0enoEiStructures\x12\x1b\n\tnpa_class\x18\x1a\x20\x01(\tR\
    \x08npaClass\x12\x16\n\x06remark\x18\x1b\x20\x01(\tR\x06remark\x12$\n\
    \x0emonths_os_comb\x18\x1c\x20\x01(\tR\x0cmonthsOsComb\x12\x19\n\x08mor_\
    type\x18\x1d\x20\x01(\tR\x07morType\x12\"\n\rfrom_mor_date\x18\x1e\x20\
    \x01(\x03R\x0bfromMorDate\x12\x1e\n\x0bto_mor_date\x18\x1f\x20\x01(\x03R\
    \ttoMorDate\x12+\n\x12recalc_ei_amt_flag\x18\x20\x20\x01(\tR\x0frecalcEi\
    AmtFlag\x12\x20\n\x0cmor_int_calc\x18!\x20\x01(\tR\nmorIntCalc\x12&\n\
    \x0fbullet_pay_flag\x18\"\x20\x01(\tR\rbulletPayFlag\x12!\n\x0crestrct_f\
    lag\x18#\x20\x01(\tR\x0brestrctFlag\x121\n\x14residential_mortgage\x18$\
    \x20\x01(\tR\x13residentialMortgage\x12\x1f\n\x0brisk_weight\x18%\x20\
    \x01(\tR\nriskWeight\x12'\n\x0finternal_rating\x18&\x20\x01(\tR\x0einter\
    nalRating\x12'\n\x0fexternal_rating\x18'\x20\x01(\tR\x0eexternalRating\
    \x12+\n\x11contractual_tenor\x18(\x20\x01(\x03R\x10contractualTenor\x12%\
    \n\x0eresidual_tenor\x18)\x20\x01(\x03R\rresidualTenor\x124\n\x16cust_co\
    nstitution_code\x18*\x20\x01(\tR\x14custConstitutionCode\x12\x1b\n\tprod\
    _code\x18+\x20\x01(\tR\x08prodCode\x12\x1a\n\tp_gl_code\x18,\x20\x01(\tR\
    \x07pGlCode\x12\x1d\n\nm_npaclass\x18-\x20\x01(\tR\tmNpaclass\x12\x19\n\
    \x08acrd_int\x18.\x20\x01(\x01R\x07acrdInt\x12\x17\n\x07cust_id\x18/\x20\
    \x01(\tR\x06custId\x12\x1b\n\tcust_name\x180\x20\x01(\tR\x08custName\x12\
    \x19\n\x08group_id\x181\x20\x01(\tR\x07groupId\x12\x1d\n\ngroup_name\x18\
    2\x20\x01(\tR\tgroupName\x12\x1f\n\x0bbranch_code\x183\x20\x01(\tR\nbran\
    chCode\x12\x16\n\x06sector\x184\x20\x01(\tR\x06sector\x12\x1a\n\x08indus\
    try\x185\x20\x01(\tR\x08industry\x12\x10\n\x03ltv\x186\x20\x01(\tR\x03lt\
    v\x12\x1f\n\x0boverdue_acc\x187\x20\x01(\tR\noverdueAcc\x12\x1d\n\nexces\
    s_acc\x188\x20\x01(\tR\texcessAcc\x12\x1b\n\tloan_type\x189\x20\x01(\tR\
    \x08loanType\x12\x1b\n\tresid_int\x18:\x20\x01(\x01R\x08residInt\x12\x10\
    \n\x03ccy\x18;\x20\x01(\tR\x03ccy\x12(\n\x10hdfc_ltd_percent\x18<\x20\
    \x01(\x01R\x0ehdfcLtdPercent\x12\x1f\n\x0bsec_percent\x18=\x20\x01(\x01R\
    \nsecPercent\x12!\n\x0coverdue_type\x18>\x20\x01(\tR\x0boverdueType\x12\
    \x19\n\x08alm_line\x18?\x20\x01(\tR\x07almLine\x12)\n\x10structure_numbe\
    r\x18@\x20\x01(\tR\x0fstructureNumber\x12\x12\n\x04memi\x18A\x20\x01(\
    \x01R\x04memi\x12\x17\n\x07ost_bal\x18B\x20\x01(\x01R\x06ostBal\x12\x10\
    \n\x03roi\x18C\x20\x01(\x01R\x03roi\x12\x1a\n\x08asondate\x18D\x20\x01(\
    \x03R\x08asondate\x12)\n\x11emi_overdue_gl_cd\x18E\x20\x01(\x03R\x0eemiO\
    verdueGlCd\x120\n\x15pre_emi_overdue_gl_cd\x18F\x20\x01(\x03R\x11preEmiO\
    verdueGlCd\x12'\n\x10excess_emi_gl_cd\x18G\x20\x01(\x03R\rexcessEmiGlCd\
    \x12.\n\x14excess_pre_emi_gl_cd\x18H\x20\x01(\x03R\x10excessPreEmiGlCd\
    \x12\x20\n\x0ctot_prin_amt\x18I\x20\x01(\x01R\ntotPrinAmt\x12\x1e\n\x0bt\
    ot_int_amt\x18J\x20\x01(\x01R\ttotIntAmt\x12\x17\n\x07balm_l2\x18K\x20\
    \x01(\tR\x06balmL2\x12*\n\x11derived_npa_class\x18L\x20\x01(\tR\x0fderiv\
    edNpaClass\x12$\n\x0ecommon_cust_id\x18M\x20\x01(\tR\x0ccommonCustId\x12\
    9\n\x19derived_next_reprice_date\x18N\x20\x01(\x03R\x16derivedNextRepric\
    eDate\x12.\n\x13derived_risk_weight\x18O\x20\x01(\tR\x11derivedRiskWeigh\
    t\x12)\n\x10restructure_flag\x18P\x20\x01(\tR\x0frestructureFlag\x12\x14\
    \n\x05resid\x18Q\x20\x01(\tR\x05resid\x12\x17\n\x07ia_line\x18R\x20\x01(\
    \tR\x06iaLine\x12\x19\n\x08sma_flag\x18S\x20\x01(\tR\x07smaFlag\x12'\n\t\
    cashflows\x18T\x20\x03(\x0b2\t.CashflowR\tcashflowsb\x06proto3\
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
