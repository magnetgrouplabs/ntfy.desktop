use keyring::Entry;

const SERVICE_NAME: &str = "ntfy-desktop";
const KEY_API_TOKEN: &str = "api_token";

fn main() {
    let entry = Entry::new(SERVICE_NAME, KEY_API_TOKEN).unwrap();
    entry.set_password("tk_flmzv8ke9yvtps3lk6b8i3j3qsgn2").unwrap();
    println!("API token stored successfully!");
}
