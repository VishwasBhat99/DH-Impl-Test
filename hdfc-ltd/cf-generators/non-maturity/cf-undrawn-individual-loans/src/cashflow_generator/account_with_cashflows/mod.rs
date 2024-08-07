// This file is generated by rust-protobuf 2.26.1. Do not edit
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
    pub lac_no: ::std::string::String,
    pub sanc_amt: f64,
    pub amt_14per: f64,
    pub amt_2per: f64,
    pub approval_dt: i64,
    pub first_cf_dt: i64,
    pub currency: ::std::string::String,
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

    // string lac_no = 1;


    pub fn get_lac_no(&self) -> &str {
        &self.lac_no
    }
    pub fn clear_lac_no(&mut self) {
        self.lac_no.clear();
    }

    // Param is passed by value, moved
    pub fn set_lac_no(&mut self, v: ::std::string::String) {
        self.lac_no = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lac_no(&mut self) -> &mut ::std::string::String {
        &mut self.lac_no
    }

    // Take field
    pub fn take_lac_no(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.lac_no, ::std::string::String::new())
    }

    // double sanc_amt = 2;


    pub fn get_sanc_amt(&self) -> f64 {
        self.sanc_amt
    }
    pub fn clear_sanc_amt(&mut self) {
        self.sanc_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sanc_amt(&mut self, v: f64) {
        self.sanc_amt = v;
    }

    // double amt_14per = 3;


    pub fn get_amt_14per(&self) -> f64 {
        self.amt_14per
    }
    pub fn clear_amt_14per(&mut self) {
        self.amt_14per = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt_14per(&mut self, v: f64) {
        self.amt_14per = v;
    }

    // double amt_2per = 4;


    pub fn get_amt_2per(&self) -> f64 {
        self.amt_2per
    }
    pub fn clear_amt_2per(&mut self) {
        self.amt_2per = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt_2per(&mut self, v: f64) {
        self.amt_2per = v;
    }

    // int64 approval_dt = 5;


    pub fn get_approval_dt(&self) -> i64 {
        self.approval_dt
    }
    pub fn clear_approval_dt(&mut self) {
        self.approval_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_approval_dt(&mut self, v: i64) {
        self.approval_dt = v;
    }

    // int64 first_cf_dt = 6;


    pub fn get_first_cf_dt(&self) -> i64 {
        self.first_cf_dt
    }
    pub fn clear_first_cf_dt(&mut self) {
        self.first_cf_dt = 0;
    }

    // Param is passed by value, moved
    pub fn set_first_cf_dt(&mut self, v: i64) {
        self.first_cf_dt = v;
    }

    // string currency = 7;


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

    // repeated .Cashflow cashflows = 8;


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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.lac_no)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sanc_amt = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt_14per = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.amt_2per = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.approval_dt = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.first_cf_dt = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
                },
                8 => {
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
        if !self.lac_no.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.lac_no);
        }
        if self.sanc_amt != 0. {
            my_size += 9;
        }
        if self.amt_14per != 0. {
            my_size += 9;
        }
        if self.amt_2per != 0. {
            my_size += 9;
        }
        if self.approval_dt != 0 {
            my_size += ::protobuf::rt::value_size(5, self.approval_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.first_cf_dt != 0 {
            my_size += ::protobuf::rt::value_size(6, self.first_cf_dt, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.currency);
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
        if !self.lac_no.is_empty() {
            os.write_string(1, &self.lac_no)?;
        }
        if self.sanc_amt != 0. {
            os.write_double(2, self.sanc_amt)?;
        }
        if self.amt_14per != 0. {
            os.write_double(3, self.amt_14per)?;
        }
        if self.amt_2per != 0. {
            os.write_double(4, self.amt_2per)?;
        }
        if self.approval_dt != 0 {
            os.write_int64(5, self.approval_dt)?;
        }
        if self.first_cf_dt != 0 {
            os.write_int64(6, self.first_cf_dt)?;
        }
        if !self.currency.is_empty() {
            os.write_string(7, &self.currency)?;
        }
        for v in &self.cashflows {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                "lac_no",
                |m: &AccountWithCashflows| { &m.lac_no },
                |m: &mut AccountWithCashflows| { &mut m.lac_no },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sanc_amt",
                |m: &AccountWithCashflows| { &m.sanc_amt },
                |m: &mut AccountWithCashflows| { &mut m.sanc_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "amt_14per",
                |m: &AccountWithCashflows| { &m.amt_14per },
                |m: &mut AccountWithCashflows| { &mut m.amt_14per },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "amt_2per",
                |m: &AccountWithCashflows| { &m.amt_2per },
                |m: &mut AccountWithCashflows| { &mut m.amt_2per },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "approval_dt",
                |m: &AccountWithCashflows| { &m.approval_dt },
                |m: &mut AccountWithCashflows| { &mut m.approval_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "first_cf_dt",
                |m: &AccountWithCashflows| { &m.first_cf_dt },
                |m: &mut AccountWithCashflows| { &mut m.first_cf_dt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency",
                |m: &AccountWithCashflows| { &m.currency },
                |m: &mut AccountWithCashflows| { &mut m.currency },
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
        self.lac_no.clear();
        self.sanc_amt = 0.;
        self.amt_14per = 0.;
        self.amt_2per = 0.;
        self.approval_dt = 0;
        self.first_cf_dt = 0;
        self.currency.clear();
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
    \n\rundrawn.proto\"r\n\x08Cashflow\x12'\n\x0finterest_amount\x18\x01\x20\
    \x01(\x01R\x0einterestAmount\x12)\n\x10principal_amount\x18\x02\x20\x01(\
    \x01R\x0fprincipalAmount\x12\x12\n\x04date\x18\x03\x20\x01(\x03R\x04date\
    \"\x86\x02\n\x14AccountWithCashflows\x12\x15\n\x06lac_no\x18\x01\x20\x01\
    (\tR\x05lacNo\x12\x19\n\x08sanc_amt\x18\x02\x20\x01(\x01R\x07sancAmt\x12\
    \x1b\n\tamt_14per\x18\x03\x20\x01(\x01R\x08amt14per\x12\x19\n\x08amt_2pe\
    r\x18\x04\x20\x01(\x01R\x07amt2per\x12\x1f\n\x0bapproval_dt\x18\x05\x20\
    \x01(\x03R\napprovalDt\x12\x1e\n\x0bfirst_cf_dt\x18\x06\x20\x01(\x03R\tf\
    irstCfDt\x12\x1a\n\x08currency\x18\x07\x20\x01(\tR\x08currency\x12'\n\tc\
    ashflows\x18\x08\x20\x03(\x0b2\t.CashflowR\tcashflowsb\x06proto3\
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
