use crate::serde_json;
use crate::data;

pub struct AttributeFactory {
    pub attributes: serde_json::Value,
    pub tags: serde_json::Value
}

impl AttributeFactory {
    pub fn new() -> Self {
        Self {
            attributes: data::read_json("./src/attributes.json".to_string()),
            tags: data::read_json("./src/tags.json".to_string())
        }
    }
    pub fn is_field_representing_attributes(&self,key: &str) -> bool{
        return key.starts_with("_") || self.is_attribute(key) || key == "attributes";
    }
    pub fn _set_node_attributes(){

    }
    pub fn get_attributes(&self,_value: serde_json::Value) -> Vec<String>{
        return vec![];
    }
    pub fn _get_attribute(&self,key: String, value: String) -> serde_json::Value{
        if self._is_boolean_attribute() {
            return serde_json::json!({"key":key});
        }
        return serde_json::json!({"key":key,"value":value});
    }
    pub fn is_attribute(&self,name: &str) -> bool{
        return (self.tags[name.to_string()] == false && self.attributes[name.to_string()] == true) || 
            self.is_attribute_prefix(&name.to_string());
    }
    pub fn is_attribute_prefix(&self,_name: &str) -> bool{
        return false;
    }

    pub fn _is_boolean_attribute(&self) -> bool{
        return false;
    }
    pub fn _process_attribute_array(){

    }
}