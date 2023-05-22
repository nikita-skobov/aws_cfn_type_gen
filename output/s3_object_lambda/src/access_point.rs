/// The AWS::S3ObjectLambda::AccessPoint resource specifies an Object Lambda       Access Point used to access a bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPoint {
    ///
    /// The name of this access point.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// A configuration used when creating an Object Lambda Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: ObjectLambdaConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectLambdaConfiguration")]
    pub object_lambda_configuration: ObjectLambdaConfiguration,

    #[serde(skip_serializing)]
    pub att_alias_status: CfnAccessPointaliasstatus,

    #[serde(skip_serializing)]
    pub att_alias_value: CfnAccessPointaliasvalue,

    #[serde(skip_serializing)]
    pub att_arn: CfnAccessPointarn,

    #[serde(skip_serializing)]
    pub att_creation_date: CfnAccessPointcreationdate,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPointaliasstatus;
impl CfnAccessPointaliasstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Alias.Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPointaliasvalue;
impl CfnAccessPointaliasvalue {
    pub fn att_name(&self) -> &'static str {
        r#"Alias.Value"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPointarn;
impl CfnAccessPointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPointcreationdate;
impl CfnAccessPointcreationdate {
    pub fn att_name(&self) -> &'static str {
        r#"CreationDate"#
    }
}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string(&self) -> &'static str {
        "AWS::S3ObjectLambda::AccessPoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.object_lambda_configuration.validate()?;

        Ok(())
    }
}

/// The alias of an Object Lambda Access Point. For more information, see How to use a bucket-style alias for your S3 bucket     Object Lambda Access Point.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Alias {
    ///
    /// The status of the Object Lambda Access Point alias. If the status is PROVISIONING, the Object Lambda Access Point is provisioning the alias and the alias is not ready for use yet. If      the status is READY, the Object Lambda Access Point alias is successfully provisioned and ready for use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: cfn_resources::StrVal,

    ///
    /// The alias value of the Object Lambda Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Alias {
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

/// The AwsLambda property type specifies Property description not available. for an AWS::S3ObjectLambda::AccessPoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsLambda {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionArn")]
    pub function_arn: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_payload: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AwsLambda {
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

/// The ContentTransformation property type specifies Property description not available. for an AWS::S3ObjectLambda::AccessPoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContentTransformation {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: AwsLambda
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsLambda")]
    pub aws_lambda: AwsLambda,
}

impl cfn_resources::CfnResource for ContentTransformation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.aws_lambda.validate()?;

        Ok(())
    }
}

/// A configuration used when creating an Object Lambda Access Point.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ObjectLambdaConfiguration {
    ///
    /// A container for allowed features. Valid inputs are GetObject-Range,         GetObject-PartNumber, HeadObject-Range, and         HeadObject-PartNumber.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_features: Option<Vec<String>>,

    ///
    /// A container for whether the CloudWatch metrics configuration is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,

    ///
    /// Standard access point associated with the Object Lambda Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportingAccessPoint")]
    pub supporting_access_point: cfn_resources::StrVal,

    ///
    /// A container for transformation configurations for an Object Lambda Access Point.
    ///
    /// Required: Yes
    ///
    /// Type: List of TransformationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransformationConfigurations")]
    pub transformation_configurations: Vec<TransformationConfiguration>,
}

impl cfn_resources::CfnResource for ObjectLambdaConfiguration {
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

/// Indicates whether this access point policy is public. For more information about how Amazon S3     evaluates policies to determine whether they are public, see The Meaning of "Public" in the Amazon S3 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyStatus {
    ///
    ///
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl cfn_resources::CfnResource for PolicyStatus {
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

/// The PublicAccessBlock configuration that you want to apply to this Amazon S3     account. You can enable the configuration options in any combination. For more information     about when Amazon S3 considers a bucket or object public, see The Meaning of "Public" in the Amazon S3 User Guide.
///
/// This data type is not supported for Amazon S3 on Outposts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublicAccessBlockConfiguration {
    ///
    /// Specifies whether Amazon S3 should block public access control lists (ACLs) for buckets in     this account. Setting this element to TRUE causes the following     behavior:
    ///
    /// PutBucketAcl and PutObjectAcl calls fail if the        specified ACL is public.               PUT Object calls fail if the request includes a public ACL.               PUT Bucket calls fail if the request includes a public ACL.
    ///
    /// Enabling this setting doesn't affect existing policies or ACLs.
    ///
    /// This property is not supported for Amazon S3 on Outposts.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should block public bucket policies for buckets in this account.     Setting this element to TRUE causes Amazon S3 to reject calls to PUT Bucket policy     if the specified bucket policy allows public access.
    ///
    /// Enabling this setting doesn't affect existing bucket policies.
    ///
    /// This property is not supported for Amazon S3 on Outposts.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should ignore public ACLs for buckets in this account. Setting     this element to TRUE causes Amazon S3 to ignore all public ACLs on buckets in this     account and any objects that they contain.
    ///
    /// Enabling this setting doesn't affect the persistence of any existing ACLs and doesn't     prevent new public ACLs from being set.
    ///
    /// This property is not supported for Amazon S3 on Outposts.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnorePublicAcls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,

    ///
    /// Specifies whether Amazon S3 should restrict public bucket policies for buckets in this     account. Setting this element to TRUE restricts access to buckets with public     policies to only AWS service principals and authorized users within this     account.
    ///
    /// Enabling this setting doesn't affect previously stored bucket policies, except that     public and cross-account access within any public bucket policy, including non-public     delegation to specific accounts, is blocked.
    ///
    /// This property is not supported for Amazon S3 on Outposts.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestrictPublicBuckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// A configuration used when creating an Object Lambda Access Point transformation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformationConfiguration {
    ///
    /// A container for the action of an Object Lambda Access Point configuration. Valid       inputs are GetObject, HeadObject, ListObject, and         ListObjectV2.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

    ///
    /// A container for the content transformation of an Object Lambda Access Point       configuration. Can include the FunctionArn and FunctionPayload. For more information,       see AwsLambdaTransformation in the Amazon S3 API       Reference.
    ///
    /// Required: Yes
    ///
    /// Type: ContentTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentTransformation")]
    pub content_transformation: ContentTransformation,
}

impl cfn_resources::CfnResource for TransformationConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.content_transformation.validate()?;

        Ok(())
    }
}
