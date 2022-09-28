use super::{user_id::UserId, user_name::UserName};

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub age: i16,
}

impl User {
    pub fn new(id: UserId, name: UserName, age: i16) -> Self {
        Self { id, name, age }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_user_new() {
        let user = User::new(
            UserId::default(),
            UserName::new("John".to_string(), "Doe".to_string()),
            30,
        );

        assert!(!user.id.value.to_string().is_empty());
    }
}
