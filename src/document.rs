use crate::serde_json;
use crate::node_factory;
use crate::node;

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
        let nodes: Vec<node::Node>;
        
        if jsd.is_string() {
            nodes = self.node_factory.create_nodes(&self.try_parse_object(&jsd));
        }
        else{
            nodes = self.node_factory.create_nodes(&jsd);
        }

        if nodes.len() == 0 {
            return jsd.to_string();
        }
        let mut htmls: Vec<String> = vec![];
        for node in nodes {
            htmls.push(node.to_html());
        }
        
        return htmls.join("");
    }
    fn try_parse_object(&self,jsd: &serde_json::Value) -> serde_json::Value{
        return serde_json::json!(&jsd.to_string());
    }
}