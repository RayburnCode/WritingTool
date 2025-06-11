use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenClaims {
    pub sub: Uuid,       // User ID
    pub jti: Uuid,       // Session ID
    pub exp: usize,      // Expiration time
    pub iat: usize,      // Issued at
}

pub struct SessionService {
    db: Database,
    jwt_secret: String,
    session_duration: chrono::Duration,
}

impl SessionService {
    pub fn new(db: Database, jwt_secret: String) -> Self {
        Self {
            db,
            jwt_secret,
            session_duration: chrono::Duration::days(30),
        }
    }

    pub async fn create_session(
        &self,
        user: &User,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<(Session, String), anyhow::Error> {
        let session_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + self.session_duration;
        
        // Create JWT token
        let claims = SessionTokenClaims {
            sub: user.id,
            jti: session_id,
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        // Store session in database
        let session = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (id, user_id, token, user_agent, ip_address, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            session_id,
            user.id,
            token,
            user_agent,
            ip_address,
            expires_at
        )
        .fetch_one(&self.db.pool)
        .await?;
        
        Ok((session, token))
    }

    pub async fn validate_session(
        &self,
        token: &str,
    ) -> Result<(Session, User), anyhow::Error> {
        // Verify JWT token
        let token_data = decode::<SessionTokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )?;
        
        let claims = token_data.claims;
        
        // Check if session exists in database
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT * FROM sessions
            WHERE id = $1 AND token = $2 AND expires_at > NOW()
            "#,
            claims.jti,
            token
        )
        .fetch_one(&self.db.pool)
        .await?;
        
        // Get associated user
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id = $1
            "#,
            claims.sub
        )
        .fetch_one(&self.db.pool)
        .await?;
        
        // Update session's last activity
        sqlx::query!(
            r#"
            UPDATE sessions
            SET updated_at = NOW()
            WHERE id = $1
            "#,
            session.id
        )
        .execute(&self.db.pool)
        .await?;
        
        Ok((session, user))
    }

    pub async fn delete_session(&self, session_id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
            session_id
        )
        .execute(&self.db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete_all_user_sessions(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.db.pool)
        .await?;
        
        Ok(())
    }
}