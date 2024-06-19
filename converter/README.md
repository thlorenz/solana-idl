# Solana IDL Converter

Converts different IDL formats to a common _classic_ IDL format.

## Warning

Not all parts of the new formats can be properly converted to the _classic_ one, thus **use
this with care**.

Specifically IDL instruction discriminants had a single `u8` (who has more than 256
instructions?) but the new anchor format uses a `Vec<u8>` instead.

## Usage

Please refer to the [tests](./tests/) for examples on how to use this.

## LICENSE

MIT
