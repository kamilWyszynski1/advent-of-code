use crate::read_input;

fn day2a(input: &str) -> isize {
    let data: Vec<(isize, isize)> = input
        .split(',')
        .map(|l| {
            let (left, right) = l.split_once('-').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    dbg!(&data);
    let mut result = 0;
    for (left, right) in data {
        for i in left..=right {
            // println!("{i}, :{}", is_invalid(i));
            if is_invalid(i) {
                result += i
            }
        }
    }
    result
}

fn is_invalid(i: isize) -> bool {
    let s = i.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let (l, r) = s.split_at(s.len() / 2);
    // dbg!(l, r);
    l == r
}

fn day2b(input: &str) -> isize {
    let data: Vec<(isize, isize)> = input
        .split(',')
        .map(|l| {
            let (left, right) = l.split_once('-').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    dbg!(&data);
    let mut result = 0;
    for (left, right) in data {
        for i in left..=right {
            // println!("{i}, :{}", is_invalid(i));
            if is_invalid_b(i) {
                result += i
            }
        }
    }
    result
}

fn is_invalid_b(i: isize) -> bool {
    let s = i.to_string();
    let mut chars = s.chars();

    let mut curr = chars.next().unwrap().to_string();
    loop {
        if curr.len() > s.len() / 2 {
            return false;
        }
        if s.len() % curr.len() == 0 {
            if curr.repeat(s.len() / curr.len()) == s {
                return true;
            }
        }
        curr.push(chars.next().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        day2::{day2a, day2b},
        read_input,
    };

    #[test]
    fn test_day2a() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        assert_eq!(1227775554, day2a(input));

        println!("day2 a result {}", day2a(&read_input("day2")))
    }

    #[test]
    fn test_day2b() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        assert_eq!(4174379265, day2b(input));

        println!("day2 b result {}", day2b(&read_input("day2")))
    }
}
