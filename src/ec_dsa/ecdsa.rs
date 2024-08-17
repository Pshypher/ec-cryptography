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

    pub fn sign(
        &self,
        hash: &BigUint,
        private_key: &BigUint,
        k_random: &BigUint,
    ) -> (BigUint, BigUint) {
        todo!()
    }

    pub fn verify(&self, hash: &BigUint, public_key: &Point, signature: &(BigUint, BigUint)) -> bool {
        todo!()
    }
}
