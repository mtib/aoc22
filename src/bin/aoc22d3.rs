use std::{fs, path::PathBuf};

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(root_dir);
    file_path.push("./res/d3elf_backpacks.txt");

    let backpacks_txt =
        fs::read_to_string(file_path.to_str().expect("file path could not be parsed"))
            .expect("input could not be read");
    let backpacks_slice = backpacks_txt.as_str();
    let backpacks_iter = backpacks_slice.split("\n").map(backpacks::Backpack::from);

    let sum_of_prios = backpacks_iter
        .clone()
        .map(|b| {
            b.items_in_both_compartments()
                .into_iter()
                .map(|c| backpacks::item_priority(c))
                .sum::<isize>()
        })
        .sum::<isize>();

    println!("Sum of priorities of dups {}", sum_of_prios);

    let backpacks_loop_iter = &mut backpacks_iter.clone();
    let mut badges = Vec::new();
    loop {
        let mut group: Vec<backpacks::Backpack> = backpacks_loop_iter.take(3).collect();
        if group.len() != 3 {
            break;
        }
        let badge = {
            let mut chars = Vec::new();
            let first = group
                .pop()
                .expect("there must be at least one member in the group");
            chars.extend(first.0.chars());
            chars.extend(first.1.chars());
            chars.sort();
            chars.dedup();

            for other in group {
                let mut other_chars: Vec<char> = other.0.chars().chain(other.1.chars()).collect();
                other_chars.sort();
                other_chars.dedup();
                chars = chars
                    .into_iter()
                    .filter(|c| other_chars.contains(c))
                    .collect();
            }

            if chars.len() != 1 {
                panic!("There are too many chars left {:?}", chars);
            }

            *chars.get(0).unwrap()
        };
        badges.push(badge);
    }
    println!(
        "Sum of badge priorities: {}",
        badges
            .into_iter()
            .map(backpacks::item_priority)
            .sum::<isize>()
    );
}

mod backpacks {
    #[derive(Debug)]
    pub struct Backpack<'a>(pub &'a str, pub &'a str);

    impl<'a> From<&'a str> for Backpack<'a> {
        fn from(value: &'a str) -> Self {
            Backpack(&value[0..value.len() / 2], &value[value.len() / 2..])
        }
    }

    impl<'a> Backpack<'a> {
        pub fn items_in_both_compartments(&self) -> Vec<char> {
            let mut duplicates: Vec<char> =
                self.0.chars().filter(|&c| self.1.contains(c)).collect();
            duplicates.sort();
            duplicates.dedup();
            duplicates
        }
    }

    const PRIORITIES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    pub fn item_priority(item: char) -> isize {
        PRIORITIES
            .chars()
            .enumerate()
            .find_map(|(priority, iter_item)| {
                if iter_item == item {
                    Some(priority as isize + 1)
                } else {
                    None
                }
            })
            .expect(&format!("Illegal item: {}", item))
    }
}
