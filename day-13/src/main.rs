use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    turning: Turning,
    removed: bool, // FIXME: ugly state handling just to make the index-access easier in the loop
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, PartialEq)]
enum Turning {
    Left,
    Straight,
    Right,
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Let's just store the entire map as a hashmap, raw chars. Bleh.
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut carts: Vec<Cart> = Vec::new();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // Store carts in separate struct,
            // insert tracks in place of the initial carts
            if c == '>' {
                carts.push(Cart {
                    x,
                    y,
                    direction: Direction::Right,
                    turning: Turning::Left,
                    removed: false,
                });
                grid.insert((x, y), '-');
            } else if c == 'v' {
                carts.push(Cart {
                    x,
                    y,
                    direction: Direction::Down,
                    turning: Turning::Left,
                    removed: false,
                });
                grid.insert((x, y), '|');
            } else if c == '<' {
                carts.push(Cart {
                    x,
                    y,
                    direction: Direction::Left,
                    turning: Turning::Left,
                    removed: false,
                });
                grid.insert((x, y), '-');
            } else if c == '^' {
                carts.push(Cart {
                    x,
                    y,
                    direction: Direction::Up,
                    turning: Turning::Left,
                    removed: false,
                });
                grid.insert((x, y), '|');
            } else {
                grid.insert((x, y), c);
            }
        }
    }

    // Update loop
    let mut first_collision = true;
    let mut last_collision = false;
    while !last_collision {
        carts.sort_by_key(|c| (c.y, c.x));
        for cart_index in 0..carts.len() {
            // Tick
            //println!("Tick");
            // debug print the coords
            /*
            println!(
                "{:?}",
                carts
                    .iter()
                    .map(|c| (c.x, c.y))
                    .collect::<Vec<(usize, usize)>>()
            );
            */

            // An index a day keeps the borrow checker away
            let mut cart = carts[cart_index];
            if cart.removed {
                continue;
            }

            // Evaluate whether we're in a turning point
            match grid.get(&(cart.x, cart.y)).unwrap() {
                '/' => match cart.direction {
                    Direction::Up => cart.direction = Direction::Right,
                    Direction::Right => cart.direction = Direction::Up,
                    Direction::Down => cart.direction = Direction::Left,
                    Direction::Left => cart.direction = Direction::Down,
                },
                '\\' => match cart.direction {
                    Direction::Up => cart.direction = Direction::Left,
                    Direction::Right => cart.direction = Direction::Down,
                    Direction::Down => cart.direction = Direction::Right,
                    Direction::Left => cart.direction = Direction::Up,
                },
                '+' => match cart.turning {
                    // In a crossing, update our direction & next turning
                    Turning::Left => {
                        match cart.direction {
                            Direction::Up => cart.direction = Direction::Left,
                            Direction::Right => cart.direction = Direction::Up,
                            Direction::Down => cart.direction = Direction::Right,
                            Direction::Left => cart.direction = Direction::Down,
                        };
                        cart.turning = Turning::Straight;
                    }
                    Turning::Straight => {
                        cart.turning = Turning::Right;
                    }
                    Turning::Right => {
                        match cart.direction {
                            Direction::Up => cart.direction = Direction::Right,
                            Direction::Right => cart.direction = Direction::Down,
                            Direction::Down => cart.direction = Direction::Left,
                            Direction::Left => cart.direction = Direction::Up,
                        }
                        cart.turning = Turning::Left;
                    }
                },
                ' ' => println!("Debug: shouldn't get here!",),
                _ => (),
            };

            // Move
            match cart.direction {
                Direction::Right => {
                    cart.x += 1;
                }
                Direction::Down => {
                    cart.y += 1;
                }
                Direction::Left => {
                    cart.x -= 1;
                }
                Direction::Up => {
                    cart.y -= 1;
                }
            };

            // Collision detection
            let collided = carts
                .iter()
                .position(|c| !c.removed && c.x == cart.x && c.y == cart.y);

            match collided {
                Some(collided_index) => {
                    if first_collision {
                        // part1
                        println!("{},{}", cart.x, cart.y);
                        first_collision = false;
                    }

                    // remove both carts
                    cart.removed = true;
                    carts[collided_index].removed = true;
                }
                None => (),
            };

            // Put the edited cart back
            carts[cart_index] = cart;
        }
        // part2
        if carts
            .iter()
            .filter(|c| !c.removed)
            .collect::<Vec<&Cart>>()
            .len()
            < 2
        {
            let last_cart = carts.iter().find(|c| !c.removed).unwrap();
            println!("{},{}", last_cart.x, last_cart.y);
            last_collision = true;
        };
    }
}
