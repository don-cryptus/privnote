use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CsrfToken {
    pub tag: usize,
    pub text: String,
}

impl CsrfToken {
    pub fn new(tag: &usize, text: &String) -> CsrfToken {
        CsrfToken {
            tag: tag.clone(),
            text: text.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeleteNoteReq {
    pub tag: usize,
    pub text: String,

    pub id: String,
    pub manual_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NoteReq {
    pub tag: usize,
    pub text: String,

    pub note: String,
    pub duration_hours: i32,
    pub manual_password: String,
    pub manual_password_confirm: String,
    pub notify_email: String,
    pub destroy_without_confirmation: bool,

    pub delete_at: Option<chrono::NaiveDateTime>,
}
