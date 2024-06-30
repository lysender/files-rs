use crate::config::{Config, UserCommand};
use crate::db::create_db_pool;
use crate::users::queries::list_users;
use crate::Result;

use super::queries::create_user;
use super::NewUser;

pub async fn run_user_command(config: Config, cmd: UserCommand) -> Result<()> {
    match cmd {
        UserCommand::List { client_id } => run_list_users(client_id).await,
        UserCommand::Create {
            client_id,
            username,
            roles,
        } => run_create_user(client_id, username, roles).await,
        UserCommand::Password { id } => run_set_user_password(config, id).await,
        UserCommand::Disable { id } => run_disable_user(config, id).await,
        UserCommand::Enable { id } => run_enable_user(config, id).await,
        UserCommand::Delete { id } => run_delete_user(config, id).await,
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

async fn run_set_user_password(config: Config, id: String) -> Result<()> {
    Ok(())
}

async fn run_disable_user(config: Config, id: String) -> Result<()> {
    Ok(())
}

async fn run_enable_user(config: Config, id: String) -> Result<()> {
    Ok(())
}

async fn run_delete_user(config: Config, id: String) -> Result<()> {
    Ok(())
}
