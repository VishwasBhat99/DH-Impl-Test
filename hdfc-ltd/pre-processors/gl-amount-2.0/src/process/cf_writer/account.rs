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
pub struct Account {
    // message fields
    pub gl_code: ::std::string::String,
    pub branch_code: ::std::string::String,
    pub dr_bal: f64,
    pub cr_bal: f64,
    pub net_bal: f64,
    pub cf_type: ::std::string::String,
    pub curr: ::std::string::String,
    pub is_gl: ::std::string::String,
    pub alm_line: ::std::string::String,
    pub code_desc: ::std::string::String,
    pub group_2: ::std::string::String,
    pub group_3: ::std::string::String,
    pub line: ::std::string::String,
    pub prefix: ::std::string::String,
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

    // string gl_code = 1;


    pub fn get_gl_code(&self) -> &str {
        &self.gl_code
    }
    pub fn clear_gl_code(&mut self) {
        self.gl_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_gl_code(&mut self, v: ::std::string::String) {
        self.gl_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gl_code(&mut self) -> &mut ::std::string::String {
        &mut self.gl_code
    }

    // Take field
    pub fn take_gl_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gl_code, ::std::string::String::new())
    }

    // string branch_code = 2;


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

    // double dr_bal = 3;


    pub fn get_dr_bal(&self) -> f64 {
        self.dr_bal
    }
    pub fn clear_dr_bal(&mut self) {
        self.dr_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_dr_bal(&mut self, v: f64) {
        self.dr_bal = v;
    }

    // double cr_bal = 4;


    pub fn get_cr_bal(&self) -> f64 {
        self.cr_bal
    }
    pub fn clear_cr_bal(&mut self) {
        self.cr_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_cr_bal(&mut self, v: f64) {
        self.cr_bal = v;
    }

    // double net_bal = 5;


    pub fn get_net_bal(&self) -> f64 {
        self.net_bal
    }
    pub fn clear_net_bal(&mut self) {
        self.net_bal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_net_bal(&mut self, v: f64) {
        self.net_bal = v;
    }

    // string cf_type = 6;


    pub fn get_cf_type(&self) -> &str {
        &self.cf_type
    }
    pub fn clear_cf_type(&mut self) {
        self.cf_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf_type(&mut self, v: ::std::string::String) {
        self.cf_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf_type(&mut self) -> &mut ::std::string::String {
        &mut self.cf_type
    }

    // Take field
    pub fn take_cf_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf_type, ::std::string::String::new())
    }

    // string curr = 7;


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

    // string is_gl = 8;


    pub fn get_is_gl(&self) -> &str {
        &self.is_gl
    }
    pub fn clear_is_gl(&mut self) {
        self.is_gl.clear();
    }

    // Param is passed by value, moved
    pub fn set_is_gl(&mut self, v: ::std::string::String) {
        self.is_gl = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_gl(&mut self) -> &mut ::std::string::String {
        &mut self.is_gl
    }

    // Take field
    pub fn take_is_gl(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.is_gl, ::std::string::String::new())
    }

    // string alm_line = 9;


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

    // string code_desc = 10;


    pub fn get_code_desc(&self) -> &str {
        &self.code_desc
    }
    pub fn clear_code_desc(&mut self) {
        self.code_desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_code_desc(&mut self, v: ::std::string::String) {
        self.code_desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code_desc(&mut self) -> &mut ::std::string::String {
        &mut self.code_desc
    }

    // Take field
    pub fn take_code_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.code_desc, ::std::string::String::new())
    }

    // string group_2 = 11;


    pub fn get_group_2(&self) -> &str {
        &self.group_2
    }
    pub fn clear_group_2(&mut self) {
        self.group_2.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_2(&mut self, v: ::std::string::String) {
        self.group_2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group_2(&mut self) -> &mut ::std::string::String {
        &mut self.group_2
    }

    // Take field
    pub fn take_group_2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.group_2, ::std::string::String::new())
    }

    // string group_3 = 12;


    pub fn get_group_3(&self) -> &str {
        &self.group_3
    }
    pub fn clear_group_3(&mut self) {
        self.group_3.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_3(&mut self, v: ::std::string::String) {
        self.group_3 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group_3(&mut self) -> &mut ::std::string::String {
        &mut self.group_3
    }

    // Take field
    pub fn take_group_3(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.group_3, ::std::string::String::new())
    }

    // string line = 13;


    pub fn get_line(&self) -> &str {
        &self.line
    }
    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: ::std::string::String) {
        self.line = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut ::std::string::String {
        &mut self.line
    }

    // Take field
    pub fn take_line(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.line, ::std::string::String::new())
    }

    // string prefix = 14;


    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::string::String) {
        self.prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.prefix
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.prefix, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gl_code)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.branch_code)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.dr_bal = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.cr_bal = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.net_bal = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf_type)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.curr)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.is_gl)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alm_line)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.code_desc)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.group_2)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.group_3)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.line)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prefix)?;
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
        if !self.gl_code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.gl_code);
        }
        if !self.branch_code.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.branch_code);
        }
        if self.dr_bal != 0. {
            my_size += 9;
        }
        if self.cr_bal != 0. {
            my_size += 9;
        }
        if self.net_bal != 0. {
            my_size += 9;
        }
        if !self.cf_type.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.cf_type);
        }
        if !self.curr.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.curr);
        }
        if !self.is_gl.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.is_gl);
        }
        if !self.alm_line.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.alm_line);
        }
        if !self.code_desc.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.code_desc);
        }
        if !self.group_2.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.group_2);
        }
        if !self.group_3.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.group_3);
        }
        if !self.line.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.line);
        }
        if !self.prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.prefix);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.gl_code.is_empty() {
            os.write_string(1, &self.gl_code)?;
        }
        if !self.branch_code.is_empty() {
            os.write_string(2, &self.branch_code)?;
        }
        if self.dr_bal != 0. {
            os.write_double(3, self.dr_bal)?;
        }
        if self.cr_bal != 0. {
            os.write_double(4, self.cr_bal)?;
        }
        if self.net_bal != 0. {
            os.write_double(5, self.net_bal)?;
        }
        if !self.cf_type.is_empty() {
            os.write_string(6, &self.cf_type)?;
        }
        if !self.curr.is_empty() {
            os.write_string(7, &self.curr)?;
        }
        if !self.is_gl.is_empty() {
            os.write_string(8, &self.is_gl)?;
        }
        if !self.alm_line.is_empty() {
            os.write_string(9, &self.alm_line)?;
        }
        if !self.code_desc.is_empty() {
            os.write_string(10, &self.code_desc)?;
        }
        if !self.group_2.is_empty() {
            os.write_string(11, &self.group_2)?;
        }
        if !self.group_3.is_empty() {
            os.write_string(12, &self.group_3)?;
        }
        if !self.line.is_empty() {
            os.write_string(13, &self.line)?;
        }
        if !self.prefix.is_empty() {
            os.write_string(14, &self.prefix)?;
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

    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "gl_code",
                |m: &Account| { &m.gl_code },
                |m: &mut Account| { &mut m.gl_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "branch_code",
                |m: &Account| { &m.branch_code },
                |m: &mut Account| { &mut m.branch_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "dr_bal",
                |m: &Account| { &m.dr_bal },
                |m: &mut Account| { &mut m.dr_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "cr_bal",
                |m: &Account| { &m.cr_bal },
                |m: &mut Account| { &mut m.cr_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "net_bal",
                |m: &Account| { &m.net_bal },
                |m: &mut Account| { &mut m.net_bal },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cf_type",
                |m: &Account| { &m.cf_type },
                |m: &mut Account| { &mut m.cf_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "curr",
                |m: &Account| { &m.curr },
                |m: &mut Account| { &mut m.curr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "is_gl",
                |m: &Account| { &m.is_gl },
                |m: &mut Account| { &mut m.is_gl },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "alm_line",
                |m: &Account| { &m.alm_line },
                |m: &mut Account| { &mut m.alm_line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "code_desc",
                |m: &Account| { &m.code_desc },
                |m: &mut Account| { &mut m.code_desc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_2",
                |m: &Account| { &m.group_2 },
                |m: &mut Account| { &mut m.group_2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "group_3",
                |m: &Account| { &m.group_3 },
                |m: &mut Account| { &mut m.group_3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "line",
                |m: &Account| { &m.line },
                |m: &mut Account| { &mut m.line },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "prefix",
                |m: &Account| { &m.prefix },
                |m: &mut Account| { &mut m.prefix },
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
        self.gl_code.clear();
        self.branch_code.clear();
        self.dr_bal = 0.;
        self.cr_bal = 0.;
        self.net_bal = 0.;
        self.cf_type.clear();
        self.curr.clear();
        self.is_gl.clear();
        self.alm_line.clear();
        self.code_desc.clear();
        self.group_2.clear();
        self.group_3.clear();
        self.line.clear();
        self.prefix.clear();
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
    \n\x0fgl_amount.proto\"\xe2\x02\n\x07Account\x12\x17\n\x07gl_code\x18\
    \x01\x20\x01(\tR\x06glCode\x12\x1f\n\x0bbranch_code\x18\x02\x20\x01(\tR\
    \nbranchCode\x12\x15\n\x06dr_bal\x18\x03\x20\x01(\x01R\x05drBal\x12\x15\
    \n\x06cr_bal\x18\x04\x20\x01(\x01R\x05crBal\x12\x17\n\x07net_bal\x18\x05\
    \x20\x01(\x01R\x06netBal\x12\x17\n\x07cf_type\x18\x06\x20\x01(\tR\x06cfT\
    ype\x12\x12\n\x04curr\x18\x07\x20\x01(\tR\x04curr\x12\x13\n\x05is_gl\x18\
    \x08\x20\x01(\tR\x04isGl\x12\x19\n\x08alm_line\x18\t\x20\x01(\tR\x07almL\
    ine\x12\x1b\n\tcode_desc\x18\n\x20\x01(\tR\x08codeDesc\x12\x17\n\x07grou\
    p_2\x18\x0b\x20\x01(\tR\x06group2\x12\x17\n\x07group_3\x18\x0c\x20\x01(\
    \tR\x06group3\x12\x12\n\x04line\x18\r\x20\x01(\tR\x04line\x12\x16\n\x06p\
    refix\x18\x0e\x20\x01(\tR\x06prefixb\x06proto3\
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
