/// Contains the identifiers for a group, a group member, and a GroupMembership object in the identity store.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupMembership {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupId")]
    pub group_id: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IdentityStoreId")]
    pub identity_store_id: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_membership_id: CfnGroupMembershipmembershipid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupMembershipmembershipid;
impl CfnGroupMembershipmembershipid {
    pub fn att_name(&self) -> &'static str {
        r#"MembershipId"#
    }
}

impl cfn_resources::CfnResource for CfnGroupMembership {
    fn type_string(&self) -> &'static str {
        "AWS::IdentityStore::GroupMembership"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.member_id.validate()?;

        Ok(())
    }
}

/// An object that contains the identifier of a group member. Setting the UserID field to the specific identifier for a user indicates that the user is a member of the group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MemberId {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserId")]
    pub user_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MemberId {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
