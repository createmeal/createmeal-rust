use std::collections::HashMap;
use serde::de::*;
use crate::serde_json;
use std::fs;

pub fn read_json(path: String) -> serde_json::Value {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    return res;
}

pub fn deser_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MapVisitor;

    impl<'de> Visitor<'de> for MapVisitor {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut values = HashMap::new();

            // We use RawValue here to access the JSON value exactly as it is occuring in the input.
            while let Some((key, value)) = (access.next_entry())?
                .map(|(k, v): (String, &'de serde_json::value::RawValue)| (k, v.get().to_owned()))
            {
                values.insert(key, value);
            }

            Ok(values)
        }
    }

    let visitor = MapVisitor;
    deserializer.deserialize_map(visitor)
}
