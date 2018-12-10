use hashbrown::HashMap;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Star {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Star {
    pub fn new(x: i32, y: i32, velocity_x: i32, velocity_y: i32) -> Star {
        Star {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }

    pub fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid_width = 200; // Square
    let run_steps = 20_000;
    let path = env::current_dir().expect("current_dir not found");
    let mut stars: Vec<Star> = Vec::new();
    let regex = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>").unwrap();
    for line in INPUT.lines() {
        let captures = regex.captures(line).unwrap();
        let x: i32 = captures[1].trim().parse().unwrap();

        let y: i32 = captures[2].trim().parse().unwrap();
        let velocity_x: i32 = captures[3].trim().parse().unwrap();
        let velocity_y: i32 = captures[4].trim().parse().unwrap();
        /*
        println!(
            "x: {} y: {} vel_x: {} vel_y: {}",
            x, y, velocity_x, velocity_y
        ); */
        let star = Star::new(x, y, velocity_x, velocity_y);
        stars.push(star);
    }

    // Grid
    let mut grid: HashMap<(i32, i32), bool> =
        HashMap::with_capacity((grid_width * grid_width) as usize);

    for iteration in 0..run_steps {
        if check_closeness(&stars, grid_width) {
            // Clear
            for x in 0..grid_width {
                for y in 0..grid_width {
                    *grid.entry((x, y)).or_insert(false) = false;
                }
            }
            // Insert stars
            for star in stars.iter() {
                *grid.get_mut(&(star.x, star.y)).unwrap() = true;
            }

            // Write file
            let mut data = String::new();
            for x in 0..grid_width {
                for y in 0..grid_width {
                    let has_star = *grid.get(&(x, y)).unwrap();
                    if has_star {
                        data.push('#')
                    } else {
                        data.push(' ')
                    }
                }
                data.push('\n');
            }

            let mut filepath = path.clone();
            filepath.push(format!("{}.txt", iteration));
            let mut file = File::create(filepath).expect("Error creating a file");
            write!(file, "{}", data).expect("Error writing data to the file");
        }
        for star in &mut stars {
            star.update();
        }
    }
}

fn check_closeness(stars: &Vec<Star>, threshold: i32) -> bool {
    stars
        .iter()
        .all(|&star| star.x.abs() < threshold && star.y.abs() < threshold)
}
