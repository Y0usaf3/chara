DEFINE TABLE session SCHEMAFULL;
DEFINE FIELD user ON session
    TYPE record(user)
    ASSERT $value != NONE;

DEFINE FIELD token ON session
    TYPE string
    ASSERT string::len($value) >= 32;

DEFINE FIELD ip ON session
    TYPE string;

DEFINE FIELD user_agent ON session
    TYPE string;

DEFINE FIELD created_at ON session
    TYPE datetime
    VALUE time::now();

DEFINE FIELD expires_at ON session
    TYPE datetime
    ASSERT $value > time::now();

DEFINE FIELD max_expires_at ON session
    TYPE datetime;

-- token lookup (auth-critical)
DEFINE INDEX session_token_unique
ON session
COLUMNS token
UNIQUE;

-- user —> sessions
DEFINE INDEX session_user_idx
ON session
COLUMNS user;

DEFINE SCOPE user_scope FOR TABLE session

    -- users can see only their own sessions
    PERMIT select
        WHERE user = $auth.id
          AND expires_at > time::now()

    -- users can delete only their own sessions, logout basically
    PERMIT delete
        WHERE user = $auth.id

    -- users CANNOT create sessions themselves
    PERMIT create NONE

    -- users CANNOT update sessions directly
    PERMIT update NONE
;

-- hardening stuff ig
ASSERT ip != NONE;
ASSERT user_agent != NONE;
ASSERT expires_at <= max_expires_at;

