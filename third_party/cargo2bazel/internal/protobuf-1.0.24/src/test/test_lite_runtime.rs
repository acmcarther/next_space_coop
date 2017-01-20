// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default,Debug)]
pub struct TestLiteRuntime {
    // message fields
    v: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestLiteRuntime {
    pub fn new() -> TestLiteRuntime {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestLiteRuntime {
        static mut instance: ::protobuf::lazy::Lazy<TestLiteRuntime> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestLiteRuntime,
        };
        unsafe {
            instance.get(|| {
                TestLiteRuntime {
                    v: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 v = 1;

    pub fn clear_v(&mut self) {
        self.v = ::std::option::Option::None;
    }

    pub fn has_v(&self) -> bool {
        self.v.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v(&mut self, v: i32) {
        self.v = ::std::option::Option::Some(v);
    }

    pub fn get_v<'a>(&self) -> i32 {
        self.v.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestLiteRuntime {
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
                    self.v = ::std::option::Option::Some(tmp);
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
        for value in self.v.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.v {
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
        ::std::any::TypeId::of::<TestLiteRuntime>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestLiteRuntime {
    fn new() -> TestLiteRuntime {
        TestLiteRuntime::new()
    }
}

impl ::protobuf::Clear for TestLiteRuntime {
    fn clear(&mut self) {
        self.clear_v();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestLiteRuntime {
    fn eq(&self, other: &TestLiteRuntime) -> bool {
        self.v == other.v &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum EnumTestLiteRuntime {
    ONE = 1,
    TWO = 2,
}

impl ::protobuf::ProtobufEnum for EnumTestLiteRuntime {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnumTestLiteRuntime> {
        match value {
            1 => ::std::option::Option::Some(EnumTestLiteRuntime::ONE),
            2 => ::std::option::Option::Some(EnumTestLiteRuntime::TWO),
            _ => ::std::option::Option::None
        }
    }
}

impl ::std::marker::Copy for EnumTestLiteRuntime {
}
