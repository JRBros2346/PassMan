use serde::{Deserialize, Serialize};
use surrealdb::*;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;

#[derive(Serialize, Deserialize)]
struct PassTable {
    user_sj: String,
    pass_sj: String,
    website: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    id: String,
}

pub async fn config() -> surrealdb::Result<()> {
    // Connect to the SurrealDB server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    
    // Sign in with the root user
    db.signin(Root {
        username: &dotenv::var("USERNAME").expect("No User"),
        password: &dotenv::var("PASSWORD").expect("No User"),
    }).await?;
    
    // Define namespace and database
    db.query("DEFINE NAMESPACE pass_ns").await?;
    db.use_ns("pass_ns").await?;
    
    db.query("DEFINE DATABASE pass_db").await?;
    db.use_db("pass_db").await?;
    
    // Define table and fields
    db.query("DEFINE TABLE pass_table").await?;
    db.query("DEFINE FIELD user_sj ON pass_table TYPE string").await?;
    db.query("DEFINE FIELD pass_sj ON pass_table TYPE string").await?;
    db.query("DEFINE FIELD website ON pass_table TYPE string").await?;
    
    // Create a record in the table
    let _created: Vec<Record> = db.create("pass_table")
        .content(PassTable {
            user_sj: "Demo".to_string(),
            pass_sj: "Demo".to_string(),
            website: "www.google.com".to_string(),
        })
        .await?;
    
    println!("Record created successfully");
    
    Ok(())
}
