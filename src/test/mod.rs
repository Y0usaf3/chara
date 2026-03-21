use crate::core::models::user::{Name, UserPatch};
use crate::core::service::user::{AuthMethod, SessionI, UserService};
use crate::core::DB;
use std::time::Instant;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

/// Helper to measure execution time and print results nicely
macro_rules! bench {
    ($name:expr, $task:expr) => {{
        let start = Instant::now();
        let result = $task.await;
        let duration = start.elapsed();
        match &result {
            Ok(_) => println!(
                " {:<20} | {:>10.2}ms",
                $name,
                duration.as_secs_f64() * 1000.0
            ),
            Err(e) => eprintln!("󰳤 {:<20} | FAILED ({:?})", $name, e),
        }
        result
    }};
}

#[tokio::test]
async fn test_user_service_lifecycle_ambitious() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("100.118.244.5:3001").await?;

    DB.signin(Root {
        username: "yousafe".to_string(),
        password: "MRAOWRR".to_string(),
    })
    .await?;

    DB.use_ns("main").use_db("main").await?;

    println!("\n---             User Service Test             ---");
    println!("{:<23} | {:>12}", "Operation", "Latency");
    println!("{}", "-".repeat(40));

    let mut service: UserService = bench!(
        "Login User",
        UserService::login(AuthMethod::Session(SessionI {
            ip: "192.168.11.100".to_string(),
            agent: "owo".to_string(),
            token: "IIOOII".to_string(),
        }))
    )?;

    bench!("Refresh User", service.refresh_user())?;

    let patch = UserPatch {
        first_name: Some(Name::new("YOUSAFE".to_string()).unwrap()),
        last_name: Some(Name::new("LMOUDEN".to_string()).unwrap()),
        is_deleted: None,
    };
    bench!("Update Self", service.update_self_user(patch))?;

    bench!("Is Admin Check", service.is_admin())?;

    // Verify Self-Deletion Prevention
    let self_id = service.id().clone();
    match service.delete_user(&self_id).await {
        Ok(_) => {
            eprintln!(
                "󰳤 {:<20} | FAILED (Expected CannotDeleteSelf, got Ok)",
                "Self Delete"
            );
            return Err("Expected CannotDeleteSelf error".into());
        }
        Err(_) => {
            println!(
                " {:<20} | Success (Correctly caught self-deletion)",
                "Self Delete Check"
            );
        }
    }

    // Verify Deletion of Another User
    // Create a dummy user directly in DB to test deletion
    let dummy_user: crate::core::models::user::User = DB
        .create("user")
        .content(crate::core::models::user::User::from_insert(
            crate::core::models::user::InsertUser {
                first_name: Name::new("Dummy".to_string()).unwrap(),
                last_name: Name::new("User".to_string()).unwrap(),
                email: "dummy@example.com".parse().unwrap(),
            },
        ))
        .await?
        .unwrap();

    let dummy_id = dummy_user.id.clone().unwrap();
    bench!("Delete Other User", service.delete_user(&dummy_id))?;

    let base = bench!("Create Base", service.create_base("TestBase".to_string()))?;
    let base_id = base.id.clone().unwrap();

    // 2. Open Base
    bench!("Open Base", service.open_base(base_id.clone()))?;
    let base_service = service.current_base.as_ref().unwrap();

    // 3. Create Table
    let table = bench!(
        "Create Table",
        base_service.create_table("TestTable".to_string())
    )?;
    let table_id = table.id.clone().unwrap();

    // 4. Invite User
    let dummy_user: crate::core::models::user::User = DB
        .create("user")
        .content(crate::core::models::user::User::from_insert(
            crate::core::models::user::InsertUser {
                first_name: Name::new("Guest".to_string()).unwrap(),
                last_name: Name::new("User".to_string()).unwrap(),
                email: "guest@example.com".parse().unwrap(),
            },
        ))
        .await?
        .unwrap();
    let guest_id = dummy_user.id.clone().unwrap();

    // View (1 << 1) | ManageInvitations (1 << 8) = 2 + 256 = 258
    bench!(
        "Invite User",
        base_service.invite_user(guest_id, 258.into())
    )?;

    // 5. Open Table
    bench!("Open Table", base_service.open_table(table_id.clone()))?;

    // 6. Delete Table
    bench!("Delete Table", base_service.delete_table(table_id))?;

    // 7. Delete Base
    bench!("Delete Base", base_service.delete())?;

    println!("{}", "-".repeat(40));
    println!("User Service Test Suite Complete.\n");

    Ok(())
}

// so im not sure if we really need this functionallity where we are able to make multiple
// workspaces, the best idea would be that there is only one workspace that is the default, we will
// not need to separate users and workspace users, bc they will be the same, and in the overall the
// security would be much easier to impl, well, tim to work on that hehe
