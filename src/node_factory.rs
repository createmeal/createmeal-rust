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
    /// check if is an object or array to call right method to create nodes.
    pub fn create_nodes(&self,jsd: &serde_json::Value) -> Vec<node::Node> {
        if jsd.is_object() {
            return self.get_nodes(jsd);
        }
        if jsd.is_array() {
            return self.create_nodes_from_array(jsd);
        }
        return vec![];
    }
    pub fn create_nodes_from_array(&self,jsd: &serde_json::Value)-> Vec<node::Node>{
        let mut nodes: Vec<node::Node>= vec![];
        
        for item in jsd.as_array().unwrap().iter() {
            nodes.append(&mut self.get_nodes(item));
        }

        return nodes;
    }
    pub fn get_nodes(&self, value: &serde_json::Value) -> Vec<node::Node>{
        if value.is_string() {
            return vec![self.get_node_from_string(value.to_string())];
        }
        if value.is_array() {
            return self.get_nodes_from_array(value);
        }
        if value.is_object() {
            return self.get_nodes_from_object(value);
        }
        return vec![node::Node::new("".to_string(), vec![], vec![])];
    }
    pub fn get_nodes_from_object(&self,jsd: &serde_json::Value) -> Vec<node::Node>{
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
                let attributes = self.attribute_factory.get_attributes(&data::read_json(value.to_string()));
                let children = self.create_nodes(&data::read_json(value));
                
                let node = node::Node::new(key.to_string(), attributes, children);
                nodes.push(node);
            }
            return nodes;
        }
    }
    pub fn get_nodes_from_array(&self,_value: &serde_json::Value) -> Vec<node::Node>{
        return vec![node::Node::new("".to_string(), vec![], vec![])];
    }
    pub fn get_node_from_string(&self,value: String) -> node::Node{
        return node::Node::new(value.to_string(), vec![], vec![]);
    }
}