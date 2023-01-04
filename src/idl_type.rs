use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IdlType {
    Array(Box<IdlType>, usize),
    Bool,
    Bytes,
    Defined(String),
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<IdlType>),
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
