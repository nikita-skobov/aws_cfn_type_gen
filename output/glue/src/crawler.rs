/// The AWS::Glue::Crawler resource specifies an AWS Glue crawler. For more       information, see Cataloging Tables with a Crawler and Crawler Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCrawler {
    ///
    /// A list of UTF-8 strings that specify the names of custom classifiers that are associated    with the crawler.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,

    ///
    /// Crawler configuration information. This versioned JSON string allows users to specify       aspects of a crawler's behavior. For more information, see Configuring a       Crawler.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<cfn_resources::StrVal>,

    ///
    /// The name of the SecurityConfiguration structure to be used by this    crawler.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlerSecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_security_configuration: Option<cfn_resources::StrVal>,

    ///
    /// The name of the database in which the crawler's output is stored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// A description of the crawler.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the crawler.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.
    ///
    /// Required: No
    ///
    /// Type: RecrawlPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecrawlPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_policy: Option<RecrawlPolicy>,

    ///
    /// The Amazon Resource Name (ARN) of an IAM role that's used to access customer resources,    such as Amazon Simple Storage Service (Amazon S3) data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: cfn_resources::StrVal,

    ///
    /// For scheduled crawlers, the schedule when the crawler runs.
    ///
    /// Required: No
    ///
    /// Type: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,

    ///
    /// The policy that specifies update and delete behaviors for the crawler. The policy tells the crawler what to do in the event that it detects a change in a table that already exists in the customer's database at the time of the crawl. The SchemaChangePolicy does not affect whether or how new tables and partitions are added. New tables and partitions are always created regardless of the SchemaChangePolicy on a crawler.
    ///
    /// The SchemaChangePolicy consists of two components, UpdateBehavior and DeleteBehavior.
    ///
    /// Required: No
    ///
    /// Type: SchemaChangePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<SchemaChangePolicy>,

    ///
    /// The prefix added to the names of tables that are created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<cfn_resources::StrVal>,

    ///
    /// The tags to use with this crawler.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// A collection of targets to crawl.
    ///
    /// Required: Yes
    ///
    /// Type: Targets
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Targets,
}

impl cfn_resources::CfnResource for CfnCrawler {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Crawler"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.crawler_security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'crawler_security_configuration'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.crawler_security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!("Min validation failed on field 'crawler_security_configuration'. {} is less than 0", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

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

        self.recrawl_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.schedule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.schema_change_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.table_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'table_prefix'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.table_prefix {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'table_prefix'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.targets.validate()?;

        Ok(())
    }
}

/// Specifies an AWS Glue Data Catalog target.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CatalogTarget {
    ///
    /// The name of the database to be synchronized.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of the tables to be synchronized.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CatalogTarget {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.database_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'database_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.database_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'database_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The DeltaTarget property type specifies Property description not available. for an AWS::Glue::Crawler.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeltaTarget {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateNativeDeltaTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_native_delta_table: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_tables: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_manifest: Option<bool>,
}

impl cfn_resources::CfnResource for DeltaTarget {
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

/// Specifies an Amazon DynamoDB table to crawl.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DynamoDBTarget {
    ///
    /// The name of the DynamoDB table to crawl.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DynamoDBTarget {
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

/// Specifies a JDBC data store to crawl.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JdbcTarget {
    ///
    /// The name of the connection to use to connect to the JDBC target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of glob patterns used to exclude from the crawl. For more information, see         Catalog Tables         with a Crawler.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,

    ///
    /// The path of the JDBC target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JdbcTarget {
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

/// Specifies an Amazon DocumentDB or MongoDB data store to crawl.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MongoDBTarget {
    ///
    /// The name of the connection to use to connect to the Amazon DocumentDB or MongoDB target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<cfn_resources::StrVal>,

    ///
    /// The path of the Amazon DocumentDB or MongoDB target (database/collection).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MongoDBTarget {
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

/// When crawling an Amazon S3 data source after the first crawl is complete, specifies whether to crawl the entire dataset again or to crawl only folders that were added since the last crawler run. For more information, see Incremental Crawls in AWS Glue in the developer guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RecrawlPolicy {
    ///
    /// Specifies whether to crawl the entire dataset again or to crawl only folders that were added since the last crawler run.
    ///
    /// A value of CRAWL_EVERYTHING specifies crawling the entire dataset again.
    ///
    /// A value of CRAWL_NEW_FOLDERS_ONLY specifies crawling only folders that were added since the last crawler run.
    ///
    /// A value of CRAWL_EVENT_MODE specifies crawling only the changes identified by Amazon S3 events.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CRAWL_EVENT_MODE | CRAWL_EVERYTHING | CRAWL_NEW_FOLDERS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecrawlBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recrawl_behavior: Option<RecrawlPolicyRecrawlBehaviorEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RecrawlPolicyRecrawlBehaviorEnum {
    /// CRAWL_EVENT_MODE
    #[serde(rename = "CRAWL_EVENT_MODE")]
    Crawleventmode,

    /// CRAWL_EVERYTHING
    #[serde(rename = "CRAWL_EVERYTHING")]
    Crawleverything,

    /// CRAWL_NEW_FOLDERS_ONLY
    #[serde(rename = "CRAWL_NEW_FOLDERS_ONLY")]
    Crawlnewfoldersonly,
}

impl Default for RecrawlPolicyRecrawlBehaviorEnum {
    fn default() -> Self {
        RecrawlPolicyRecrawlBehaviorEnum::Crawleventmode
    }
}

impl cfn_resources::CfnResource for RecrawlPolicy {
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

/// Specifies a data store in Amazon Simple Storage Service (Amazon S3).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3Target {
    ///
    /// The name of a connection which allows a job or crawler to access data in Amazon S3 within an Amazon Virtual Private Cloud environment (Amazon VPC).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<cfn_resources::StrVal>,

    ///
    /// A valid Amazon dead-letter SQS ARN. For example, arn:aws:sqs:region:account:deadLetterQueue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DlqEventQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_event_queue_arn: Option<cfn_resources::StrVal>,

    ///
    /// A valid Amazon SQS ARN. For example, arn:aws:sqs:region:account:sqs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_queue_arn: Option<cfn_resources::StrVal>,

    ///
    /// A list of glob patterns used to exclude from the crawl. For more information, see         Catalog Tables         with a Crawler.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exclusions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,

    ///
    /// The path to the Amazon S3 target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// Sets the number of files in each leaf folder to be crawled when crawling sample files in a dataset. If not set, all the files are crawled. A valid value is an integer between 1 and 249.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i64>,
}

impl cfn_resources::CfnResource for S3Target {
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

/// A scheduling object using a cron statement to schedule an event.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    ///
    /// A cron expression used to specify the schedule. For more information, see         Time-Based Schedules for         Jobs and Crawlers. For example, to run something every day at 12:15 UTC,       specify cron(15 12 * * ? *).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Schedule {
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

/// The policy that specifies update and delete behaviors for the crawler. The policy tells the crawler what to do in the event that it detects a change in a table that already exists in the customer's database at the time of the crawl. The SchemaChangePolicy does not affect whether or how new tables and partitions are added. New tables and partitions are always created regardless of the SchemaChangePolicy on a crawler.
///
/// The SchemaChangePolicy consists of two components, UpdateBehavior and DeleteBehavior.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SchemaChangePolicy {
    ///
    /// The deletion behavior when the crawler finds a deleted object.
    ///
    /// A value of LOG specifies that if a table or partition is found to no longer exist, do not delete it, only log that it was found to no longer exist.
    ///
    /// A value of DELETE_FROM_DATABASE specifies that if a table or partition is found to have been removed, delete it from the database.
    ///
    /// A value of DEPRECATE_IN_DATABASE specifies that if a table has been found to no longer exist, to add a property to the table that says "DEPRECATED" and includes a timestamp with the time of deprecation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE_FROM_DATABASE | DEPRECATE_IN_DATABASE | LOG
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_behavior: Option<SchemaChangePolicyDeleteBehaviorEnum>,

    ///
    /// The update behavior when the crawler finds a changed schema.
    ///
    /// A value of LOG specifies that if a table or a partition already exists, and a change is detected, do not update it, only log that a change was detected. Add new tables and new partitions (including on existing tables).
    ///
    /// A value of UPDATE_IN_DATABASE specifies that if a table or partition already exists, and a change is detected, update it. Add new tables and partitions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LOG | UPDATE_IN_DATABASE
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpdateBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_behavior: Option<SchemaChangePolicyUpdateBehaviorEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SchemaChangePolicyDeleteBehaviorEnum {
    /// DELETE_FROM_DATABASE
    #[serde(rename = "DELETE_FROM_DATABASE")]
    Deletefromdatabase,

    /// DEPRECATE_IN_DATABASE
    #[serde(rename = "DEPRECATE_IN_DATABASE")]
    Deprecateindatabase,

    /// LOG
    #[serde(rename = "LOG")]
    Log,
}

impl Default for SchemaChangePolicyDeleteBehaviorEnum {
    fn default() -> Self {
        SchemaChangePolicyDeleteBehaviorEnum::Deletefromdatabase
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SchemaChangePolicyUpdateBehaviorEnum {
    /// LOG
    #[serde(rename = "LOG")]
    Log,

    /// UPDATE_IN_DATABASE
    #[serde(rename = "UPDATE_IN_DATABASE")]
    Updateindatabase,
}

impl Default for SchemaChangePolicyUpdateBehaviorEnum {
    fn default() -> Self {
        SchemaChangePolicyUpdateBehaviorEnum::Log
    }
}

impl cfn_resources::CfnResource for SchemaChangePolicy {
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

/// Specifies data stores to crawl.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Targets {
    ///
    /// Specifies AWS Glue Data Catalog targets.
    ///
    /// Required: No
    ///
    /// Type: List of CatalogTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_targets: Option<Vec<CatalogTarget>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of DeltaTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_targets: Option<Vec<DeltaTarget>>,

    ///
    /// Specifies Amazon DynamoDB targets.
    ///
    /// Required: No
    ///
    /// Type: List of DynamoDBTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_dbtargets: Option<Vec<DynamoDBTarget>>,

    ///
    /// Specifies JDBC targets.
    ///
    /// Required: No
    ///
    /// Type: List of JdbcTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "JdbcTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,

    ///
    /// A list of Mongo DB targets.
    ///
    /// Required: No
    ///
    /// Type: List of MongoDBTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "MongoDBTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_dbtargets: Option<Vec<MongoDBTarget>>,

    ///
    /// Specifies Amazon Simple Storage Service (Amazon S3) targets.
    ///
    /// Required: No
    ///
    /// Type: List of S3Target
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_targets: Option<Vec<S3Target>>,
}

impl cfn_resources::CfnResource for Targets {
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
