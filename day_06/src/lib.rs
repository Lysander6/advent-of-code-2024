use std::{collections::HashSet, str::FromStr};

//   x
//   |
// y-+---->
//   |
//   v

#[derive(Debug)]
pub struct Problem {
    map_height: usize,
    map_width: usize,
    obstacles: HashSet<(usize, usize)>,
    starting_position: (usize, usize),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn next_direction(d: &Direction) -> Direction {
    use Direction::{Down, Left, Right, Up};

    match d {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn get_movement_delta(d: &Direction) -> (isize, isize) {
    use Direction::{Down, Left, Right, Up};

    match d {
        Up => (-1, 0),
        Right => (0, 1),
        Down => (1, 0),
        Left => (0, -1),
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_position = (0, 0);

        let map_height = s.lines().count();
        let map_width = s.lines().next().unwrap().chars().count();

        let obstacles = s
            .lines()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(y, c)| match c {
                        '^' => {
                            starting_position = (x, y);
                            None
                        }
                        '.' => None,
                        '#' => Some((x, y)),
                        _ => unreachable!("Unknown map symbol"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Ok(Problem {
            map_height,
            map_width,
            obstacles,
            starting_position,
        })
    }
}

fn walk_maze(
    starting_position: &(usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    map_height: usize,
    map_width: usize,
) -> usize {
    let mut visited_spaces = HashSet::from([*starting_position]);
    let mut current_position = *starting_position;
    let mut movement_direction = Direction::Up;

    loop {
        let movement_delta = get_movement_delta(&movement_direction);

        // TODO: replace if-let-else below with following let-else:
        // let (Some(next_x), Some(next_y)) = (
        //     current_position.0.checked_add_signed(movement_delta.0),
        //     current_position.1.checked_add_signed(movement_delta.1),
        // ) else {
        //     // out of map
        //     break;
        // };

        if let (Some(next_x), Some(next_y)) = (
            current_position.0.checked_add_signed(movement_delta.0),
            current_position.1.checked_add_signed(movement_delta.1),
        ) {
            if next_x >= map_height || next_y >= map_width {
                // out of map
                break;
            }

            if obstacles.contains(&(next_x, next_y)) {
                // occupied space, rotate
                movement_direction = next_direction(&movement_direction);
                continue;
            }

            current_position = (next_x, next_y);
            visited_spaces.insert(current_position);
        } else {
            // out of map
            break;
        }
    }

    visited_spaces.len()
}

fn walk_maze_and_check_for_loop(
    starting_position: &(usize, usize),
    starting_direction: Direction,
    obstacles: &HashSet<(usize, usize)>,
    map_height: usize,
    map_width: usize,
) -> bool {
    let mut collided_obstacles: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut current_position = *starting_position;
    let mut movement_direction = starting_direction;

    loop {
        let movement_delta = get_movement_delta(&movement_direction);

        if let (Some(next_x), Some(next_y)) = (
            current_position.0.checked_add_signed(movement_delta.0),
            current_position.1.checked_add_signed(movement_delta.1),
        ) {
            if next_x >= map_height || next_y >= map_width {
                // out of map
                break;
            }

            if obstacles.contains(&(next_x, next_y)) {
                // check if it is repeated collision which indicates a loop
                if collided_obstacles.contains(&(next_x, next_y, movement_direction.clone())) {
                    return true;
                }

                // first store the collision
                collided_obstacles.insert((next_x, next_y, movement_direction.clone()));

                // then rotate
                movement_direction = next_direction(&movement_direction);
                continue;
            }

            current_position = (next_x, next_y);
        } else {
            // out of map
            break;
        }
    }

    false
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem {
        map_height,
        map_width,
        obstacles,
        starting_position,
    } = p;

    walk_maze(starting_position, obstacles, *map_height, *map_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(p.map_height, 10);
        assert_eq!(p.map_width, 10);
        assert_eq!(p.obstacles.contains(&(0, 0)), false);
        assert_eq!(p.obstacles.contains(&(0, 4)), true);
        assert_eq!(p.obstacles.contains(&(6, 4)), false);
        assert_eq!(p.starting_position, (6, 4));
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 41);
    }

    #[test]
    fn test_walk_maze_and_check_for_loop() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &p.obstacles,
                p.map_height,
                p.map_width
            ),
            false
        );

        let mut obstacles_with_loop_1 = p.obstacles.clone();
        obstacles_with_loop_1.insert((6, 3));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_1,
                p.map_height,
                p.map_width
            ),
            true
        );

        let mut obstacles_with_loop_2 = p.obstacles.clone();
        obstacles_with_loop_2.insert((7, 6));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_2,
                p.map_height,
                p.map_width
            ),
            true
        );

        let mut obstacles_with_loop_3 = p.obstacles.clone();
        obstacles_with_loop_3.insert((7, 7));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_3,
                p.map_height,
                p.map_width
            ),
            true
        );

        let mut obstacles_with_loop_4 = p.obstacles.clone();
        obstacles_with_loop_4.insert((8, 1));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_4,
                p.map_height,
                p.map_width
            ),
            true
        );

        let mut obstacles_with_loop_5 = p.obstacles.clone();
        obstacles_with_loop_5.insert((8, 3));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_5,
                p.map_height,
                p.map_width
            ),
            true
        );


        let mut obstacles_with_loop_6 = p.obstacles.clone();
        obstacles_with_loop_6.insert((9, 7));

        assert_eq!(
            walk_maze_and_check_for_loop(
                &p.starting_position,
                Direction::Up,
                &obstacles_with_loop_6,
                p.map_height,
                p.map_width
            ),
            true
        );
    }
}
