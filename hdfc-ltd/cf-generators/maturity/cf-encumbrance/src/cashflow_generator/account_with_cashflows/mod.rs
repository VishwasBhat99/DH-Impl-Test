// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

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

impl Cashflow {
    pub fn new() -> Cashflow {
        ::std::default::Default::default()
    }

    // double interest_amount = 1;

    pub fn clear_interest_amount(&mut self) {
        self.interest_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_interest_amount(&mut self, v: f64) {
        self.interest_amount = v;
    }

    pub fn get_interest_amount(&self) -> f64 {
        self.interest_amount
    }

    // double principal_amount = 2;

    pub fn clear_principal_amount(&mut self) {
        self.principal_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_principal_amount(&mut self, v: f64) {
        self.principal_amount = v;
    }

    pub fn get_principal_amount(&self) -> f64 {
        self.principal_amount
    }

    // int64 date = 3;

    pub fn clear_date(&mut self) {
        self.date = 0;
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: i64) {
        self.date = v;
    }

    pub fn get_date(&self) -> i64 {
        self.date
    }
}

impl ::protobuf::Message for Cashflow {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Cashflow {
        Cashflow::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
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
                ::protobuf::reflect::MessageDescriptor::new::<Cashflow>(
                    "Cashflow",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Cashflow {
        static mut instance: ::protobuf::lazy::Lazy<Cashflow> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cashflow,
        };
        unsafe {
            instance.get(Cashflow::new)
        }
    }
}

impl ::protobuf::Clear for Cashflow {
    fn clear(&mut self) {
        self.clear_interest_amount();
        self.clear_principal_amount();
        self.clear_date();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cashflow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cashflow {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountWithCashflows {
    // message fields
    pub deal_number: ::std::string::String,
    pub deal_type: ::std::string::String,
    pub collateral_id: ::std::string::String,
    pub collateral_amount: f64,
    pub collateral_market_value: ::std::string::String,
    pub currency: ::std::string::String,
    pub location: ::std::string::String,
    pub cashflows: ::protobuf::RepeatedField<Cashflow>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl AccountWithCashflows {
    pub fn new() -> AccountWithCashflows {
        ::std::default::Default::default()
    }

    // string deal_number = 1;

    pub fn clear_deal_number(&mut self) {
        self.deal_number.clear();
    }

    // Param is passed by value, moved
    pub fn set_deal_number(&mut self, v: ::std::string::String) {
        self.deal_number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deal_number(&mut self) -> &mut ::std::string::String {
        &mut self.deal_number
    }

    // Take field
    pub fn take_deal_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.deal_number, ::std::string::String::new())
    }

    pub fn get_deal_number(&self) -> &str {
        &self.deal_number
    }

    // string deal_type = 2;

    pub fn clear_deal_type(&mut self) {
        self.deal_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_deal_type(&mut self, v: ::std::string::String) {
        self.deal_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deal_type(&mut self) -> &mut ::std::string::String {
        &mut self.deal_type
    }

    // Take field
    pub fn take_deal_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.deal_type, ::std::string::String::new())
    }

    pub fn get_deal_type(&self) -> &str {
        &self.deal_type
    }

    // string collateral_id = 3;

    pub fn clear_collateral_id(&mut self) {
        self.collateral_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_collateral_id(&mut self, v: ::std::string::String) {
        self.collateral_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collateral_id(&mut self) -> &mut ::std::string::String {
        &mut self.collateral_id
    }

    // Take field
    pub fn take_collateral_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collateral_id, ::std::string::String::new())
    }

    pub fn get_collateral_id(&self) -> &str {
        &self.collateral_id
    }

    // double collateral_amount = 4;

    pub fn clear_collateral_amount(&mut self) {
        self.collateral_amount = 0.;
    }

    // Param is passed by value, moved
    pub fn set_collateral_amount(&mut self, v: f64) {
        self.collateral_amount = v;
    }

    pub fn get_collateral_amount(&self) -> f64 {
        self.collateral_amount
    }

    // string collateral_market_value = 5;

    pub fn clear_collateral_market_value(&mut self) {
        self.collateral_market_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_collateral_market_value(&mut self, v: ::std::string::String) {
        self.collateral_market_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collateral_market_value(&mut self) -> &mut ::std::string::String {
        &mut self.collateral_market_value
    }

    // Take field
    pub fn take_collateral_market_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collateral_market_value, ::std::string::String::new())
    }

    pub fn get_collateral_market_value(&self) -> &str {
        &self.collateral_market_value
    }

    // string currency = 6;

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

    pub fn get_currency(&self) -> &str {
        &self.currency
    }

    // string location = 7;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.location, ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    // repeated .Cashflow cashflows = 8;

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

    pub fn get_cashflows(&self) -> &[Cashflow] {
        &self.cashflows
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

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.deal_number)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.deal_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collateral_id)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.collateral_amount = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collateral_market_value)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.location)?;
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
        if !self.deal_number.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.deal_number);
        }
        if !self.deal_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.deal_type);
        }
        if !self.collateral_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.collateral_id);
        }
        if self.collateral_amount != 0. {
            my_size += 9;
        }
        if !self.collateral_market_value.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.collateral_market_value);
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.currency);
        }
        if !self.location.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.location);
        }
        for value in &self.cashflows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.deal_number.is_empty() {
            os.write_string(1, &self.deal_number)?;
        }
        if !self.deal_type.is_empty() {
            os.write_string(2, &self.deal_type)?;
        }
        if !self.collateral_id.is_empty() {
            os.write_string(3, &self.collateral_id)?;
        }
        if self.collateral_amount != 0. {
            os.write_double(4, self.collateral_amount)?;
        }
        if !self.collateral_market_value.is_empty() {
            os.write_string(5, &self.collateral_market_value)?;
        }
        if !self.currency.is_empty() {
            os.write_string(6, &self.currency)?;
        }
        if !self.location.is_empty() {
            os.write_string(7, &self.location)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AccountWithCashflows {
        AccountWithCashflows::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deal_number",
                    |m: &AccountWithCashflows| { &m.deal_number },
                    |m: &mut AccountWithCashflows| { &mut m.deal_number },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deal_type",
                    |m: &AccountWithCashflows| { &m.deal_type },
                    |m: &mut AccountWithCashflows| { &mut m.deal_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collateral_id",
                    |m: &AccountWithCashflows| { &m.collateral_id },
                    |m: &mut AccountWithCashflows| { &mut m.collateral_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "collateral_amount",
                    |m: &AccountWithCashflows| { &m.collateral_amount },
                    |m: &mut AccountWithCashflows| { &mut m.collateral_amount },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collateral_market_value",
                    |m: &AccountWithCashflows| { &m.collateral_market_value },
                    |m: &mut AccountWithCashflows| { &mut m.collateral_market_value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currency",
                    |m: &AccountWithCashflows| { &m.currency },
                    |m: &mut AccountWithCashflows| { &mut m.currency },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    |m: &AccountWithCashflows| { &m.location },
                    |m: &mut AccountWithCashflows| { &mut m.location },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cashflow>>(
                    "cashflows",
                    |m: &AccountWithCashflows| { &m.cashflows },
                    |m: &mut AccountWithCashflows| { &mut m.cashflows },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountWithCashflows>(
                    "AccountWithCashflows",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static AccountWithCashflows {
        static mut instance: ::protobuf::lazy::Lazy<AccountWithCashflows> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountWithCashflows,
        };
        unsafe {
            instance.get(AccountWithCashflows::new)
        }
    }
}

impl ::protobuf::Clear for AccountWithCashflows {
    fn clear(&mut self) {
        self.clear_deal_number();
        self.clear_deal_type();
        self.clear_collateral_id();
        self.clear_collateral_amount();
        self.clear_collateral_market_value();
        self.clear_currency();
        self.clear_location();
        self.clear_cashflows();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountWithCashflows {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountWithCashflows {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11proto/encum.proto\"r\n\x08Cashflow\x12'\n\x0finterest_amount\x18\
    \x01\x20\x01(\x01R\x0einterestAmount\x12)\n\x10principal_amount\x18\x02\
    \x20\x01(\x01R\x0fprincipalAmount\x12\x12\n\x04date\x18\x03\x20\x01(\x03\
    R\x04date\"\xbf\x02\n\x14AccountWithCashflows\x12\x1f\n\x0bdeal_number\
    \x18\x01\x20\x01(\tR\ndealNumber\x12\x1b\n\tdeal_type\x18\x02\x20\x01(\t\
    R\x08dealType\x12#\n\rcollateral_id\x18\x03\x20\x01(\tR\x0ccollateralId\
    \x12+\n\x11collateral_amount\x18\x04\x20\x01(\x01R\x10collateralAmount\
    \x126\n\x17collateral_market_value\x18\x05\x20\x01(\tR\x15collateralMark\
    etValue\x12\x1a\n\x08currency\x18\x06\x20\x01(\tR\x08currency\x12\x1a\n\
    \x08location\x18\x07\x20\x01(\tR\x08location\x12'\n\tcashflows\x18\x08\
    \x20\x03(\x0b2\t.CashflowR\tcashflowsb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
