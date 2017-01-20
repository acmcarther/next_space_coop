use pb_test_enum_alias::*;

use protobuf::ProtobufEnum;

use test::*;

#[test]
fn test_enum() {
    assert_eq!(10, EnumWithAlias::A.value());
    assert_eq!(10, EnumWithAlias::A_AGAIN.value());
    assert_eq!(&[EnumWithAlias::A, EnumWithAlias::B, EnumWithAlias::A_AGAIN], EnumWithAlias::values());
    // broken now
    //assert_eq!(EnumWithAlias::A, EnumWithAlias::A_AGAIN);
}

#[test]
fn test_enum_in_message() {
    let mut m = TestEnumWithAlias::new();
    m.set_en(EnumWithAlias::A);
    test_serialize_deserialize("08 0a", &m);
}
