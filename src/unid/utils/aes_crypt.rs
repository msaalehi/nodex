use crate::unid::utils::data_t::DataT;
use alloc::vec::Vec;

pub struct AesCrypt {
  encryptor  : extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32),
  decryptor : extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32),
}

impl AesCrypt {
  pub const fn empty() -> AesCrypt {
    AesCrypt {
      encryptor : AesCrypt::noop_encryptor,
      decryptor : AesCrypt::noop_decryptor
    }
  }

  extern "C" fn noop_encryptor(_plaintext_data_t: *mut DataT, _key_data_t: *mut DataT, _iv_data_t: *mut DataT, _encrypt_ptr: *mut u8, _len: u32) {}

  extern "C" fn noop_decryptor(_ciphertext_data_t: *mut DataT, _key_data_t: *mut DataT, _iv_data_t: *mut DataT, _encrypt_ptr: *mut u8, _len: u32) {}

  pub fn init(
    &mut self,
    encryptor: extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32),
    decryptor: extern "C" fn(*mut DataT, *mut DataT, *mut DataT, *mut u8, u32)
  ) {
    self.encryptor = encryptor;
    self.decryptor = decryptor;
  }

  pub fn encrypt(&self, plaintext_vec: Vec<u8>, key_vec: Vec<u8>, iv_vec: Vec<u8> ) -> Vec<u8> {
    
    let pos = plaintext_vec.len();
    let len = (pos + 16) & !15;
    let pad = len - pos;

    let mut padded_plaintext_vec : Vec<u8> = alloc::vec![pad as u8; len];

    padded_plaintext_vec[..pos].copy_from_slice(&plaintext_vec);

    let mut padded_plaintext_data_t : DataT = DataT::new(padded_plaintext_vec);

    let mut key_data_t : DataT = DataT::new(key_vec);

    let mut iv_data_t : DataT = DataT::new(iv_vec);

    let encrypt_vec: Vec<u8> = alloc::vec![0u8; len];
    let encrypt_data_t: DataT = DataT::new(encrypt_vec);
    let encrypt_ptr: *mut u8 = encrypt_data_t.ptr;
    let encrypt_len: u32 = encrypt_data_t.len;

    (self.encryptor)(&mut key_data_t, &mut iv_data_t, &mut padded_plaintext_data_t, encrypt_ptr, encrypt_len);

    unsafe {
      let logger = crate::Logger::new(crate::MUTEX_HANDLERS.lock().get_debug_message_handler());

      logger.debug(alloc::format!("len, pos, pad = {:?} {:?} {:?}", len, pos, pad));

      logger.debug(alloc::format!("padded_plaintext_data_t ptr = {:?}", padded_plaintext_data_t.ptr));

      logger.debug(alloc::format!("key_data_t ptr = {:?}", key_data_t.ptr));
  
      logger.debug(alloc::format!("iv_data_t ptr  = {:?}", iv_data_t.ptr));

      logger.debug(alloc::format!("padded plaintext bytes = {:?}", padded_plaintext_data_t.to_vec()));
      logger.debug(alloc::format!("padded plaintext size = {:?}", padded_plaintext_data_t.len));

      logger.debug(alloc::format!("ciphertext bytes = {:?}", encrypt_data_t.to_vec()));
      logger.debug(alloc::format!("ciphertext size = {:?}", encrypt_data_t.len));
    };

    encrypt_data_t.to_vec()
  }

  pub fn decrypt(&self, ciphertext_vec: Vec<u8>, key_vec: Vec<u8>, iv_vec: Vec<u8> ) -> Vec<u8> {
    
    let len: usize = ciphertext_vec.len();
    assert!(len >= 16 && len % 16 == 0);

    let mut ciphertext_data_t : DataT = DataT::new(ciphertext_vec);

    let mut key_data_t : DataT = DataT::new(key_vec);

    let mut iv_data_t : DataT = DataT::new(iv_vec);

    let decrypt_vec: Vec<u8> = alloc::vec![0u8; len];
    let decrypt_data_t: DataT = DataT::new(decrypt_vec);
    let decrypt_ptr: *mut u8 = decrypt_data_t.ptr;
    let decrypt_len: u32 = decrypt_data_t.len;

    (self.decryptor)(&mut key_data_t, &mut iv_data_t, &mut ciphertext_data_t, decrypt_ptr, decrypt_len);

    let padded_plaintext_vec: Vec<u8> = decrypt_data_t.to_vec();

    let pad : usize = padded_plaintext_vec[len-1] as usize;
    let pos : usize = len - pad;

    unsafe {
      let logger = crate::Logger::new(crate::MUTEX_HANDLERS.lock().get_debug_message_handler());
      
      logger.debug(alloc::format!("len, pos, pad = {:?} {:?} {:?}", len, pos, pad));

      logger.debug(alloc::format!("ciphertext_data_t ptr = {:?}", ciphertext_data_t.ptr));

      logger.debug(alloc::format!("key_data_t ptr = {:?}", key_data_t.ptr));
  
      logger.debug(alloc::format!("iv_data_t ptr = {:?}", iv_data_t.ptr));

      logger.debug(alloc::format!("ciphertext bytes = {:?}", ciphertext_data_t.to_vec()));
      logger.debug(alloc::format!("ciphertext size = {:?}", ciphertext_data_t.len));

      logger.debug(alloc::format!("padded plaintext bytes = {:?}", decrypt_data_t.to_vec()));
      logger.debug(alloc::format!("padded plaintext size = {:?}", decrypt_data_t.len));   
    }

    (&padded_plaintext_vec[..pos]).to_vec()
  }
}


#[cfg(test)]
pub mod tests {
  use super::*;
  
  #[test]
  #[ignore]
  pub fn it_should_aes_encrypt() {
    let plaintext: &str= "hello";
    let key_base64: &str = "ZEj+lI1NEpwbxqpMfFTwxIK8/XbHsSJtj+dam59NavI=";
    let iv_base64: &str = "QUJDREVGR0hJSktMTU5PUA==";
    let plaintext_vec: Vec<u8> = plaintext.as_bytes().to_vec();
    let key_vec: Vec<u8> = base64::decode(key_base64).unwrap();
    let iv_vec: Vec<u8> = base64::decode(iv_base64).unwrap();

    let ciphertext_vec: Vec<u8> = unsafe { crate::AES_CRYPT.encrypt(plaintext_vec, key_vec, iv_vec) };
    let ciphertext_base64 = base64::encode(ciphertext_vec);

    assert_eq!(ciphertext_base64, "5FBuToCO9PiApjHbK+25Vg==".to_string());
  }

  #[test]
  #[ignore]
  pub fn it_should_aes_decrypt() {
    let ciphertext_base64: &str= "5FBuToCO9PiApjHbK+25Vg==";
    let key_base64: &str = "ZEj+lI1NEpwbxqpMfFTwxIK8/XbHsSJtj+dam59NavI=";
    let iv_base64: &str = "QUJDREVGR0hJSktMTU5PUA==";
    let ciphertext_vec: Vec<u8> = base64::decode(ciphertext_base64).unwrap();
    let key_vec: Vec<u8> = base64::decode(key_base64).unwrap();
    let iv_vec: Vec<u8> = base64::decode(iv_base64).unwrap();

    let plaintext_vec: Vec<u8> = unsafe { crate::AES_CRYPT.decrypt(ciphertext_vec, key_vec, iv_vec) };
    let plaintext_string: String = String::from_utf8(plaintext_vec).unwrap();

    assert_eq!(plaintext_string, "hello");
  }
}