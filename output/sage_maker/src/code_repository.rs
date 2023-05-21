/// Creates a Git repository as a resource in your SageMaker account. You can associate the       repository with notebook instances so that you can use Git source control for the       notebooks you create. The Git repository is a resource in your SageMaker account, so it can       be associated with more than one notebook instance, and it persists independently from       the lifecycle of any notebook instances it is associated with.
///
/// The repository can be hosted either in AWS CodeCommit       or in any other Git repository.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCodeRepository {
    ///
    /// The name of the Git repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: Option<String>,

    ///
    /// Configuration details for the Git repository, including the URL where it is located       and the ARN of the AWS Secrets Manager secret that contains the       credentials used to access the repository.
    ///
    /// Required: Yes
    ///
    /// Type: GitConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "GitConfig")]
    pub git_config: GitConfig,

    ///
    /// List of tags for Code Repository.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnCodeRepository {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::CodeRepository"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.code_repository_name {
            if the_val.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'code_repository_name'. {} is greater than 63",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.code_repository_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'code_repository_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.git_config.validate()?;

        Ok(())
    }
}

/// Specifies configuration details for a Git repository in your AWS       account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GitConfig {
    ///
    /// The default branch for the Git repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [^ ~^:?*\[]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Branch")]
    pub branch: Option<String>,

    ///
    /// The URL where the Git repository is located.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^https://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Secrets Manager secret that       contains the credentials used to access the git repository. The secret must have a       staging label of AWSCURRENT and must be in the following format:
    ///
    /// {"username": UserName, "password":           Password}
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws[a-z\-]*:secretsmanager:[a-z0-9\-]*:[0-9]{12}:secret:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
}

impl cfn_resources::CfnResource for GitConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.branch {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'branch'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.branch {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'branch'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.secret_arn {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'secret_arn'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.secret_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'secret_arn'. {} is less than 1",
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
