use sqlx::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct Sessions {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub created_at: String,
    pub expires_at: String
}