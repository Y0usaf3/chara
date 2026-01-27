-- WorkspaceUserRole ENUM
DEFINE ENUM WorkspaceUserRole
    [ 'guest', 'user', 'admin' ];


-- Table to store fine-grained (per-table) permissions, mapping TableId -> TablePermissions bitmask
DEFINE TABLE user_permissions SCHEMALESS
    PERMISSIONS
        FOR select, update, delete WHERE $auth.id = user_id
        FOR create WHERE $auth.id = user_id;

-- TablePermissions is stored as a u32 per-table in the tables HashMap
-- Bases is a placeholder (u32 for now)

-- Table to store workspace membership data
DEFINE TABLE workspace_user SCHEMALESS
    PERMISSIONS
        FOR select, update, delete WHERE $auth.id = user_id
        FOR create WHERE $auth.id = user_id;

-- Fields:
--   workspace_id: record id to workspace
--   user_id: record id to user
--   joined_at: datetime
--   last_modified: datetime
--   username: optional string
--   role: enum WorkspaceUserRole
--   permissions: field (UserPermissions object)
--   invited_by: user id (nullable)

-- Example object structure:
-- {
--    workspace_id: <workspace record id>,
--    user_id: <user record id>,
--    joined_at: <datetime>,
--    last_modified: <datetime>,
--    username: <option<string>>,
--    role: <WorkspaceUserRole>,
--    permissions: {
--       tables: {
--         <table_id>: <u32 TablePermissions bitmask>,
--         ...
--       },
--       bases: {
--         <base_id>: <u32>,
--         ...
--       }
--    },
--    invited_by: <option<user id>>
-- }

DEFINE FIELD workspace_id ON TABLE workspace_user TYPE record;
DEFINE FIELD user_id      ON TABLE workspace_user TYPE record;
DEFINE FIELD joined_at    ON TABLE workspace_user TYPE datetime;
DEFINE FIELD last_modified ON TABLE workspace_user TYPE datetime;
DEFINE FIELD username     ON TABLE workspace_user TYPE option<string>;
DEFINE FIELD role         ON TABLE workspace_user TYPE WorkspaceUserRole;
DEFINE FIELD permissions  ON TABLE workspace_user TYPE object;
DEFINE FIELD invited_by   ON TABLE workspace_user TYPE option<record>;

-- You may define indexes as needed
DEFINE INDEX idx_workspace_user_user_id ON TABLE workspace_user COLUMNS user_id;
DEFINE INDEX idx_workspace_user_workspace_id ON TABLE workspace_user COLUMNS workspace_id;
