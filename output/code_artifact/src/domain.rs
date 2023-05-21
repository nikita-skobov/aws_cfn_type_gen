/// The AWS::CodeArtifact::Domain resource creates an AWS CodeArtifact domain.      CodeArtifact domains make it easier to manage multiple repositories across an       organization. You can use a domain to apply permissions across many repositories owned by different      AWS accounts. For more information about domains, see the       Domain concepts information      in the CodeArtifact User Guide. For more information about the CreateDomain API, see       CreateDomain      in the CodeArtifact API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomain {
    ///
    /// A string that specifies the name of the requested domain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 50
    ///
    /// Pattern: [a-z][a-z0-9\-]{0,48}[a-z0-9]
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,

    ///
    /// The key used to encrypt the domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Pattern: \S+
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,

    ///
    /// The document that defines the resource policy that is set on a domain.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsPolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_policy_document: Option<serde_json::Value>,

    ///
    /// A list of tags to be applied to the domain.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDomain {
    fn type_string(&self) -> &'static str {
        "AWS::CodeArtifact::Domain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain_name;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'domain_name'. {} is greater than 50",
                the_val.len()
            ));
        }

        let the_val = &self.domain_name;

        if the_val.len() < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'domain_name'. {} is less than 2",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.encryption_key {
            if the_val.len() > 1011 as _ {
                return Err(format!(
                    "Max validation failed on field 'encryption_key'. {} is greater than 1011",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.encryption_key {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'encryption_key'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

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
