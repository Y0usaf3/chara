DEFINE TABLE relation SCHEMAFULL;

DEFINE FIELD from_record ON relation TYPE record(record);  -- reference to the source record
DEFINE FIELD to_record ON relation TYPE record(record);    -- reference to the target record
DEFINE FIELD field ON relation TYPE record(field);         -- reference to the field defining this relation
DEFINE FIELD created_at ON relation TYPE datetime;
DEFINE FIELD updated_at ON relation TYPE datetime;

DEFINE INDEX from_record_idx ON relation COLUMNS from_record;
DEFINE INDEX to_record_idx ON relation COLUMNS to_record;
DEFINE INDEX field_idx ON relation COLUMNS field;

DEFINE SCOPE user_scope FOR TABLE relation
    PERMIT select WHERE from_record.table.workspace.owner = $auth.id
    PERMIT create WHERE from_record.table.workspace.owner = $auth.id
    PERMIT update WHERE from_record.table.workspace.owner = $auth.id;

