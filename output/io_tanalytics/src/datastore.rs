

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
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnDatastore {
    fn type_string() -> &'static str {
        "AWS::IoTAnalytics::Datastore"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    pub key_prefix: Option<String>,

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
    pub timestamp_partition: Option<TimestampPartition>,

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
    pub partitions: Option<Vec<DatastorePartition>>,

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
    pub service_managed_s3: Option<serde_json::Value>,

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
    pub parquet_configuration: Option<ParquetConfiguration>,

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
    pub customer_managed_s3_storage: Option<CustomerManagedS3Storage>,

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
    pub schema_definition: Option<SchemaDefinition>,

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
    pub columns: Option<Vec<Column>>,

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
    pub timestamp_format: Option<String>,

}


