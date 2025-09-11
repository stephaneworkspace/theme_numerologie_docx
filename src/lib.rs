extern crate core;

mod core_docx;
mod api;
mod password;
pub mod html_tools;
mod prepare_docx;

// mod tools;
use docx_rs::*;
use crate::api::{MultiAuth, TNumerologieClient};
use std::ffi::CString;
use std::io::{Cursor, Read};
use base64::engine::general_purpose;
use base64::Engine as _;
use docx_rs::{Docx, Paragraph, Pic, Run};
use serde_json::Value;
use log::error;
use std::fs;
use std::path::PathBuf;
use crate::prepare_docx::prepare_docx;

#[no_mangle]
pub extern "C" fn theme(password: *const libc::c_char, path_cartes: *const libc::c_char, nom: *const libc::c_char, date: *const libc::c_char, id: libc::c_int) -> *const libc::c_char {
    use std::ffi::CStr;
    unsafe {
        let name = CStr::from_ptr(nom).to_str().unwrap_or("invalid");
        let date = CStr::from_ptr(date).to_str().unwrap_or("invalid");
        println!("Rust theme called! name={} date={}", name, date);
    }

    // Conversion
    let c_str = unsafe { CStr::from_ptr(password) };
    let password_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let c_str = unsafe { CStr::from_ptr(path_cartes) };
    let path_cartes_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let c_str = unsafe { CStr::from_ptr(nom) };
    let nom_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let c_str = unsafe { CStr::from_ptr(date) };
    let date_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let id_u32 = id as u32;

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur lors de la création du runtime Tokio: {}", e);
            return CString::new("{\"error\":\"tokio runtime\"}").unwrap().into_raw();
        }
    };

    let result = rt.block_on(async {
        // MultiAuth sécurisé
        let auth = MultiAuth::new(password_str.to_string()).await;
        let (token_n, token_t) = auth.get_token();

        // Créer un buffer avec Cursor
        let mut buffer = Cursor::new(Vec::new());
        let mut docx_res: XMLDocx = prepare_docx(token_n.clone(), token_t.clone(), id_u32, path_cartes_str.to_string()).await.unwrap();
        let pack = docx_res.pack(&mut buffer);

        if let Err(e) = pack {
            error!("Erreur lors de la génération du docx : {}", e);
            return Err(format!("Erreur docx: {}", e));
        }

        let b64 = general_purpose::STANDARD.encode(buffer.get_ref());

        Ok(serde_json::json!({
            "token_n": token_n,
            "token_t": token_t,
            "docx_base64": b64
        }))
    });

    let json_cstring = match result {
        Ok(json) => CString::new(json.to_string()).unwrap(),
        Err(msg) => {
            eprintln!("Erreur durant l'exécution async : {}", msg);
            CString::new(format!("{{\"error\":\"{}\"}}", msg)).unwrap()
        }
    };
    eprintln!("Json : {:?}", json_cstring.clone().into_raw());

    json_cstring.into_raw()
}