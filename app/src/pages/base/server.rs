use leptos::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct BaseTable {
    pub name: String,
    pub id: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct TableField {
    pub id: String,
    pub name: String,
    pub config: charac::models::field::FieldConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct TableRecord {
    pub id: String,
    pub cells: std::collections::HashMap<String, String>, // field_name -> value_string
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TableData {
    pub fields: Vec<TableField>,
    pub records: Vec<TableRecord>,
}

#[server]
pub async fn get_base_tables(base_id: String) -> Result<Vec<BaseTable>, ServerFnError> {
    use charac::models::ids::BaseId;
    use std::time::Instant;
    use surrealdb::types::RecordId;
    use surrealdb::types::ToSql;

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
            id: t.id.unwrap().0.to_sql(),
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
    use surrealdb::types::RecordId;
    use surrealdb::types::ToSql;

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
        id: table.id.unwrap().0.to_sql(),
    })
}

#[server]
pub async fn get_table_data(base_id: String, table_id: String) -> Result<TableData, ServerFnError> {
    use charac::models::ids::{BaseId, TableId};
    use std::time::Instant;
    use surrealdb::types::RecordId;
    use surrealdb::types::ToSql;

    let start = Instant::now();

    let mut user_service = crate::get_authenticated_service().await?;
    dbg!(&base_id, &table_id);

    let base_record_id = RecordId::parse_simple(format!("base:{}", base_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the base id"))?;
    let base_id_typed = BaseId(base_record_id);

    user_service
        .open_base(base_id_typed)
        .await
        .map_err(|e| ServerFnError::new(format!("Opening Base failed: {e:?}")))?;

    let table_record_id = RecordId::parse_simple(table_id.as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the table id"))?;
    let table_id_typed = TableId(table_record_id);

    let table_service = user_service
        .current_base
        .as_ref()
        .unwrap()
        .open_table(table_id_typed)
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let (fields, records) = table_service
        .get_full_data(None)
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let table_fields = fields
        .into_iter()
        .map(|f| TableField {
            id: f.id.clone().unwrap().0.to_sql(),
            name: f.name,
            config: f.config,
        })
        .collect::<Vec<_>>();

    let table_records = records
        .into_iter()
        .map(|r| {
            let mut cells = std::collections::HashMap::new();
            for (field_name, cell_value) in r.cells {
                cells.insert(field_name, cell_value.value.to_string());
            }
            TableRecord {
                id: r.id.clone().unwrap().0.to_sql(),
                cells,
            }
        })
        .collect();

    let duration = start.elapsed().as_millis();
    println!("[get_table_data] finished in {}ms", duration);

    Ok(TableData {
        fields: table_fields,
        records: table_records,
    })
}

#[server]
pub async fn update_record_cell(
    base_id: String,
    table_id: String,
    record_id: String,
    field_name: String,
    new_value: String,
) -> Result<(), ServerFnError> {
    use charac::db::DB;
    use charac::models::field::{Field, FieldConfig};
    use charac::models::ids::{BaseId, RecordId as CharacRecordId, TableId};
    use charac::models::record::RecordPatch;
    use charac::models::record::cell::CellValue;
    use std::time::Instant;
    use surrealdb::types::RecordId;

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

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the table id"))?;
    let table_id_typed = TableId(table_record_id);

    let table_service = user_service
        .current_base
        .as_ref()
        .unwrap()
        .open_table(table_id_typed.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    // Fetch field config for validation
    let mut res = DB
        .query("SELECT * FROM field WHERE table = $table AND name = $name AND is_deleted = false")
        .bind(("table", table_id_typed))
        .bind(("name", field_name.clone()))
        .await?;
    let field: Field = res
        .take::<Option<Field>>(0)?
        .ok_or(ServerFnError::new("Field not found"))?;

    // Validate and convert value
    let value = match field.config {
        FieldConfig::Text(_) => {
            use charac::models::record::cell::{SingleLineValue, Value};
            let v = SingleLineValue::new(None, Some(new_value))
                .map_err(|e| ServerFnError::new(format!("Validation failed: {e}")))?;
            Value::SingleLine(v)
        }
        FieldConfig::Number(_) => {
            use charac::models::record::cell::{NumberValue, Value};
            let n = new_value
                .parse::<usize>()
                .map_err(|_| ServerFnError::new("Invalid number"))?;
            let v = NumberValue::new(Some(n), None)
                .map_err(|e| ServerFnError::new(format!("Validation failed: {e}")))?;
            Value::Number(v)
        }
        _ => {
            use charac::models::record::cell::{SingleLineValue, Value};
            let v = SingleLineValue::new(None, Some(new_value))
                .map_err(|e| ServerFnError::new(format!("Validation failed: {e}")))?;
            Value::SingleLine(v)
        }
    };

    let cell_value = CellValue::new(value);
    let record_record_id = RecordId::parse_simple(&record_id)
        .map_err(|_| ServerFnError::new("coudlnt parse the record id"))?;

    table_service
        .update_record(
            CharacRecordId(record_record_id),
            RecordPatch::new(Some(vec![(field_name, cell_value)])),
        )
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[update_record_cell] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn delete_record(
    base_id: String,
    table_id: String,
    record_id: String,
) -> Result<(), ServerFnError> {
    use charac::models::ids::{BaseId, RecordId as CharacRecordId, TableId};
    use std::time::Instant;
    use surrealdb::types::RecordId;

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

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the table id"))?;
    let table_id_typed = TableId(table_record_id);

    let table_service = user_service
        .current_base
        .as_ref()
        .unwrap()
        .open_table(table_id_typed.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let record_record_id = RecordId::parse_simple(&record_id)
        .map_err(|_| ServerFnError::new("coudlnt parse the record id"))?;

    table_service
        .delete_record(CharacRecordId(record_record_id))
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[delete_record] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn create_record(
    base_id: String,
    table_id: String,
) -> Result<TableRecord, ServerFnError> {
    use charac::models::ids::{BaseId, TableId};
    use charac::models::record::InsertRecord;
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

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the table id"))?;
    let table_id_typed = TableId(table_record_id);

    let table_service = user_service
        .current_base
        .as_ref()
        .unwrap()
        .open_table(table_id_typed.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let record = table_service
        .create_record(InsertRecord::new(
            table_id_typed,
            std::collections::HashMap::new(),
        ))
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[create_record] finished in {}ms", duration);

    Ok(TableRecord {
        id: record.id.unwrap().0.to_sql(),
        cells: std::collections::HashMap::new(),
    })
}

#[server]
pub async fn create_field(
    base_id: String,
    table_id: String,
    name: String,
) -> Result<TableField, ServerFnError> {
    use charac::models::field::{FieldConfig, InsertField, TextConfig};
    use charac::models::ids::{BaseId, TableId};
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

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id.as_str()).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse the table id"))?;
    let table_id_typed = TableId(table_record_id);

    let table_service = user_service
        .current_base
        .as_ref()
        .unwrap()
        .open_table(table_id_typed.clone())
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let config = FieldConfig::Text(TextConfig::SingleLine {
        default: None,
        max_length: 255,
    });
    let insert_field = InsertField::new(name.clone(), config.clone(), false, true, false);

    let field = table_service
        .create_field(insert_field)
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[create_field] finished in {}ms", duration);

    Ok(TableField {
        id: field.id.unwrap().0.to_sql(),
        name,
        config,
    })
}
