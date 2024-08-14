use num_bigint::{BigUint};

struct Point {
    x: BigUint,
    y: BigUint,
}
struct EllipticCurve {
    // y^2 = x^3 + ax + b;
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn add(c: &Point, d: &Point) -> Point {
        todo!()
    }

    fn double(c: &Point) -> Point {
        todo!()
    }

    fn scalar_multiplication(c: &Point, d: &BigUint) -> Point {
        // addition/doubling algorithm
        // B = d * A
        todo!()
    }
}

struct FiniteField;

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p
        let r = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        let r = c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        assert!(c < p, "number: {} is bigger or equal than: {}", c, p);
        p - c
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function uses Fermat's Little Theorem and thus is only valid for primes(p)
        // only for p as a prime
        // c^(-1) mod p = c^(p-2) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
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
    fn test_inverse_multiplication_identity() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(17u32);

        let c_inverse = FiniteField::inverse_multiplication(&c, &p);
        let r = FiniteField::multiplication(&c, &c_inverse, &p);

        assert_eq!(r, BigUint::from(1u32));
    }
}