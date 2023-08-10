// #[derive(Debug)]
// struct Point {
//     x: usize,
//     y: usize,
// }
//
// fn walk(
//     maze: Vec<&str>,
//     wall: &str,
//     curr: Point,
//     end: Point,
//     seen: &mut Vec<Vec<bool>>,
//     path: &mut Vec<Point>,
// ) -> bool {
//     if maze[curr.x][curr.y] == wall {
//         return false;
//     }
//
//     return false;
// }
//
// pub fn solve(maze: Vec<&str>, wall: &str, start: Point, end: Point) -> Vec<Point> {
//     let mut seen: Vec<Vec<bool>> = vec![];
//     let mut path: Vec<Point> = vec![];
//
//     for _ in 0..maze.len() {
//         let mut vec = Vec::with_capacity(maze[0].len());
//         vec.fill(false);
//         seen.push(vec);
//     }
//
//     walk(maze, wall, start, end, &mut seen, &mut path);
//     println!("path is: {:?}", path);
//
//     return path;
// }

// solve(vec!["### #", "#   #", "# ###"], "#", Point { x: 3, y: 0 }, Point { x: 2, y: 1 });
