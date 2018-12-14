fn main() {
    const INPUT: &str = include_str!("input.txt");
    let mut recipes: Vec<usize> = vec![3, 7];
    let input: usize = INPUT.parse().unwrap();
    let total_recipes = input + 10;
    let num_digits = INPUT.len();
    let mut elf1: usize = 0;
    let mut elf2: usize = 1;
    let mut part1 = false;
    let mut part2 = false;
    let mut recipe_string: String;
    while !part1 || !part2 {
        // sum
        let sum: usize = recipes[elf1] + recipes[elf2];
        // insert new recipes
        let new_recipes: Vec<usize> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect();
        recipes.extend(new_recipes);
        // move
        let move_amount_elf1: usize = 1 + recipes[elf1];
        elf1 = (elf1 + move_amount_elf1) % (recipes.len());
        let move_amount_elf2 = 1 + recipes[elf2];
        elf2 = (elf2 + move_amount_elf2) % (recipes.len());

        // checks
        if !part1 && recipes.len() >= total_recipes {
            let next_10_recipes = &recipes[input..input + 10];
            let next_10_recipes: String = next_10_recipes.iter().map(|c| c.to_string()).collect();
            println!("Part 1: {:?}", next_10_recipes);
            part1 = true;
        }
        if !part2 && recipes.len() > num_digits {
            // Check last n+1 recipes: the sequence will be found at the end, but we might have added two digits
            let tail_position = recipes.len() - num_digits - 1;
            recipe_string = recipes
                .get(tail_position..)
                .unwrap()
                .iter()
                .map(|c| c.to_string())
                .collect();
            if recipe_string.contains(&INPUT) {
                let position = recipes
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<String>()
                    .find(INPUT)
                    .unwrap();
                println!("Part 2: {:?}", position);
                part2 = true;
            }
        }
    }
}
