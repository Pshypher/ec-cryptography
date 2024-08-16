use num_bigint::BigUint;

#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
pub struct EllipticCurve {
    // y^2 = x^3 + ax + b;
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    pub fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        Self { a, b, p }
    }
    pub fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "{:?} is not on curve", c);
        assert!(self.is_on_curve(d), "{:?} is not on curve", d);
        assert_ne!(*c, *d, "Points should not be the same");

        match (c, d) {
            (Point::Identity, Point::Coordinate(x, y)) => Point::Coordinate(x.clone(), y.clone()),
            (Point::Coordinate(x, y), Point::Identity) => Point::Coordinate(x.clone(), y.clone()),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                if FiniteField::add(y1, y2, &self.p) == BigUint::from(0u32) && x1 == x2 {
                    return Point::Identity
                }
                // s = (y2 - y1) / (x2 - x1) mod p
                // x3 = s^2 - x1 - x2 mod p
                // y3 = s(x1 - x3) - y1 mod p
                let delta_y = FiniteField::subtract(y2, y1, &self.p);
                let delta_x = FiniteField::subtract(x2, x1, &self.p);
                let s = FiniteField::divide(&delta_y, &delta_x, &self.p);
                self.compute_third_point(x1, y1, x2, &s)
            }
            _ => Point::Identity,
        }
    }

    pub fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "{:?} is not on curve", c);

        if let Point::Coordinate(x1, y1) = c {
            // s = (3 * x1^2 + a) / (2 * y1) mod p
            // x3 = s^2 - 2 * x1 mod p
            // y3 = s(x1 - x3) - y1 mod p
            let x1_squared = x1.modpow(&BigUint::from(2u32), &self.p);
            let numerator = FiniteField::add(
                &FiniteField::multiplication(&BigUint::from(3u32), &x1_squared, &self.p),
                &self.a,
                &self.p
            );
            let denominator = FiniteField::multiplication(&BigUint::from(2u32), y1, &self.p);
            let s = FiniteField::divide(&numerator, &denominator, &self.p);
            self.compute_third_point(x1, y1, x1, &s)
        } else {
            Point::Identity
        }
    }

    pub fn scalar_multiplication(&self, a: &Point, d: &BigUint) -> Point {
        // addition/doubling algorithm - B = d * A
        //
        // T = A
        // for i in range(bits of d - 1, 0)
        //      T = 2 * T
        //      if bit i of d == 1
        //          T = T + A
        let mut t = a.clone();
        for i in (0..d.bits() - 1).rev() {
            t = self.double(&t);
            if d.bit(i) {
                t = self.add(&t, a);
            }
        }
        t
    }

    pub fn is_on_curve(&self, c: &Point) -> bool {
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

    fn compute_third_point(&self, x1: &BigUint, y1: &BigUint, x2: &BigUint, s: &BigUint) -> Point {
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
        assert!(x3 < self.p, "{x3} >= {}", self.p);
        assert!(y3 < self.p, "{y3} >= {}", self.p);

        Point::Coordinate(x3, y3)
    }
}

pub struct FiniteField;

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        assert!(c < p, "{c} >= {p}");
        assert!(d < p, "{d} >= {p}");
        // c + d = r mod p
        let r = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
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

    fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
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

    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
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

    #[test]
    fn test_ec_point_doubling() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        // (5, 1) + (5, 1) = 2 * (5, 1) = (6, 3)
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let result = ec.double(&p1);
        assert_eq!(result, pr);
    }

    #[test]
    fn test_ec_point_doubling_identity() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        // Point::Identity + Point::Identity = 2 * Point::Identity = Point::Identity
        let p1 = Point::Identity;
        let pr = Point::Identity;

        let result = ec.double(&p1);
        assert_eq!(result, pr);
    }

    #[test]
    fn test_ec_scalar_multiplication() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        let a = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        // 2 * (5, 1) = (6, 3)
        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let result = ec.scalar_multiplication(&a, &BigUint::from(2u32));
        assert_eq!(result, pr);

        // 10 * (5, 1) = (7, 11)
        let pr = Point::Coordinate(BigUint::from(7u32), BigUint::from(11u32));
        let result = ec.scalar_multiplication(&a, &BigUint::from(10u32));
        assert_eq!(result, pr);

        // 16 * (5, 1) = (10, 11)
        let pr = Point::Coordinate(BigUint::from(10u32), BigUint::from(11u32));
        let result = ec.scalar_multiplication(&a, &BigUint::from(16u32));
        assert_eq!(result, pr);

        // 17 * (5, 1) = (6, 14)
        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(14u32));
        let result = ec.scalar_multiplication(&a, &BigUint::from(17u32));
        assert_eq!(result, pr);

        // 18 * (5, 1) = (5, 16)
        let pr = Point::Coordinate(BigUint::from(5u32), BigUint::from(16u32));
        let result = ec.scalar_multiplication(&a, &BigUint::from(18u32));
        assert_eq!(result, pr);

        // 19 * (5, 1) = Point::Identity
        let pr = Point::Identity;
        let result = ec.scalar_multiplication(&a, &BigUint::from(19u32));
        assert_eq!(result, pr);
    }

    #[test]
    fn test_bits() {
        let a = BigUint::from(2u32);
        assert!(!a.bit(0));
        assert!(a.bit(1));
    }
}
