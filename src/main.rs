// Import necessary modules from the `sigmund` crate.
use sigmund::{Address, Bytecode, Client};

// Default RPC URL for Ethereum node interaction.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build and parse the command-line interface arguments.
    let matches = sigmund::cli::build_cli().get_matches();

    // Retrieve and validate the contract's `Address`.
    let address = matches.get_one::<String>("address").expect("address should be provided");
    let address = Address::try_from(address.clone()).map_err(|e| e.to_string())?;

    // Determine the RPC URL to use, defaulting to `DEFAULT_RPC_URL` if not provided.
    // [INFO]: This is guaranteed to be present because the default value is set in the CLI.
    let url = matches.get_one::<String>("rpc-url").unwrap();

    // Initialize the Ethereum RPC client with the specified URL.
    let client = Client::new(url);

    // Fetch the smart contract code for the given address.
    let code = client.get_code(&address).await?;

    // Convert the fetched code into `Bytecode` format.
    let bytecode = Bytecode::try_from(code.result)?;

    // Extract the selectors from the bytecode.
    let selectors = bytecode.find_function_selectors();

    // If the 'selectors' flag is set, print the found function selectors.
    if matches.get_flag("selectors") {
        println!("{:?}", selectors)
    }

    // If the 'decode' flag is set, fetch and decode each signature.
    if matches.get_flag("decode") {
        // Retrieve signature information for each found signature.
        let responses = client.get_signatures(selectors).await?;

        responses.iter().filter_map(|result| result.as_ref()).for_each(|result| {
            result.items.iter().enumerate().for_each(|(i, v)| println!("{}", v.fmt(i)));
        });
    }

    Ok(())
}
