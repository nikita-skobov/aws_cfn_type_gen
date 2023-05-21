

/// The AWS::DynamoDB::GlobalTable resource enables you to create and manage       a Version 2019.11.21 global table. This resource cannot be used to create or manage a       Version 2017.11.29 global table. For more information, see Global       tables.
///
/// You should be aware of the following behaviors when working with DynamoDB global       tables.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGlobalTable {


    /// 
    /// A list of attributes that describe the key schema for the global table and       indexes.
    /// 
    /// Required: Yes
    ///
    /// Type: List of AttributeDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,


    /// 
    /// Specifies how you are charged for read and write throughput and how you manage       capacity. Valid values are:
    /// 
    /// PAY_PER_REQUEST               PROVISIONED
    /// 
    /// All replicas in your global table will have the same billing mode. If you use         PROVISIONED billing mode, you must provide an auto scaling       configuration via the WriteProvisionedThroughputSettings property. The       default value of this property is PROVISIONED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PAY_PER_REQUEST | PROVISIONED
    ///
    /// Update requires: No interruption
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<GlobalTableBillingModeEnum>,


    /// 
    /// Global secondary indexes to be created on the global table. You can create up to 20       global secondary indexes. Each replica in your global table will have the same global       secondary index settings. You can only create or delete one global secondary index in a       single stack operation.
    /// 
    /// Since the backfilling of an index could take a long time, CloudFormation does not wait       for the index to become active. If a stack operation rolls back, CloudFormation might       not delete an index that has been added. In that case, you will need to delete the index       manually.
    /// 
    /// Required: No
    ///
    /// Type: List of GlobalSecondaryIndex
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,


    /// 
    /// Specifies the attributes that make up the primary key for the table. The attributes in       the KeySchema property must also be defined in the         AttributeDefinitions property.
    /// 
    /// Required: Yes
    ///
    /// Type: List of KeySchema
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,


    /// 
    /// Local secondary indexes to be created on the table. You can create up to five local       secondary indexes. Each index is scoped to a given hash key value. The size of each hash       key can be up to 10 gigabytes. Each replica in your global table will have the same       local secondary index settings.
    /// 
    /// Required: No
    ///
    /// Type: List of LocalSecondaryIndex
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,


    /// 
    /// Specifies the list of replicas for your global table. The list must contain at least       one element, the region where the stack defining the global table is deployed. For       example, if you define your table in a stack deployed to us-east-1, you must have an       entry in Replicas with the region us-east-1. You cannot remove the replica       in the stack region.
    /// 
    /// ImportantAdding a replica might take a few minutes for an empty table, or up to several         hours for large tables. If you want to add or remove a replica, we recommend         submitting an UpdateStack operation containing only that change.If you add or delete a replica during an update, we recommend that you don't         update any other resources. If your stack fails to update and is rolled back while         adding a new replica, you might need to manually delete the replica.
    /// 
    /// You can create a new global table with as many replicas as needed. You can add or       remove replicas after table creation, but you can only add or remove a single replica in       each update.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ReplicaSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "Replicas")]
    pub replicas: Vec<ReplicaSpecification>,


    /// 
    /// Specifies the settings to enable server-side encryption. These settings will be       applied to all replicas. If you plan to use customer-managed KMS keys, you must provide       a key for each replica using the         ReplicaSpecification.ReplicaSSESpecification property.
    /// 
    /// Required: No
    ///
    /// Type: SSESpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<SSESpecification>,


    /// 
    /// Specifies the streams settings on your global table. You must provide a value for this       property if your global table contains more than one replica. You can only change the       streams settings if your global table has only one replica.
    /// 
    /// Required: No
    ///
    /// Type: StreamSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,


    /// 
    /// A name for the global table. If you don't specify a name, AWS CloudFormation       generates a unique ID and uses that ID as the table name. For more information, see         Name       type.
    /// 
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this         resource. You can perform updates that require no or some interruption. If you must         replace the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,


    /// 
    /// Specifies the time to live (TTL) settings for the table. This setting will be applied       to all replicas.
    /// 
    /// Required: No
    ///
    /// Type: TimeToLiveSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,


    /// 
    /// Specifies an auto scaling policy for write capacity. This policy will be applied to       all replicas. This setting must be specified if BillingMode is set to         PROVISIONED.
    /// 
    /// Required: No
    ///
    /// Type: WriteProvisionedThroughputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteProvisionedThroughputSettings")]
    pub write_provisioned_throughput_settings: Option<WriteProvisionedThroughputSettings>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum GlobalTableBillingModeEnum {

    /// PAY_PER_REQUEST
    #[serde(rename = "PAY_PER_REQUEST")]
    Payperrequest,

    /// PROVISIONED
    #[serde(rename = "PROVISIONED")]
    Provisioned,

}

impl Default for GlobalTableBillingModeEnum {
    fn default() -> Self {
        GlobalTableBillingModeEnum::Payperrequest
    }
}


impl cfn_resources::CfnResource for CfnGlobalTable {
    fn type_string() -> &'static str {
        "AWS::DynamoDB::GlobalTable"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.ssespecification.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.stream_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.table_name {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'table_name'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.table_name {

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'table_name'. {} is less than 3", the_val.len()));
        }

        }
        
        self.time_to_live_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.write_provisioned_throughput_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the type and format of extension access. Only one of 				CustomObjectIdentifier or AccessMethodType may be 			provided. Providing both results in InvalidArgsException.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AttributeDefinition {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html#cfn-dynamodb-globaltable-attributedefinition-attributename
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html#cfn-dynamodb-globaltable-attributedefinition-attributetype
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,

}



impl cfn_resources::CfnResource for AttributeDefinition {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Configures a scalable target and an autoscaling policy for a table or global secondary       index's read or write capacity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CapacityAutoScalingSettings {


    /// 
    /// The maximum provisioned capacity units for the global table.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,


    /// 
    /// The minimum provisioned capacity units for the global table.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,


    /// 
    /// When switching billing mode from PAY_PER_REQUEST to         PROVISIONED, DynamoDB requires you to specify read and write capacity       unit values for the table and for each global secondary index. These values will be       applied to all replicas. The table will use these provisioned values until       CloudFormation creates the autoscaling policies you configured in your template.       CloudFormation cannot determine what capacity the table and its global secondary indexes       will require in this time period, since they are application-dependent.
    /// 
    /// If you want to switch a table's billing mode from PAY_PER_REQUEST to         PROVISIONED, you must specify a value for this property for each       autoscaled resource. If you specify different values for the same resource in different       regions, CloudFormation will use the highest value found in either the         SeedCapacity or ReadCapacityUnits properties. For example,       if your global secondary index myGSI has a SeedCapacity of 10       in us-east-1 and a fixed ReadCapacityUnits of 20 in eu-west-1,       CloudFormation will initially set the read capacity for myGSI to 20. Note       that if you disable ScaleIn for myGSI in us-east-1, its read       capacity units might not be set back to 10.
    /// 
    /// You must also specify a value for SeedCapacity when you plan to switch a       table's billing mode from PROVISIONED to PAY_PER_REQUEST,       because CloudFormation might need to roll back the operation (reverting the billing mode       to PROVISIONED) and this cannot succeed without specifying a value for         SeedCapacity.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SeedCapacity")]
    pub seed_capacity: Option<i64>,


    /// 
    /// Defines a target tracking scaling policy.
    /// 
    /// Required: Yes
    ///
    /// Type: TargetTrackingScalingPolicyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: TargetTrackingScalingPolicyConfiguration,

}



impl cfn_resources::CfnResource for CapacityAutoScalingSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.target_tracking_scaling_policy_configuration.validate()?;

        Ok(())
    }
}

/// Configures contributor insights settings for a replica or one of its indexes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContributorInsightsSpecification {


    /// 
    /// Indicates whether CloudWatch Contributor Insights are to be enabled (true) or disabled       (false).
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Allowed values: DISABLE | ENABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}



impl cfn_resources::CfnResource for ContributorInsightsSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Allows you to specify a global secondary index for the global table. The index will be       defined on all replicas.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlobalSecondaryIndex {


    /// 
    /// The name of the global secondary index. The name must be unique among all other       indexes on this table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// The complete key schema for a global secondary index, which consists of one or more       pairs of attribute names and key types:
    /// 
    /// HASH - partition key                          RANGE - sort key
    /// 
    /// NoteThe partition key of an item is also known as its hash           attribute. The term "hash attribute" derives from DynamoDB's usage of         an internal hash function to evenly distribute data items across partitions, based         on their partition key values.The sort key of an item is also known as its range attribute.         The term "range attribute" derives from the way DynamoDB stores items with the same         partition key physically close together, in sorted order by the sort key         value.
    /// 
    /// Required: Yes
    ///
    /// Type: List of KeySchema
    ///
    /// Maximum: 2
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,


    /// 
    /// Represents attributes that are copied (projected) from the table into the global       secondary index. These are in addition to the primary key attributes and index key       attributes, which are automatically projected.
    /// 
    /// Required: Yes
    ///
    /// Type: Projection
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Projection")]
    pub projection: Projection,


    /// 
    /// Defines write capacity settings for the global secondary index. You must specify a       value for this property if the table's BillingMode is         PROVISIONED. All replicas will have the same write capacity settings       for this global secondary index.
    /// 
    /// Required: No
    ///
    /// Type: WriteProvisionedThroughputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteProvisionedThroughputSettings")]
    pub write_provisioned_throughput_settings: Option<WriteProvisionedThroughputSettings>,

}



impl cfn_resources::CfnResource for GlobalSecondaryIndex {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.index_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'index_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.index_name;

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'index_name'. {} is less than 3", the_val.len()));
        }

        
        let the_val = &self.key_schema;

        if the_val.len() > 2 as _ {
            return Err(format!("Max validation failed on field 'key_schema'. {} is greater than 2", the_val.len()));
        }

        
        self.projection.validate()?;

        self.write_provisioned_throughput_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents a single element of a key schema. A key schema       specifies the attributes that make up the primary key of a table, or the key attributes       of an index.
///
/// A KeySchemaElement represents exactly one attribute of the primary key.       For example, a simple primary key would be represented by one         KeySchemaElement (for the partition key). A composite primary key would       require one KeySchemaElement for the partition key, and another         KeySchemaElement for the sort key.
///
/// A KeySchemaElement must be a scalar, top-level attribute (not a nested       attribute). The data type must be one of String, Number, or Binary. The attribute cannot       be nested within a List or a Map.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeySchema {


    /// 
    /// The name of a key attribute.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,


    /// 
    /// The role that this key attribute will assume:
    /// 
    /// HASH - partition key                          RANGE - sort key
    /// 
    /// NoteThe partition key of an item is also known as its hash           attribute. The term "hash attribute" derives from DynamoDB's usage of         an internal hash function to evenly distribute data items across partitions, based         on their partition key values.The sort key of an item is also known as its range attribute.         The term "range attribute" derives from the way DynamoDB stores items with the same         partition key physically close together, in sorted order by the sort key         value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: HASH | RANGE
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyType")]
    pub key_type: KeySchemaKeyTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum KeySchemaKeyTypeEnum {

    /// HASH
    #[serde(rename = "HASH")]
    Hash,

    /// RANGE
    #[serde(rename = "RANGE")]
    Range,

}

impl Default for KeySchemaKeyTypeEnum {
    fn default() -> Self {
        KeySchemaKeyTypeEnum::Hash
    }
}


impl cfn_resources::CfnResource for KeySchema {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.attribute_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'attribute_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.attribute_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'attribute_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The Kinesis Data Streams configuration for the specified global table replica.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamSpecification {


    /// 
    /// The ARN for a specific Kinesis data stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 37
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,

}



impl cfn_resources::CfnResource for KinesisStreamSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.stream_arn;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'stream_arn'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.stream_arn;

        if the_val.len() < 37 as _ {
            return Err(format!("Min validation failed on field 'stream_arn'. {} is less than 37", the_val.len()));
        }

        
        Ok(())
    }
}

/// Represents the properties of a local secondary index. A local secondary index can only       be created when its parent table is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LocalSecondaryIndex {


    /// 
    /// The name of the local secondary index. The name must be unique among all other indexes       on this table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// The complete key schema for the local secondary index, consisting of one or more pairs       of attribute names and key types:
    /// 
    /// HASH - partition key                        RANGE - sort key
    /// 
    /// NoteThe partition key of an item is also known as its hash           attribute. The term "hash attribute" derives from DynamoDB's usage of         an internal hash function to evenly distribute data items across partitions, based         on their partition key values.The sort key of an item is also known as its range attribute.         The term "range attribute" derives from the way DynamoDB stores items with the same         partition key physically close together, in sorted order by the sort key         value.
    /// 
    /// Required: Yes
    ///
    /// Type: List of KeySchema
    ///
    /// Maximum: 2
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,


    /// 
    /// Represents attributes that are copied (projected) from the table into the local       secondary index. These are in addition to the primary key attributes and index key       attributes, which are automatically projected.
    /// 
    /// Required: Yes
    ///
    /// Type: Projection
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Projection")]
    pub projection: Projection,

}



impl cfn_resources::CfnResource for LocalSecondaryIndex {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.index_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'index_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.index_name;

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'index_name'. {} is less than 3", the_val.len()));
        }

        
        let the_val = &self.key_schema;

        if the_val.len() > 2 as _ {
            return Err(format!("Max validation failed on field 'key_schema'. {} is greater than 2", the_val.len()));
        }

        
        self.projection.validate()?;

        Ok(())
    }
}

/// Represents the settings used to enable point in time recovery.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PointInTimeRecoverySpecification {


    /// 
    /// Indicates whether point in time recovery is enabled (true) or disabled (false) on the       table.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,

}



impl cfn_resources::CfnResource for PointInTimeRecoverySpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Represents attributes that are copied (projected) from the table into an index. These       are in addition to the primary key attributes and index key attributes, which are       automatically projected.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Projection {


    /// 
    /// Represents the non-key attribute names which will be projected into the index.
    /// 
    /// For local secondary indexes, the total count of NonKeyAttributes summed       across all of the local secondary indexes, must not exceed 100. If you project the same       attribute into two different indexes, this counts as two distinct attributes when       determining the total.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,


    /// 
    /// The set of attributes that are projected into the index:
    /// 
    /// KEYS_ONLY - Only the index and primary keys are projected into the           index.                        INCLUDE - In addition to the attributes described in             KEYS_ONLY, the secondary index will include other non-key           attributes that you specify.                        ALL - All of the table attributes are projected into the           index.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | INCLUDE | KEYS_ONLY
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ProjectionType")]
    pub projection_type: Option<ProjectionProjectionTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ProjectionProjectionTypeEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// INCLUDE
    #[serde(rename = "INCLUDE")]
    Include,

    /// KEYS_ONLY
    #[serde(rename = "KEYS_ONLY")]
    Keysonly,

}

impl Default for ProjectionProjectionTypeEnum {
    fn default() -> Self {
        ProjectionProjectionTypeEnum::All
    }
}


impl cfn_resources::CfnResource for Projection {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.non_key_attributes {

        if the_val.len() > 20 as _ {
            return Err(format!("Max validation failed on field 'non_key_attributes'. {} is greater than 20", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Allows you to specify the read capacity settings for a replica table or a replica       global secondary index when the BillingMode is set to         PROVISIONED. You must specify a value for either         ReadCapacityUnits or ReadCapacityAutoScalingSettings, but       not both. You can switch between fixed capacity and auto scaling.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReadProvisionedThroughputSettings {


    /// 
    /// Specifies auto scaling settings for the replica table or global secondary       index.
    /// 
    /// Required: No
    ///
    /// Type: CapacityAutoScalingSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadCapacityAutoScalingSettings")]
    pub read_capacity_auto_scaling_settings: Option<CapacityAutoScalingSettings>,


    /// 
    /// Specifies a fixed read capacity for the replica table or global secondary       index.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<i64>,

}



impl cfn_resources::CfnResource for ReadProvisionedThroughputSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.read_capacity_auto_scaling_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents the properties of a global secondary index that can be set on a per-replica       basis.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicaGlobalSecondaryIndexSpecification {


    /// 
    /// Updates the status for contributor insights for a specific table or index. CloudWatch       Contributor Insights for DynamoDB graphs display the partition key and (if applicable)       sort key of frequently accessed items and frequently throttled items in plaintext. If       you require the use of AWS Key Management Service (KMS) to encrypt this       table’s partition key and sort key data with an AWS managed key or       customer managed key, you should not enable CloudWatch Contributor Insights for DynamoDB       for this table.
    /// 
    /// Required: No
    ///
    /// Type: ContributorInsightsSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,


    /// 
    /// The name of the global secondary index. The name must be unique among all other       indexes on this table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// Allows you to specify the read capacity settings for a replica global secondary index       when the BillingMode is set to PROVISIONED.
    /// 
    /// Required: No
    ///
    /// Type: ReadProvisionedThroughputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadProvisionedThroughputSettings")]
    pub read_provisioned_throughput_settings: Option<ReadProvisionedThroughputSettings>,

}



impl cfn_resources::CfnResource for ReplicaGlobalSecondaryIndexSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.contributor_insights_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.index_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'index_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.index_name;

        if the_val.len() < 3 as _ {
            return Err(format!("Min validation failed on field 'index_name'. {} is less than 3", the_val.len()));
        }

        
        self.read_provisioned_throughput_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Allows you to specify a KMS key identifier to be used for server-side encryption. The       key can be specified via ARN, key ID, or alias. The key must be created in the same       region as the replica.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicaSSESpecification {


    /// 
    /// The AWS KMS key that should be used for the AWS KMS encryption.       To specify a key, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN.       Note that you should only provide this parameter if the key is different from the       default DynamoDB key alias/aws/dynamodb.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSMasterKeyId")]
    pub kmsmaster_key_id: String,

}



impl cfn_resources::CfnResource for ReplicaSSESpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Defines settings specific to a single replica of a global table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicaSpecification {


    /// 
    /// The settings used to enable or disable CloudWatch Contributor Insights for the       specified replica. When not specified, defaults to contributor insights disabled for the       replica.
    /// 
    /// Required: No
    ///
    /// Type: ContributorInsightsSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,


    /// 
    /// Determines if a replica is protected from deletion. When enabled, the table cannot be deleted by any user or process.       This setting is disabled by default.       For more information, see Using deletion protection in       the Amazon DynamoDBDeveloper Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtectionEnabled")]
    pub deletion_protection_enabled: Option<bool>,


    /// 
    /// Defines additional settings for the global secondary indexes of this replica.
    /// 
    /// Required: No
    ///
    /// Type: List of ReplicaGlobalSecondaryIndexSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndexSpecification>>,


    /// 
    /// Defines the Kinesis Data Streams configuration for the specified replica.
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamSpecification")]
    pub kinesis_stream_specification: Option<KinesisStreamSpecification>,


    /// 
    /// The settings used to enable point in time recovery. When not specified, defaults to       point in time recovery disabled for the replica.
    /// 
    /// Required: No
    ///
    /// Type: PointInTimeRecoverySpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PointInTimeRecoverySpecification")]
    pub point_in_time_recovery_specification: Option<PointInTimeRecoverySpecification>,


    /// 
    /// Defines read capacity settings for the replica table.
    /// 
    /// Required: No
    ///
    /// Type: ReadProvisionedThroughputSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadProvisionedThroughputSettings")]
    pub read_provisioned_throughput_settings: Option<ReadProvisionedThroughputSettings>,


    /// 
    /// The region in which this replica exists.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,


    /// 
    /// Allows you to specify a customer-managed key for the replica. When using       customer-managed keys for server-side encryption, this property must have a value in all       replicas.
    /// 
    /// Required: No
    ///
    /// Type: ReplicaSSESpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<ReplicaSSESpecification>,


    /// 
    /// The table class of the specified table. Valid values are STANDARD and         STANDARD_INFREQUENT_ACCESS.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: STANDARD | STANDARD_INFREQUENT_ACCESS
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableClass")]
    pub table_class: Option<ReplicaSpecificationTableClassEnum>,


    /// 
    /// An array of key-value pairs to apply to this replica.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicaSpecificationTableClassEnum {

    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

    /// STANDARD_INFREQUENT_ACCESS
    #[serde(rename = "STANDARD_INFREQUENT_ACCESS")]
    Standardinfrequentaccess,

}

impl Default for ReplicaSpecificationTableClassEnum {
    fn default() -> Self {
        ReplicaSpecificationTableClassEnum::Standard
    }
}


impl cfn_resources::CfnResource for ReplicaSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.contributor_insights_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.kinesis_stream_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.point_in_time_recovery_specification.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.read_provisioned_throughput_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.ssespecification.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents the settings used to enable server-side encryption.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SSESpecification {


    /// 
    /// Indicates whether server-side encryption is performed using an AWS       managed key or an AWS owned key. If enabled (true), server-side       encryption type is set to KMS and an AWS managed key is used (AWS KMS charges apply). If disabled (false) or not specified,server-side       encryption is set to an AWS owned key. If you choose to use KMS       encryption, you can also use customer managed KMS keys by specifying them in the         ReplicaSpecification.SSESpecification object. You cannot mix AWS managed and customer managed KMS keys.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSEEnabled")]
    pub sseenabled: bool,


    /// 
    /// Server-side encryption type. The only supported value is:
    /// 
    /// KMS - Server-side encryption that uses AWS Key Management Service. The           key is stored in your account and is managed by AWS KMS (AWS KMS charges apply).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AES256 | KMS
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSEType")]
    pub ssetype: Option<SSESpecificationSSETypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SSESpecificationSSETypeEnum {

    /// AES256
    #[serde(rename = "AES256")]
    Aes256,

    /// KMS
    #[serde(rename = "KMS")]
    Kms,

}

impl Default for SSESpecificationSSETypeEnum {
    fn default() -> Self {
        SSESpecificationSSETypeEnum::Aes256
    }
}


impl cfn_resources::CfnResource for SSESpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Represents the DynamoDB Streams configuration for a table in DynamoDB.
///
/// You can only modify this value if your AWS::DynamoDB::GlobalTable       contains only one entry in Replicas. You must specify a value for this       property if your AWS::DynamoDB::GlobalTable contains more than one       replica.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StreamSpecification {


    /// 
    /// When an item in the table is modified, StreamViewType determines what       information is written to the stream for this table. Valid values for         StreamViewType are:
    /// 
    /// KEYS_ONLY - Only the key attributes of the modified item are           written to the stream.                        NEW_IMAGE - The entire item, as it appears after it was modified,           is written to the stream.                        OLD_IMAGE - The entire item, as it appeared before it was modified,           is written to the stream.                        NEW_AND_OLD_IMAGES - Both the new and the old item images of the           item are written to the stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: KEYS_ONLY | NEW_AND_OLD_IMAGES | NEW_IMAGE | OLD_IMAGE
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: StreamSpecificationStreamViewTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StreamSpecificationStreamViewTypeEnum {

    /// KEYS_ONLY
    #[serde(rename = "KEYS_ONLY")]
    Keysonly,

    /// NEW_AND_OLD_IMAGES
    #[serde(rename = "NEW_AND_OLD_IMAGES")]
    Newandoldimages,

    /// NEW_IMAGE
    #[serde(rename = "NEW_IMAGE")]
    Newimage,

    /// OLD_IMAGE
    #[serde(rename = "OLD_IMAGE")]
    Oldimage,

}

impl Default for StreamSpecificationStreamViewTypeEnum {
    fn default() -> Self {
        StreamSpecificationStreamViewTypeEnum::Keysonly
    }
}


impl cfn_resources::CfnResource for StreamSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Defines a target tracking scaling policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetTrackingScalingPolicyConfiguration {


    /// 
    /// Indicates whether scale in by the target tracking scaling policy is disabled. The       default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,


    /// 
    /// The amount of time, in seconds, after a scale-in activity completes before another       scale-in activity can start.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<i64>,


    /// 
    /// The amount of time, in seconds, after a scale-out activity completes before another       scale-out activity can start.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<i64>,


    /// 
    /// Defines a target value for the scaling policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}



impl cfn_resources::CfnResource for TargetTrackingScalingPolicyConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Represents the settings used to enable or disable Time to Live (TTL) for the specified       table. All replicas will have the same time to live configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimeToLiveSpecification {


    /// 
    /// The name of the attribute used to store the expiration time for items in the       table.
    /// 
    /// Currently, you cannot directly change the attribute name used to evaluate time to       live. In order to do so, you must first disable time to live, and then re-enable it with       the new attribute name. It can take up to one hour for changes to time to live to take       effect. If you attempt to modify time to live within that time window, your stack       operation might be delayed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: Option<String>,


    /// 
    /// Indicates whether TTL is to be enabled (true) or disabled (false) on the table.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}



impl cfn_resources::CfnResource for TimeToLiveSpecification {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.attribute_name {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'attribute_name'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.attribute_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'attribute_name'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Specifies an auto scaling policy for write capacity. This policy will be applied to       all replicas. This setting must be specified if BillingMode is set to         PROVISIONED.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WriteProvisionedThroughputSettings {


    /// 
    /// Specifies auto scaling settings for the replica table or global secondary       index.
    /// 
    /// Required: No
    ///
    /// Type: CapacityAutoScalingSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteCapacityAutoScalingSettings")]
    pub write_capacity_auto_scaling_settings: Option<CapacityAutoScalingSettings>,

}



impl cfn_resources::CfnResource for WriteProvisionedThroughputSettings {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.write_capacity_auto_scaling_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}