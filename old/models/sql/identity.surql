DEFINE TABLE identity SCHEMAFULL;

DEFINE FIELD user ON identity TYPE record(user);
DEFINE FIELD external_user_id ON identity TYPE string;
DEFINE FIELD created_at ON identity TYPE datetime;

DEFINE INDEX user_idx ON identity COLUMNS user;
DEFINE INDEX external_user_idx ON identity COLUMNS external_user_id;

DEFINE SCOPE user_scope FOR TABLE identity
    PERMIT select WHERE user = $auth.id OR $auth.role = "admin"
    PERMIT create WHERE user = $auth.id 
    PERMIT update WHERE user = $auth.id OR $auth.role = "admin";

