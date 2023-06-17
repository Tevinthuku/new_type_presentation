use std::ops::{Deref, DerefMut};

use anyhow::bail;

struct NonEmptyString(String);

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonEmptyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            bail!("Value cannot be empty")
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::NonEmptyString;

    #[test]
    fn test_non_empty_string() {
        let mut name = NonEmptyString::try_from("name".to_owned()).unwrap();
        name.push_str("string");
    }
}
