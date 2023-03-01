use dryoc::dryocsecretbox::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoxWithNonce<Nonce, Box> {
    nonce: Nonce,
    message: Box,
}

impl<Nonce, Box> BoxWithNonce<Nonce, Box> {
    pub fn new(nonce: Nonce, message: Box) -> Self {
        Self { nonce, message }
    }
}

fn main() {
    // Generate a random secret key and nonce
    let secret_key = Key::gen();
    let nonce = Nonce::gen();
    let message = b"Why hello there, fren";

    // Encrypt `message`, into a Vec-based box
    let box_with_nonce = BoxWithNonce::new(
        nonce.clone(),
        DryocSecretBox::encrypt_to_vecbox(message, &nonce, &secret_key),
    );

    println!(
        "as json: {:?}",
        serde_json::to_string(&box_with_nonce).unwrap()
    );
    println!(
        "as msgpack: {:?}",
        rmp_serde::to_vec(&box_with_nonce).unwrap()
    );
}
