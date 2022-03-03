use crate::data;

pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub attributes: Vec<String>
}

impl Node {
    pub fn new(name: String,attributes: Vec<String>,children: Vec<Node>) -> Self {
        Self {
            attributes: attributes,
            children:children,
            name: name
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    pub fn add_children(&mut self,children: &mut  Vec<Node>){
        self.children.append(children);
    }
    pub fn set_attribute(&mut self,key: String, value: String){
        if value.is_empty() {
            self.attributes.push(key);
        }
        else {
            let mut attribute: String = key;
            let equal: &str = "=";
            attribute.push_str(equal);
            attribute.push_str(&value);
            self.attributes.push(attribute);
        }
    }
    pub fn set_attributes(&mut self, attributes: &mut Vec<Vec<String>>){
        for a in attributes {
            self.set_attribute(a[0].to_string(), a[1].to_string());
        }
    }
    pub fn get_attributes(&self) -> String{
        return self.attributes.join(" ");
    }
    pub fn get_children(&self) -> String{
        let children: &mut String = &mut "".to_string();
        for c in &self.children {
            children.push_str(&c.to_html());
        }
        return children.to_string();
    }
    pub fn get_open_tag(&self) -> String {
        if self.attributes.len() == 0 {
            return format!("<{}>",self.name);
        }
        else{
            return format!("<{} {}>",self.name,self.get_attributes());
        }
    }
    pub fn get_close_tag(&self) -> String {
        return format!("</{}>",self.name);
    }
    pub fn to_html(&self) -> String {
        if self.is_self_closing_tag() {
            return self.get_open_tag();
        }
        return format!("{}{}{}",self.get_open_tag(),self.get_children(),self.get_close_tag());
    }
    pub fn is_self_closing_tag(&self) -> bool{
        let result = data::read_json_file("./src/self_closing_tags.json".to_string());
        return result[&self.name] == true;
    }
}
