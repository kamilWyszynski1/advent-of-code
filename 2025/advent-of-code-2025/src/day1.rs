use crate::read_input;

fn day1a(input: &str) -> isize {
    let data: Vec<isize> = input
        .lines()
        .map(|l| l.replace(" ", ""))
        .map(|l| {
            if let Some(rest) = l.strip_prefix('R') {
                rest.parse::<isize>().unwrap()
            } else if let Some(rest) = l.strip_prefix('L') {
                -(rest.parse::<isize>().unwrap())
            } else {
                println!("{}", l);
                panic!("invalid data")
            }
        })
        .collect();

    let mut point = 50;
    let mut sum = 0;
    for rotation in data {
        point = (point + rotation).rem_euclid(100);
        if point == 0 {
            sum += 1
        }
    }

    sum
}

fn day1b(input: &str) -> isize {
    let data: Vec<isize> = input
        .lines()
        .map(|l| l.replace(" ", ""))
        .map(|l| {
            if let Some(rest) = l.strip_prefix('R') {
                rest.parse::<isize>().unwrap()
            } else if let Some(rest) = l.strip_prefix('L') {
                -(rest.parse::<isize>().unwrap())
            } else {
                println!("{}", l);
                panic!("invalid data")
            }
        })
        .collect();

    let mut point = 50;
    let mut sum = 0;
    for rotation in data {
        if point != 0 {
            if rotation > 0 {
                sum += (point + rotation) / 100;
            } else if point + rotation <= 0 {
                sum += ((point + rotation) / -100) + 1;
            }
        } else {
            sum += (rotation.abs()) / 100;
        }
        point = (point + rotation).rem_euclid(100);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::day1a;
    use crate::{day1::day1b, read_input};

    #[test]
    fn test_day1a() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        assert_eq!(3, day1a(input));

        println!("day1 a result {}", day1a(&read_input("day1")))
    }

    #[test]
    fn test_day1b() {
        let input = r#"L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"#;

        assert_eq!(6, day1b(input));

        println!("day1 b result {}", day1b(&read_input("day1")))
    }
}
