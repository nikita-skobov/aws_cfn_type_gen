/// Manages a detector and associated detector versions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDetector {
    ///
    /// The models to associate with this detector. You must provide the ARNs of all the models you want to associate.
    ///
    /// Required: No
    ///
    /// Type: List of Model
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatedModels")]
    pub associated_models: Option<Vec<Model>>,

    ///
    /// The detector description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name of the detector.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[0-9a-z_-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DetectorId")]
    pub detector_id: String,

    ///
    /// The status of the detector version. If a value is not provided for this property, AWS CloudFormation assumes DRAFT status.
    ///
    /// Valid values: ACTIVE | DRAFT
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetectorVersionStatus")]
    pub detector_version_status: Option<String>,

    ///
    /// The event type associated with this detector.
    ///
    /// Required: Yes
    ///
    /// Type: EventType
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventType")]
    pub event_type: EventType,

    ///
    /// The rule execution mode for the rules included in the detector version.
    ///
    /// Valid values: FIRST_MATCHED | ALL_MATCHED Default value: FIRST_MATCHED
    ///
    /// You can define and edit the rule mode at the detector version level, when it is in draft status.
    ///
    /// If you specify FIRST_MATCHED, Amazon Fraud Detector      evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.
    ///
    /// If you specifiy ALL_MATCHED, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleExecutionMode")]
    pub rule_execution_mode: Option<String>,

    ///
    /// The rules to include in the detector version.
    ///
    /// Required: Yes
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDetector {
    fn type_string(&self) -> &'static str {
        "AWS::FraudDetector::Detector"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
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

        let the_val = &self.detector_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'detector_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.detector_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'detector_id'. {} is less than 1",
                the_val.len()
            ));
        }

        self.event_type.validate()?;

        Ok(())
    }
}

/// The entity type details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EntityType {
    ///
    /// The entity type ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// Timestamp of when the entity type was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The entity type description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    ///
    /// For example, when creating AWS::FraudDetector::Detector you must define at least two variables. You can set Inline=true for these Variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your detector but not execute any      changes to the variables.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

    ///
    /// Timestamp of when the entity type was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The entity type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for EntityType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'created_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'created_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
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

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'last_updated_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'last_updated_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The event type details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventType {
    ///
    /// The entity type ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// Timestamp of when the event type was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The event type description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The event type entity types.
    ///
    /// Required: No
    ///
    /// Type: List of EntityType
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityTypes")]
    pub entity_types: Option<Vec<EntityType>>,

    ///
    /// The event type event variables.
    ///
    /// Required: No
    ///
    /// Type: List of EventVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventVariables")]
    pub event_variables: Option<Vec<EventVariable>>,

    ///
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    ///
    /// For example, when creating AWS::FraudDetector::Detector you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the Variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your detector but not execute any      changes to the variables.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

    ///
    /// The event type labels.
    ///
    /// Required: No
    ///
    /// Type: List of Label
    ///
    /// Update requires: No interruption
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<Label>>,

    ///
    /// Timestamp of when the event type was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The event type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for EventType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'created_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'created_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
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

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'last_updated_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'last_updated_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The event type variable for the detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventVariable {
    ///
    /// The event variable ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// Timestamp for when the event variable was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The data source of the event variable.
    ///
    /// Valid values: EVENT | EXTERNAL_MODEL_SCORE
    ///
    /// When defining a variable within a detector, you can only use the EVENT value for DataSource when the Inline property is set to true.      If the Inline property is set false, you can use either EVENT or MODEL_SCORE for DataSource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,

    ///
    /// The data type of the event variable.
    ///
    /// Valid values: STRING | INTEGER | BOOLEAN | FLOAT
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,

    ///
    /// The default value of the event variable. This is required if you are providing the details of your variables instead of the ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

    ///
    /// The description of the event variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    ///
    /// For example, when creating AWS::FraudDetector::Detector you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your detector but not execute any      changes to the variables.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

    ///
    /// Timestamp for when the event variable was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The name of the event variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of event variable. For more information, see Variable types.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,
}

impl cfn_resources::CfnResource for EventVariable {
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

/// The label details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Label {
    ///
    /// The label ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// Timestamp of when the event type was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The label description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    ///
    /// For example, when creating AWS::FraudDetector::Detector you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your detector but not execute any      changes to the variables.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

    ///
    /// Timestamp of when the label was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The label name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for Label {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'created_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'created_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
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

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'last_updated_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'last_updated_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The Model property type specifies Property description not available. for an AWS::FraudDetector::Detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Model {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
}

impl cfn_resources::CfnResource for Model {
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

/// The outcome.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Outcome {
    ///
    /// The outcome ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn\:aws[a-z-]{0,15}\:frauddetector\:[a-z0-9-]{3,20}\:[0-9]{12}\:[^\s]{2,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// The timestamp when the outcome was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The outcome description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Indicates whether the resource is defined within this CloudFormation template and impacts the create, update, and delete behavior of the stack. If the value is true,     CloudFormation will create/update/delete the resource when creating/updating/deleting the stack. If the value is false, CloudFormation will validate that the object exists and      then use it within the resource without making changes to the object.
    ///
    /// For example, when creating AWS::FraudDetector::Detector you must define at least two variables. You can set Inline=true for these variables and CloudFormation will      create/update/delete the variables as part of stack operations. However, if you set Inline=false, CloudFormation will associate the variables to your detector but not execute any      changes to the variables.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,

    ///
    /// The timestamp when the outcome was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 11
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The outcome name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[0-9a-z_-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for Outcome {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'created_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.created_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'created_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 128",
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

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'last_updated_time'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.last_updated_time {
            if the_val.len() < 11 as _ {
                return Err(format!(
                    "Min validation failed on field 'last_updated_time'. {} is less than 11",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A rule. Rule is a condition that tells Amazon Fraud Detector how to interpret variables values during a fraud prediction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rule {
    ///
    /// The rule ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

    ///
    /// Timestamp for when the rule was created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

    ///
    /// The rule description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The detector for which the rule is associated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[0-9a-z_-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetectorId")]
    pub detector_id: Option<String>,

    ///
    /// The rule expression. A rule expression captures the business logic. For more information, see Rule language reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

    ///
    /// The rule language.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Language")]
    pub language: Option<String>,

    ///
    /// Timestamp for when the rule was last updated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

    ///
    /// The rule outcome.
    ///
    /// Required: No
    ///
    /// Type: List of Outcome
    ///
    /// Update requires: No interruption
    #[serde(rename = "Outcomes")]
    pub outcomes: Option<Vec<Outcome>>,

    ///
    /// The rule ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[0-9a-z_-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleId")]
    pub rule_id: Option<String>,

    ///
    /// The rule version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5
    ///
    /// Pattern: ^([1-9][0-9]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleVersion")]
    pub rule_version: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for Rule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.detector_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'detector_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.detector_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'detector_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rule_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'rule_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rule_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'rule_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rule_version {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'rule_version'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rule_version {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'rule_version'. {} is less than 1",
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
