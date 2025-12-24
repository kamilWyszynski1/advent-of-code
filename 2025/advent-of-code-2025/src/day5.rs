fn day5a(input: &str) -> usize {
    let (ranges, ings) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let ings: Vec<usize> = ings.lines().map(|v| v.parse().unwrap()).collect();

    let mut result = 0;

    for i in ings.iter() {
        for (left, right) in &ranges {
            if left <= i && i <= right {
                result += 1;
                break;
            }
        }
    }

    result
}

fn day5b(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    'outer: loop {
        for i in (1..ranges.len()).rev() {
            let left = ranges[i - 1];
            let right = ranges[i];

            let mut new = (0, 0);

            if left.0 <= right.0 && left.1 <= right.1 && left.1 >= right.0 {
                new = (left.0, right.1.max(left.1));
            } else if left.0 <= right.1 && left.1 >= right.1 {
                new = (left.0, left.1);
            }

            if new != (0, 0) {
                ranges.remove(i - 1);
                ranges.remove(i - 1);
                ranges.insert(i - 1, new);
                continue 'outer;
            }
        }
        break;
    }

    let mut result = 0;
    for (left, right) in ranges {
        result += right - left + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        day5::{day5a, day5b},
        read_input,
    };

    #[test]
    fn test_day5a() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

        assert_eq!(3, day5a(input));

        println!("day5 a result {}", day5a(&read_input("day5")))
    }

    #[test]
    fn test_day5b() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

        assert_eq!(14, day5b(input));

        println!("day5 b result {}", day5b(&read_input("day5")))
    }
}
