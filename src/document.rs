use crate::serde_json;
use crate::node_factory;

pub struct Document {
    node_factory: node_factory::NodeFactory
}

impl Document {
    pub fn new() -> Self {
        Self {
            node_factory: node_factory::NodeFactory::new()
        }
    }
    pub fn to_html(&self,jsd: serde_json::Value) -> String {
        if jsd.is_array() {
            return jsd.to_string();
        }
        else if jsd.is_object() {
            self.node_factory.create_nodes_from_object(jsd);
            return "".to_string();
        }
        else if jsd.is_string() {
            return self.try_parse_object(jsd).to_string();
        }
        else{
            return "".to_string();
        }
    }
    fn try_parse_object(&self,jsd: serde_json::Value) -> serde_json::Value{
        return serde_json::json!(&jsd.to_string());
    }
}