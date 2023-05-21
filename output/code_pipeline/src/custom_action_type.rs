/// The AWS::CodePipeline::CustomActionType resource creates a custom action    for activities that aren't included in the CodePipeline default actions, such as running an    internally developed build process or a test suite. You can use these custom actions in the    stage of a pipeline. For more information, see Create and Add a Custom     Action in AWS CodePipeline in the AWS CodePipeline User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCustomActionType {
    ///
    /// The category of the custom action, such as a build action or a test       action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Approval | Build | Deploy | Invoke | Source | Test
    ///
    /// Update requires: Replacement
    #[serde(rename = "Category")]
    pub category: CustomActionTypeCategoryEnum,

    ///
    /// The configuration properties for the custom action.
    ///
    /// NoteYou can refer to a name in the configuration properties of the custom action         within the URL templates by following the format of {Config:name}, as long as the         configuration property is both required and not secret. For more information, see           Create a           Custom Action for a Pipeline.
    ///
    /// Required: No
    ///
    /// Type: List of ConfigurationProperties
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<Vec<ConfigurationProperties>>,

    ///
    /// The details of the input artifact for the action, such as its commit ID.
    ///
    /// Required: Yes
    ///
    /// Type: ArtifactDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,

    ///
    /// The details of the output artifact of the action, such as its commit ID.
    ///
    /// Required: Yes
    ///
    /// Type: ArtifactDetails
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,

    ///
    /// The provider of the service used in the custom action, such as CodeDeploy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 35
    ///
    /// Pattern: [0-9A-Za-z_-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Provider")]
    pub provider: String,

    ///
    /// URLs that provide users information about this custom action.
    ///
    /// Required: No
    ///
    /// Type: Settings
    ///
    /// Update requires: Replacement
    #[serde(rename = "Settings")]
    pub settings: Option<Settings>,

    ///
    /// The tags for the custom action.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The version identifier of the custom action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 9
    ///
    /// Pattern: [0-9A-Za-z_-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomActionTypeCategoryEnum {
    /// Approval
    #[serde(rename = "Approval")]
    Approval,

    /// Build
    #[serde(rename = "Build")]
    Build,

    /// Deploy
    #[serde(rename = "Deploy")]
    Deploy,

    /// Invoke
    #[serde(rename = "Invoke")]
    Invoke,

    /// Source
    #[serde(rename = "Source")]
    Source,

    /// Test
    #[serde(rename = "Test")]
    Test,
}

impl Default for CustomActionTypeCategoryEnum {
    fn default() -> Self {
        CustomActionTypeCategoryEnum::Approval
    }
}

impl cfn_resources::CfnResource for CfnCustomActionType {
    fn type_string(&self) -> &'static str {
        "AWS::CodePipeline::CustomActionType"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.configuration_properties {
            if the_val.len() > 10 as _ {
                return Err(format!("Max validation failed on field 'configuration_properties'. {} is greater than 10", the_val.len()));
            }
        }

        self.input_artifact_details.validate()?;

        self.output_artifact_details.validate()?;

        let the_val = &self.provider;

        if the_val.len() > 35 as _ {
            return Err(format!(
                "Max validation failed on field 'provider'. {} is greater than 35",
                the_val.len()
            ));
        }

        let the_val = &self.provider;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'provider'. {} is less than 1",
                the_val.len()
            ));
        }

        self.settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.version;

        if the_val.len() > 9 as _ {
            return Err(format!(
                "Max validation failed on field 'version'. {} is greater than 9",
                the_val.len()
            ));
        }

        let the_val = &self.version;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'version'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Returns information about the details of an artifact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArtifactDetails {
    ///
    /// The maximum number of artifacts allowed for the action type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaximumCount")]
    pub maximum_count: i64,

    ///
    /// The minimum number of artifacts allowed for the action type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinimumCount")]
    pub minimum_count: i64,
}

impl cfn_resources::CfnResource for ArtifactDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.maximum_count;

        if *the_val > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'maximum_count'. {} is greater than 5",
                the_val
            ));
        }

        let the_val = &self.maximum_count;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'maximum_count'. {} is less than 0",
                the_val
            ));
        }

        let the_val = &self.minimum_count;

        if *the_val > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'minimum_count'. {} is greater than 5",
                the_val
            ));
        }

        let the_val = &self.minimum_count;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'minimum_count'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// The configuration properties for the custom action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationProperties {
    ///
    /// The description of the action configuration property that is displayed to       users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 160
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Whether the configuration property is a key.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: bool,

    ///
    /// The name of the action configuration property.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Indicates that the property is used with PollForJobs. When creating a       custom action, an action can have up to one queryable property. If it has one, that       property must be both required and not secret.
    ///
    /// If you create a pipeline with a custom action type, and that custom action contains       a queryable property, the value for that configuration property is subject to other       restrictions. The value must be less than or equal to twenty (20) characters. The value       can contain only alphanumeric characters, underscores, and hyphens.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Queryable")]
    pub queryable: Option<bool>,

    ///
    /// Whether the configuration property is a required value.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Required")]
    pub required: bool,

    ///
    /// Whether the configuration property is secret. Secrets are hidden from all calls       except for GetJobDetails, GetThirdPartyJobDetails,         PollForJobs, and PollForThirdPartyJobs.
    ///
    /// When updating a pipeline, passing * * * * * without changing any other values of       the action preserves the previous value of the secret.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Secret")]
    pub secret: bool,

    ///
    /// The type of the configuration property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Boolean | Number | String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<ConfigurationPropertiesTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConfigurationPropertiesTypeEnum {
    /// Boolean
    #[serde(rename = "Boolean")]
    Boolean,

    /// Number
    #[serde(rename = "Number")]
    Number,

    /// String
    #[serde(rename = "String")]
    String,
}

impl Default for ConfigurationPropertiesTypeEnum {
    fn default() -> Self {
        ConfigurationPropertiesTypeEnum::Boolean
    }
}

impl cfn_resources::CfnResource for ConfigurationProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 160 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 160",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'description'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.name;

        if the_val.len() > 50 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 50",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Settings is a property of the AWS::CodePipeline::CustomActionType    resource that provides URLs that users can access to view information about the CodePipeline    custom action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Settings {
    ///
    /// The URL returned to the CodePipeline console that provides a deep link to the       resources of the external system, such as the configuration page for a CodeDeploy       deployment group. This link is provided as part of the action display in the       pipeline.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntityUrlTemplate")]
    pub entity_url_template: Option<String>,

    ///
    /// The URL returned to the CodePipeline console that contains a link to the       top-level landing page for the external system, such as the console page for CodeDeploy. This link is shown on the pipeline view page in the CodePipeline console       and provides a link to the execution entity of the external action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutionUrlTemplate")]
    pub execution_url_template: Option<String>,

    ///
    /// The URL returned to the CodePipeline console that contains a link to the page       where customers can update or change the configuration of the external action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "RevisionUrlTemplate")]
    pub revision_url_template: Option<String>,

    ///
    /// The URL of a sign-up page where users can sign up for an external service and       perform initial configuration of the action provided by that service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThirdPartyConfigurationUrl")]
    pub third_party_configuration_url: Option<String>,
}

impl cfn_resources::CfnResource for Settings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.entity_url_template {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'entity_url_template'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.entity_url_template {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'entity_url_template'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.execution_url_template {
            if the_val.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'execution_url_template'. {} is greater than 2048", the_val.len()));
            }
        }

        if let Some(the_val) = &self.execution_url_template {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'execution_url_template'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.revision_url_template {
            if the_val.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'revision_url_template'. {} is greater than 2048", the_val.len()));
            }
        }

        if let Some(the_val) = &self.revision_url_template {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'revision_url_template'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.third_party_configuration_url {
            if the_val.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'third_party_configuration_url'. {} is greater than 2048", the_val.len()));
            }
        }

        if let Some(the_val) = &self.third_party_configuration_url {
            if the_val.len() < 1 as _ {
                return Err(format!("Min validation failed on field 'third_party_configuration_url'. {} is less than 1", the_val.len()));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
