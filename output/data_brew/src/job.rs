/// Specifies a new DataBrew job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnJob {
    ///
    /// One or more artifacts that represent the AWS Glue Data Catalog output       from running the job.
    ///
    /// Required: No
    ///
    /// Type: List of DataCatalogOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogOutputs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,

    ///
    /// Represents a list of JDBC database output objects which defines the output       destination for a DataBrew recipe job to write into.
    ///
    /// Required: No
    ///
    /// Type: List of DatabaseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOutputs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dataset_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption_key_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub encryption_mode: Option<JobEncryptionModeEnum>,

    ///
    /// A sample configuration for profile jobs only, which determines the number of rows on which the       profile job is run. If a JobSample value isn't provided, the default value       is used. The default value is CUSTOM_ROWS for the mode parameter and 20,000 for the       size parameter.
    ///
    /// Required: No
    ///
    /// Type: JobSample
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobSample")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub job_sample: Option<JobSample>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub log_subscription: Option<JobLogSubscriptionEnum>,

    ///
    /// The maximum number of nodes that can be consumed when the job processes data.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_capacity: Option<i64>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_retries: Option<i64>,

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
    pub name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: OutputLocation
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputLocation")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub output_location: Option<OutputLocation>,

    ///
    /// One or more artifacts that represent output from running the job.
    ///
    /// Required: No
    ///
    /// Type: List of Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub outputs: Option<Vec<Output>>,

    ///
    /// Configuration for profile jobs. Configuration can be used to select columns, do evaluations, and override default       parameters of evaluations. When configuration is undefined, the profile job will apply default settings to all       supported columns.
    ///
    /// Required: No
    ///
    /// Type: ProfileConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProfileConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub profile_configuration: Option<ProfileConfiguration>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub project_name: Option<cfn_resources::StrVal>,

    ///
    /// A series of data transformation steps that the job runs.
    ///
    /// Required: No
    ///
    /// Type: Recipe
    ///
    /// Update requires: No interruption
    #[serde(rename = "Recipe")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub recipe: Option<Recipe>,

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
    pub role_arn: cfn_resources::StrVal,

    ///
    /// Metadata tags that have been applied to the job.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub timeout: Option<i64>,

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
    /// List of validation configurations that are applied to the profile job.
    ///
    /// Required: No
    ///
    /// Type: List of ValidationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationConfigurations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for CfnJob {
    fn type_string(&self) -> &'static str {
        "AWS::DataBrew::Job"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.dataset_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'dataset_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.dataset_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'dataset_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.encryption_key_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'encryption_key_arn'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.encryption_key_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'encryption_key_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        self.job_sample
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.max_retries {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max_retries'. {} is less than 0",
                    the_val
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 240 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 240",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.output_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.profile_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.project_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'project_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.project_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'project_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.recipe.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.timeout {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'timeout'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Configuration of statistics that are allowed to be run on columns that       contain detected entities. When undefined, no statistics will be computed       on columns that contain detected entities.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for AllowedStatistics {
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

/// Selector of a column from a dataset for profile job configuration.       One selector includes either a column name or a regular expression.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub regex: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ColumnSelector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.regex {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'regex'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.regex {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'regex'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Configuration for column evaluations for a profile job. ColumnStatisticsConfiguration can be used to select       evaluations and override parameters of evaluations for particular columns.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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

impl cfn_resources::CfnResource for ColumnStatisticsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.statistics.validate()?;

        Ok(())
    }
}

/// Represents a set of options that define how DataBrew will write a       comma-separated value (CSV) file.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub delimiter: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CsvOutputOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.delimiter {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1 as _ {
                    return Err(format!(
                        "Max validation failed on field 'delimiter'. {} is greater than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.delimiter {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'delimiter'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Represents options that specify how and where in the AWS Glue Data Catalog DataBrew       writes the output generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataCatalogOutput {
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub catalog_id: Option<cfn_resources::StrVal>,

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
    pub database_name: cfn_resources::StrVal,

    ///
    /// Represents options that specify how and where DataBrew writes the database output       generated by recipe jobs.
    ///
    /// Required: No
    ///
    /// Type: DatabaseTableOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub overwrite: Option<bool>,

    ///
    /// Represents options that specify how and where DataBrew writes the Amazon S3       output generated by recipe jobs.
    ///
    /// Required: No
    ///
    /// Type: S3TableOutputOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Options")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DataCatalogOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.catalog_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'catalog_id'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.catalog_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'catalog_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.database_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'database_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.database_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'database_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.database_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'table_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'table_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents a JDBC database output object which defines the output destination for       a DataBrew recipe job to write into.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DatabaseOutput {
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub database_output_mode: Option<DatabaseOutputDatabaseOutputModeEnum>,

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
    pub glue_connection_name: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for DatabaseOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.database_options.validate()?;

        let the_val = &self.glue_connection_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'glue_connection_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.glue_connection_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'glue_connection_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents options that specify how and where DataBrew writes the database       output generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DatabaseTableOutputOptions {
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
    pub table_name: cfn_resources::StrVal,

    ///
    /// Represents an Amazon S3 location (bucket name and object key) where DataBrew can store       intermediate results.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub temp_directory: Option<S3Location>,
}

impl cfn_resources::CfnResource for DatabaseTableOutputOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'table_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'table_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.temp_directory
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration of entity detection for a profile job. When undefined, entity       detection is disabled.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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

impl cfn_resources::CfnResource for EntityDetectorConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.allowed_statistics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A sample configuration for profile jobs only, which determines the number of rows on which the       profile job is run. If a JobSample value isn't provided, the       default is used. The default value is CUSTOM_ROWS for the mode parameter and       20,000 for the size parameter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub size: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for JobSample {
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

/// Represents options that specify how and where in Amazon S3 DataBrew writes the output generated by       recipe jobs or profile jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Output {
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub compression_format: Option<OutputCompressionFormatEnum>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub format: Option<OutputFormatEnum>,

    ///
    /// Represents options that define how DataBrew formats job output files.
    ///
    /// Required: No
    ///
    /// Type: OutputFormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub format_options: Option<OutputFormatOptions>,

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
    /// The maximum number of files to be generated by the job and written to the output folder.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxOutputFiles")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_output_files: Option<i64>,

    ///
    /// A value that, if true, means that any data in the location specified for output is       overwritten with new output.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overwrite")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub overwrite: Option<bool>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub partition_columns: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for Output {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.format_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.location.validate()?;

        if let Some(the_val) = &self.partition_columns {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'partition_columns'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents a set of options that define the structure of comma-separated (CSV) job output.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub csv: Option<CsvOutputOptions>,
}

impl cfn_resources::CfnResource for OutputFormatOptions {
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

/// The location in Amazon S3 or AWS Glue Data Catalog where the job       writes its output.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub bucket: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketOwner")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_owner: Option<cfn_resources::StrVal>,

    ///
    /// The unique name of the object in the bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OutputLocation {
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

/// Configuration for profile jobs. Configuration can be used to select columns, do evaluations, and override default       parameters of evaluations. When configuration is undefined, the profile job will apply default settings to all       supported columns.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub column_statistics_configurations: Option<Vec<ColumnStatisticsConfiguration>>,

    ///
    /// Configuration for inter-column evaluations. Configuration can be used to select evaluations and override       parameters of evaluations. When configuration is undefined, the profile job will run all supported       inter-column evaluations.
    ///
    /// Required: No
    ///
    /// Type: StatisticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetStatisticsConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dataset_statistics_configuration: Option<StatisticsConfiguration>,

    ///
    /// Configuration of entity detection for a profile job. When undefined, entity detection is disabled.
    ///
    /// Required: No
    ///
    /// Type: EntityDetectorConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityDetectorConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub entity_detector_configuration: Option<EntityDetectorConfiguration>,

    ///
    /// List of column selectors. ProfileColumns can be used to select columns from the dataset. When       ProfileColumns is undefined, the profile job will profile all supported columns.
    ///
    /// Required: No
    ///
    /// Type: List of ColumnSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProfileColumns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub profile_columns: Option<Vec<ColumnSelector>>,
}

impl cfn_resources::CfnResource for ProfileConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dataset_statistics_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.entity_detector_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents one or more actions to be performed on a DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Recipe {
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
    pub name: cfn_resources::StrVal,

    ///
    /// The identifier for the version for the recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Recipe {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read       input data, or write output from a job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3Location {
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
    pub bucket: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_owner: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Location {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket'. {} is less than 3",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.bucket_owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 12 as _ {
                    return Err(format!(
                        "Max validation failed on field 'bucket_owner'. {} is greater than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.bucket_owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 12 as _ {
                    return Err(format!(
                        "Min validation failed on field 'bucket_owner'. {} is less than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 1280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Represents options that specify how and where DataBrew writes the Amazon S3 output       generated by recipe jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for S3TableOutputOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.location.validate()?;

        Ok(())
    }
}

/// Override of a particular evaluation for a profile job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StatisticOverride {
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
    pub statistic: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StatisticOverride {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.statistic;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'statistic'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.statistic;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'statistic'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Configuration of evaluations for a profile job. This configuration can be used to select       evaluations and override the parameters of selected evaluations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StatisticsConfiguration {
    ///
    /// List of included evaluations. When the list is undefined, all supported       evaluations will be included.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedStatistics")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub included_statistics: Option<Vec<String>>,

    ///
    /// List of overrides for evaluations.
    ///
    /// Required: No
    ///
    /// Type: List of StatisticOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub overrides: Option<Vec<StatisticOverride>>,
}

impl cfn_resources::CfnResource for StatisticsConfiguration {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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

/// Configuration for data quality validation. Used to select the Rulesets and Validation Mode       to be used in the profile job. When ValidationConfiguration is null, the profile       job will run without data quality validation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub ruleset_arn: cfn_resources::StrVal,

    ///
    /// Mode of data quality validation. Default mode is “CHECK_ALL” which verifies all rules       defined in the selected ruleset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub validation_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ValidationConfiguration {
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
