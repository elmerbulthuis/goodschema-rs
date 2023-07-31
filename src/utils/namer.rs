use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    rc::{Rc, Weak},
};
use string_morph::to_pascal_case;

type NameNodeRc<K> = Rc<RefCell<NameNode<K>>>;
type NameNodeWeak<K> = Weak<RefCell<NameNode<K>>>;
type NameMap<K> = HashMap<String, Vec<(Option<NameNodeRc<K>>, NameNodeRc<K>)>>;

pub static STARTS_WITH_LETTER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z]").unwrap());

#[derive(Default)]
struct NameNode<K> {
    part: String,
    children: HashMap<String, NameNodeRc<K>>,
    parent: Option<NameNodeWeak<K>>,
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
    root_name_node: NameNodeRc<K>,
    leaf_nodes: HashMap<K, NameNodeRc<K>>,
}

impl<K> Namer<K>
where
    K: Default + Ord + Hash + Clone,
{
    pub fn new(root_name_part: &str) -> Self {
        let root_name_part = to_pascal_case(root_name_part);
        let root_name_node = NameNode::new(root_name_part);
        let root_name_node = RefCell::new(root_name_node);
        let root_name_node = Rc::new(root_name_node);
        Self {
            root_name_node,
            ..Default::default()
        }
    }

    pub fn register_path(&mut self, key: K, path: &str) {
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

            let name_part = name_part.to_string();
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

    pub fn get_names(&self) -> HashMap<String, String> {
        let mut name_map: NameMap<K> = HashMap::new();

        /*
        Should we continue?
        */
        let mut should_continue_counter = 0;

        /*
        Initially fill nameMap
        */
        for (_, leaf_node) in self.leaf_nodes.iter() {
            let nodes = name_map.entry(leaf_node.borrow().part.clone()).or_default();
            if !nodes.is_empty() || STARTS_WITH_LETTER_REGEX.is_match(&leaf_node.borrow().part) {
                should_continue_counter += 1;
            }
            nodes.push((Some(leaf_node.clone()), leaf_node.clone()));
        }

        while should_continue_counter > 0 {
            let mut new_name_map: NameMap<K> = HashMap::new();

            should_continue_counter = 0;

            for (part, mut nodes) in name_map.into_iter() {
                /*
                if nodes.length is one then there are no duplicates. If then name starts
                with a letter, we can move on to the next name.
                */
                if nodes.len() == 1 && STARTS_WITH_LETTER_REGEX.is_match(&part) {
                    let (current_node, target_node) = nodes.pop().unwrap();
                    assert!(new_name_map
                        .insert(part, vec![(current_node, target_node)])
                        .is_none());
                    continue;
                }

                /*
                Collect unique parents nameParts. If there are no unique parents, we want
                to not include the parents namePart in the name.
                */
                let mut unique_parent_name_parts = BTreeSet::new();
                for (current_node, _) in nodes {
                    if let Some(current_node) = current_node {
                        if let Some(parent) = &current_node.borrow().parent {
                            let parent = parent.upgrade().unwrap();
                            unique_parent_name_parts.insert(parent.borrow().part.clone());
                        }
                    }
                }

                todo!();
            }

            name_map = new_name_map;
        }

        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_string_map((k, v): (&str, &str)) -> (String, String) {
        (k.to_string(), v.to_string())
    }

    #[test]
    fn test_1() {
        let mut namer = Namer::new("o");

        namer.register_path("/A", "/A");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/A", "A")].map(to_string_map))
        );

        namer.register_path("/B", "/B");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/A", "A"), ("/B", "B"),].map(to_string_map))
        );

        namer.register_path("/B/C", "/B/C");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/A", "A"), ("/B", "B"), ("/B/C", "C"),].map(to_string_map))
        );

        namer.register_path("/A/C", "/A/C");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [("/A", "A"), ("/B", "B"), ("/B/C", "BC"), ("/A/C", "AC"),].map(to_string_map)
            )
        );

        namer.register_path("/C/A", "/C/A");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [
                    ("/A", "OA"),
                    ("/B", "B"),
                    ("/B/C", "BC"),
                    ("/A/C", "AC"),
                    ("/C/A", "CA"),
                ]
                .map(to_string_map)
            )
        );

        namer.register_path("/A/B/C", "/A/B/C");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [
                    ("/A", "OA"),
                    ("/B", "B"),
                    ("/B/C", "OBC"),
                    ("/A/C", "AC"),
                    ("/C/A", "CA"),
                    ("/A/B/C", "ABC"),
                ]
                .map(to_string_map)
            )
        );

        namer.register_path("/A/B/C/D/E/F", "/A/B/C/D/E/F");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [
                    ("/A", "OA"),
                    ("/B", "B"),
                    ("/B/C", "OBC"),
                    ("/A/C", "AC"),
                    ("/C/A", "CA"),
                    ("/A/B/C", "ABC"),
                    ("/A/B/C/D/E/F", "F"),
                ]
                .map(to_string_map)
            )
        );

        namer.register_path("/X/Y/Z/D/E/F", "/X/Y/Z/D/E/F");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [
                    ("/A", "OA"),
                    ("/B", "B"),
                    ("/B/C", "OBC"),
                    ("/A/C", "AC"),
                    ("/C/A", "CA"),
                    ("/A/B/C", "ABC"),
                    ("/A/B/C/D/E/F", "CF"),
                    ("/X/Y/Z/D/E/F", "ZF"),
                ]
                .map(to_string_map)
            )
        );
        namer.register_path("/X/Y/Z/D/E/1", "/X/Y/Z/D/E/1");
        assert_eq!(
            namer.get_names(),
            HashMap::from(
                [
                    ("/A", "OA"),
                    ("/B", "B"),
                    ("/B/C", "OBC"),
                    ("/A/C", "AC"),
                    ("/C/A", "CA"),
                    ("/A/B/C", "ABC"),
                    ("/A/B/C/D/E/F", "CF"),
                    ("/X/Y/Z/D/E/F", "ZF"),
                    ("/X/Y/Z/D/E/1", "E1"),
                ]
                .map(to_string_map)
            )
        );
    }

    #[test]
    fn test_2() {
        let mut namer = Namer::new("o");

        namer.register_path("/", "/");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/", "O"),].map(to_string_map))
        );

        namer.register_path("/A", "/A");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/", "O"), ("/A", "A"),].map(to_string_map))
        );
    }

    #[test]
    fn test_3() {
        let mut namer = Namer::new("o");

        namer.register_path("/", "/");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/", "O"),].map(to_string_map))
        );

        namer.register_path("/1", "/1");
        assert_eq!(
            namer.get_names(),
            HashMap::from([("/", "O"), ("/1", "O1"),].map(to_string_map))
        );
    }
}
