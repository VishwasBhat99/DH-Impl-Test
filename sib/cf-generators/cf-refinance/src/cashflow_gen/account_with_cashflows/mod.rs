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
    pub date_of_availment: i64,
    pub source: ::std::string::String,
    pub amount: f64,
    pub roi: f64,
    pub mat_date: i64,
    pub repayment_sch: ::std::string::String,
    pub res_mat: i64,
    pub frequency: ::std::string::String,
    pub pmt_st_dt: i64,
    pub remaining_prin: f64,
    pub crncy_code: ::std::string::String,
    pub p_installment: f64,
    pub no_of_installment: i64,
    pub total_inst_amt: f64,
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

    // int64 date_of_availment = 1;


    pub fn get_date_of_availment(&self) -> i64 {
        self.date_of_availment
    }
    pub fn clear_date_of_availment(&mut self) {
        self.date_of_availment = 0;
    }

    // Param is passed by value, moved
    pub fn set_date_of_availment(&mut self, v: i64) {
        self.date_of_availment = v;
    }

    // string source = 2;


    pub fn get_source(&self) -> &str {
        &self.source
    }
    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source, ::std::string::String::new())
    }

    // double amount = 3;


    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: f64) {
        self.amount = v;
    }

    // double roi = 4;


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

    // int64 mat_date = 5;


    pub fn get_mat_date(&self) -> i64 {
        self.mat_date
    }
    pub fn clear_mat_date(&mut self) {
        self.mat_date = 0;
    }

    // Param is passed by value, moved
    pub fn set_mat_date(&mut self, v: i64) {
        self.mat_date = v;
    }

    // string repayment_sch = 6;


    pub fn get_repayment_sch(&self) -> &str {
        &self.repayment_sch
    }
    pub fn clear_repayment_sch(&mut self) {
        self.repayment_sch.clear();
    }

    // Param is passed by value, moved
    pub fn set_repayment_sch(&mut self, v: ::std::string::String) {
        self.repayment_sch = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_repayment_sch(&mut self) -> &mut ::std::string::String {
        &mut self.repayment_sch
    }

    // Take field
    pub fn take_repayment_sch(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.repayment_sch, ::std::string::String::new())
    }

    // int64 res_mat = 7;


    pub fn get_res_mat(&self) -> i64 {
        self.res_mat
    }
    pub fn clear_res_mat(&mut self) {
        self.res_mat = 0;
    }

    // Param is passed by value, moved
    pub fn set_res_mat(&mut self, v: i64) {
        self.res_mat = v;
    }

    // string frequency = 8;


    pub fn get_frequency(&self) -> &str {
        &self.frequency
    }
    pub fn clear_frequency(&mut self) {
        self.frequency.clear();
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: ::std::string::String) {
        self.frequency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frequency(&mut self) -> &mut ::std::string::String {
        &mut self.frequency
    }

    // Take field
    pub fn take_frequency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.frequency, ::std::string::String::new())
    }

    // int64 pmt_st_dt = 9;


    pub fn get_pmt_st_dt(&self) -> i64 {
        self.pmt_st_dt
    }
    pub fn clear_pmt_st_dt(&mut self) {
        self.pmt_st_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_pmt_st_dt(&mut self, v: i64) {
        self.pmt_st_dt = v;
    }

    // double remaining_prin = 10;


    pub fn get_remaining_prin(&self) -> f64 {
        self.remaining_prin
    }
    pub fn clear_remaining_prin(&mut self) {
        self.remaining_prin = 0.;
    }

    // Param is passed by value, moved
    pub fn set_remaining_prin(&mut self, v: f64) {
        self.remaining_prin = v;
    }

    // string crncy_code = 11;


    pub fn get_crncy_code(&self) -> &str {
        &self.crncy_code
    }
    pub fn clear_crncy_code(&mut self) {
        self.crncy_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_crncy_code(&mut self, v: ::std::string::String) {
        self.crncy_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crncy_code(&mut self) -> &mut ::std::string::String {
        &mut self.crncy_code
    }

    // Take field
    pub fn take_crncy_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.crncy_code, ::std::string::String::new())
    }

    // double p_installment = 12;


    pub fn get_p_installment(&self) -> f64 {
        self.p_installment
    }
    pub fn clear_p_installment(&mut self) {
        self.p_installment = 0.;
    }

    // Param is passed by value, moved
    pub fn set_p_installment(&mut self, v: f64) {
        self.p_installment = v;
    }

    // int64 no_of_installment = 13;


    pub fn get_no_of_installment(&self) -> i64 {
        self.no_of_installment
    }
    pub fn clear_no_of_installment(&mut self) {
        self.no_of_installment = 0;
    }

    // Param is passed by value, moved
    pub fn set_no_of_installment(&mut self, v: i64) {
        self.no_of_installment = v;
    }

    // double total_inst_amt = 14;


    pub fn get_total_inst_amt(&self) -> f64 {
        self.total_inst_amt
    }
    pub fn clear_total_inst_amt(&mut self) {
        self.total_inst_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_inst_amt(&mut self, v: f64) {
        self.total_inst_amt = v;
    }

    // repeated .Cashflow cashflows = 15;


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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.date_of_availment = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amount = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.roi = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mat_date = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.repayment_sch)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.res_mat = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.frequency)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.pmt_st_dt = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.remaining_prin = tmp;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.crncy_code)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.p_installment = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.no_of_installment = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_inst_amt = tmp;
                },
                15 => {
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
        if self.date_of_availment != 0 {
            my_size += ::protobuf::rt::value_size(1, self.date_of_availment, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.source.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.source);
        }
        if self.amount != 0. {
            my_size += 9;
        }
        if self.roi != 0. {
            my_size += 9;
        }
        if self.mat_date != 0 {
            my_size += ::protobuf::rt::value_size(5, self.mat_date, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.repayment_sch.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.repayment_sch);
        }
        if self.res_mat != 0 {
            my_size += ::protobuf::rt::value_size(7, self.res_mat, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.frequency.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.frequency);
        }
        if self.pmt_st_dt != 0 {
            my_size += ::protobuf::rt::value_size(9, self.pmt_st_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.remaining_prin != 0. {
            my_size += 9;
        }
        if !self.crncy_code.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.crncy_code);
        }
        if self.p_installment != 0. {
            my_size += 9;
        }
        if self.no_of_installment != 0 {
            my_size += ::protobuf::rt::value_size(13, self.no_of_installment, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_inst_amt != 0. {
            my_size += 9;
        }
        for value in &self.cashflows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.date_of_availment != 0 {
            os.write_int64(1, self.date_of_availment)?;
        }
        if !self.source.is_empty() {
            os.write_string(2, &self.source)?;
        }
        if self.amount != 0. {
            os.write_double(3, self.amount)?;
        }
        if self.roi != 0. {
            os.write_double(4, self.roi)?;
        }
        if self.mat_date != 0 {
            os.write_int64(5, self.mat_date)?;
        }
        if !self.repayment_sch.is_empty() {
            os.write_string(6, &self.repayment_sch)?;
        }
        if self.res_mat != 0 {
            os.write_int64(7, self.res_mat)?;
        }
        if !self.frequency.is_empty() {
            os.write_string(8, &self.frequency)?;
        }
        if self.pmt_st_dt != 0 {
            os.write_int64(9, self.pmt_st_dt)?;
        }
        if self.remaining_prin != 0. {
            os.write_double(10, self.remaining_prin)?;
        }
        if !self.crncy_code.is_empty() {
            os.write_string(11, &self.crncy_code)?;
        }
        if self.p_installment != 0. {
            os.write_double(12, self.p_installment)?;
        }
        if self.no_of_installment != 0 {
            os.write_int64(13, self.no_of_installment)?;
        }
        if self.total_inst_amt != 0. {
            os.write_double(14, self.total_inst_amt)?;
        }
        for v in &self.cashflows {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "date_of_availment",
                |m: &AccountWithCashflows| { &m.date_of_availment },
                |m: &mut AccountWithCashflows| { &mut m.date_of_availment },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "source",
                |m: &AccountWithCashflows| { &m.source },
                |m: &mut AccountWithCashflows| { &mut m.source },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "amount",
                |m: &AccountWithCashflows| { &m.amount },
                |m: &mut AccountWithCashflows| { &mut m.amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "roi",
                |m: &AccountWithCashflows| { &m.roi },
                |m: &mut AccountWithCashflows| { &mut m.roi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "mat_date",
                |m: &AccountWithCashflows| { &m.mat_date },
                |m: &mut AccountWithCashflows| { &mut m.mat_date },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "repayment_sch",
                |m: &AccountWithCashflows| { &m.repayment_sch },
                |m: &mut AccountWithCashflows| { &mut m.repayment_sch },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "res_mat",
                |m: &AccountWithCashflows| { &m.res_mat },
                |m: &mut AccountWithCashflows| { &mut m.res_mat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "frequency",
                |m: &AccountWithCashflows| { &m.frequency },
                |m: &mut AccountWithCashflows| { &mut m.frequency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "pmt_st_dt",
                |m: &AccountWithCashflows| { &m.pmt_st_dt },
                |m: &mut AccountWithCashflows| { &mut m.pmt_st_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "remaining_prin",
                |m: &AccountWithCashflows| { &m.remaining_prin },
                |m: &mut AccountWithCashflows| { &mut m.remaining_prin },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "crncy_code",
                |m: &AccountWithCashflows| { &m.crncy_code },
                |m: &mut AccountWithCashflows| { &mut m.crncy_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "p_installment",
                |m: &AccountWithCashflows| { &m.p_installment },
                |m: &mut AccountWithCashflows| { &mut m.p_installment },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "no_of_installment",
                |m: &AccountWithCashflows| { &m.no_of_installment },
                |m: &mut AccountWithCashflows| { &mut m.no_of_installment },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "total_inst_amt",
                |m: &AccountWithCashflows| { &m.total_inst_amt },
                |m: &mut AccountWithCashflows| { &mut m.total_inst_amt },
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
        self.date_of_availment = 0;
        self.source.clear();
        self.amount = 0.;
        self.roi = 0.;
        self.mat_date = 0;
        self.repayment_sch.clear();
        self.res_mat = 0;
        self.frequency.clear();
        self.pmt_st_dt = 0;
        self.remaining_prin = 0.;
        self.crncy_code.clear();
        self.p_installment = 0.;
        self.no_of_installment = 0;
        self.total_inst_amt = 0.;
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
    \n\rref_sib.proto\"R\n\x08Cashflow\x12\x17\n\x07int_amt\x18\x01\x20\x01(\
    \x01R\x06intAmt\x12\x19\n\x08prin_amt\x18\x02\x20\x01(\x01R\x07prinAmt\
    \x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\"\xfd\x03\n\x14AccountW\
    ithCashflows\x12*\n\x11date_of_availment\x18\x01\x20\x01(\x03R\x0fdateOf\
    Availment\x12\x16\n\x06source\x18\x02\x20\x01(\tR\x06source\x12\x16\n\
    \x06amount\x18\x03\x20\x01(\x01R\x06amount\x12\x10\n\x03roi\x18\x04\x20\
    \x01(\x01R\x03roi\x12\x19\n\x08mat_date\x18\x05\x20\x01(\x03R\x07matDate\
    \x12#\n\rrepayment_sch\x18\x06\x20\x01(\tR\x0crepaymentSch\x12\x17\n\x07\
    res_mat\x18\x07\x20\x01(\x03R\x06resMat\x12\x1c\n\tfrequency\x18\x08\x20\
    \x01(\tR\tfrequency\x12\x1a\n\tpmt_st_dt\x18\t\x20\x01(\x03R\x07pmtStDt\
    \x12%\n\x0eremaining_prin\x18\n\x20\x01(\x01R\rremainingPrin\x12\x1d\n\n\
    crncy_code\x18\x0b\x20\x01(\tR\tcrncyCode\x12#\n\rp_installment\x18\x0c\
    \x20\x01(\x01R\x0cpInstallment\x12*\n\x11no_of_installment\x18\r\x20\x01\
    (\x03R\x0fnoOfInstallment\x12$\n\x0etotal_inst_amt\x18\x0e\x20\x01(\x01R\
    \x0ctotalInstAmt\x12'\n\tcashflows\x18\x0f\x20\x03(\x0b2\t.CashflowR\tca\
    shflowsb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).expect("Error Parsing Proto File Descriptor")
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
