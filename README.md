# Solana IDL

Rust Solana IDL types definitions de/serializable with _serde_ extracted from [shank](https://crates.io/crates/shank).

## Development

Fork the repo makes some changes and make sure that all is dandy. Then provide a pull request.

If you are a contributor with access to publish to crates.io do the below in order to publish a
new version. NOTE that this only works from the _master_ branch and should be performed _after_
merging a PR into master.

```sh
cargo release <major|minor|patch>
```

The above runs all tests and dry-runs the release process. You should verify closely what it is
about to do and then re-run the release command as shown below.

```sh
cargo release <major|minor|patch> --execute
```

## LICENSE

MIT
