/// The AWS::DataSync::LocationS3 resource specifies an endpoint for an Amazon     S3 bucket.
///
/// For more information, see Create       an Amazon S3 location in the AWS DataSync User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationS3 {
    ///
    /// The ARN of the Amazon S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 156
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):(s3|s3-outposts):[a-z\-0-9]*:[0-9]*:.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3BucketArn")]
    pub s3_bucket_arn: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that is used     to access an Amazon S3 bucket.
    ///
    /// For detailed information about using such a role, see Creating       a Location for Amazon S3 in the AWS DataSync User       Guide.
    ///
    /// Required: Yes
    ///
    /// Type: S3Config
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,

    ///
    /// The Amazon S3 storage class that you want to store your files in when this location is     used as a task destination. For buckets in AWS Regions, the storage class     defaults to S3 Standard.
    ///
    /// For more information about S3 storage classes, see Amazon S3 Storage Classes. Some storage classes have     behaviors that can affect your S3 storage costs. For detailed information, see Considerations When Working with Amazon S3 Storage Classes in DataSync.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEEP_ARCHIVE | GLACIER | GLACIER_INSTANT_RETRIEVAL | INTELLIGENT_TIERING | ONEZONE_IA | OUTPOSTS | STANDARD | STANDARD_IA
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3StorageClass")]
    pub s3_storage_class: Option<LocationS3S3StorageClassEnum>,

    ///
    /// A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read    data from the S3 source location or write data to the S3 destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\p{Zs}]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    pub subdirectory: Option<String>,

    ///
    /// The key-value pair that represents the tag that you want to add to the location. The    value can be an empty string. We recommend using tags to name your resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LocationS3S3StorageClassEnum {
    /// DEEP_ARCHIVE
    #[serde(rename = "DEEP_ARCHIVE")]
    Deeparchive,

    /// GLACIER
    #[serde(rename = "GLACIER")]
    Glacier,

    /// GLACIER_INSTANT_RETRIEVAL
    #[serde(rename = "GLACIER_INSTANT_RETRIEVAL")]
    Glacierinstantretrieval,

    /// INTELLIGENT_TIERING
    #[serde(rename = "INTELLIGENT_TIERING")]
    Intelligenttiering,

    /// ONEZONE_IA
    #[serde(rename = "ONEZONE_IA")]
    Onezoneia,

    /// OUTPOSTS
    #[serde(rename = "OUTPOSTS")]
    Outposts,

    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

    /// STANDARD_IA
    #[serde(rename = "STANDARD_IA")]
    Standardia,
}

impl Default for LocationS3S3StorageClassEnum {
    fn default() -> Self {
        LocationS3S3StorageClassEnum::Deeparchive
    }
}

impl cfn_resources::CfnResource for CfnLocationS3 {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationS3"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.s3_bucket_arn {
            if the_val.len() > 156 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_bucket_arn'. {} is greater than 156",
                    the_val.len()
                ));
            }
        }

        self.s3_config.validate()?;

        if let Some(the_val) = &self.subdirectory {
            if the_val.len() > 4096 as _ {
                return Err(format!(
                    "Max validation failed on field 'subdirectory'. {} is greater than 4096",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role used to access    an Amazon S3 bucket.
///
/// For detailed information about using such a role, see Creating a     Location for Amazon S3 in the         AWS DataSync User    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Config {
    ///
    /// The ARN of the IAM role for accessing the S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):iam::[0-9]{12}:role/.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "BucketAccessRoleArn")]
    pub bucket_access_role_arn: String,
}

impl cfn_resources::CfnResource for S3Config {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket_access_role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket_access_role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

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
