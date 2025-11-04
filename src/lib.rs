extern crate core;

mod core_docx;
pub mod api;
pub mod password;
pub mod html_tools;
mod prepare_docx;
pub mod prepare_selection;

// mod tools;
use docx_rs::*;
pub use crate::api::{MultiAuth, TNumerologieClient};
use std::ffi::{CStr, CString};
use std::io::{Cursor, Read};
use base64::engine::general_purpose;
use base64::Engine as _;
use docx_rs::{Docx, Paragraph, Pic, Run};
use serde_json::Value;
use log::error;
use std::fs;
use std::path::PathBuf;
use crate::api::TraitementNumerologie;
use crate::prepare_docx::prepare_docx;
use crate::prepare_selection::prepare_selection;

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

    json_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn selection_traitment(password: *const libc::c_char, type_traitement: libc::c_int, id: libc::c_int, carte: libc::c_int) -> *const libc::c_char {
    use std::ffi::CStr;
    println!("Selection");
    // Conversion
    let c_str = unsafe { CStr::from_ptr(password) };
    let password_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let type_traitement_u32 = type_traitement as u32;
    let id_u32 = id as u32;
    let carte_u32 = carte as u32;

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur lors de la création du runtime Tokio: {}", e);
            return CString::new("{\"error\":\"tokio runtime\"}").unwrap().into_raw();
        }
    };

    let result: Result<String, Box<dyn std::error::Error>> = rt.block_on(async {
        // MultiAuth sécurisé
        let auth = MultiAuth::new(password_str.to_string()).await;
        let (token_n, token_t) = auth.get_token();
        let traitement: TraitementNumerologie = match type_traitement_u32 {
            1 => TraitementNumerologie::Ppr,
            4 => TraitementNumerologie::Cai,
            2 => TraitementNumerologie::Cae,
            5 => TraitementNumerologie::Coi,
            3 => TraitementNumerologie::Coe,
            6 => TraitementNumerologie::Int,
            7 => TraitementNumerologie::Nem,
            8 => TraitementNumerologie::Pex,
            9 => TraitementNumerologie::Rha,
            _ => {
                TraitementNumerologie::Ppr
            }
        };
        let c = if carte_u32 == 0 {
            None
        } else {
            Some(carte_u32)
        };
        let json: String = prepare_selection(token_n, token_t, id_u32, traitement, c).await?;

        Ok(json)
    });

    let json_cstring = match result {
        Ok(json) => CString::new(json.to_string()).unwrap(),
        Err(msg) => {
            eprintln!("Erreur durant l'exécution async : {}", msg);
            CString::new(format!("{{\"error\":\"{}\"}}", msg)).unwrap()
        }
    };

    json_cstring.into_raw()
}

#[no_mangle]
pub extern "C" fn free_cstring(ptr: *mut libc::c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reprend la propriété de la mémoire et la libère automatiquement
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn cyclesPng(j: libc::c_int,
                            m: libc::c_int,
                            a: libc::c_int,
                            age: libc::c_int,
                            path_cycle: *const libc::c_char, ) -> *const libc::c_char {
    let c_str = unsafe { CStr::from_ptr(path_cycle) };
    let path_cycle_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result = cycles_numerologie_du_tarot::generate(j as usize, m as usize, a as usize, age as usize, path_cycle_str.to_string());
    let json_cstring = match result {
        Ok(png_bytes) => {
            let b64 = general_purpose::STANDARD.encode(&png_bytes);
            let json = format!("{{\"png\":\"{}\"}}", b64);
            CString::new(json).unwrap()
        },
        Err(msg) => {
            eprintln!("Erreur durant l'exécution cyclesPng : {}", msg);
            CString::new(format!("{{\"error\":\"{}\"}}", msg)).unwrap()
        }
    };

    json_cstring.into_raw()
}
