use std::collections::HashMap;
use crate::tools::get_input_or_panic;

#[allow(dead_code)]
pub fn day8() {
    day8_1();
    day8_2();
}

#[derive(Debug)]
struct Node {
    name : String,
    left : String,
    right : String,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(())
        }
    }
}

struct Input {
    directions : Directions,
    nodes : HashMap<String, Node>,
}

#[derive(Debug, Clone)]
struct Directions(Vec<Direction>, usize);

impl From<Vec<Direction>> for Directions {
    fn from(value: Vec<Direction>) -> Self {
        Directions(value, 0)
    }
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0[self.1].clone();
        self.1 += 1;
        if self.1 == self.0.len() {
            self.1 = 0;
        }
        Some(next)
    }
}

impl Node {
    fn try_parse(value: &str) -> Option<Node> {
        // Input: "TJS = (LFP, HKT)"
        //         0  3   7  10^13^16
        // Sine the distance is always fixed, we can parse these via slices
        // let's do some asserts to make sure we're right :-)
        assert_eq!(value.len(), 16);
        assert_eq!(&value[3..7], " = (");
        assert_eq!(&value[10..12], ", ");
        assert_eq!(&value[15..], ")");

        // with that out of the way, things get easy!
        Some(Node {
            name: String::from(&value[0..3]),
            left: String::from(&value[7..10]),
            right: String::from(&value[12..15]),
        })
    }
}

fn parse_input(input : &String) -> Option<Input> {
    let mut lines = input.lines();
    assert!(lines.clone().count() > 3);
    let directions = lines
        .next()?
        .chars()
        .map(Direction::try_from)
        .map(Result::ok)
        .collect::<Option<Vec<Direction>>>()?
        .into();

    let next = lines.next();
    assert!(next.unwrap().is_empty());

    let mut nodes = HashMap::new();
    while let Some(node) = lines.next() {
        let node = Node::try_parse(node)?;
        nodes.insert(node.name.clone(), node);
    }

    Some(Input { directions , nodes })
}

fn follow_directions(mut input: Input) -> Option<u64> {
    let limit = 1_000_000; // just to be safe :-)
    let mut current_node = input.nodes.get("AAA").expect("Start node not found!");
    let mut counter = 0;
    while counter < limit {
        current_node = input.nodes.get(
            &*match input.directions.next()? {
                Direction::Left => &current_node.left,
                Direction::Right => &current_node.right,
            }
        ).expect(&*format!("We have reached an unknown node: {:?}!", current_node));
        counter += 1;

        if current_node.name == "ZZZ" {
            return Some(counter);
        }
    }

    panic!("Iteration limit reached!");
}

fn day8_1() {
    let input = get_input_or_panic("8-1");
    let data = parse_input(&input).unwrap();
    let steps = follow_directions(data).unwrap();

    assert_eq!(steps, 19951);
    println!("Number of steps: {steps}")
}

fn day8_2() {
    let input = get_input_or_panic("8-1");
}