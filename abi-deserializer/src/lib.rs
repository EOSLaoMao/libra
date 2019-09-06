use canonical_serialization::CanonicalDeserializer;
use failure::{prelude::*, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, map::Map, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbiDeserializer(Map<String, Value>);

impl AbiDeserializer {
    pub fn from_map(m: &Map<String, Value>) -> Self {
        AbiDeserializer(m.clone())
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let mut val: Value = serde_json::from_str(s)?;
        ensure!(val.is_object(), "invalid abi object");
        let m = val.as_object_mut().unwrap();
        Ok(AbiDeserializer(m.clone()))
    }

    pub fn get_map(&self) -> &Map<String, Value> {
        &self.0
    }

    pub fn get_map_mut(&mut self) -> &mut Map<String, Value> {
        &mut self.0
    }

    pub fn deserialize(
        &self,
        deserializer: &mut impl CanonicalDeserializer,
    ) -> Result<Map<String, Value>> {
        let mut result = Map::new();
        self.deserialize_impl(deserializer, &self.0, &mut result)?;
        Ok(result)
    }

    pub fn deserialize_to_string(
        &self,
        deserializer: &mut impl CanonicalDeserializer,
    ) -> Result<String> {
        let mut result = Map::new();
        self.deserialize_impl(deserializer, &self.0, &mut result)?;
        serde_json::to_string(&Value::Object(result)).map_err(|e| From::from(e))
    }

    pub fn deserialize_to_string_pretty(
        &self,
        deserializer: &mut impl CanonicalDeserializer,
    ) -> Result<String> {
        let mut result = Map::new();
        self.deserialize_impl(deserializer, &self.0, &mut result)?;
        serde_json::to_string_pretty(&Value::Object(result)).map_err(|e| From::from(e))
    }

    fn deserialize_impl(
        &self,
        deserializer: &mut impl CanonicalDeserializer,
        abi: &Map<String, Value>,
        result: &mut Map<String, Value>,
    ) -> Result<()> {
        for (key, val) in abi.iter() {
            let result = match val {
                Value::String(s) => match s.as_str() {
                    "u64" => {
                        result
                            .entry(key)
                            .or_insert(json!(deserializer.decode_u64()?));
                        Ok(())
                    }
                    "bytes" => {
                        result
                            .entry(key)
                            .or_insert(json!(deserializer.decode_bytes()?));
                        Ok(())
                    }
                    "bool" => {
                        result
                            .entry(key)
                            .or_insert(json!(deserializer.decode_bool()?));
                        Ok(())
                    }
                    "u32" => {
                        result
                            .entry(key)
                            .or_insert(json!(deserializer.decode_u32()?));
                        Ok(())
                    }
                    _ => bail!("unsupported data type"),
                },
                Value::Object(abi) => {
                    let child = result.entry(key).or_insert(Value::Object(Map::new()));
                    self.deserialize_impl(deserializer, &abi, child.as_object_mut().unwrap()) // safe to unwrap
                }
                _ => bail!("unsupported data type"),
            };

            if let Err(e) = result {
                return Err(format_err!("unable to decode field: {} - {}", key, e));
            };
        }

        Ok(())
    }
}
