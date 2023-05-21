

/// Creates an IAM resource that describes an identity provider (IdP) that supports SAML       2.0.
///
/// The SAML provider resource that you create with this operation can be used as a       principal in an IAM role's trust policy. Such a policy can enable federated users who       sign in using the SAML IdP to assume the role. You can create an IAM role that       supports Web-based single sign-on (SSO) to the AWS Management Console or one that supports API access       to AWS.
///
/// When you create the SAML provider resource, you upload a SAML metadata document that       you get from your IdP. That document includes the issuer's name, expiration information,       and keys that can be used to validate the SAML authentication response (assertions) that       the IdP sends. You must generate the metadata document using the identity management       software that is used as your organization's IdP.
///
/// For more information, see Enabling SAML 2.0         federated users to access the AWS Management Console and About SAML 2.0-based         federation in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSAMLProvider {


    /// 
    /// An XML document generated by an identity provider (IdP) that supports SAML 2.0. The       document includes the issuer's name, expiration information, and keys that can be used       to validate the SAML authentication response (assertions) that are received from the       IdP. You must generate the metadata document using the identity management software that       is used as your organization's IdP.
    /// 
    /// For more information, see About SAML 2.0-based         federation in the IAM User Guide
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1000
    ///
    /// Maximum: 10000000
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamlMetadataDocument")]
    pub saml_metadata_document: String,


    /// 
    /// The name of the provider to create.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w._-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A list of tags that you want to attach to the new IAM SAML provider.    Each tag consists of a key name and an associated value. For more information about tagging, see Tagging IAM resources in the    IAM User Guide.
    /// 
    /// NoteIf any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request   fails and the resource is not created.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnSAMLProvider {
    fn type_string() -> &'static str {
        "AWS::IAM::SAMLProvider"
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


