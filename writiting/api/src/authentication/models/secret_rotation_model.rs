use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use secrecy::{Secret, Zeroize};
use validator::Validate;

#[derive(Debug, Clone, FromRow, Validate)]
pub struct EncryptedSecret {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(skip_serializing)]
    pub current_value: Vec<u8>,
    #[serde(skip_serializing)]
    pub previous_value: Option<Vec<u8>>,
    pub encryption_key_id: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub rotated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// Custom serialization to protect secrets
#[derive(Debug, Serialize)]
pub struct EncryptedSecretResponse {
    pub id: Uuid,
    pub name: String,
    pub encryption_key_id: String,
    pub rotated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub is_rotated: bool,
}

impl From<EncryptedSecret> for EncryptedSecretResponse {
    fn from(secret: EncryptedSecret) -> Self {
        Self {
            id: secret.id,
            name: secret.name,
            encryption_key_id: secret.encryption_key_id,
            rotated_at: secret.rotated_at,
            created_at: secret.created_at,
            is_rotated: secret.previous_value.is_some(),
        }
    }
}

impl EncryptedSecret {
    /// Creates a new encrypted secret
    pub fn new(
        name: String,
        plaintext: &Secret<String>,
        encryption_key_id: &str,
        encryptor: &dyn SecretEncryptor,
    ) -> Result<Self, SecretError> {
        let current_value = encryptor.encrypt(plaintext, encryption_key_id)?;
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            current_value,
            previous_value: None,
            encryption_key_id: encryption_key_id.to_string(),
            rotated_at: Utc::now(),
            created_at: Utc::now(),
        })
    }

    /// Rotates the secret value
    pub fn rotate(
        &mut self,
        new_plaintext: &Secret<String>,
        new_key_id: &str,
        encryptor: &dyn SecretEncryptor,
    ) -> Result<(), SecretError> {
        let new_value = encryptor.encrypt(new_plaintext, new_key_id)?;
        
        self.previous_value = Some(std::mem::replace(&mut self.current_value, new_value));
        self.encryption_key_id = new_key_id.to_string();
        self.rotated_at = Utc::now();
        
        Ok(())
    }

    /// Decrypts the current value
    pub fn decrypt_current(&self, encryptor: &dyn SecretEncryptor) -> Result<Secret<String>, SecretError> {
        encryptor.decrypt(&self.current_value, &self.encryption_key_id)
    }

    /// Decrypts the previous value if exists
    pub fn decrypt_previous(&self, encryptor: &dyn SecretEncryptor) -> Option<Result<Secret<String>, SecretError>> {
        self.previous_value.as_ref().map(|v| 
            encryptor.decrypt(v, &self.encryption_key_id)
        )
    }

    /// Checks if secret needs rotation (based on policy)
    pub fn needs_rotation(&self, rotation_period_days: i64) -> bool {
        let rotation_period = chrono::Duration::days(rotation_period_days);
        Utc::now() > self.rotated_at + rotation_period
    }
}

// Zeroize implementation for secure memory cleanup
impl Zeroize for EncryptedSecret {
    fn zeroize(&mut self) {
        self.current_value.zeroize();
        if let Some(ref mut prev) = self.previous_value {
            prev.zeroize();
        }
        self.encryption_key_id.zeroize();
    }
}

// Encryption trait for different providers
pub trait SecretEncryptor: Send + Sync {
    fn encrypt(&self, plaintext: &Secret<String>, key_id: &str) -> Result<Vec<u8>, SecretError>;
    fn decrypt(&self, ciphertext: &[u8], key_id: &str) -> Result<Secret<String>, SecretError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Key not found")]
    KeyNotFound,
}

// Example KMS implementation
pub struct KmsEncryptor {
    client: aws_sdk_kms::Client,
}

impl KmsEncryptor {
    pub fn new(client: aws_sdk_kms::Client) -> Self {
        Self { client }
    }
}

impl SecretEncryptor for KmsEncryptor {
    fn encrypt(&self, plaintext: &Secret<String>, key_id: &str) -> Result<Vec<u8>, SecretError> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.client.encrypt()
                .key_id(key_id)
                .plaintext(plaintext.expose_secret().as_bytes())
                .send()
                .await
                .map_err(|_| SecretError::EncryptionFailed)
                .map(|res| res.ciphertext_blob.unwrap().into_inner())
        })
    }

    fn decrypt(&self, ciphertext: &[u8], key_id: &str) -> Result<Secret<String>, SecretError> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.client.decrypt()
                .key_id(key_id)
                .ciphertext_blob(ciphertext)
                .send()
                .await
                .map_err(|_| SecretError::DecryptionFailed)
                .and_then(|res| {
                    String::from_utf8(res.plaintext.unwrap().into_inner())
                        .map(Secret::new)
                        .map_err(|_| SecretError::DecryptionFailed)
                })
        })
    }
}

// Repository pattern for database operations
pub struct SecretRepository {
    pool: sqlx::PgPool,
    encryptor: Box<dyn SecretEncryptor>,
}

impl SecretRepository {
    pub fn new(pool: sqlx::PgPool, encryptor: Box<dyn SecretEncryptor>) -> Self {
        Self { pool, encryptor }
    }

    pub async fn create(
        &self,
        name: &str,
        plaintext: &Secret<String>,
        key_id: &str,
    ) -> Result<EncryptedSecretResponse, SecretError> {
        let secret = EncryptedSecret::new(
            name.to_string(),
            plaintext,
            key_id,
            &*self.encryptor,
        )?;

        sqlx::query!(
            r#"
            INSERT INTO encrypted_secrets 
            (id, name, current_value, encryption_key_id, rotated_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            secret.id,
            secret.name,
            secret.current_value,
            secret.encryption_key_id,
            secret.rotated_at,
            secret.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|_| SecretError::EncryptionFailed)?;

        Ok(secret.into())
    }

    pub async fn rotate(
        &mut self,
        name: &str,
        new_plaintext: &Secret<String>,
        new_key_id: &str,
    ) -> Result<EncryptedSecretResponse, SecretError> {
        let mut secret = sqlx::query_as!(
            EncryptedSecret,
            r#"SELECT * FROM encrypted_secrets WHERE name = $1"#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| SecretError::KeyNotFound)?;

        secret.rotate(new_plaintext, new_key_id, &*self.encryptor)?;

        sqlx::query!(
            r#"
            UPDATE encrypted_secrets
            SET 
                current_value = $1,
                previous_value = $2,
                encryption_key_id = $3,
                rotated_at = $4
            WHERE id = $5
            "#,
            secret.current_value,
            secret.previous_value,
            secret.encryption_key_id,
            secret.rotated_at,
            secret.id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| SecretError::EncryptionFailed)?;

        Ok(secret.into())
    }
}