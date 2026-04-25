use leptos::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct BaseTable {
    pub name: String,
    pub id: String,
}

#[server]
pub async fn get_base_tables(base_id: String) -> Result<Vec<BaseTable>, ServerFnError> {
    use charac::models::ids::BaseId;
    use std::time::Instant;
    use surrealdb::types::{RecordId, ToSql};

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let base_record_id = RecordId::parse_simple(format!("base:{}", base_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the base id"))?;

    let base_id_typed = BaseId(base_record_id);
    user_service
        .open_base(base_id_typed)
        .await
        .map_err(|e| ServerFnError::new(format!("Opening Base failed: {e:?}")))?;

    let tables = user_service
        .current_base
        .as_ref()
        .unwrap()
        .list_tables()
        .await
        .map_err(|e| ServerFnError::new(format!("Listing Tables failed: {e:?}")))?;

    let base_tables = tables
        .into_iter()
        .map(|t| BaseTable {
            name: t.name,
            id: t.id.unwrap().0.key.to_sql(),
        })
        .collect();

    let duration = start.elapsed().as_millis();
    println!("[get_base_tables] finished in {}ms", duration);

    Ok(base_tables)
}

#[server]
pub async fn create_table_in_base(
    base_id: String,
    name: String,
) -> Result<BaseTable, ServerFnError> {
    use charac::models::ids::BaseId;
    use std::time::Instant;
    use surrealdb::types::{RecordId, ToSql};

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let base_record_id = RecordId::parse_simple(format!("base:{}", base_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the base id"))?;

    let base_id_typed = BaseId(base_record_id);
    user_service
        .open_base(base_id_typed)
        .await
        .map_err(|e| ServerFnError::new(format!("{e}")))?;

    let table = user_service
        .current_base
        .as_ref()
        .unwrap()
        .create_table(name)
        .await
        .map_err(|e| ServerFnError::new(format!("{e}")))?;

    let duration = start.elapsed().as_millis();
    println!("[create_table_in_base] finished in {}ms", duration);

    Ok(BaseTable {
        name: table.name,
        id: table.id.unwrap().0.key.to_sql(),
    })
}
