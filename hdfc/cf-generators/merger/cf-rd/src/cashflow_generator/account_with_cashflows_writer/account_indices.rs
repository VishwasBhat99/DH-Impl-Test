// This file is generated by rust-protobuf 2.0.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

#[derive(PartialEq, Clone, Default)]
pub  struct Index {
    // message fields
    pub  key: ::std::string::String,
    pub  offset: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Index {
    pub  fn new() -> Index {
        ::std::default::Default::default()
    }

    // string key = 1;

    pub  fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub  fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub  fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub  fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub  fn get_key(&self) -> &str {
        &self.key
    }

    // int64 offset = 2;

    pub  fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub  fn set_offset(&mut self, v: i64) {
        self.offset = v;
    }

    pub  fn get_offset(&self) -> i64 {
        self.offset
    }
}

impl ::protobuf::Message for Index {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_int64()?;
                    self.offset = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if self.offset != 0 {
            my_size +=
                ::protobuf::rt::value_size(2, self.offset, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if self.offset != 0 {
            os.write_int64(2, self.offset)?;
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

    fn new() -> Index {
        Index::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "key", |m: &Index| &m.key, |m: &mut Index| &mut m.key
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeInt64,
                >(
                    "offset",
                    |m: &Index| &m.offset,
                    |m: &mut Index| &mut m.offset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Index>(
                    "Index",
                    fields,
                    file_descriptor_proto(),
                )
            })
        }
    }

    fn default_instance() -> &'static Index {
        static mut instance: ::protobuf::lazy::Lazy<Index> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Index,
        };
        unsafe { instance.get(Index::new) }
    }
}

impl ::protobuf::Clear for Index {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Index {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Index {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x07a.proto\"1\n\x05Index\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12\x16\n\x06offset\x18\x02\x20\x01(\x03R\x06offsetb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<
    ::protobuf::descriptor::FileDescriptorProto,
> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub  fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
