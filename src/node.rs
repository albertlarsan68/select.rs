use std::collections::HashMap;

use document::Document;
use predicate::Predicate;
use selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(String),
    Element(String, HashMap<String, String>, Vec<usize>),
    Comment(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub data: Data
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<'a> {
    document: &'a Document,
    index: usize
}

impl<'a> Node<'a> {
    pub fn new(document: &'a Document, index: usize) -> Node<'a> {
        Node {
            document: document,
            index: index
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn data(&self) -> &Data {
        &self.document.nodes[self.index].data
    }

    pub fn name(&self) -> Option<&str> {
        match self.document.nodes[self.index].data {
            Data::Element(ref name, _, _) => Some(name),
            _ => None
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match self.document.nodes[self.index].data {
            Data::Element(_, ref attrs, _) => attrs.get(name).map(|s| &s[..]),
            _ => None
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.document.nodes[self.index].parent.map(|index| self.document.nth(index))
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.document.nodes[self.index].prev.map(|index| self.document.nth(index))
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.document.nodes[self.index].next.map(|index| self.document.nth(index))
    }

    pub fn text(&self) -> String {
        let mut string = String::new();
        recur(&self.document, self.index, &mut string);
        return string;

        fn recur(document: &Document, index: usize, string: &mut String) {
            match document.nodes[index].data {
                Data::Text(ref text) => string.push_str(text),
                Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(document, child, string)
                    }
                },
                Data::Comment(_) => {}
            }
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection::new(self.document, [self.index].iter().cloned().collect()).find(p)
    }

    pub fn is<P: Predicate>(&self, p: P) -> bool {
        p.matches(self)
    }

    pub fn as_text(&self) -> Option<&str> {
        match self.data() {
            &Data::Text(ref text) => Some(&text),
            _ => None
        }
    }

    pub fn as_comment(&self) -> Option<&str> {
        match self.data() {
            &Data::Comment(ref comment) => Some(&comment),
            _ => None
        }
    }
}
