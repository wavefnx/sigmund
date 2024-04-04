use std::{collections::HashSet, ops::Deref, path::PathBuf};

/// A struct that in this context, represents the bytecode of a smart contract.
pub struct Bytecode {
    /// The internal representation of the bytecode as a vector of bytes.
    pub inner: Vec<u8>,
}

impl Bytecode {
    /// Find selectors in the bytecode.
    ///
    /// The bytecode pattern `PUSH4 <selector> EQ` usually occus at the initialization of the code where
    /// the function selectors are compared to the input calldata to determine the byte offset
    /// of the function to be executed.
    ///
    /// Returns:
    /// A `HashSet<String>` containing the unique hexadecimal function selectors found in the bytecode.
    ///
    /// The exact steps depend on the compiler and version, although the general pattern is:
    /// ```rs
    /// // 1. Extract the function selector from the calldata and push in the stack. (mask,shr, ...)
    /// // 2. Duplicate the selector (to be consumed by `EQ`)
    /// DUP1
    /// // 3. Push the function selector to compare against
    /// PUSH4 <selector>
    /// // 4. Compare the two selectors, push 1 if equal, 0 otherwise
    /// EQ
    /// // 5. Push the jump destination if the selectors match
    /// PUSH2 <offset>
    /// // 6. Jump to <offset> if the result of `EQ` is 1
    /// JUMPI
    /// ```
    #[inline]
    pub fn find_function_selectors(&self) -> HashSet<String> {
        let mut selectors = HashSet::new();
        let selector_size = 5;

        for idx in 0..self.inner.len().saturating_sub(selector_size) {
            // since we use `saturating_sub(pattern_length)` the next `pattern_length` bytes will be available
            if self.inner[idx] == 0x63 && self.inner[idx + selector_size] == 0x14 {
                selectors.insert(hex::encode(&self.inner[idx + 1..idx + selector_size]));
            }
        }

        selectors
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

impl TryFrom<&PathBuf> for Bytecode {
    type Error = Box<dyn std::error::Error>;

    /// Tries to create a `Bytecode` instance from a file.
    ///
    /// Read the file at the given path and attempt to decode the contents
    /// as hexadecimal bytecode.
    ///
    /// Arguments:
    /// `path`: A `PathBuf` representing the path to the file containing the bytecode.
    ///
    /// Returns:
    /// `Result<Bytecode, Box<dyn std::error::Error>>` - Ok if the decoding is successful,
    /// and an error if the file is not found or the contents are not a valid hexadecimal.
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let bytecode = std::fs::read_to_string(path)?;
        Bytecode::try_from(bytecode.trim().to_string())
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
