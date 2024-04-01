
    
<div align="center">
    
![sigmund-logo](https://github.com/wavefnx/sigmund/assets/157986149/f96b0b88-7e8d-49a4-9d28-b6a14f5793a9)
</div>

<div align="center"> 
    
[Overview](#Overview) | [Disclaimer](#Disclaimer)  | [Tests](#Tests) | [Installation](#Installation) | [Usage](#Usage) | [Examples](#Examples) | [Aknowledgements](#Aknowledgements) | [License](#License)
</div>


<div align="center">
    
[![CI](https://img.shields.io/github/actions/workflow/status/wavefnx/sigmund/ci.yml?style=flat-square&label=CI&labelColor=%23343940&color=%2340C057)](https://github.com/wavefnx/sigmund/actions/workflows/ci.yml)
[![MPL-2.0](https://img.shields.io/github/license/wavefnx/sigmund?style=flat-square&color=blue&label=)](LICENSE)
</div>

## Overview
`Sigmund` provides a CLI that allows users to collect function selectors from any EVM network and for any smart-contract, verified or not. It does so by searching for specific bytecode patterns which allows for efficient and EVM native interactions without the need for any APIs or ABI. Additionally, it provides the possibility to decode those function selectors and get the function signatures if they are known.

It's purpose is to be used as a tool for recon, analysis, general research and experimentation. 

## Disclaimer
This library is in early development stages and subject to potential breaking changes.
Backward compatibility is not guaranteed and the package is intentionally not published on crates.io until and if there's an `alpha` release in the future.

Contributions are welcome. Users are encouraged to submit pull requests, fork, or alter the code in accordance with the terms outlined in the [LICENSE](LICENSE).

It's suggested that the `decode` argument is used deliberately to not overload the external function signature API.

## Tests
To run the implemented tests, execute the following command at the root of the repository:  
```rust
cargo test
```

## Installation
You can currently build from source by running the following command in the root of the repository:
```rust
cargo build --release
```

## Usage
```
A tool for quickly collecting function selectors and decoding signatures from on-chain EVM bytecode

Usage: sigmund [OPTIONS] <--address <ADDRESS>|--file <FILE>>

Options:
  -o, --output <OUTPUT>    Export the signatures as a JSON file
      --signatures         Collect all known function signatures from the contract's selectors
      --address <ADDRESS>  The address of the EVM contract
  -f, --file <FILE>        Path to a local file containing the contract's bytecode
      --most-probable      Return only the signatures with the highest probability of being correct
      --rpc-url <RPC_URL>  To use your own Node or collect bytecode from a different network, provide the relevant RPC URL [default: https://ethereum-rpc.publicnode.com]
  -h, --help               Print help
  -V, --version            Print version
```

## Examples

```sh
# Get function selectors for an unverified contract on Mainnet
sigmund --address 0x0000130d512ca69ca38add5b9ab2f9deff95c882
# {"8da5cb5b", "3aeebedb", "c6723cc9", ...}

# Get function selectors from a local file containing the bytecode
sigmund --file bytecode.txt
# {"7b6e0f15", "3aeebedb", "b603cd80", ...}

# Get function signatures for an unverified contract on Mainnet
# Similarly, for a local file just point to that file's path
sigmund --signatures --address 0x0000130d512ca69ca38add5b9ab2f9deff95c882
# [8da5cb5b]: owner()
# [c86283c8]: withdrawTo(uint256,address)

# To generate a `json` output, you can use any combination 
# as long as the input <--file | --address> is provided
sigmund --file bytecode.txt --output example.json
# { "selectors": Vec<String> , "signatures": <Vec<Signatures>) }

# You can use `Sigmund` in any EVM network
# by setting the --rpc-url to the relevant provider
sigmund --rpc-url <rpc-provider> <--signatures?> <--file <path>| --address <address>> 
```

## Aknowledgements
The underlying function signature API is [Etherface](https://github.com/volsa/etherface).

## License
This library is released under the terms of the [Mozilla Public License](https://www.mozilla.org/en-US/MPL/) version 2.0. See [LICENSE](LICENSE).