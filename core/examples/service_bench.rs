use charac::db::{DB, init};
use charac::models::field::kinds::*;
use charac::models::record::cell::*;
use charac::models::*;
use charac::service::approved;
use charac::service::base::BaseService;
use charac::service::crypter::{decrypt_token, encrypt_token};
use charac::service::table::{PaginationParams, TableService};
use charac::service::user::{AuthMethod, Session, UserService};
use std::collections::HashMap;
use std::time::Instant;

macro_rules! bench_async {
    ($name:expr, $iterations:expr, $block:expr) => {
        let start = Instant::now();
        for _ in 0..$iterations {
            $block.await.expect("Benchmark failed");
        }
        let duration = start.elapsed();
        println!(
            "Benchmark {:<30} | Iterations: {:<6} | Total: {:<12?} | Avg: {:?}",
            $name,
            $iterations,
            duration,
            duration / $iterations
        );
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    init().await;

    println!("--- Testing Crypter Service ---");
    test_crypter().await?;

    println!("\n--- Testing User Service ---");
    let (user_service, user_id) = test_user_service().await?;

    println!("\n--- Testing Base Service ---");
    let (base_service, base_id) = test_base_service(&user_service, &user_id).await?;

    println!("\n--- Testing Table Service ---");
    test_table_service(&base_service, &user_id, &base_id).await?;

    println!("\n--- Testing Miscellaneous ---");
    test_misc().await?;

    println!("\n--- Benchmarking ---");
    run_benchmarks().await?;

    Ok(())
}

async fn test_crypter() -> Result<(), Box<dyn std::error::Error>> {
    let token = "my-secret-token";
    let encrypted = encrypt_token(token).await?;
    let decrypted = decrypt_token(encrypted).await?;
    assert_eq!(token, decrypted);
    println!("Crypter: Encrypt/Decrypt successful");
    Ok(())
}

async fn test_user_service() -> Result<(UserService, UserId), Box<dyn std::error::Error>> {
    // Manually create a user for testing
    let user: User = DB
        .query("CREATE user SET first_name = 'Test', last_name = 'User', email = 'test@example.com', role = 'admin' RETURN AFTER")
        .await?
        .take::<Option<User>>(0)?
        .unwrap();
    let user_id = UserId(user.id.as_ref().unwrap().0.clone());

    // Test UserService::login with session (bypass HCAUTH for now)
    let token = "test-session-token";
    let ip = "127.0.0.1".to_string();
    let agent = "TestAgent".to_string();

    DB.query("CREATE session SET user = $user, token = $tokenn, ip = $ip, user_agent = $agent, expires_at = time::now() + 1d")
        .bind(("user", user_id.clone()))
        .bind(("tokenn", token))
        .bind(("ip", ip.clone()))
        .bind(("agent", agent.clone()))
        .await?;

    let user_service = UserService::login(AuthMethod::Session(Session {
        token: token.to_string(),
        ip,
        agent,
    }))
    .await?;

    assert_eq!(user_service.user.email, "test@example.com");
    println!("UserService: Login successful");

    // Test is_admin
    assert!(user_service.is_admin().await?);
    println!("UserService: is_admin successful");

    // Test create_base
    let base = user_service.create_base("TestBase".to_string()).await?;
    assert_eq!(base.name, "TestBase");
    println!("UserService: create_base successful");

    Ok((user_service, user_id))
}

async fn test_base_service(
    user_service: &UserService,
    user_id: &UserId,
) -> Result<(BaseService, BaseId), Box<dyn std::error::Error>> {
    let bases = user_service.list_bases().await?;
    let base_id = BaseId(bases[0].id.as_ref().unwrap().0.clone());

    // Test BaseService::new
    let base_service = BaseService::new(base_id.clone(), user_id.clone()).await?;
    assert_eq!(base_service.id().0, base_id.0);
    println!("BaseService: new successful");

    // Test create_table
    let table = base_service.create_table("test_table".to_string()).await?;
    assert_eq!(table.name, "test_table");
    println!("BaseService: create_table successful");

    let table_id = TableId(table.id.unwrap().0);

    // Test open_table
    let _table_service = base_service.open_table(table_id.clone()).await?;
    println!("BaseService: open_table successful");

    Ok((base_service, base_id))
}

async fn test_table_service(
    base_service: &BaseService,
    user_id: &UserId,
    base_id: &BaseId,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = base_service
        .create_table("table_for_tests".to_string())
        .await?;
    let table_id = TableId(table.id.unwrap().0);

    let table_service =
        TableService::new(table_id.clone(), base_id.clone(), user_id.clone()).await?;

    // Test create_field
    let config = FieldConfig::Text(TextConfig::SingleLine {
        default: None,
        max_length: 255,
    });
    let insert_field = InsertField::new("test_field".to_string(), config, false, true, false);

    let field = table_service.create_field(insert_field).await?;
    assert_eq!(field.name, "test_field");
    println!("TableService: create_field successful");

    let field_id = FieldId(field.id.clone().unwrap().0);

    // Test get_field_config
    let config_fr = table_service.get_field_config(field_id.clone()).await?;
    assert_eq!(config_fr.name, "test_field");
    println!("TableService: get_field_config successful");

    // Test create_record
    let mut cells = HashMap::new();
    let val = Value::SingleLine(SingleLineValue::new(None, Some("test value".to_string()))?);
    cells.insert("test_field".to_string(), CellValue::new(val));
    let insert_record = InsertRecord::new(table_id.clone(), cells);
    let record = table_service.create_record(insert_record).await?;
    println!("TableService: create_record successful");

    let record_id = RecordId(record.id.clone().unwrap().0);

    // Test get_record
    let fetched_record = table_service.get_record(record_id.clone()).await?;
    if let Value::SingleLine(ref slv) = fetched_record.cells.get("test_field").unwrap().value {
        assert_eq!(slv.value(), "test value");
    } else {
        panic!("Wrong value type");
    }
    println!("TableService: get_record successful");

    // Test update_record
    let mut changed_cells = Vec::new();
    let val_updated = Value::SingleLine(SingleLineValue::new(
        None,
        Some("updated value".to_string()),
    )?);
    changed_cells.push(("test_field".to_string(), CellValue::new(val_updated)));
    let patch = RecordPatch::new(Some(changed_cells));
    let updated_record = table_service
        .update_record(record_id.clone(), patch)
        .await?;
    if let Value::SingleLine(ref slv) = updated_record.cells.get("test_field").unwrap().value {
        assert_eq!(slv.value(), "updated value");
    } else {
        panic!("Wrong value type");
    }
    println!("TableService: update_record successful");

    // Test list_records
    let records = table_service
        .list_records(PaginationParams {
            offset: Some(0),
            limit: Some(10),
        })
        .await?;
    assert!(!records.is_empty());
    println!("TableService: list_records successful");

    // Test check_migration
    let target_config = FieldConfig::Number(NumberConfig::Number { default: None });
    let report = table_service
        .check_migration(field_id.clone(), target_config)
        .await?;
    assert!(report.affected_records > 0);
    println!("TableService: check_migration successful");

    Ok(())
}

async fn test_misc() -> Result<(), Box<dyn std::error::Error>> {
    assert!(approved("valid_name").is_ok());
    assert!(approved("").is_err());
    assert!(approved("too_long_name_that_exceeds_thirty_characters").is_err());
    assert!(approved("invalid!char").is_err());
    println!("Misc: approved() successful");
    Ok(())
}

async fn run_benchmarks() -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 100;

    // Crypter Benchmarks
    bench_async!("encrypt_token", iterations, encrypt_token("some-token"));
    let encrypted = encrypt_token("some-token").await?;
    bench_async!(
        "decrypt_token",
        iterations,
        decrypt_token(encrypted.clone())
    );

    // Approved Benchmarks
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = approved("some_name");
    }
    let duration = start.elapsed();
    println!(
        "Benchmark {:<30} | Iterations: {:<6} | Total: {:<12?} | Avg: {:?}",
        "approved",
        iterations,
        duration,
        duration / iterations
    );

    Ok(())
}
