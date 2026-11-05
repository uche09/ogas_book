use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Manager,
    Owner,
    Worker,
}

#[derive(Debug, Serialize)]
pub struct MembershipView {
    pub business_id: Uuid,
    pub name: String,
    pub role: Role,
    pub branch_id: Option<Uuid>,
}
