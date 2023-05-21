/// A group of versioned models in the model registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModelPackageGroup {
    ///
    /// The description for the model group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelPackageGroupDescription")]
    pub model_package_group_description: Option<String>,

    ///
    /// The name of the model group.
    ///
    /// Required: Yes
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
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: String,

    ///
    /// A resouce policy to control access to a model group. For information about resoure       policies, see Identity-based         policies and resource-based policies in the AWS         Identity and Access Management User Guide..
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageGroupPolicy")]
    pub model_package_group_policy: Option<serde_json::Value>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
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

impl cfn_resources::CfnResource for CfnModelPackageGroup {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::ModelPackageGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.model_package_group_description {
            if the_val.len() > 1024 as _ {
                return Err(format!("Max validation failed on field 'model_package_group_description'. {} is greater than 1024", the_val.len()));
            }
        }

        let the_val = &self.model_package_group_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'model_package_group_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.model_package_group_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'model_package_group_name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
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
