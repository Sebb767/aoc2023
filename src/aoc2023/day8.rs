#![allow(clippy::all)]
use crate::tools::get_input_or_panic;
use std::collections::HashMap;
use std::ops::{Div, Mul, Sub};

#[allow(dead_code)]
pub fn day8() {
    day8_1();
    day8_2();
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    directions: Directions,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
struct Directions(Vec<Direction>, usize);

impl Directions {
    fn get_offset(&self) -> usize {
        return self.1;
    }
}

impl From<Vec<Direction>> for Directions {
    fn from(value: Vec<Direction>) -> Self {
        Directions(value, 0)
    }
}

#[derive(Debug, Copy, Clone)]
struct Path<'a> {
    current: &'a Node,
    lookup: &'a HashMap<String, Node>,
}

#[derive(Debug)]
struct Loop {
    offset: usize,
    length: usize,
}

type PathError = ();
impl Path<'_> {
    fn new<'a>(start: &'a Node, lookup: &'a HashMap<String, Node>) -> Path<'a> {
        return Path {
            current: start,
            lookup,
        };
    }
    fn advance(&mut self, direction: Direction) -> Result<(), PathError> {
        let next = self.current.lookup(direction, &self.lookup);
        return if let Some(next) = next {
            self.current = next;
            Ok(())
        } else {
            Err(())
        };
    }

    fn is_end_node(&self) -> bool {
        return self.current.name.chars().nth(2).unwrap() == 'Z';
    }

    fn find_loop(&self, mut directions: Directions) -> Option<Loop> {
        let mut copy = (*self).clone();
        let limit = 100_000; // due to directions, loops can be longer than our amount of nodes

        // We found a loop when we arrived at the same node again with the same offset in the
        // pathing loop
        let mut visited = HashMap::<(&String, usize), usize>::new();
        let mut steps = 0;
        let mut end_node_offset: Option<usize> = None;

        // We will loop until we find the first visited node
        while let Some(dir) = directions.next() {
            if copy.is_end_node() {
                if end_node_offset.is_some() {
                    // turns out this doesn't happen, making the whole thing a lot easier
                    panic!("Found loop with multiple end nodes!");
                } else {
                    end_node_offset = Some(steps);
                }
            }

            let key = (&copy.current.name, directions.get_offset());
            if visited.contains_key(&key) {
                let offset = visited.get(&key).unwrap();
                // We found the loop!
                // The loop offset is the number of steps to the current node and the loop length is
                // the difference from our counter to the offset. However, what we actually want is
                // the offset to the first end node, since that's technically our first loop end.
                if end_node_offset.is_none() {
                    // Luckily it turns out this doesn't happen, either
                    panic!("Found loop without end node!");
                }

                return Some(Loop {
                    offset: end_node_offset.unwrap(),
                    length: steps - *offset,
                });
            } else if steps > limit {
                // we found no loop, give up
                break;
            } else {
                visited.insert(key, steps);
            }

            Self::advance(&mut copy, dir).expect("Failed to advance!");
            steps += 1;
        }

        panic!("Found no loop, giving up!");
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

    fn lookup<'a>(
        &'a self,
        direction: Direction,
        lookup: &'a HashMap<String, Node>,
    ) -> Option<&Node> {
        lookup.get(&*match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        })
    }
}

fn parse_input(input: &String) -> Option<Input> {
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

    Some(Input { directions, nodes })
}

fn follow_directions(mut input: Input) -> Option<u64> {
    let limit = 1_000_000; // just to be safe :-)
    let mut current_node = input.nodes.get("AAA").expect("Start node not found!");
    let mut counter = 0;
    while counter < limit {
        current_node = current_node
            .lookup(input.directions.next()?, &input.nodes)
            .expect(&*format!(
                "We have reached an unknown node: {:?}!",
                current_node
            ));
        counter += 1;

        if current_node.name == "ZZZ" {
            return Some(counter);
        }
    }

    panic!("Iteration limit reached!");
}

fn follow_ghost_directions(input: Input) -> Option<usize> {
    let loops = input
        .nodes
        .iter()
        .filter(|(_, node)| node.name.chars().nth(2).unwrap() == 'A')
        .map(|(_, node)| Path::new(node, &input.nodes))
        .map(|path| path.find_loop(input.directions.clone()))
        .collect::<Option<Vec<Loop>>>()?;
    //loops.iter().for_each(|lp|  println!("{:?}", lp));

    // Turns out, the following is true, making the problem a lot easier:
    assert!(loops.iter().all(|l| l.offset == l.length));

    // Now we just need to find the lowest common denominator
    let lcm = loops
        .iter()
        .map(|l| l.length)
        .reduce(lowest_common_multiple)?;

    Some(lcm)
}

fn lowest_common_multiple<T>(a: T, b: T) -> T
where
    T: Eq,
    T: Ord,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Copy,
{
    (a * b) / greatest_common_divisor(a, b)
}

fn greatest_common_divisor<T>(mut a: T, mut b: T) -> T
where
    T: Eq,
    T: Ord,
    T: Sub<Output = T>,
    T: Copy,
{
    while a != b {
        if b > a {
            (b, a) = (a, b);
        }
        a = a - b;
    }
    a
}

fn day8_1() {
    let input = get_input_or_panic("8-1", 2023);
    let data = parse_input(&input).unwrap();
    let steps = follow_directions(data).unwrap();

    assert_eq!(steps, 19951);
    println!("Number of steps: {steps}")
}

fn day8_2() {
    let input = get_input_or_panic("8-1", 2023);
    let data = parse_input(&input).unwrap();
    let steps = follow_ghost_directions(data).unwrap();

    assert_eq!(steps, 16342438708751);
    println!("Number of steps: {steps}")
}
