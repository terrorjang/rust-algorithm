// 각 자리 수의 합.

pub fn sum_digits_iterative(num: i32) -> i32 {
    let mut result = 0;
    let mut num = num;

    while num != 0 {
        result += num % 10;
        num /= 10;
    }

    result
}

pub fn sum_digits_recursive(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    num % 10 + sum_digits_recursive(num / 10)
}

#[cfg(test)]
mod tests {
    mod iterative {
        use super::super::sum_digits_iterative as sum_digits;

        #[test]
        fn zero() {
            assert_eq!(0, sum_digits(0));
        }

        #[test]
        fn positive_number() {
            assert_eq!(1, sum_digits(1));
            assert_eq!(10, sum_digits(1234));
            assert_eq!(30, sum_digits(454566));
            assert_eq!(7, sum_digits(500020));
        }

        #[test]
        fn negative_number() {
            assert_eq!(-1, sum_digits(-1));
            assert_eq!(-10, sum_digits(-1234));
            assert_eq!(-30, sum_digits(-454566));
            assert_eq!(-7, sum_digits(-500020));
        }

        #[test]
        fn trailing_zeros() {
            assert_eq!(1, sum_digits(1000000000));
            assert_eq!(3, sum_digits(300));
        }
    }

    mod recursive {
        use super::super::sum_digits_recursive as sum_digits;

        #[test]
        fn zero() {
            assert_eq!(0, sum_digits(0));
        }

        #[test]
        fn positive_number() {
            assert_eq!(1, sum_digits(1));
            assert_eq!(10, sum_digits(1234));
            assert_eq!(30, sum_digits(454566));
            assert_eq!(7, sum_digits(500020));
        }

        #[test]
        fn negative_number() {
            assert_eq!(-1, sum_digits(-1));
            assert_eq!(-10, sum_digits(-1234));
            assert_eq!(-30, sum_digits(-454566));
            assert_eq!(-7, sum_digits(-500020));
        }

        #[test]
        fn trailing_zeros() {
            assert_eq!(1, sum_digits(1000000000));
            assert_eq!(3, sum_digits(300));
        }
    }
}
