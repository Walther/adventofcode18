use hashbrown::HashMap;
use regex::Regex;

// FIXME: this currently takes ~45min to run for part 2 on a 6-core macbook
// Needs optimization / a better algorithm :D

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let regex = Regex::new(r"(\d*) players; last marble is worth (\d*) points").unwrap();
    let num_players: i32 = regex.captures(INPUT).unwrap()[1].parse().unwrap();
    let max_marble: i32 = regex.captures(INPUT).unwrap()[2].parse().unwrap();
    //println!("{}", a);
    //println!("{}", b);
    let part1 = solve_marbles(num_players, max_marble);
    println!("{}", part1);
    let part2 = solve_marbles(num_players, max_marble * 100);
    println!("{}", part2);
}

fn solve_marbles(num_players: i32, max_marble: i32) -> i64 {
    // Manually insert first marble; no need for zero-checks in loop
    let mut circle: Vec<i32> = Vec::with_capacity(max_marble as usize + 1);
    circle.push(0);
    let mut current_player = 1;
    let mut current_marble_pos = 1;
    let marbles = 1..=max_marble;
    // Preallocate players; no need for or_insert in loop
    let mut players: HashMap<i32, i64> = HashMap::with_capacity(num_players as usize); // player id, points
    for player in 0..num_players {
        players.insert(player, 0);
    }

    for marble in marbles {
        // Marble counter for speed estimation
        /* if marble % 10_000 == 0 {
            println!("playing marble {}", marble)
        } */
        if marble % 23 == 0 {
            *players.get_mut(&current_player).unwrap() += marble as i64;
            current_marble_pos = (current_marble_pos + circle.len() - 7) % circle.len();
            let removed = circle.remove(current_marble_pos);
            *players.get_mut(&current_player).unwrap() += removed as i64;
        } else {
            current_marble_pos = (current_marble_pos + 2) % circle.len();
            circle.insert(current_marble_pos, marble);
        }
        current_player = (current_player + 1) % num_players;
    }
    let winning_score: i64 = *players.values().max().unwrap();
    winning_score
}
