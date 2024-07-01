use crate::config::UserCommand;
use crate::db::create_db_pool;
use crate::users::queries::{delete_user, list_users, update_user_password, update_user_status};
use crate::Result;

use super::queries::{create_user, get_user};
use super::NewUser;

pub async fn run_user_command(cmd: UserCommand) -> Result<()> {
    match cmd {
        UserCommand::List { client_id } => run_list_users(client_id).await,
        UserCommand::Create {
            client_id,
            username,
            roles,
        } => run_create_user(client_id, username, roles).await,
        UserCommand::Password { id } => run_set_user_password(id).await,
        UserCommand::Disable { id } => run_disable_user(id).await,
        UserCommand::Enable { id } => run_enable_user(id).await,
        UserCommand::Delete { id } => run_delete_user(id).await,
    }
}

async fn run_list_users(client_id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let users = list_users(&db_pool, &client_id).await?;
    for user in users.iter() {
        println!(
            "ID: {}, Username: {}, Status: {}",
            user.id, user.username, user.status
        );
    }
    Ok(())
}

async fn run_create_user(client_id: String, username: String, roles: String) -> Result<()> {
    let Ok(password) = rpassword::prompt_password("Enter password for the new user: ") else {
        return Err("Failed to read password".into());
    };

    let password = password.trim().to_string();
    let new_user = NewUser {
        username,
        password,
        roles,
    };

    let db_pool = create_db_pool();
    let user = create_user(&db_pool, &client_id, &new_user).await?;
    println!("ID: {}, Username: {}", user.id, user.username);
    println!("Created user.");
    Ok(())
}

async fn run_set_user_password(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let user = get_user(&db_pool, &id).await?;
    if let Some(node) = user {
        let prompt = format!("Enter new password for {}: ", node.username);
        let Ok(password) = rpassword::prompt_password(prompt) else {
            return Err("Failed to read password".into());
        };
        let password = password.trim().to_string();
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".into());
        }
        if password.len() > 100 {
            return Err("Password must be at most 100 characters".into());
        }
        let _ = update_user_password(&db_pool, &id, &password).await?;
        println!("Password updated.");
    } else {
        println!("User not found.");
    }
    Ok(())
}

async fn run_disable_user(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let user = get_user(&db_pool, &id).await?;
    if let Some(node) = user {
        if &node.status == "inactive" {
            println!("User already disabled.");
            return Ok(());
        }
        let _ = update_user_status(&db_pool, &id, "inactive").await?;
        println!("User disabled.");
    } else {
        println!("User not found.");
    }
    Ok(())
}

async fn run_enable_user(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let user = get_user(&db_pool, &id).await?;
    if let Some(node) = user {
        if &node.status == "inactive" {
            println!("User already disabled.");
            return Ok(());
        }
        let _ = update_user_status(&db_pool, &id, "inactive").await?;
        println!("User disabled.");
    } else {
        println!("User not found.");
    }
    Ok(())
}

async fn run_delete_user(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let user = get_user(&db_pool, &id).await?;
    if let Some(_) = user {
        let _ = delete_user(&db_pool, &id).await?;
        println!("User deleted.");
    } else {
        println!("User not found.");
    }
    Ok(())
}
