use std::fmt;

#[derive(Debug, Default)]
pub struct Tag {
    name: String,
    ids: Vec<String>,
    //classes: Vec<String>,
    //styles: Vec<(String, String)>
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Tag::default()
        }
    }

    pub fn add_id(&mut self, id: String) {
        self.ids.push(id);
    }

    pub fn start_tag(&self) -> String {
        format!("<{}{}>", self.name, self.id_string())
    }

    pub fn id_string(&self) -> String {
        if self.ids.len() == 0 {
            String::new()
        } else {
            format!(" id=\"{}\"", self.ids.join(" "))
        }
    }

    pub fn end_tag(&self) -> String {
        format!("</{}>", self.name)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.name)
    }
}

#[derive(Debug)]
pub enum Tree {
    Node(Node),
    Literal(String),
}

impl Tree {
    pub fn to_html(&self) {
        self.to_html_tabbed(0);
    }

    fn to_html_tabbed(&self, tabs: usize) {
        let spacing = (0..tabs * 4).map(|_| " ").collect::<String>();
        use Tree::*;
        match self {
            Node(node) => {
                println!("{}{}", spacing, node.tag.start_tag());
                for child in &node.children {
                    child.to_html_tabbed(tabs + 1);
                }
                println!("{}{}", spacing, node.tag.end_tag());
            }
            Literal(s) => {
                println!("{}{}", spacing, s);
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    tag: Tag,
    children: Vec<Box<Tree>>,
}

impl Node {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Tree) {
        self.children.push(Box::new(child));
    }
}
