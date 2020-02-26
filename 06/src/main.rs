use std::collections::HashMap;
use std::io::Read;

struct OrbitNode {
    name: String,
    orbiters: Vec<Box<OrbitNode>>,
}

fn make_node_map(input_string: &String) -> HashMap<String, Vec<String>> {
    let pairs: Vec<(String, String)> = input_string
        .lines()
        .map(|line_str| {
            let mut iter = line_str.split(")");
            let label = iter.next().unwrap().to_string();
            let child = iter.next().unwrap().to_string();
            (label, child)
        })
        .collect();

    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in pairs {
        if !res.contains_key(&v) {
            res.insert(v.to_string(), vec![]);
        }
        if !res.contains_key(&k) {
            res.insert(k, vec![v]);
        } else {
            res.get_mut(&k).unwrap().push(v);
        }
    }
    res
}

fn build_tree(node_map: &HashMap<String, Vec<String>>) -> Box<OrbitNode> {
    let root_name = "COM".to_string();
    let mut root = Box::new(OrbitNode {
        name: root_name.to_string(),
        orbiters: vec![],
    });

    {
        let mut iter_stack: Vec<&mut OrbitNode> = vec![&mut (*root)];
        while !iter_stack.is_empty() {
            let cur = iter_stack.pop().unwrap();
            for child in node_map.get(&cur.name).unwrap() {
                cur.orbiters.push(Box::new(OrbitNode {
                    name: child.to_string(),
                    orbiters: vec![],
                }));
            }
            for o in &mut cur.orbiters {
                iter_stack.push(&mut (*o));
            }
        }
        iter_stack.clear();
    }

    root
}

fn has_both(name1: &String, name2: &String, root_node: &Box<OrbitNode>) -> bool {
    let mut iter_stack = vec![(root_node, 0)];
    let mut has_1: bool = false;
    let mut has_2: bool = false;
    while !iter_stack.is_empty() && !(has_1 && has_2) {
        let (cur, depth) = iter_stack.pop().unwrap();

        if cur.name == *name1 {
            has_1 = true;
        }
        if cur.name == *name2 {
            has_2 = true;
        }

        for child in cur.orbiters.iter() {
            iter_stack.push((child, depth + 1));
        }
    }

    has_1 && has_2
}

fn naive_common_parent<'a>(
    name1: String,
    name2: String,
    root_node: &Box<OrbitNode>,
) -> &Box<OrbitNode> {
    let mut iter_stack = vec![(root_node, 0)];
    let mut min_depth: i32 = std::i32::MAX;
    let mut min_common_parent: &Box<OrbitNode> = root_node;
    while !iter_stack.is_empty() {
        let (cur, depth) = iter_stack.pop().unwrap();

        if has_both(&name1, &name2, cur) {
            min_depth = std::cmp::min(min_depth, depth);
            min_common_parent = cur;
        }
        for child in cur.orbiters.iter() {
            iter_stack.push((child, depth + 1));
        }
    }
    min_common_parent
}

fn depth_of_node(name: String, root_node: &Box<OrbitNode>) -> i32 {
    let mut iter_stack = vec![(root_node, 0)];
    let mut res_depth: i32 = -1;
    while !iter_stack.is_empty() {
        let (cur, depth) = iter_stack.pop().unwrap();
        if cur.name == name {
            res_depth = depth;
        }
        for child in cur.orbiters.iter() {
            iter_stack.push((child, depth + 1));
        }
    }
    res_depth
}

fn count_orbits(root_node: &Box<OrbitNode>) -> i32 {
    let mut iter_stack = vec![(root_node, 0)];
    let mut depths: i32 = 0;
    while !iter_stack.is_empty() {
        let (cur, depth) = iter_stack.pop().unwrap();
        depths += depth;
        for child in cur.orbiters.iter() {
            iter_stack.push((child, depth + 1));
        }
    }

    depths
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide one argument with path to the program");
    }

    let file = std::fs::File::open(&args[1]).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut file_string = String::new();
    reader
        .read_to_string(&mut file_string)
        .expect("Unable to read file");

    let map = make_node_map(&file_string);
    let root = build_tree(&map);

    println!("{}", count_orbits(&root));

    let common_parent = naive_common_parent("YOU".to_string(), "SAN".to_string(), &root);
    let depth_you = depth_of_node("YOU".to_string(), common_parent);
    let depth_san = depth_of_node("SAN".to_string(), common_parent);
    println!("Transfers: {}", depth_you + depth_san - 2);
}

#[cfg(test)]
mod tests {
    use build_tree;
    use count_orbits;
    use depth_of_node;
    use make_node_map;
    use naive_common_parent;

    #[test]
    fn test1() {
        let raw_test_str: String =
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string();

        let map = make_node_map(&raw_test_str);
        let root = build_tree(&map);
        assert_eq!(count_orbits(&root), 42);
    }

    #[test]
    fn test2() {
        let raw_test_str: String =
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string();

        let map = make_node_map(&raw_test_str);
        let root = build_tree(&map);
        assert_eq!(
            naive_common_parent("L".to_string(), "I".to_string(), &root).name,
            "D"
        );
    }

    #[test]
    fn test3() {
        let raw_test_str: String =
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string();

        let map = make_node_map(&raw_test_str);
        let root = build_tree(&map);
        let common_parent = naive_common_parent("L".to_string(), "I".to_string(), &root);
        let depth_1 = depth_of_node("L".to_string(), common_parent);
        let depth_2 = depth_of_node("I".to_string(), common_parent);
        assert_eq!(depth_1 + depth_2 - 2, 3);
    }
}
