#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserName {
    first_name: String,
    last_name: String,
    pub full_name: String,
}

impl UserName {
    pub fn new(first_name: String, last_name: String) -> Self {
        if first_name.is_empty() || last_name.is_empty() {
            panic!("first_name and last_name must not be empty");
        }

        let full_name = format!("{} {}", first_name, last_name);
        Self {
            first_name,
            last_name,
            full_name,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn success_user_name_new() {
        let user_name = UserName::new("John".to_string(), "Doe".to_string());
        assert_eq!(user_name.full_name, "John Doe");
    }

    #[test]
    #[should_panic(expected = "first_name and last_name must not be empty")]
    fn require_first_name() {
        let user_name = UserName::new("".to_string(), "".to_string());
        assert_eq!(user_name.full_name, "John Doe");
    }
}
