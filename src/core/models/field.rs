use crate::core::models::ids::*;
use ::serde::{Deserialize, Serialize};
use iso_currency::Currency;
use surrealdb::sql::Datetime;

// alr, so we got a LOT of field types, wich makes migrations etc a bit tricky
// my guess is that we allow migration between the fiel kinds, we got
// - Text based
// - numerical
// - select based (like a list and we only select from this list)
// - date time based
// - user based (a list of users in the current workspace)
// there is exeption for custom types (attachement, checkbox, qrcode or even json) those cant be
// migrated to other type (like from check box to qrcode) not like text based field kinds, that can
// migrate from formated text to non formatted text.
// each field kind requires different type of migration wich will be a lil tricky

pub enum FieldKind {
    Text(TextBased),
    Number(Numerical),
    Select(SelectBased),
    Relation(LinksBased),
    DateTime(DateTimeBased),
    User(UserBased),
    Custom(CustomTypes),
}

pub enum TextBased {
    SingleLineText,
    LongText,
    RichText,
    Email,
    PhoneNumber,
    Url,
}

pub enum Numerical {
    Number,
    Decimal { precision: u8, scale: u8 },
    Percent { scale: u8 },
    Currency { currency: Currency },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
}

pub enum SelectBased {
    SingleSelect { types: Vec<SelectOption> },
    MultipleSelect { types: Vec<SelectOption> },
}

pub enum RelationType {
    ManyToMany,
    HasMany,
    OneToOne,
}

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

pub enum LinksBased {
    Link {
        relation: RelationType,
        foreign_table: TableId,
    },
    Lookup {
        link_field: FieldId,
        lookup_field: FieldId,
    },
    Rollup {
        link_field: FieldId,
        rollup_field: FieldId,
        aggregate: AggregateFunction,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Image,
    Video,
    Audio,
    Pdf,
    Document,
    Archive,
    Other(String),
}

pub enum RatingIcon {
    Star,
    Circle,
}

pub enum CustomTypes {
    Attachement { file_type: FileType },
    CheckBox,
    Rating { icon: RatingIcon, max: u8 },
    Json,
}

#[allow(clippy::upper_case_acronyms)]
pub enum DateFormat {
    ISO, // 2026-01-27
    US,  // MM/DD/YYYY
    EU,  // DD/MM/YYYY
    Custom(String),
}
pub enum TimeFormat {
    H24,
    H12,
}

pub enum DateTimeBased {
    Date { format: DateFormat },
    Time { format: TimeFormat },
    DateTime { date: DateFormat, time: TimeFormat },
    Duration,
    CreatedTime { date: DateFormat, time: TimeFormat },
    LastModifiedTime { date: DateFormat, time: TimeFormat },
}

pub enum UserBased {
    SingleUser,
    MultipleUsers,
}
