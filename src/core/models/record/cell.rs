use crate::core::models::field::{AggregateFunction, LinkType};
use crate::core::models::ids::*;
use iso_currency::CurrencySymbol;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Duration};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CellValue {
    id: CellId,
    created_at: Datetime,
    updated_at: Datetime,
    value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Value {
    SingleLine(SingleLineValue),
    LongText(LongTextValue),
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
    LookUp(LookUpValue),
    RollUp(RollUpValue),
}

// so for rollups and lookups, its a bit tricky, we have the field id, but we also have to update
// it, so the solution is that we use surrealdbs event/trigger stuff to automaticly update the data
// to the tables

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

    pub fn value(&self) -> Value {
        *self.computed_values.clone()
    }

    pub fn function(&self) -> AggregateFunction {
        self.function.clone()
    }
    pub fn target_field_id(&self) -> FieldId {
        self.target_field_id.clone()
    }
    pub fn link_field_id(&self) -> FieldId {
        self.link_field_id.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    pub fn link_field_id(&self) -> FieldId {
        self.link_field_id.clone()
    }
    pub fn target_field_id(&self) -> FieldId {
        self.target_field_id.clone()
    }

    pub fn value(&self) -> Value {
        *self.computed_values.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkValue {
    pub target_table_id: TableId,
    pub record_ids: Vec<RecordId>,
    pub link_type: LinkType,
}

impl LinkValue {
    pub fn new(
        target_table_id: TableId,
        link_type: LinkType,
        record_ids: Option<Vec<RecordId>>,
    ) -> Self {
        let ids = record_ids.unwrap_or_default();

        let final_ids = if link_type == LinkType::OneToOne && ids.len() > 1 {
            vec![ids[0].clone()]
        } else {
            ids
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DurationValue {
    value: Duration,
}

impl DurationValue {
    pub fn new(value: Option<Duration>) -> Self {
        if let Some(v) = value {
            DurationValue { value: v }
        } else {
            DurationValue {
                value: Duration::new(0, 0),
            }
        }
    }

    pub fn value(&self) -> Duration {
        self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DateValue {
    value: Datetime,
}

impl DateValue {
    pub fn new(value: Option<Datetime>) -> Self {
        if let Some(v) = value {
            DateValue { value: v }
        } else {
            DateValue {
                value: Datetime::from(chrono::Utc::now()),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RatingValue {
    value: u8,
}

impl RatingValue {
    pub fn new(value: Option<u8>, max: u8) -> Self {
        let mut ratings;
        if let Some(v) = value {
            ratings = v;
        } else {
            ratings = 0;
        };
        if ratings > max {
            ratings = max;
        }
        Self { value: ratings }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PercentValue {
    value: i32,
}

impl PercentValue {
    pub fn new(value: i32) -> Self {
        PercentValue { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CurrencyValue {
    value: i64,
    currency_symbole: CurrencySymbol,
}

impl CurrencyValue {
    pub fn new(value: i64, currency_symbole: CurrencySymbol) -> Self {
        CurrencyValue {
            value,
            currency_symbole,
        }
    }
    pub fn value_as_int(&self) -> i64 {
        self.value
    }
    pub fn value_as_str(&self) -> String {
        format!("{} {}", self.value, self.currency_symbole.symbol)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DecimalValue {
    value: OrderedFloat<f32>,
}

impl DecimalValue {
    pub fn new(value: Option<f32>, default: Option<f32>) -> Result<Self, String> {
        if value.is_none() && default.is_none() {
            return Err("BOTH_VALUES_ARE_NONE".to_string());
        };
        if let Some(v) = value {
            Ok(DecimalValue {
                value: OrderedFloat::from(v),
            })
        } else {
            Ok(DecimalValue {
                value: OrderedFloat::from(default.unwrap_or(0.0)),
            })
        }
    }

    pub fn value(&self) -> &OrderedFloat<f32> {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NumberValue {
    value: usize,
}

impl NumberValue {
    pub fn new(value: Option<usize>, default: Option<usize>) -> Result<Self, String> {
        if value.is_none() && default.is_none() {
            return Err("BOTH_VALUES_ARE_NONE".to_string());
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PhoneValue {
    value: String,
}

impl PhoneValue {
    pub fn new(value: String, default_region: Option<&str>) -> Result<Self, String> {
        let region = default_region.and_then(|r| r.parse().ok());

        match phonenumber::parse(region, &value) {
            Ok(phone) => {
                if phonenumber::is_valid(&phone) {
                    let formatted = phone.format().mode(phonenumber::Mode::E164).to_string();
                    Ok(Self { value: formatted })
                } else {
                    Err("INVALID_PHONE_NUMBER".to_string())
                }
            }
            Err(_) => Err("UNPARSEABLE_PHONE_NUMBER".to_string()),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UrlValue {
    value: String,
}

impl UrlValue {
    pub fn new(value: String) -> Result<Self, String> {
        if validator::ValidateUrl::validate_url(&value) {
            Ok(Self {
                value: value.trim().to_string(),
            })
        } else {
            Err("INVALID_URL_FORMAT".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, String> {
        if validator::ValidateEmail::validate_email(&value) {
            Ok(Self {
                value: value.trim().to_lowercase(),
            })
        } else {
            Err("INVALID_EMAIL_FORMAT".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LongTextValue {
    value: String,
}

impl LongTextValue {
    pub fn new(value: String, rich_text: bool) -> Self {
        let processed = if rich_text {
            value.trim().to_string()
        } else {
            value
                .replace(['*', '_', '#', '`', '[', ']'], "")
                .trim()
                .to_string()
        };
        Self { value: processed }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SingleLineValue {
    value: String,
}

impl SingleLineValue {
    pub fn new(default: Option<String>, max_length: u16, value: Option<String>) -> Self {
        let raw = value.or(default).unwrap_or_default();
        let single_line = raw.replace(['\n', '\r'], " ");

        // Unicode-safe truncation
        let adapted = single_line
            .chars()
            .take(max_length as usize)
            .collect::<String>();

        Self {
            value: adapted.trim().to_string(),
        }
    }
}
