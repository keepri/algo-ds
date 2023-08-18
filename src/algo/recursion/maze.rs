use std::{cmp::Ordering, time::Instant};

use crate::utils;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

// up, right, down, left
const DIRS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn walk(
    maze: &Vec<&str>,
    wall: char,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    let maze_row = maze[curr.x];
    if let Some(char) = maze_row.chars().nth(curr.y).take() {
        if char.cmp(&wall) == Ordering::Equal {
            return false;
        }
    }

    if curr.x == end.x && curr.y == end.y {
        path.push(curr);
        return true;
    }

    if seen[curr.x][curr.y] == true {
        return false;
    }

    path.push(curr);
    seen[curr.x][curr.y] = true;

    for dir_pair in DIRS {
        let x = curr.x as isize + dir_pair[0];
        let y = curr.y as isize + dir_pair[1];

        if x < 0 || x >= maze.len() as isize || y < 0 || y >= maze[0].len() as isize {
            continue;
        }

        if walk(
            maze,
            wall,
            Point {
                x: x as usize,
                y: y as usize,
            },
            end,
            seen,
            path,
        ) {
            return true;
        }
    }

    path.pop();

    return false;
}

pub fn solve(logger: bool) {
    let maze = vec![
        "####S#####",
        "#  #     #",
        "## # ### #",
        "#  #  #  #",
        "# #####  #",
        "#        #",
        "#E########",
    ];
    let mut path: Vec<Point> = vec![];

    let mut seen = Vec::<Vec<bool>>::with_capacity(maze.len());
    for _ in 0..maze.len() {
        let vec = vec![false; maze[0].len()];
        seen.push(vec);
    }

    let start_time = Instant::now();
    walk(
        &maze,
        '#',
        Point { x: 0, y: 4 },
        Point { x: 6, y: 1 },
        &mut seen,
        &mut path,
    );
    let duration = Instant::now().duration_since(start_time);

    let mut solution: Vec<String> = maze.iter().map(|row| row.to_string()).collect();
    for i in 0..seen.len() {
        let seen_row = &seen[i];
        for j in 0..seen_row.len() {
            if seen_row[j] == true {
                let mut solution_row: Vec<char> = solution[i].chars().collect();
                solution_row[j] = 'i';
                solution[i] = solution_row.iter().collect();
            }
        }
    }

    for point in path {
        let row = &solution[point.x];
        let mut chars: Vec<char> = row.chars().collect();
        chars[point.y] = '•';
        solution[point.x] = chars.iter().collect();
    }

    if logger == true {
        println!(
            "Maze solution finished in {}s",
            utils::parse_duration(duration)
        );
        println!();
        for row in &solution {
            println!("{row}");
        }
    }
}
