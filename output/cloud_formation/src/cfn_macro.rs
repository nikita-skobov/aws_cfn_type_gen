/// The AWS::CloudFormation::Macro resource is a CloudFormation resource type that creates a   CloudFormation macro to perform custom processing on CloudFormation templates. For more  information, see Using   AWS CloudFormation macros to perform custom processing on templates.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMacro {
    ///
    /// A description of the macro.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the underlying AWS Lambda function that you want AWS CloudFormation to invoke when the macro is run.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionName")]
    pub function_name: cfn_resources::StrVal,

    ///
    /// The CloudWatch Logs group to which AWS CloudFormation sends error logging information when invoking the  macro's underlying AWS Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the role AWS CloudFormation should assume when sending log entries to CloudWatch Logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogRoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the macro. The name of the macro must be unique across all macros in the account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnMacro {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::Macro"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
