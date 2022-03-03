use serde_json;
mod node;
mod data;
mod document;
mod attribute_factory;
mod node_factory;

fn main() {
    let doc = document::Document::new();
    doc.to_html(serde_json::json!({
        "html": {
            "head": {},
            "body": {
                "img": {
                    "src": "",
                    "alt": "alternative text"
                }
            }
        }
    }));
    let head = node::Node::new("head".to_string(),vec![],vec![]);
    let img = node::Node::new("img".to_string(),vec!["src=\"\"".to_string()],vec![]);
    let body = node::Node::new("body".to_string(),vec![],vec![img]);
    let mut html = node::Node::new("html".to_string(), vec![], vec![]);

    html.set_attribute("hidden".to_string(), "".to_string());
    html.set_attributes(&mut vec![vec!["disabled".to_string(), "".to_string()],vec!["name".to_string(), "any".to_string()]]);
    html.add_child(head);
    html.add_children(&mut vec![body]);
    
    println!("expected: {}",html.to_html());
}