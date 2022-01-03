// Arithmetic
pub mod p31 {
    // P31 (**) Determine whether a given integer number is prime.
// 7.isPrime
// result: true
    trait PrimeCheck {
        fn is_prime(&self) -> bool;
    }

    impl PrimeCheck for u8 {
        fn is_prime(&self) -> bool {
            if self.eq(&1) {
                true
            } else {
                let can_divide = (2..(self - 1)).into_iter().any(|x| self % x == 0);
                !can_divide
            }
        }
    }

    pub fn p31_is_prime(number: u8) -> bool {
        number.is_prime()
    }
}

// P32 (**) Determine the greatest common divisor of two positive integer numbers.
// Use Euclid's algorithm.
// gcd(36, 63)
// reult: 9
pub fn p32_gcd(a: u16, b: u16) -> u16 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        p32_gcd(b, a % b)
    }
}

// P33 (*) Determine whether two positive integer numbers are coprime.
// Two numbers are coprime if their greatest common divisor equals 1.
// 35.isCoprimeTo(64)
// result: true
mod p33 {
    use crate::arithmetic_p31_to_p41::p32_gcd;

    pub trait Coprime {
        fn is_coprime_to(&self, b: u16) -> bool;
    }

    impl Coprime for u16{
        fn is_coprime_to(&self, b: u16) -> bool {
            p32_gcd(*self, b) == 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::p31::*;
    use super::p33::*;

    #[test]
    fn test_p31_is_prime() {
        assert!(p31_is_prime(1));
        assert!(p31_is_prime(2));
        assert!(p31_is_prime(3));
        assert!(!p31_is_prime(4));
        assert!(p31_is_prime(7));
    }

    #[test]
    fn test_p32_gcd() {
        assert_eq!(p32_gcd(270, 192), 6);
        assert_eq!(p32_gcd(36, 63), 9);
    }

    #[test]
    fn test_p33_is_coprime_to() {
        assert!(35_u16.is_coprime_to(64));
    }
}