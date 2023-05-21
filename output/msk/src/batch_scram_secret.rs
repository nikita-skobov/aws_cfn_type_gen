

/// The AWS::MSK::BatchScramSecret resource Property description not available. for MSK.
#[derive(Default, serde::Serialize)]
pub struct CfnBatchScramSecret {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArnList")]
    pub secret_arn_list: Option<Vec<String>>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,

}
