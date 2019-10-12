use serde::*;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoginUserStatus {
    Enabled,
    Disabled,
    PasswordChangeRequired,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    user_id: String,
    password_hash: String,
    status: LoginUserStatus,
}

#[test]
fn it_should_serialize() {
    let login = Login {
        user_id: "aaa".to_owned(),
        password_hash: "bbb".to_owned(),
        status: LoginUserStatus::PasswordChangeRequired,
    };
    assert_eq!(
        serde_json::to_value(&login).unwrap(),
        serde_json::from_str::<serde_json::Value>(
            r#"
                {
                    "user_id": "aaa",
                    "password_hash": "bbb",
                    "status": "password_change_required"
                }
            "#
        )
        .unwrap()
    );
}
