const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_RPC_URL: &str = "https://ethereum-rpc.publicnode.com";

pub mod config;

use std::collections::HashSet;

use config::Config;

mod client;
use client::Client;

mod bytecode;
pub use bytecode::Bytecode;

mod address;
pub use address::Address;

mod signature;
pub use signature::Signature;

/// Represents the output of Sigmund's operations, including both function selectors
/// and optionally decoded signatures.
#[derive(Debug, serde::Serialize)]
pub struct SigmundOut {
    /// Decoded signatures, present if the `decode` operation is executed.
    pub signatures: Vec<Signature>,
    /// Extracted function selectors from contract bytecode.
    pub selectors: HashSet<String>,
}

impl SigmundOut {
    /// Constructs a new `SigmundOut`.
    ///
    /// Encapsulates the results of processing Ethereum contract bytecode,
    /// including the function selectors and, if applicable, the decoded signatures.
    /// That allows for a consistent but flexible output format
    /// which would allow us in the future to manipulate as needed.
    ///
    /// Arguments:
    /// * `signatures`: An `Option<Vec<Signature>>` containing the decoded signatures if provided.
    /// * `selectors`: A `HashSet<String>` containing the 4-byte selectors extracted from the bytecode.
    ///
    /// Returns:
    /// A `SigmundOut` instance containing the processed data.
    pub fn new(selectors: HashSet<String>, signatures: Vec<Signature>) -> Self {
        Self { selectors, signatures }
    }
}

/// The `Sigmund` struct encapsulates the functionality for collecting Ethereum contract
/// function selectors and optionally decoding their corresponding signatures.
///
/// It utilizes the provided configuration settings to determine the source of the bytecode
/// (either a file or an Ethereum address) and the desired operations (collecting selectors
/// and/or decoding signatures).
pub struct Sigmund {
    client: Client,
    config: Config,
}

impl Sigmund {
    /// Creates a new `Sigmund` instance from the given configuration.
    ///
    /// Initializes the `Sigmund` struct with a `Client` and the provided configuration settings.
    /// This method prepares the `Sigmund` instance for executing the defined operations.
    ///
    /// Arguments:
    /// * `config`: The `Config` struct containing settings like RPC URL, input source, and operation flags.
    ///
    /// Returns:
    /// A `Sigmund` instance ready to perform operations based on the provided configuration.
    pub fn from_config(config: Config) -> Self {
        Self {
            client: Client::new(&config.rpc_url),
            config,
        }
    }

    /// Asynchronously retrieves bytecode from the specified source.
    ///
    /// The method fetches Ethereum contract bytecode from either a specified file or
    /// an Ethereum contract address based on the configuration. It then attempts to
    /// parse and return the bytecode in a structured format.
    ///
    /// Returns:
    /// A `Result` containing `Bytecode` on success, or an error if the retrieval or parsing fails.
    async fn get_bytecode(&self) -> Result<Bytecode, Box<dyn std::error::Error>> {
        match &self.config.file {
            // Try generating bytecode from the file
            Some(file) => Bytecode::try_from(file),
            None => {
                // #![INFO]: Address will always be set since it's required in the CLI
                let address = self.config.address.to_owned().unwrap();
                // Try creating an address from the string, after verifying it's a valid EVM address
                let address = Address::try_from(address)?;
                // Get the bytecode from the RPC url using the`eth_getCode` method
                let code = self.client.get_code(&address).await?;
                // Try generating bytecode from the result
                Bytecode::try_from(code.result)
            }
        }
    }

    /// Asynchronously retrieves bytecode and processes it to extract function selectors and/or decode signatures.
    ///
    /// Depending on the configuration settings, this method fetches bytecode from either a local file
    /// or an Ethereum contract address. It then extracts 4-byte selectors and optionally decodes
    /// the corresponding signatures.
    ///
    /// The results are either printed to the console or saved to a file specified in the configuration.
    ///
    /// Returns:
    /// A `Result` indicating the success or failure of the operations.
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the bytecode from the specified source
        let bytecode = self.get_bytecode().await.map_err(|e| e.to_string())?;
        // Extract function selectors from the bytecode
        let selectors = bytecode.find_function_selectors();

        let signatures = if self.config.signatures {
            // Collect all signatures that exist in the database
            let signatures = self.client.get_signatures(&selectors, self.config.most_common).await;
            let signatures = signatures.map_err(|e| e.to_string())?;
            // Print the formatted signatures to the console
            signatures.iter().for_each(|s| println!("{}", s));

            Some(signatures)
        } else {
            // Otherwise print the selectors
            println!("{selectors:?}");
            None
        };

        // Use a Default when no signatures exist to provide a more safe
        // and consistent output format when accessed by users
        let out = SigmundOut::new(selectors, signatures.unwrap_or_default());

        // Write the output to a file if specified
        if let Some(output) = &self.config.output {
            std::fs::write(output, serde_json::to_string_pretty(&out)?)?;
        };

        Ok(())
    }
}
