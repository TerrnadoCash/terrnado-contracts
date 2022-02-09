# Terrnado - Smart Contracts

This monorepository contains the source code for the Terrnado:

> An anonymous protocol for private transactions on [Terra](https://terra.money) blockchain.
More information about the smart contracts available on the official [documentation](https://docs.terrnado.cash/) page.

## Contracts

| Contract                       | Reference | Description | 
|--------------------------------| --------- | ------------|
| [`anonymizer`](./contracts/terrnado-anonymizer)            | [doc](https://docs.terrnado.cash/smart-contracts/deployed-contracts/anonymizer) | Terrnado Anonymizer |


## Development

### Environment Setup

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup update
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Unit / Integration Tests

Each contract contains Rust unit tests embedded within the contract source directories. You can run:

```sh
cargo test
```

### Building contracts

```sh
cargo build
```

### Compiling for production

For production builds, run the following (Windows):

```sh
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.5
```

For production builds, run the following (Mac & Linux):

```sh
docker run --rm --platform linux/arm64 -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/workspace-optimizer-arm64:0.12.5
```

This performs several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.

## License

Copyright 2022 TerrnadoCash Labs

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the
License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0. Unless required by
applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

See the License for the specific language governing permissions and limitations under the License.