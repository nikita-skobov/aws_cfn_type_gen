/// The AWS::CodeArtifact::Repository resource creates an AWS CodeArtifact repository.      CodeArtifact repositories contain a set of package versions.    For more information about repositories, see the    Repository concepts information      in the CodeArtifact User Guide. For more information about the CreateRepository API, see    CreateRepository      in the CodeArtifact API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRepository {
    ///
    /// A text description of the repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Pattern: \P{C}*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    /// The name of the domain that contains the repository.
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
    /// The 12-digit account number of the AWS account that owns the domain that contains the repository. It does not include     dashes or spaces.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: [0-9]{12}
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,

    ///
    /// An array of external connections associated with the repository.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_connections: Option<Vec<String>>,

    ///
    /// The document that defines the resource policy that is set on a repository.
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
    /// The name of an upstream repository.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9._\-]{1,99}
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryName")]
    pub repository_name: String,

    ///
    /// A list of tags to be applied to the repository.
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
    /// A list of upstream repositories to associate with the repository. The order of the upstream repositories     in the list determines their priority order when AWS CodeArtifact looks for a requested package version. For more     information, see Working with upstream repositories.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Upstreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CfnRepository {
    fn type_string(&self) -> &'static str {
        "AWS::CodeArtifact::Repository"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

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

        if let Some(the_val) = &self.domain_owner {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain_owner'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.domain_owner {
            if the_val.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'domain_owner'. {} is less than 12",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.repository_name;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'repository_name'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.repository_name;

        if the_val.len() < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'repository_name'. {} is less than 2",
                the_val.len()
            ));
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
