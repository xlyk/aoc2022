use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut total: i32 = 0;

    let alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();

    let file = File::open("inputs/day_03a.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (a, b) = get_compartments(line.as_str());

        let matches = get_matches(a, b);

        for m in matches {
            let mut index = alphabet
                .iter()
                .position(|&r| r.to_string() == m.to_lowercase().to_string())
                .unwrap() + 1;

            if m.is_uppercase() {
                index += 26;
            }

            total += index as i32;
        }
    }

    let _ = dbg!(total);

    Ok(())
}

fn get_compartments(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn get_matches(a: &str, b: &str) -> Vec<char> {
    let mut map1: HashMap<char, i32> = HashMap::new();
    for c in a.chars() {
        map1.insert(c, 1);
    }

    let mut map2: HashMap<char, i32> = HashMap::new();
    for c in b.chars() {
        if map1.get(&c).unwrap_or(&0) > &0 {
            map2.insert(c, 1);
        }
    }

    let mut ret = Vec::new();
    for (k, v) in map2.into_iter() {
        if v > 0 {
            ret.push(k);
        }
    }

    ret
}
