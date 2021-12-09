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
        let s = self
            .attributes
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tag() {
        let tag = Tag::new("div".to_string());
        assert_eq!("<div>", tag.start_tag());
        assert_eq!("</div>", tag.end_tag());
    }

    #[test]
    fn tag_with_ids() {
        let mut tag = Tag::new("div".to_string());
        tag.add_id("red".to_string());
        assert_eq!("<div id=\"red\">", tag.start_tag());

        tag.add_id("blue".to_string());
        assert_eq!("<div id=\"red blue\">", tag.start_tag());
    }

    #[test]
    fn tag_with_classes() {
        let mut tag = Tag::new("div".to_string());
        tag.add_class("red".to_string());
        assert_eq!("<div class=\"red\">", tag.start_tag());

        tag.add_class("blue".to_string());
        assert_eq!("<div class=\"red blue\">", tag.start_tag());
    }

    #[test]
    fn tag_with_styles() {
        let mut tag = Tag::new("div".to_string());
        tag.add_style("color".to_string(), "red".to_string());
        assert_eq!("<div style=\"color:red\">", tag.start_tag());

        tag.add_style("display".to_string(), "none".to_string());
        assert_eq!("<div style=\"color:red; display:none\">", tag.start_tag());
    }

    #[test]
    fn tag_with_attributes() {
        let mut tag = Tag::new("div".to_string());
        tag.add_attribute("focus".to_string(), None);
        assert_eq!("<div focus>", tag.start_tag());

        tag.add_attribute("name".to_string(), Some("red".to_string()));
        assert_eq!("<div focus name=\"red\">", tag.start_tag());
    }
}
