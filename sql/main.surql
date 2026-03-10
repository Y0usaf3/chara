-- PERM FUNCTION
-- ADMIN == 1 
-- an admin can do anything! muhehe
DEFINE FUNCTION OVERWRITE fn::can($mask: int, $flag: int) {
    RETURN (fn::bitwise::and($mask, 1) == 1) OR (fn::bitwise::and($mask, $flag) == $flag);
};

-- copy pasted from @EllisVerhoest in his issue msg "https://github.com/surrealdb/surrealdb/issues/1335"
DEFINE FUNCTION OVERWRITE fn::bitwise::and($a: number, $b: number) {
    RETURN function($a, $b) { return arguments[0] & arguments[1] }
};

-- USER 
DEFINE TABLE OVERWRITE user SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE user TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE user TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE user TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE first_name ON TABLE user TYPE string PERMISSIONS FOR select WHERE id = $user;
DEFINE FIELD OVERWRITE last_name ON TABLE user TYPE string PERMISSIONS FOR select WHERE id = $user;
DEFINE FIELD OVERWRITE email ON TABLE user TYPE string
ASSERT string::is_email($value);
DEFINE FIELD OVERWRITE role ON TABLE user TYPE string 
DEFAULT 'user'
ASSERT $value INSIDE ['admin', 'user'];

-- SESSION 
DEFINE TABLE OVERWRITE session SCHEMAFULL;
-- PERMISSIONS 
--     FOR select, update, delete NONE;
DEFINE FIELD OVERWRITE token ON TABLE session TYPE string
    VALUE  crypto::argon2::generate($value);
DEFINE FIELD OVERWRITE ip ON TABLE session TYPE string
    ASSERT string::is_ip($value);
DEFINE FIELD OVERWRITE user_agent ON TABLE session TYPE string;
DEFINE FIELD OVERWRITE created_at ON TABLE session TYPE datetime
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON TABLE session TYPE datetime
VALUE time::now() + 2d READONLY;
DEFINE FIELD OVERWRITE user ON TABLE session TYPE record<user>;
DEFINE INDEX OVERWRITE unique ON TABLE session 
    FIELDS user, token, user_agent UNIQUE;

-- IDENTITY 
DEFINE TABLE OVERWRITE identity SCHEMAFULL
PERMISSIONS 
    FOR select, update, delete NONE;
DEFINE FIELD OVERWRITE created_at ON TABLE identity TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE identity TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE identity TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE user ON TABLE identity TYPE record<user>;
DEFINE FIELD OVERWRITE external_id ON TABLE identity TYPE string;
DEFINE FIELD OVERWRITE access_token ON TABLE identity TYPE string;
DEFINE FIELD OVERWRITE refresh_token ON TABLE identity TYPE string;
DEFINE FIELD OVERWRITE expires_at ON TABLE identity TYPE datetime;
DEFINE INDEX OVERWRITE user_unique ON TABLE identity FIELDS user UNIQUE;
DEFINE INDEX OVERWRITE external_id_unique ON TABLE identity FIELDS external_id UNIQUE;

-- WORKSPACE 
DEFINE TABLE OVERWRITE workspace SCHEMAFULL
PERMISSIONS
  -- Use a custom variable $user instead of $auth
  FOR select WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perms FROM can_access_workspace WHERE in.user = $user AND out = $this.id)[0], 
      2
  )
  FOR update WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perms FROM can_access_workspace WHERE in.user = $user AND out = $this.id)[0], 
      4
  );
DEFINE FIELD OVERWRITE created_at ON TABLE workspace TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE workspace TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE workspace TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE owner ON TABLE workspace TYPE record<user>;
DEFINE FIELD OVERWRITE name ON TABLE workspace TYPE string;
DEFINE INDEX OVERWRITE unique_name_per_owner ON TABLE workspace FIELDS name, owner UNIQUE;

-- WORKSPACE USER 
DEFINE TABLE OVERWRITE workspace_user SCHEMAFULL
PERMISSIONS FOR select, update 
WHERE is_deleted = false AND user = $user
OR is_deleted = false AND fn::bitwise::and(
    (SELECT VALUE perm FROM can_access_workspace 
      WHERE in.user = $user 
      AND out = $this.workspace_id
    )[0], 1
) == 1;
DEFINE FIELD OVERWRITE created_at ON TABLE workspace_user TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE workspace_user TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE workspace_user TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE workspace_id ON TABLE workspace_user TYPE record<workspace>;
DEFINE FIELD OVERWRITE username ON TABLE workspace_user TYPE option<string> 
VALUE $value OR user.first_name;
DEFINE FIELD OVERWRITE user ON TABLE workspace_user TYPE record<user>;
DEFINE INDEX OVERWRITE unique_user_per_workspace 
    ON TABLE workspace_user 
    FIELDS user, workspace_id UNIQUE;

-- BASE 
DEFINE TABLE OVERWRITE base SCHEMAFULL
PERMISSIONS
  -- View (1 << 1) 
  FOR select WHERE is_deleted = false AND 
  fn::can(
      (SELECT VALUE perm FROM can_access_base WHERE in.user = $user AND out = $this.id)[0], 
      2
  )
  -- Edit (1 << 2)
  FOR update WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perm FROM can_access_base WHERE in.user = $user AND out = $this.id)[0], 
      4
  );
DEFINE FIELD OVERWRITE created_at ON TABLE base TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE base TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE base TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE name ON TABLE base TYPE string;

-- TABLE 
DEFINE TABLE OVERWRITE table SCHEMAFULL
PERMISSIONS
  -- View (1 << 1) 
  FOR select WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perm FROM can_access_table WHERE in.user = $user AND out = $this.id)[0], 
      2
  )
  -- Edit (1 << 2)
  FOR update WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perm FROM can_access_table WHERE in.user = $user AND out = $this.id)[0], 
      4
  );
DEFINE FIELD OVERWRITE created_at ON TABLE table TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE table TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE table TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE name ON TABLE table TYPE string;

-- WORKSPACE |-relation-> BASE 
DEFINE TABLE OVERWRITE have_base TYPE RELATION 
    IN workspace 
    OUT base 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_base TYPE datetime
    VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_public ON TABLE have_base TYPE bool;

-- BASE |-relation-> TABLE
DEFINE TABLE OVERWRITE have_table TYPE RELATION 
    IN base
    OUT table
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_table TYPE datetime
    VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE is_public ON TABLE have_table TYPE bool;

-- FIELD 
DEFINE TABLE OVERWRITE field SCHEMAFULL
PERMISSIONS
  -- View (1 << 1) 
  FOR select WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perm FROM can_access_field WHERE in.user = $user AND out = $this.id)[0], 
      2
  )
  -- Edit (1 << 2)
  FOR update WHERE is_deleted = false AND
  fn::can(
      (SELECT VALUE perm FROM can_access_field WHERE in.user = $user AND out = $this.id)[0], 
      4
  );
DEFINE FIELD OVERWRITE created_at ON TABLE field TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE field TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE field TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE is_primary ON TABLE field TYPE bool
DEFAULT true;
DEFINE FIELD OVERWRITE is_nullable ON TABLE field TYPE bool;
DEFINE FIELD OVERWRITE is_unique ON TABLE field TYPE bool;
DEFINE FIELD OVERWRITE name ON TABLE field TYPE string;
DEFINE FIELD OVERWRITE order ON TABLE field TYPE int;
DEFINE FIELD OVERWRITE description ON TABLE field TYPE option<string>;
DEFINE FIELD OVERWRITE config ON TABLE field TYPE object;

-- RECORD 
DEFINE TABLE OVERWRITE record SCHEMAFULL
PERMISSIONS
    FOR select WHERE is_deleted = false AND
    fn::can(
        (SELECT VALUE perms FROM can_access_table WHERE in.user = $user AND out = $this.table)[0],
        2
    )
    FOR update WHERE is_deleted = false AND
    fn::can(
        (SELECT VALUE perms FROM can_access_table WHERE in.user = $user AND out = $this.id.table)[0],
        4
    );
DEFINE FIELD OVERWRITE created_at ON TABLE record TYPE datetime 
VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON TABLE record TYPE datetime
VALUE time::now();
DEFINE FIELD OVERWRITE is_deleted ON TABLE record TYPE bool 
DEFAULT false;
DEFINE FIELD OVERWRITE cells ON TABLE record TYPE object FLEXIBLE
ASSERT type::is_object($value);
DEFINE FIELD OVERWRITE table ON TABLE record TYPE record<table>;

-- TABLE |-relation-> FIELD
DEFINE TABLE OVERWRITE have_field TYPE RELATION 
    IN table
    OUT field
    SCHEMAFULL;
DEFINE FIELD OVERWRITE created_at ON TABLE have_field TYPE datetime
    VALUE time::now() READONLY;
DEFINE INDEX OVERWRITE table_fields_idx ON TABLE have_field FIELDS in, out UNIQUE;

-- TABLE |-relation-> RECORD 
-- DEFINE TABLE OVERWRITE have_record TYPE RELATION 
--    IN table
--    OUT record
--    SCHEMAFULL;
-- DEFINE FIELD OVERWRITE created_at ON TABLE have_record TYPE datetime
--    VALUE time::now() READONLY;

-- WORKSPACE_USER |-relation-> WORKSPACE
DEFINE TABLE OVERWRITE can_access_workspace TYPE RELATION 
    IN workspace_user 
    OUT workspace 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE perms ON TABLE can_access_workspace TYPE int;
DEFINE INDEX OVERWRITE user_workspace_idx ON TABLE can_access_workspace FIELDS in, out UNIQUE;
DEFINE INDEX OVERWRITE workspace_users_reverse_idx ON TABLE can_access_workspace FIELDS out, in UNIQUE; 
--     ↑ this just for admin panels when we want to access all the users who have access to the workspace 

-- WORKSPACE_USER |-relation-> BASE
DEFINE TABLE OVERWRITE can_access_base TYPE RELATION 
    IN workspace_user 
    OUT base 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE perms ON TABLE can_access_base TYPE int;
DEFINE INDEX OVERWRITE user_base_idx ON TABLE can_access_base FIELDS in, out UNIQUE;

-- WORKSPACE_USER |-relation-> TABLE
DEFINE TABLE OVERWRITE can_access_table TYPE RELATION 
    IN workspace_user 
    OUT table 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE perms ON TABLE can_access_table TYPE int;
DEFINE INDEX OVERWRITE user_table_idx ON TABLE can_access_table FIELDS in, out UNIQUE;

-- WORKSPACE_USER |-relation-> FIELD
DEFINE TABLE OVERWRITE can_access_field TYPE RELATION 
    IN workspace_user 
    OUT field 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE perms ON TABLE can_access_field TYPE int;
DEFINE INDEX OVERWRITE user_field_idx ON TABLE can_access_field FIELDS in, out UNIQUE;

-- WORKSPACE_USER |-relation-> WORKSPACE_USER
DEFINE TABLE OVERWRITE can_access_workspace_user TYPE RELATION 
    IN workspace_user 
    OUT workspace_user 
    SCHEMAFULL;
DEFINE FIELD OVERWRITE perms ON TABLE can_access_workspace_user TYPE int;
DEFINE INDEX OVERWRITE user_workspace_user_idx ON TABLE can_access_workspace_user FIELDS in, out UNIQUE;

