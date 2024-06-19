use solana_idl::try_extract_classic_idl;

#[test]
fn test_extract_classic_idl() {
    let classic_counter_json = include_str!(
        "../../converter/tests/fixtures/anchor_counter.converted.json"
    );
    let new_counter_json =
        include_str!("../../converter/tests/fixtures/anchor_counter.json");

    let from_classic_idl =
        try_extract_classic_idl(classic_counter_json).unwrap();
    let from_new_idl = try_extract_classic_idl(new_counter_json).unwrap();
    assert_eq!(from_classic_idl, from_new_idl);
}
