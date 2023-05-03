#![no_main]
use libfuzzer_sys::fuzz_target;
use rand::SeedableRng;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep, pkcs8::DecodePrivateKey};
use rand_chacha::ChaCha8Rng;

const PRIV_KEY: &str = include_str!("../fuzz_privkey");

fuzz_target!(|input: &[u8]| {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let priv_key = RsaPrivateKey::from_pkcs8_pem(PRIV_KEY).unwrap();
    let pub_key = RsaPublicKey::from(priv_key.clone());
    
    let enc_data = pub_key.encrypt(&mut rng, Oaep::new::<sha2::Sha256>(), input).unwrap();
    let dec_data = priv_key.decrypt(Oaep::new::<sha2::Sha256>(), &enc_data).unwrap();
    assert_eq!(dec_data[..], input[..]);
});