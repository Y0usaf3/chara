use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use iso_currency::{Currency, CurrencySymbol};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FieldConfig {
    Text(TextConfig),
    Number(NumberConfig),
    Select(SelectConfig),
    Datetime(DatetimeConfig),
    Relation(RelationConfig),
    User(UserConfig),
    Computed(ComputedTypes),
    Custom(CustomConfig),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TextConfig {
    SingleLine {
        default: Option<String>,
        max_length: u16,
    },
    LongText {
        rich_text: bool,
    },
    Email,
    URL,
    Phone,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NumberConfig {
    Number {
        default: Option<usize>,
    },
    Decimal {
        default: Option<f32>,
        precision: u8,
    },
    Currency {
        currency: Currency,
        precision: usize, /* 0 - 8*/
    },
    Percent {
        precision: usize, /* 0 - 8 */
        show_bar: bool,
    },
    Rating {
        max: usize, /* 0- 10 */
        icon_type: RatingIcon,
        color: [u8; 6], // thats enough for colors
    },
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RatingIcon {
    Star,
    Heart,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub id: u8,
    pub label: String,
    pub color: [u8; 6],
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SelectConfig {
    Single { options: Vec<SelectOption> },
    Multi { options: Vec<SelectOption> },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DateFormat {
    ISO,
    US,
    EU,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TimeUnits {
    Sec,
    Min,
    Hour,
    Day,
    Week,
    Month,
    Year,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DatetimeConfig {
    Date {
        format: DateFormat,
        include_time: bool,
    },
    Duration {
        unit: TimeUnits,
        format: DateFormat,
    },
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Count,
    Max,
    Min,
    Avg,
    Sum,
    CountDistinct,
    SumDistinct,
    AvgDistinct,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationConfig {
    Link {
        target_table_id: TableId,
        r#type: LinkType,
        inverse_field_id: FieldId,
    },
    LookUp {
        link_field_id: FieldId,
        target_field_id: FieldId,
    },
    RollUp {
        link_field_id: FieldId,
        target_field_id: FieldId,
        functions: AggregateFunction,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserConfig {
    User { is_multi: bool, notify: bool },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComputedTypes {
    Formula { expression: String },
    CreatedAt { format: DateFormat },
    ModifiedTime { format: DateFormat },
    AutoNumber { prefix: Prefix, start_at: usize },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Prefix {
    Dot,
    Star,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomConfig {
    Attachment,
    JSON,
}
