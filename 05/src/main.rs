fn main() {
    const INPUT: &str = include_str!("input.txt");
    let chars: Vec<char> = INPUT.chars().collect();

    // part 1
    let polymer = polymerize(chars);
    println!("{}", polymer.len());

    // part 2
    let chars_without_a = INPUT
        .chars()
        .filter(|letter| letter.to_lowercase().to_string() != "a")
        .collect();

    let chars_without_b = INPUT
        .chars()
        .filter(|letter| letter.to_lowercase().to_string() != "b")
        .collect();
        ;
    let chars_without_c = INPUT
        .chars()
        .filter(|letter| letter.to_lowercase().to_string() != "c")
        .collect();

    let chars_without_d = INPUT
        .chars()
        .filter(|letter| letter.to_lowercase().to_string() != "d")
        .collect();

    let polymer_a = polymerize(chars_without_a);
    let polymer_b = polymerize(chars_without_b);
    let polymer_c = polymerize(chars_without_c);
    let polymer_d = polymerize(chars_without_d);

    let lengths: Vec<usize> = [polymer_a, polymer_b, polymer_c, polymer_d]
        .iter()
        .map(|polymer| polymer.len())
        .collect();
    let shortest = lengths.iter().min().unwrap();
    println!("{}", shortest)
}

fn polymerize(chars: Vec<char>) -> Vec<char> {
    // Poor man's stack implementation
    let mut polymer: Vec<char> = Vec::new();
    let mut last: char = ' ';
    for letter in chars {
        if letter.to_lowercase().to_string() == last.to_lowercase().to_string()
            && letter.is_lowercase() != last.is_lowercase()
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
    polymer
}
