pub mod tag;

pub use tag::Tag;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lonely_literal() {
        let root = Tree::Literal("Hello!".to_string());
        assert_eq!("Hello!".to_string(), root.to_html_min());
    }

    #[test]
    fn lonely_node() {
        let tag = Tag::new("div".to_string());
        let root = Tree::Node(Node::new(tag));
        assert_eq!("<div></div>".to_string(), root.to_html_min());
    }

    #[test]
    fn literal_children() {
        let mut node = Node::new(Tag::new("div".to_string()));
        node.add_child(Tree::Literal("Hello".to_string()));
        node.add_child(Tree::Literal(" World!".to_string()));
        let t = Tree::Node(node);
        assert_eq!("<div>Hello World!</div>".to_string(), t.to_html_min());
    }

    #[test]
    fn diverse_children() {
        let mut node = Node::new(Tag::new("div".to_string()));
        let span = Node::new(Tag::new("span".to_string()));

        node.add_child(Tree::Literal("Hello".to_string()));
        node.add_child(Tree::Node(span));
        node.add_child(Tree::Literal("World!".to_string()));
        let t = Tree::Node(node);
        assert_eq!(
            "<div>Hello<span></span>World!</div>".to_string(), 
            t.to_html_min()
        );
    }

}
