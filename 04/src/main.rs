use std::str::FromStr;

fn digits(x: i32) -> [i32; 6] {
    let mut digs: [i32; 6] = [0; 6];

    if x <= 0 || x > 999999 {
        panic!("malformed digit: {}", x);
    }

    let mut x_div = x;
    for i in digs.iter_mut().rev() {
        let digit = x_div % 10;
        *i = digit;
        x_div = x_div / 10;
    }
    if x_div != 0 {
        panic!("should be zero now: {}", x_div);
    }
    digs
}

fn monotonic_digits(digits: &[i32; 6]) -> bool {
    !digits
        .iter()
        .zip(digits.iter().skip(1))
        .map(|(f, s)| f > s)
        .any(|x| x)
}

fn adjacent_digits_match_sol1(digits: &[i32; 6]) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .map(|(i, j)| i == j)
        .any(|x| x)
}
fn adjacent_digits_match_sol2(digits: &[i32; 6]) -> bool {

    // generate run counts in the digits plus trailing zero
    // to signal implicit end of run at the end
    // 444553 ->
    // 12010 0
    let runs: Vec<i32> = digits
        .iter()
        .zip(digits.iter().skip(1))
        .map(|(f, s)| (f == s) as i32)
        .scan(0, |state, x| {
            *state = if x > 0 { *state + x } else { 0 };
            Some(*state)
        })
        .chain(std::iter::once(0))
        .collect();

    // Find any adjacent (1, 0) pairs indicating that
    // somewhere there was a run of length 1, meaning two
    // of the same digit but not more than two
    runs
        .iter()
        .zip(runs.iter().skip(1))
        .map(|(&f, &s)| f == 1 && s == 0)
        .any(|x| x)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide one argument with the range");
    }

    let mut iter = args[1].split("-");
    let range_low = i32::from_str(iter.next().expect("Need two values")).expect("failed to parse");
    let range_high = i32::from_str(iter.next().expect("Need two values")).expect("failed to parse");

    let mut sol1: Vec<i32> = vec![];
    let mut sol2: Vec<i32> = vec![];
    for x in range_low..range_high + 1 {
        let digits = digits(x);
        if monotonic_digits(&digits) {
            if adjacent_digits_match_sol1(&digits) {
                sol1.push(x);
            }
            if adjacent_digits_match_sol2(&digits) {
                sol2.push(x);
            }
        }
    }
    println!("sol1: {}", sol1.len());
    println!("sol2: {}", sol2.len());
}

#[cfg(test)]
mod tests {
    use adjacent_digits_match_sol1;
    use adjacent_digits_match_sol2;
    use digits;
    use monotonic_digits;

    #[test]
    fn test1() {
        assert_eq!(adjacent_digits_match_sol1(&digits(112345)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(122345)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123345)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123445)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123455)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(111345)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(122245)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123335)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123444)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(555555)), true);
        assert_eq!(adjacent_digits_match_sol1(&digits(123456)), false);
    }
    #[test]
    fn test2() {
        assert_eq!(monotonic_digits(&digits(123456)), true);
        assert_eq!(monotonic_digits(&digits(112345)), true);
        assert_eq!(monotonic_digits(&digits(111111)), true);
        assert_eq!(monotonic_digits(&digits(101111)), false);
        assert_eq!(monotonic_digits(&digits(110111)), false);
        assert_eq!(monotonic_digits(&digits(111011)), false);
        assert_eq!(monotonic_digits(&digits(111101)), false);
        assert_eq!(monotonic_digits(&digits(111110)), false);
    }
    #[test]
    fn test3() {
        assert_eq!(adjacent_digits_match_sol2(&digits(112345)), true);
        assert_eq!(adjacent_digits_match_sol2(&digits(122345)), true);
        assert_eq!(adjacent_digits_match_sol2(&digits(123345)), true);
        assert_eq!(adjacent_digits_match_sol2(&digits(123445)), true);
        assert_eq!(adjacent_digits_match_sol2(&digits(123455)), true);
        assert_eq!(adjacent_digits_match_sol2(&digits(111345)), false);
        assert_eq!(adjacent_digits_match_sol2(&digits(122245)), false);
        assert_eq!(adjacent_digits_match_sol2(&digits(123335)), false);
        assert_eq!(adjacent_digits_match_sol2(&digits(123444)), false);
        assert_eq!(adjacent_digits_match_sol2(&digits(555555)), false);
        assert_eq!(adjacent_digits_match_sol2(&digits(123456)), false);
    }

}
