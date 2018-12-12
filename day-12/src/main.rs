use regex::Regex;
use std::collections::HashMap;

type Pots = HashMap<i32, bool>;
type Rule = (bool, bool, bool, bool, bool);
type Rules = HashMap<Rule, bool>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut lines = INPUT.lines();

    // Initial state
    let line = lines.next().unwrap();
    let regex = Regex::new(r"initial state: (.*)").unwrap();
    let captures = regex.captures(line).unwrap();
    let state_string = captures[1].trim();
    let orig_state: Pots = state_string
        .chars()
        .enumerate()
        .map(|(index, letter)| (index as i32, char_is_plant(letter)))
        .collect();
    //println!("{:?}", num_pots);
    //println!("{:?}", state);

    // Empty line
    let _line = lines.next().unwrap();

    // Rules
    let mut rules: Rules = HashMap::new();
    for line in lines {
        let regex = Regex::new(r"(.*) => (.)").unwrap();
        let captures = regex.captures(line).unwrap();
        let state: Vec<bool> = captures[1].chars().map(char_is_plant).collect();
        let state: Rule = (state[0], state[1], state[2], state[3], state[4]);
        let result = char_is_plant(captures[2].chars().nth(0).unwrap());
        rules.insert(state, result);
    }

    // Part 1
    // Iterate
    let mut state = orig_state.clone();
    for _ in 0..20 {
        let new_state = step(state, &rules);
        state = new_state;
    }

    let pot_sum = state
        .iter()
        .fold(0, |acc, (index, pot)| if *pot { acc + index } else { acc });
    println!("{}", pot_sum);

    // Part 2
    let mut state = orig_state.clone();

    let mut iter_count = 0;
    let mut history: Vec<i32> = Vec::new();
    loop {
        // Step
        let new_state: Pots = step(state, &rules);
        iter_count += 1;

        // Sum
        let pot_sum = new_state
            .iter()
            .fold(0, |acc, (index, pot)| if *pot { acc + index } else { acc });
        // Check if we've reached a stable diff
        if history.len() > 5 {
            let last_sum1 = history.iter().nth(history.len() - 1).unwrap();
            let last_sum2 = history.iter().nth(history.len() - 2).unwrap();
            let pot_sum_diff1 = pot_sum - last_sum1;
            let pot_sum_diff2 = last_sum1 - last_sum2;
            if pot_sum_diff1 == pot_sum_diff2 {
                let final_sum: i64 =
                    pot_sum as i64 + (50000000000 - iter_count as i64) * pot_sum_diff1 as i64;
                println!("{}", final_sum);
                break;
            }
        }

        history.push(pot_sum);
        state = new_state;
    }
}

fn char_is_plant(letter: char) -> bool {
    letter == '#'
}

fn next_state(pot_index: i32, pots: &Pots, rules: &Rules) -> bool {
    let left2 = pot_index - 2;
    let left1 = pot_index - 1;
    let right1 = pot_index + 1;
    let right2 = pot_index + 2;

    let rule: Rule = (
        safe_has_plant(left2, pots),
        safe_has_plant(left1, pots),
        safe_has_plant(pot_index, pots),
        safe_has_plant(right1, pots),
        safe_has_plant(right2, pots),
    );

    safe_rule_eval(&rule, rules)
}

fn safe_rule_eval(rule: &Rule, rules: &Rules) -> bool {
    match rules.get(&rule) {
        Some(result) => *result,
        None => false,
    }
}

fn safe_has_plant(pot_index: i32, pots: &Pots) -> bool {
    match pots.get(&pot_index) {
        Some(result) => *result,
        None => false,
    }
}

fn step(state: Pots, rules: &Rules) -> Pots {
    // Current pots
    let mut new_state: Pots = state
        .iter()
        .map(|(index, _pot)| (*index, next_state(*index, &state, &rules)))
        .collect();
    // Edgy pots
    let leftmost = state.keys().min().unwrap();
    let rightmost = state.keys().max().unwrap();
    // At most, we need to consider a pot that is 2 away from the furthest current pot
    let left2 = leftmost - 2;
    let left1 = leftmost - 1;
    let right1 = rightmost + 1;
    let right2 = rightmost + 2;

    if next_state(left2, &state, &rules) {
        new_state.insert(left2, true);
    }
    if next_state(left1, &state, &rules) {
        new_state.insert(left1, true);
    }
    if next_state(right1, &state, &rules) {
        new_state.insert(right1, true);
    }
    if next_state(right2, &state, &rules) {
        new_state.insert(right2, true);
    }

    new_state
}
