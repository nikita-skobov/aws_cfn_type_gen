

/// You can use the AWS::Cassandra::Table resource to create a new table in       Amazon Keyspaces (for Apache Cassandra). For more information, see Create a         keyspace and a table in the Amazon Keyspaces Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTable {


    /// 
    /// One or more columns that are not part of the primary key - that is, columns that are         not defined as partition key columns or clustering key       columns.
    /// 
    /// You can add regular columns to existing tables by adding them to the template.
    /// 
    /// Required: No
    ///
    /// Type: List of Column
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegularColumns")]
    pub regular_columns: Option<Vec<Column>>,


    /// 
    /// The name of the table to be created. The table name is case sensitive. If you don't specify a name, AWS CloudFormation       generates a unique ID and uses that ID for the table name. For more information, see         Name       type.
    /// 
    /// ImportantIf you specify a name, you can't perform updates that require replacing this         resource. You can perform updates that require no interruption or some interruption. If you must         replace the resource, specify a new name.
    /// 
    /// Length constraints: Minimum length of 3. Maximum length of       255.
    /// 
    /// Pattern:       ^[a-zA-Z0-9][a-zA-Z0-9_]{1,47}$
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,


    /// 
    /// The encryption at rest options for the table.
    /// 
    /// AWS owned key (default) - The key is owned by Amazon Keyspaces.Customer managed key - The key is stored in your account and is created, owned, and         managed by you.           NoteIf you choose encryption with a customer managed key, you must specify             a valid customer managed KMS key with permissions granted to Amazon             Keyspaces.
    /// 
    /// For more information,       see Encryption at rest in Amazon Keyspaces       in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionSpecification")]
    pub encryption_specification: Option<EncryptionSpecification>,


    /// 
    /// The billing mode for the table, which determines how you'll be charged for reads and writes:
    /// 
    /// On-demand mode (default) - You pay based on           the actual reads and writes your application performs.               Provisioned mode - Lets you specify the           number of reads and writes per second that you need for your application.
    /// 
    /// If you don't specify a value for this property, then the table will use on-demand mode.
    /// 
    /// Required: No
    ///
    /// Type: BillingMode
    ///
    /// Update requires: No interruption
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,


    /// The default Time To Live (TTL) value for all rows in a table in seconds.     The maximum configurable value is 630,720,000 seconds, which is the equivalent of 20 years. By default, the TTL value for a table is 0, which means data does not expire.
    /// 
    /// For more information,       see Setting the default TTL value for a table       in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTimeToLive")]
    pub default_time_to_live: Option<i64>,


    /// 
    /// A list of key-value pair tags to be       attached to the resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies if point-in-time recovery is enabled or disabled for the table. The options are PointInTimeRecoveryEnabled=true and        PointInTimeRecoveryEnabled=false.       If not specified, the default is PointInTimeRecoveryEnabled=false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,


    /// 
    /// One or more columns that uniquely identify every row in the table. Every table must       have a partition key.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Column
    ///
    /// Update requires: Replacement
    #[serde(rename = "PartitionKeyColumns")]
    pub partition_key_columns: Vec<Column>,


    /// 
    /// One or more columns that determine how the table data is sorted.
    /// 
    /// Required: No
    ///
    /// Type: List of ClusteringKeyColumn
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusteringKeyColumns")]
    pub clustering_key_columns: Option<Vec<ClusteringKeyColumn>>,


    /// 
    /// The name of the keyspace to create the table in. The keyspace must already       exist.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: String,


    /// 
    /// Enables client-side timestamps for the table. By default, the setting is disabled.       You can enable client-side timestamps with the following option:
    /// 
    /// status: "enabled"
    /// 
    /// After client-side timestamps are enabled for a table, you can't disable this setting.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientSideTimestampsEnabled")]
    pub client_side_timestamps_enabled: Option<bool>,

}



impl cfn_resources::CfnResource for CfnTable {
    fn type_string() -> &'static str {
        "AWS::Cassandra::Table"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Defines an individual column within the clustering key.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusteringKeyColumn {


    /// 
    /// The name and data type of this clustering key column.
    /// 
    /// Required: Yes
    ///
    /// Type: Column
    ///
    /// Update requires: Replacement
    #[serde(rename = "Column")]
    pub column: Column,


    /// 
    /// The order in which this column's data is stored:
    /// 
    /// ASC (default) - The column's data is stored in ascending           order.               DESC - The column's data is stored in descending order.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrderBy")]
    pub order_by: Option<String>,

}




/// Specifies the encryption at rest option selected for the table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionSpecification {


    /// 
    /// The encryption at rest options for the table.
    /// 
    /// AWS owned key (default) - AWS_OWNED_KMS_KEYCustomer managed key - CUSTOMER_MANAGED_KMS_KEY         ImportantIf you choose CUSTOMER_MANAGED_KMS_KEY, a kms_key_identifier in the format of a           key ARN is required.
    /// 
    /// Valid values: CUSTOMER_MANAGED_KMS_KEY | AWS_OWNED_KMS_KEY.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,


    /// Requires a kms_key_identifier in the format of a   key ARN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyIdentifier")]
    pub kms_key_identifier: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}




/// The provisioned throughput for the table, which consists of         ReadCapacityUnits and WriteCapacityUnits.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProvisionedThroughput {


    /// 
    /// The amount of write capacity that's provisioned for the table. For more information,       see Read/write capacity         mode in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,


    /// 
    /// The amount of read capacity that's provisioned for the table. For more information,       see Read/write capacity         mode in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,

}




/// Determines the billing mode for the table - on-demand or provisioned.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BillingMode {


    /// 
    /// The billing mode for the table:
    /// 
    /// On-demand mode - ON_DEMAND               Provisioned mode - PROVISIONED         NoteIf you choose PROVISIONED mode, then you also need to specify             provisioned throughput (read and write capacity) for the table.
    /// 
    /// Valid values: ON_DEMAND | PROVISIONED
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: String,


    /// 
    /// The provisioned read capacity and write capacity for the table. For more information,       see Provisioned throughput capacity mode       in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: ProvisionedThroughput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,

}




/// The name and data type of an individual column in a table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Column {


    /// 
    /// The name of the column. For more information, see Identifiers in the       Amazon Keyspaces Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ColumnName")]
    pub column_name: String,


    /// 
    /// The data type of the column. For more information, see Data types       in the Amazon Keyspaces Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ColumnType")]
    pub column_type: String,

}


