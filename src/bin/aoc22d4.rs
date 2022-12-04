use std::{fs, ops::RangeInclusive, path::PathBuf};

use regex::Regex;

fn range_contain_in_other(range1: &RangeInclusive<isize>, range2: &RangeInclusive<isize>) -> bool {
    fn r1_in_r2(r1: &RangeInclusive<isize>, r2: &RangeInclusive<isize>) -> bool {
        r1.start() >= r2.start() && r1.end() <= r2.end()
    }
    r1_in_r2(range1, range2) || r1_in_r2(range2, range1)
}

fn range_overlap(range1: &RangeInclusive<isize>, range2: &RangeInclusive<isize>) -> bool {
    (range1.start() <= range2.start() && range1.end() >= range2.start())
        || (range2.start() <= range1.start() && range2.end() >= range1.start())
}

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(root_dir);
    file_path.push("./res/d4elf_assignments.txt");

    let assignments_txt =
        fs::read_to_string(file_path.to_str().expect("file path could not be parsed"))
            .expect("input could not be read");

    let pair_regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("Failed to compile regex");
    let pairs_iter = assignments_txt.split("\n").map(|pair_txt| {
        let captures = pair_regex.captures(pair_txt).expect("Failed to capture");
        (
            captures.get(1).unwrap().as_str().parse::<isize>().unwrap()
                ..=captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<isize>().unwrap()
                ..=captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        )
    });

    println!(
        "Number of pairs fully conatined in each other: {}",
        pairs_iter
            .clone()
            .filter(|(r1, r2)| range_contain_in_other(r1, r2))
            .count()
    );
    println!(
        "Number of pairs that overlap at all: {}",
        pairs_iter.filter(|(r1, r2)| range_overlap(r1, r2)).count()
    );
}
