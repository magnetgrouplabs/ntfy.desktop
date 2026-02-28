use anyhow::Result;
use keyring::Entry;

const SERVICE_NAME: &str = "ntfy-desktop";

/// Credential keys stored in the OS keychain
const KEY_API_TOKEN: &str = "api_token";
const KEY_AUTH_USER: &str = "auth_user";
const KEY_AUTH_PASS: &str = "auth_pass";

fn get_entry(key: &str) -> Result<Entry> {
    Entry::new(SERVICE_NAME, key).map_err(|e| anyhow::anyhow!("Keyring error: {}", e))
}

fn get_secret(key: &str) -> Result<String> {
    let entry = get_entry(key)?;
    match entry.get_password() {
        Ok(val) => Ok(val),
        Err(keyring::Error::NoEntry) => Ok(String::new()),
        Err(e) => Err(anyhow::anyhow!("Failed to read credential '{}': {}", key, e)),
    }
}

fn set_secret(key: &str, value: &str) -> Result<()> {
    let entry = get_entry(key)?;
    if value.is_empty() {
        // Delete the entry if the value is empty
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already gone
            Err(e) => Err(anyhow::anyhow!("Failed to delete credential '{}': {}", key, e)),
        }
    } else {
        entry
            .set_password(value)
            .map_err(|e| anyhow::anyhow!("Failed to save credential '{}': {}", key, e))
    }
}

/// Credentials stored securely in the OS keychain
#[derive(Debug, Clone, Default)]
pub struct Credentials {
    pub api_token: String,
    pub auth_user: String,
    pub auth_pass: String,
}

/// Load all credentials from the OS keychain
pub fn load_credentials() -> Result<Credentials> {
    println!("Loading credentials from OS keychain (service: {})", SERVICE_NAME);

    let api_token = get_secret(KEY_API_TOKEN)?;
    let auth_user = get_secret(KEY_AUTH_USER)?;
    let auth_pass = get_secret(KEY_AUTH_PASS)?;

    println!(
        "Credentials loaded: api_token={} chars, auth_user={}",
        api_token.len(),
        if auth_user.is_empty() { "empty" } else { "present" }
    );

    Ok(Credentials {
        api_token,
        auth_user,
        auth_pass,
    })
}

/// Save all credentials to the OS keychain
pub fn save_credentials(creds: &Credentials) -> Result<()> {
    println!("Saving credentials to OS keychain (service: {})", SERVICE_NAME);

    set_secret(KEY_API_TOKEN, &creds.api_token)?;
    set_secret(KEY_AUTH_USER, &creds.auth_user)?;
    set_secret(KEY_AUTH_PASS, &creds.auth_pass)?;

    println!(
        "Credentials saved: api_token={} chars, auth_user={}",
        creds.api_token.len(),
        if creds.auth_user.is_empty() { "empty" } else { "present" }
    );

    Ok(())
}
