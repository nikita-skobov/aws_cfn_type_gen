/// The AWS::S3Outposts::Bucket resource specifies a new Amazon S3 on Outposts bucket.    To create an S3 on Outposts bucket, you must have S3 on Outposts capacity provisioned on your Outpost.    For more information, see     Using Amazon S3 on Outposts.
///
/// S3 on Outposts buckets support the following:
///
/// For a complete list of restrictions and Amazon S3 feature limitations on S3 on Outposts,     see     Amazon S3 on Outposts Restrictions and Limitations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBucket {
    ///
    /// A name for the S3 on Outposts bucket. If you don't specify a name, AWS CloudFormation generates a    unique ID and uses that ID for the bucket name.    The bucket name must contain only lowercase letters, numbers, periods (.), and dashes (-)    and must follow     Amazon S3 bucket restrictions and limitations.    For more information, see Bucket     naming rules.
    ///
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you need to     replace the resource, specify a new name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

    ///
    /// Creates a new lifecycle configuration for the S3 on Outposts bucket or replaces an existing    lifecycle configuration. Outposts buckets only support lifecycle configurations that delete/expire objects    after a certain period of time and abort incomplete multipart uploads.
    ///
    /// Required: No
    ///
    /// Type: LifecycleConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,

    ///
    /// The ID of the Outpost of the specified bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,

    ///
    /// Sets the tags for an S3 on Outposts bucket. For more information, see Using Amazon S3 on Outposts.
    ///
    /// Use tags to organize your AWS bill to reflect your own cost structure. To do this, sign up to get your    AWS account bill with tag key values included. Then, to see the cost of combined resources, organize your    billing information according to resources with the same tag key values. For example, you can tag several    resources with a specific application name, and then organize your billing information to see the total cost    of that application across several services. For more information, see    Cost allocation and     tags.
    ///
    /// NoteWithin a bucket, if you add a tag that has the same key as an existing tag, the new value overwrites    the old value. For more information, see     Using cost allocation and bucket tags.
    ///
    /// To use this resource, you must have permissions to perform the    s3-outposts:PutBucketTagging. The S3 on Outposts bucket owner has this    permission by default and can grant this permission to others. For more information about    permissions, see Permissions     Related to Bucket Subresource Operations and Managing access permissions to your Amazon S3 resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnBucket {
    fn type_string(&self) -> &'static str {
        "AWS::S3Outposts::Bucket"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lifecycle_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the days since the initiation of an incomplete multipart upload that Amazon S3 on Outposts waits    before permanently removing all parts of the upload. For more information, see Aborting Incomplete     Multipart Uploads Using a Bucket Lifecycle Policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AbortIncompleteMultipartUpload {
    ///
    /// Specifies the number of days after initiation that Amazon S3 on Outposts aborts an incomplete multipart upload.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: i64,
}

impl cfn_resources::CfnResource for AbortIncompleteMultipartUpload {
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

/// The Filter property type specifies Property description not available. for an AWS::S3Outposts::Bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Filter {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FilterAndOperator
    ///
    /// Update requires: No interruption
    #[serde(rename = "AndOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_operator: Option<FilterAndOperator>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FilterTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<FilterTag>,
}

impl cfn_resources::CfnResource for Filter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.and_operator
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.tag.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FilterAndOperator property type specifies Property description not available. for an AWS::S3Outposts::Bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterAndOperator {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of FilterTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Vec<FilterTag>,
}

impl cfn_resources::CfnResource for FilterAndOperator {
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

/// The FilterTag property type specifies Property description not available. for an AWS::S3Outposts::Bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterTag {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for FilterTag {
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

/// The container for the lifecycle configuration for the objects stored in an S3 on Outposts bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LifecycleConfiguration {
    ///
    /// The container for the lifecycle configuration rules for the objects stored in the S3 on Outposts bucket.
    ///
    /// Required: Yes
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
}

impl cfn_resources::CfnResource for LifecycleConfiguration {
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

/// A container for an Amazon S3 on Outposts bucket lifecycle rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rule {
    ///
    /// The container for the abort incomplete multipart upload rule.
    ///
    /// Required: No
    ///
    /// Type: AbortIncompleteMultipartUpload
    ///
    /// Update requires: No interruption
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,

    ///
    /// Specifies the expiration for the lifecycle of the object by specifying an expiry date.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,

    ///
    /// Specifies the expiration for the lifecycle of the object in the form of days that the object has been in the S3 on Outposts bucket.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpirationInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i64>,

    ///
    /// The container for the filter of the lifecycle rule.
    ///
    /// Required: No
    ///
    /// Type: Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,

    ///
    /// The unique identifier for the lifecycle rule. The value can't be longer than 255    characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    ///
    /// If Enabled, the rule is currently being applied. If Disabled,    the rule is not currently being applied.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,
}

impl cfn_resources::CfnResource for Rule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.abort_incomplete_multipart_upload
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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
