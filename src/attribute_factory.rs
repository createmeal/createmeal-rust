use crate::serde_json;
use crate::data;

pub struct AttributeFactory {
    pub attributes: serde_json::Value,
    pub tags: serde_json::Value
}

impl AttributeFactory {
    pub fn new() -> Self {
        Self {
            attributes: data::read_json_file("./attributes.json".to_string()),
            tags: data::read_json_file("./tags.json".to_string())
        }
    }
    pub fn is_field_representing_attributes(&self,key: &str) -> bool{
        return key.starts_with("_") || self.is_attribute(key) || key == "attributes";
    }
    pub fn _set_node_attributes(){

    }
    pub fn get_attributes(&self,jsd: &serde_json::Value) -> Vec<String>{
        
        
        let mut attributes: Vec<String> = vec![];
        if jsd.is_string() {
            println!("key: {}",jsd);
        }
        if jsd.is_array() {
            for item in jsd.as_array().unwrap().iter() {
                attributes.append(&mut self.get_attributes(item));
            }
        }
        else if jsd.is_object() {
            let value = jsd.to_string();
            let mut deser = serde_json::Deserializer::from_str(&value);
            let m = data::deser_hashmap(&mut deser).unwrap();
            
            for (key,value) in m {
                if key.starts_with("_") {
                    let key_name = data::remove_first_charactere(&key);
                    attributes.push(self.get_attribute(&key_name.to_string(),&value));
                    continue;
                }
                if self.is_attribute_prefix(&key) {
                    attributes.push(self.get_attribute(&key,&value));
                    continue;
                }
                if key == "attributes" {
                    for item in jsd.as_array().unwrap().iter() {
                        attributes.append(&mut self.get_attributes(item));
                    }
                    break;
                }
                if self.is_attribute(&key) {
                    attributes.push(self.get_attribute(&key,&value));
                }
            }
        }
        return attributes;
    }
    pub fn get_attribute(&self,key: &String, value: &String) -> String{
        

        if self.is_boolean_attribute() {
            return key.to_string();
        }
        let attribute = format!("{}={}",key,value);
        return attribute;
    }
    pub fn is_attribute(&self,name: &str) -> bool{
        return (self.tags[name.to_string()] == false && self.attributes[name.to_string()] == true) || 
            self.is_attribute_prefix(&name.to_string());
    }
    pub fn is_attribute_prefix(&self,_name: &str) -> bool{
        return false;
    }

    pub fn is_boolean_attribute(&self) -> bool{
        return false;
    }
    pub fn _process_attribute_array(){

    }
}