/// Specifies an authorizer.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAuthorizer {
    ///
    /// The authorizer's Lambda function ARN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerFunctionArn")]
    pub authorizer_function_arn: cfn_resources::StrVal,

    ///
    /// The authorizer name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableCachingForHttp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching_for_http: Option<bool>,

    ///
    /// Specifies whether AWS IoT validates the token signature in an authorization request.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SigningDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_disabled: Option<bool>,

    ///
    /// The status of the authorizer.
    ///
    /// Valid values: ACTIVE | INACTIVE
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// Metadata which can be used to manage the custom authorizer.
    ///
    /// NoteFor URI Request parameters use format: ...key1=value1&key2=value2...For the CLI command-line parameter use format: &&tags       "key1=value1&key2=value2..."For the cli-input-json file use format: "tags":       "key1=value1&key2=value2..."
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
    /// The key used to extract the token from the HTTP headers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<cfn_resources::StrVal>,

    ///
    /// The public keys used to validate the token signature returned by your custom     authentication service.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenSigningPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnAuthorizerarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAuthorizerarn;
impl CfnAuthorizerarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAuthorizer {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::Authorizer"
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
