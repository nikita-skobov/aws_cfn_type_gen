

/// Applies an Amazon S3 bucket policy to an Amazon S3 bucket. If you are using an identity    other than the root user of the AWS account that owns the bucket, the calling    identity must have the PutBucketPolicy permissions on the specified bucket and    belong to the bucket owner's account in order to use this operation.
///
/// If you don't have PutBucketPolicy permissions, Amazon S3 returns a 403     Access Denied error. If you have the correct permissions, but you're not using an    identity that belongs to the bucket owner's account, Amazon S3 returns a 405 Method Not     Allowed error.
///
/// For more information, see Bucket policy    examples.
///
/// The following operations are related to PutBucketPolicy:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketPolicy {


    /// 
    /// A policy document containing permissions to add to the specified bucket. In IAM, you must    provide policy documents in JSON format. However, in CloudFormation you can provide the policy    in JSON or YAML format because CloudFormation converts YAML to JSON before submitting it to    IAM. For more information, see the AWS::IAM::Policy PolicyDocument resource description in this guide and Access Policy Language     Overview in the Amazon S3 User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,


    /// 
    /// The name of the Amazon S3 bucket to which the policy applies.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,

}

impl cfn_resources::CfnResource for CfnBucketPolicy {
    fn type_string() -> &'static str {
        "AWS::S3::BucketPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
