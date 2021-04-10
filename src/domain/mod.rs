

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDTO {
    pub token: String,
    pub username: String,
    pub nickname: String,
}