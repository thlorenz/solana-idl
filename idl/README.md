# solana-idl

Extracts Rust Solana IDL types definitions de/serializable with _serde_ extracted from
[shank](https://crates.io/crates/shank) or anchor.

Handles multiple IDL versions to provide the _classic_ IDL structure.

## Example

```rust
use solana_idl::try_extract_classic_idl;
let idl_json = // read JSON from somewhere
let classic_idl = try_extract_classic_idl(idl_json).unwrap();
```

## LICENSE

MIT

