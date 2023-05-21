/// Describes a parameter group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClusterParameterGroup {
    ///
    /// The description of the parameter group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: String,

    ///
    /// The name of the cluster parameter group family that this cluster parameter group is       compatible with. You can create a custom parameter group and then associate your cluster with it. For more information, see        Amazon Redshift parameter groups.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParameterGroupFamily")]
    pub parameter_group_family: String,

    ///
    /// The name of the cluster parameter group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,

    ///
    /// An array of parameters to be modified. A maximum of 20 parameters can be modified       in a single request.
    ///
    /// For each parameter to be modified, you must supply at least the parameter name and       parameter value; other name-value pairs of the parameter are optional.
    ///
    /// For the workload management (WLM) configuration, you must supply all the name-value       pairs in the wlm_json_configuration parameter.
    ///
    /// Required: No
    ///
    /// Type: List of Parameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<Parameter>>,

    ///
    /// The list of tags for the cluster parameter group.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnClusterParameterGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::ClusterParameterGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.description;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'description'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        let the_val = &self.parameter_group_family;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'parameter_group_family'. {} is greater than 2147483647", the_val.len()));
        }

        if let Some(the_val) = &self.parameter_group_name {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'parameter_group_name'. {} is greater than 2147483647", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Describes a parameter in a cluster parameter group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Parameter {
    ///
    /// The name of the parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,

    ///
    /// The value of the parameter. If ParameterName is wlm_json_configuration,       then the maximum size of ParameterValue is 8000 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
}

impl cfn_resources::CfnResource for Parameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.parameter_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'parameter_name'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        let the_val = &self.parameter_value;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'parameter_value'. {} is greater than 2147483647",
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
