use serde::{Deserialize, Serialize};

/// Types that can be included in accounts or user defined structs or instruction args of an IDL.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IdlType {
    Array(Box<IdlType>, usize),
    Bool,
    Bytes,
    Defined(String),
    F32,
    F64,
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<IdlType>),
    #[serde(rename = "coption")]
    COption(Box<IdlType>),
    Tuple(Vec<IdlType>),
    PublicKey,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
    Vec(Box<IdlType>),
    HashMap(Box<IdlType>, Box<IdlType>),
    BTreeMap(Box<IdlType>, Box<IdlType>),
    HashSet(Box<IdlType>),
    BTreeSet(Box<IdlType>),
}
