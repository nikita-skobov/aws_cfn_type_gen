/// An object that represents the details about the remediation configuration that includes the remediation action, parameters, and data to execute the action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRemediationConfiguration {
    ///
    /// The remediation is triggered automatically.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Automatic")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub automatic: Option<bool>,

    ///
    /// The name of the AWS Config rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: cfn_resources::StrVal,

    ///
    /// An ExecutionControls object.
    ///
    /// Required: No
    ///
    /// Type: ExecutionControls
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionControls")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub execution_controls: Option<ExecutionControls>,

    ///
    /// The maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
    ///
    /// For example, if you specify MaximumAutomaticAttempts as 5 with RetryAttemptSeconds as 50 seconds, 			 			AWS Config will put a RemediationException on your behalf for the failing resource after the 5th failed attempt within 50 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumAutomaticAttempts")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub maximum_automatic_attempts: Option<i64>,

    ///
    /// An object of the RemediationParameterValue. For more information, see RemediationParameterValue.
    ///
    /// NoteThe type is a map of strings to RemediationParameterValue.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub parameters: Option<serde_json::Value>,

    ///
    /// The type of a resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resource_type: Option<cfn_resources::StrVal>,

    ///
    /// Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
    ///
    /// For example, if you specify RetryAttemptSeconds as 50 seconds and MaximumAutomaticAttempts as 5, 		AWS Config will run auto-remediations 5 times within 50 seconds before throwing an exception.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryAttemptSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub retry_attempt_seconds: Option<i64>,

    ///
    /// Target ID is the name of the SSM document.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetId")]
    pub target_id: cfn_resources::StrVal,

    ///
    /// The type of the target. Target executes remediation. For example, SSM document.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SSM_DOCUMENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetType")]
    pub target_type: RemediationConfigurationTargetTypeEnum,

    ///
    /// Version of the target. For example, version of the SSM document.
    ///
    /// NoteIf you make backward incompatible changes to the SSM document, 			you must call PutRemediationConfiguration API again to ensure the remediations can run.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_version: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RemediationConfigurationTargetTypeEnum {
    /// SSM_DOCUMENT
    #[serde(rename = "SSM_DOCUMENT")]
    Ssmdocument,
}

impl Default for RemediationConfigurationTargetTypeEnum {
    fn default() -> Self {
        RemediationConfigurationTargetTypeEnum::Ssmdocument
    }
}

impl cfn_resources::CfnResource for CfnRemediationConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::Config::RemediationConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.config_rule_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'config_rule_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.config_rule_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'config_rule_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.execution_controls
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.maximum_automatic_attempts {
            if *the_val > 25 as _ {
                return Err(format!("Max validation failed on field 'maximum_automatic_attempts'. {} is greater than 25", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_automatic_attempts {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'maximum_automatic_attempts'. {} is less than 1", the_val));
            }
        }

        let the_val = &self.target_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_id'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// An ExecutionControls object.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExecutionControls {
    ///
    /// A SsmControls object.
    ///
    /// Required: No
    ///
    /// Type: SsmControls
    ///
    /// Update requires: No interruption
    #[serde(rename = "SsmControls")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ssm_controls: Option<SsmControls>,
}

impl cfn_resources::CfnResource for ExecutionControls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ssm_controls
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The value is either a dynamic (resource) value or a static value. You must select either a dynamic value or a static value.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RemediationParameterValue {
    ///
    /// The value is dynamic and changes at run-time.
    ///
    /// Required: No
    ///
    /// Type: ResourceValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceValue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resource_value: Option<ResourceValue>,

    ///
    /// The value is static and does not change at run-time.
    ///
    /// Required: No
    ///
    /// Type: StaticValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticValue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub static_value: Option<StaticValue>,
}

impl cfn_resources::CfnResource for RemediationParameterValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.resource_value
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.static_value
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The dynamic value of the resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourceValue {
    ///
    /// The value is a resource ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: RESOURCE_ID
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub value: Option<ResourceValueValueEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResourceValueValueEnum {
    /// RESOURCE_ID
    #[serde(rename = "RESOURCE_ID")]
    Resourceid,
}

impl Default for ResourceValueValueEnum {
    fn default() -> Self {
        ResourceValueValueEnum::Resourceid
    }
}

impl cfn_resources::CfnResource for ResourceValue {
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

/// AWS Systems Manager (SSM) specific remediation controls.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SsmControls {
    ///
    /// The maximum percentage of remediation actions allowed to run in parallel on the non-compliant resources for that specific rule. You can specify a percentage, such as 10%. The default value is 10.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConcurrentExecutionRatePercentage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub concurrent_execution_rate_percentage: Option<i64>,

    ///
    /// The percentage of errors that are allowed before SSM stops running automations on non-compliant resources for that specific rule. 			You can specify a percentage of errors, for example 10%. If you do not specifiy a percentage, the default is 50%. 			For example, if you set the ErrorPercentage to 40% for 10 non-compliant resources, then SSM stops running the automations when the fifth error is received.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorPercentage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub error_percentage: Option<i64>,
}

impl cfn_resources::CfnResource for SsmControls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.concurrent_execution_rate_percentage {
            if *the_val > 100 as _ {
                return Err(format!("Max validation failed on field 'concurrent_execution_rate_percentage'. {} is greater than 100", the_val));
            }
        }

        if let Some(the_val) = &self.concurrent_execution_rate_percentage {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'concurrent_execution_rate_percentage'. {} is less than 1", the_val));
            }
        }

        if let Some(the_val) = &self.error_percentage {
            if *the_val > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'error_percentage'. {} is greater than 100",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.error_percentage {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'error_percentage'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The static value of the resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StaticValue {
    ///
    /// A list of values. For example, the ARN of the assumed role.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for StaticValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.values {
            if the_val.len() > 25 as _ {
                return Err(format!(
                    "Max validation failed on field 'values'. {} is greater than 25",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
