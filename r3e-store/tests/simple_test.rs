use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    created_at: u64,
}

#[test]
fn test_rocksdb_simple() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("RocksDB Simple Test");

    // Create RocksDB options
    let mut options = Options::default();
    options.create_if_missing(true);
    options.create_missing_column_families(true);

    // Define column families
    let cf_names = vec!["users", "metadata"];

    // Create column family descriptors
    let cf_descriptors: Vec<ColumnFamilyDescriptor> = cf_names
        .iter()
        .map(|name| ColumnFamilyDescriptor::new(*name, Options::default()))
        .collect();

    // Open DB with column families
    let db_path = "r3e-store/tests/data/rocksdb_simple_test";

    // Open DB with column families
    let db = DB::open_cf_descriptors(&options, db_path, cf_descriptors)?;

    // Create a user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    // Serialize user
    let user_bytes = bincode::serialize(&user)?;

    // Get column family handle
    let cf_users = db.cf_handle("users").unwrap();

    // Store user in DB
    db.put_cf(cf_users, user.id.as_bytes(), user_bytes)?;

    // Retrieve user from DB
    let retrieved_bytes = db.get_cf(cf_users, user.id.as_bytes())?;

    if let Some(bytes) = retrieved_bytes {
        let retrieved_user: User = bincode::deserialize(&bytes)?;
        println!("Retrieved user: {:?}", retrieved_user);
        assert_eq!(user.username, retrieved_user.username);
        assert_eq!(user.email, retrieved_user.email);
    } else {
        panic!("User not found in database");
    }

    // Clean up by dropping the DB instance
    drop(db);

    println!("RocksDB test completed successfully");
    Ok(())
}
