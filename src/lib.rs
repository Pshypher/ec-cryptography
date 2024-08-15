use num_bigint::BigUint;

#[derive(Debug, PartialEq)]
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
struct EllipticCurve {
    // y^2 = x^3 + ax + b;
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        Self { a, b, p }
    }
    fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "{:?} is not on curve", c);
        assert!(self.is_on_curve(d), "{:?} is not on curve", d);
        assert_ne!(*c, *d, "Points should not be the same");

        match (c, d) {
            (Point::Identity, Point::Coordinate(x, y)) => Point::Coordinate(x.clone(), y.clone()),
            (Point::Coordinate(x, y), Point::Identity) => Point::Coordinate(x.clone(), y.clone()),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                // s = (y2 - y1) / (x2 - x1) mod p
                // x3 = s^2 - x1 - x2 mod p
                // y3 = s(x1 - x3) - y1 mod p
                let delta_y = FiniteField::subtract(y2, y1, &self.p);
                let delta_x = FiniteField::subtract(x2, x1, &self.p);
                let s = FiniteField::divide(&delta_y, &delta_x, &self.p);
                let s_square = s.modpow(&BigUint::from(2u32), &self.p);
                let x3 = FiniteField::subtract(
                    &FiniteField::subtract(&s_square, x1, &self.p),
                    &x2,
                    &self.p,
                );
                let y3 = FiniteField::subtract(
                    &FiniteField::multiplication(
                        &s,
                        &FiniteField::subtract(x1, &x3, &self.p),
                        &self.p,
                    ),
                    y1,
                    &self.p,
                );
                Point::Coordinate(x3, y3)
            }
            _ => Point::Identity,
        }
    }

    fn double(c: &Point) -> Point {
        todo!()
    }

    fn scalar_multiplication(c: &Point, d: &BigUint) -> Point {
        // addition/doubling algorithm
        // B = d * A
        todo!()
    }

    fn is_on_curve(&self, c: &Point) -> bool {
        if let Point::Coordinate(x, y) = c {
            // y^2 = x^3 + a * x + b
            let y_square = y.modpow(&BigUint::from(2u32), &self.p);
            let x_cubed = x.modpow(&BigUint::from(3u32), &self.p);
            let ax = FiniteField::multiplication(&self.a, x, &self.p);
            y_square
                == FiniteField::add(&x_cubed, &FiniteField::add(&ax, &self.b, &self.p), &self.p)
        } else {
            true
        }
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

    fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inverse = FiniteField::inverse_addition(d, p);
        FiniteField::add(c, &d_inverse, p)
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function uses Fermat's Little Theorem and thus is only valid for primes(p)
        // only for p as a prime
        // c^(-1) mod p = c^(p-2) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inverse = FiniteField::inverse_multiplication(d, p);
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

    #[test]
    fn test_ec_point_addition() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        // (6, 3) + (5, 1) = (10, 6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let p3 = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));

        let result = ec.add(&p1, &p2);
        assert_eq!(result, p3);
    }
}
