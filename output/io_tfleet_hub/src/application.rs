/// Represents a Fleet Hub for AWS IoT Device Management web application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {
    ///
    /// An optional description of the web application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[ -~]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,

    ///
    /// The name of the web application.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[ -~]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

    ///
    /// The ARN of the role that the web application assumes when it interacts with AWS IoT Core.
    ///
    /// NoteThe name of the role must be in the form FleetHub_random_string.
    ///
    /// Pattern: ^arn:[!-~]+$
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

    ///
    /// A set of key/value pairs that you can use to manage the web application resource.
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

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string(&self) -> &'static str {
        "AWS::IoTFleetHub::Application"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.application_description {
            if the_val.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'application_description'. {} is greater than 2048", the_val.len()));
            }
        }

        if let Some(the_val) = &self.application_description {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'application_description'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.application_name;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'application_name'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.application_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'application_name'. {} is less than 1",
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
