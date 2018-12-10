use hashbrown::HashMap;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Star {
    x: i64,
    y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

impl Star {
    pub fn new(x: i64, y: i64, velocity_x: i64, velocity_y: i64) -> Star {
        Star {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }

    pub fn step_forward(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn step_backward(&mut self) {
        self.x -= self.velocity_x;
        self.y -= self.velocity_y;
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let run_steps = 20_000;
    let path = env::current_dir().expect("current_dir not found");
    let mut stars: Vec<Star> = Vec::new();
    let regex = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>").unwrap();
    for line in INPUT.lines() {
        let captures = regex.captures(line).unwrap();
        let x: i64 = captures[1].trim().parse().unwrap();

        let y: i64 = captures[2].trim().parse().unwrap();
        let velocity_x: i64 = captures[3].trim().parse().unwrap();
        let velocity_y: i64 = captures[4].trim().parse().unwrap();
        /*
        println!(
            "x: {} y: {} vel_x: {} vel_y: {}",
            x, y, velocity_x, velocity_y
        ); */
        let star = Star::new(x, y, velocity_x, velocity_y);
        stars.push(star);
    }

    let mut minimum_bounding_box = get_bounding_box(&stars);

    for iteration in 0..run_steps {
        for star in &mut stars {
            star.step_forward();
        }
        let bounding_box = get_bounding_box(&stars);
        if get_bounding_box_area(bounding_box) < get_bounding_box_area(minimum_bounding_box) {
            minimum_bounding_box = bounding_box;
        } else {
            // We're done, step back and print
            for star in &mut stars {
                star.step_backward();
            }
            let bounding_box = get_bounding_box(&stars);

            // Grid
            let mut grid: HashMap<(i64, i64), bool> = HashMap::new();

            // Clear
            for x in bounding_box.0..=bounding_box.1 {
                for y in bounding_box.2..=bounding_box.3 {
                    *grid.entry((x, y)).or_insert(false) = false;
                }
            }

            // Insert stars
            for star in stars.iter() {
                *grid.entry((star.x, star.y)).or_insert(true) = true;
            }

            // Write file
            let mut data = String::new();
            // Printing in computer graphics order instead of math order
            for y in bounding_box.2..=bounding_box.3 {
                for x in bounding_box.0..=bounding_box.1 {
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

            // Remember to exit. Whoops.
            break;
        }
    }
}

fn get_bounding_box(stars: &Vec<Star>) -> (i64, i64, i64, i64) {
    let left = stars.iter().map(|star| star.x).min().unwrap();
    let right = stars.iter().map(|star| star.x).max().unwrap();
    let top = stars.iter().map(|star| star.y).min().unwrap();
    let bottom = stars.iter().map(|star| star.y).max().unwrap();

    (left, right, top, bottom)
}

fn get_bounding_box_area((left, right, top, bottom): (i64, i64, i64, i64)) -> i64 {
    (right - left) * (bottom - top)
}
