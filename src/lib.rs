#![no_std]
#![no_main]
#![feature(libc)]
#![feature(default_alloc_error_handler)]
#![feature(const_fn_fn_ptr_basics)]

extern crate alloc;

use alloc::string::{String, ToString};
use linked_list_allocator::LockedHeap;
use cstr_core::{CStr, CString, c_char};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn libunid_init() {
    let heap_start = 0x20000000;
    let heap_end   = 0x20000000 + (1024 * 10); /* 1024 bytes (1k) x 10 = 10k */
    let heap_size  = heap_end - heap_start;

    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

// Utils
pub mod utils;

// Ciphers
pub mod ciphers;

// Runtime
pub mod runtime;

// TODO: FREE_MEMORY

/// runtime :: bip39 :: generate_mnemonic
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn runtime_bip39_generate_mnemonic() -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// utils :: random :: get_random_bytes
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn utils_random_get_random_bytes(_length: i32) -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// utils :: codec :: base64_encode
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn utils_codec_base64_encode(content: *const c_char) -> *mut c_char {
    let v1 = {
        assert!(! content.is_null());

        CStr::from_ptr(content)
    };
    let v1_str = v1.to_str().unwrap().to_string();

    let r = utils::codec::Codec::base64_encode(v1_str);
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// utils :: codec :: base64_decode
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn utils_codec_base64_decode(content: *const c_char) -> *mut c_char {
    let v1 = {
        assert!(! content.is_null());

        CStr::from_ptr(content)
    };
    let v1_str = v1.to_str().unwrap().to_string();

    let r = utils::codec::Codec::base64_decode(v1_str);
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// utils :: multihasher :: hash
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn utils_multihasher_hash(_content: *const c_char) -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: signer :: sign
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_signer_sign() -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: signer :: verify
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_signer_verify() -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: cipher :: encrypt
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_cipher_encrypt() -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: cipher :: decrypt
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_cipher_decrypt() -> *mut c_char {
    let r = String::from("DUMMY");
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: hasher :: digest
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_hasher_digest(content: *const c_char, secret: *const c_char) -> *mut c_char {
    // v1
    let v1 = {
        assert!(! content.is_null());

        CStr::from_ptr(content)
    };
    let v1_str = v1.to_str().unwrap().to_string();

    // v2
    let v2 = {
        assert!(! secret.is_null());

        CStr::from_ptr(secret)
    };
    let v2_str = v2.to_str().unwrap().to_string();

    // result
    let r = ciphers::hasher::Hasher::digest(v1_str, v2_str);
    let r_c_str = CString::new(r).unwrap();

    r_c_str.into_raw()
}

/// ciphers :: hasher :: verify
/// 
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn ciphers_hasher_verify(content: *const c_char, digest: *const c_char, secret: *const c_char) -> bool {
    // v1
    let v1 = {
        assert!(! content.is_null());

        CStr::from_ptr(content)
    };
    let v1_str = v1.to_str().unwrap().to_string();

    // v2
    let v2 = {
        assert!(! digest.is_null());

        CStr::from_ptr(digest)
    };
    let v2_str = v2.to_str().unwrap().to_string();

    // v3
    let v3 = {
        assert!(! secret.is_null());

        CStr::from_ptr(secret)
    };
    let v3_str = v3.to_str().unwrap().to_string();

    // result
    ciphers::hasher::Hasher::verify(v1_str, v2_str, v3_str)
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub extern "C" fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}