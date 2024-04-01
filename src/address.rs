use std::ops::Deref;
use thiserror::Error;
/// Error types for address validation and creation.
///
/// This enum encapsulates errors that can occur when
/// working with an EVM address.
#[derive(Error, Debug)]
pub enum AddressError {
    /// Indicates the provided address has an incorrect length.
    /// The address length in Ethereum should be exactly 42 characters:
    /// 2 characters for the "0x" prefix and 40 hexadecimal characters.
    ///
    /// The usize field stores the actual length of the provided address.
    #[error("Invalid address length: {0}, expected 42 characters.")]
    Length(usize),

    /// Error for addresses that do not start with the "0x" prefix.
    #[error("Address must start with 0x")]
    Prefix,

    /// Error for addresses that are not valid hexadecimal strings
    /// after the "0x" prefix. This error means that the address contains
    /// characters outside the range of valid hexadecimal digits.
    #[error("Address must be a valid hex string")]
    Hex,
}

/// A struct representing a validated EVM address.
///
/// Wrapper that holds a valid Ethereum address.
/// The address is validated for proper length, prefix, and hexadecimal
/// format. The struct ensures that any `Address` instance represents
/// a valid address.
///
/// The `Address` struct also implements Deref to allow using it as a string reference.
///
pub struct Address {
    /// The actual string representation of the address.
    /// Stored as a `String` to own the address data.
    inner: String,
}

impl Address {
    /// Validates a given EVM address string.
    ///
    /// This function performs three key validations:
    /// 1. Checks if the address length is exactly 42 characters (0x + 20 bytes).
    /// 2. Checks if the address starts with "0x".
    /// 3. Verifies that the address is a valid hexadecimal string.
    ///
    /// # Arguments
    /// * `address` - A reference to the string to validate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the address is valid, otherwise returns an `AddressError`
    /// with details about the specific validation failure.
    #[inline]
    pub fn validate(address: &str) -> Result<(), AddressError> {
        if address.len() != 42 {
            // Return an error if the address length is not 42 characters
            return Err(AddressError::Length(address.len()));
        }

        if !address.starts_with("0x") {
            // Return an error if the address does not start with "0x"
            return Err(AddressError::Prefix);
        }

        // Attempt to decode the hexadecimal part of the address (after "0x").
        // If decoding fails, it means the address contains non-hex characters.
        if hex::decode(&address[2..]).is_err() {
            return Err(AddressError::Hex);
        }

        Ok(())
    }
}

impl TryFrom<String> for Address {
    type Error = AddressError;

    /// Attempts to create an `Address` instance from a `String`.
    ///
    /// This method utilizes the `validate` function to ensure the provided
    /// string is a valid Ethereum address. If validation passes, it creates
    /// and returns an `Address` instance. If validation fails, it returns
    /// the corresponding `AddressError`.
    ///
    /// # Arguments
    /// * `address` - The string representation of the address to be converted.
    ///
    /// # Returns
    /// `Ok(Self)` containing the address if valid, otherwise
    /// returns an `AddressError`.
    fn try_from(address: String) -> Result<Self, Self::Error> {
        Self::validate(&address)?;

        Ok(Self { inner: address })
    }
}

impl Deref for Address {
    type Target = String;

    /// Allows an `Address` instance to be treated like a `String` reference.
    ///
    /// This implementation of `Deref` enables easier access to the inner
    /// `String` object, allowing the `Address` struct to be used in contexts
    /// where a string reference is expected.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
