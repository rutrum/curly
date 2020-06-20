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
pub struct TagTree {
    this: Tag,
    children: Vec<Box<TagTree>>,
}

impl TagTree {
    pub fn new(this: Tag) -> Self {
        Self {
            this,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TagTree) {
        self.children.push(Box::new(child));
    }

    pub fn to_html(&self) {
        self.to_html_tabbed(0);
    }

    fn to_html_tabbed(&self, tabs: usize) {
        let spacing = (0..tabs * 4).map(|_| " ").collect::<String>();
        println!("{}{}", spacing, self.this.start_tag());
        for child in &self.children {
            child.to_html_tabbed(tabs + 1);
        }
        println!("{}{}", spacing, self.this.end_tag());
    }
}
