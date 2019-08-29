pub fn parse_string(raw: &str) -> Vec<usize> {
    raw.split(' ').map(|num| num.parse().unwrap()).collect()
}

#[derive(Debug, PartialEq)]
struct Node {
    children: Vec<Node>,
    meta: Vec<usize>
}
impl Node {
    fn first_check(&self) -> usize {
        self.meta.iter().sum::<usize>() +
        self.children.iter().map(|node| node.first_check()).sum::<usize>()
    }

    fn second_check(&self) -> usize {
        match self.children.is_empty() {
            true => self.meta.iter().sum(),
            false => {
                let mut sum = 0;
                for m in &self.meta {
                    match self.children.get(m-1) {
                        None => (),
                        Some(node) => sum += node.second_check()
                    }
                }
                sum
            }
        }
    }

    fn parse_node(input: &[usize]) -> (Node, usize) {
        let children_count = input[0];
        let meta_count = input[1];

        let mut consumed = 2;
        let mut children = vec![];
        let meta: Vec<usize>;

        if children_count == 0 {
            meta = input[consumed..(consumed+meta_count)].to_vec();
            consumed += meta_count;
        } else {
            for _i in 0..children_count {
                let (node, c) = Self::parse_node(&input[consumed..]);
                children.push(node);
                consumed += c;
            }
            meta = input[consumed..consumed+meta_count].to_vec();
            consumed += meta_count;
        } 

        (Node { children, meta }, consumed)
    }
}

#[derive(Debug, PartialEq)]
pub struct Tree {
    root: Node
}
impl Tree {
    pub fn new(input: Vec<usize>) -> Self {
        let (root, _consumed) = Node::parse_node(&input);
        Self { root }
    }

    pub fn first_check(&self) -> usize {
        self.root.first_check()
    }

    pub fn second_check(&self) -> usize {
        self.root.second_check()
    }
}

#[test]
fn test_parse_raw_string() {
    let test_data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let parsed = parse_string(test_data);

    assert_eq!(parsed, vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2])
}

#[test]
fn test_tree_building() {
    let test_vector = parse_string("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    let tree = Tree::new(test_vector);

    let expected_tree = Tree {
        root: Node {
            children: vec![
                Node {
                    children: vec![],
                    meta: vec![10, 11, 12]
                },
                Node {
                    children: vec![
                        Node {
                            children: vec![],
                            meta: vec![99]
                        }
                    ],
                    meta: vec![2]
                }
            ],
            meta: vec![1, 1, 2]
        }
    };

    let expected_first_check = 138;
    let expected_second_check = 66;

    assert_eq!(tree, expected_tree);
    assert_eq!(tree.first_check(), expected_first_check);
    assert_eq!(tree.second_check(), expected_second_check)
}
