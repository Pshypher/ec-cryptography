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

struct FiniteField {}

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p
        todo!()
    }

    fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        todo!()
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        todo!()
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // c^-1 mod p
        todo!()
    }
}