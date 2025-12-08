use crate::read_input;

fn day3a(input: &str) -> usize {
    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    data.iter().map(|v| find_largest(v)).sum()
}

fn find_largest(v: &[usize]) -> usize {
    dbg!(v);
    let mut tens = v[0];
    let mut inx = 0;

    let mut second_tens = usize::MIN;
    let mut second_inx = 0;

    for (i, val) in v.iter().enumerate() {
        if *val > tens {
            second_tens = tens;
            tens = *val;

            second_inx = inx;
            inx = i;
        }
    }

    if inx == v.len() - 1 {
        println!("backup: {} {}", second_tens, second_inx);
        // need backup
        return 10 * second_tens + find_max(v, second_inx + 1);
    }

    println!("res: {} at {}, {}", tens, inx, find_max(v, inx + 1));
    10 * tens + find_max(v, inx + 1)
}

fn find_max(v: &[usize], inx: usize) -> usize {
    let mut max = usize::MIN;
    for i in inx..v.len() {
        max = max.max(v[i]);
    }
    max
}

fn day3b(input: &str) -> usize {
    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    data.iter()
        .map(|v| find_largest_b(v.to_vec(), Vec::new()))
        .sum()
}

fn find_largest_b(v: Vec<usize>, mut curr: Vec<usize>) -> usize {
    if curr.len() == 12 {
        let mut sum: usize = 0;
        for (i, val) in (0..12).rev().enumerate() {
            sum += 10_usize.pow(val.try_into().unwrap()) * curr[i];
        }
        return sum;
    }
    for i in [9, 8, 7, 6, 5, 4, 3, 2, 1] {
        for (inx, val) in v.iter().enumerate() {
            if *val == i && inx <= v.len() - (12 - curr.len()) {
                curr.push(*val);

                let c = v.clone();

                return find_largest_b((c[inx + 1..]).to_vec(), curr);
            }
        }
    }
    panic!("should not reach")
}

#[cfg(test)]
mod tests {
    use crate::{
        day3::{day3a, day3b},
        read_input,
    };

    #[test]
    fn test_day3a() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        assert_eq!(357, day3a(input));

        println!("day3 a result {}", day3a(&read_input("day3")))
    }

    #[test]
    fn test_day3b() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

        assert_eq!(3121910778619, day3b(input));

        println!("day3 b result {}", day3b(&read_input("day3")))
    }
}
