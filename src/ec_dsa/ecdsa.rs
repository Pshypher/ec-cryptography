use crate::ec_generic::elliptic_curve::{EllipticCurve, Point};
use crate::ec_generic::finite_field::FiniteField;
use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use sha256::{digest, try_digest};

struct ECDSA {
    elliptic_curve: EllipticCurve,
    a_generator: Point,
    q_order: BigUint,
}

impl ECDSA {
    pub fn new(elliptic_curve: EllipticCurve, a: Point, q: BigUint) -> Self {
        Self {
            elliptic_curve,
            a_generator: a,
            q_order: q,
        }
    }

    // Generates: d, B where B = d * A
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let private_key = self.generate_private_key();
        let public_key = self.generate_public_key(&private_key);
        (private_key, public_key)
    }

    pub fn generate_private_key(&self) -> BigUint {
        self.generate_random_positive_number_less_than(&self.q_order)
    }

    pub fn generate_public_key(&self, private_key: &BigUint) -> Point {
        self.elliptic_curve.scalar_multiplication(&self.a_generator, private_key)
    }

    // (0, max)
    pub fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), max)
    }

    ///
    /// R = k * A -> take `r = x` component
    /// s = (hash(message) + d * r) * k^(-1) mod q
    ///
    pub fn sign(
        &self,
        hash: &BigUint,
        private_key: &BigUint,
        k_random: &BigUint,
    ) -> (BigUint, BigUint) {
        assert!(
            *hash < self.q_order,
            "Hash is bigger than the order of the EC group"
        );
        assert!(
            *private_key < self.q_order,
            "Private key is bigger than the order of the EC group"
        );
        assert!(
            *k_random < self.q_order,
            "Random number `k` is bigger than the order of the EC group"
        );

        let R = self.elliptic_curve.scalar_multiplication(&self.a_generator, k_random);
        if let Point::Coordinate(r, _) = R {
            let k_inverse = FiniteField::inverse_multiplication(k_random, &self.q_order);
            let s = FiniteField::add(
                hash, &FiniteField::multiplication(private_key, &r, &self.q_order), &self.q_order
            );
            let s = FiniteField::multiplication(&s, &k_inverse, &self.q_order);
            (r, s)
        } else {
            panic!("The random point R should not be the identity");
        }
    }

    pub fn verify(&self, hash: &BigUint, public_key: &Point, signature: &(BigUint, BigUint)) -> bool {
        todo!()
    }

    /// 0 < hash < max
    pub fn generate_hash_less_than(message: &str, max: &BigUint) -> BigUint {
        let digest = digest(message);
        let hash_bytes = hex::decode(&digest).expect("Could not convert hash to Vec<u8>");
        let hash = BigUint::from_bytes_be(&hash_bytes);
        let hash = hash.modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
        hash + BigUint::from(1u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve::new(
            BigUint::from(2u32),
            BigUint::from(2u32),
            BigUint::from(17u32),
        );

        let a_generator = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let q_order = BigUint::from(19u32);
        let ecdsa = ECDSA::new(elliptic_curve, a_generator, q_order);

        let private_key = BigUint::from(7u32);
        let public_key = ecdsa.generate_public_key(&private_key);

        let hash = BigUint::from(10u32);
        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1 SOL -> Alice";
        let hash= ECDSA::generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa.sign(&hash, &private_key, &k_random);
        println!("{:?}", signature)
    }
}
