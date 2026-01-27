DEFINE TABLE record SCHEMAFULL;

DEFINE FIELD table ON record TYPE record(base);  -- reference to Base/Table
DEFINE FIELD created_at ON record TYPE datetime;
DEFINE FIELD updated_at ON record TYPE datetime;

DEFINE INDEX table_idx ON record COLUMNS table;

DEFINE SCOPE user_scope FOR TABLE record
    PERMIT select WHERE table.workspace.owner = $auth.id
    PERMIT create WHERE table.workspace.owner = $auth.id
    PERMIT update WHERE table.workspace.owner = $auth.id;

