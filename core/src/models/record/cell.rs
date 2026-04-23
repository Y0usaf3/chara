use std::str::FromStr;

use crate::models::field::{AggregateFunction, LinkType, Prefix};
use crate::models::ids::*;
use iso_currency::CurrencySymbol;
use ordered_float::OrderedFloat;
use surrealdb::types::{Datetime, Duration, Kind, Number, SurrealValue, Value as XValue};
use thiserror::Error;
use uuid::Uuid;
use validator::ValidateLength;

pub const MAX_TEXT_LENGHT: u32 = 999_999; // ~1MB single byte chars

#[derive(Error, Debug, Clone, PartialEq, Eq, SurrealValue)]
pub enum CellError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),

    #[error("Unparseable phone number: {0}")]
    UnparseablePhoneNumber(String),

    #[error("Value exceeds maximum rating of {max} ({value})")]
    RatingExceedsMax { value: u8, max: u8 },

    #[error("Required value is missing (both value and default are None)")]
    MissingValue,

    #[error("JSON parsing failed: {0}")]
    InvalidJson(String),

    #[error("Formula evaluation failed: {0}")]
    FormulaEvaluationError(String),

    #[error("Circular reference detected in formula or link")]
    CircularReference,

    #[error("Link error: One-to-One relationship cannot contain multiple IDs")]
    LinkConstraintViolation,

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Text too big (lenght: {0})")]
    TextTooBig(u64),
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct CellValue {
    pub id: CellId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub value: Value,
}

impl CellValue {
    pub fn new(value: Value) -> Self {
        use surrealdb::types::RecordId as Thing;
        Self {
            id: CellId(Thing {
                table: "cell".into(),
                key: uuid::Uuid::new_v4().to_string().into(),
            }),
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            value,
        }
    }
}

impl Value {
    pub fn convert_to(
        &self,
        target_config: &crate::models::field::FieldConfig,
    ) -> Result<Self, CellError> {
        use crate::models::field::*;

        // 1. To Text (Always safe)
        if let FieldConfig::Text(text_config) = target_config {
            let s = self.to_string();
            return match text_config {
                TextConfig::SingleLine { default, .. } => {
                    SingleLineValue::new(default.clone(), Some(s)).map(Value::SingleLine)
                }
                TextConfig::LongText { rich_text } => {
                    LongTextValue::new(s, *rich_text).map(|v| Value::LongText(Box::new(v)))
                }
                TextConfig::Email => Email::new(s).map(Value::Email),
                TextConfig::URL => UrlValue::new(s).map(Value::URL),
                TextConfig::Phone => PhoneValue::new(s, None).map(Value::Phone),
            };
        }

        // 2. To Number
        if let FieldConfig::Number(num_config) = target_config {
            let s = self.to_string();
            let n_float = s.parse::<f64>().map_err(|_| CellError::MissingValue)?; // Placeholder error

            return match num_config {
                NumberConfig::Number { default } => {
                    NumberValue::new(Some(n_float as usize), *default).map(Value::Number)
                }
                NumberConfig::Decimal { default, .. } => {
                    DecimalValue::new(Some(n_float), default.map(|f| f as f64)).map(Value::Decimal)
                }
                _ => Err(CellError::MissingValue), // TODO: Add more specific error or handle other number types
            };
        }

        // 3. To Datetime
        if let FieldConfig::Datetime(dt_config) = target_config {
            if let DatetimeConfig::Date { .. } = dt_config {
                let s = self.to_string();
                let dt = Datetime::from_str(&s).map_err(|_| CellError::MissingValue)?;
                return Ok(Value::Date(DateValue::new(dt)));
            }
        }

        // 4. Default: Return error if incompatible
        Err(CellError::FieldNotFound(
            "Incompatible types for migration".into(),
        ))
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::SingleLine(v) => v.value().to_string(),
            Value::LongText(v) => v.value().to_string(),
            Value::Email(v) => v.value().to_string(),
            Value::URL(v) => v.value().to_string(),
            Value::Phone(v) => v.value().to_string(),
            Value::Number(v) => v.value().to_string(),
            Value::Decimal(v) => v.value().to_string(),
            Value::Currency(v) => v.value_as_str().to_string(),
            Value::Percent(v) => v.value().to_string(),
            Value::Rating(v) => v.value().to_string(),
            Value::Date(v) => v.value().to_string(),
            Value::Duration(v) => v.value().to_string(),
            Value::CreatedAt(v) => v.value().to_string(),
            Value::ModifiedTime(v) => v.value().to_string(),
            Value::JSON(v) => v.value(),
            _ => "".to_string(), // Fallback for complex types
        }
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub enum Value {
    SingleLine(SingleLineValue),
    LongText(Box<LongTextValue>),
    Email(Email),
    URL(UrlValue),
    Phone(PhoneValue),
    Number(NumberValue),
    Decimal(DecimalValue),
    Currency(CurrencyValue),
    Percent(PercentValue),
    Rating(RatingValue),
    Date(DateValue),
    Duration(DurationValue),
    Link(LinkValue),
    LookUp(Box<LookUpValue>),
    RollUp(Box<RollUpValue>),
    Formula(Box<FormulaValue>),
    AutoNumber(AutoNumberValue),
    CreatedAt(CreatedAtValue),
    ModifiedTime(ModifiedTimeValue),
    Attachment(Box<AttachmentValue>),
    JSON(Box<JsonValue>),
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct AttachmentItem {
    file_id: Uuid,   // UUID
    name: String,    // the original name of the file
    mime_type: Meme, // e.g., "image/jpeg"
    size: usize,     // size in bytes
    uploaded_at: Datetime,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meme(pub mime::Mime);

impl surrealdb::types::SurrealValue for Meme {
    fn kind_of() -> Kind {
        Kind::String
    }

    fn into_value(self) -> XValue {
        XValue::String(String::from(self.0.to_string()))
    }

    fn from_value(value: XValue) -> Result<Self, surrealdb::Error> {
        match value {
            XValue::String(s) => mime::Mime::from_str(&s)
                .map(Meme)
                .map_err(|e| surrealdb::Error::thrown(format!("Invalid MIME type: {}", e))),
            _ => Err(surrealdb::Error::thrown(
                "Expected a string (Strand) for MimeWrapper".to_string(),
            )),
        }
    }

    fn is_value(value: &XValue) -> bool {
        matches!(value, XValue::String(_))
    }
}

impl AttachmentItem {
    // function for only new attachement, if we already have one then serde should do the job
    pub fn new(file_id: Uuid, name: String, mime_type: mime::Mime, size: usize) -> Self {
        Self {
            file_id,
            name,
            mime_type: Meme(mime_type),
            size,
            uploaded_at: Datetime::from(chrono::Utc::now()),
        }
    }

    // functions placeholder so we can add S3 after
    //pub fn s3_key(&self) -> String {
    //    self.file_id.to_string()
    //}
    //
    //pub fn content_disposition(&self) -> String {
    //    format!("attachment; filename=\"{}\"", self.name)
    //}

    pub fn mime_str(&self) -> String {
        self.mime_type.0.to_string()
    }

    pub fn is_displayable_image(&self) -> bool {
        matches!(
            (self.mime_type.0.type_(), self.mime_type.0.subtype()),
            (mime::IMAGE, mime::JPEG)
                | (mime::IMAGE, mime::PNG)
                | (mime::IMAGE, mime::GIF)
                | (mime::IMAGE, mime::SVG)
        )
    }

    pub fn readable_size(&self) -> String {
        format!("{} Mb", self.size / (1024 * 1024))
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct AttachmentValue {
    files: Vec<AttachmentItem>,
}

impl AttachmentValue {
    pub fn new(files: Vec<AttachmentItem>) -> Self {
        Self { files }
    }
    pub fn value(&self) -> Vec<AttachmentItem> {
        self.files.clone()
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct JsonValue {
    pub value: String, // use a strong type for json so we make sure its parseable
}

impl JsonValue {
    pub fn new(value: String) -> Result<Self, CellError> {
        serde_json::from_str::<serde_json::Value>(&value)
            .map_err(|e| CellError::InvalidJson(e.to_string()))?;
        Ok(JsonValue { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct ModifiedTimeValue {
    pub value: Datetime,
}

impl ModifiedTimeValue {
    pub fn new(value: Datetime) -> Self {
        ModifiedTimeValue { value }
    }
    pub fn value(&self) -> &Datetime {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct CreatedAtValue {
    pub value: Datetime,
}

impl CreatedAtValue {
    pub fn new(value: Datetime) -> Self {
        CreatedAtValue { value }
    }
    pub fn value(&self) -> &Datetime {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct AutoNumberValue {
    value: usize,
    prefix: Prefix,
    formatted: String,
}

impl AutoNumberValue {
    pub fn new(value: usize, prefix: Prefix) -> Self {
        let prefix_str = match prefix {
            Prefix::Dot => '.',
            Prefix::Star => '*',
        };

        let formatted = format!("{}{}", prefix_str, value);
        AutoNumberValue {
            value,
            prefix,
            formatted,
        }
    }

    pub fn formatted(&self) -> &str {
        self.formatted.as_str()
    }

    pub fn prefix(&self) -> &Prefix {
        &self.prefix
    }

    pub fn value(&self) -> &usize {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct FormulaValue {
    expression: String, // change it by an Expression type to make sure abt safty
    result: Box<Value>,
}

impl FormulaValue {
    pub fn new(expression: String, value: Value) -> Self {
        // TODO: make sure expression is correct, and should return a Box<Value>
        FormulaValue {
            expression,
            result: Box::new(value),
        }
    }

    pub fn result(&self) -> &Value {
        &self.result
    }

    pub fn expression(&self) -> &String {
        &self.expression
    }
}

// so for rollups and lookups, its a bit tricky, we have the field id, but we also have to update
// it, so the solution is that we use surrealdbs event/trigger stuff to automaticly update the data
// to the tables
// work on code type safety to make this as ez as cake :p

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct RollUpValue {
    link_field_id: FieldId,
    target_field_id: FieldId,
    function: AggregateFunction,
    computed_values: Box<Value>,
}

impl RollUpValue {
    pub fn new(
        link_field_id: FieldId,
        target_field_id: FieldId,
        function: AggregateFunction,
        computed_values: Value,
    ) -> Self {
        Self {
            link_field_id,
            target_field_id,
            function,
            computed_values: Box::new(computed_values),
        }
    }

    pub fn value(&self) -> &Value {
        &self.computed_values
    }

    pub fn function(&self) -> &AggregateFunction {
        &self.function
    }
    pub fn target_field_id(&self) -> &FieldId {
        &self.target_field_id
    }
    pub fn link_field_id(&self) -> &FieldId {
        &self.link_field_id
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct LookUpValue {
    link_field_id: FieldId,
    target_field_id: FieldId,
    computed_values: Box<Value>,
}

impl LookUpValue {
    pub fn new(link_field_id: FieldId, target_field_id: FieldId, computed_values: Value) -> Self {
        Self {
            link_field_id,
            target_field_id,
            computed_values: Box::new(computed_values),
        }
    }
    pub fn link_field_id(&self) -> &FieldId {
        &self.link_field_id
    }
    pub fn target_field_id(&self) -> &FieldId {
        &self.target_field_id
    }

    pub fn value(&self) -> &Value {
        &self.computed_values
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct LinkValue {
    pub target_table_id: TableId,
    pub record_ids: Vec<RecordId>,
    pub link_type: LinkType,
}

impl LinkValue {
    pub fn new(target_table_id: TableId, link_type: LinkType, record_ids: Vec<RecordId>) -> Self {
        let final_ids = if link_type == LinkType::OneToOne && record_ids.len() > 1 {
            vec![record_ids[0].clone()]
        } else {
            record_ids
        };

        LinkValue {
            target_table_id,
            link_type,
            record_ids: final_ids,
        }
    }

    pub fn record_ids(&self) -> &[RecordId] {
        &self.record_ids
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct DurationValue {
    value: Duration,
}

impl DurationValue {
    pub fn new(value: Duration) -> Self {
        DurationValue { value }
    }

    pub fn value(&self) -> &Duration {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct DateValue {
    value: Datetime,
}

impl DateValue {
    pub fn new(value: Datetime) -> Self {
        DateValue { value }
    }

    pub fn value(&self) -> &Datetime {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct RatingValue {
    value: u8,
}

impl RatingValue {
    pub fn new(value: Option<u8>, max: u8) -> Result<Self, CellError> {
        let ratings = value.unwrap_or(0);
        if ratings > max {
            return Err(CellError::RatingExceedsMax {
                value: ratings,
                max,
            });
        };
        Ok(Self { value: ratings })
    }
    pub fn value(&self) -> &u8 {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct PercentValue {
    value: i32,
}

impl PercentValue {
    pub fn new(value: i32) -> Self {
        PercentValue { value }
    }
    pub fn value(&self) -> &i32 {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct CurrencyValue {
    value: i64,
    currency_symbole: String,
    formatted: String,
}

impl CurrencyValue {
    pub fn new(value: i64, currency_symbole: CurrencySymbol) -> Self {
        let formatted = format!("{} {}", value, &currency_symbole.symbol);
        CurrencyValue {
            value,
            currency_symbole: currency_symbole.to_string(),
            formatted,
        }
    }

    pub fn value_as_int(&self) -> &i64 {
        &self.value
    }

    pub fn value_as_str(&self) -> &str {
        &self.formatted
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct DecimalValue {
    value: OrderedFloatIThink,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedFloatIThink(pub OrderedFloat<f64>);

impl surrealdb::types::SurrealValue for OrderedFloatIThink {
    fn kind_of() -> Kind {
        // In the database, this is essentially a decimal/number
        Kind::Float
    }

    fn into_value(self) -> XValue {
        XValue::Number(Number::Float(self.0.0))
    }

    fn from_value(value: XValue) -> Result<Self, surrealdb::Error> {
        match value {
            XValue::Number(Number::Float(n)) => Ok(OrderedFloatIThink(OrderedFloat::<f64>(n))),
            _ => Err(surrealdb::Error::thrown(
                "Expected a number for DecimalValue".to_string(),
            )),
        }
    }

    fn is_value(value: &XValue) -> bool {
        matches!(value, XValue::Number(_))
    }
}

impl DecimalValue {
    pub fn new(value: Option<f64>, default: Option<f64>) -> Result<Self, CellError> {
        if value.is_none() && default.is_none() {
            return Err(CellError::MissingValue);
        };
        if let Some(v) = value {
            Ok(DecimalValue {
                value: OrderedFloatIThink(OrderedFloat::from(v)),
            })
        } else {
            Ok(DecimalValue {
                value: OrderedFloatIThink(OrderedFloat::from(default.unwrap_or(0.0))),
            })
        }
    }

    pub fn value(&self) -> &OrderedFloat<f64> {
        &self.value.0
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct NumberValue {
    value: usize,
}

impl NumberValue {
    pub fn new(value: Option<usize>, default: Option<usize>) -> Result<Self, CellError> {
        if value.is_none() && default.is_none() {
            return Err(CellError::MissingValue);
        };
        if let Some(v) = value {
            Ok(NumberValue { value: v })
        } else {
            Ok(NumberValue {
                value: default.unwrap_or(0),
            })
        }
    }
    pub fn value(&self) -> &usize {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct PhoneValue {
    value: String,
}

impl PhoneValue {
    pub fn new(value: String, default_region: Option<&str>) -> Result<Self, CellError> {
        let region = default_region.and_then(|r| r.parse().ok());

        match phonenumber::parse(region, &value) {
            Ok(phone) => {
                if phonenumber::is_valid(&phone) {
                    let formatted = phone.format().mode(phonenumber::Mode::E164).to_string();
                    Ok(Self { value: formatted })
                } else {
                    Err(CellError::InvalidPhoneNumber(value))
                }
            }
            Err(_) => Err(CellError::UnparseablePhoneNumber(value)),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct UrlValue {
    value: String,
}

impl UrlValue {
    pub fn new(value: String) -> Result<Self, CellError> {
        if validator::ValidateUrl::validate_url(&value) {
            Ok(Self {
                value: value.trim().to_string(),
            })
        } else {
            Err(CellError::InvalidUrl(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, CellError> {
        if validator::ValidateEmail::validate_email(&value) {
            Ok(Self {
                value: value.trim().to_lowercase(),
            })
        } else {
            Err(CellError::InvalidEmail(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct LongTextValue {
    value: String,
}

impl LongTextValue {
    pub fn new(value: String, rich_text: bool) -> Result<Self, CellError> {
        let processed = if rich_text {
            value.trim().to_string()
        } else {
            value
                .replace(['*', '_', '#', '`', '[', ']'], "")
                .trim()
                .to_string()
        };
        let text_lenght = processed.length().unwrap();
        if text_lenght > MAX_TEXT_LENGHT.into() {
            return Err(CellError::TextTooBig(text_lenght));
        };
        Ok(Self { value: processed })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, SurrealValue, PartialEq, Eq, Hash)]
pub struct SingleLineValue {
    value: String,
}

impl SingleLineValue {
    pub fn new(default: Option<String>, value: Option<String>) -> Result<Self, CellError> {
        let raw = value.or(default).unwrap_or_default();
        let single_line = raw.replace(['\n', '\r'], " ");

        let text_lenght = single_line.length().unwrap();
        if text_lenght > MAX_TEXT_LENGHT.into() {
            return Err(CellError::TextTooBig(text_lenght));
        };

        Ok(Self { value: single_line })
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
