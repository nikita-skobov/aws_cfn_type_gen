
pub mod cfn_keyspace {

#[derive(serde::Serialize, Default)]
pub struct CfnKeyspace {
    /// Name for Cassandra keyspace
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_table {

#[derive(serde::Serialize, Default)]
pub struct CfnTable {
    /// An array of key-value pairs to apply to this resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Clustering key columns of the table
    #[serde(rename = "ClusteringKeyColumns")]
    pub clustering_key_columns: Option<Vec<ClusteringKeyColumn>>,
    /// Non-key columns of the table
    #[serde(rename = "RegularColumns")]
    pub regular_columns: Option<Vec<Column>>,
    /// No documentation provided by AWS
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<BillingMode>,
    /// Partition key columns of the table
    #[serde(rename = "PartitionKeyColumns")]
    pub partition_key_columns: Vec<Column>,
    /// Indicates whether client side timestamps are enabled (true) or disabled (false) on the table. False by default, once it is enabled it cannot be disabled again.
    #[serde(rename = "ClientSideTimestampsEnabled")]
    pub client_side_timestamps_enabled: Option<bool>,
    /// Indicates whether point in time recovery is enabled (true) or disabled (false) on the table
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,
    /// Default TTL (Time To Live) in seconds, where zero is disabled. If the value is greater than zero, TTL is enabled for the entire table and an expiration timestamp is added to each column.
    #[serde(rename = "DefaultTimeToLive")]
    pub default_time_to_live: Option<usize>,
    /// Name for Cassandra table
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    /// Name for Cassandra keyspace
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: String,
    /// Represents the settings used to enable server-side encryption
    #[serde(rename = "EncryptionSpecification")]
    pub encryption_specification: Option<EncryptionSpecification>,

}


#[derive(serde::Serialize, Default)]
pub struct BillingMode {
    #[serde(rename = "Mode")]
    pub mode: Mode,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionSpecification {
    #[serde(rename = "KmsKeyIdentifier")]
    pub kms_key_identifier: Option<KmsKeyIdentifier>,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: EncryptionType,

}
pub type Mode = String;
#[derive(serde::Serialize, Default)]
pub struct ClusteringKeyColumn {
    #[serde(rename = "OrderBy")]
    pub order_by: Option<String>,
    #[serde(rename = "Column")]
    pub column: Column,

}
pub type EncryptionType = String;
#[derive(serde::Serialize, Default)]
pub struct Column {
    #[serde(rename = "ColumnType")]
    pub column_type: String,
    #[serde(rename = "ColumnName")]
    pub column_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: usize,
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type KmsKeyIdentifier = String;

}
