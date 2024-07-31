use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct PedersenCommitment {
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
}

impl PedersenCommitment {
    pub fn new<R: Rng + rand::CryptoRng>(rng: &mut R) -> Self {
        let g = RistrettoPoint::random(rng);
        let h = RistrettoPoint::random(rng);
        PedersenCommitment { g, h }
    }

    pub fn commit<R: Rng + rand::CryptoRng>(&self, v: Scalar, rng: &mut R) -> RistrettoPoint {
        let r = Scalar::random(rng);
        self.g * v + self.h * r
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
