fn main() {
    let digits = to_digits(&std::env::args().nth(1).unwrap());
    let shift = match std::env::args().nth(2) {
        Some(ref arg) if arg.parse::<usize>().is_ok() => arg.parse().unwrap(),
        Some(ref s) if s == "-h" => digits.len() / 2,
        _ => 1
    };
    println!("digits = {:?}", digits);
    println!("len = {}", digits.len());
    println!("shift = {:?}", shift);
    println!("Unlock = {}", sum_digits_shifted(&digits, shift))
}


fn to_digits<S: AsRef<str>>(s: S) -> Vec<u32> {
    s.as_ref().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn sum_digits<V: AsRef<[u32]>>(digits: V) -> u32 {
    sum_digits_shifted(digits, 1)
}

fn sum_digits_shifted<V: AsRef<[u32]>>(digits: V, shift: usize) -> u32 {
    let digits = digits.as_ref();
    digits.iter()
        .zip(digits.iter().cycle().skip(shift))
        .filter_map(|pair|
            {
                let (a,b) = pair;
                match a == b {
                    true => Some(a),
                    false => None
                }
            }
        ).sum::<u32>()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_string_to_digits() {
        assert_eq!(Vec::<u32>::new(), to_digits(""));
        assert_eq!(vec![1, 2, 3], to_digits("123"));
        assert_eq!(vec![9, 1, 1], to_digits("911"));
    }

    #[test]
    fn to_digits_should_accept_string_too() {
        assert_eq!(vec![1, 2, 3], to_digits(String::from("123")));
    }

    #[test]
    fn sum_digits_that_match_next() {
        assert_eq!(3, sum_digits(&vec![1, 1, 2, 2]));
        assert_eq!(0, sum_digits(&vec![]));
        assert_eq!(0, sum_digits(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn sum_digits_shift_should_be_circular() {
        assert_eq!(4, sum_digits_shifted(&vec![1, 1, 1, 1], 2));
        assert_eq!(9, sum_digits_shifted(&vec![9, 1, 2, 1, 2, 1, 2, 9], 1));
        assert_eq!(6, sum_digits_shifted(&vec![9, 1, 2, 1, 2, 1, 2, 9], 4));
    }

    #[test]
    fn sum_digits_should_take_vector_too() {
        assert_eq!(3, sum_digits(vec![1, 1, 2, 2]))
    }
}
