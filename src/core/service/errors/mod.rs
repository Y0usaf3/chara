use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid or expired authentication token")]
    InvalidToken,

    #[error("Authentication token has expired")]
    TokenExpired,

    #[error("Failed to verify authentication credentials")]
    VerificationFailed,

    #[error("Broken or malformed authentication response")]
    BrokenAuthResponse,

    #[error("Session token does not exist")]
    SessionNotFound,

    #[error("Session has expired")]
    SessionExpired,

    #[error("IP address mismatch with session")]
    IpMismatch,

    #[error("User agent mismatch with session")]
    UserAgentMismatch,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User does not exist")]
    NotFound,

    #[error("User already exists")]
    AlreadyExists,

    #[error("User account has been deleted")]
    Deleted,

    #[error("Failed to update user: {0}")]
    UpdateFailed(String),

    #[error("Cannot delete self")]
    CannotDeleteSelf,
}

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("Insufficient permissions for this operation")]
    Insufficient,

    #[error("Admin role required")]
    AdminRequired,

    #[error("Workspace owner permissions required")]
    OwnerRequired,

    #[error("User lacks permission '{0}' in this workspace")]
    MissingPermission(String),
}

#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Workspace does not exist")]
    NotFound,

    #[error("Workspace has been deleted")]
    Deleted,

    #[error("Cannot delete workspace: {0}")]
    DeletionFailed(String),

    #[error("Workspace name already exists")]
    NameTaken,

    #[error("Failed to create workspace user relationship")]
    UserRelationshipFailed,
}

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Failed to encrypt data")]
    EncryptionFailed,

    #[error("Failed to decrypt data")]
    DecryptionFailed,

    #[error("Invalid encryption key")]
    InvalidKey,

    #[error("Invalid or missing nonce")]
    InvalidNonce,

    #[error("Encryption authentication failed - data may be corrupted")]
    AuthenticationFailed,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database query failed: {0}")]
    QueryFailed(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Data serialization/deserialization error")]
    SerializationError,

    #[error("Invalid record ID")]
    InvalidRecordId,

    #[error("Duplicate entry in database")]
    DuplicateEntry,
}
