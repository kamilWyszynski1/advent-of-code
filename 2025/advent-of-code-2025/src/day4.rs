use crate::read_input;

fn day4a(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut result = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '@' && check(&grid, i, j) < 4 {
                result += 1
            }
        }
    }

    result
}

fn check(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut adj = 0;

    let checks: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (di, dj) in checks {
        if grid
            .get((i as isize - di) as usize)
            .unwrap_or(&vec![])
            .get((j as isize - dj) as usize)
            .unwrap_or(&'.')
            == &'@'
        {
            adj += 1
        }
    }
    adj
}

fn day4b(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut result = 0;

    'outer: loop {
        for (i, line) in grid.iter().enumerate() {
            for (j, ch) in line.iter().enumerate() {
                if *ch == '@' && check(&grid, i, j) < 4 {
                    *grid.get_mut(i).unwrap().get_mut(j).unwrap() = '*';
                    result += 1;
                    continue 'outer;
                }
            }
        }

        break;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        day4::{day4a, day4b},
        read_input,
    };

    #[test]
    fn test_day4a() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

        assert_eq!(13, day4a(input));

        println!("day4 a result {}", day4a(&read_input("day4")))
    }

    #[test]
    fn test_day4b() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

        assert_eq!(43, day4b(input));

        println!("day4 b result {}", day4b(&read_input("day4")))
    }
}
