use anchor_lang_idl::types::Idl as NewIdl;
use solana_idl_converter::anchor_to_classic::try_convert;

#[test]
fn test_convert_counter_example() {
    let new_json = include_str!("./fixtures/anchor_counter.json");
    let new_idl = serde_json::from_str::<NewIdl>(new_json).unwrap();
    let idl = try_convert(new_idl).unwrap();
    let json = serde_json::to_string_pretty(&idl).unwrap();
    eprintln!("{}", json);
}
