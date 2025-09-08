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
use std::fs;
use std::path::PathBuf;

#[no_mangle]
pub extern "C" fn theme(password: *const libc::c_char, png: *const libc::c_char, nom: *const libc::c_char, date: *const libc::c_char) -> *const libc::c_char {
    use std::ffi::CStr;
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

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur lors de la création du runtime Tokio: {}", e);
            return CString::new("{\"error\":\"tokio runtime\"}").unwrap().into_raw();
        }
    };

    let mut b64 = String::new();

    let result = rt.block_on(async {
        // MultiAuth sécurisé
        let auth = MultiAuth::new(password_str.to_string()).await;
        let (token_n, token_t) = auth.get_token();

        // La partie PNG est temporairement désactivée
        /*
        // Décodage Base64 PNG
        let img_bytes: Vec<u8> = match general_purpose::STANDARD.decode(png_str) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Erreur de décodage Base64 PNG: {}", e);
                return Err(format!("Erreur de décodage Base64 PNG: {}", e));
            }
        };

        // Écrire PNG dans un fichier temporaire dans tmp/








        // FETCH DEPUIS RAILS ET BASE64 CORRECT DEPUIS RAILS








        let tmp_dir = PathBuf::from("tmp");
        if let Err(e) = fs::create_dir_all(&tmp_dir) {
            error!("Erreur création dossier tmp: {}", e);
            return Err(format!("Erreur création dossier tmp: {}", e));
        }
        let tmp_file_path = tmp_dir.join("temp_image.png");
        if let Err(e) = fs::write(&tmp_file_path, &img_bytes) {
            error!("Erreur écriture fichier PNG temporaire: {}", e);
            return Err(format!("Erreur écriture fichier PNG temporaire: {}", e));
        }

        // Lire le fichier temporaire PNG en mémoire et créer Pic depuis les bytes
        let pic_bytes = match fs::read(&tmp_file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Erreur lecture fichier PNG temporaire: {}", e);
                return Err(format!("Erreur lecture fichier PNG temporaire: {}", e));
            }
        };
        let pic = Pic::new(&pic_bytes)
            .size(720 * 192 * 38, 397 * 192 * 38);
        */

        // Créer un buffer avec Cursor
        let mut buffer = Cursor::new(Vec::new());
        let docx_res = Docx::new()
            .add_table(core_docx::titre_1("Numérologie").unwrap())
            .add_paragraph(Paragraph::new().
                add_run(Run::new()
                    .add_text("")))
            .add_table(core_docx::titre_2("Thème").unwrap())
            // La partie PNG est désactivée, donc on ne passe pas de pic ici
            //.add_table(core_docx::theme_2(pic, nom_str, date_str).unwrap())
            //.add_table(core_docx::theme_2(/*pic*/ /* Temporarily disabled */, nom_str, date_str).unwrap())
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

        b64 = general_purpose::STANDARD.encode(buffer.get_ref());

        Ok(serde_json::json!({
            "token_n": token_n,
            "token_t": token_t,
            "docx_base64": "b64"
        }))
    });

    let json_cstring = match result {
        Ok(json) => CString::new(json.to_string()).unwrap(),
        Err(msg) => {
            eprintln!("Erreur durant l'exécution async: {}", msg);
            CString::new(format!("{{\"error\":\"{}\"}}", msg)).unwrap()
        }
    };
    //json_cstring.into_raw()
    CString::new(b64).unwrap().into_raw()
}