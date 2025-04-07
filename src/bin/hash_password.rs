use argon2::{ self, Argon2 };
use password_hash::{ PasswordHasher, SaltString };
use rand_core::OsRng;
use rpassword::prompt_password;
use std::fs;
use std::io::{ self, Write };
use std::path::Path;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Argon2id Password Hash Generator for ADMIN_PASSWORD_HASH");
    println!("-------------------------------------------------------");
    println!("This will securely prompt for a password and automatically");
    println!("update/add ADMIN_PASSWORD_HASH in the .env file");
    println!("in the current directory.");
    println!("!!! Ensure you run this from the project root directory !!!");
    println!("-------------------------------------------------------");

    let password = prompt_password("Enter new admin password: ")?;
    let confirmation = prompt_password("Confirm admin password: ")?;

    if password != confirmation {
        eprintln!("\nError: Passwords do not match.");
        exit(1);
    }

    if password.is_empty() {
        eprintln!("\nError: Password cannot be empty.");
        exit(1);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19456, 2, 1, None).unwrap()
    );

    print!("Hashing password using Argon2id (this may take a moment)... ");
    io::stdout().flush()?;

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Argon2 hashing failed: {}", e))?
        .to_string();

    println!("Done.");

    // Escape all dollar signs in the hash to prevent environment variable substitution
    let escaped_hash = password_hash.replace('$', "\\$");

    // --- Update .env file ---
    let env_file_path_str = ".env";
    let env_path = Path::new(env_file_path_str);
    let key_to_update = "ADMIN_PASSWORD_HASH";
    let new_env_line = format!("{}={}", key_to_update, escaped_hash);

    let mut lines: Vec<String> = Vec::new();
    let mut key_found = false;

    if env_path.exists() {
        let content = fs
            ::read_to_string(env_path)
            .map_err(|e| format!("Error reading {}: {}", env_file_path_str, e))?;

        for line in content.lines() {
            if line.trim_start().starts_with(&format!("{}=", key_to_update)) {
                lines.push(new_env_line.clone());
                key_found = true;
            } else {
                lines.push(line.to_string());
            }
        }
    }

    // If the key wasn't found in the file (or the file didn't exist), add the new line
    if !key_found {
        lines.push(new_env_line);
    }

    fs
        ::write(env_path, lines.join("\n"))
        .map_err(|e| format!("Error writing {}: {}", env_file_path_str, e))?;

    println!("\nSuccess!");
    println!("{} has been updated successfully with the new hash.", env_file_path_str);
    println!(
        "The $ symbols have been escaped as \\$ to prevent environment variable substitution."
    );

    Ok(())
}
