/// In a CloudFormation template, you use the AWS::CloudFormation::CustomResource or   Custom::String   resource type to specify custom resources.
///
/// Custom resources provide a way for you to write custom provisioning logic in CloudFormation template  and have CloudFormation run it during a stack operation, such as when you create, update or delete a stack.  For more information, see Custom resources.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCustomResource {
    ///
    /// NoteOnly one property is defined by AWS for a custom resource: ServiceToken. All   other properties are defined by the service provider.
    ///
    /// The service token that was given to the template developer by the service provider to access the service, such  as an Amazon SNS topic ARN or Lambda function ARN. The service token must be from the same Region  in which you are creating the stack.
    ///
    /// Updates aren't supported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceToken")]
    pub service_token: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnCustomResource {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::CustomResource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
