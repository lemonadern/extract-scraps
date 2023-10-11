use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Page {
    title: String,
    lines: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScrapboxJson {
    pages: Vec<Page>,
}

fn main() {
    let input_file = File::open("lemonadern.json").unwrap();
    let data: ScrapboxJson = serde_json::from_reader(input_file).unwrap();

    let extracted: Vec<Page> = data
        .pages
        .iter()
        .filter(|Page { title, lines }| {
            title.contains("lab") || contains_pattern_in_lines(lines, "#hirano-lab")
        })
        .cloned()
        .collect();

    let serialised = serde_json::to_string(&ScrapboxJson { pages: extracted }).unwrap();

    let mut output_file = File::create("extracted.json").unwrap();
    output_file.write_all(serialised.as_bytes()).unwrap();

    println!("Serializing finished!");
}

fn contains_pattern_in_lines(lines: &Vec<String>, pattern: &str) -> bool {
    for line in lines {
        if line.contains(pattern) {
            return true;
        }
    }
    return false;
}
