use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    rc::{Rc, Weak},
};

use string_morph::to_pascal_case;

#[derive(Default)]
struct NameNode<K> {
    part: String,
    children: HashMap<String, Rc<RefCell<NameNode<K>>>>,
    parent: Option<Weak<RefCell<NameNode<K>>>>,
    keys: BTreeSet<K>,
}

impl<K> NameNode<K>
where
    K: Default,
{
    fn new(part: String) -> Self {
        Self {
            part,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Namer<K> {
    root_name_node: Rc<RefCell<NameNode<K>>>,
    leaf_nodes: HashMap<K, Rc<RefCell<NameNode<K>>>>,
}

impl<K> Namer<K>
where
    K: Default + Ord + Hash + Clone,
{
    pub fn new(root_name_part: Cow<str>) -> Self {
        let root_name_part = to_pascal_case(root_name_part.as_ref());
        let root_name_node = NameNode::new(root_name_part);
        let root_name_node = RefCell::new(root_name_node);
        let root_name_node = Rc::new(root_name_node);
        Self {
            root_name_node,
            ..Default::default()
        }
    }

    pub fn register_path(&mut self, key: K, path: Cow<str>) {
        let name_parts = path
            .split('/')
            .map(|part| urlencoding::decode(part).unwrap())
            .collect();
        self.register_name_parts(key, name_parts)
    }

    pub fn register_name_parts(&mut self, key: K, name_parts: Vec<Cow<str>>) {
        let mut node = self.root_name_node.clone();
        for name_part in name_parts {
            if let Some(child_node) = node.clone().borrow().children.get(name_part.as_ref()) {
                node = child_node.clone();
                continue;
            }

            let name_part = name_part.into_owned();
            let mut child_node = NameNode::new(name_part.clone());
            child_node.parent = Some(Rc::downgrade(&node));
            let child_node = RefCell::new(child_node);
            let child_node = Rc::new(child_node);
            assert!(node
                .borrow_mut()
                .children
                .insert(name_part, child_node.clone())
                .is_none());
        }

        assert!(self.leaf_nodes.insert(key.clone(), node.clone()).is_none());
        assert!(node.borrow_mut().keys.insert(key));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut namer = Namer::new(Cow::Borrowed("o"));

        namer.register_path("/A", Cow::Borrowed("/A"));
        namer.register_path("/B", Cow::Borrowed("/B"));
        namer.register_path("/B/C", Cow::Borrowed("/B/C"));
        namer.register_path("/A/C", Cow::Borrowed("/A/C"));
        namer.register_path("/C/A", Cow::Borrowed("/C/A"));
        namer.register_path("/A/B/C", Cow::Borrowed("/A/B/C"));
        namer.register_path("/A/B/C/D/E/F", Cow::Borrowed("/A/B/C/D/E/F"));
        namer.register_path("/X/Y/Z/D/E/F", Cow::Borrowed("/X/Y/Z/D/E/F"));
        namer.register_path("/X/Y/Z/D/E/1", Cow::Borrowed("/X/Y/Z/D/E/1"));
    }

    #[test]
    fn test_2() {
        let mut namer = Namer::new(Cow::Borrowed("o"));

        namer.register_path("/", Cow::Borrowed("/"));
        namer.register_path("/A", Cow::Borrowed("/A"));
    }

    #[test]
    fn test_3() {
        let mut namer = Namer::new(Cow::Borrowed("o"));

        namer.register_path("/", Cow::Borrowed("/"));
        namer.register_path("/1", Cow::Borrowed("/1"));
    }
}
