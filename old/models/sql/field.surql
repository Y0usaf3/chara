DEFINE TABLE field SCHEMAFULL;

DEFINE FIELD table ON field TYPE record(base);  
DEFINE FIELD name ON field TYPE string;
DEFINE FIELD field_type ON field TYPE string;  -- store serialized FieldType 
DEFINE FIELD "order" ON field TYPE int;
DEFINE FIELD created_at ON field TYPE datetime;
DEFINE FIELD updated_at ON field TYPE datetime;

DEFINE INDEX table_idx ON field COLUMNS table;
DEFINE INDEX name_idx ON field COLUMNS name;

DEFINE SCOPE user_scope FOR TABLE field
    PERMIT select WHERE table.workspace.owner = $auth.id
    PERMIT create WHERE table.workspace.owner = $auth.id
    PERMIT update WHERE table.workspace.owner = $auth.id;

