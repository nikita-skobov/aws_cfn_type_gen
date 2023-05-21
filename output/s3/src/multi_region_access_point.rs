

/// The AWS::S3::MultiRegionAccessPoint resource creates an Amazon S3    Multi-Region Access Point. To learn more about Multi-Region Access Points, see     Multi-Region Access Points in Amazon S3 in the in the Amazon S3 User     Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnMultiRegionAccessPoint {


    /// 
    /// The PublicAccessBlock configuration that you want to apply to this Multi-Region Access    Point. You can enable the configuration options in any combination. For more information about    when Amazon S3 considers an object public, see The Meaning of "Public" in the Amazon S3 User Guide.
    /// 
    /// Required: No
    ///
    /// Type: PublicAccessBlockConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,


    /// 
    /// The name of the Multi-Region Access Point.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A collection of the Regions and buckets associated with the Multi-Region Access    Point.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Region
    ///
    /// Update requires: Replacement
    #[serde(rename = "Regions")]
    pub regions: Vec<Region>,

}


/// The PublicAccessBlock configuration that you want to apply to this Amazon S3 Multi-Region    Access Point. You can enable the configuration options in any combination. For more    information about when Amazon S3 considers an object public, see The Meaning of "Public" in the Amazon S3 User Guide.
#[derive(Default, serde::Serialize)]
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
    /// Update requires: Replacement
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
    /// Update requires: Replacement
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,


    /// 
    /// Specifies whether Amazon S3 should restrict public bucket policies for this bucket. Setting     this element to TRUE restricts access to this bucket to only AWS service principals and authorized users within this account if the bucket has     a public policy.
    /// 
    /// Enabling this setting doesn't affect previously stored bucket policies, except that     public and cross-account access within any public bucket policy, including non-public     delegation to specific accounts, is blocked.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,


    /// 
    /// Specifies whether Amazon S3 should ignore public ACLs for this bucket and objects in this     bucket. Setting this element to TRUE causes Amazon S3 to ignore all public ACLs on     this bucket and objects in this bucket.
    /// 
    /// Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't     prevent new public ACLs from being set.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,

}


/// A bucket associated with a specific Region when creating Multi-Region Access    Points.
#[derive(Default, serde::Serialize)]
pub struct Region {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<String>,


    /// 
    /// The name of the associated bucket for the Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bucket")]
    pub bucket: String,

}
