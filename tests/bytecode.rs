use sigmund::Bytecode;

#[test]
fn it_creates_bytecode_from_valid_input() {
    let valid_hex = "0x57638063abcd1234".to_string();
    let bytecode = Bytecode::try_from(valid_hex);
    assert!(bytecode.is_ok(), "Failed to create Bytecode from a valid hexadecimal string.");
}

#[test]
fn it_fails_to_create_bytecode_from_invalid_input() {
    let invalid_hex = "0xg1234567".to_string();
    let bytecode = Bytecode::try_from(invalid_hex);
    assert!(bytecode.is_err(), "Bytecode was created from an invalid hexadecimal string.");
}

#[test]
fn it_finds_function_signatures() {
    let hex_with_signatures = "0xe01c63ddc632621461".to_string();
    let bytecode = Bytecode::try_from(hex_with_signatures).unwrap();
    let signatures = bytecode.find_function_selectors(false);
    assert_eq!(signatures.len(), 1);
    assert!(signatures.contains("ddc63262"));
}

#[test]
fn it_does_not_find_signatures_when_none_exist() {
    let hex_no_signatures = "0x12345678".to_string();
    let bytecode = Bytecode::try_from(hex_no_signatures).unwrap();
    let signatures = bytecode.find_function_selectors(false);
    assert!(signatures.is_empty());
}
