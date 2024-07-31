use pc_rs::PedersenCommitment;
use curve25519_dalek::Scalar;
use rand::thread_rng;

fn main() {
    let pc = PedersenCommitment::new(&mut thread_rng());
    let v = Scalar::from(42u64);
    //let commitment = pc.commit(&v, &mut thread_rng());
    let commitment = pc.commit(v, &mut thread_rng());

    println!("Commitment: {:?}", commitment);
}
