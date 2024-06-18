use anchor_lang_idl::types::IdlMetadata as NewIdlMetadata;
use solana_idl_classic::IdlMetadata;

pub fn convert(metadata: NewIdlMetadata, address: Option<String>) -> IdlMetadata {
    let address = address.or(match metadata.deployments {
        Some(deployments) => {
            // We ignore the fact that we could have conflicting addresses, i.e.
            // the program could be deployed under a different address on mainnet
            // than on devnet
            // It's very unlikely + we can only include one address in the classic metadata.
            deployments
                .mainnet
                .or(deployments.devnet)
                .or(deployments.testnet)
                .or(deployments.localnet)
        }
        None => None,
    });

    IdlMetadata {
        origin: Some("anchor".to_string()),
        address,
        // The new IDL version allows specifying a serializer per account and we have no
        // way to express that here.
        // As an improvement we could try to find the common one used and pass that here,
        // but for now we just admit that we don't know it.
        serializer: None,
    }
}
