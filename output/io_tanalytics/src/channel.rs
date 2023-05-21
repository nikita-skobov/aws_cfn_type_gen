

/// The AWS::IoTAnalytics::Channel resource collects data from an MQTT topic and archives the      raw, unprocessed messages before publishing the data to a pipeline. For more information, see             How to Use AWS IoT Analytics in the AWS IoT Analytics User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnChannel {


    /// 
    /// The name of the channel.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: (^(?!_{2}))(^[a-zA-Z0-9_]+$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,


    /// 
    /// Where channel data is stored.
    /// 
    /// Required: No
    ///
    /// Type: ChannelStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelStorage")]
    pub channel_storage: Option<ChannelStorage>,


    /// 
    /// How long, in days, message data is kept for the channel.
    /// 
    /// Required: No
    ///
    /// Type: RetentionPeriod
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<RetentionPeriod>,


    /// 
    /// Metadata which can be used to manage the channel.
    /// 
    /// For more information, see Tag.
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



impl cfn_resources::CfnResource for CfnChannel {
    fn type_string(&self) -> &'static str {
        "AWS::IoTAnalytics::Channel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.channel_name {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'channel_name'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.channel_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'channel_name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.channel_storage.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.retention_period.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Where channel data is stored. You may choose one of serviceManagedS3,     customerManagedS3 storage. If not specified, the default is     serviceManagedS3. This can't be changed after creation of the channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChannelStorage {


    /// 
    /// Used to store channel data in an S3 bucket that you manage. If customer managed storage is    selected, the retentionPeriod parameter is ignored. You can't change the choice    of S3 storage after the data store is created.
    /// 
    /// Required: No
    ///
    /// Type: CustomerManagedS3
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerManagedS3")]
    pub customer_managed_s3: Option<CustomerManagedS3>,


    /// 
    /// Used to store channel data in an S3 bucket managed by AWS IoT Analytics. You can't change the choice    of S3 storage after the data store is created.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceManagedS3")]
    pub service_managed_s3: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for ChannelStorage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.customer_managed_s3.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Used to store channel data in an S3 bucket that you manage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomerManagedS3 {


    /// 
    /// The name of the S3 bucket in which channel data is stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9.\-_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// (Optional) The prefix used to create the keys of the channel data objects. Each object in    an S3 bucket has a key that is its unique identifier within the bucket (each object in a    bucket has exactly one key). The prefix must end with a forward slash (/).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9!_.*'()/{}:-]*/$
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPrefix")]
    pub key_prefix: Option<String>,


    /// 
    /// The ARN of the role that grants AWS IoT Analytics permission to interact with your Amazon S3    resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



impl cfn_resources::CfnResource for CustomerManagedS3 {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.bucket;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'bucket'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.bucket;

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'bucket'. {} is less than 3", the_val.len()));
        }

        
        if let Some(the_val) = &self.key_prefix {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'key_prefix'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.key_prefix {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'key_prefix'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'role_arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.role_arn;

        if the_val.len() < 20 as _ {
            return Err(format!("Min validation failed on field 'role_arn'. {} is less than 20", the_val.len()));
        }

        
        Ok(())
    }
}

/// How long, in days, message data is kept.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetentionPeriod {


    /// 
    /// The number of days that message data is kept. The unlimited parameter must be    false.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfDays")]
    pub number_of_days: Option<i64>,


    /// 
    /// If true, message data is kept indefinitely.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,

}



impl cfn_resources::CfnResource for RetentionPeriod {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.number_of_days {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'number_of_days'. {} is less than 1", the_val));
        }

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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}