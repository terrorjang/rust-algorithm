pub fn factorial(number: u64) -> u64 {
    // 0! and 1! are equal to 1
    if number <= 1 {
        return 1;
    }

    // simple
    // (2..=number).product()
    //
    // loop solution
    let mut result = 1;
    for i in 2..=number {
        result *= i;
    }

    result
}

pub fn factorial_recursive(number: u64) -> u64 {
    if number <= 1 {
        return 1;
    }

    number * factorial_recursive(number - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(720, factorial(6));
        assert_eq!(3628800, factorial(10));
    }

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(1, factorial_recursive(0));
        assert_eq!(1, factorial_recursive(1));
        assert_eq!(720, factorial_recursive(6));
        assert_eq!(3628800, factorial_recursive(10));
    }
}
