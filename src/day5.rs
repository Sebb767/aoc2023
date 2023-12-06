use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::tools::{get_input_or_panic, string_with_spaces_and_numbers_to_vec_of_numbers};

#[derive(Debug, Copy, Clone)]
struct SeedRange {
    start : i64,
    end : i64
}

#[derive(Debug)]
struct MapperRange {
    start_input : i64,
    start_output : i64,
    range : i64
}

#[derive(Debug)]
struct Mapper {
    from : String,
    to : String,
    ranges : Vec<MapperRange>,
}

#[derive(Debug)]
struct DayFiveInput {
    seeds : Vec<i64>,
    mapper : HashMap<String, Mapper>,
}

#[derive(Debug)]
struct SeedRangeMapResult {
    lower : Option<SeedRange>,
    mapped : Option<SeedRange>,
    higher : Option<SeedRange>,
}

#[derive(Debug)]
struct ParseRangeError;
impl FromStr for MapperRange {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // input must look like: "0 1894195346 315486903"
        // Values are destination start, source start, range
        let numbers : Result<Vec<i64>, ParseIntError> = s
            .split(' ')
            .map(|s| i64::from_str_radix(s, 10))
            .collect();
        if numbers.is_err() {
            return Err(ParseRangeError);
        }

        let [start_output, start_input, range] = numbers.unwrap()[..] else { return Err(ParseRangeError); };
        Ok(MapperRange {
            start_input,
            start_output,
            range
        })
    }
}

impl MapperRange {
    fn map_seed_range(&self, mut input : SeedRange) -> SeedRangeMapResult {
        let mut result = SeedRangeMapResult { lower: None, mapped: None, higher: None };
        let end_input = self.start_input + self.range - 1;

        if input.end < self.start_input {
            // all values are below our range
            result.lower = Some(input);
            return result;
        }
        else if input.start > end_input {
            // all values are above our range
            result.higher = Some(input);
            return result;
        }

        if input.start < self.start_input {
            // We know that not all values are below our input, since otherwise we would have exited
            // in the line above
            result.lower = Some(SeedRange {
               start: input.start,
                end: self.start_input - 1
            });
            input.start = self.start_input
        }

        if input.end > end_input {
            result.higher = Some(SeedRange {
                start: end_input + 1,
                end: input.end
            });
            input.end = end_input;
        }

        result.mapped = Some(SeedRange {
            start: input.start + self.start_output - self.start_input,
            end: input.end + self.start_output - self.start_input
        });

        result
    }
}

impl Mapper {
    fn map_range(&self, input : Vec<i64>) -> Vec<i64> {
        return input.into_iter().map(|num| {
            for mapper in self.ranges.iter() {
                if num < mapper.start_input {
                    // mappers are sorted by start input, so if we didn't match by the time our
                    // start inputs are larger than the number, no mapper matches
                    break;
                }
                // we now know num >= start input from the previous clause
                // so if we're in this non-inclusive range, we match
                else if num < mapper.start_input + mapper.range {
                    return num + mapper.start_output - mapper.start_input;
                }
                // implicit no-match-continue
            }
            num
        }).collect();
    }

    fn map_seed_ranges(&self, input : Vec<SeedRange>) -> Vec<SeedRange> {
        let mut result = Vec::new();
        let mut iter = input.iter();

        for seed_range in input.iter() {
            let mut remainder = (*seed_range).clone();
            for mapper in self.ranges.iter() {
                let SeedRangeMapResult { lower, mapped, higher } = mapper.map_seed_range(remainder);
                if lower.is_some() {
                    // Mappers are ordered by the start value, so lower values will not be matched
                    // by the remaining mappers
                    result.push(lower.unwrap());
                }
                if mapped.is_some() {
                    result.push(mapped.unwrap());
                }
                if higher.is_some() {
                    remainder = higher.unwrap();
                }
                else {
                    // Again, since our mappers are sorted by start value, if no values remain above
                    // the current mapper, the later mappers will never match
                    break;
                }
            }
        }

        result
    }
}

fn parse_mapper(lines : &mut std::str::Lines) -> Option<Mapper> {
    let header = lines.next()?;
    // "seed-to-soil map:" -> ["seed", "to", "soil", "map:"]
    let header_parts = header.split(['-', ' ']);
    let [ ref from, _, ref to, _ ] = header_parts.map(String::from).collect::<Vec<_>>()[..] else { return None; };
    let mut ranges = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        ranges.push(MapperRange::from_str(line).ok()?);
    }

    ranges.sort_by(|r1, r2 | r1.start_input.cmp(&r2.start_input));
    //println!("{from}-to-{to}-mapper: {:?}", ranges);

    Some(Mapper{
        from: (*(from.clone())).parse().unwrap(),
        to: (*(to.clone())).parse().unwrap(),
        ranges
    })
}

fn parse_day5_input(input : String) -> Option<DayFiveInput> {
    let mut iter = input.lines();

    let mut seeds : Vec<i64>;
    {
        let seeds_line = iter.next()?;
        assert!(seeds_line.starts_with("seeds: "));
        seeds = string_with_spaces_and_numbers_to_vec_of_numbers(&seeds_line[7..]).ok()?;
    }

    {
        let empty_line = iter.next()?;
        assert!(empty_line.is_empty());
    }

    let mut mapper_map : HashMap<String, Mapper> = HashMap::new();
    while let Some(mapper) = parse_mapper(&mut iter) {
        mapper_map.insert(mapper.from.clone(), mapper);
    }

    return Some(DayFiveInput {
        seeds,
        mapper: mapper_map
    })
}

fn seed_range_input_to_seeds(input : Vec<i64>) -> Vec<SeedRange> {
    let mut output = Vec::with_capacity(input.capacity()/2);

    let mut iter =  input.iter();
    while let Some(seed) = iter.next() {
        let range = iter.next().unwrap();
        output.push(SeedRange { start: *seed, end: seed + range - 1 });
    }

    output
}

fn run_mappers(input : DayFiveInput) -> (String, Vec<i64>) {
    let mut category = "seed";
    let mut seeds = input.seeds;
    while let Some(mapper) = input.mapper.get(category) {
        //println!("Mapping from {category} to {}", mapper.to);
        seeds = mapper.map_range(seeds);
        category = mapper.to.as_str();
    }

    (String::from(category), seeds)
}

fn run_mappers_seed_ranges(mut input : Vec<SeedRange>, mapper : HashMap<String, Mapper>) -> (String, Vec<SeedRange>) {
    let mut category = "seed";
    while let Some(mapper) = mapper.get(category) {
        //println!("Mapping from {category} to {}", mapper.to);
        input = mapper.map_seed_ranges(input);
        category = mapper.to.as_str();
    }

    (String::from(category), input)
}

#[allow(dead_code)]
pub fn day5() {
    day5_1();
    day5_2();
}

fn day5_1() {
    let input = get_input_or_panic("5-1");
    let data = parse_day5_input(input).unwrap();

    let (category, seeds) = run_mappers(data);


    println!("Seeds as {category} after final transformation: {:?}", seeds);
    let min = *seeds.iter().min().unwrap();
    assert_eq!(min, 265018614);
    println!("Min {category}: {min}");
}

fn day5_2() {
    let input = get_input_or_panic("5-1");
    let DayFiveInput{ seeds, mapper } = parse_day5_input(input).unwrap();

    let transformed_seeds = seed_range_input_to_seeds(seeds);
    let (category, seeds) = run_mappers_seed_ranges(transformed_seeds, mapper);

    println!("Seed ranges as {category} after final transformation: {:?}", seeds);
    let min = seeds.iter().map(|sr| sr.start).min().unwrap();
    //assert_eq!(min, 265018614);
    println!("Min {category} range mapping: {min}");
}