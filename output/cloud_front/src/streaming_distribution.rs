/// This resource is deprecated.     Amazon CloudFront is deprecating real-time messaging protocol (RTMP) distributions on December 31, 2020.     For more information, read the announcement on the Amazon CloudFront discussion forum.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStreamingDistribution {
    ///
    /// The current configuration information for the RTMP distribution.
    ///
    /// Required: Yes
    ///
    /// Type: StreamingDistributionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamingDistributionConfig")]
    pub streaming_distribution_config: StreamingDistributionConfig,

    ///
    /// A complex type that contains zero or more Tag elements.
    ///
    /// Required: Yes
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

impl cfn_resources::CfnResource for CfnStreamingDistribution {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFront::StreamingDistribution"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.streaming_distribution_config.validate()?;

        Ok(())
    }
}

/// A complex type that controls whether access logs are written for the streaming 			distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Logging {
    ///
    /// The Amazon S3 bucket to store the access logs in, for example, 				myawslogbucket.s3.amazonaws.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: cfn_resources::StrVal,

    ///
    /// Specifies whether you want CloudFront to save access logs to an Amazon S3 bucket. If you don't 			want to enable logging when you create a streaming distribution or if you want to 			disable logging for an existing streaming distribution, specify false for 				Enabled, and specify empty Bucket and Prefix 			elements. If you specify false for Enabled but you specify 			values for Bucket and Prefix, the values are automatically 			deleted.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// An optional string that you want CloudFront to prefix to the access log filenames for this 			streaming distribution, for example, myprefix/. If you want to enable 			logging, but you don't want to specify a prefix, you still must include an empty 				Prefix element in the Logging element.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Logging {
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

/// A complex type that contains information about the Amazon S3 bucket from which you want 			CloudFront to get your media files for distribution.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Origin {
    ///
    /// The DNS name of the Amazon S3 origin.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// The CloudFront origin access identity to associate with the distribution. Use an origin 			access identity to configure the distribution so that end users can only access objects 			in an Amazon S3 bucket through CloudFront.
    ///
    /// If you want end users to be able to access objects using either the CloudFront URL or the 			Amazon S3 URL, specify an empty OriginAccessIdentity element.
    ///
    /// To delete the origin access identity from an existing distribution, update the 			distribution configuration and include an empty OriginAccessIdentity 			element.
    ///
    /// To replace the origin access identity, update the distribution configuration and 			specify the new origin access identity.
    ///
    /// For more information, see Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content in 			the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OriginAccessIdentity")]
    pub origin_access_identity: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for S3Origin {
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

/// The RTMP distribution's configuration information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StreamingDistributionConfig {
    ///
    /// A complex type that contains information about CNAMEs (alternate domain names), if 			any, for this streaming distribution.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    ///
    /// Any comments you want to include about the streaming distribution.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: cfn_resources::StrVal,

    ///
    /// Whether the streaming distribution is enabled to accept user requests for 			content.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// A complex type that controls whether access logs are written for the streaming 			distribution.
    ///
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    ///
    /// A complex type that contains information about price class for this streaming 			distribution.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PriceClass_100 | PriceClass_200 | PriceClass_All
    ///
    /// Update requires: No interruption
    #[serde(rename = "PriceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<StreamingDistributionConfigPriceClassEnum>,

    ///
    /// A complex type that contains information about the Amazon S3 bucket from which you want 			CloudFront to get your media files for distribution.
    ///
    /// Required: Yes
    ///
    /// Type: S3Origin
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Origin")]
    pub s3_origin: S3Origin,

    ///
    /// A complex type that specifies any AWS accounts that you want to permit to create 			signed URLs for private content. If you want the distribution to use signed URLs, 			include this element; if you want the distribution to use public URLs, remove this 			element. For more information, see Serving Private 				Content through CloudFront in the Amazon CloudFront Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: TrustedSigners
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedSigners")]
    pub trusted_signers: TrustedSigners,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum StreamingDistributionConfigPriceClassEnum {
    /// PriceClass_100
    #[serde(rename = "PriceClass_100")]
    Priceclass100,

    /// PriceClass_200
    #[serde(rename = "PriceClass_200")]
    Priceclass200,

    /// PriceClass_All
    #[serde(rename = "PriceClass_All")]
    Priceclassall,
}

impl Default for StreamingDistributionConfigPriceClassEnum {
    fn default() -> Self {
        StreamingDistributionConfigPriceClassEnum::Priceclass100
    }
}

impl cfn_resources::CfnResource for StreamingDistributionConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.logging.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.s3_origin.validate()?;

        self.trusted_signers.validate()?;

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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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

/// A list of AWS accounts whose public keys CloudFront can use to verify the signatures of 			signed URLs and signed cookies.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrustedSigners {
    ///
    /// An AWS account number that contains active CloudFront key pairs that CloudFront can use to 			verify the signatures of signed URLs and signed cookies. If the AWS account that owns 			the key pairs is the same account that owns the CloudFront distribution, the value of this 			field is self.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsAccountNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_numbers: Option<Vec<String>>,

    ///
    /// This field is true if any of the AWS accounts have public keys that 			CloudFront can use to verify the signatures of signed URLs and signed cookies. If not, this 			field is false.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for TrustedSigners {
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
