/// This resource configures how Amazon CodeGuru Reviewer retrieves the source code to be reviewed. You can use an      AWS CloudFormation template to create an association with the following repository types:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRepositoryAssociation {
    ///
    /// The name of the bucket. This is required for your S3Bucket repository. The name must start with the prefix codeguru-reviewer-*.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of an AWS CodeStar Connections connection. Its format is      arn:aws:codestar-connections:region-id:aws-account_id:connection/connection-id. For more information, see      Connection in      the AWS CodeStar Connections API Reference.
    ///
    /// ConnectionArn must be specified for Bitbucket and GitHub Enterprise Server repositories. It has no effect if      it is specified for an AWS CodeCommit repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws(-[\w]+)*:.+:.+:[0-9]{12}:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub connection_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the repository.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^\S[\w.-]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The owner of the repository. For a GitHub Enterprise Server or Bitbucket repository, this is the username     for the account that owns the repository.
    ///
    /// Owner must be specified for Bitbucket and GitHub Enterprise Server repositories. It has no effect if      it is specified for an AWS CodeCommit repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^\S(.*\S)?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub owner: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs used to tag an associated repository. A tag is a custom attribute label with two parts:
    ///
    /// A tag key (for example, CostCenter, 						Environment, Project, or Secret). Tag 						keys are case sensitive. 				 					An optional field known as a tag value (for example, 						111122223333, Production, or a team name). 						Omitting the tag value is the same as using an empty string. Like tag keys, tag 						values are case sensitive.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of repository that contains the source code to be reviewed. The valid values are:
    ///
    /// CodeCommit            Bitbucket            GitHubEnterpriseServer           S3Bucket
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_association_arn: CfnRepositoryAssociationassociationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRepositoryAssociationassociationarn;
impl CfnRepositoryAssociationassociationarn {
    pub fn att_name(&self) -> &'static str {
        r#"AssociationArn"#
    }
}

impl cfn_resources::CfnResource for CfnRepositoryAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::CodeGuruReviewer::RepositoryAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.connection_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'connection_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.connection_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'connection_arn'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'owner'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'owner'. {} is less than 1",
                        s.len()
                    ));
                }
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
