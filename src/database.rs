use sqlx::PgPool;
use sqlx::Row;

use crate::router::Login;

pub struct DB {}

impl DB { 
    pub async fn get_user(email: String, dbconn: PgPool) -> Result<Login, sqlx::Error> {
        let meme = sqlx::query("SELECT email, password FROM users WHERE username = $1")
            .bind(email)
            .fetch_one(&dbconn);
    
        match meme.await {
            Ok(result) => {
                let meme = Login {
                    email: result.get("email"),
                    password: result.get("password"),
                };
    
                Ok(meme)
            }
            Err(err) => Err(err),
        }
    }
    
    pub async fn create_user(
        email: String,
        password: String,
        dbconn: PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
            .bind(email)
            .bind(password)
            .execute(&dbconn)
            .await?;
    
        Ok(())
    }
}