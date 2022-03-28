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
                    "_src": "",
                    "_alt": "alternative text"
                }
                },
                {"button": {

                }
            }]
        }]
    }));
    println!("result2: {}",result);
}