// Arithmetic

// P31 (**) Determine whether a given integer number is prime.
// 7.isPrime
// result: true
trait PrimeCheck {
    fn is_prime(&self) -> bool;
}

impl PrimeCheck for u8 {
    fn is_prime(&self) -> bool {
        if self.eq(&1) {
            return true;
        } else {
            let can_divide = (2..(self - 1)).into_iter().find(|x| self % x == 0).is_some();
            return !can_divide;
        }
    }
}
pub fn p31_is_prime(number: u8) -> bool {
    number.is_prime()
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
        p32_gcd(b, a %b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p31_is_prime() {
        assert!(p31_is_prime(1));
        assert!(p31_is_prime(2));
        assert!(p31_is_prime(3));
        assert!(!p31_is_prime(4));
        assert!(p31_is_prime(7));
    }

    #[test]
    fn test_p32_gcd(){
        assert_eq!(p32_gcd(270, 192),  6);
        assert_eq!(p32_gcd(36, 63),  9);
    }
}