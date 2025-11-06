use std::path::PathBuf;
use rusqlite::{Connection, Result as SqliteResult};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use anyhow::{Result, anyhow};
use colored::*;

pub struct PasswordManager {
    conn: Connection,
    key: Key<Aes256Gcm>,
}

impl PasswordManager {
    pub fn new(master_password: &str, db_path: PathBuf) -> Result<Self> {
        // Initialize database
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY,
                service TEXT NOT NULL,
                username TEXT NOT NULL,
                encrypted_password TEXT NOT NULL,
                nonce TEXT NOT NULL
            )",
            [],
        )?;

        // Generate encryption key from master password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let key = argon2
            .hash_password(master_password.as_bytes(), &salt)?
            .hash
            .ok_or_else(|| anyhow!("Failed to generate key"))?
            .as_bytes()
            .to_vec();
        
        let key: &Key<Aes256Gcm> = Key::from_slice(&key[0..32]);

        Ok(PasswordManager {
            conn,
            key: *key,
        })
    }

    pub fn add_password(&self, service: &str, username: &str, password: &str) -> Result<()> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let encrypted_password = cipher
            .encrypt(&nonce, password.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let encrypted_b64 = BASE64.encode(encrypted_password);
        let nonce_b64 = BASE64.encode(nonce);

        self.conn.execute(
            "INSERT INTO passwords (service, username, encrypted_password, nonce) VALUES (?1, ?2, ?3, ?4)",
            [service, username, &encrypted_b64, &nonce_b64],
        )?;

        println!("{}", "Password stored successfully!".green());
        Ok(())
    }

    pub fn get_password(&self, service: &str, username: &str) -> Result<String> {
        let mut stmt = self.conn.prepare(
            "SELECT encrypted_password, nonce FROM passwords WHERE service = ? AND username = ?"
        )?;

        let (encrypted_b64, nonce_b64): (String, String) = stmt
            .query_row([service, username], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|_| anyhow!("No password found for this service and username"))?;

        let encrypted = BASE64.decode(encrypted_b64)?;
        let nonce = BASE64.decode(nonce_b64)?;
        let nonce = nonce.as_slice().try_into()
            .map_err(|_| anyhow!("Invalid nonce"))?;

        let cipher = Aes256Gcm::new(&self.key);
        let password = cipher
            .decrypt(nonce, encrypted.as_slice())
            .map_err(|_| anyhow!("Decryption failed"))?;

        String::from_utf8(password)
            .map_err(|_| anyhow!("Invalid UTF-8"))
    }

    pub fn list_services(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT service, username FROM passwords")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut services = Vec::new();
        for row in rows {
            services.push(row?);
        }

        Ok(services)
    }

    pub fn delete_password(&self, service: &str, username: &str) -> Result<()> {
        let affected = self.conn.execute(
            "DELETE FROM passwords WHERE service = ? AND username = ?",
            [service, username],
        )?;

        if affected == 0 {
            Err(anyhow!("No password found for this service and username"))
        } else {
            println!("{}", "Password deleted successfully!".green());
            Ok(())
        }
    }
}