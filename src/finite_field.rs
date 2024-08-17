use num_bigint::BigUint;

pub struct FiniteField;

impl FiniteField {
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p

        assert!(c < p, "{c} >= {p}");
        assert!(d < p, "{d} >= {p}");

        let r = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p

        assert!(c < p, "{c} >= {p}");
        assert!(d < p, "{d} >= {p}");

        let r = c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p

        assert!(c < p, "number: {} is bigger or equal than: {}", c, p);

        p - c
    }

    pub fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c - d mod p

        assert!(c < p, "{c} >= {p}");
        assert!(d < p, "{d} >= {p}");

        let d_inverse = FiniteField::inverse_addition(d, p);
        assert!(d_inverse < p.clone(), "{d_inverse} >= {p}");

        FiniteField::add(c, &d_inverse, p)
    }

    // TODO: this function uses Fermat's Little Theorem and thus is only valid for primes(p)
    // only for p as a prime
    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // c^(-1) mod p = c^(p-2) mod p

        assert!(c < p, "{c} >= {p}");

        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    pub fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        assert!(c < p, "{c} >= {p}");
        assert!(d < p, "{d} >= {p}");

        let d_inverse = FiniteField::inverse_multiplication(d, p);
        assert!(d_inverse < p.clone(), "{d_inverse} >= {p}");

        FiniteField::multiplication(c, &d_inverse, p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_one() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::add(&c, &d, &p);

        assert_eq!(r, BigUint::from(3u32));
    }

    #[test]
    fn test_add_two() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(31u32);

        let r = FiniteField::add(&c, &d, &p);

        assert_eq!(r, BigUint::from(14u32));
    }

    #[test]
    fn test_multiplication_one() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::multiplication(&c, &d, &p);

        assert_eq!(r, BigUint::from(7u32));
    }

    #[test]
    fn test_multiplication_two() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(51u32);

        let r = FiniteField::multiplication(&c, &d, &p);

        assert_eq!(r, BigUint::from(40u32));
    }

    #[test]
    fn test_inverse_addition_one() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(31u32);

        let r = FiniteField::inverse_addition(&c, &p);

        assert_eq!(r, BigUint::from(27u32));
    }

    #[test]
    #[should_panic]
    fn test_inverse_addition_two() {
        let c = BigUint::from(32u32);
        let p = BigUint::from(31u32);

        FiniteField::inverse_addition(&c, &p);
    }

    #[test]
    fn test_inverse_addition_identity() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(31u32);

        let c_inverse = FiniteField::inverse_addition(&c, &p);
        let r = FiniteField::add(&c, &c_inverse, &p);

        assert_eq!(r, BigUint::from(0u32));
    }

    #[test]
    fn test_subtract() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(31u32);

        assert_eq!(FiniteField::subtract(&c, &c, &p), BigUint::from(0u32))
    }

    #[test]
    fn test_inverse_multiplication_identity() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(17u32);

        let c_inverse = FiniteField::inverse_multiplication(&c, &p);
        let r = FiniteField::multiplication(&c, &c_inverse, &p);

        assert_eq!(r, BigUint::from(1u32));
    }

    #[test]
    fn test_divide() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        assert_eq!(FiniteField::divide(&c, &c, &p), BigUint::from(1u32));
    }


}
