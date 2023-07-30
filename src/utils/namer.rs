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
                node = child_node.clone()
            } else {
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
            };
        }

        assert!(self.leaf_nodes.insert(key.clone(), node.clone()).is_none());
        assert!(node.borrow_mut().keys.insert(key));
    }
}
