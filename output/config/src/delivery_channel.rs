/// Specifies a delivery channel object to deliver configuration 			information to an Amazon S3 bucket and Amazon SNS topic.
///
/// Before you can create a delivery channel, you must create a 			configuration recorder. You can use this action to change the Amazon S3 bucket or an 			Amazon SNS topic of the existing delivery channel. To change the 			Amazon S3 bucket or an Amazon SNS topic, call this action and 			specify the changed values for the S3 bucket and the SNS topic. If 			you specify a different value for either the S3 bucket or the SNS 			topic, this action will keep the existing value for the parameter 			that is not changed.
///
/// You can have only one delivery channel per region per AWS account, and the delivery channel is required to use AWS Config.
///
/// When you create the delivery channel, you can specify; how often AWS Config delivers configuration snapshots to your Amazon S3 bucket (for example, 24 hours),            the S3 bucket to which AWS Config sends configuration snapshots and configuration history files, and the       Amazon SNS topic to which AWS Config sends notifications about configuration changes, such as updated resources, AWS Config rule evaluations, and when AWS Config delivers the configuration snapshot to your S3 bucket.      For more information, see Deliver Configuration Items in the AWS Config Developer Guide.
///
/// For more information, see Managing the Delivery Channel in the AWS Config Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeliveryChannel {
    ///
    /// The options for how often AWS Config delivers configuration 			snapshots to the Amazon S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: ConfigSnapshotDeliveryProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigSnapshotDeliveryProperties")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,

    ///
    /// A name for the delivery channel. If you don't specify a name, AWS CloudFormation generates a       unique physical ID and uses that ID for the delivery channel name. For more information,       see Name Type.
    ///
    /// Updates are not supported. To change the name, you must run two separate updates. In the first update, delete this resource, and then recreate it with a new name in the second update.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The name of the Amazon S3 bucket to which AWS Config delivers 			configuration snapshots and configuration history files.
    ///
    /// If you specify a bucket that belongs to another AWS account, 			that bucket must have policies that grant access permissions to AWS Config. For more information, see Permissions for the Amazon S3 Bucket in the         AWS Config 			Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,

    ///
    /// The prefix for the specified Amazon S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Key Management Service (AWS KMS ) AWS KMS key (KMS key) used to encrypt objects delivered by AWS Config. 			Must belong to the same Region as the destination S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KmsKeyArn")]
    pub s3_kms_key_arn: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic to which 			AWS Config sends notifications about configuration 			changes.
    ///
    /// If you choose a topic from another account, the topic must have 			policies that grant access permissions to AWS Config. For more 			information, see Permissions for the Amazon SNS Topic in the         AWS Config 			Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicARN")]
    pub sns_topic_arn: Option<String>,
}

impl cfn_resources::CfnResource for CfnDeliveryChannel {
    fn type_string(&self) -> &'static str {
        "AWS::Config::DeliveryChannel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.config_snapshot_delivery_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Provides options for how often AWS Config delivers 			configuration snapshots to the Amazon S3 bucket in your delivery 			channel.
///
/// The frequency for a rule that triggers evaluations for your 			resources when AWS Config delivers the configuration snapshot is set 			by one of two values, depending on which is less frequent:
///
/// If the deliveryFrequency value is less frequent 			than the MaximumExecutionFrequency value for a rule, 			AWS Config invokes the rule only as often as the 				deliveryFrequency value.
///
/// You should set the MaximumExecutionFrequency value 			to be at least as frequent as the deliveryFrequency 			value. You can view the deliveryFrequency value by 			using the DescribeDeliveryChannnels action.
///
/// To update the deliveryFrequency with which AWS Config delivers your configuration snapshots, use the PutDeliveryChannel action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigSnapshotDeliveryProperties {
    ///
    /// The frequency with which AWS Config delivers configuration 			snapshots.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: One_Hour | Six_Hours | Three_Hours | Twelve_Hours | TwentyFour_Hours
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryFrequency")]
    pub delivery_frequency: Option<ConfigSnapshotDeliveryPropertiesDeliveryFrequencyEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConfigSnapshotDeliveryPropertiesDeliveryFrequencyEnum {
    /// One_Hour
    #[serde(rename = "One_Hour")]
    Onehour,

    /// Six_Hours
    #[serde(rename = "Six_Hours")]
    Sixhours,

    /// Three_Hours
    #[serde(rename = "Three_Hours")]
    Threehours,

    /// Twelve_Hours
    #[serde(rename = "Twelve_Hours")]
    Twelvehours,

    /// TwentyFour_Hours
    #[serde(rename = "TwentyFour_Hours")]
    Twentyfourhours,
}

impl Default for ConfigSnapshotDeliveryPropertiesDeliveryFrequencyEnum {
    fn default() -> Self {
        ConfigSnapshotDeliveryPropertiesDeliveryFrequencyEnum::Onehour
    }
}

impl cfn_resources::CfnResource for ConfigSnapshotDeliveryProperties {
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
