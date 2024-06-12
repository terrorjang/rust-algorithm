use std::ops::Neg;

use num_traits::Zero;

pub fn abs<T>(num: T) -> T
where
    T: PartialOrd + Neg<Output = T> + Zero,
{
    // PartialOrd
    if num < T::zero() {
        return -num;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        assert_eq!(80, abs(80));
        assert_eq!(80, abs(-80));
    }

    #[test]
    fn test_f64() {
        assert_eq!(80.80, abs(80.80));
        assert_eq!(80.80, abs(-80.80));
    }

    #[test]
    fn zero() {
        assert_eq!(0.0, abs(0.0));
    }
}

/*
first error
----------------
error[E0369]: binary operation `<` cannot be applied to type `T`
 --> src/math/abs.rs:8:12
  |
8 |     if num < 0 {
  |        --- ^ - {integer}
  |        |
  |        T
  |
help: consider restricting type parameter `T`
  |
3 | pub fn abs<T: std::cmp::PartialOrd<i32>>(num: T) -> T

----- solution
add T : PartialOrd

Second error
----------------
error[E0600]: cannot apply unary operator `-` to type `T`
 --> src/math/abs.rs:9:16
  |
9 |         return -num;
  |                ^^^^ cannot apply unary operator `-`
  |
help: consider restricting type parameter `T`
  |
3 | pub fn abs<T: std::ops::Neg>(num: T) -> T
  |             +++++++++++++++

  solution
add Neg


ERROR
error[E0308]: mismatched types
 --> src/math/abs.rs:9:16
  |
3 | pub fn abs<T>(num: T) -> T
  |            -             - expected `T` because of return type
  |            |
  |            expected this type parameter
...
9 |         return -num;
  |                ^^^^ expected type parameter `T`, found associated type
  |
  = note: expected type parameter `T`
            found associated type `<T as Neg>::Output`
help: consider further restricting this bound
  |
5 |     T: PartialOrd + Neg<Output = T>,
  |                        ++++++++++++

SOLUTION
add <Output = T>


ERROR
error[E0308]: mismatched types
 --> src/math/abs.rs:8:14
  |
3 | pub fn abs<T>(num: T) -> T
  |            - expected this type parameter
...
8 |     if num < 0 {
  |        ---   ^ expected type parameter `T`, found integer
  |        |
  |        expected because this is `T`
  |
  = note: expected type parameter `T`
                       found type `{integer}`

SOLUTION
add num_traits::Zero
https://users.rust-lang.org/t/expected-type-parameter-t-found-integer-when-use-generics-instead-of-uint/77906/2

add crate
- num-traits = "0.2.19"
*/
