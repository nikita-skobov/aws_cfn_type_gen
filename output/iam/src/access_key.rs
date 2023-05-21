

/// Creates a new AWS secret access key and corresponding AWS     access key ID for the specified user. The default status for new keys is       Active.
///
/// For information about quotas on the number of keys you can create, see IAM and        AWS STS quotas in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessKey {


    /// 
    /// The name of the IAM user that the new key will belong to.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: String,


    /// 
    /// The status of the access key. Active means that the key is valid for API     calls, while Inactive means it is not.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Active | Inactive
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<AccessKeyStatusEnum>,


    /// 
    /// This value is specific to CloudFormation and can only be       incremented. Incrementing this value notifies CloudFormation that you want to rotate your access key. When you update your stack,       CloudFormation will replace the existing access key with a new key.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Serial")]
    pub serial: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AccessKeyStatusEnum {

    /// Active
    #[serde(rename = "Active")]
    Active,

    /// Inactive
    #[serde(rename = "Inactive")]
    Inactive,

}

impl Default for AccessKeyStatusEnum {
    fn default() -> Self {
        AccessKeyStatusEnum::Active
    }
}


impl cfn_resources::CfnResource for CfnAccessKey {
    fn type_string() -> &'static str {
        "AWS::IAM::AccessKey"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
