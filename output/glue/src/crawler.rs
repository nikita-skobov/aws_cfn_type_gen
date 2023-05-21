

/// The AWS::Glue::Crawler resource specifies an AWS Glue crawler. For more       information, see Cataloging Tables with a Crawler and Crawler Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub configuration: Option<String>,


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
    pub crawler_security_configuration: Option<String>,


    /// 
    /// The name of the database in which the crawler's output is stored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


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
    pub description: Option<String>,


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
    pub name: Option<String>,


    /// 
    /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.
    /// 
    /// Required: No
    ///
    /// Type: RecrawlPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecrawlPolicy")]
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
    pub role: String,


    /// 
    /// For scheduled crawlers, the schedule when the crawler runs.
    /// 
    /// Required: No
    ///
    /// Type: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
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
    pub table_prefix: Option<String>,


    /// 
    /// The tags to use with this crawler.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
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
    fn type_string() -> &'static str {
        "AWS::Glue::Crawler"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies an AWS Glue Data Catalog target.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub database_name: Option<String>,


    /// 
    /// A list of the tables to be synchronized.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tables")]
    pub tables: Option<Vec<String>>,

}




/// The DeltaTarget property type specifies Property description not available. for an AWS::Glue::Crawler.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeltaTarget {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateNativeDeltaTable")]
    pub create_native_delta_table: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTables")]
    pub delta_tables: Option<Vec<String>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteManifest")]
    pub write_manifest: Option<bool>,

}




/// Specifies an Amazon DynamoDB table to crawl.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub path: Option<String>,

}




/// Specifies a JDBC data store to crawl.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub connection_name: Option<String>,


    /// 
    /// A list of glob patterns used to exclude from the crawl. For more information, see         Catalog Tables         with a Crawler.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exclusions")]
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
    pub path: Option<String>,

}




/// Specifies an Amazon DocumentDB or MongoDB data store to crawl.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub connection_name: Option<String>,


    /// 
    /// The path of the Amazon DocumentDB or MongoDB target (database/collection).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,

}




/// When crawling an Amazon S3 data source after the first crawl is complete, specifies whether to crawl the entire dataset again or to crawl only folders that were added since the last crawler run. For more information, see Incremental Crawls in AWS Glue in the developer guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub recrawl_behavior: Option<RecrawlPolicyRecrawlBehaviorEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
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



/// Specifies a data store in Amazon Simple Storage Service (Amazon S3).
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub connection_name: Option<String>,


    /// 
    /// A valid Amazon dead-letter SQS ARN. For example, arn:aws:sqs:region:account:deadLetterQueue.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DlqEventQueueArn")]
    pub dlq_event_queue_arn: Option<String>,


    /// 
    /// A valid Amazon SQS ARN. For example, arn:aws:sqs:region:account:sqs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventQueueArn")]
    pub event_queue_arn: Option<String>,


    /// 
    /// A list of glob patterns used to exclude from the crawl. For more information, see         Catalog Tables         with a Crawler.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exclusions")]
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
    pub path: Option<String>,


    /// 
    /// Sets the number of files in each leaf folder to be crawled when crawling sample files in a dataset. If not set, all the files are crawled. A valid value is an integer between 1 and 249.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleSize")]
    pub sample_size: Option<i64>,

}




/// A scheduling object using a cron statement to schedule an event.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub schedule_expression: Option<String>,

}




/// The policy that specifies update and delete behaviors for the crawler. The policy tells the crawler what to do in the event that it detects a change in a table that already exists in the customer's database at the time of the crawl. The SchemaChangePolicy does not affect whether or how new tables and partitions are added. New tables and partitions are always created regardless of the SchemaChangePolicy on a crawler.
///
/// The SchemaChangePolicy consists of two components, UpdateBehavior and DeleteBehavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub update_behavior: Option<SchemaChangePolicyUpdateBehaviorEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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



/// Specifies data stores to crawl.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub catalog_targets: Option<Vec<CatalogTarget>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of DeltaTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeltaTargets")]
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
    pub s3_targets: Option<Vec<S3Target>>,

}


