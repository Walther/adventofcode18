struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let numbers: Vec<i32> = INPUT.split(" ").map(|x| x.parse().unwrap()).collect();

    let (node, _) = parse(&numbers, 0);
    let part1 = metadata_sum(&node);
    println!("{}", part1);
    let part2 = values(&node);
    println!("{}", part2);
}

fn parse(numbers: &Vec<i32>, mut position: usize) -> (Node, usize) {
    let mut node = Node::new();

    let num_children = numbers[position];
    position += 1;
    let num_metadata = numbers[position];
    position += 1;

    while node.children.len() < num_children as usize {
        let (child, new_pos) = parse(&numbers, position);
        node.children.push(child);
        position = new_pos;
    }

    while node.metadata.len() < num_metadata as usize {
        let meta = numbers[position] as usize;
        node.metadata.push(meta);
        position += 1;
    }

    (node, position)
}

fn metadata_sum(node: &Node) -> usize {
    node.metadata.iter().sum::<usize>()
        + node
            .children
            .iter()
            .map(|child| metadata_sum(child))
            .sum::<usize>()
}

fn values(node: &Node) -> usize {
    let value = match node.children.len() {
        0 => node.metadata.iter().sum::<usize>(),
        _ => node.metadata.iter().fold(0, |acc, &meta| {
            if meta == 0 || meta > node.children.len() {
                acc
            } else {
                acc + values(&node.children[meta - 1])
            }
        }),
    };

    value
}
