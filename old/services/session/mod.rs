use crate::models::session::Session;
use crate::services::Error;
use crate::services::DB;

pub async fn get_session_with_token(session_token: String) -> Result<Session, Error> {
    DB.query("SELECT * FROM session WHERE token = $token AND expires_at > time::now()")
        .bind(("token", session_token))
        .await?
        .take::<Option<Session>>(0)?
        .ok_or(Error::Db)
}
