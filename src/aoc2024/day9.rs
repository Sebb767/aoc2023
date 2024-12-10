use std::collections::VecDeque;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

pub(crate) struct Day9;

type FsBlock = Option<u64>;

type Fs = Box<Vec<FsBlock>>;

#[derive(Copy, Clone)]
struct Fs2Block {
    id: Option<u64>, // None if free space
    size: u64,
}

impl Fs2Block {
    fn is_free_space(&self) -> bool { self.id.is_none() }
}

type Fs2 = Box<VecDeque<Fs2Block>>;

fn preprocess_input2(input: &String) -> Fs2 {
    let mut fs: Fs2 = Box::new(VecDeque::new());
    let numbers: Vec<u32> = input.chars().map(|c| c.to_digit(10)).collect::<Option<Vec<u32>>>().unwrap();
    let mut is_file = true;
    let mut file_number = 0u64;

    for number in numbers {
        let id = if is_file {
            let rv = Some(file_number);
            file_number += 1;
            is_file = false;
            rv
        } else {
            is_file = true;
            None
        };

        fs.push_back(Fs2Block { id, size: number as u64 });
    }

    fs
}

fn preprocess_input(input: &String) -> Fs {
    let mut fs: Fs = Box::new(Vec::new());
    let numbers: Vec<u32> = input.chars().map(|c| c.to_digit(10)).collect::<Option<Vec<u32>>>().unwrap();
    let mut is_file = true;
    let mut file_number = 0u64;

    for number in numbers {
        if is_file {
            for _ in 0..number {
                fs.push(Some(file_number));
            }
            file_number += 1;

            is_file = false;
        } else {
            for _ in 0..number {
                fs.push(None);
            }
            is_file = true;
        }
    }

    fs
}

fn insert_fs_block_in_free_space(l: &mut Fs2, idx: usize, block: Fs2Block) {
    let mut tail = l.split_off(idx);
    let mut free_space = tail.pop_front().unwrap();

    assert!(free_space.is_free_space(), "Trying to replace a non-free space");
    assert!(free_space.size >= block.size, "Trying to replace free space with unfitting block");

    free_space.size -= block.size;
    l.push_back(block);
    if free_space.size > 0 {
        l.push_back(free_space);
    }

    l.append(&mut tail);
}

fn remove_fs_block(l: &mut Fs2, idx: usize, mut size_of_block: u64) {
    assert!(idx > 0);
    let mut tail = l.split_off(idx - 1);

    let prev = tail.pop_front().unwrap();
    if prev.is_free_space() {
        size_of_block += prev.size;
    } else {
        l.push_back(prev);
    }

    // Remove the element we're replacing
    tail.pop_front();

    if let Some(following) = tail.get(0) {
        if following.is_free_space() {
            size_of_block += following.size;
            tail.pop_front();
        }
    }

    l.push_back(Fs2Block {
        id: None,
        size: size_of_block,
    });

    l.append(&mut tail);
}

fn checksum_fs(fs: &Fs) -> DayResult {
    let mut result = 0;
    for (idx, block) in fs.iter().enumerate() {
        if let Some(file_id) = block {
            result += (idx as DayResult) * (*file_id as DayResult);
        }
    }
    result
}

fn checksum_fs2(fs: &Fs2) -> DayResult {
    let mut result = 0;
    let mut index = 0;

    for block in fs.iter() {
        if block.is_free_space() {
            index += block.size;
        } else {
            for _ in 1..(block.size + 1) {
                result += index * block.id.unwrap();
                index += 1;
            }
        }
    }

    result as DayResult
}

#[allow(dead_code)]
fn display_fs(fs: &Fs) {
    for item in fs.iter() {
        match item {
            None => print!("."),
            Some(block) => print!("{block}")
        }
    }
    println!();
}

#[allow(dead_code)]
fn display_fs2(fs: &Fs2) {
    for block in fs.iter() {
        for _ in 0..block.size {
            if block.is_free_space() {
                print!(".");
            } else {
                print!("{}", block.id.unwrap());
            }
        }
    }

    println!();
}

impl Day for Day9 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let fs = preprocess_input(&input);
        let mut result = fs.clone();
        let mut iter = fs.iter().enumerate();
        let mut tail = iter.next_back().unwrap();

        'outer: while let Some((idx, block)) = iter.next() {
            if block.is_some() {
                continue;
            }

            // we found a free space, we need to swap it with an end
            while tail.1.is_none() {
                if let Some(next) = iter.next_back() {
                    tail = next
                } else {
                    // we've reached the end
                    break 'outer;
                }
            }

            result.swap(idx, tail.0);
            //display_fs(&result);

            if let Some(next) = iter.next_back() {
                tail = next;
            } else {
                break;
            }
        }

        Some(checksum_fs(&result))
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let mut fs = preprocess_input2(&input);
        let mut index = fs.len() - 1;
        //display_fs2(&fs);

        while index > 0 {
            let next = *fs.get(index).unwrap();
            if !next.is_free_space() {
                let mut index_back = 0;

                while index > index_back {
                    let block = fs.get(index_back).unwrap();
                    if block.is_free_space() {
                        let block = *block;
                        if block.size >= next.size {
                            remove_fs_block(&mut fs, index, next.size);
                            insert_fs_block_in_free_space(&mut fs, index_back, next);
                            //display_fs2(&fs);
                            break;
                        }
                    }

                    index_back += 1;
                }
            }

            index -= 1;
        }

        //unimplemented!();
        Some(checksum_fs2(&fs))
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(1928, 6242766523059, 2858, 6272188244509)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(9)
    }

    fn part1_result_description(&self) -> String {
        String::from("Checksum of defragmented filesystem")
    }

    /*fn part2_result_description(&self) -> String {
        String::from("")
    }*/
}