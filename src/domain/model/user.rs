use serde::*;

// `derive Deserialize` here is kinda ugly hack
// using deriving automation to deserialize string representation
//
// Do not forget to add #[serde(deserialize_with = "role_serde::deserialize")]
#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Unknown,
    User,
    PowerUser,
    Admin,
}

impl Role {
    pub fn new_from_str(rep: &str) -> Role {
        match serde_json::to_string(rep)
            .and_then(|r| serde_json::from_str(&r))
            .ok()
        {
            Some(t) => t,
            None => Role::Unknown,
        }
    }

    pub fn as_string(&self) -> String {
        serde_json::from_str(&serde_json::to_string(self).unwrap()).unwrap()
    }
}

mod role_serde {
    use super::Role;
    use serde::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Role, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Role::new_from_str(&s))
    }
}

#[test]
fn role_new_as_str_inverse() {
    assert_eq!(Role::new_from_str(&Role::Admin.as_string()), Role::Admin);
    assert_eq!(Role::new_from_str("admin").as_string(), "admin");
}

#[test]
fn admin_should_stronger_than_all_other_role() {
    use Role::*;
    assert!(Admin >= Unknown);
    assert!(Admin >= User);
    assert!(Admin >= PowerUser);
}

#[test]
fn unknown_should_weaker_than_all_other_role() {
    use Role::*;
    assert!(Unknown <= User);
    assert!(Unknown <= PowerUser);
    assert!(Unknown <= Admin);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
    #[serde(deserialize_with = "role_serde::deserialize")]
    pub role: Role,
}

#[test]
fn it_should_serialize() {
    let user = User {
        id: "id".to_owned(),
        name: "name".to_owned(),
        display_name: "日本語".to_owned(),
        role: Role::PowerUser,
    };
    assert_eq!(
        serde_json::to_value(&user).unwrap(),
        serde_json::from_str::<serde_json::Value>(
            r#"
                {
                    "id": "id",
                    "name": "name",
                    "display_name": "日本語",
                    "role": "power_user"
                }
            "#
        )
        .unwrap()
    );
}

#[test]
fn it_should_parse_json_into_user() {
    let json = r#"
        {
            "id": "aaa",
            "name": "bbb",
            "display_name": "日本語",
            "role": "power_user"
        }
    "#;

    let user = serde_json::from_str::<User>(json).unwrap();
    assert_eq!("aaa", user.id);
    assert_eq!("bbb", user.name);
    assert_eq!("日本語", user.display_name);
    assert_eq!(Role::PowerUser, user.role);
}

#[test]
fn it_should_parse_role_fallback_into_unknown() {
    let json = r#"
        {
            "id": "",
            "name": "",
            "display_name": "",
            "role": "???"
        }
    "#;

    let user = serde_json::from_str::<User>(json).unwrap();
    assert_eq!(Role::Unknown, user.role);
}
