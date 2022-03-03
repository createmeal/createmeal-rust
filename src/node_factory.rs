use crate::data;
use crate::serde_json;
use crate::node;
use crate::attribute_factory;

pub struct NodeFactory {
    attribute_factory: attribute_factory::AttributeFactory,
}

impl NodeFactory {
    pub fn new() -> Self {
        Self {
            attribute_factory: attribute_factory::AttributeFactory::new()
        }
    }
    pub fn create_nodes_from_object(&self,jsd: serde_json::Value) -> Vec<node::Node>{
        let mut nodes = vec![];
        if jsd.is_null() {
            return nodes;
        }
        else{        
            let value = jsd.to_string();
            let mut deser = serde_json::Deserializer::from_str(&value);
            let m = data::deser_hashmap(&mut deser).unwrap();
            for (key,value) in m {
                if self.attribute_factory.is_field_representing_attributes(&key) {
                    continue;
                }
                let attributes = self.attribute_factory.get_attributes(serde_json::json!(value));
                let children = self.create_nodes(data::read_json(value));
                let node = node::Node::new(key.to_string(), attributes, children);
                nodes.push(node);
            }
            for node in nodes {
                println!("{}",&node.to_html());
            }
            return vec![];
        }
    }
    pub fn create_nodes(&self,jsd: serde_json::Value) -> Vec<node::Node>{
        if jsd.is_object() {
            return self.create_nodes_from_object(jsd);
        }
        if jsd.is_array() {
            return self.create_nodes_from_array(jsd);
        }
        return vec![];
    }
    pub fn create_nodes_from_array(&self,_jsd: serde_json::Value)-> Vec<node::Node>{
        return vec![];
    }
    /*pub fn get_object_node(&self,_value: serde_json::Value){

    }
    pub fn get_array_node(&self,_value: Vec<serde_json::Value>) -> node::Node{
        return node::Node::new("".to_string(), vec![], vec![]);
    }
    pub fn get_string_node(&self,_value: String) -> node::Node{
        return node::Node::new("".to_string(), vec![], vec![]);
    }*/
}