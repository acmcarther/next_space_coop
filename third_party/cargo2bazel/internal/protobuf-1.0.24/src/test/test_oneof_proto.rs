// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct MessageForOneof {
    // message fields
    f: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl MessageForOneof {
    pub fn new() -> MessageForOneof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageForOneof {
        static mut instance: ::protobuf::lazy::Lazy<MessageForOneof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageForOneof,
        };
        unsafe {
            instance.get(|| {
                MessageForOneof {
                    f: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 f = 1;

    pub fn clear_f(&mut self) {
        self.f = ::std::option::Option::None;
    }

    pub fn has_f(&self) -> bool {
        self.f.is_some()
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: i32) {
        self.f = ::std::option::Option::Some(v);
    }

    pub fn get_f<'a>(&self) -> i32 {
        self.f.unwrap_or(0)
    }
}

impl ::protobuf::Message for MessageForOneof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.f = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.f.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.f {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<MessageForOneof>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageForOneof {
    fn new() -> MessageForOneof {
        MessageForOneof::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageForOneof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "f",
                    MessageForOneof::has_f,
                    MessageForOneof::get_f,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageForOneof>(
                    "MessageForOneof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageForOneof {
    fn clear(&mut self) {
        self.clear_f();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MessageForOneof {
    fn eq(&self, other: &MessageForOneof) -> bool {
        self.f == other.f &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MessageForOneof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestOneof {
    // message fields
    s: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    one: ::std::option::Option<TestOneof_oneof_one>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum TestOneof_oneof_one {
    double_field(f64),
    float_field(f32),
    int32_field(i32),
    int64_field(i64),
    uint32_field(u32),
    uint64_field(u64),
    sint32_field(i32),
    sint64_field(i64),
    fixed32_field(u32),
    fixed64_field(u64),
    sfixed32_field(i32),
    sfixed64_field(i64),
    bool_field(bool),
    string_field(::std::string::String),
    bytes_field(::std::vec::Vec<u8>),
    enum_field(EnumForOneof),
    message_field(MessageForOneof),
}

impl TestOneof {
    pub fn new() -> TestOneof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestOneof {
        static mut instance: ::protobuf::lazy::Lazy<TestOneof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestOneof,
        };
        unsafe {
            instance.get(|| {
                TestOneof {
                    s: ::protobuf::SingularField::none(),
                    one: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string s = 29;

    pub fn clear_s(&mut self) {
        self.s.clear();
    }

    pub fn has_s(&self) -> bool {
        self.s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::string::String) {
        self.s = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.s.is_none() {
            self.s.set_default();
        };
        self.s.as_mut().unwrap()
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::string::String {
        self.s.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s<'a>(&'a self) -> &'a str {
        match self.s.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_double_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::double_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::double_field(v))
    }

    pub fn get_double_field<'a>(&self) -> f64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::double_field(v)) => v,
            _ => 0.,
        }
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_float_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::float_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::float_field(v))
    }

    pub fn get_float_field<'a>(&self) -> f32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::float_field(v)) => v,
            _ => 0.,
        }
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_int32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::int32_field(v))
    }

    pub fn get_int32_field<'a>(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int32_field(v)) => v,
            _ => 0,
        }
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_int64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::int64_field(v))
    }

    pub fn get_int64_field<'a>(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int64_field(v)) => v,
            _ => 0,
        }
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_uint32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(v))
    }

    pub fn get_uint32_field<'a>(&self) -> u32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(v)) => v,
            _ => 0,
        }
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_uint64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(v))
    }

    pub fn get_uint64_field<'a>(&self) -> u64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(v)) => v,
            _ => 0,
        }
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sint32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(v))
    }

    pub fn get_sint32_field<'a>(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(v)) => v,
            _ => 0,
        }
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sint64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(v))
    }

    pub fn get_sint64_field<'a>(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(v)) => v,
            _ => 0,
        }
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(v))
    }

    pub fn get_fixed32_field<'a>(&self) -> u32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(v)) => v,
            _ => 0,
        }
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(v))
    }

    pub fn get_fixed64_field<'a>(&self) -> u64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(v)) => v,
            _ => 0,
        }
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(v))
    }

    pub fn get_sfixed32_field<'a>(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(v)) => v,
            _ => 0,
        }
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(v))
    }

    pub fn get_sfixed64_field<'a>(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(v)) => v,
            _ => 0,
        }
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_bool_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bool_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::bool_field(v))
    }

    pub fn get_bool_field<'a>(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bool_field(v)) => v,
            _ => false,
        }
    }

    // optional string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_string_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_field<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if let ::std::option::Option::Some(TestOneof_oneof_one::string_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(::std::string::String::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        if self.has_string_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::string_field(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string_field<'a>(&'a self) -> &'a str {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(ref v)) => v,
            _ => "",
        }
    }

    // optional bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_bytes_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_field<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(::std::vec::Vec::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_bytes_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_bytes_field<'a>(&'a self) -> &'a [u8] {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(ref v)) => v,
            _ => &[],
        }
    }

    // optional .EnumForOneof enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_enum_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::enum_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: EnumForOneof) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::enum_field(v))
    }

    pub fn get_enum_field<'a>(&self) -> EnumForOneof {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::enum_field(v)) => v,
            _ => EnumForOneof::A,
        }
    }

    // optional .MessageForOneof message_field = 17;

    pub fn clear_message_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_message_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_message_field(&mut self, v: MessageForOneof) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_field<'a>(&'a mut self) -> &'a mut MessageForOneof {
        if let ::std::option::Option::Some(TestOneof_oneof_one::message_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(MessageForOneof::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_message_field(&mut self) -> MessageForOneof {
        if self.has_message_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::message_field(v)) => v,
                _ => panic!(),
            }
        } else {
            MessageForOneof::new()
        }
    }

    pub fn get_message_field<'a>(&'a self) -> &'a MessageForOneof {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(ref v)) => v,
            _ => MessageForOneof::default_instance(),
        }
    }
}

impl ::protobuf::Message for TestOneof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s.set_default();
                    try!(is.read_string_into(tmp))
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::double_field(try!(is.read_double())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::float_field(try!(is.read_float())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::int32_field(try!(is.read_int32())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::int64_field(try!(is.read_int64())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(try!(is.read_uint32())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(try!(is.read_uint64())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(try!(is.read_sint32())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(try!(is.read_sint64())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(try!(is.read_fixed32())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(try!(is.read_fixed64())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(try!(is.read_sfixed32())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(try!(is.read_sfixed64())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::bool_field(try!(is.read_bool())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(try!(is.read_string())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(try!(is.read_bytes())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::enum_field(try!(is.read_enum())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(try!(is.read_message())));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.s.iter() {
            my_size += ::protobuf::rt::string_size(29, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.one {
            match v {
                &TestOneof_oneof_one::double_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::float_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::int32_field(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::int64_field(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::uint32_field(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::uint64_field(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::sint32_field(v) => {
                    my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::sint64_field(v) => {
                    my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::fixed32_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::fixed64_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::sfixed32_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::sfixed64_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::bool_field(v) => {
                    my_size += 2;
                },
                &TestOneof_oneof_one::string_field(ref v) => {
                    my_size += ::protobuf::rt::string_size(14, &v);
                },
                &TestOneof_oneof_one::bytes_field(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(15, &v);
                },
                &TestOneof_oneof_one::enum_field(v) => {
                    my_size += ::protobuf::rt::enum_size(16, v);
                },
                &TestOneof_oneof_one::message_field(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s.as_ref() {
            try!(os.write_string(29, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.one {
            match v {
                &TestOneof_oneof_one::double_field(v) => {
                    try!(os.write_double(1, v));
                },
                &TestOneof_oneof_one::float_field(v) => {
                    try!(os.write_float(2, v));
                },
                &TestOneof_oneof_one::int32_field(v) => {
                    try!(os.write_int32(3, v));
                },
                &TestOneof_oneof_one::int64_field(v) => {
                    try!(os.write_int64(4, v));
                },
                &TestOneof_oneof_one::uint32_field(v) => {
                    try!(os.write_uint32(5, v));
                },
                &TestOneof_oneof_one::uint64_field(v) => {
                    try!(os.write_uint64(6, v));
                },
                &TestOneof_oneof_one::sint32_field(v) => {
                    try!(os.write_sint32(7, v));
                },
                &TestOneof_oneof_one::sint64_field(v) => {
                    try!(os.write_sint64(8, v));
                },
                &TestOneof_oneof_one::fixed32_field(v) => {
                    try!(os.write_fixed32(9, v));
                },
                &TestOneof_oneof_one::fixed64_field(v) => {
                    try!(os.write_fixed64(10, v));
                },
                &TestOneof_oneof_one::sfixed32_field(v) => {
                    try!(os.write_sfixed32(11, v));
                },
                &TestOneof_oneof_one::sfixed64_field(v) => {
                    try!(os.write_sfixed64(12, v));
                },
                &TestOneof_oneof_one::bool_field(v) => {
                    try!(os.write_bool(13, v));
                },
                &TestOneof_oneof_one::string_field(ref v) => {
                    try!(os.write_string(14, v));
                },
                &TestOneof_oneof_one::bytes_field(ref v) => {
                    try!(os.write_bytes(15, v));
                },
                &TestOneof_oneof_one::enum_field(v) => {
                    try!(os.write_enum(16, v as i32));
                },
                &TestOneof_oneof_one::message_field(ref v) => {
                    try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestOneof>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestOneof {
    fn new() -> TestOneof {
        TestOneof::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestOneof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "s",
                    TestOneof::has_s,
                    TestOneof::get_s,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_field",
                    TestOneof::has_double_field,
                    TestOneof::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float_field",
                    TestOneof::has_float_field,
                    TestOneof::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32_field",
                    TestOneof::has_int32_field,
                    TestOneof::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64_field",
                    TestOneof::has_int64_field,
                    TestOneof::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32_field",
                    TestOneof::has_uint32_field,
                    TestOneof::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64_field",
                    TestOneof::has_uint64_field,
                    TestOneof::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sint32_field",
                    TestOneof::has_sint32_field,
                    TestOneof::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_field",
                    TestOneof::has_sint64_field,
                    TestOneof::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fixed32_field",
                    TestOneof::has_fixed32_field,
                    TestOneof::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fixed64_field",
                    TestOneof::has_fixed64_field,
                    TestOneof::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sfixed32_field",
                    TestOneof::has_sfixed32_field,
                    TestOneof::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sfixed64_field",
                    TestOneof::has_sfixed64_field,
                    TestOneof::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool_field",
                    TestOneof::has_bool_field,
                    TestOneof::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string_field",
                    TestOneof::has_string_field,
                    TestOneof::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_field",
                    TestOneof::has_bytes_field,
                    TestOneof::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "enum_field",
                    TestOneof::has_enum_field,
                    TestOneof::get_enum_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "message_field",
                    TestOneof::has_message_field,
                    TestOneof::get_message_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestOneof>(
                    "TestOneof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestOneof {
    fn clear(&mut self) {
        self.clear_s();
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.clear_enum_field();
        self.clear_message_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestOneof {
    fn eq(&self, other: &TestOneof) -> bool {
        self.s == other.s &&
        self.one == other.one &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestOneof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum EnumForOneof {
    A = 10,
}

impl ::protobuf::ProtobufEnum for EnumForOneof {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnumForOneof> {
        match value {
            10 => ::std::option::Option::Some(EnumForOneof::A),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EnumForOneof>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnumForOneof", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnumForOneof {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x6f, 0x6e, 0x65, 0x6f, 0x66, 0x5f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1c, 0x0a, 0x0f, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x12, 0x09, 0x0a, 0x01, 0x66,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0xd5, 0x03, 0x0a, 0x09, 0x54, 0x65, 0x73, 0x74, 0x4f,
    0x6e, 0x65, 0x6f, 0x66, 0x12, 0x09, 0x0a, 0x01, 0x73, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x16, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x01, 0x48, 0x00, 0x12, 0x15, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x48, 0x00, 0x12, 0x15,
    0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x48, 0x00, 0x12, 0x15, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c,
    0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0d, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c,
    0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x11, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x12, 0x48, 0x00, 0x12, 0x17, 0x0a, 0x0d,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x07, 0x48, 0x00, 0x12, 0x17, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x48, 0x00, 0x12, 0x18,
    0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x48, 0x00, 0x12, 0x18, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78,
    0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10,
    0x48, 0x00, 0x12, 0x14, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x12, 0x16, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x12, 0x15, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x0f, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x12, 0x23, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x45, 0x6e,
    0x75, 0x6d, 0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x48, 0x00, 0x12, 0x29, 0x0a, 0x0d,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x46, 0x6f, 0x72,
    0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x48, 0x00, 0x42, 0x05, 0x0a, 0x03, 0x6f, 0x6e, 0x65, 0x2a, 0x15,
    0x0a, 0x0c, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x12, 0x05,
    0x0a, 0x01, 0x41, 0x10, 0x0a, 0x4a, 0xc6, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d, 0x01,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x00, 0x00, 0x02, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x00, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x01, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x01, 0x04, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x01, 0x08,
    0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x05, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x13, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x17, 0x18, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x14, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x01, 0x08, 0x00, 0x12, 0x04, 0x0a, 0x04, 0x1c, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x0a, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0b, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x0f, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0c, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0d,
    0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x0e, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x0e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x10, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x10, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x10, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x11, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x11, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x11, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08,
    0x12, 0x03, 0x12, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x12, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x0f,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x12, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x13, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x13, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x13, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03,
    0x14, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x14, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x10, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x14, 0x20, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x15, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0b, 0x05, 0x12, 0x03, 0x15, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x15, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12,
    0x03, 0x15, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x16, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x16, 0x11, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x16, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x0d, 0x12, 0x03, 0x17, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d,
    0x05, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12,
    0x03, 0x17, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x17,
    0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x03, 0x18, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x18, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x18, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x0f, 0x12, 0x03, 0x19, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x05, 0x12,
    0x03, 0x19, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x19,
    0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x19, 0x1c, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x10, 0x12, 0x03, 0x1a, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x10, 0x06, 0x12, 0x03, 0x1a, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1a, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x10, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x11, 0x12,
    0x03, 0x1b, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x06, 0x12, 0x03, 0x1b,
    0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x03, 0x12, 0x03, 0x1b, 0x28, 0x2a,
];

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
