use num_bigint::BigUint;
use crate::ec_generic::finite_field::FiniteField;

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
                    return Point::Identity;
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

        if let Point::Coordinate(x, y) = c {
            // s = (3 * x^2 + a) / (2 * y) mod p
            // x0 = s^2 - 2 * x mod p
            // y0 = s(x - x0) - y mod p
            if *y == BigUint::from(0u32) {
                return Point::Identity
            }
            let x_squared = x.modpow(&BigUint::from(2u32), &self.p);
            let numerator = FiniteField::add(
                &FiniteField::multiplication(&BigUint::from(3u32), &x_squared, &self.p),
                &self.a,
                &self.p,
            );
            let denominator = FiniteField::multiplication(&BigUint::from(2u32), y, &self.p);
            let s = FiniteField::divide(&numerator, &denominator, &self.p);
            self.compute_third_point(x, y, x, &s)
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

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_ec_secp256k1() {
        /*
            y^2 = x^3 + 7 mod p (large)

            p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
            n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
            G = (
                x = 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798,
                y = 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
            )
            a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
            b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007
        */

        // n * G = Point::Identity
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16
        ).expect("Could not convert p");
        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16
        ).expect("Could not convert n");
        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16
        ).expect("Could not convert gx");
        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16
        ).expect("Could not convert gy");

        let ec = EllipticCurve::new(
            BigUint::from(0u32),
            BigUint::from(7u32),
            p,
        );

        let g = Point::Coordinate(gx, gy);

        let result = ec.scalar_multiplication(&g, &n);

        assert_eq!(result, Point::Identity);
    }
}