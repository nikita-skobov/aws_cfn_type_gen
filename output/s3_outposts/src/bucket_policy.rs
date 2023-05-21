/// This resource applies a bucket policy to an Amazon S3 on Outposts bucket.
///
/// If you are using an identity other than the root user of the AWS account    that owns the S3 on Outposts bucket, the calling identity must have    the s3-outposts:PutBucketPolicy permissions on the specified    Outposts bucket and belong to the bucket owner's account in order to use    this resource.
///
/// If you don't have s3-outposts:PutBucketPolicy permissions,    S3 on Outposts returns a 403 Access Denied error.
///
/// For more information, see the AWS::IAM::Policy    PolicyDocument resource description in this guide and        Access Policy Language Overview.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucketPolicy {
    ///
    /// The name of the Amazon S3 Outposts bucket to which the policy applies.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,

    ///
    /// A policy document containing permissions to add to the specified bucket. In IAM, you must    provide policy documents in JSON format. However, in CloudFormation, you can provide the    policy in JSON or YAML format because CloudFormation converts YAML to JSON before submitting    it to IAM. For more information, see the AWS::IAM::Policy PolicyDocument resource description in this guide and Access Policy Language     Overview.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,
}

impl cfn_resources::CfnResource for CfnBucketPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::S3Outposts::BucketPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
