

/// Contains the identifiers for a group, a group member, and a GroupMembership object in the identity store.
#[derive(Default, serde::Serialize)]
pub struct CfnGroupMembership {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: String,


    /// 
    /// An object containing the identifier of a group member. Setting MemberId's UserId field     to a specific User's ID indicates we should consider that User as a group member.
    /// 
    /// Required: Yes
    ///
    /// Type: MemberId
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberId")]
    pub member_id: MemberId,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupId")]
    pub group_id: String,

}


/// An object that contains the identifier of a group member. Setting the UserID field to the specific identifier for a user indicates that the user is a member of the group.
#[derive(Default, serde::Serialize)]
pub struct MemberId {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserId")]
    pub user_id: String,

}
