use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let grid_serial: i64 = INPUT.parse().unwrap();
    let cells: HashMap<(i64, i64), i64> =
    // Populate the fuel cells
    (1..301i64).into_par_iter().flat_map(|x| {
        (1..301i64).into_par_iter().map(move |y| {
            let cell_rack_id = x + 10;
            let mut power_level = cell_rack_id * y;
            power_level += grid_serial;
            power_level *= cell_rack_id;
            if power_level < 100 {
                power_level = 0;
            } else {
                power_level = (power_level / 100) % 10;
            }
            power_level -= 5;

            ((x, y), power_level)
        })
    }).collect();

    // Explicit borrow. Srs bsns. How does this work? Why?
    let ref_cells = &cells;

    // Sum all 3x3's
    let sum_cells: HashMap<(i64, i64), i64> =
    // main grid x
    (1i64..299i64).into_par_iter().flat_map(|grid_x| {
        // main grid y
        (1i64..299i64).into_par_iter().map(move |grid_y| {
            // hashmap's struct tuple
            (
                // coordinates tuple for the 3x3 square's top-left
                (grid_x , grid_y),
                // offsets for calculating sum of 3x3
                (0i64..3i64).into_par_iter().flat_map(|offset_x| {
                    (0i64..3i64).into_par_iter().map(move |offset_y| {
                        // get the value at this coord+offset
                        ref_cells.get(&(grid_x + offset_x, grid_y + offset_y )).unwrap()
                    })
                })
                .sum::<i64>()
            )
        })
    })
    .collect();

    // Part 1
    // Find best
    let ((x, y), sum) = sum_cells.iter().fold(((1i64, 1i64), 0i64), |acc, entry| {
        if entry.1 > &acc.1 {
            return (*entry.0, *entry.1);
        } else {
            return acc;
        }
    });
    println!("{}, {}, {}", x, y, sum);
}
