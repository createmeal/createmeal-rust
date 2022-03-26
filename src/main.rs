use serde_json;
mod node;
mod data;
mod document;
mod attribute_factory;
mod node_factory;

fn main() {
    let doc = document::Document::new();
    let result = doc.to_html(serde_json::json!({
        "html": [{
            "head": {}},
            {"body": [{
                "img": {
                    "src": "",
                    "alt": "alternative text"
                }
                },
                {"button": {

                }
            }]
        }]
    }));
    /*
    let head = node::Node::new("head".to_string(),vec![],vec![]);
    let img = node::Node::new("img".to_string(),vec!["src=\"\"".to_string()],vec![]);
    let button = node::Node::new("button".to_string(), vec!["disabled".to_string(), "name=any".to_string()], vec![]);
    let body = node::Node::new("body".to_string(),vec![],vec![img,button]);
    let mut html = node::Node::new("html".to_string(), vec![], vec![]);

    html.add_child(head);
    html.add_children(&mut vec![body]);
    
    println!("result: {}",html.to_html());*/
    println!("result2: {}",result);
}