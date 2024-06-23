use std::io::{self, Write};

use crate::{
    auth::password::hash_password,
    util::{base64_encode, valid_username_format},
    Result,
};

pub fn generate_admin_hash() -> Result<()> {
    let mut username = String::new();

    print!("Enter admin username: ");
    io::stdout().flush().expect("Failed to write to stdout");

    let Ok(_) = io::stdin().read_line(&mut username) else {
        return Err("Failed to read username".into());
    };
    username = username.trim().to_string();

    if !valid_username_format(&username) {
        return Err("Invalid username format".into());
    }

    let Ok(password) = rpassword::prompt_password("Enter admin password: ") else {
        return Err("Failed to read password".into());
    };

    let password = password.trim().to_string();
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }

    let combined = format!("{}:{}", username, password);
    let admin_hash = hash_password(&combined)?;
    let admin_hash = base64_encode(&admin_hash);

    println!("Admin Hash:");
    println!("{}", admin_hash);
    println!("Set this value as the ADMIN_HASH environment variable.");

    Ok(())
}
