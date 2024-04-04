use clap::{ArgGroup, Parser};
use serde::Serialize;
use std::path::PathBuf;

/// A tool for quickly collecting function selectors and decoding signatures from on-chain EVM bytecode.
#[derive(Parser, Debug, Serialize)]
#[clap(version = crate::VERSION, author = "wavefnx @wavefnx")]
#[clap(group(ArgGroup::new("input").args(&["address", "file"]).required(true)))]
pub struct Config {
    /// Export the signatures as a JSON file
    #[clap(short = 'o', long, value_parser)]
    pub output: Option<PathBuf>,

    /// Collect all known function signatures from the contract's selectors
    #[clap(long, action = clap::ArgAction::SetTrue)]
    pub signatures: bool,

    /// The address of the EVM contract
    #[clap(long)]
    pub address: Option<String>,

    /// Path to a local file containing the contract's bytecode
    #[clap(short = 'f', long, value_parser)]
    pub file: Option<PathBuf>,

    /// Collect all four-byte pushes (fn, err, ...), including non-selectors
    #[clap(long, action = clap::ArgAction::SetTrue)]
    pub deep: bool,

    /// Return all available signature matches for each selector
    #[clap(long, action = clap::ArgAction::SetTrue, requires = "signatures")]
    pub all_matches: bool,

    /// To use your own Node or collect bytecode from a different network, provide the relevant RPC URL.
    #[clap(long, default_value = crate::DEFAULT_RPC_URL)]
    pub rpc_url: String,
}
