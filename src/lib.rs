mod core_docx;
mod api;
mod password;

// mod tools;
use docx_rs::*;
use crate::api::MultiAuth;
use std::ffi::CString;
use std::io::{Cursor, Read};
use base64::engine::general_purpose;
use base64::Engine as _;
use docx_rs::{Docx, Paragraph, Pic, Run};
use serde_json::Value;
use log::error;

#[no_mangle]
pub extern "C" fn theme(password: *const libc::c_char, png: *const libc::c_char, nom: *const libc::c_char, date: *const libc::c_char) -> *const libc::c_char {
    use std::ffi::{CStr, CString};
    unsafe {
        let name = CStr::from_ptr(nom).to_str().unwrap_or("invalid");
        let date = CStr::from_ptr(date).to_str().unwrap_or("invalid");
        println!("Rust theme called! name={} date={}", name, date);
    }

    // Convertir C string en Rust string
    let c_str = unsafe { CStr::from_ptr(password) };
    let password_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let c_str = unsafe { CStr::from_ptr(png) };
    let png_str = match c_str.to_str() {
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
    let mut json_payload: Value = serde_json::json!({
        "token_n": "",
        "token_t": "",
        "docx_base64": ""
    });
    // Nouvelle implémentation: gérer l'async, décoder PNG, réencoder, générer docx, récupérer tokens, logs/erreurs
    use log::{info, error};
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
        info!("Token N: {:?}", token_n);
        info!("Token T: {:?}", token_t);

        // Décodage Base64 PNG
        let img_bytes: Vec<u8> = match general_purpose::STANDARD.decode(png_str) {
            Ok(bytes) => {
                if bytes.len() < 8 {
                    error!("Erreur: Base64 PNG décodé trop court ({} octets)", bytes.len());
                    return Err("PNG décodé trop court".to_string());
                }
                info!("Premiers 8 octets PNG: {:?}", &bytes[0..8]);
                bytes
            },
            Err(e) => {
                error!("Erreur de décodage Base64 PNG: {}", e);
                return Err(format!("Erreur de décodage Base64 PNG: {}", e));
            }
        };
        // Charger l'image
        let img = match image::load_from_memory(&img_bytes.as_slice()) {
            Ok(im) => im,
            Err(e) => {
                error!("Impossible de charger l'image en mémoire: {}", e);
                return Err(format!("Impossible de charger l'image: {}", e));
            }
        };
        // Réencoder en PNG standard
        let mut png_buffer = Vec::new();
        if let Err(e) = img.write_to(&mut std::io::Cursor::new(&mut png_buffer), image::ImageOutputFormat::Png) {
            error!("Impossible de réencoder PNG: {}", e);
            return Err(format!("Impossible de réencoder PNG: {}", e));
        }
        // Créer le Pic à partir du buffer réencodé
        let width = ((720 as f64) * 192.0 * 38.8).round() as u32;
        let height = ((397 as f64) * 192.0 * 38.8).round() as u32;
        let pic = Pic::new(&png_buffer).size(width, height);

        // Créer un buffer avec Cursor
        let mut buffer = Cursor::new(Vec::new());
        let docx_res = Docx::new()
            .add_table(core_docx::titre_1("Numérologie").unwrap())
            .add_paragraph(Paragraph::new().
                add_run(Run::new()
                    .add_text("")))
            .add_table(core_docx::titre_2("Thème").unwrap())
            .add_table(core_docx::theme_2(pic, nom_str, date_str).unwrap())
            .add_paragraph(Paragraph::new().
                add_run(Run::new()
                    .add_text("")))
            .add_table(core_docx::titre_2("Meilleur moyen pour se connecter à son intuition").unwrap())
            .add_table(core_docx::content_2("Le meilleur moyen...").unwrap())
            .build()
            .pack(&mut buffer);
        if let Err(e) = docx_res {
            error!("Erreur lors de la génération du docx: {}", e);
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
            eprintln!("Erreur durant l'exécution async: {}", msg);
            CString::new(format!("{{\"error\":\"{}\"}}", msg)).unwrap()
        }
    };
    json_cstring.into_raw()
}