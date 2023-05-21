/// Specifies a MemoryDB user. For more information, see Authenticating       users with Access Contol Lists (ACLs).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUser {
    ///
    /// Access permissions string used for this user.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<String>,

    ///
    /// Denotes whether the user requires a password to authenticate.
    ///
    /// Example:
    ///
    /// mynewdbuser:   Type: AWS::MemoryDB::User   Properties:    AccessString: on ~* &* +@all   AuthenticationMode:      Passwords: '1234567890123456'     Type: password   UserName: mynewdbuser      AuthenticationMode:   {     "Passwords": ["1234567890123456"],     "Type": "Password"   }
    ///
    /// Required: No
    ///
    /// Type: AuthenticationMode
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The name of the user.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: String,
}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string(&self) -> &'static str {
        "AWS::MemoryDB::User"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authentication_mode
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AuthenticationMode property type specifies Property description not available. for an AWS::MemoryDB::User.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthenticationMode {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Passwords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<String>,
}

impl cfn_resources::CfnResource for AuthenticationMode {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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
