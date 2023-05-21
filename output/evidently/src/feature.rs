/// Creates or updates an Evidently feature that you want to launch or test. You can define up to       five variations of a feature, and use these variations in your launches and experiments. A feature must be created in       a project. For information about creating a project, see       CreateProject.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFeature {
    ///
    /// The name of the variation to use as the default variation. The default       variation is served to users who are not allocated to any ongoing launches       or experiments of this feature.
    ///
    /// This variation must also be listed in the Variations structure.
    ///
    /// If you omit DefaultVariation, the first variation listed in       the Variations structure is used as the default variation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultVariation")]
    pub default_variation: Option<String>,

    ///
    /// An optional description of the feature.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Specify users that should always be served a specific variation of a feature. Each user       is specified by a key-value pair . For each key, specify a user by entering their user ID,       account ID, or some other identifier. For the value, specify the name of the variation that       they are to be served.
    ///
    /// Required: No
    ///
    /// Type: List of EntityOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityOverrides")]
    pub entity_overrides: Option<Vec<EntityOverride>>,

    ///
    /// Specify ALL_RULES to activate the traffic allocation specified by any       ongoing launches or experiments. Specify DEFAULT_VARIATION to serve the default       variation to all users instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationStrategy")]
    pub evaluation_strategy: Option<String>,

    ///
    /// The name for the feature. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The name or ARN of the project that is to contain the new feature.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Project")]
    pub project: String,

    ///
    /// Assigns one or more tags (key-value pairs) to the feature.
    ///
    /// Tags can help you organize and categorize your resources. You can also use them to scope user       permissions by granting a user       permission to access or change only resources with certain tag values.
    ///
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    ///
    /// You can associate as many as 50 tags with a feature.
    ///
    /// For more information, see Tagging AWS resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// An array of structures that contain the configuration of the feature's different variations.
    ///
    /// Each VariationObject in the Variations array for     a feature must have the same type of value (BooleanValue, DoubleValue, LongValue     or StringValue).
    ///
    /// Required: Yes
    ///
    /// Type: List of VariationObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variations")]
    pub variations: Vec<VariationObject>,
}

impl cfn_resources::CfnResource for CfnFeature {
    fn type_string(&self) -> &'static str {
        "AWS::Evidently::Feature"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A set of key-value pairs that specify users who should always be served a specific       variation of a feature. Each key specifies a user using their user ID, account ID, or some       other identifier. The value specifies the name of the variation that the user is to be       served.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EntityOverride {
    ///
    /// The entity ID to be served the variation specified in Variation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityId")]
    pub entity_id: Option<String>,

    ///
    /// The name of the variation to serve to the user session that matches the EntityId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variation")]
    pub variation: Option<String>,
}

impl cfn_resources::CfnResource for EntityOverride {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This structure contains the name and variation value of one variation of a feature. It       can contain only one of the following parameters: BooleanValue, DoubleValue, LongValue       or StringValue.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VariationObject {
    ///
    /// The value assigned to this variation, if the variation type is boolean.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<bool>,

    ///
    /// The value assigned to this variation, if the variation type is a double.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<f64>,

    ///
    /// The value assigned to this variation, if the variation type is a long.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "LongValue")]
    pub long_value: Option<f64>,

    ///
    /// The value assigned to this variation, if the variation type is a string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

    ///
    /// A name for the variation. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariationName")]
    pub variation_name: String,
}

impl cfn_resources::CfnResource for VariationObject {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
