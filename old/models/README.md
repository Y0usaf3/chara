all the tables :p
yeah this gave me headache

User
├─ id: string (auto)
├─ created_at: datetime
├─ updated_at: datetime
├─ email: string
├─ first_name: string
├─ last_name: string
└─ username: string

Workspace
├─ id: string (auto)
├─ owner: record(user)
├─ name: string
├─ created_at: datetime
└─ updated_at: datetime

Base
├─ id: string (auto)
├─ workspace: record(workspace)
├─ name: string
├─ created_at: datetime
└─ updated_at: datetime

Table
├─ id: string (auto)
├─ base: record(base)
├─ name: string
├─ order: int
├─ created_at: datetime
└─ updated_at: datetime

Field
├─ id: string (auto)
├─ table: record(table)
├─ name: string
├─ field_type: JSON (Text, Number, Bool, Date, Select, Relation)
├─ order: int
├─ created_at: datetime
└─ updated_at: datetime

Record
├─ id: string (auto)
├─ table: record(table)
├─ created_at: datetime
└─ updated_at: datetime

Cell
├─ id: string (auto)
├─ record: record(record)
├─ field: record(field)
├─ value: JSON
├─ created_at: datetime
└─ updated_at: datetime

Identity
├─ id: string (auto)
├─ user: record(user)
├─ external_user_id: string
└─ created_at: datetime

Relation
├─ id: string (auto)
├─ from_record: record(record)
├─ to_record: record(record)
├─ field: record(field)
├─ created_at: datetime
└─ updated_at: datetime
