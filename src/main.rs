use rocksdb::{DB, ColumnFamilyDescriptor, Options};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    created_at: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    println!("RocksDB Simple Test");
    
    // Create RocksDB options
    let mut options = Options::default();
    options.create_if_missing(true);
    options.create_missing_column_families(true);
    
    // Define column families
    let cf_names = vec!["users", "metadata"];
    let cf_descriptors: Vec<_> = cf_names.iter()
        .map(|name| {
            let mut cf_opts = Options::default();
            cf_opts.set_max_write_buffer_number(16);
            ColumnFamilyDescriptor::new(*name, cf_opts)
        })
        .collect();
    
    // Open database with column families
    let db_path = "./data/rocksdb_simple_test";
    let db = DB::open_cf_descriptors(&options, db_path, cf_descriptors)?;
    
    // Get column family handles
    let users_cf = db.cf_handle("users").expect("users CF not found");
    let metadata_cf = db.cf_handle("metadata").expect("metadata CF not found");
    
    // Create a user
    let user_id = Uuid::new_v4().to_string();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis() as u64;
        
    let user = User {
        id: user_id.clone(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: now,
    };
    
    // Serialize the user
    let user_bytes = bincode::serialize(&user)?;
    
    // Store the user
    println!("Storing user with ID: {}", user_id);
    db.put_cf(&users_cf, user_id.as_bytes(), &user_bytes)?;
    
    // Store some metadata
    db.put_cf(&metadata_cf, b"last_user_id", user_id.as_bytes())?;
    db.put_cf(&metadata_cf, b"user_count", b"1")?;
    
    // Retrieve the user
    let retrieved_bytes = db.get_cf(&users_cf, user_id.as_bytes())?;
    
    if let Some(bytes) = retrieved_bytes {
        let retrieved_user: User = bincode::deserialize(&bytes)?;
        println!("Retrieved user: {:?}", retrieved_user);
    } else {
        println!("User not found");
    }
    
    // Retrieve metadata
    if let Some(last_id) = db.get_cf(&metadata_cf, b"last_user_id")? {
        println!("Last user ID: {}", String::from_utf8(last_id)?);
    }
    
    if let Some(count) = db.get_cf(&metadata_cf, b"user_count")? {
        println!("User count: {}", String::from_utf8(count)?);
    }
    
    // Delete the user
    println!("Deleting user");
    db.delete_cf(&users_cf, user_id.as_bytes())?;
    
    // Verify deletion
    let deleted_user = db.get_cf(&users_cf, user_id.as_bytes())?;
    if deleted_user.is_none() {
        println!("User successfully deleted");
    } else {
        println!("User still exists!");
    }
    
    // Cleanup
    drop(db);
    
    println!("RocksDB test completed successfully");
    Ok(())
} 