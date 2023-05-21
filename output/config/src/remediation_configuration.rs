

/// An object that represents the details about the remediation configuration that includes the remediation action, parameters, and data to execute the action.
#[derive(Default, serde::Serialize)]
pub struct CfnRemediationConfiguration {


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
    pub target_id: String,


    /// 
    /// An ExecutionControls object.
    /// 
    /// Required: No
    ///
    /// Type: ExecutionControls
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionControls")]
    pub execution_controls: Option<ExecutionControls>,


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
    pub target_version: Option<String>,


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
    pub parameters: Option<serde_json::Value>,


    /// 
    /// The remediation is triggered automatically.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Automatic")]
    pub automatic: Option<bool>,


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
    pub target_type: String,


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
    pub config_rule_name: String,


    /// 
    /// The type of a resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,


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
    pub maximum_automatic_attempts: Option<i64>,

}


/// The value is either a dynamic (resource) value or a static value. You must select either a dynamic value or a static value.
#[derive(Default, serde::Serialize)]
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
    pub static_value: Option<StaticValue>,

}


/// AWS Systems Manager (SSM) specific remediation controls.
#[derive(Default, serde::Serialize)]
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
    pub error_percentage: Option<i64>,

}


/// The dynamic value of the resource.
#[derive(Default, serde::Serialize)]
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
    pub value: Option<String>,

}


/// The static value of the resource.
#[derive(Default, serde::Serialize)]
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
    pub values: Option<Vec<String>>,

}


/// An ExecutionControls object.
#[derive(Default, serde::Serialize)]
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
    pub ssm_controls: Option<SsmControls>,

}
