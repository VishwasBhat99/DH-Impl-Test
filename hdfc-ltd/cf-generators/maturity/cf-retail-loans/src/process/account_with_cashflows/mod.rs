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
pub struct Account {
    // message fields
    pub account_number: ::std::string::String,
    pub disbursed_amt: f64,
    pub os_loan_bal_local_currency: f64,
    pub curr_applicable_interest_rate: f64,
    pub ei_amount_current: f64,
    pub interest_type: ::std::string::String,
    pub os_p_bal_due_local_currency: f64,
    pub os_i_bal_due_local_currency: f64,
    pub ei_amt_paid_advance_local_curr: f64,
    pub pre_ei_bal_local_curr: f64,
    pub account_open_value_date: i64,
    pub maturity_date: i64,
    pub ei_start_date_current: i64,
    pub ei_end_date_current: i64,
    pub ei_payment_frequency_current: ::std::string::String,
    pub emi_last_paid_date_current: i64,
    pub ei_payment_day: ::std::string::String,
    pub ei_orginal_term: ::std::string::String,
    pub ei_balance_term: ::std::string::String,
    pub repricing_benchmark: ::std::string::String,
    pub spread: ::std::string::String,
    pub last_repricing_date: i64,
    pub next_repricing_date: i64,
    pub repricing_frequency: ::std::string::String,
    pub number_ei_structures: i64,
    pub npa_classification: ::std::string::String,
    pub remark: ::std::string::String,
    pub months_os_comb: ::std::string::String,
    pub moratorium_type: ::std::string::String,
    pub from_moratorium_date: i64,
    pub to_moratorium_date: i64,
    pub recalculate_ei_amount_flag: ::std::string::String,
    pub moratorium_interest_calculation: ::std::string::String,
    pub bullet_payment_flag: ::std::string::String,
    pub restructured_flag: ::std::string::String,
    pub residential_mortgage: ::std::string::String,
    pub risk_weight: ::std::string::String,
    pub internal_rating: ::std::string::String,
    pub external_rating: ::std::string::String,
    pub contractual_tenor: ::std::string::String,
    pub residual_tenor: ::std::string::String,
    pub customer_constitution_code: ::std::string::String,
    pub product_code: ::std::string::String,
    pub p_gl_code: ::std::string::String,
    pub m_npa_classification: ::std::string::String,
    pub accrued_interest: ::std::string::String,
    pub customer_id: ::std::string::String,
    pub customer_name: ::std::string::String,
    pub group_id: ::std::string::String,
    pub group_name: ::std::string::String,
    pub branch_code: ::std::string::String,
    pub sector: ::std::string::String,
    pub industry: ::std::string::String,
    pub ltv: ::std::string::String,
    pub overdue_account: ::std::string::String,
    pub excess_account: ::std::string::String,
    pub loan_type: ::std::string::String,
    pub residual_interest: ::std::string::String,
    pub currency: ::std::string::String,
    pub hdfc_ltd_percentage: f64,
    pub securitization_percentage: f64,
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
    pub sma_flag: ::std::string::String,
    pub cashflows: ::protobuf::RepeatedField<Cashflow>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Account {
    fn default() -> &'a Account {
        <Account as ::protobuf::Message>::default_instance()
    }
}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    // string account_number = 1;


    pub fn get_account_number(&self) -> &str {
        &self.account_number
    }
    pub fn clear_account_number(&mut self) {
        self.account_number.clear();
    }

    // Param is passed by value, moved
    pub fn set_account_number(&mut self, v: ::std::string::String) {
        self.account_number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_number(&mut self) -> &mut ::std::string::String {
        &mut self.account_number
    }

    // Take field
    pub fn take_account_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.account_number, ::std::string::String::new())
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

    // double os_loan_bal_local_currency = 3;


    pub fn get_os_loan_bal_local_currency(&self) -> f64 {
        self.os_loan_bal_local_currency
    }
    pub fn clear_os_loan_bal_local_currency(&mut self) {
        self.os_loan_bal_local_currency = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_loan_bal_local_currency(&mut self, v: f64) {
        self.os_loan_bal_local_currency = v;
    }

    // double curr_applicable_interest_rate = 4;


    pub fn get_curr_applicable_interest_rate(&self) -> f64 {
        self.curr_applicable_interest_rate
    }
    pub fn clear_curr_applicable_interest_rate(&mut self) {
        self.curr_applicable_interest_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_curr_applicable_interest_rate(&mut self, v: f64) {
        self.curr_applicable_interest_rate = v;
    }

    // double ei_amount_current = 5;


    pub fn get_ei_amount_current(&self) -> f64 {
        self.ei_amount_current
    }
    pub fn clear_ei_amount_current(&mut self) {
        self.ei_amount_current = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ei_amount_current(&mut self, v: f64) {
        self.ei_amount_current = v;
    }

    // string interest_type = 6;


    pub fn get_interest_type(&self) -> &str {
        &self.interest_type
    }
    pub fn clear_interest_type(&mut self) {
        self.interest_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_interest_type(&mut self, v: ::std::string::String) {
        self.interest_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interest_type(&mut self) -> &mut ::std::string::String {
        &mut self.interest_type
    }

    // Take field
    pub fn take_interest_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.interest_type, ::std::string::String::new())
    }

    // double os_p_bal_due_local_currency = 7;


    pub fn get_os_p_bal_due_local_currency(&self) -> f64 {
        self.os_p_bal_due_local_currency
    }
    pub fn clear_os_p_bal_due_local_currency(&mut self) {
        self.os_p_bal_due_local_currency = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_p_bal_due_local_currency(&mut self, v: f64) {
        self.os_p_bal_due_local_currency = v;
    }

    // double os_i_bal_due_local_currency = 8;


    pub fn get_os_i_bal_due_local_currency(&self) -> f64 {
        self.os_i_bal_due_local_currency
    }
    pub fn clear_os_i_bal_due_local_currency(&mut self) {
        self.os_i_bal_due_local_currency = 0.;
    }

    // Param is passed by value, moved
    pub fn set_os_i_bal_due_local_currency(&mut self, v: f64) {
        self.os_i_bal_due_local_currency = v;
    }

    // double ei_amt_paid_advance_local_curr = 9;


    pub fn get_ei_amt_paid_advance_local_curr(&self) -> f64 {
        self.ei_amt_paid_advance_local_curr
    }
    pub fn clear_ei_amt_paid_advance_local_curr(&mut self) {
        self.ei_amt_paid_advance_local_curr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ei_amt_paid_advance_local_curr(&mut self, v: f64) {
        self.ei_amt_paid_advance_local_curr = v;
    }

    // double pre_ei_bal_local_curr = 10;


    pub fn get_pre_ei_bal_local_curr(&self) -> f64 {
        self.pre_ei_bal_local_curr
    }
    pub fn clear_pre_ei_bal_local_curr(&mut self) {
        self.pre_ei_bal_local_curr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_pre_ei_bal_local_curr(&mut self, v: f64) {
        self.pre_ei_bal_local_curr = v;
    }

    // int64 account_open_value_date = 11;


    pub fn get_account_open_value_date(&self) -> i64 {
        self.account_open_value_date
    }
    pub fn clear_account_open_value_date(&mut self) {
        self.account_open_value_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_account_open_value_date(&mut self, v: i64) {
        self.account_open_value_date = v;
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

    // int64 ei_start_date_current = 13;


    pub fn get_ei_start_date_current(&self) -> i64 {
        self.ei_start_date_current
    }
    pub fn clear_ei_start_date_current(&mut self) {
        self.ei_start_date_current = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_start_date_current(&mut self, v: i64) {
        self.ei_start_date_current = v;
    }

    // int64 ei_end_date_current = 14;


    pub fn get_ei_end_date_current(&self) -> i64 {
        self.ei_end_date_current
    }
    pub fn clear_ei_end_date_current(&mut self) {
        self.ei_end_date_current = 0;
    }

    // Param is passed by value, moved
    pub fn set_ei_end_date_current(&mut self, v: i64) {
        self.ei_end_date_current = v;
    }

    // string ei_payment_frequency_current = 15;


    pub fn get_ei_payment_frequency_current(&self) -> &str {
        &self.ei_payment_frequency_current
    }
    pub fn clear_ei_payment_frequency_current(&mut self) {
        self.ei_payment_frequency_current.clear();
    }

    // Param is passed by value, moved
    pub fn set_ei_payment_frequency_current(&mut self, v: ::std::string::String) {
        self.ei_payment_frequency_current = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ei_payment_frequency_current(&mut self) -> &mut ::std::string::String {
        &mut self.ei_payment_frequency_current
    }

    // Take field
    pub fn take_ei_payment_frequency_current(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ei_payment_frequency_current, ::std::string::String::new())
    }

    // int64 emi_last_paid_date_current = 16;


    pub fn get_emi_last_paid_date_current(&self) -> i64 {
        self.emi_last_paid_date_current
    }
    pub fn clear_emi_last_paid_date_current(&mut self) {
        self.emi_last_paid_date_current = 0;
    }

    // Param is passed by value, moved
    pub fn set_emi_last_paid_date_current(&mut self, v: i64) {
        self.emi_last_paid_date_current = v;
    }

    // string ei_payment_day = 17;


    pub fn get_ei_payment_day(&self) -> &str {
        &self.ei_payment_day
    }
    pub fn clear_ei_payment_day(&mut self) {
        self.ei_payment_day.clear();
    }

    // Param is passed by value, moved
    pub fn set_ei_payment_day(&mut self, v: ::std::string::String) {
        self.ei_payment_day = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ei_payment_day(&mut self) -> &mut ::std::string::String {
        &mut self.ei_payment_day
    }

    // Take field
    pub fn take_ei_payment_day(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ei_payment_day, ::std::string::String::new())
    }

    // string ei_orginal_term = 18;


    pub fn get_ei_orginal_term(&self) -> &str {
        &self.ei_orginal_term
    }
    pub fn clear_ei_orginal_term(&mut self) {
        self.ei_orginal_term.clear();
    }

    // Param is passed by value, moved
    pub fn set_ei_orginal_term(&mut self, v: ::std::string::String) {
        self.ei_orginal_term = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ei_orginal_term(&mut self) -> &mut ::std::string::String {
        &mut self.ei_orginal_term
    }

    // Take field
    pub fn take_ei_orginal_term(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ei_orginal_term, ::std::string::String::new())
    }

    // string ei_balance_term = 19;


    pub fn get_ei_balance_term(&self) -> &str {
        &self.ei_balance_term
    }
    pub fn clear_ei_balance_term(&mut self) {
        self.ei_balance_term.clear();
    }

    // Param is passed by value, moved
    pub fn set_ei_balance_term(&mut self, v: ::std::string::String) {
        self.ei_balance_term = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ei_balance_term(&mut self) -> &mut ::std::string::String {
        &mut self.ei_balance_term
    }

    // Take field
    pub fn take_ei_balance_term(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ei_balance_term, ::std::string::String::new())
    }

    // string repricing_benchmark = 20;


    pub fn get_repricing_benchmark(&self) -> &str {
        &self.repricing_benchmark
    }
    pub fn clear_repricing_benchmark(&mut self) {
        self.repricing_benchmark.clear();
    }

    // Param is passed by value, moved
    pub fn set_repricing_benchmark(&mut self, v: ::std::string::String) {
        self.repricing_benchmark = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repricing_benchmark(&mut self) -> &mut ::std::string::String {
        &mut self.repricing_benchmark
    }

    // Take field
    pub fn take_repricing_benchmark(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.repricing_benchmark, ::std::string::String::new())
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

    // int64 last_repricing_date = 22;


    pub fn get_last_repricing_date(&self) -> i64 {
        self.last_repricing_date
    }
    pub fn clear_last_repricing_date(&mut self) {
        self.last_repricing_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_repricing_date(&mut self, v: i64) {
        self.last_repricing_date = v;
    }

    // int64 next_repricing_date = 23;


    pub fn get_next_repricing_date(&self) -> i64 {
        self.next_repricing_date
    }
    pub fn clear_next_repricing_date(&mut self) {
        self.next_repricing_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_next_repricing_date(&mut self, v: i64) {
        self.next_repricing_date = v;
    }

    // string repricing_frequency = 24;


    pub fn get_repricing_frequency(&self) -> &str {
        &self.repricing_frequency
    }
    pub fn clear_repricing_frequency(&mut self) {
        self.repricing_frequency.clear();
    }

    // Param is passed by value, moved
    pub fn set_repricing_frequency(&mut self, v: ::std::string::String) {
        self.repricing_frequency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repricing_frequency(&mut self) -> &mut ::std::string::String {
        &mut self.repricing_frequency
    }

    // Take field
    pub fn take_repricing_frequency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.repricing_frequency, ::std::string::String::new())
    }

    // int64 number_ei_structures = 25;


    pub fn get_number_ei_structures(&self) -> i64 {
        self.number_ei_structures
    }
    pub fn clear_number_ei_structures(&mut self) {
        self.number_ei_structures = 0;
    }

    // Param is passed by value, moved
    pub fn set_number_ei_structures(&mut self, v: i64) {
        self.number_ei_structures = v;
    }

    // string npa_classification = 26;


    pub fn get_npa_classification(&self) -> &str {
        &self.npa_classification
    }
    pub fn clear_npa_classification(&mut self) {
        self.npa_classification.clear();
    }

    // Param is passed by value, moved
    pub fn set_npa_classification(&mut self, v: ::std::string::String) {
        self.npa_classification = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_npa_classification(&mut self) -> &mut ::std::string::String {
        &mut self.npa_classification
    }

    // Take field
    pub fn take_npa_classification(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.npa_classification, ::std::string::String::new())
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

    // string moratorium_type = 29;


    pub fn get_moratorium_type(&self) -> &str {
        &self.moratorium_type
    }
    pub fn clear_moratorium_type(&mut self) {
        self.moratorium_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_moratorium_type(&mut self, v: ::std::string::String) {
        self.moratorium_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_moratorium_type(&mut self) -> &mut ::std::string::String {
        &mut self.moratorium_type
    }

    // Take field
    pub fn take_moratorium_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.moratorium_type, ::std::string::String::new())
    }

    // int64 from_moratorium_date = 30;


    pub fn get_from_moratorium_date(&self) -> i64 {
        self.from_moratorium_date
    }
    pub fn clear_from_moratorium_date(&mut self) {
        self.from_moratorium_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_from_moratorium_date(&mut self, v: i64) {
        self.from_moratorium_date = v;
    }

    // int64 to_moratorium_date = 31;


    pub fn get_to_moratorium_date(&self) -> i64 {
        self.to_moratorium_date
    }
    pub fn clear_to_moratorium_date(&mut self) {
        self.to_moratorium_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_to_moratorium_date(&mut self, v: i64) {
        self.to_moratorium_date = v;
    }

    // string recalculate_ei_amount_flag = 32;


    pub fn get_recalculate_ei_amount_flag(&self) -> &str {
        &self.recalculate_ei_amount_flag
    }
    pub fn clear_recalculate_ei_amount_flag(&mut self) {
        self.recalculate_ei_amount_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_recalculate_ei_amount_flag(&mut self, v: ::std::string::String) {
        self.recalculate_ei_amount_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_recalculate_ei_amount_flag(&mut self) -> &mut ::std::string::String {
        &mut self.recalculate_ei_amount_flag
    }

    // Take field
    pub fn take_recalculate_ei_amount_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.recalculate_ei_amount_flag, ::std::string::String::new())
    }

    // string moratorium_interest_calculation = 33;


    pub fn get_moratorium_interest_calculation(&self) -> &str {
        &self.moratorium_interest_calculation
    }
    pub fn clear_moratorium_interest_calculation(&mut self) {
        self.moratorium_interest_calculation.clear();
    }

    // Param is passed by value, moved
    pub fn set_moratorium_interest_calculation(&mut self, v: ::std::string::String) {
        self.moratorium_interest_calculation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_moratorium_interest_calculation(&mut self) -> &mut ::std::string::String {
        &mut self.moratorium_interest_calculation
    }

    // Take field
    pub fn take_moratorium_interest_calculation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.moratorium_interest_calculation, ::std::string::String::new())
    }

    // string bullet_payment_flag = 34;


    pub fn get_bullet_payment_flag(&self) -> &str {
        &self.bullet_payment_flag
    }
    pub fn clear_bullet_payment_flag(&mut self) {
        self.bullet_payment_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_bullet_payment_flag(&mut self, v: ::std::string::String) {
        self.bullet_payment_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bullet_payment_flag(&mut self) -> &mut ::std::string::String {
        &mut self.bullet_payment_flag
    }

    // Take field
    pub fn take_bullet_payment_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.bullet_payment_flag, ::std::string::String::new())
    }

    // string restructured_flag = 35;


    pub fn get_restructured_flag(&self) -> &str {
        &self.restructured_flag
    }
    pub fn clear_restructured_flag(&mut self) {
        self.restructured_flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_restructured_flag(&mut self, v: ::std::string::String) {
        self.restructured_flag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_restructured_flag(&mut self) -> &mut ::std::string::String {
        &mut self.restructured_flag
    }

    // Take field
    pub fn take_restructured_flag(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.restructured_flag, ::std::string::String::new())
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

    // string contractual_tenor = 40;


    pub fn get_contractual_tenor(&self) -> &str {
        &self.contractual_tenor
    }
    pub fn clear_contractual_tenor(&mut self) {
        self.contractual_tenor.clear();
    }

    // Param is passed by value, moved
    pub fn set_contractual_tenor(&mut self, v: ::std::string::String) {
        self.contractual_tenor = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contractual_tenor(&mut self) -> &mut ::std::string::String {
        &mut self.contractual_tenor
    }

    // Take field
    pub fn take_contractual_tenor(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.contractual_tenor, ::std::string::String::new())
    }

    // string residual_tenor = 41;


    pub fn get_residual_tenor(&self) -> &str {
        &self.residual_tenor
    }
    pub fn clear_residual_tenor(&mut self) {
        self.residual_tenor.clear();
    }

    // Param is passed by value, moved
    pub fn set_residual_tenor(&mut self, v: ::std::string::String) {
        self.residual_tenor = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_residual_tenor(&mut self) -> &mut ::std::string::String {
        &mut self.residual_tenor
    }

    // Take field
    pub fn take_residual_tenor(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.residual_tenor, ::std::string::String::new())
    }

    // string customer_constitution_code = 42;


    pub fn get_customer_constitution_code(&self) -> &str {
        &self.customer_constitution_code
    }
    pub fn clear_customer_constitution_code(&mut self) {
        self.customer_constitution_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_customer_constitution_code(&mut self, v: ::std::string::String) {
        self.customer_constitution_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_customer_constitution_code(&mut self) -> &mut ::std::string::String {
        &mut self.customer_constitution_code
    }

    // Take field
    pub fn take_customer_constitution_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.customer_constitution_code, ::std::string::String::new())
    }

    // string product_code = 43;


    pub fn get_product_code(&self) -> &str {
        &self.product_code
    }
    pub fn clear_product_code(&mut self) {
        self.product_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_code(&mut self, v: ::std::string::String) {
        self.product_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product_code(&mut self) -> &mut ::std::string::String {
        &mut self.product_code
    }

    // Take field
    pub fn take_product_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.product_code, ::std::string::String::new())
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

    // string m_npa_classification = 45;


    pub fn get_m_npa_classification(&self) -> &str {
        &self.m_npa_classification
    }
    pub fn clear_m_npa_classification(&mut self) {
        self.m_npa_classification.clear();
    }

    // Param is passed by value, moved
    pub fn set_m_npa_classification(&mut self, v: ::std::string::String) {
        self.m_npa_classification = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_m_npa_classification(&mut self) -> &mut ::std::string::String {
        &mut self.m_npa_classification
    }

    // Take field
    pub fn take_m_npa_classification(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.m_npa_classification, ::std::string::String::new())
    }

    // string accrued_interest = 46;


    pub fn get_accrued_interest(&self) -> &str {
        &self.accrued_interest
    }
    pub fn clear_accrued_interest(&mut self) {
        self.accrued_interest.clear();
    }

    // Param is passed by value, moved
    pub fn set_accrued_interest(&mut self, v: ::std::string::String) {
        self.accrued_interest = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accrued_interest(&mut self) -> &mut ::std::string::String {
        &mut self.accrued_interest
    }

    // Take field
    pub fn take_accrued_interest(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.accrued_interest, ::std::string::String::new())
    }

    // string customer_id = 47;


    pub fn get_customer_id(&self) -> &str {
        &self.customer_id
    }
    pub fn clear_customer_id(&mut self) {
        self.customer_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_customer_id(&mut self, v: ::std::string::String) {
        self.customer_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_customer_id(&mut self) -> &mut ::std::string::String {
        &mut self.customer_id
    }

    // Take field
    pub fn take_customer_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.customer_id, ::std::string::String::new())
    }

    // string customer_name = 48;


    pub fn get_customer_name(&self) -> &str {
        &self.customer_name
    }
    pub fn clear_customer_name(&mut self) {
        self.customer_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_customer_name(&mut self, v: ::std::string::String) {
        self.customer_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_customer_name(&mut self) -> &mut ::std::string::String {
        &mut self.customer_name
    }

    // Take field
    pub fn take_customer_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.customer_name, ::std::string::String::new())
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

    // string overdue_account = 55;


    pub fn get_overdue_account(&self) -> &str {
        &self.overdue_account
    }
    pub fn clear_overdue_account(&mut self) {
        self.overdue_account.clear();
    }

    // Param is passed by value, moved
    pub fn set_overdue_account(&mut self, v: ::std::string::String) {
        self.overdue_account = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_overdue_account(&mut self) -> &mut ::std::string::String {
        &mut self.overdue_account
    }

    // Take field
    pub fn take_overdue_account(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.overdue_account, ::std::string::String::new())
    }

    // string excess_account = 56;


    pub fn get_excess_account(&self) -> &str {
        &self.excess_account
    }
    pub fn clear_excess_account(&mut self) {
        self.excess_account.clear();
    }

    // Param is passed by value, moved
    pub fn set_excess_account(&mut self, v: ::std::string::String) {
        self.excess_account = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_excess_account(&mut self) -> &mut ::std::string::String {
        &mut self.excess_account
    }

    // Take field
    pub fn take_excess_account(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.excess_account, ::std::string::String::new())
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

    // string residual_interest = 58;


    pub fn get_residual_interest(&self) -> &str {
        &self.residual_interest
    }
    pub fn clear_residual_interest(&mut self) {
        self.residual_interest.clear();
    }

    // Param is passed by value, moved
    pub fn set_residual_interest(&mut self, v: ::std::string::String) {
        self.residual_interest = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_residual_interest(&mut self) -> &mut ::std::string::String {
        &mut self.residual_interest
    }

    // Take field
    pub fn take_residual_interest(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.residual_interest, ::std::string::String::new())
    }

    // string currency = 59;


    pub fn get_currency(&self) -> &str {
        &self.currency
    }
    pub fn clear_currency(&mut self) {
        self.currency.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency(&mut self) -> &mut ::std::string::String {
        &mut self.currency
    }

    // Take field
    pub fn take_currency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency, ::std::string::String::new())
    }

    // double hdfc_ltd_percentage = 60;


    pub fn get_hdfc_ltd_percentage(&self) -> f64 {
        self.hdfc_ltd_percentage
    }
    pub fn clear_hdfc_ltd_percentage(&mut self) {
        self.hdfc_ltd_percentage = 0.;
    }

    // Param is passed by value, moved
    pub fn set_hdfc_ltd_percentage(&mut self, v: f64) {
        self.hdfc_ltd_percentage = v;
    }

    // double securitization_percentage = 61;


    pub fn get_securitization_percentage(&self) -> f64 {
        self.securitization_percentage
    }
    pub fn clear_securitization_percentage(&mut self) {
        self.securitization_percentage = 0.;
    }

    // Param is passed by value, moved
    pub fn set_securitization_percentage(&mut self, v: f64) {
        self.securitization_percentage = v;
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

    // string sma_flag = 75;


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

    // repeated .Cashflow cashflows = 76;


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

impl ::protobuf::Message for Account {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.account_number)?;
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
                    self.os_loan_bal_local_currency = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.curr_applicable_interest_rate = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ei_amount_current = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.interest_type)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_p_bal_due_local_currency = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.os_i_bal_due_local_currency = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ei_amt_paid_advance_local_curr = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.pre_ei_bal_local_curr = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.account_open_value_date = tmp;
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
                    self.ei_start_date_current = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ei_end_date_current = tmp;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ei_payment_frequency_current)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.emi_last_paid_date_current = tmp;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ei_payment_day)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ei_orginal_term)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ei_balance_term)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.repricing_benchmark)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.spread)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_repricing_date = tmp;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.next_repricing_date = tmp;
                },
                24 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.repricing_frequency)?;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.number_ei_structures = tmp;
                },
                26 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.npa_classification)?;
                },
                27 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remark)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.months_os_comb)?;
                },
                29 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.moratorium_type)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.from_moratorium_date = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.to_moratorium_date = tmp;
                },
                32 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.recalculate_ei_amount_flag)?;
                },
                33 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.moratorium_interest_calculation)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.bullet_payment_flag)?;
                },
                35 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.restructured_flag)?;
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.contractual_tenor)?;
                },
                41 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.residual_tenor)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.customer_constitution_code)?;
                },
                43 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.product_code)?;
                },
                44 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.p_gl_code)?;
                },
                45 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.m_npa_classification)?;
                },
                46 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.accrued_interest)?;
                },
                47 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.customer_id)?;
                },
                48 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.customer_name)?;
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.overdue_account)?;
                },
                56 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.excess_account)?;
                },
                57 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loan_type)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.residual_interest)?;
                },
                59 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.hdfc_ltd_percentage = tmp;
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.securitization_percentage = tmp;
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sma_flag)?;
                },
                76 => {
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
        if !self.account_number.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.account_number);
        }
        if self.disbursed_amt != 0. {
            my_size += 9;
        }
        if self.os_loan_bal_local_currency != 0. {
            my_size += 9;
        }
        if self.curr_applicable_interest_rate != 0. {
            my_size += 9;
        }
        if self.ei_amount_current != 0. {
            my_size += 9;
        }
        if !self.interest_type.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.interest_type);
        }
        if self.os_p_bal_due_local_currency != 0. {
            my_size += 9;
        }
        if self.os_i_bal_due_local_currency != 0. {
            my_size += 9;
        }
        if self.ei_amt_paid_advance_local_curr != 0. {
            my_size += 9;
        }
        if self.pre_ei_bal_local_curr != 0. {
            my_size += 9;
        }
        if self.account_open_value_date != 0 {
            my_size += ::protobuf::rt::value_size(11, self.account_open_value_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.maturity_date != 0 {
            my_size += ::protobuf::rt::value_size(12, self.maturity_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_start_date_current != 0 {
            my_size += ::protobuf::rt::value_size(13, self.ei_start_date_current, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ei_end_date_current != 0 {
            my_size += ::protobuf::rt::value_size(14, self.ei_end_date_current, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ei_payment_frequency_current.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.ei_payment_frequency_current);
        }
        if self.emi_last_paid_date_current != 0 {
            my_size += ::protobuf::rt::value_size(16, self.emi_last_paid_date_current, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ei_payment_day.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.ei_payment_day);
        }
        if !self.ei_orginal_term.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.ei_orginal_term);
        }
        if !self.ei_balance_term.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.ei_balance_term);
        }
        if !self.repricing_benchmark.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.repricing_benchmark);
        }
        if !self.spread.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.spread);
        }
        if self.last_repricing_date != 0 {
            my_size += ::protobuf::rt::value_size(22, self.last_repricing_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.next_repricing_date != 0 {
            my_size += ::protobuf::rt::value_size(23, self.next_repricing_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.repricing_frequency.is_empty() {
            my_size += ::protobuf::rt::string_size(24, &self.repricing_frequency);
        }
        if self.number_ei_structures != 0 {
            my_size += ::protobuf::rt::value_size(25, self.number_ei_structures, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.npa_classification.is_empty() {
            my_size += ::protobuf::rt::string_size(26, &self.npa_classification);
        }
        if !self.remark.is_empty() {
            my_size += ::protobuf::rt::string_size(27, &self.remark);
        }
        if !self.months_os_comb.is_empty() {
            my_size += ::protobuf::rt::string_size(28, &self.months_os_comb);
        }
        if !self.moratorium_type.is_empty() {
            my_size += ::protobuf::rt::string_size(29, &self.moratorium_type);
        }
        if self.from_moratorium_date != 0 {
            my_size += ::protobuf::rt::value_size(30, self.from_moratorium_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.to_moratorium_date != 0 {
            my_size += ::protobuf::rt::value_size(31, self.to_moratorium_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.recalculate_ei_amount_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(32, &self.recalculate_ei_amount_flag);
        }
        if !self.moratorium_interest_calculation.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.moratorium_interest_calculation);
        }
        if !self.bullet_payment_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.bullet_payment_flag);
        }
        if !self.restructured_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(35, &self.restructured_flag);
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
        if !self.contractual_tenor.is_empty() {
            my_size += ::protobuf::rt::string_size(40, &self.contractual_tenor);
        }
        if !self.residual_tenor.is_empty() {
            my_size += ::protobuf::rt::string_size(41, &self.residual_tenor);
        }
        if !self.customer_constitution_code.is_empty() {
            my_size += ::protobuf::rt::string_size(42, &self.customer_constitution_code);
        }
        if !self.product_code.is_empty() {
            my_size += ::protobuf::rt::string_size(43, &self.product_code);
        }
        if !self.p_gl_code.is_empty() {
            my_size += ::protobuf::rt::string_size(44, &self.p_gl_code);
        }
        if !self.m_npa_classification.is_empty() {
            my_size += ::protobuf::rt::string_size(45, &self.m_npa_classification);
        }
        if !self.accrued_interest.is_empty() {
            my_size += ::protobuf::rt::string_size(46, &self.accrued_interest);
        }
        if !self.customer_id.is_empty() {
            my_size += ::protobuf::rt::string_size(47, &self.customer_id);
        }
        if !self.customer_name.is_empty() {
            my_size += ::protobuf::rt::string_size(48, &self.customer_name);
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
        if !self.overdue_account.is_empty() {
            my_size += ::protobuf::rt::string_size(55, &self.overdue_account);
        }
        if !self.excess_account.is_empty() {
            my_size += ::protobuf::rt::string_size(56, &self.excess_account);
        }
        if !self.loan_type.is_empty() {
            my_size += ::protobuf::rt::string_size(57, &self.loan_type);
        }
        if !self.residual_interest.is_empty() {
            my_size += ::protobuf::rt::string_size(58, &self.residual_interest);
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(59, &self.currency);
        }
        if self.hdfc_ltd_percentage != 0. {
            my_size += 10;
        }
        if self.securitization_percentage != 0. {
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
        if !self.sma_flag.is_empty() {
            my_size += ::protobuf::rt::string_size(75, &self.sma_flag);
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
        if !self.account_number.is_empty() {
            os.write_string(1, &self.account_number)?;
        }
        if self.disbursed_amt != 0. {
            os.write_double(2, self.disbursed_amt)?;
        }
        if self.os_loan_bal_local_currency != 0. {
            os.write_double(3, self.os_loan_bal_local_currency)?;
        }
        if self.curr_applicable_interest_rate != 0. {
            os.write_double(4, self.curr_applicable_interest_rate)?;
        }
        if self.ei_amount_current != 0. {
            os.write_double(5, self.ei_amount_current)?;
        }
        if !self.interest_type.is_empty() {
            os.write_string(6, &self.interest_type)?;
        }
        if self.os_p_bal_due_local_currency != 0. {
            os.write_double(7, self.os_p_bal_due_local_currency)?;
        }
        if self.os_i_bal_due_local_currency != 0. {
            os.write_double(8, self.os_i_bal_due_local_currency)?;
        }
        if self.ei_amt_paid_advance_local_curr != 0. {
            os.write_double(9, self.ei_amt_paid_advance_local_curr)?;
        }
        if self.pre_ei_bal_local_curr != 0. {
            os.write_double(10, self.pre_ei_bal_local_curr)?;
        }
        if self.account_open_value_date != 0 {
            os.write_int64(11, self.account_open_value_date)?;
        }
        if self.maturity_date != 0 {
            os.write_int64(12, self.maturity_date)?;
        }
        if self.ei_start_date_current != 0 {
            os.write_int64(13, self.ei_start_date_current)?;
        }
        if self.ei_end_date_current != 0 {
            os.write_int64(14, self.ei_end_date_current)?;
        }
        if !self.ei_payment_frequency_current.is_empty() {
            os.write_string(15, &self.ei_payment_frequency_current)?;
        }
        if self.emi_last_paid_date_current != 0 {
            os.write_int64(16, self.emi_last_paid_date_current)?;
        }
        if !self.ei_payment_day.is_empty() {
            os.write_string(17, &self.ei_payment_day)?;
        }
        if !self.ei_orginal_term.is_empty() {
            os.write_string(18, &self.ei_orginal_term)?;
        }
        if !self.ei_balance_term.is_empty() {
            os.write_string(19, &self.ei_balance_term)?;
        }
        if !self.repricing_benchmark.is_empty() {
            os.write_string(20, &self.repricing_benchmark)?;
        }
        if !self.spread.is_empty() {
            os.write_string(21, &self.spread)?;
        }
        if self.last_repricing_date != 0 {
            os.write_int64(22, self.last_repricing_date)?;
        }
        if self.next_repricing_date != 0 {
            os.write_int64(23, self.next_repricing_date)?;
        }
        if !self.repricing_frequency.is_empty() {
            os.write_string(24, &self.repricing_frequency)?;
        }
        if self.number_ei_structures != 0 {
            os.write_int64(25, self.number_ei_structures)?;
        }
        if !self.npa_classification.is_empty() {
            os.write_string(26, &self.npa_classification)?;
        }
        if !self.remark.is_empty() {
            os.write_string(27, &self.remark)?;
        }
        if !self.months_os_comb.is_empty() {
            os.write_string(28, &self.months_os_comb)?;
        }
        if !self.moratorium_type.is_empty() {
            os.write_string(29, &self.moratorium_type)?;
        }
        if self.from_moratorium_date != 0 {
            os.write_int64(30, self.from_moratorium_date)?;
        }
        if self.to_moratorium_date != 0 {
            os.write_int64(31, self.to_moratorium_date)?;
        }
        if !self.recalculate_ei_amount_flag.is_empty() {
            os.write_string(32, &self.recalculate_ei_amount_flag)?;
        }
        if !self.moratorium_interest_calculation.is_empty() {
            os.write_string(33, &self.moratorium_interest_calculation)?;
        }
        if !self.bullet_payment_flag.is_empty() {
            os.write_string(34, &self.bullet_payment_flag)?;
        }
        if !self.restructured_flag.is_empty() {
            os.write_string(35, &self.restructured_flag)?;
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
        if !self.contractual_tenor.is_empty() {
            os.write_string(40, &self.contractual_tenor)?;
        }
        if !self.residual_tenor.is_empty() {
            os.write_string(41, &self.residual_tenor)?;
        }
        if !self.customer_constitution_code.is_empty() {
            os.write_string(42, &self.customer_constitution_code)?;
        }
        if !self.product_code.is_empty() {
            os.write_string(43, &self.product_code)?;
        }
        if !self.p_gl_code.is_empty() {
            os.write_string(44, &self.p_gl_code)?;
        }
        if !self.m_npa_classification.is_empty() {
            os.write_string(45, &self.m_npa_classification)?;
        }
        if !self.accrued_interest.is_empty() {
            os.write_string(46, &self.accrued_interest)?;
        }
        if !self.customer_id.is_empty() {
            os.write_string(47, &self.customer_id)?;
        }
        if !self.customer_name.is_empty() {
            os.write_string(48, &self.customer_name)?;
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
        if !self.overdue_account.is_empty() {
            os.write_string(55, &self.overdue_account)?;
        }
        if !self.excess_account.is_empty() {
            os.write_string(56, &self.excess_account)?;
        }
        if !self.loan_type.is_empty() {
            os.write_string(57, &self.loan_type)?;
        }
        if !self.residual_interest.is_empty() {
            os.write_string(58, &self.residual_interest)?;
        }
        if !self.currency.is_empty() {
            os.write_string(59, &self.currency)?;
        }
        if self.hdfc_ltd_percentage != 0. {
            os.write_double(60, self.hdfc_ltd_percentage)?;
        }
        if self.securitization_percentage != 0. {
            os.write_double(61, self.securitization_percentage)?;
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
        if !self.sma_flag.is_empty() {
            os.write_string(75, &self.sma_flag)?;
        }
        for v in &self.cashflows {
            os.write_tag(76, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "account_number",
                |m: &Account| { &m.account_number },
                |m: &mut Account| { &mut m.account_number },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "disbursed_amt",
                |m: &Account| { &m.disbursed_amt },
                |m: &mut Account| { &mut m.disbursed_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_loan_bal_local_currency",
                |m: &Account| { &m.os_loan_bal_local_currency },
                |m: &mut Account| { &mut m.os_loan_bal_local_currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "curr_applicable_interest_rate",
                |m: &Account| { &m.curr_applicable_interest_rate },
                |m: &mut Account| { &mut m.curr_applicable_interest_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ei_amount_current",
                |m: &Account| { &m.ei_amount_current },
                |m: &mut Account| { &mut m.ei_amount_current },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "interest_type",
                |m: &Account| { &m.interest_type },
                |m: &mut Account| { &mut m.interest_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_p_bal_due_local_currency",
                |m: &Account| { &m.os_p_bal_due_local_currency },
                |m: &mut Account| { &mut m.os_p_bal_due_local_currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "os_i_bal_due_local_currency",
                |m: &Account| { &m.os_i_bal_due_local_currency },
                |m: &mut Account| { &mut m.os_i_bal_due_local_currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ei_amt_paid_advance_local_curr",
                |m: &Account| { &m.ei_amt_paid_advance_local_curr },
                |m: &mut Account| { &mut m.ei_amt_paid_advance_local_curr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "pre_ei_bal_local_curr",
                |m: &Account| { &m.pre_ei_bal_local_curr },
                |m: &mut Account| { &mut m.pre_ei_bal_local_curr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "account_open_value_date",
                |m: &Account| { &m.account_open_value_date },
                |m: &mut Account| { &mut m.account_open_value_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "maturity_date",
                |m: &Account| { &m.maturity_date },
                |m: &mut Account| { &mut m.maturity_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_start_date_current",
                |m: &Account| { &m.ei_start_date_current },
                |m: &mut Account| { &mut m.ei_start_date_current },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "ei_end_date_current",
                |m: &Account| { &m.ei_end_date_current },
                |m: &mut Account| { &mut m.ei_end_date_current },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ei_payment_frequency_current",
                |m: &Account| { &m.ei_payment_frequency_current },
                |m: &mut Account| { &mut m.ei_payment_frequency_current },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "emi_last_paid_date_current",
                |m: &Account| { &m.emi_last_paid_date_current },
                |m: &mut Account| { &mut m.emi_last_paid_date_current },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ei_payment_day",
                |m: &Account| { &m.ei_payment_day },
                |m: &mut Account| { &mut m.ei_payment_day },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ei_orginal_term",
                |m: &Account| { &m.ei_orginal_term },
                |m: &mut Account| { &mut m.ei_orginal_term },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ei_balance_term",
                |m: &Account| { &m.ei_balance_term },
                |m: &mut Account| { &mut m.ei_balance_term },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "repricing_benchmark",
                |m: &Account| { &m.repricing_benchmark },
                |m: &mut Account| { &mut m.repricing_benchmark },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "spread",
                |m: &Account| { &m.spread },
                |m: &mut Account| { &mut m.spread },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "last_repricing_date",
                |m: &Account| { &m.last_repricing_date },
                |m: &mut Account| { &mut m.last_repricing_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "next_repricing_date",
                |m: &Account| { &m.next_repricing_date },
                |m: &mut Account| { &mut m.next_repricing_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "repricing_frequency",
                |m: &Account| { &m.repricing_frequency },
                |m: &mut Account| { &mut m.repricing_frequency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "number_ei_structures",
                |m: &Account| { &m.number_ei_structures },
                |m: &mut Account| { &mut m.number_ei_structures },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "npa_classification",
                |m: &Account| { &m.npa_classification },
                |m: &mut Account| { &mut m.npa_classification },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "remark",
                |m: &Account| { &m.remark },
                |m: &mut Account| { &mut m.remark },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "months_os_comb",
                |m: &Account| { &m.months_os_comb },
                |m: &mut Account| { &mut m.months_os_comb },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "moratorium_type",
                |m: &Account| { &m.moratorium_type },
                |m: &mut Account| { &mut m.moratorium_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "from_moratorium_date",
                |m: &Account| { &m.from_moratorium_date },
                |m: &mut Account| { &mut m.from_moratorium_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "to_moratorium_date",
                |m: &Account| { &m.to_moratorium_date },
                |m: &mut Account| { &mut m.to_moratorium_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "recalculate_ei_amount_flag",
                |m: &Account| { &m.recalculate_ei_amount_flag },
                |m: &mut Account| { &mut m.recalculate_ei_amount_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "moratorium_interest_calculation",
                |m: &Account| { &m.moratorium_interest_calculation },
                |m: &mut Account| { &mut m.moratorium_interest_calculation },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "bullet_payment_flag",
                |m: &Account| { &m.bullet_payment_flag },
                |m: &mut Account| { &mut m.bullet_payment_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "restructured_flag",
                |m: &Account| { &m.restructured_flag },
                |m: &mut Account| { &mut m.restructured_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "residential_mortgage",
                |m: &Account| { &m.residential_mortgage },
                |m: &mut Account| { &mut m.residential_mortgage },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "risk_weight",
                |m: &Account| { &m.risk_weight },
                |m: &mut Account| { &mut m.risk_weight },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "internal_rating",
                |m: &Account| { &m.internal_rating },
                |m: &mut Account| { &mut m.internal_rating },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "external_rating",
                |m: &Account| { &m.external_rating },
                |m: &mut Account| { &mut m.external_rating },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "contractual_tenor",
                |m: &Account| { &m.contractual_tenor },
                |m: &mut Account| { &mut m.contractual_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "residual_tenor",
                |m: &Account| { &m.residual_tenor },
                |m: &mut Account| { &mut m.residual_tenor },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "customer_constitution_code",
                |m: &Account| { &m.customer_constitution_code },
                |m: &mut Account| { &mut m.customer_constitution_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "product_code",
                |m: &Account| { &m.product_code },
                |m: &mut Account| { &mut m.product_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "p_gl_code",
                |m: &Account| { &m.p_gl_code },
                |m: &mut Account| { &mut m.p_gl_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "m_npa_classification",
                |m: &Account| { &m.m_npa_classification },
                |m: &mut Account| { &mut m.m_npa_classification },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "accrued_interest",
                |m: &Account| { &m.accrued_interest },
                |m: &mut Account| { &mut m.accrued_interest },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "customer_id",
                |m: &Account| { &m.customer_id },
                |m: &mut Account| { &mut m.customer_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "customer_name",
                |m: &Account| { &m.customer_name },
                |m: &mut Account| { &mut m.customer_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_id",
                |m: &Account| { &m.group_id },
                |m: &mut Account| { &mut m.group_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_name",
                |m: &Account| { &m.group_name },
                |m: &mut Account| { &mut m.group_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "branch_code",
                |m: &Account| { &m.branch_code },
                |m: &mut Account| { &mut m.branch_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sector",
                |m: &Account| { &m.sector },
                |m: &mut Account| { &mut m.sector },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "industry",
                |m: &Account| { &m.industry },
                |m: &mut Account| { &mut m.industry },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ltv",
                |m: &Account| { &m.ltv },
                |m: &mut Account| { &mut m.ltv },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "overdue_account",
                |m: &Account| { &m.overdue_account },
                |m: &mut Account| { &mut m.overdue_account },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "excess_account",
                |m: &Account| { &m.excess_account },
                |m: &mut Account| { &mut m.excess_account },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "loan_type",
                |m: &Account| { &m.loan_type },
                |m: &mut Account| { &mut m.loan_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "residual_interest",
                |m: &Account| { &m.residual_interest },
                |m: &mut Account| { &mut m.residual_interest },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency",
                |m: &Account| { &m.currency },
                |m: &mut Account| { &mut m.currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "hdfc_ltd_percentage",
                |m: &Account| { &m.hdfc_ltd_percentage },
                |m: &mut Account| { &mut m.hdfc_ltd_percentage },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "securitization_percentage",
                |m: &Account| { &m.securitization_percentage },
                |m: &mut Account| { &mut m.securitization_percentage },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "overdue_type",
                |m: &Account| { &m.overdue_type },
                |m: &mut Account| { &mut m.overdue_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &Account| { &m.alm_line },
                |m: &mut Account| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "structure_number",
                |m: &Account| { &m.structure_number },
                |m: &mut Account| { &mut m.structure_number },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "memi",
                |m: &Account| { &m.memi },
                |m: &mut Account| { &mut m.memi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ost_bal",
                |m: &Account| { &m.ost_bal },
                |m: &mut Account| { &mut m.ost_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "roi",
                |m: &Account| { &m.roi },
                |m: &mut Account| { &mut m.roi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "asondate",
                |m: &Account| { &m.asondate },
                |m: &mut Account| { &mut m.asondate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "emi_overdue_gl_cd",
                |m: &Account| { &m.emi_overdue_gl_cd },
                |m: &mut Account| { &mut m.emi_overdue_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "pre_emi_overdue_gl_cd",
                |m: &Account| { &m.pre_emi_overdue_gl_cd },
                |m: &mut Account| { &mut m.pre_emi_overdue_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "excess_emi_gl_cd",
                |m: &Account| { &m.excess_emi_gl_cd },
                |m: &mut Account| { &mut m.excess_emi_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "excess_pre_emi_gl_cd",
                |m: &Account| { &m.excess_pre_emi_gl_cd },
                |m: &mut Account| { &mut m.excess_pre_emi_gl_cd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_prin_amt",
                |m: &Account| { &m.tot_prin_amt },
                |m: &mut Account| { &mut m.tot_prin_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_int_amt",
                |m: &Account| { &m.tot_int_amt },
                |m: &mut Account| { &mut m.tot_int_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "sma_flag",
                |m: &Account| { &m.sma_flag },
                |m: &mut Account| { &mut m.sma_flag },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cashflow>>(
                "cashflows",
                |m: &Account| { &m.cashflows },
                |m: &mut Account| { &mut m.cashflows },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Account>(
                "Account",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Account {
        static instance: ::protobuf::rt::LazyV2<Account> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Account::new)
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.account_number.clear();
        self.disbursed_amt = 0.;
        self.os_loan_bal_local_currency = 0.;
        self.curr_applicable_interest_rate = 0.;
        self.ei_amount_current = 0.;
        self.interest_type.clear();
        self.os_p_bal_due_local_currency = 0.;
        self.os_i_bal_due_local_currency = 0.;
        self.ei_amt_paid_advance_local_curr = 0.;
        self.pre_ei_bal_local_curr = 0.;
        self.account_open_value_date = 0;
        self.maturity_date = 0;
        self.ei_start_date_current = 0;
        self.ei_end_date_current = 0;
        self.ei_payment_frequency_current.clear();
        self.emi_last_paid_date_current = 0;
        self.ei_payment_day.clear();
        self.ei_orginal_term.clear();
        self.ei_balance_term.clear();
        self.repricing_benchmark.clear();
        self.spread.clear();
        self.last_repricing_date = 0;
        self.next_repricing_date = 0;
        self.repricing_frequency.clear();
        self.number_ei_structures = 0;
        self.npa_classification.clear();
        self.remark.clear();
        self.months_os_comb.clear();
        self.moratorium_type.clear();
        self.from_moratorium_date = 0;
        self.to_moratorium_date = 0;
        self.recalculate_ei_amount_flag.clear();
        self.moratorium_interest_calculation.clear();
        self.bullet_payment_flag.clear();
        self.restructured_flag.clear();
        self.residential_mortgage.clear();
        self.risk_weight.clear();
        self.internal_rating.clear();
        self.external_rating.clear();
        self.contractual_tenor.clear();
        self.residual_tenor.clear();
        self.customer_constitution_code.clear();
        self.product_code.clear();
        self.p_gl_code.clear();
        self.m_npa_classification.clear();
        self.accrued_interest.clear();
        self.customer_id.clear();
        self.customer_name.clear();
        self.group_id.clear();
        self.group_name.clear();
        self.branch_code.clear();
        self.sector.clear();
        self.industry.clear();
        self.ltv.clear();
        self.overdue_account.clear();
        self.excess_account.clear();
        self.loan_type.clear();
        self.residual_interest.clear();
        self.currency.clear();
        self.hdfc_ltd_percentage = 0.;
        self.securitization_percentage = 0.;
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
        self.sma_flag.clear();
        self.cashflows.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15cf_retail_loans.proto\"r\n\x08Cashflow\x12'\n\x0finterest_amount\
    \x18\x01\x20\x01(\x01R\x0einterestAmount\x12)\n\x10principal_amount\x18\
    \x02\x20\x01(\x01R\x0fprincipalAmount\x12\x12\n\x04date\x18\x03\x20\x01(\
    \x03R\x04date\"\x87\x19\n\x07Account\x12%\n\x0eaccount_number\x18\x01\
    \x20\x01(\tR\raccountNumber\x12#\n\rdisbursed_amt\x18\x02\x20\x01(\x01R\
    \x0cdisbursedAmt\x12:\n\x1aos_loan_bal_local_currency\x18\x03\x20\x01(\
    \x01R\x16osLoanBalLocalCurrency\x12A\n\x1dcurr_applicable_interest_rate\
    \x18\x04\x20\x01(\x01R\x1acurrApplicableInterestRate\x12*\n\x11ei_amount\
    _current\x18\x05\x20\x01(\x01R\x0feiAmountCurrent\x12#\n\rinterest_type\
    \x18\x06\x20\x01(\tR\x0cinterestType\x12;\n\x1bos_p_bal_due_local_curren\
    cy\x18\x07\x20\x01(\x01R\x16osPBalDueLocalCurrency\x12;\n\x1bos_i_bal_du\
    e_local_currency\x18\x08\x20\x01(\x01R\x16osIBalDueLocalCurrency\x12A\n\
    \x1eei_amt_paid_advance_local_curr\x18\t\x20\x01(\x01R\x19eiAmtPaidAdvan\
    ceLocalCurr\x120\n\x15pre_ei_bal_local_curr\x18\n\x20\x01(\x01R\x11preEi\
    BalLocalCurr\x125\n\x17account_open_value_date\x18\x0b\x20\x01(\x03R\x14\
    accountOpenValueDate\x12#\n\rmaturity_date\x18\x0c\x20\x01(\x03R\x0cmatu\
    rityDate\x121\n\x15ei_start_date_current\x18\r\x20\x01(\x03R\x12eiStartD\
    ateCurrent\x12-\n\x13ei_end_date_current\x18\x0e\x20\x01(\x03R\x10eiEndD\
    ateCurrent\x12?\n\x1cei_payment_frequency_current\x18\x0f\x20\x01(\tR\
    \x19eiPaymentFrequencyCurrent\x12:\n\x1aemi_last_paid_date_current\x18\
    \x10\x20\x01(\x03R\x16emiLastPaidDateCurrent\x12$\n\x0eei_payment_day\
    \x18\x11\x20\x01(\tR\x0ceiPaymentDay\x12&\n\x0fei_orginal_term\x18\x12\
    \x20\x01(\tR\reiOrginalTerm\x12&\n\x0fei_balance_term\x18\x13\x20\x01(\t\
    R\reiBalanceTerm\x12/\n\x13repricing_benchmark\x18\x14\x20\x01(\tR\x12re\
    pricingBenchmark\x12\x16\n\x06spread\x18\x15\x20\x01(\tR\x06spread\x12.\
    \n\x13last_repricing_date\x18\x16\x20\x01(\x03R\x11lastRepricingDate\x12\
    .\n\x13next_repricing_date\x18\x17\x20\x01(\x03R\x11nextRepricingDate\
    \x12/\n\x13repricing_frequency\x18\x18\x20\x01(\tR\x12repricingFrequency\
    \x120\n\x14number_ei_structures\x18\x19\x20\x01(\x03R\x12numberEiStructu\
    res\x12-\n\x12npa_classification\x18\x1a\x20\x01(\tR\x11npaClassificatio\
    n\x12\x16\n\x06remark\x18\x1b\x20\x01(\tR\x06remark\x12$\n\x0emonths_os_\
    comb\x18\x1c\x20\x01(\tR\x0cmonthsOsComb\x12'\n\x0fmoratorium_type\x18\
    \x1d\x20\x01(\tR\x0emoratoriumType\x120\n\x14from_moratorium_date\x18\
    \x1e\x20\x01(\x03R\x12fromMoratoriumDate\x12,\n\x12to_moratorium_date\
    \x18\x1f\x20\x01(\x03R\x10toMoratoriumDate\x12;\n\x1arecalculate_ei_amou\
    nt_flag\x18\x20\x20\x01(\tR\x17recalculateEiAmountFlag\x12F\n\x1fmorator\
    ium_interest_calculation\x18!\x20\x01(\tR\x1dmoratoriumInterestCalculati\
    on\x12.\n\x13bullet_payment_flag\x18\"\x20\x01(\tR\x11bulletPaymentFlag\
    \x12+\n\x11restructured_flag\x18#\x20\x01(\tR\x10restructuredFlag\x121\n\
    \x14residential_mortgage\x18$\x20\x01(\tR\x13residentialMortgage\x12\x1f\
    \n\x0brisk_weight\x18%\x20\x01(\tR\nriskWeight\x12'\n\x0finternal_rating\
    \x18&\x20\x01(\tR\x0einternalRating\x12'\n\x0fexternal_rating\x18'\x20\
    \x01(\tR\x0eexternalRating\x12+\n\x11contractual_tenor\x18(\x20\x01(\tR\
    \x10contractualTenor\x12%\n\x0eresidual_tenor\x18)\x20\x01(\tR\rresidual\
    Tenor\x12<\n\x1acustomer_constitution_code\x18*\x20\x01(\tR\x18customerC\
    onstitutionCode\x12!\n\x0cproduct_code\x18+\x20\x01(\tR\x0bproductCode\
    \x12\x1a\n\tp_gl_code\x18,\x20\x01(\tR\x07pGlCode\x120\n\x14m_npa_classi\
    fication\x18-\x20\x01(\tR\x12mNpaClassification\x12)\n\x10accrued_intere\
    st\x18.\x20\x01(\tR\x0faccruedInterest\x12\x1f\n\x0bcustomer_id\x18/\x20\
    \x01(\tR\ncustomerId\x12#\n\rcustomer_name\x180\x20\x01(\tR\x0ccustomerN\
    ame\x12\x19\n\x08group_id\x181\x20\x01(\tR\x07groupId\x12\x1d\n\ngroup_n\
    ame\x182\x20\x01(\tR\tgroupName\x12\x1f\n\x0bbranch_code\x183\x20\x01(\t\
    R\nbranchCode\x12\x16\n\x06sector\x184\x20\x01(\tR\x06sector\x12\x1a\n\
    \x08industry\x185\x20\x01(\tR\x08industry\x12\x10\n\x03ltv\x186\x20\x01(\
    \tR\x03ltv\x12'\n\x0foverdue_account\x187\x20\x01(\tR\x0eoverdueAccount\
    \x12%\n\x0eexcess_account\x188\x20\x01(\tR\rexcessAccount\x12\x1b\n\tloa\
    n_type\x189\x20\x01(\tR\x08loanType\x12+\n\x11residual_interest\x18:\x20\
    \x01(\tR\x10residualInterest\x12\x1a\n\x08currency\x18;\x20\x01(\tR\x08c\
    urrency\x12.\n\x13hdfc_ltd_percentage\x18<\x20\x01(\x01R\x11hdfcLtdPerce\
    ntage\x12;\n\x19securitization_percentage\x18=\x20\x01(\x01R\x18securiti\
    zationPercentage\x12!\n\x0coverdue_type\x18>\x20\x01(\tR\x0boverdueType\
    \x12\x19\n\x08alm_line\x18?\x20\x01(\tR\x07almLine\x12)\n\x10structure_n\
    umber\x18@\x20\x01(\tR\x0fstructureNumber\x12\x12\n\x04memi\x18A\x20\x01\
    (\x01R\x04memi\x12\x17\n\x07ost_bal\x18B\x20\x01(\x01R\x06ostBal\x12\x10\
    \n\x03roi\x18C\x20\x01(\x01R\x03roi\x12\x1a\n\x08asondate\x18D\x20\x01(\
    \x03R\x08asondate\x12)\n\x11emi_overdue_gl_cd\x18E\x20\x01(\x03R\x0eemiO\
    verdueGlCd\x120\n\x15pre_emi_overdue_gl_cd\x18F\x20\x01(\x03R\x11preEmiO\
    verdueGlCd\x12'\n\x10excess_emi_gl_cd\x18G\x20\x01(\x03R\rexcessEmiGlCd\
    \x12.\n\x14excess_pre_emi_gl_cd\x18H\x20\x01(\x03R\x10excessPreEmiGlCd\
    \x12\x20\n\x0ctot_prin_amt\x18I\x20\x01(\x01R\ntotPrinAmt\x12\x1e\n\x0bt\
    ot_int_amt\x18J\x20\x01(\x01R\ttotIntAmt\x12\x19\n\x08sma_flag\x18K\x20\
    \x01(\tR\x07smaFlag\x12'\n\tcashflows\x18L\x20\x03(\x0b2\t.CashflowR\tca\
    shflowsb\x06proto3\
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
