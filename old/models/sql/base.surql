DEFINE TABLE base SCHEMAFULL;

DEFINE FIELD workspace ON base TYPE record(workspace);
DEFINE FIELD name ON base TYPE string;
DEFINE FIELD created_at ON base TYPE datetime;
DEFINE FIELD updated_at ON base TYPE datetime;

DEFINE INDEX workspace_idx ON base COLUMNS workspace;
DEFINE INDEX name_idx ON base COLUMNS name;

DEFINE SCOPE user_scope FOR TABLE base
    PERMIT select WHERE workspace.owner = $auth.id
    PERMIT create WHERE workspace.owner = $auth.id
    PERMIT update WHERE workspace.owner = $auth.id;

