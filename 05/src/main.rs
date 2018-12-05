fn main() {
    const INPUT: &str = include_str!("input.txt");
    let inputstring: String = String::from(INPUT);

    // part 1
    let polymer = polymerize(&inputstring);
    println!("{}", polymer.len());

    // part 2
    let polymer_a = polymerize(&filter_unit(&inputstring, 'a'));
    let polymer_b = polymerize(&filter_unit(&inputstring, 'b'));
    let polymer_c = polymerize(&filter_unit(&inputstring, 'c'));
    let polymer_d = polymerize(&filter_unit(&inputstring, 'd'));

    let lengths: Vec<usize> = [polymer_a, polymer_b, polymer_c, polymer_d]
        .iter()
        .map(|polymer| polymer.len())
        .collect();
    let shortest = lengths.iter().min().unwrap();
    println!("{}", shortest)
}

fn polymerize(string: &String) -> String {
    let chars = string.chars();
    // Poor man's stack implementation
    let mut polymer: Vec<char> = Vec::new();
    let mut last: char = ' ';
    for letter in chars {
        if letter.is_lowercase() != last.is_lowercase()
            && letter.to_lowercase().to_string() == last.to_lowercase().to_string()
        {
            polymer.pop();
            last = match polymer.last() {
                Some(c) => *c,
                None => ' ',
            };
            continue;
        } else {
            polymer.push(letter);
            last = letter;
        }
    }
    polymer.iter().collect()
}

fn filter_unit(string: &String, remove: char) -> String {
    let chars: Vec<char> = string.chars().collect();
    chars
        .iter()
        .filter(|letter| letter.to_lowercase().to_string() != remove.to_string())
        .collect()
}
