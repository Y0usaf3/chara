use charac::models::field::FieldConfig;
use charac::models::record::cell::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TableData {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub records: Vec<RecordInfo>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FieldInfo {
    pub name: String,
    pub id: String,
    pub config: FieldConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RecordInfo {
    pub id: String,
    pub cells: HashMap<String, String>, // field_name -> value_string
}

#[server]
pub async fn get_table_data(table_id_str: String) -> Result<TableData, ServerFnError> {
    use charac::models::ids::{BaseId, TableId};
    use charac::service::table::PaginationParams;
    use charac::{models::Record, service::table::TableService};
    use std::time::Instant;
    use surrealdb::types::{RecordId, ToSql};

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;

    let table_id = TableId(table_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to find base for table: {e}")))?;

    let base_id: BaseId = base_res
        .take::<Option<BaseId>>(0)?
        .ok_or_else(|| ServerFnError::new("Table has no base"))?;
    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;
    // Get fields
    let mut field_res = charac::db::DB
        .query("SELECT * FROM field WHERE table = $table AND is_deleted = false ORDER BY order ASC")
        .bind(("table", table_id.clone()))
        .await
        .map_err(|e| ServerFnError::new(format!("Querying fields failed: {e}")))?;

    let fields: Vec<charac::models::field::Field> = field_res.take(0)?;
    let field_infos: Vec<FieldInfo> = fields
        .iter()
        .map(|f| FieldInfo {
            name: f.name.clone(),
            id: f.id.as_ref().unwrap().0.key.to_sql(),
            config: f.config.clone(),
        })
        .collect();

    // Get records
    let records: Vec<Record> = table_service
        .list_records(PaginationParams {
            offset: None,
            limit: Some(100),
        })
        .await
        .map_err(|e| ServerFnError::new(format!("Listing records failed: {e:?}")))?;

    let record_infos = records
        .into_iter()
        .map(|r| {
            let mut cells = HashMap::new();
            for (name, cell) in r.cells {
                cells.insert(name, cell.value.to_string());
            }
            RecordInfo {
                id: r.id.unwrap().0.key.to_sql(),
                cells,
            }
        })
        .collect();

    let duration = start.elapsed().as_millis();
    println!("[get_table_data] finished in {}ms", duration);

    Ok(TableData {
        name: table_service.table.name.clone(),
        fields: field_infos,
        records: record_infos,
    })
}

#[server]
pub async fn delete_record(
    table_id_str: String,
    record_id_str: String,
) -> Result<(), ServerFnError> {
    use charac::models::ids::{BaseId, RecordId as IdRecord, TableId};
    use charac::service::table::TableService;
    use std::time::Instant;
    use surrealdb::types::RecordId;

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;
    let table_id = TableId(table_record_id);

    let record_record_id = RecordId::parse_simple(format!("record:{}", record_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse record id"))?;
    let record_id = IdRecord(record_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await?;
    let base_id: BaseId = base_res.take::<Option<BaseId>>(0)?.unwrap();

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    table_service
        .delete_record(record_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Delete record failed: {e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[delete_record] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn delete_field(table_id_str: String, field_id_str: String) -> Result<(), ServerFnError> {
    use charac::models::ids::{BaseId, FieldId as IdField, TableId};
    use charac::service::table::TableService;
    use std::time::Instant;
    use surrealdb::types::RecordId;

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;
    let table_id = TableId(table_record_id);

    let field_record_id = RecordId::parse_simple(format!("field:{}", field_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse field id"))?;
    let field_id = IdField(field_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await?;
    let base_id: BaseId = base_res.take::<Option<BaseId>>(0)?.unwrap();

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    table_service
        .delete_field(field_id)
        .await
        .map_err(|e| ServerFnError::new(format!("Delete field failed: {e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[delete_field] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn add_record(table_id_str: String) -> Result<(), ServerFnError> {
    use charac::models::ids::{BaseId, TableId};
    use charac::models::record::InsertRecord;
    use charac::service::table::TableService;
    use std::time::Instant;
    use surrealdb::types::RecordId;

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;
    let table_id = TableId(table_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to find base: {e}")))?;
    let base_id: BaseId = base_res
        .take::<Option<BaseId>>(0)?
        .ok_or_else(|| ServerFnError::new("Table has no base"))?;

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    let insert = InsertRecord::new(table_id, std::collections::HashMap::new());
    table_service
        .create_record(insert)
        .await
        .map_err(|e| ServerFnError::new(format!("Create record failed: {e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[add_record] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn update_cell(
    table_id_str: String,
    record_id_str: String,
    field_id_str: String,
    value: String,
) -> Result<(), ServerFnError> {
    use charac::models::ids::{BaseId, FieldId as IdField, RecordId as IdRecord, TableId};
    use charac::models::record::RecordPatch;
    use charac::models::record::cell::*;
    use charac::service::table::TableService;
    use std::str::FromStr;
    use std::time::Instant;
    use surrealdb::types::{Datetime, RecordId};

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;
    let table_id = TableId(table_record_id);

    let record_record_id = RecordId::parse_simple(format!("record:{}", record_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse record id"))?;
    let record_id = IdRecord(record_record_id);

    let field_record_id = RecordId::parse_simple(format!("field:{}", field_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse field id"))?;
    let field_id = IdField(field_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await?;
    let base_id: BaseId = base_res.take::<Option<BaseId>>(0)?.unwrap();

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    // Fetch field to know the type
    let mut field_res = charac::db::DB
        .query("SELECT * FROM $field")
        .bind(("field", field_id.clone()))
        .await?;
    let field: charac::models::field::Field = field_res
        .take::<Option<charac::models::field::Field>>(0)?
        .ok_or_else(|| ServerFnError::new("Field not found"))?;

    let val = match field.config {
        FieldConfig::Text(text_config) => match text_config {
            charac::models::field::TextConfig::Email => {
                Value::Email(Email::new(value).map_err(|e| ServerFnError::new(e.to_string()))?)
            }
            charac::models::field::TextConfig::URL => {
                Value::URL(UrlValue::new(value).map_err(|e| ServerFnError::new(e.to_string()))?)
            }
            charac::models::field::TextConfig::Phone => {
                Value::Phone(PhoneValue::new(value, None).map_err(|e| ServerFnError::new(e.to_string()))?)
            }
            charac::models::field::TextConfig::LongText { rich_text } => {
                Value::LongText(Box::new(LongTextValue::new(value, rich_text).map_err(|e| ServerFnError::new(e.to_string()))?))
            }
            _ => Value::SingleLine(
                SingleLineValue::new(None, Some(value))
                    .map_err(|e| ServerFnError::new(e.to_string()))?,
            ),
        },
        FieldConfig::Number(num_config) => match num_config {
            charac::models::field::NumberConfig::Decimal { default, .. } => {
                if value.trim().is_empty() {
                     Value::Decimal(DecimalValue::new(None, default.map(|f| f as f64)).map_err(|e| ServerFnError::new(e.to_string()))?)
                } else {
                    let n = value
                        .parse::<f64>()
                        .map_err(|_| ServerFnError::new("Invalid decimal"))?;
                    Value::Decimal(
                        DecimalValue::new(Some(n), default.map(|f| f as f64)).map_err(|e| ServerFnError::new(e.to_string()))?,
                    )
                }
            }
            charac::models::field::NumberConfig::Percent { .. } => {
                if value.trim().is_empty() {
                    Value::Percent(PercentValue::new(0))
                } else {
                    let n = value
                        .parse::<i32>()
                        .map_err(|_| ServerFnError::new("Invalid percent"))?;
                    Value::Percent(PercentValue::new(n))
                }
            }
            charac::models::field::NumberConfig::Rating { max, .. } => {
                if value.trim().is_empty() {
                    Value::Rating(RatingValue::new(Some(0), max as u8).map_err(|e| ServerFnError::new(e.to_string()))?)
                } else {
                    let n = value
                        .parse::<u8>()
                        .map_err(|_| ServerFnError::new("Invalid rating"))?;
                    Value::Rating(
                        RatingValue::new(Some(n), max as u8).map_err(|e| ServerFnError::new(e.to_string()))?,
                    )
                }
            }
            charac::models::field::NumberConfig::Currency { .. } => {
                 if value.trim().is_empty() {
                    Value::Currency(CurrencyValue::new(0, iso_currency::Currency::from_code("USD").unwrap().symbol()))
                } else {
                    let n = value
                        .parse::<i64>()
                        .map_err(|_| ServerFnError::new("Invalid currency value"))?;
                    Value::Currency(CurrencyValue::new(n, iso_currency::Currency::from_code("USD").unwrap().symbol()))
                }
            }
            charac::models::field::NumberConfig::Number { default } => {
                if value.trim().is_empty() {
                     Value::Number(NumberValue::new(None, default).map_err(|e| ServerFnError::new(e.to_string()))?)
                } else {
                    let n = value
                        .parse::<usize>()
                        .map_err(|_| ServerFnError::new("Invalid number"))?;
                    Value::Number(
                        NumberValue::new(Some(n), default).map_err(|e| ServerFnError::new(e.to_string()))?,
                    )
                }
            }
        },
        FieldConfig::Datetime(_) => {
            if value.trim().is_empty() {
                return Err(ServerFnError::new("Date cannot be empty"));
            }
            let dt = Datetime::from_str(&value)
                .map_err(|_| ServerFnError::new("Invalid date format"))?;
            Value::Date(DateValue::new(dt))
        }
        _ => Value::SingleLine(
            SingleLineValue::new(None, Some(value))
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        ),
    };

    let cell = CellValue::new(val);

    let patch = RecordPatch::new(Some(vec![(field.name, cell)]));
    table_service
        .update_record(record_id, patch)
        .await
        .map_err(|e| ServerFnError::new(format!("Update failed: {e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[update_cell] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn add_field(
    table_id_str: String,
    name: String,
    field_type: String,
) -> Result<(), ServerFnError> {
    use charac::models::field::*;
    use charac::models::ids::{BaseId, TableId};
    use charac::service::table::TableService;
    use std::time::Instant;
    use surrealdb::types::RecordId;

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;

    let table_id = TableId(table_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await?;
    let base_id: BaseId = base_res.take::<Option<BaseId>>(0)?.unwrap();

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    let config = match field_type.as_str() {
        "Text" => FieldConfig::Text(TextConfig::SingleLine {
            default: None,
            max_length: 255,
        }),
        "Long Text" => FieldConfig::Text(TextConfig::LongText { rich_text: false }),
        "Email" => FieldConfig::Text(TextConfig::Email),
        "URL" => FieldConfig::Text(TextConfig::URL),
        "Phone" => FieldConfig::Text(TextConfig::Phone),
        "Number" => FieldConfig::Number(NumberConfig::Number { default: None }),
        "Decimal" => FieldConfig::Number(NumberConfig::Decimal {
            default: None,
            precision: 2,
        }),
        "Currency" => FieldConfig::Number(NumberConfig::Currency {
            currency: "USD".to_string(),
            precision: 2,
        }),
        "Percent" => FieldConfig::Number(NumberConfig::Percent {
            precision: 2,
            show_bar: false,
        }),
        "Rating" => FieldConfig::Number(NumberConfig::Rating {
            max: 5,
            icon_type: charac::models::field::RatingIcon::Star,
            color: [255, 200, 0],
        }),
        "Datetime" => FieldConfig::Datetime(DatetimeConfig::Date {
            format: DateFormat::ISO,
            include_time: true,
        }),
        "Select" => FieldConfig::Select(charac::models::field::SelectConfig::Single { options: vec![] }),
        "Select Multi" => FieldConfig::Select(charac::models::field::SelectConfig::Multi { options: vec![] }),
        _ => FieldConfig::Text(TextConfig::SingleLine {
            default: None,
            max_length: 255,
        }),
    };

    let insert = InsertField::new(name, config, false, true, false);

    table_service
        .create_field(insert)
        .await
        .map_err(|e| ServerFnError::new(format!("Create field failed: {e:?}")))?;

    let duration = start.elapsed().as_millis();
    println!("[add_field] finished in {}ms", duration);

    Ok(())
}

#[server]
pub async fn update_field(
    table_id_str: String,
    field_id_str: String,
    name: String,
    config: FieldConfig,
) -> Result<(), ServerFnError> {
    use charac::models::field::*;
    use charac::models::ids::{BaseId, FieldId as IdField, TableId};
    use charac::service::table::TableService;
    use std::time::Instant;
    use surrealdb::types::RecordId;

    let start = Instant::now();
    let service = crate::get_authenticated_service().await?;
    let mut user_service = service;

    let table_record_id = RecordId::parse_simple(format!("table:{}", table_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse table id"))?;
    let table_id = TableId(table_record_id);

    let field_record_id = RecordId::parse_simple(format!("field:{}", field_id_str).as_str())
        .ok()
        .ok_or(ServerFnError::new("coudlnt parse field id"))?;
    let field_id = IdField(field_record_id);

    let mut base_res = charac::db::DB
        .query("SELECT VALUE base FROM $table")
        .bind(("table", table_id.clone()))
        .await?;
    let base_id: BaseId = base_res.take::<Option<BaseId>>(0)?.unwrap();

    user_service.open_base(base_id).await?;
    let table_service: TableService = user_service
        .current_base
        .unwrap()
        .open_table(table_id.clone())
        .await?;

    let insert = InsertField::new(name, config, false, true, false);
    table_service
        .update_field(field_id, insert)
        .await
        .map_err(|e| ServerFnError::new(format!("Update field failed: {e:?}")))?
        .map_err(|e| ServerFnError::new(format!("Migration needed: {:?}", e)))?;

    let duration = start.elapsed().as_millis();
    println!("[update_field] finished in {}ms", duration);

    Ok(())
}
