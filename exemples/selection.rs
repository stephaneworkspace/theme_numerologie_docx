use std::env;
use theme_numerologie_docx::api::TraitementNumerologie;
use theme_numerologie_docx::MultiAuth;
use theme_numerologie_docx::password;
use theme_numerologie_docx::prepare_selection::prepare_selection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupère le 1er argument après le nom du binaire
    let id: u32 = env::args()
        .nth(1)
        .as_deref()
        .unwrap_or("1") // valeur par défaut si pas d'argument
        .parse()?;       // échoue si ce n'est pas un u32
    println!("Paramètre u32 reçu: {id}");
    let password = password::load_password("Secrets.yaml");
    let auth = MultiAuth::new(password.unwrap()).await;
    let (token_n, token_t) = auth.get_token();

    println!("Token N: {:?}", token_n);
    println!("Token T: {:?}", token_t);
    let traitement: TraitementNumerologie = TraitementNumerologie::Cai;
    prepare_selection(token_n, token_t, id, traitement).await?;
    Ok(())
}

