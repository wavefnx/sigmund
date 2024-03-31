use sigmund::Address;

#[test]
fn it_creates_address_from_valid_input() {
    let address = String::from("0x1234567890123456789012345678901234567890");
    let addr = Address::try_from(address).unwrap();
    assert_eq!(addr.to_owned(), "0x1234567890123456789012345678901234567890");
}

#[test]
fn it_accepts_valid_address_input() {
    let address = String::from("0x1234567890123456789012345678901234567890");
    assert!(Address::validate(&address).is_ok());
}

#[test]
fn it_fails_on_invalid_address_length() {
    let addr = "0x123".to_string();
    assert!(Address::validate(&addr).is_err())
}

#[test]
fn it_fails_on_invalid_address_prefix() {
    let addr = "123456789012345678901234567890123456789012".to_string();
    assert!(Address::validate(&addr).is_err())
}

#[test]
fn it_fails_on_invalid_address_hex() {
    let addr = "0x123456789012345678901234567890123456789z".to_string();
    assert!(Address::validate(&addr).is_err())
}
