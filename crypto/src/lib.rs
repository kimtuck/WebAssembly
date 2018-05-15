#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate crypto;


use wasm_bindgen::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[wasm_bindgen(module = "./index")]
extern {
    fn update_text(s: &str);
}

#[wasm_bindgen]
pub fn decrypt_rust(symmetricKey: &[u8], iv: &[u8], encryptedText: &[u8]) {
    let decryptedArray = decrypt_block(encryptedText, symmetricKey,iv);
    let decrypted = str::from_utf8(decryptedArray).unwrap();
    decrypted;
}

fn decrypt_block(block: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor: Box<symmetriccipher::Decryptor> = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding
    );

    let mut final_result = Vec::<u8>::new();
    let mut buffer = [0, ..4096];
    let mut read_buffer = buffer::RefReadBuffer::new(block);
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.push_all(write_buffer.take_read_buffer().take_remaining());
        match result {
            buffer::BufferResult::BufferUnderflow => break,
            buffer::BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

