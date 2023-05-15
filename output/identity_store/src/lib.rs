
pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// A string containing the description of the group.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The globally unique identifier for the identity store.
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: String,
    /// A string containing the name of the group. This value is commonly displayed when the group is referenced.
    #[serde(rename = "DisplayName")]
    pub display_name: String,

}



}

pub mod cfn_group_membership {

#[derive(serde::Serialize, Default)]
pub struct CfnGroupMembership {
    /// The globally unique identifier for the identity store.
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: String,
    /// An object containing the identifier of a group member.
    #[serde(rename = "MemberId")]
    pub member_id: MemberId,
    /// The unique identifier for a group in the identity store.
    #[serde(rename = "GroupId")]
    pub group_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct MemberId {
    #[serde(rename = "UserId")]
    pub user_id: String,

}


}
