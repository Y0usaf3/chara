DEFINE TABLE cell SCHEMAFULL;

DEFINE FIELD record ON cell TYPE record(record);  -- reference to a Record
DEFINE FIELD field ON cell TYPE record(field);    -- reference to a Field
DEFINE FIELD value ON cell TYPE string;           -- store JSON as string
DEFINE FIELD created_at ON cell TYPE datetime;
DEFINE FIELD updated_at ON cell TYPE datetime;

DEFINE INDEX record_idx ON cell COLUMNS record;
DEFINE INDEX field_idx ON cell COLUMNS field;

DEFINE SCOPE user_scope FOR TABLE cell
    PERMIT select WHERE record.base.workspace.owner = $auth.id
    PERMIT create WHERE record.base.workspace.owner = $auth.id
    PERMIT update WHERE record.base.workspace.owner = $auth.id;

