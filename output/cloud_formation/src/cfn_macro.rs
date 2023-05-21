

/// The AWS::CloudFormation::Macro resource is a CloudFormation resource type that creates a   CloudFormation macro to perform custom processing on CloudFormation templates. For more  information, see Using   AWS CloudFormation macros to perform custom processing on templates.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub description: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the underlying AWS Lambda function that you want AWS CloudFormation to invoke when the macro is run.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionName")]
    pub function_name: String,


    /// 
    /// The CloudWatch Logs group to which AWS CloudFormation sends error logging information when invoking the  macro's underlying AWS Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,


    /// 
    /// The ARN of the role AWS CloudFormation should assume when sending log entries to CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogRoleARN")]
    pub log_role_arn: Option<String>,


    /// 
    /// The name of the macro. The name of the macro must be unique across all macros in the account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for CfnMacro {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::Macro"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}