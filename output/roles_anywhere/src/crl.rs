

/// Imports the certificate revocation list (CRL). A CRL is a list of certificates that have     been revoked by the issuing certificate Authority (CA). IAM Roles Anywhere     validates against the CRL before issuing credentials.
///
/// Required permissions: rolesanywhere:ImportCrl.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCRL {


    /// 
    /// The x509 v3 specified certificate revocation list (CRL).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrlData")]
    pub crl_data: String,


    /// 
    /// Specifies whether the certificate revocation list (CRL) is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The name of the certificate revocation list (CRL).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[ a-zA-Z0-9-_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The ARN of the TrustAnchor the certificate revocation list (CRL) will provide revocation for.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: ^arn:aws(-[^:]+)?:rolesanywhere(:.*){2}(:trust-anchor.*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustAnchorArn")]
    pub trust_anchor_arn: Option<String>,


    /// 
    /// A list of tags to attach to the certificate revocation list (CRL).
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

impl cfn_resources::CfnResource for CfnCRL {
    fn type_string() -> &'static str {
        "AWS::RolesAnywhere::CRL"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
