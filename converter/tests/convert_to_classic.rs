use anchor_lang_idl::types::Idl as NewIdl;
use solana_idl_classic::Idl;
use solana_idl_converter::anchor_to_classic::try_convert;

#[test]
fn test_convert_counter() {
    let new_json = include_str!("./fixtures/anchor_counter.json");
    let new_idl = serde_json::from_str::<NewIdl>(new_json).unwrap();
    let idl = try_convert(new_idl).unwrap();
    let expected_json =
        include_str!("./fixtures/anchor_counter.converted.json");
    let expected_idl = serde_json::from_str::<Idl>(expected_json).unwrap();
    assert_eq!(idl, expected_idl);
}

#[test]
fn test_convert_world() {
    let new_json = include_str!("./fixtures/world.json");
    let new_idl = serde_json::from_str::<NewIdl>(new_json).unwrap();
    let idl = try_convert(new_idl).unwrap();
    let expected_json = include_str!("./fixtures/world.converted.json");
    let expected_idl = serde_json::from_str::<Idl>(expected_json).unwrap();
    assert_eq!(idl, expected_idl);
}
