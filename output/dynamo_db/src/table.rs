/// The AWS::DynamoDB::Table resource creates a DynamoDB table.       For more information, see CreateTable in       the Amazon DynamoDB API Reference.
///
/// You should be aware of the following behaviors when working with DynamoDB       tables:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTable {
    ///
    /// A list of attributes that describe the key schema for the table and indexes.
    ///
    /// This property is required to create a DynamoDB table.
    ///
    /// Update requires: Some interruptions. Replacement if you edit an existing       AttributeDefinition.
    ///
    /// Required: Conditional
    ///
    /// Type: List of AttributeDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,

    ///
    /// Specify how you are charged for read and write throughput and how you manage       capacity.
    ///
    /// Valid values include:
    ///
    /// PROVISIONED - We recommend using PROVISIONED for           predictable workloads. PROVISIONED sets the billing mode to Provisioned Mode.                          PAY_PER_REQUEST - We recommend using PAY_PER_REQUEST           for unpredictable workloads. PAY_PER_REQUEST sets the billing mode           to On-Demand Mode.
    ///
    /// If not specified, the default is PROVISIONED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PAY_PER_REQUEST | PROVISIONED
    ///
    /// Update requires: No interruption
    #[serde(rename = "BillingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<TableBillingModeEnum>,

    ///
    /// The settings used to enable or disable CloudWatch Contributor Insights for the       specified table.
    ///
    /// Required: No
    ///
    /// Type: ContributorInsightsSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,

    ///
    /// Determines if a table is protected from deletion. When enabled, the table cannot be deleted by any user or process.       This setting is disabled by default.       For more information, see Using deletion protection in       the Amazon DynamoDBDeveloper Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,

    ///
    /// Global secondary indexes to be created on the table. You can create up to 20 global       secondary indexes.
    ///
    /// ImportantIf you update a table to include a new global secondary index, AWS CloudFormation initiates the index creation and then proceeds with the stack         update. AWS CloudFormation doesn't wait for the index to complete creation         because the backfilling phase can take a long time, depending on the size of the         table. You can't use the index or update the table until the index's status is           ACTIVE. You can track its status by using the DynamoDB DescribeTable command.If you add or delete an index during an update, we recommend that you don't update         any other resources. If your stack fails to update and is rolled back while adding a         new index, you must manually delete the index. Updates are not supported. The following are exceptions:                                   If you update either the contributor insights specification or the             provisioned throughput values of global secondary indexes, you can update             the table without interruption.                   You can delete or add one global secondary index without interruption. If             you do both in the same update (for example, by changing the index's logical             ID), the update fails.
    ///
    /// Required: No
    ///
    /// Type: List of GlobalSecondaryIndex
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,

    ///
    /// Specifies the properties of data being imported from the S3 bucket source to the       table.
    ///
    /// ImportantIf you specify the ImportSourceSpecification property, and also         specify either the StreamSpecification, the TableClass         property, or the DeletionProtectionEnabled property, the IAM entity creating/updating stack must have UpdateTable         permission.
    ///
    /// Required: No
    ///
    /// Type: ImportSourceSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImportSourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source_specification: Option<ImportSourceSpecification>,

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
    /// The Kinesis Data Streams configuration for the specified table.
    ///
    /// Required: No
    ///
    /// Type: KinesisStreamSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_specification: Option<KinesisStreamSpecification>,

    ///
    /// Local secondary indexes to be created on the table. You can create up to 5 local       secondary indexes. Each index is scoped to a given hash key value. The size of each hash       key can be up to 10 gigabytes.
    ///
    /// Required: No
    ///
    /// Type: List of LocalSecondaryIndex
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,

    ///
    /// The settings used to enable point in time recovery.
    ///
    /// Required: No
    ///
    /// Type: PointInTimeRecoverySpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PointInTimeRecoverySpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_specification: Option<PointInTimeRecoverySpecification>,

    ///
    /// Throughput for the specified table, which consists of values for         ReadCapacityUnits and WriteCapacityUnits. For more       information about the contents of a provisioned throughput structure, see Amazon DynamoDB Table ProvisionedThroughput.
    ///
    /// If you set BillingMode as PROVISIONED, you must specify this       property. If you set BillingMode as PAY_PER_REQUEST, you       cannot specify this property.
    ///
    /// Required: Conditional
    ///
    /// Type: ProvisionedThroughput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,

    ///
    /// Specifies the settings to enable server-side encryption.
    ///
    /// Required: No
    ///
    /// Type: SSESpecification
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SSESpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssespecification: Option<SSESpecification>,

    ///
    /// The settings for the DynamoDB table stream, which capture changes to items       stored in the table.
    ///
    /// Required: No
    ///
    /// Type: StreamSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,

    ///
    /// The table class of the new table. Valid values are STANDARD and         STANDARD_INFREQUENT_ACCESS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: STANDARD | STANDARD_INFREQUENT_ACCESS
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_class: Option<TableTableClassEnum>,

    ///
    /// A name for the table. If you don't specify a name, AWS CloudFormation generates       a unique physical ID and uses that ID for the table name. For more information, see         Name       Type.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Specifies the Time to Live (TTL) settings for the table.
    ///
    /// NoteFor detailed information about the limits in DynamoDB, see Limits in Amazon DynamoDB in the Amazon DynamoDB Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: TimeToLiveSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeToLiveSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TableBillingModeEnum {
    /// PAY_PER_REQUEST
    #[serde(rename = "PAY_PER_REQUEST")]
    Payperrequest,

    /// PROVISIONED
    #[serde(rename = "PROVISIONED")]
    Provisioned,
}

impl Default for TableBillingModeEnum {
    fn default() -> Self {
        TableBillingModeEnum::Payperrequest
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TableTableClassEnum {
    /// STANDARD
    #[serde(rename = "STANDARD")]
    Standard,

    /// STANDARD_INFREQUENT_ACCESS
    #[serde(rename = "STANDARD_INFREQUENT_ACCESS")]
    Standardinfrequentaccess,
}

impl Default for TableTableClassEnum {
    fn default() -> Self {
        TableTableClassEnum::Standard
    }
}

impl cfn_resources::CfnResource for CfnTable {
    fn type_string(&self) -> &'static str {
        "AWS::DynamoDB::Table"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.contributor_insights_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.import_source_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_stream_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.point_in_time_recovery_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.provisioned_throughput
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ssespecification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stream_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.table_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'table_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.table_name {
            if the_val.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'table_name'. {} is less than 3",
                    the_val.len()
                ));
            }
        }

        self.time_to_live_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents an attribute for describing the key schema for the table and       indexes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AttributeDefinition {
    ///
    /// A name for the attribute.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

    ///
    /// The data type for the attribute, where:
    ///
    /// S - the attribute is of type String                        N - the attribute is of type Number                        B - the attribute is of type Binary
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: B | N | S
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeType")]
    pub attribute_type: AttributeDefinitionAttributeTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AttributeDefinitionAttributeTypeEnum {
    /// B
    #[serde(rename = "B")]
    B,

    /// N
    #[serde(rename = "N")]
    N,

    /// S
    #[serde(rename = "S")]
    S,
}

impl Default for AttributeDefinitionAttributeTypeEnum {
    fn default() -> Self {
        AttributeDefinitionAttributeTypeEnum::B
    }
}

impl cfn_resources::CfnResource for AttributeDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.attribute_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'attribute_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.attribute_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'attribute_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The settings used to enable or disable CloudWatch Contributor Insights.
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

/// The options for imported source files in CSV format. The values are Delimiter and       HeaderList.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Csv {
    ///
    /// The delimiter used for separating items in the CSV file being imported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Pattern: [,;:|\t ]
    ///
    /// Update requires: Replacement
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,

    ///
    /// List of the headers used to specify a common header for all source CSV files being       imported. If this field is specified then the first line of each CSV file is treated as       data instead of the header. If this field is not specified the the first line of each       CSV file is treated as the header.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "HeaderList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_list: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for Csv {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.delimiter {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'delimiter'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.delimiter {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'delimiter'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.header_list {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'header_list'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents the properties of a global secondary index.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlobalSecondaryIndex {
    ///
    /// The settings used to enable or disable CloudWatch Contributor Insights for the       specified global secondary index.
    ///
    /// Required: No
    ///
    /// Type: ContributorInsightsSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContributorInsightsSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Update requires: Updates are not supported.
    #[serde(rename = "IndexName")]
    pub index_name: String,

    ///
    /// The complete key schema for a global secondary index, which consists of one or more       pairs of attribute names and key types:
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
    /// Represents the provisioned throughput settings for the specified global secondary       index.
    ///
    /// For current minimum and maximum provisioned throughput values, see Service,         Account, and Table Quotas in the Amazon DynamoDB Developer         Guide.
    ///
    /// Required: No
    ///
    /// Type: ProvisionedThroughput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

impl cfn_resources::CfnResource for GlobalSecondaryIndex {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.contributor_insights_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.index_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'index_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.index_name;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'index_name'. {} is less than 3",
                the_val.len()
            ));
        }

        let the_val = &self.key_schema;

        if the_val.len() > 2 as _ {
            return Err(format!(
                "Max validation failed on field 'key_schema'. {} is greater than 2",
                the_val.len()
            ));
        }

        self.projection.validate()?;

        self.provisioned_throughput
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the properties of data being imported from the S3 bucket source to the       table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImportSourceSpecification {
    ///
    /// Type of compression to be used on the input coming from the imported table.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GZIP | NONE | ZSTD
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputCompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_compression_type: Option<ImportSourceSpecificationInputCompressionTypeEnum>,

    ///
    /// The format of the source data. Valid values for ImportFormat are         CSV, DYNAMODB_JSON or ION.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | DYNAMODB_JSON | ION
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputFormat")]
    pub input_format: ImportSourceSpecificationInputFormatEnum,

    ///
    /// Additional properties that specify how the input is formatted,
    ///
    /// Required: No
    ///
    /// Type: InputFormatOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputFormatOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_options: Option<InputFormatOptions>,

    ///
    /// The S3 bucket that provides the source for the import.
    ///
    /// Required: Yes
    ///
    /// Type: S3BucketSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3BucketSource")]
    pub s3_bucket_source: S3BucketSource,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ImportSourceSpecificationInputCompressionTypeEnum {
    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// ZSTD
    #[serde(rename = "ZSTD")]
    Zstd,
}

impl Default for ImportSourceSpecificationInputCompressionTypeEnum {
    fn default() -> Self {
        ImportSourceSpecificationInputCompressionTypeEnum::Gzip
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ImportSourceSpecificationInputFormatEnum {
    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// DYNAMODB_JSON
    #[serde(rename = "DYNAMODB_JSON")]
    Dynamodbjson,

    /// ION
    #[serde(rename = "ION")]
    Ion,
}

impl Default for ImportSourceSpecificationInputFormatEnum {
    fn default() -> Self {
        ImportSourceSpecificationInputFormatEnum::Csv
    }
}

impl cfn_resources::CfnResource for ImportSourceSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input_format_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_bucket_source.validate()?;

        Ok(())
    }
}

/// The format options for the data that was imported into the target table. There is one       value, CsvOption.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputFormatOptions {
    ///
    /// The options for imported source files in CSV format. The values are Delimiter and       HeaderList.
    ///
    /// Required: No
    ///
    /// Type: Csv
    ///
    /// Update requires: Replacement
    #[serde(rename = "Csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<Csv>,
}

impl cfn_resources::CfnResource for InputFormatOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csv.as_ref().map_or(Ok(()), |val| val.validate())?;

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
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

    ///
    /// The role that this key attribute will assume:
    ///
    /// HASH - partition key                        RANGE - sort key
    ///
    /// NoteThe partition key of an item is also known as its hash           attribute. The term "hash attribute" derives from DynamoDB's usage of an internal hash function to evenly distribute data items across         partitions, based on their partition key values.The sort key of an item is also known as its range attribute.         The term "range attribute" derives from the way DynamoDB stores items with         the same partition key physically close together, in sorted order by the sort key         value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: HASH | RANGE
    ///
    /// Update requires: No interruption
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.attribute_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'attribute_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.attribute_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'attribute_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The Kinesis Data Streams configuration for the specified table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamSpecification {
    ///
    /// The ARN for a specific Kinesis data stream.
    ///
    /// Length Constraints: Minimum length of 37. Maximum length of 1024.
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.stream_arn;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'stream_arn'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.stream_arn;

        if the_val.len() < 37 as _ {
            return Err(format!(
                "Min validation failed on field 'stream_arn'. {} is less than 37",
                the_val.len()
            ));
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.index_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'index_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.index_name;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'index_name'. {} is less than 3",
                the_val.len()
            ));
        }

        let the_val = &self.key_schema;

        if the_val.len() > 2 as _ {
            return Err(format!(
                "Max validation failed on field 'key_schema'. {} is greater than 2",
                the_val.len()
            ));
        }

        self.projection.validate()?;

        Ok(())
    }
}

/// The settings used to enable point in time recovery.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for PointInTimeRecoverySpecification {
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
    /// Update requires: No interruption
    #[serde(rename = "NonKeyAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Update requires: No interruption
    #[serde(rename = "ProjectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.non_key_attributes {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'non_key_attributes'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Throughput for the specified table, which consists of values for         ReadCapacityUnits and WriteCapacityUnits. For more       information about the contents of a provisioned throughput structure, see Amazon DynamoDB Table ProvisionedThroughput.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProvisionedThroughput {
    ///
    /// The maximum number of strongly consistent reads consumed per second before DynamoDB       returns a ThrottlingException. For more information, see Specifying Read and Write Requirements in the Amazon DynamoDB         Developer Guide.
    ///
    /// If read/write capacity mode is PAY_PER_REQUEST the value is set to       0.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,

    ///
    /// The maximum number of writes consumed per second before DynamoDB returns a         ThrottlingException. For more information, see Specifying Read and Write Requirements in the Amazon DynamoDB         Developer Guide.
    ///
    /// If read/write capacity mode is PAY_PER_REQUEST the value is set to       0.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,
}

impl cfn_resources::CfnResource for ProvisionedThroughput {
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

/// The S3 bucket that is being imported from.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3BucketSource {
    ///
    /// The S3 bucket that is being imported from.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-z0-9A-Z]+[\.\-\w]*[a-z0-9A-Z]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,

    ///
    /// The account number of the S3 bucket that is being imported from. If the bucket is       owned by the requester this is optional.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [0-9]{12}
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3BucketOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,

    ///
    /// The key prefix shared by all S3 Objects that are being imported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

impl cfn_resources::CfnResource for S3BucketSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.s3_bucket;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 's3_bucket'. {} is greater than 255",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.s3_key_prefix {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_key_prefix'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents the settings used to enable server-side encryption.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SSESpecification {
    ///
    /// The AWS KMS key that should be used for the AWS KMS encryption.       To specify a key, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN.       Note that you should only provide this parameter if the key is different from the       default DynamoDB key alias/aws/dynamodb.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmsmaster_key_id: Option<String>,

    ///
    /// Indicates whether server-side encryption is done using an AWS managed       key or an AWS owned key. If enabled (true), server-side encryption type       is set to KMS and an AWS managed key is used (AWS KMS charges apply). If disabled (false) or not specified, server-side       encryption is set to AWS owned key.
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
    /// Update requires: No interruption
    #[serde(rename = "SSEType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssetype: Option<String>,
}

impl cfn_resources::CfnResource for SSESpecification {
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

/// Represents the DynamoDB Streams configuration for a table in DynamoDB.
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

/// Represents the settings used to enable or disable Time to Live (TTL) for the specified       table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimeToLiveSpecification {
    ///
    /// The name of the TTL attribute used to store the expiration time for items in the       table.
    ///
    /// NoteTo update this property, you must first disable TTL then enable TTL with the new         attribute name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.attribute_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'attribute_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.attribute_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'attribute_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
