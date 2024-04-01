use crate::client::SignatureItem;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A representation of a signature extracted from a smart contract.
///
/// The `Signature` struct contains the textual representation,
/// hash, and function selector of an EVM smart-contract signature.
///
/// Fields:
/// * `text`: The function signature.
/// * `hash`: The Keccak-256 hash of the signature.
/// * `selector`: The 4-byte function selector derived from the hash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub text: String,
    pub hash: String,
    pub selector: String,
}

impl Signature {
    /// Creates a new `Signature` instance with the given text and hash. The selector
    /// is derived from the 4 first bytes of the hash.
    ///
    /// Arguments:
    /// * `text`: The full text of the signature.
    /// * `hash`: The hash of the signature.
    ///
    /// Returns:
    /// A new `Signature` instance.
    pub fn new(text: String, hash: String) -> Self {
        Self {
            selector: format!("{:.8}", hash),
            hash,
            text,
        }
    }
}

// Constants for terminal coloring
const BLUE: &str = "\x1b[38;5;39m";
const GRAY: &str = "\x1b[38;5;248m";

impl Display for Signature {
    /// Formats a `Signature` for display.
    ///
    /// Represents the `Signature` in a formatted string, suitable for console output.
    /// The hash is displayed in blue, and the signature text in gray.
    ///
    /// Arguments:
    /// * `f`: The formatter.
    ///
    /// Returns:
    /// A `Result` as per the `std::fmt::Display` trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("{BLUE}[{:.8}]: {GRAY}{}", self.hash, self.text);
        write!(f, "{}", out)
    }
}

impl From<SignatureItem> for Signature {
    /// Converts a `SignatureItem` into a `Signature`.
    ///
    /// Creates a `Signature` from a given `SignatureItem`, typically used
    /// for converting raw signature data into a more structured format.
    ///
    /// Arguments:
    /// * `item`: The `SignatureItem` to convert.
    ///
    /// Returns:
    /// A `Signature` instance derived from the `SignatureItem`.
    fn from(item: SignatureItem) -> Self {
        Self::new(item.text, item.hash)
    }
}

impl From<&SignatureItem> for Signature {
    /// Converts a `&SignatureItem` into a `Signature`.
    /// It clones the `SignatureItem` before converting it in order to provide a clean conversion.
    ///
    /// Arguments:
    /// * `item`: The `&SignatureItem` to convert after cloning.
    ///
    /// Returns:
    /// A `Signature` instance derived from the `&SignatureItem`.
    fn from(item: &SignatureItem) -> Self {
        let item = item.clone();
        Self::new(item.text, item.hash)
    }
}
