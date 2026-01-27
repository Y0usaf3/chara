DEFINE TABLE workspace SCHEMAFULL;

DEFINE FIELD owner ON workspace TYPE record(user);
DEFINE FIELD name ON workspace TYPE string;
DEFINE FIELD created_at ON workspace TYPE datetime;
DEFINE FIELD updated_at ON workspace TYPE datetime;

DEFINE INDEX owner_idx ON workspace COLUMNS owner;
DEFINE INDEX name_idx ON workspace COLUMNS name;

DEFINE SCOPE user_scope FOR TABLE workspace
    PERMIT select WHERE owner = $auth.id
    PERMIT create WHERE owner = $auth.id
    PERMIT update WHERE owner = $auth.id;


DEFINE TABLE workspace_member SCHEMAFULL;

DEFINE FIELD user ON workspace_member TYPE record<user>;
DEFINE FIELD workspace ON workspace_member TYPE record<workspace>;
DEFINE FIELD permissions ON workspace_member TYPE object;

DEFINE INDEX unique_membership
ON workspace_member
FIELDS user, workspace
UNIQUE;
// so i think working all on the truct is a good idea and only stor a list of permissions, with ofc id etc
// also damn i cant see a single text in this luminosity
