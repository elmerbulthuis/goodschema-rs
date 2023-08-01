use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
    rc::{Rc, Weak},
};

type NameNodeRc<K> = Rc<RefCell<NameNode<K>>>;
type NameNodeWeak<K> = Weak<RefCell<NameNode<K>>>;
type NameMap<K> = HashMap<String, Vec<(Option<NameNodeRc<K>>, NameNodeRc<K>)>>;

pub static STARTS_WITH_LETTER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z]").unwrap());
pub static NON_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());

#[derive(Default)]
struct NameNode<K> {
    part: String,
    children: HashMap<String, NameNodeRc<K>>,
    parent: Option<NameNodeWeak<K>>,
    /*
    keep sorted set of keys for stable names, the first key always gets a
    lower number appended when resolving collision
    */
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
        let root_name_node = NameNode::new(root_name_part.to_string());
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
            .map(|part| urlencoding::decode(part).unwrap().to_string())
            .map(|part| {
                NON_IDENTIFIER_REGEX
                    .replace_all(part.as_str(), "_")
                    .to_string()
            })
            .filter(|part| !part.is_empty());
        self.register_name_parts(key, name_parts)
    }

    fn register_name_parts(&mut self, key: K, name_parts: impl Iterator<Item = String>) {
        let mut node = self.root_name_node.clone();
        for name_part in name_parts {
            let child_node = node.clone();
            let mut child_node = child_node.borrow_mut();
            let child_node = child_node
                .children
                .entry(name_part.clone())
                .or_insert_with(|| {
                    let mut child_node = NameNode::new(name_part.clone());
                    child_node.parent = Some(Rc::downgrade(&node));
                    let child_node = RefCell::new(child_node);
                    Rc::new(child_node)
                });

            node = child_node.clone();
        }

        assert!(self.leaf_nodes.insert(key.clone(), node.clone()).is_none());
        assert!(node.borrow_mut().keys.insert(key));
    }

    pub fn get_names(&self) -> HashMap<K, String> {
        let mut name_map: NameMap<K> = HashMap::new();

        /*
        Should we continue?
        */
        let mut should_continue_counter = 0;

        /*
        Initially fill nameMap
        */
        for (_, leaf_node) in self.leaf_nodes.iter() {
            let nodes = {
                let leaf_node = leaf_node.borrow();
                let nodes = name_map.entry(leaf_node.part.clone()).or_default();
                if !nodes.is_empty() || !STARTS_WITH_LETTER_REGEX.is_match(&leaf_node.part) {
                    should_continue_counter += 1;
                }
                nodes
            };
            nodes.push((Some(leaf_node.clone()), leaf_node.clone()));
        }

        while should_continue_counter > 0 {
            let mut new_name_map: NameMap<K> = HashMap::new();

            should_continue_counter = 0;

            for (name_part, nodes) in name_map.iter() {
                /*
                if nodes.length is one then there are no duplicates. If then name starts
                with a letter, we can move on to the next name.
                */
                if nodes.len() == 1 && STARTS_WITH_LETTER_REGEX.is_match(name_part) {
                    let (current_node, target_node) = nodes.first().unwrap();
                    assert!(new_name_map
                        .insert(
                            name_part.clone(),
                            vec![(current_node.clone(), target_node.clone())]
                        )
                        .is_none());
                    continue;
                }

                /*
                Collect unique parents nameParts. If there are no unique parents, we want
                to not include the parents namePart in the name.
                */
                let mut parent_name_parts = HashSet::new();
                for (current_node, _) in nodes.iter() {
                    if let Some(current_node) = current_node {
                        let current_node = current_node.borrow();
                        if let Some(parent) = &current_node.parent {
                            let parent = parent.upgrade().unwrap();
                            let parent = parent.borrow();
                            parent_name_parts.insert(parent.part.clone());
                        }
                    }
                }

                for (current_node, target_node) in nodes.iter() {
                    if current_node.is_none() {
                        new_name_map.insert(name_part.clone(), vec![(None, target_node.clone())]);
                        if !STARTS_WITH_LETTER_REGEX.is_match(name_part) {
                            should_continue_counter += 1;
                        }
                        continue;
                    }

                    let current_node = current_node.clone().unwrap();
                    let current_node = current_node.borrow();
                    let new_current_node =
                        current_node.parent.clone().map(|v| v.upgrade().unwrap());
                    let mut new_part = name_part.clone();
                    if let Some(new_current_node) = new_current_node.clone() {
                        if parent_name_parts.len() > 1
                            || !STARTS_WITH_LETTER_REGEX.is_match(&new_part)
                        {
                            new_part = new_current_node.borrow().part.clone() + &new_part;
                        }
                    }

                    let new_nodes = new_name_map.entry(new_part.clone()).or_default();
                    if !new_nodes.is_empty() || !STARTS_WITH_LETTER_REGEX.is_match(&new_part) {
                        should_continue_counter += 1;
                    }
                    new_nodes.push((new_current_node, target_node.clone()));
                }
            }

            name_map = new_name_map;
        }

        let mut result = HashMap::new();
        /*
        Output nameMap
        */
        for (name, nodes) in name_map.iter() {
            let nodes_len = nodes.len();
            assert_eq!(nodes_len, 1);
            let (_, target_node) = nodes.first().unwrap();

            let target_node = target_node.borrow();
            let keys_len = target_node.keys.len();
            match keys_len {
                1 => {
                    let key = target_node.keys.first().unwrap();
                    result.insert(key.clone(), name.clone());
                }
                _ => {
                    for (index, key) in target_node.keys.iter().enumerate() {
                        result.insert(key.clone(), format!("{}${}", name, index));
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_string_map<'l>((k, v): (&'l str, &'l str)) -> (&'l str, String) {
        (k, v.to_string())
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
