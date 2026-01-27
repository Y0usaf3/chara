DEFINE TABLE table SCHEMAFULL;

DEFINE FIELD base ON table TYPE record(base);  -- reference to the Base
DEFINE FIELD name ON table TYPE string;
DEFINE FIELD "order" ON table TYPE int;
DEFINE FIELD created_at ON table TYPE datetime;
DEFINE FIELD updated_at ON table TYPE datetime;

DEFINE INDEX base_idx ON table COLUMNS base;
DEFINE INDEX name_idx ON table COLUMNS name;

DEFINE SCOPE user_scope FOR TABLE table
    PERMIT select WHERE base.workspace.owner = $auth.id
    PERMIT create WHERE base.workspace.owner = $auth.id
    PERMIT update WHERE base.workspace.owner = $auth.id;

