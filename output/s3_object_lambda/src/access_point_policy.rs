

/// The AWS::S3ObjectLambda::AccessPointPolicy resource specifies the Object       Lambda Access Point resource policy document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPointPolicy {


    /// 
    /// An access point with an attached AWS Lambda function used to access transformed data from an Amazon S3     bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectLambdaAccessPoint")]
    pub object_lambda_access_point: String,


    /// 
    /// Object Lambda Access Point resource policy document.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

}



impl cfn_resources::CfnResource for CfnAccessPointPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::S3ObjectLambda::AccessPointPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}