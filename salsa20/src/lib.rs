//! The Salsa20 stream cipher.
//!
//! Cipher functionality is accessed using traits from re-exported
//! [`stream-cipher`](https://docs.rs/stream-cipher) crate.
//!
//! # Security Warning
//!
//! This crate does not ensure ciphertexts are authentic! Thus ciphertext integrity
//! is not verified, which can lead to serious vulnerabilities!
//!
//! # Usage
//!
//! ```
//! use salsa20::Salsa20;
//! use salsa20::stream_cipher::generic_array::GenericArray;
//! use salsa20::stream_cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};
//!
//! let mut data = [1, 2, 3, 4, 5, 6, 7];
//!
//! let key = GenericArray::from_slice(b"an example very very secret key.");
//! let nonce = GenericArray::from_slice(b"a nonce.");
//!
//! // create cipher instance
//! let mut cipher = Salsa20::new(&key, &nonce);
//!
//! // apply keystream (encrypt)
//! cipher.apply_keystream(&mut data);
//! assert_eq!(data, [182, 14, 133, 113, 210, 25, 165]);
//!
//! // seek to the keystream beginning and apply it again to the `data` (decrypt)
//! cipher.seek(0);
//! cipher.apply_keystream(&mut data);
//! assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
//! ```

#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]
#![warn(missing_docs, rust_2018_idioms, trivial_casts, unused_qualifications)]

pub use stream_cipher;

mod block;
mod cipher;
#[cfg(feature = "xsalsa20")]
mod xsalsa20;

#[cfg(feature = "xsalsa20")]
pub use self::xsalsa20::XSalsa20;

#[cfg(feature = "hsalsa20")]
pub use self::xsalsa20::hsalsa20;

use crate::{block::Block, cipher::Cipher};
use core::convert::TryInto;
use stream_cipher::generic_array::typenum::{U32, U8};
use stream_cipher::generic_array::GenericArray;
use stream_cipher::{LoopError, NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek};

/// Size of a Salsa20 block in bytes
pub const BLOCK_SIZE: usize = 64;

/// Size of a Salsa20 key in bytes
pub const KEY_SIZE: usize = 32;

/// Number of bytes in the Salsa20 IV
const IV_SIZE: usize = 8;

/// Number of 32-bit words in the Salsa20 state
const STATE_WORDS: usize = 16;

/// State initialization constant ("expand 32-byte k")
const CONSTANTS: [u32; 4] = [0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574];

/// The Salsa20 cipher.
#[derive(Debug)]
pub struct Salsa20(Cipher);

impl NewStreamCipher for Salsa20 {
    /// Key size in bytes
    type KeySize = U32;

    /// Nonce size in bytes
    type NonceSize = U8;

    fn new(key: &GenericArray<u8, Self::KeySize>, iv: &GenericArray<u8, Self::NonceSize>) -> Self {
        let block = Block::new(key.as_slice().try_into().unwrap(), (*iv).into());

        Salsa20(Cipher::new(block))
    }
}

impl SyncStreamCipherSeek for Salsa20 {
    fn current_pos(&self) -> u64 {
        self.0.current_pos()
    }

    fn seek(&mut self, pos: u64) {
        self.0.seek(pos);
    }
}

impl SyncStreamCipher for Salsa20 {
    fn try_apply_keystream(&mut self, data: &mut [u8]) -> Result<(), LoopError> {
        self.0.try_apply_keystream(data)
    }
}
