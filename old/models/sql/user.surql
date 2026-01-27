DEFINE TABLE user SCHEMAFULL;

DEFINE FIELD created_at ON user TYPE datetime;
DEFINE FIELD updated_at ON user TYPE datetime;
DEFINE FIELD email ON user TYPE string;
DEFINE FIELD first_name ON user TYPE string;
DEFINE FIELD last_name ON user TYPE string;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD is_deleted ON user TYPE bool;

DEFINE FIELD role ON user
    TYPE string
    ASSERT $value IN ["user", "admin"];

DEFINE INDEX email_idx ON user COLUMNS email UNIQUE;
DEFINE INDEX username_idx ON user COLUMNS username UNIQUE;

-- so only the user can select itself, and the admin, but only the admin can change the data
DEFINE SCOPE user_scope FOR TABLE user
    PERMIT select WHERE id = $auth.id OR $auth.role = "admin"
    PERMIT update WHERE $auth.role = "admin";

