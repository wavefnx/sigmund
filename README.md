
    
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
A tool for collecting and decoding function selectors and signatures from on-chain EVM bytecode.

Usage: sigmund [OPTIONS] <--selectors|--decode> <address>

Arguments:
  <address>  The address of the EVM contract

Options:
  -s, --selectors          Collect the 4-byte selectors from a contract's bytecode
  -d, --decode             Get the decoded functions signatures from a contract's bytecode
      --rpc-url <rpc-url>  Optional argument for specifying the RPC URL (defaults to Mainnet)
  -h, --help               Print help
  -V, --version            Print version
```

## Examples

```sh
# Get the function selectors for an unverified contract on Mainnet
sigmund --selectors 0x0000130d512ca69ca38add5b9ab2f9deff95c882
# {"8da5cb5b", "3aeebedb", "c6723cc9", ...}

# Get the function signatures for an unverified contract on Mainnet
sigmund --decode 0x0000130d512ca69ca38add5b9ab2f9deff95c882
# [0:8da5cb5b]: owner()
# [0:c86283c8]: withdrawTo(uint256,address)

# You can use `Sigmund` in any EVM network by setting the --rpc-url to that provider
sigmund <--selectors | --decode> --rpc-url <rpc_provider> <contract_address>
```

## Aknowledgements
The underlying function signature API is [Etherface](https://github.com/volsa/etherface).

## License
This library is released under the terms of the [Mozilla Public License](https://www.mozilla.org/en-US/MPL/) version 2.0. See [LICENSE](LICENSE).