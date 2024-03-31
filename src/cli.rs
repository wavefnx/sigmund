use clap::{Arg, ArgAction, ArgGroup, Command};

pub fn build_cli() -> Command {
    // Argument to fetch 4-byte selectors from bytecode.
    // Enabled with the '-s' or '--selectors' flag.
    let selectors_arg = Arg::new("selectors")
        .short('s')
        .long("selectors")
        .help("Collect the 4-byte selectors from a contract's bytecode")
        .action(ArgAction::SetTrue); // Sets a true flag when this argument is used.

    // Argument to decode functions from bytecode.
    // Enabled with the '-d' or '--decode' flag.
    let decode_arg = Arg::new("decode")
        .short('d')
        .long("decode")
        .help("Get the decoded functions signatures from a contract's bytecode")
        .action(ArgAction::SetTrue); // Sets a true flag when this argument is used.

    // Argument for specifying the EVM contract address.
    // This is a positional argument and is required.
    let address_arg = Arg::new("address")
        .help("The address of the EVM contract")
        .index(1) // Indicates that this is the first positional argument.
        .required(true); // Marks the argument as required.

    // Optional argument for specifying a different RPC URL.
    let rpc_url_arg = Arg::new("rpc-url")
        .long("rpc-url")
        .default_value(crate::DEFAULT_RPC_URL)
        .help("Optional argument for specifying the RPC URL")
        .required(false);

    // Group to enforce that either 'selectors' or 'decode' is provided.
    let mode_group = ArgGroup::new("mode")
        .args(["selectors", "decode"])
        // At least one of the arguments in the group must be provided.
        .required(true)
        // Only one argument in the group can be used at a time.
        .multiple(false);

    Command::new("Sigmund")
        .version("0.1.0")
        .author("wavefnx @wavefnx")
        .about("A tool for collecting and decoding function selectors and signatures from on-chain EVM bytecode.")
        .arg(selectors_arg)
        .arg(decode_arg)
        .arg(rpc_url_arg)
        .arg(address_arg)
        .group(mode_group)
}
