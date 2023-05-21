

/// The AWS::Lightsail::Certificate resource specifies an SSL/TLS certificate     that you can use with a content delivery network (CDN) distribution and a container     service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificate {


    /// 
    /// The name of the certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,


    /// 
    /// The domain name of the certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// An array of strings that specify the alternate domains (such as example.org)     and subdomains (such as blog.example.com) of the certificate.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<Vec<String>>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    /// 
    /// NoteThe Value of Tags is optional for Lightsail resources.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string() -> &'static str {
        "AWS::Lightsail::Certificate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}