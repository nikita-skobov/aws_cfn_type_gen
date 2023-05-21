

/// The AWS::SecurityHub::Hub resource represents the implementation of the AWS Security Hub service in your account. One hub resource is created for each Region in which you enable Security Hub.
///
/// The CIS AWS Foundations Benchmark standard and the Foundational Security Best Practices standard are also enabled in each Region where you enable Security Hub.
#[derive(Default, serde::Serialize)]
pub struct CfnHub {


    /// 
    /// The tags to add to the hub resource.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,

}
