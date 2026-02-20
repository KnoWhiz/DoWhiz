#![allow(dead_code)]

pub fn require_supabase_db_url(test_name: &str) -> Option<String> {
    dotenvy::dotenv().ok();
    match std::env::var("SUPABASE_DB_URL") {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ => {
            eprintln!("Skipping {test_name}; SUPABASE_DB_URL not set.");
            None
        }
    }
}

pub fn start_mockito_server(test_name: &str) -> Option<mockito::ServerGuard> {
    let server = std::panic::catch_unwind(|| mockito::Server::new());
    match server {
        Ok(server) => Some(server),
        Err(_) => {
            eprintln!(
                "Skipping {test_name}; unable to start mockito server in this environment."
            );
            None
        }
    }
}
