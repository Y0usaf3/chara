use crate::core::models::user::{Name, UserPatch};
use crate::core::service::user::{AuthMethod, SessionI, UserService};
use crate::Root;
use crate::Ws;
use crate::DB;
use std::time::Instant;

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

    println!("\n---             Service Test             ---");
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
        first_name: Some(Name::new("Y0USAF3".to_string()).unwrap()),
        last_name: Some(Name::new("LM0UD3N".to_string()).unwrap()),
        is_deleted: None,
    };
    bench!("Update Self", service.update_self_user(patch))?;

    bench!("Is Admin Check", service.is_admin())?;

    let ws = bench!("Create Workspace", service.create_workspace("Test".into()))?;
    let ws_id = ws.id.unwrap().0;
    bench!("Delete Workspace", service.delete_workspace(ws_id))?;

    let user_id = service.id().clone();
    bench!("Delete User", service.delete_user(&user_id))?;

    println!("{}", "-".repeat(40));
    println!("Test Suite Complete.\n");

    Ok(())
}

// so im not sure if we really need this functionallity where we are able to make multiple
// workspaces, the best idea would be that there is only one workspace that is the default, we will
// not need to separate users and workspace users, bc they will be the same, and in the overall the
// security would be much easier to impl, well, tim to work on that hehe
