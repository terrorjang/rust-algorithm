// 올림 함수
pub fn ceil(x: f64) -> f64 {
    let temp = x as i64 as f64;
    if temp == x || x < 0.0 {
        return temp;
    }

    temp + 1_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
        let x = 1.10;
        assert_eq!(ceil(x), x.ceil());
        // 1.1 > 2.0
    }

    #[test]
    fn positive_decimal_with_small_number() {
        let x = 3.01;
        assert_eq!(ceil(x), x.ceil()); // 4.0
    }

    #[test]
    fn positive_integer() {
        let x = 1.00;
        assert_eq!(ceil(x), x.ceil()); // 1.0
    }

    #[test]
    fn negative_decimal() {
        let x = -1.10;
        assert_eq!(ceil(x), x.ceil()); // -1.0
    }

    #[test]
    fn negative_integer() {
        let x = -1.00;
        assert_eq!(ceil(x), x.ceil());
    }

    #[test]
    fn zero() {
        let x = 0.0;
        assert_eq!(ceil(x), x.ceil());
    }
}
