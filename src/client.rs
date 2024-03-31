use futures::{stream::FuturesUnordered, TryStreamExt};
use reqwest::{Client as ReqwestClient, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Etherface API response for a signature hash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignatureResponse {
    pub items: Vec<Item>,
}

/// Item values of the Etherface API response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub hash: String,
    pub text: String,
}

const BLUE: &str = "\x1b[38;5;39m";
const GRAY: &str = "\x1b[38;5;248m";
const PURPLE: &str = "\x1b[38;5;189m";
const RESET: &str = "\x1b[0m";

impl Item {
    pub fn fmt(&self, idx: usize) -> String {
        format!("{BLUE}[{PURPLE}{}:{BLUE}{:.8}]: {GRAY}{}{RESET}", idx, self.hash, self.text)
    }
}

/// RPC response for the `eth_getCode` method.
#[derive(Deserialize)]
pub struct GetCodeResponse {
    pub result: String,
}

/// The `Client` struct encapsulates functionalities to interact with Ethereum
/// nodes via RPC and with external services to retrieve signature information.
///
/// Fields:
/// - `url`: The URL of the EVM compatible RPC server that supports the `eth_getCode` method.
/// - `inner`: The internal HTTP client used for making requests.
pub struct Client {
    url: String,
    inner: ReqwestClient,
}

impl Client {
    /// Initialize a new `Client` instance with the specified RPC server URL.
    ///
    /// Arguments:
    /// * `url`: The RPC URL to connect to.
    ///
    /// Returns:
    /// A new instance of `Client`.
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            inner: ReqwestClient::new(),
        }
    }

    /// Collects the smart contract code for a given EVM address.
    ///
    /// Retrieves the smart contract code associated with the specified Ethereum address.
    ///
    /// Arguments:
    /// * `address`: The EVM smart contract address to get the code for.
    ///
    /// Returns:
    /// A `Result` which is `Ok` containing the `GetCodeResult` on successful retrieval, or an `Err`
    /// with a `ReqwestError` in case of failure.
    pub async fn get_code(&self, address: &str) -> Result<GetCodeResponse, ReqwestError> {
        // Construct the JSON-RPC request body
        let json = format!(r#"{{"jsonrpc":"2.0","method":"eth_getCode","params":["{address}","latest"],"id":1}}"#);
        // Send the request and await the response
        let response = self.inner.post(&self.url).body(json).send().await?;
        // Parse the JSON response into a GetCodeResponse
        response.json::<GetCodeResponse>().await
    }

    /// Asynchronously retrieves a signature from the Etherface API.
    ///
    /// Get signature information associated with a given signature hash.
    /// The signature hash is expected to be a hex-encoded string and without the "0x" prefix.
    ///
    /// Arguments:
    /// * `signature`: A `String` representing the hex-encoded signature hash.
    ///
    /// Returns:
    /// A `Result` which is `Ok` containing an `Option<SignatureResponse>` if the signature
    /// was successfully retrieved, or `None` if the signature is not found. Returns an `Err`
    /// with a `ReqwestError` in case of a request failure due to network or server issues.
    async fn get_signature(&self, signature: String) -> Result<Option<SignatureResponse>, ReqwestError> {
        let url = format!("https://api.etherface.io/v1/signatures/hash/all/{}/1", signature);
        let response = self.inner.get(&url).send().await?;

        // Parse the JSON response if possible, otherwise return None
        Ok(response.json::<SignatureResponse>().await.ok())
    }

    /// Asynchronously retrieves signature information for a set of signature hashes.
    ///
    /// This method processes a collection of signature hashes and attempts to fetch
    /// the corresponding signature information for each.
    ///
    /// Arguments:
    /// * `signatures`: A `HashSet<String>` containing hex-encoded signature hashes.
    ///
    /// Returns:
    /// A `Result` containing a `Vec<Option<SignatureResponse>>`. Each element in the
    /// vector corresponds to one of the input hashes and contains either the retrieved
    /// `SignatureResponse` or `None` if no data was found for that signature.
    /// Returns an `Err`
    /// with a `ReqwestError` in case of failure in processing any of the requests.
    pub async fn get_signatures(&self, selectors: HashSet<String>) -> Result<Vec<Option<SignatureResponse>>, ReqwestError> {
        // Create futures for each signature request
        let futures = selectors.into_iter().map(|sig| self.get_signature(sig));
        // Collect the results of the futures into a vector
        let results: Result<Vec<_>, _> = FuturesUnordered::from_iter(futures).try_collect().await;

        results
    }
}
