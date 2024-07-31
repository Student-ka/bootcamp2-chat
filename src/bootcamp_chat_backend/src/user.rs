
use canidid::CandidType;
#[derive(Clone)]

pub struct UserData {
    nickname: String,
    avatar_url: Option<String>,
}

impl UserData {
    pub fn new(nickname: String) -> Self {
        Self {
            nickname,
            avatar_url: None,
        }
    }
}