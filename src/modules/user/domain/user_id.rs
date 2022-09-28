use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserId {
    pub value: Ulid,
}

impl Default for UserId {
    fn default() -> Self {
        let value = Ulid::new();
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_user_id_new() {
        let user_id = UserId::default();
        assert!(!user_id.value.to_string().is_empty());
    }
}
