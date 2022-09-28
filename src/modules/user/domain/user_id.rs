use serde::Serialize;
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct UserId {
    pub value: String,
}

impl Default for UserId {
    fn default() -> Self {
        let value = Ulid::new().to_string();
        Self { value }
    }
}

impl UserId {
    pub fn restore(value: &str) -> Self {
        let value = Ulid::from_string(value).unwrap().to_string();
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_user_id_new() {
        let user_id = UserId::default();
        assert!(!user_id.value.is_empty());
    }
}
