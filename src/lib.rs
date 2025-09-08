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

#[no_mangle]
pub extern "C" fn theme(password: *const libc::c_char, png: *const libc::c_char, nom: *const libc::c_char, date: *const libc::c_char) -> *const libc::c_char {
    use std::ffi::{CStr, CString};

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
    let mut json_payload;
    // bloquer l'async avec block_on
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let auth = MultiAuth::new(password_str.to_string()).await;
            let (token_n, token_t) = auth.get_token();

            println!("Token N: {:?}", token_n);
            println!("Token T: {:?}", token_t);

            let img_bytes: Vec<u8> = general_purpose::STANDARD
                .decode(png_str)
                .expect("Base64 invalide");

            // Créer le Pic
            let width = ((720 as f64) * 192.0 * 38.8).round() as u32;
            let height = ((397 as f64) * 192.0 * 38.8).round() as u32;
            let pic = Pic::new(&img_bytes).size(width, height);

            // Créer un buffer avec Cursor
            let mut buffer = Cursor::new(Vec::new());

            Docx::new()
                .add_table(core_docx::titre_1("Numérologie")?)
                .add_paragraph(Paragraph::new().
                    add_run(Run::new()
                        .add_text("")))
                .add_table(core_docx::titre_2("Thème")?)
                .add_table(core_docx::theme_2(pic, "Stéphane Bressani", "03.04.1986")?)
                .add_paragraph(Paragraph::new().
                    add_run(Run::new()
                        .add_text("")))
                .add_table(core_docx::titre_2("Meilleur moyen pour se connecter à son intuition")?)
                .add_table(core_docx::content_2("Le meilleur moyen...")?)
                .build()
                .pack(&mut buffer).expect("Panic TODO");

            let b64 = general_purpose::STANDARD.encode(buffer.get_ref());
            json_payload = serde_json::json!({
                "token_n": token_n,
                "token_t": token_t,
                "docx_base64": b64
            });
        });
    CString::new(json_payload.to_string()).unwrap().into_raw()
}