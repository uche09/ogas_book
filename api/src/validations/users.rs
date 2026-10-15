use super::ValidationError;

pub fn is_strong_password(password: &str) -> Result<(), ValidationError> {
    let mut error = ValidationError::new("Weak password");
    error.message = Some(
        "Password must be at least 8 characters long and must contain: \
        uppercase letter, lowercase letter, digit and special character."
        .into()
    );

    if password.len() < 8 {
        return Err(error);
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_uppercase() { has_uppercase = true; }
        else if c.is_lowercase() { has_lowercase = true; }
        else if c.is_numeric() { has_digit = true; }
        else if c.is_ascii_punctuation() { has_special = true; }

        // Short-circuit: stop iterating early if all conditions are satisfied!
        if has_uppercase && has_lowercase && has_digit && has_special {
            return Ok(());
        }
    }

    Err(error)
}