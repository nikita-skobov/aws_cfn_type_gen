

/// Specifies a new DataBrew job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnJob {


    /// 
    /// The Amazon Resource Name (ARN) of an encryption key that is used to protect the job       output. For more information, see Encrypting data         written by DataBrew jobs
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionKeyArn")]
    pub encryption_key_arn: Option<String>,


    /// 
    /// One or more artifacts that represent output from running the job.
    /// 
    /// Required: No
    ///
    /// Type: List of Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Outputs")]
    pub outputs: Option<Vec<Output>>,


    /// 
    /// Metadata tags that have been applied to the job.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The job type of the job, which must be one of the following:
    /// 
    /// PROFILE - A job to analyze a dataset, to determine its size, data           types, data distribution, and more.                        RECIPE - A job to apply one or more transformations to a           dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: PROFILE | RECIPE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: JobTypeEnum,


    /// 
    /// The current status of Amazon CloudWatch logging for the job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLE | ENABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogSubscription")]
    pub log_subscription: Option<JobLogSubscriptionEnum>,


    /// 
    /// One or more artifacts that represent the AWS Glue Data Catalog output       from running the job.
    /// 
    /// Required: No
    ///
    /// Type: List of DataCatalogOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogOutputs")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,


    /// 
    /// A dataset that the job is to process.
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
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,


    /// 
    /// A series of data transformation steps that the job runs.
    /// 
    /// Required: No
    ///
    /// Type: Recipe
    ///
    /// Update requires: No interruption
    #[serde(rename = "Recipe")]
    pub recipe: Option<Recipe>,


    /// 
    /// The name of the project that the job is associated with.
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
    #[serde(rename = "ProjectName")]
    pub project_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: OutputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,


    /// 
    /// The encryption mode for the job, which can be one of the following:
    /// 
    /// SSE-KMS - Server-side encryption with keys managed by AWS KMS.                        SSE-S3 - Server-side encryption with keys managed by Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SSE-KMS | SSE-S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<JobEncryptionModeEnum>,


    /// 
    /// The maximum number of nodes that can be consumed when the job processes data.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<i64>,


    /// 
    /// Configuration for profile jobs. Configuration can be used to select columns, do evaluations, and override default       parameters of evaluations. When configuration is undefined, the profile job will apply default settings to all       supported columns.
    /// 
    /// Required: No
    ///
    /// Type: ProfileConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProfileConfiguration")]
    pub profile_configuration: Option<ProfileConfiguration>,


    /// 
    /// The job's timeout in minutes. A job that attempts to run longer than this timeout       period ends with a status of TIMEOUT.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


    /// 
    /// List of validation configurations that are applied to the profile job.
    /// 
    /// Required: No
    ///
    /// Type: List of ValidationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationConfigurations")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,


    /// 
    /// The maximum number of times to retry the job after a job run fails.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<i64>,


    /// 
    /// Represents a list of JDBC database output objects which defines the output       destination for a DataBrew recipe job to write into.
    /// 
    /// Required: No
    ///
    /// Type: List of DatabaseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOutputs")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,


    /// 
    /// The Amazon Resource Name (ARN) of the role to be assumed for this job.
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


    /// 
    /// A sample configuration for profile jobs only, which determines the number of rows on which the       profile job is run. If a JobSample value isn't provided, the default value       is used. The default value is CUSTOM_ROWS for the mode parameter and 20,000 for the       size parameter.
    /// 
    /// Required: No
    ///
    /// Type: JobSample
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobSample")]
    pub job_sample: Option<JobSample>,


    /// 
    /// The unique name of the job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 240
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum JobTypeEnum {

    /// PROFILE
    #[serde(rename = "PROFILE")]
    Profile,

    /// RECIPE
    #[serde(rename = "RECIPE")]
    Recipe,

}

impl Default for JobTypeEnum {
    fn default() -> Self {
        JobTypeEnum::Profile
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JobEncryptionModeEnum {

    /// SSE-KMS
    #[serde(rename = "SSE-KMS")]
    Ssekms,

    /// SSE-S3
    #[serde(rename = "SSE-S3")]
    Sses3,

}

impl Default for JobEncryptionModeEnum {
    fn default() -> Self {
        JobEncryptionModeEnum::Ssekms
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JobLogSubscriptionEnum {

    /// DISABLE
    #[serde(rename = "DISABLE")]
    Disable,

    /// ENABLE
    #[serde(rename = "ENABLE")]
    Enable,

}

impl Default for JobLogSubscriptionEnum {
    fn default() -> Self {
        JobLogSubscriptionEnum::Disable
    }
}


impl cfn_resources::CfnResource for CfnJob {
    fn type_string() -> &'static str {
        "AWS::DataBrew::Job"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Represents one or more actions to be performed on a DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Recipe {


    /// 
    /// The identifier for the version for the recipe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The unique name for the recipe.
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
    #[serde(rename = "Name")]
    pub name: String,

}




/// A sample configuration for profile jobs only, which determines the number of rows on which the       profile job is run. If a JobSample value isn't provided, the       default is used. The default value is CUSTOM_ROWS for the mode parameter and       20,000 for the size parameter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JobSample {


    /// 
    /// A value that determines whether the profile job is run on the entire dataset or a       specified number of rows. This value must be one of the following:
    /// 
    /// FULL_DATASET - The profile job is run on the entire dataset.               CUSTOM_ROWS - The profile job is run on the number of rows specified in the           Size parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM_ROWS | FULL_DATASET
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: Option<JobSampleModeEnum>,


    /// 
    /// The Size parameter is only required when the mode is CUSTOM_ROWS. The       profile job is run on the specified number of rows. The maximum value for size is       Long.MAX_VALUE.
    /// 
    /// Long.MAX_VALUE = 9223372036854775807
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum JobSampleModeEnum {

    /// CUSTOM_ROWS
    #[serde(rename = "CUSTOM_ROWS")]
    Customrows,

    /// FULL_DATASET
    #[serde(rename = "FULL_DATASET")]
    Fulldataset,

}

impl Default for JobSampleModeEnum {
    fn default() -> Self {
        JobSampleModeEnum::Customrows
    }
}



/// Configuration of statistics that are allowed to be run on columns that       contain detected entities. When undefined, no statistics will be computed       on columns that contain detected entities.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AllowedStatistics {


    /// 
    /// One or more column statistics to allow for columns that contain detected entities.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistics")]
    pub statistics: Vec<String>,

}




/// Represents options that specify how and where in the AWS Glue Data Catalog DataBrew       writes the output generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataCatalogOutput {


    /// 
    /// Represents options that specify how and where DataBrew writes the database output       generated by recipe jobs.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseTableOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOptions")]
    pub database_options: Option<DatabaseTableOutputOptions>,


    /// 
    /// A value that, if true, means that any data in the location specified for output       is overwritten with new output. Not supported with DatabaseOptions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overwrite")]
    pub overwrite: Option<bool>,


    /// 
    /// The unique identifier of the AWS account that holds the Data Catalog that       stores the data.
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// Represents options that specify how and where DataBrew writes the Amazon S3       output generated by recipe jobs.
    /// 
    /// Required: No
    ///
    /// Type: S3TableOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Options")]
    pub s3_options: Option<S3TableOutputOptions>,


    /// 
    /// The name of a table in the Data Catalog.
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
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The name of a database in the Data Catalog.
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
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

}




/// Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read       input data, or write output from a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The AWS account ID of the bucket owner.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,


    /// 
    /// The unique name of the object in the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1280
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The Amazon S3 bucket name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

}




/// Override of a particular evaluation for a profile job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatisticOverride {


    /// 
    /// The name of an evaluation
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[A-Z\_]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: String,


    /// 
    /// A map that includes overrides of an evaluation’s parameters.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: std::collections::HashMap<String, String>,

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




/// Configuration for profile jobs. Configuration can be used to select columns, do evaluations, and override default       parameters of evaluations. When configuration is undefined, the profile job will apply default settings to all       supported columns.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProfileConfiguration {


    /// 
    /// List of configurations for column evaluations. ColumnStatisticsConfigurations are used to       select evaluations and override parameters of evaluations for particular columns. When       ColumnStatisticsConfigurations is undefined, the profile job will profile all supported columns       and run all supported evaluations.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnStatisticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnStatisticsConfigurations")]
    pub column_statistics_configurations: Option<Vec<ColumnStatisticsConfiguration>>,


    /// 
    /// List of column selectors. ProfileColumns can be used to select columns from the dataset. When       ProfileColumns is undefined, the profile job will profile all supported columns.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProfileColumns")]
    pub profile_columns: Option<Vec<ColumnSelector>>,


    /// 
    /// Configuration of entity detection for a profile job. When undefined, entity detection is disabled.
    /// 
    /// Required: No
    ///
    /// Type: EntityDetectorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityDetectorConfiguration")]
    pub entity_detector_configuration: Option<EntityDetectorConfiguration>,


    /// 
    /// Configuration for inter-column evaluations. Configuration can be used to select evaluations and override       parameters of evaluations. When configuration is undefined, the profile job will run all supported       inter-column evaluations.
    /// 
    /// Required: No
    ///
    /// Type: StatisticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetStatisticsConfiguration")]
    pub dataset_statistics_configuration: Option<StatisticsConfiguration>,

}




/// Represents options that specify how and where DataBrew writes the database       output generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseTableOutputOptions {


    /// 
    /// Represents an Amazon S3 location (bucket name and object key) where DataBrew can store       intermediate results.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,


    /// 
    /// A prefix for the name of a table DataBrew will create in the database.
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
    #[serde(rename = "TableName")]
    pub table_name: String,

}




/// Represents options that specify how and where in Amazon S3 DataBrew writes the output generated by       recipe jobs or profile jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Output {


    /// 
    /// The location in Amazon S3 where the job writes its output.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: S3Location,


    /// 
    /// The data format of the output of the job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVRO | CSV | GLUEPARQUET | JSON | ORC | PARQUET | TABLEAUHYPER | XML
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<OutputFormatEnum>,


    /// 
    /// A value that, if true, means that any data in the location specified for output is       overwritten with new output.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overwrite")]
    pub overwrite: Option<bool>,


    /// 
    /// The compression algorithm used to compress the output text of the job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BROTLI | BZIP2 | DEFLATE | GZIP | LZ4 | LZO | SNAPPY | ZLIB | ZSTD
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<OutputCompressionFormatEnum>,


    /// 
    /// The names of one or more partition columns for the output of the job.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionColumns")]
    pub partition_columns: Option<Vec<String>>,


    /// 
    /// Represents options that define how DataBrew formats job output files.
    /// 
    /// Required: No
    ///
    /// Type: OutputFormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatOptions")]
    pub format_options: Option<OutputFormatOptions>,


    /// 
    /// The maximum number of files to be generated by the job and written to the output folder.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxOutputFiles")]
    pub max_output_files: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OutputCompressionFormatEnum {

    /// BROTLI
    #[serde(rename = "BROTLI")]
    Brotli,

    /// BZIP2
    #[serde(rename = "BZIP2")]
    Bzip2,

    /// DEFLATE
    #[serde(rename = "DEFLATE")]
    Deflate,

    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// LZ4
    #[serde(rename = "LZ4")]
    Lz4,

    /// LZO
    #[serde(rename = "LZO")]
    Lzo,

    /// SNAPPY
    #[serde(rename = "SNAPPY")]
    Snappy,

    /// ZLIB
    #[serde(rename = "ZLIB")]
    Zlib,

    /// ZSTD
    #[serde(rename = "ZSTD")]
    Zstd,

}

impl Default for OutputCompressionFormatEnum {
    fn default() -> Self {
        OutputCompressionFormatEnum::Brotli
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OutputFormatEnum {

    /// AVRO
    #[serde(rename = "AVRO")]
    Avro,

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// GLUEPARQUET
    #[serde(rename = "GLUEPARQUET")]
    Glueparquet,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// ORC
    #[serde(rename = "ORC")]
    Orc,

    /// PARQUET
    #[serde(rename = "PARQUET")]
    Parquet,

    /// TABLEAUHYPER
    #[serde(rename = "TABLEAUHYPER")]
    Tableauhyper,

    /// XML
    #[serde(rename = "XML")]
    Xml,

}

impl Default for OutputFormatEnum {
    fn default() -> Self {
        OutputFormatEnum::Avro
    }
}



/// Represents a set of options that define how DataBrew will write a       comma-separated value (CSV) file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsvOutputOptions {


    /// 
    /// A single character that specifies the delimiter used to create CSV job output.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}




/// Represents a set of options that define the structure of comma-separated (CSV) job output.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputFormatOptions {


    /// 
    /// Represents a set of options that define the structure of comma-separated value (CSV)       job output.
    /// 
    /// Required: No
    ///
    /// Type: CsvOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Csv")]
    pub csv: Option<CsvOutputOptions>,

}




/// Selector of a column from a dataset for profile job configuration.       One selector includes either a column name or a regular expression.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnSelector {


    /// 
    /// The name of a column from a dataset.
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A regular expression for selecting a column from a dataset.
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
    #[serde(rename = "Regex")]
    pub regex: Option<String>,

}




/// Configuration for column evaluations for a profile job. ColumnStatisticsConfiguration can be used to select       evaluations and override parameters of evaluations for particular columns.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ColumnStatisticsConfiguration {


    /// 
    /// List of column selectors. Selectors can be used to select columns from the dataset.       When selectors are undefined, configuration will be applied to all supported columns.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "Selectors")]
    pub selectors: Option<Vec<ColumnSelector>>,


    /// 
    /// Configuration for evaluations. Statistics can be used to select evaluations and override       parameters of evaluations.
    /// 
    /// Required: Yes
    ///
    /// Type: StatisticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistics")]
    pub statistics: StatisticsConfiguration,

}




/// Configuration for data quality validation. Used to select the Rulesets and Validation Mode       to be used in the profile job. When ValidationConfiguration is null, the profile       job will run without data quality validation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValidationConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) for the ruleset to be validated in the profile job.       The TargetArn of the selected ruleset should be the same as the Amazon Resource Name (ARN) of       the dataset that is associated with the profile job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RulesetArn")]
    pub ruleset_arn: String,


    /// 
    /// Mode of data quality validation. Default mode is “CHECK_ALL” which verifies all rules       defined in the selected ruleset.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationMode")]
    pub validation_mode: Option<String>,

}




/// Configuration of evaluations for a profile job. This configuration can be used to select       evaluations and override the parameters of selected evaluations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatisticsConfiguration {


    /// 
    /// List of overrides for evaluations.
    /// 
    /// Required: No
    ///
    /// Type: List of StatisticOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<StatisticOverride>>,


    /// 
    /// List of included evaluations. When the list is undefined, all supported       evaluations will be included.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedStatistics")]
    pub included_statistics: Option<Vec<String>>,

}




/// Represents a JDBC database output object which defines the output destination for       a DataBrew recipe job to write into.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseOutput {


    /// 
    /// The output mode to write into the database. Currently supported option: NEW_TABLE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NEW_TABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOutputMode")]
    pub database_output_mode: Option<DatabaseOutputDatabaseOutputModeEnum>,


    /// 
    /// Represents options that specify how and where DataBrew writes the database output       generated by recipe jobs.
    /// 
    /// Required: Yes
    ///
    /// Type: DatabaseTableOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOptions")]
    pub database_options: DatabaseTableOutputOptions,


    /// 
    /// The AWS Glue connection that stores the connection information for the       target database.
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
    #[serde(rename = "GlueConnectionName")]
    pub glue_connection_name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DatabaseOutputDatabaseOutputModeEnum {

    /// NEW_TABLE
    #[serde(rename = "NEW_TABLE")]
    Newtable,

}

impl Default for DatabaseOutputDatabaseOutputModeEnum {
    fn default() -> Self {
        DatabaseOutputDatabaseOutputModeEnum::Newtable
    }
}



/// Configuration of entity detection for a profile job. When undefined, entity       detection is disabled.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EntityDetectorConfiguration {


    /// 
    /// Configuration of statistics that are allowed to be run on columns that       contain detected entities. When undefined, no statistics will be computed       on columns that contain detected entities.
    /// 
    /// Required: No
    ///
    /// Type: AllowedStatistics
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedStatistics")]
    pub allowed_statistics: Option<AllowedStatistics>,


    /// 
    /// Entity types to detect. Can be any of the following:
    /// 
    /// USA_SSN               EMAIL               USA_ITIN               USA_PASSPORT_NUMBER               PHONE_NUMBER               USA_DRIVING_LICENSE               BANK_ACCOUNT               CREDIT_CARD               IP_ADDRESS               MAC_ADDRESS               USA_DEA_NUMBER               USA_HCPCS_CODE               USA_NATIONAL_PROVIDER_IDENTIFIER               USA_NATIONAL_DRUG_CODE               USA_HEALTH_INSURANCE_CLAIM_NUMBER               USA_MEDICARE_BENEFICIARY_IDENTIFIER               USA_CPT_CODE               PERSON_NAME               DATE
    /// 
    /// The Entity type group USA_ALL is also supported, and includes all of the       above entity types except PERSON_NAME and DATE.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<String>,

}




/// The location in Amazon S3 or AWS Glue Data Catalog where the job       writes its output.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputLocation {


    /// 
    /// The Amazon S3 bucket name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// 
    /// The unique name of the object in the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,

}




/// Represents options that specify how and where DataBrew writes the Amazon S3 output       generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3TableOutputOptions {


    /// 
    /// Represents an Amazon S3 location (bucket name and object key) where DataBrew can write output       from a job.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: S3Location,

}


