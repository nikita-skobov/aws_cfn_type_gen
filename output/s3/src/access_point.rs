/// The AWS::S3::AccessPoint resource is an Amazon S3 resource type that you can use to access    buckets.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub bucket: cfn_resources::StrVal,

    ///
    /// The AWS account ID associated with the S3 bucket associated with this access point.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketAccountId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_account_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of this access point. If you don't specify a name, AWS CloudFormation    generates a unique ID and uses that ID for the access point name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The access point policy associated with this access point.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub policy: Option<serde_json::Value>,

    ///
    /// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You    can enable the configuration options in any combination. For more information about when    Amazon S3 considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
    ///
    /// Required: No
    ///
    /// Type: PublicAccessBlockConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,

    ///
    /// The Virtual Private Cloud (VPC) configuration for this access point, if one exists.
    ///
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_configuration: Option<VpcConfiguration>,

    #[serde(skip_serializing)]
    pub att_alias: CfnAccessPointalias,

    #[serde(skip_serializing)]
    pub att_arn: CfnAccessPointarn,

    #[serde(skip_serializing)]
    pub att_name: CfnAccessPointname,

    #[serde(skip_serializing)]
    pub att_network_origin: CfnAccessPointnetworkorigin,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointalias;
impl CfnAccessPointalias {
    pub fn att_name(&self) -> &'static str {
        r#"Alias"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointarn;
impl CfnAccessPointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointname;
impl CfnAccessPointname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointnetworkorigin;
impl CfnAccessPointnetworkorigin {
    pub fn att_name(&self) -> &'static str {
        r#"NetworkOrigin"#
    }
}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string(&self) -> &'static str {
        "AWS::S3::AccessPoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.public_access_block_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The PublicAccessBlock configuration that you want to apply to this Amazon S3 bucket. You can     enable the configuration options in any combination. For more information about when Amazon S3     considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub restrict_public_buckets: Option<bool>,
}

impl cfn_resources::CfnResource for PublicAccessBlockConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Virtual Private Cloud (VPC) configuration for this access point.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpcConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
