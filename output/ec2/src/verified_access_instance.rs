

/// Describes a Verified Access instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVerifiedAccessInstance {


    /// 
    /// A description for the AWS Verified Access instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The current logging configuration for the Verified Access instances.
    /// 
    /// Required: No
    ///
    /// Type: VerifiedAccessLogs
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfigurations")]
    pub logging_configurations: Option<VerifiedAccessLogs>,


    /// 
    /// The tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The IDs of the AWS Verified Access trust providers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessTrustProviderIds")]
    pub verified_access_trust_provider_ids: Option<Vec<String>>,


    /// 
    /// The IDs of the AWS Verified Access trust providers.
    /// 
    /// Required: No
    ///
    /// Type: List of VerifiedAccessTrustProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessTrustProviders")]
    pub verified_access_trust_providers: Option<Vec<VerifiedAccessTrustProvider>>,

}



impl cfn_resources::CfnResource for CfnVerifiedAccessInstance {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VerifiedAccessInstance"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.logging_configurations.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Options for CloudWatch Logs as a logging destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogs {


    /// 
    /// Indicates whether logging is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The ID of the CloudWatch Logs log group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,

}



impl cfn_resources::CfnResource for CloudWatchLogs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Options for Kinesis as a logging destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisDataFirehose {


    /// 
    /// The ID of the delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: Option<String>,


    /// 
    /// Indicates whether logging is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}



impl cfn_resources::CfnResource for KinesisDataFirehose {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Options for Amazon S3 as a logging destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3 {


    /// 
    /// The bucket name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// The AWS account number that owns the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,


    /// 
    /// Indicates whether logging is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The bucket prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}



impl cfn_resources::CfnResource for S3 {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes the destinations for Verified Access logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VerifiedAccessLogs {


    /// 
    /// CloudWatch Logs logging destination.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLogs
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogs>,


    /// 
    /// Kinesis logging destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisDataFirehose
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisDataFirehose")]
    pub kinesis_data_firehose: Option<KinesisDataFirehose>,


    /// 
    /// Amazon S3 logging options.
    /// 
    /// Required: No
    ///
    /// Type: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3>,

}



impl cfn_resources::CfnResource for VerifiedAccessLogs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cloud_watch_logs.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.kinesis_data_firehose.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.s3.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a Verified Access trust provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VerifiedAccessTrustProvider {


    /// 
    /// A description for the AWS Verified Access trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The type of device-based trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: crowdstrike | jamf
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceTrustProviderType")]
    pub device_trust_provider_type: Option<VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum>,


    /// 
    /// The type of Verified Access trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: device | user
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustProviderType")]
    pub trust_provider_type: Option<VerifiedAccessTrustProviderTrustProviderTypeEnum>,


    /// 
    /// The type of user-based trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: iam-identity-center | oidc
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserTrustProviderType")]
    pub user_trust_provider_type: Option<VerifiedAccessTrustProviderUserTrustProviderTypeEnum>,


    /// 
    /// The ID of the AWS Verified Access trust provider.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VerifiedAccessTrustProviderId")]
    pub verified_access_trust_provider_id: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum {

    /// crowdstrike
    #[serde(rename = "crowdstrike")]
    Crowdstrike,

    /// jamf
    #[serde(rename = "jamf")]
    Jamf,

}

impl Default for VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderDeviceTrustProviderTypeEnum::Crowdstrike
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VerifiedAccessTrustProviderTrustProviderTypeEnum {

    /// device
    #[serde(rename = "device")]
    Device,

    /// user
    #[serde(rename = "user")]
    User,

}

impl Default for VerifiedAccessTrustProviderTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderTrustProviderTypeEnum::Device
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VerifiedAccessTrustProviderUserTrustProviderTypeEnum {

    /// iam-identity-center
    #[serde(rename = "iam-identity-center")]
    Iamidentitycenter,

    /// oidc
    #[serde(rename = "oidc")]
    Oidc,

}

impl Default for VerifiedAccessTrustProviderUserTrustProviderTypeEnum {
    fn default() -> Self {
        VerifiedAccessTrustProviderUserTrustProviderTypeEnum::Iamidentitycenter
    }
}


impl cfn_resources::CfnResource for VerifiedAccessTrustProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}