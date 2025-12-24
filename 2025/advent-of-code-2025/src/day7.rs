use std::collections::{HashMap, HashSet};

fn day7a(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let starting_col = grid
        .first()
        .unwrap()
        .iter()
        .position(|&x| x == 'S')
        .unwrap();

    let mut res = 0;
    let mut cache = HashSet::new();
    traverse(&mut grid, (0, starting_col), &mut res, &mut cache);

    res
}

fn traverse(
    grid: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    res: &mut usize,
    cache: &mut HashSet<(usize, usize)>,
) {
    if pos.0 >= grid.len() {
        return;
    }
    if pos.1 >= grid[0].len() {
        return;
    }

    if grid[pos.0][pos.1] == '^' {
        // perform split
        if cache.contains(&pos) {
            return;
        }

        *res += 1;
        cache.insert(pos);

        traverse(grid, (pos.0, pos.1 - 1), res, cache);
        traverse(grid, (pos.0, pos.1 + 1), res, cache);
        return;
    }
    traverse(grid, (pos.0 + 1, pos.1), res, cache);
}

fn traverse_b(
    grid: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if pos.0 >= grid.len() {
        return 1;
    }
    if pos.1 >= grid[0].len() {
        return 0;
    }

    if grid[pos.0][pos.1] == '^' {
        if let Some(cached) = cache.get(&pos) {
            return *cached;
        }

        let posibilities = traverse_b(grid, (pos.0, pos.1 - 1), cache)
            + traverse_b(grid, (pos.0, pos.1 + 1), cache);
        cache.insert(pos, posibilities);

        return posibilities;
    }
    traverse_b(grid, (pos.0 + 1, pos.1), cache)
}

fn day7b(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let starting_col = grid
        .first()
        .unwrap()
        .iter()
        .position(|&x| x == 'S')
        .unwrap();

    traverse_b(&mut grid, (0, starting_col), &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use crate::{
        day7::{day7a, day7b},
        read_input,
    };

    #[test]
    fn test_day7a() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        assert_eq!(21, day7a(input));

        println!("day7 a result {}", day7a(&read_input("day7")))
    }

    #[test]
    fn test_day7b() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        assert_eq!(40, day7b(input));

        println!("day7 b result {}", day7b(&read_input("day7")))
    }
}
