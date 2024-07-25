use crate::clients::{delete_client, get_client, list_clients, update_client_status};
use crate::config::ClientCommand;

use crate::db::create_db_pool;
use crate::Result;

use super::{create_client, NewClient};

pub async fn run_client_command(cmd: ClientCommand) -> Result<()> {
    match cmd {
        ClientCommand::List => run_list_clients().await,
        ClientCommand::Create { name } => run_create_client(name).await,
        ClientCommand::Enable { id } => run_enable_client(id).await,
        ClientCommand::Disable { id } => run_disable_client(id).await,
        ClientCommand::Delete { id } => run_delete_client(id).await,
    }
}

async fn run_list_clients() -> Result<()> {
    let db_pool = create_db_pool();
    let clients = list_clients(&db_pool).await?;
    for client in clients.iter() {
        println!(
            "{{ id = {}, name = {}, status = {}, default_bucket = {} }}",
            client.id, client.name, client.status, "None"
        );
    }
    Ok(())
}

async fn run_create_client(name: String) -> Result<()> {
    let db_pool = create_db_pool();
    let new_client = NewClient { name };
    let client = create_client(&db_pool, &new_client).await?;
    println!("{{ id = {}, name = {} }}", client.id, client.name);
    println!("Created client.");
    Ok(())
}

async fn run_enable_client(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let client = get_client(&db_pool, &id).await?;
    if let Some(node) = client {
        if &node.status == "active" {
            println!("Client already enabled.");
            return Ok(());
        }

        let _ = update_client_status(&db_pool, &id, "active").await?;
        println!("Client enabled.");
    } else {
        println!("Client not found.");
    }
    Ok(())
}

async fn run_disable_client(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let client = get_client(&db_pool, &id).await?;
    if let Some(node) = client {
        if &node.status == "inactive" {
            println!("Client already disabled.");
            return Ok(());
        }

        let _ = update_client_status(&db_pool, &id, "inactive").await?;
        println!("Client disabled.");
    } else {
        println!("Client not found.");
    }
    Ok(())
}

async fn run_delete_client(id: String) -> Result<()> {
    let db_pool = create_db_pool();
    let client = get_client(&db_pool, &id).await?;
    if let Some(_) = client {
        let _ = delete_client(&db_pool, &id).await?;
        println!("Client deleted.");
    } else {
        println!("Client not found.");
    }
    Ok(())
}
