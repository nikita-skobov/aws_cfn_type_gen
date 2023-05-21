/// AWS::IoTAnalytics::Datastore resource is a repository for messages. For more information, see          How to Use AWS IoT Analytics in the AWS IoT Analytics User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDatastore {
    ///
    /// The name of the data store.
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
    #[serde(rename = "DatastoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,

    ///
    /// Information about the partition dimensions in a data store.
    ///
    /// Required: No
    ///
    /// Type: DatastorePartitions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatastorePartitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_partitions: Option<DatastorePartitions>,

    ///
    /// Where data store data is stored.
    ///
    /// Required: No
    ///
    /// Type: DatastoreStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatastoreStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_storage: Option<DatastoreStorage>,

    ///
    /// Contains the configuration information of file formats. AWS IoT Analytics data stores support JSON and Parquet.
    ///
    /// The default file format is JSON. You can specify only one format.
    ///
    /// You can't change the file format after you create the data store.
    ///
    /// Required: No
    ///
    /// Type: FileFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileFormatConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format_configuration: Option<FileFormatConfiguration>,

    ///
    /// How long, in days, message data is kept for the data store. When     customerManagedS3 storage is selected, this parameter is ignored.
    ///
    /// Required: No
    ///
    /// Type: RetentionPeriod
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,

    ///
    /// Metadata which can be used to manage the data store.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnDatastore {
    fn type_string(&self) -> &'static str {
        "AWS::IoTAnalytics::Datastore"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.datastore_name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'datastore_name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.datastore_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'datastore_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.datastore_partitions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.datastore_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.file_format_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.retention_period
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// Contains information about a column that stores your data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Column {
    ///
    /// The name of the column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The type of data. For more information about the supported data types, see Common data types    in the         AWS Glue Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,
}

impl cfn_resources::CfnResource for Column {
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

/// S3-customer-managed; When you choose customer-managed storage, the retentionPeriod parameter is ignored. You can't change the choice of Amazon S3 storage after your data store is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomerManagedS3 {
    ///
    /// The name of the Amazon S3 bucket where your data is stored.
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
    /// (Optional) The prefix used to create the keys of the data store data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier in the bucket. Each object in a bucket has exactly one key. The prefix must end with a forward slash (/).
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,

    ///
    /// The ARN of the role that grants AWS IoT Analytics permission to interact with your Amazon S3 resources.
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.bucket;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'bucket'. {} is less than 3",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.key_prefix {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'key_prefix'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.key_prefix {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key_prefix'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 20 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 20",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Amazon S3-customer-managed; When you choose customer-managed storage, the retentionPeriod parameter is ignored.      You can't change the choice of Amazon S3 storage after your data store is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomerManagedS3Storage {
    ///
    /// The name of the Amazon S3 bucket where your data is stored.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

    ///
    /// (Optional) The prefix used to create the keys of the data store data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier in the bucket.      Each object in a bucket has exactly one key. The prefix must end with a forward slash (/).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
}

impl cfn_resources::CfnResource for CustomerManagedS3Storage {
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

/// A single dimension to partition a data store. The dimension must be an AttributePartition or a TimestampPartition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatastorePartition {
    ///
    /// A partition dimension defined by an attribute.
    ///
    /// Required: No
    ///
    /// Type: Partition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<Partition>,

    ///
    /// A partition dimension defined by a timestamp attribute.
    ///
    /// Required: No
    ///
    /// Type: TimestampPartition
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampPartition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_partition: Option<TimestampPartition>,
}

impl cfn_resources::CfnResource for DatastorePartition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.partition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timestamp_partition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about the partition dimensions in a data store.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatastorePartitions {
    ///
    /// A list of partition dimensions in a data store.
    ///
    /// Required: No
    ///
    /// Type: List of DatastorePartition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<DatastorePartition>>,
}

impl cfn_resources::CfnResource for DatastorePartitions {
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

/// Where data store data is stored.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatastoreStorage {
    ///
    /// Use this to store data store data in an S3 bucket that you manage. The choice of     service-managed or customer-managed S3 storage cannot be changed after creation     of the data store.
    ///
    /// Required: No
    ///
    /// Type: CustomerManagedS3
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3: Option<CustomerManagedS3>,

    ///
    /// Use this to store data used by AWS IoT SiteWise in an Amazon S3 bucket that you manage.      You can't change the choice of Amazon S3 storage after your data store is created.
    ///
    /// Required: No
    ///
    /// Type: IotSiteWiseMultiLayerStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotSiteWiseMultiLayerStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise_multi_layer_storage: Option<IotSiteWiseMultiLayerStorage>,

    ///
    /// Use this to store data store data in an S3 bucket managed by the AWS IoT Analytics service.     The choice of service-managed or customer-managed S3 storage cannot be changed after creation     of the data store.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_s3: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for DatastoreStorage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.customer_managed_s3
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_site_wise_multi_layer_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains the configuration information of file formats. AWS IoT Analytics data stores support JSON and Parquet.
///
/// The default file format is JSON. You can specify only one format.
///
/// You can't change the file format after you create the data store.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileFormatConfiguration {
    ///
    /// Contains the configuration information of the JSON format.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_configuration: Option<serde_json::Value>,

    ///
    /// Contains the configuration information of the Parquet format.
    ///
    /// Required: No
    ///
    /// Type: ParquetConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParquetConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_configuration: Option<ParquetConfiguration>,
}

impl cfn_resources::CfnResource for FileFormatConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.parquet_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Stores data used by AWS IoT SiteWise in an Amazon S3 bucket that you manage.      You can't change the choice of Amazon S3 storage after your data store is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotSiteWiseMultiLayerStorage {
    ///
    /// Stores data used by AWS IoT SiteWise in an Amazon S3 bucket that you manage.
    ///
    /// Required: No
    ///
    /// Type: CustomerManagedS3Storage
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerManagedS3Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3_storage: Option<CustomerManagedS3Storage>,
}

impl cfn_resources::CfnResource for IotSiteWiseMultiLayerStorage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.customer_managed_s3_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains the configuration information of the Parquet format.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ParquetConfiguration {
    ///
    /// Information needed to define a schema.
    ///
    /// Required: No
    ///
    /// Type: SchemaDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_definition: Option<SchemaDefinition>,
}

impl cfn_resources::CfnResource for ParquetConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.schema_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A single dimension to partition a data store. The dimension must be an AttributePartition or a TimestampPartition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Partition {
    ///
    /// The name of the attribute that defines a partition dimension.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
}

impl cfn_resources::CfnResource for Partition {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
}

impl cfn_resources::CfnResource for RetentionPeriod {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.number_of_days {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'number_of_days'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Information needed to define a schema.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SchemaDefinition {
    ///
    /// Specifies one or more columns that store your data.
    ///
    /// Each schema can have up to 100 columns. Each column can have up to 100 nested    types.
    ///
    /// Required: No
    ///
    /// Type: List of Column
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
}

impl cfn_resources::CfnResource for SchemaDefinition {
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

/// A partition dimension defined by a timestamp attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimestampPartition {
    ///
    /// The attribute name of the partition defined by a timestamp.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

    ///
    /// The timestamp format of a partition defined by a timestamp. The default format is seconds since epoch (January 1, 1970 at midnight UTC time).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
}

impl cfn_resources::CfnResource for TimestampPartition {
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
