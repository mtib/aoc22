use std::{fs, path::PathBuf};

type Calories = usize;

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(root_dir);
    file_path.push("./res/d1elf_calories.txt");

    let foo: String =
        fs::read_to_string(file_path.to_str().expect("file path could not be parsed"))
            .expect("input could not be read");

    let mut ranked_iter: Vec<Calories> = foo
        .split("\n\n")
        .map(|rows| {
            rows.split("\n")
                .map(|num_str| num_str.parse::<Calories>().unwrap_or(0))
                .sum()
        })
        .collect();
    ranked_iter.sort_by(|cal1, cal2| cal1.cmp(cal2).reverse());

    let top_three = &ranked_iter[0..3];
    println!("#1 elf carries {} calories", top_three.get(0).unwrap_or(&0));
    println!(
        "top three elves carry {} together",
        top_three.iter().sum::<usize>()
    );
}
