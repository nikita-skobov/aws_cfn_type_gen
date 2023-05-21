

/// The AWS::CloudFormation::Macro resource is a CloudFormation resource type that creates a   CloudFormation macro to perform custom processing on CloudFormation templates. For more  information, see Using   AWS CloudFormation macros to perform custom processing on templates.
#[derive(Default, serde::Serialize)]
pub struct CfnMacro {


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
    /// A description of the macro.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}
