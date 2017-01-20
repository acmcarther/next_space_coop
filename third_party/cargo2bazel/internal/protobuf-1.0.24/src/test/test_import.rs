// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::test_import_imported::ImportedMessage;
use super::test_import_imported::ImportedEnum;

#[derive(Clone,Default)]
pub struct ContainsImported {
    // message fields
    imported_message: ::protobuf::SingularPtrField<ImportedMessage>,
    imported_enum: ::std::option::Option<ImportedEnum>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ContainsImported {
    pub fn new() -> ContainsImported {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainsImported {
        static mut instance: ::protobuf::lazy::Lazy<ContainsImported> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainsImported,
        };
        unsafe {
            instance.get(|| {
                ContainsImported {
                    imported_message: ::protobuf::SingularPtrField::none(),
                    imported_enum: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ImportedMessage imported_message = 1;

    pub fn clear_imported_message(&mut self) {
        self.imported_message.clear();
    }

    pub fn has_imported_message(&self) -> bool {
        self.imported_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_imported_message(&mut self, v: ImportedMessage) {
        self.imported_message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_imported_message<'a>(&'a mut self) -> &'a mut ImportedMessage {
        if self.imported_message.is_none() {
            self.imported_message.set_default();
        };
        self.imported_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_imported_message(&mut self) -> ImportedMessage {
        self.imported_message.take().unwrap_or_else(|| ImportedMessage::new())
    }

    pub fn get_imported_message<'a>(&'a self) -> &'a ImportedMessage {
        self.imported_message.as_ref().unwrap_or_else(|| ImportedMessage::default_instance())
    }

    // optional .ImportedEnum imported_enum = 2;

    pub fn clear_imported_enum(&mut self) {
        self.imported_enum = ::std::option::Option::None;
    }

    pub fn has_imported_enum(&self) -> bool {
        self.imported_enum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_imported_enum(&mut self, v: ImportedEnum) {
        self.imported_enum = ::std::option::Option::Some(v);
    }

    pub fn get_imported_enum<'a>(&self) -> ImportedEnum {
        self.imported_enum.unwrap_or(ImportedEnum::SOMETHING)
    }
}

impl ::protobuf::Message for ContainsImported {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.imported_message.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.imported_enum = ::std::option::Option::Some(tmp);
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
        for value in self.imported_message.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.imported_enum.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.imported_message.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.imported_enum {
            try!(os.write_enum(2, v as i32));
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
        ::std::any::TypeId::of::<ContainsImported>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContainsImported {
    fn new() -> ContainsImported {
        ContainsImported::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContainsImported>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "imported_message",
                    ContainsImported::has_imported_message,
                    ContainsImported::get_imported_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "imported_enum",
                    ContainsImported::has_imported_enum,
                    ContainsImported::get_imported_enum,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContainsImported>(
                    "ContainsImported",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContainsImported {
    fn clear(&mut self) {
        self.clear_imported_message();
        self.clear_imported_enum();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ContainsImported {
    fn eq(&self, other: &ContainsImported) -> bool {
        self.imported_message == other.imported_message &&
        self.imported_enum == other.imported_enum &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ContainsImported {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1a, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74,
    0x5f, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x64, 0x0a, 0x10, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x49, 0x6d, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x12, 0x2a, 0x0a, 0x10, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x5f,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x49, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x24, 0x0a, 0x0d, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x75, 0x6d,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0d, 0x2e, 0x49, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x45, 0x6e, 0x75, 0x6d, 0x4a, 0xb5, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x05, 0x01,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x00, 0x07, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x03, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x1d, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x04, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x04, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x1a, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x2a, 0x2b,
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
