

/// The AWS::S3::AccessPoint resource is an Amazon S3 resource type that you can use to access    buckets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPoint {


    /// 
    /// The name of the bucket associated with this access point.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// The access point policy associated with this access point.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: Option<serde_json::Value>,


    /// 
    /// The Virtual Private Cloud (VPC) configuration for this access point, if one exists.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,


    /// 
    /// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You    can enable the configuration options in any combination. For more information about when    Amazon S3 considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PublicAccessBlockConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,


    /// 
    /// The AWS account ID associated with the S3 bucket associated with this access point.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<String>,


    /// 
    /// The name of this access point. If you don't specify a name, AWS CloudFormation    generates a unique ID and uses that ID for the access point name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string() -> &'static str {
        "AWS::S3::AccessPoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The Virtual Private Cloud (VPC) configuration for this access point.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {


    /// 
    /// If this field is specified, the access point will only allow connections from the    specified VPC ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}




/// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can     enable the configuration options in any combination. For more information about when Amazon S3     considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicAccessBlockConfiguration {


    /// 
    /// Specifies whether Amazon S3 should block public access control lists (ACLs) for this bucket     and objects in this bucket. Setting this element to TRUE causes the following     behavior:
    /// 
    /// PUT Bucket ACL and PUT Object ACL calls fail if the specified ACL is        public.               PUT Object calls fail if the request includes a public ACL.               PUT Bucket calls fail if the request includes a public ACL.
    /// 
    /// Enabling this setting doesn't affect existing policies or ACLs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<bool>,


    /// 
    /// Specifies whether Amazon S3 should block public bucket policies for this bucket. Setting this     element to TRUE causes Amazon S3 to reject calls to PUT Bucket policy if the     specified bucket policy allows public access.
    /// 
    /// Enabling this setting doesn't affect existing bucket policies.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,


    /// 
    /// Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this     bucket. Setting this element to TRUE causes Amazon S3 to ignore all public ACLs on     this bucket and objects in this bucket.
    /// 
    /// Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't     prevent new public ACLs from being set.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,


    /// 
    /// Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting     this element to TRUE restricts access to this bucket to only AWS service principals and authorized users within this account if the bucket has     a public policy.
    /// 
    /// Enabling this setting doesn't affect previously stored bucket policies, except that     public and cross-account access within any public bucket policy, including non-public     delegation to specific accounts, is blocked.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,

}


