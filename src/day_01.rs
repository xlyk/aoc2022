use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut dict = HashMap::new();
    let mut idx: i32 = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            idx += 1;
            continue;
        }

        let count = &line.parse::<i32>().unwrap();
        let existing = dict.get(&idx).unwrap_or(&0);
        dict.insert(idx, count + existing);
    }

    // get values from vector
    let mut v: Vec<i32> = dict.into_iter().map(|(_k, v)| v).collect();

    // reverse sort
    v.sort_by(|a, b| b.cmp(a));
    println!("v: {:?}", v);

    // get slice of top N items
    let top_items = &v[0..=2];

    // get sum of top N items
    let sum: i32 = top_items.iter().sum();

    println!("sum: {:?}", sum);

    Ok(())
}
