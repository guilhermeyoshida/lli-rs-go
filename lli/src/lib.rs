use std::ffi::{c_char, CStr, CString};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use tiktoken_rs::CoreBPE;
use tiktoken_rs::tokenizer::{get_tokenizer, Tokenizer};

#[no_mangle]
pub extern "C" fn hello_to_my_name(name: *const c_char) -> *mut c_char {
    let name: &str = unsafe { CStr::from_ptr(name).to_str().unwrap() };
    let result: String = format!("Hello, {}!", name);
    let result: CString = CString::new(result).unwrap();
    result.into_raw()
}

pub fn get_token_bpe(t: Tokenizer) -> Result<Arc<Mutex<CoreBPE>>> {
    let token_bpe: Arc<Mutex<CoreBPE>> = match t {
        Tokenizer::Cl100kBase => tiktoken_rs::cl100k_base_singleton(),
        Tokenizer::P50kBase => tiktoken_rs::p50k_base_singleton(),
        Tokenizer::R50kBase => tiktoken_rs::r50k_base_singleton(),
        Tokenizer::P50kEdit => tiktoken_rs::p50k_edit_singleton(),
        Tokenizer::Gpt2 => tiktoken_rs::r50k_base_singleton()
    };
    Ok(token_bpe)
}

#[no_mangle]
pub extern "C" fn get_qtd_tokens(model_name: *const libc::c_char, txt: *const libc::c_char) -> libc::c_uint {
    let model_name: &str = unsafe { CStr::from_ptr(model_name).to_str().unwrap() };
    let txt: &str = unsafe { CStr::from_ptr(txt).to_str().unwrap() };
    let tokenizer = get_tokenizer(model_name).ok_or_else(|| anyhow!("Model {} doesn't exists", model_name));
    if tokenizer.is_err() {
        return 0 as libc::c_uint;
    }
    let bpe = get_token_bpe(tokenizer.unwrap()).unwrap();
    let result = bpe.lock().encode_with_special_tokens(txt).len();
    result as libc::c_uint
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn test_hello_to_my_name() {
        let name = CString::new("Rust").unwrap();
        let raw_result = { hello_to_my_name(name.as_ptr()) };
        let result_str = unsafe { CString::from_raw(raw_result) }.to_str().unwrap().to_owned();
        assert_eq!(result_str, "Hello, Rust!");
    }

    #[test]
    fn test_get_qtd_token() {
        let model_name: CString = CString::new("gpt-3.5-turbo").unwrap();
        let txt: CString = CString::new("Hello my world!").unwrap();
        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 4);
    }

    #[test]
    fn test_get_qtd_token_nothing() {
        let model_name: CString = CString::new("nothing").unwrap();
        let txt: CString = CString::new("Hello my world!").unwrap();
        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 0);
    }
}