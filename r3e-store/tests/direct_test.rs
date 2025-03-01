use rocksdb::{DB, ColumnFamilyDescriptor, Options};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    id: String,
    username: String,
    email: String,
    created_at: u64,
}

#[test]
fn test_rocksdb_direct() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    let _ = env_logger::builder().is_test(true).try_init();
    
    println!("RocksDB Direct Test - Using RocksDB directly without r3e-store wrapper");
    
    // Setup DB path
    let db_path = Path::new("./testdb_direct");
    if db_path.exists() {
        fs::remove_dir_all(db_path)?;
    }
    
    // Define column families
    let cf_users = "users";
    let cf_metadata = "metadata";
    
    // Create RocksDB options
    let mut options = Options::default();
    options.create_if_missing(true);
    options.create_missing_column_families(true);
    
    // Create column family descriptors
    let cf_descriptors = vec![
        ColumnFamilyDescriptor::new(cf_users, Options::default()),
        ColumnFamilyDescriptor::new(cf_metadata, Options::default()),
    ];
    
    // Open DB with column families
    let db = DB::open_cf_descriptors(&options, db_path, cf_descriptors)?;
    
    // Create a user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "directuser".to_string(),
        email: "direct@example.com".to_string(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    // Serialize user (using bincode for direct comparison with simple_test)
    let user_bytes = bincode::serialize(&user)?;
    
    // Get column family handle
    let cf_users_handle = db.cf_handle(cf_users).unwrap();
    
    // Store user in DB
    println!("Storing user with ID: {}", user.id);
    db.put_cf(cf_users_handle, user.id.as_bytes(), user_bytes)?;
    
    // Store metadata
    let cf_metadata_handle = db.cf_handle(cf_metadata).unwrap();
    db.put_cf(cf_metadata_handle, "last_user_id", user.id.as_bytes())?;
    
    // Retrieve user from DB
    let retrieved_bytes = db.get_cf(cf_users_handle, user.id.as_bytes())?;
    
    if let Some(bytes) = retrieved_bytes {
        let retrieved_user: User = bincode::deserialize(&bytes)?;
        println!("Retrieved user: {:?}", retrieved_user);
        assert_eq!(user.id, retrieved_user.id);
        assert_eq!(user.username, retrieved_user.username);
        assert_eq!(user.email, retrieved_user.email);
    } else {
        panic!("User not found in database");
    }
    
    // Test existence
    let exists = db.get_cf(cf_users_handle, user.id.as_bytes())?.is_some();
    assert!(exists, "User should exist in the database");
    
    // Test delete
    println!("Deleting user with ID: {}", user.id);
    db.delete_cf(cf_users_handle, user.id.as_bytes())?;
    
    // Verify deletion
    let should_be_none = db.get_cf(cf_users_handle, user.id.as_bytes())?;
    assert!(should_be_none.is_none(), "User should be deleted from database");
    
    // Retrieve metadata
    let metadata_bytes = db.get_cf(cf_metadata_handle, "last_user_id")?.unwrap();
    let metadata_value = std::str::from_utf8(&metadata_bytes)?;
    assert_eq!(user.id, metadata_value);
    
    // Clean up
    drop(db);
    fs::remove_dir_all(db_path)?;
    
    println!("RocksDB direct test completed successfully");
    Ok(())
} 