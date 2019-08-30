extern crate abi_deserializer;

use abi_deserializer::AbiDeserializer;
use canonical_serialization::{
    CanonicalDeserialize, CanonicalDeserializer, CanonicalSerialize, CanonicalSerializer,
    SimpleDeserializer, SimpleSerializer,
};
use failure::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Addr(pub [u8; 32]);

impl Addr {
    fn new(bytes: [u8; 32]) -> Self {
        Addr(bytes)
    }
}

impl CanonicalDeserialize for Addr {
    fn deserialize(deserializer: &mut impl CanonicalDeserializer) -> Result<Self> {
        let mut data_slice: [u8; 32] = [0; 32];
        let data_decoded = deserializer.decode_bytes()?;
        data_slice.copy_from_slice(data_decoded.as_slice());
        Ok(Addr::new(data_slice))
    }
}

impl CanonicalSerialize for Addr {
    fn serialize(&self, serializer: &mut impl CanonicalSerializer) -> Result<()> {
        serializer.encode_bytes(&self.0)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Bar {
    a: u64,
    b: Vec<u8>,
    c: bool,
    d: u32,
    e: Addr,
}

impl CanonicalDeserialize for Bar {
    fn deserialize(deserializer: &mut impl CanonicalDeserializer) -> Result<Self> {
        Ok(Bar {
            a: deserializer.decode_u64()?,
            b: deserializer.decode_bytes()?,
            c: deserializer.decode_bool()?,
            d: deserializer.decode_u32()?,
            e: deserializer.decode_struct()?,
        })
    }
}

impl CanonicalSerialize for Bar {
    fn serialize(&self, serializer: &mut impl CanonicalSerializer) -> Result<()> {
        serializer
            .encode_u64(self.a)?
            .encode_bytes(&self.b)?
            .encode_bool(self.c)?
            .encode_u32(self.d)?
            .encode_struct(&self.e)?;
        Ok(())
    }
}

#[test]
fn deserialize_test_from_map() {
    let bar = Bar {
        a: 15,
        b: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        c: true,
        d: 34,
        e: Addr::new([5u8; 32]),
    };

    let serialized: Vec<u8> = SimpleSerializer::serialize(&bar).expect("Serialization should work");
    let mut deserializer = SimpleDeserializer::new(&serialized);

    let abi = r#"
        {
            "a": "u64",
            "b": "bytes",
            "c": "bool",
            "d": "u32",
            "e": {
                "0": "bytes"
            }
        }"#;

    let expect = json!({
        "a": 15,
        "b": [0,1,2,3,4,5,6,7,8],
        "c": true,
        "d": 34,
        "e": {
            "0": [5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5]
        }
    });

    let mut val: Value = serde_json::from_str(abi).unwrap();
    let abi = val.as_object_mut().unwrap();
    let de = AbiDeserializer::from_map(abi);
    let map = de.deserialize(&mut deserializer).unwrap();
    assert_eq!(Value::Object(map), expect);
}

#[test]
fn deserialize_test_from_str() {
    let bar = Bar {
        a: 33,
        b: vec![1, 1, 2, 3, 4, 5, 6, 7, 8],
        c: false,
        d: 22,
        e: Addr::new([8u8; 32]),
    };

    let serialized: Vec<u8> = SimpleSerializer::serialize(&bar).expect("Serialization should work");
    let mut deserializer = SimpleDeserializer::new(&serialized);

    let abi = r#"
        {
            "a": "u64",
            "b": "bytes",
            "c": "bool",
            "d": "u32",
            "e": {
                "0": "bytes"
            }
        }"#;

    let expect = json!({
        "a": 33,
        "b": [1,1,2,3,4,5,6,7,8],
        "c": false,
        "d": 22,
        "e": {
            "0": [8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8]
        }
    });

    let de = AbiDeserializer::from_str(abi).unwrap();
    let map = de.deserialize(&mut deserializer).unwrap();
    assert_eq!(Value::Object(map), expect);
}
