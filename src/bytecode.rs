use std::{collections::HashSet, ops::Deref};

/// A struct that in this context, represents the bytecode of a smart contract.
pub struct Bytecode {
    /// The internal representation of the bytecode as a vector of bytes.
    pub inner: Vec<u8>,
}

impl Bytecode {
    /// Finds function selectors within the bytecode.
    ///
    /// Scan the bytecode for specific patterns that correspond to
    /// the start of function selectors. It returns a set of found selectors
    /// as hexadecimal strings.
    /// ```rs
    /// JUMPI (0x57)
    /// DUP1  (0x80)
    /// PUSH4 (0x63)
    /// ```
    /// Returns:
    /// A `HashSet<String>` containing the unique hexadecimal function selectors found in the bytecode.
    #[inline]
    pub fn find_function_selectors(&self) -> HashSet<String> {
        self.inner
            .windows(7)
            .filter_map(|window| {
                // Check if the window starts with the specific pattern
                if window.starts_with(&[0x57, 0x80, 0x63]) {
                    // If yes, extract the next 4 bytes as the selector
                    Some(hex::encode(&window[3..7]))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl TryFrom<String> for Bytecode {
    type Error = Box<dyn std::error::Error>;

    /// Tries to create a `Bytecode` instance from a hexadecimal string.
    ///
    /// Trim the "0x" prefix, if present, at the start,
    /// and attempt to decode the remaining hexadecimal string into bytes.
    ///
    /// Arguments:
    /// `bytecode`: A string slice representing the hexadecimal bytecode.
    ///
    /// Returns:
    /// `Result<Bytecode, Box<dyn std::error::Error>>` - Ok if the decoding is successful,
    /// and an error if the string is not a valid hexadecimal.
    fn try_from(bytecode: String) -> Result<Self, Self::Error> {
        let inner = hex::decode(bytecode.trim_start_matches("0x"))?;
        Ok(Self { inner })
    }
}

impl Deref for Bytecode {
    type Target = Vec<u8>;

    /// Provides dereferencing to the inner `Vec<u8>`.
    ///
    /// Allows direct access to the underlying byte vector.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
