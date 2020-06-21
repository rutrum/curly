use std::fmt;

#[derive(Debug, Default)]
pub struct Tag {
    name: String,
    ids: Vec<String>,
    classes: Vec<String>,
    styles: Vec<(String, String)>,
    attributes: Vec<(String, Option<String>)>,
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

    pub fn add_class(&mut self, class: String) {
        self.classes.push(class);
    }

    pub fn add_style(&mut self, prop: String, value: String) {
        self.styles.push((prop, value));
    }

    pub fn add_attribute(&mut self, prop: String, value: Option<String>) {
        self.attributes.push((prop, value));
    }

    pub fn start_tag(&self) -> String {
        format!(
            "<{}{}{}{}{}>",
            self.name,
            self.id_string(),
            self.class_string(),
            self.style_string(),
            self.attribute_string(),
        )
    }

    pub fn attribute_string(&self) -> String {
        if self.attributes.is_empty() {
            return String::new();
        }
        let s = self.attributes
            .iter()
            .map(|(p, opt)| match opt {
                Some(v) => format!("{}=\"{}\"", p, v),
                None => p.to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        format!(" {}", s)
    }

    pub fn style_string(&self) -> String {
        if self.styles.is_empty() {
            String::new()
        } else {
            format!(
                " style=\"{}\"",
                self.styles
                    .iter()
                    .map(|(p, v)| format!("{}:{}", p, v))
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
    }

    pub fn class_string(&self) -> String {
        if self.classes.is_empty() {
            String::new()
        } else {
            format!(" class=\"{}\"", self.classes.join(" "))
        }
    }

    pub fn id_string(&self) -> String {
        if self.ids.is_empty() {
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
    pub fn to_html_min(&self) -> String {
        use Tree::*;
        match self {
            Node(node) => {
                let middle = node
                    .children
                    .iter()
                    .map(Tree::to_html_min)
                    .collect::<String>();
                format!("{}{}{}", node.tag.start_tag(), middle, node.tag.end_tag())
            }
            Literal(s) => s.to_string(),
        }
    }

    pub fn to_html(&self) -> String {
        self.to_html_tabbed(0)
    }

    fn to_html_tabbed(&self, tabs: usize) -> String {
        let spacing = (0..tabs * 4).map(|_| " ").collect::<String>();
        use Tree::*;
        match self {
            Node(node) => match node.children.len() {
                0 => format!(
                    "{}{}{}\n",
                    spacing,
                    node.tag.start_tag(),
                    node.tag.end_tag()
                ),
                _ => {
                    let middle = node
                        .children
                        .iter()
                        .map(|x| x.to_html_tabbed(tabs + 1))
                        .collect::<String>();
                    format!(
                        "{}{}\n{}{}{}\n",
                        spacing,
                        node.tag.start_tag(),
                        middle,
                        spacing,
                        node.tag.end_tag(),
                    )
                }
            },
            Literal(s) => format!("{}{}\n", spacing, s),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    tag: Tag,
    children: Vec<Tree>,
}

impl Node {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Tree) {
        self.children.push(child);
    }
}
