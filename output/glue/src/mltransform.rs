

/// The AWS::Glue::MLTransform is an AWS Glue resource type that manages machine learning transforms.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMLTransform {


    /// 
    /// A user-defined, long-form description text for the machine learning transform.
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
    /// This value determines which version of AWS Glue this machine learning transform is compatible with. Glue 1.0 is recommended for most customers. If the value is not set, the Glue compatibility defaults to Glue 0.9. For more information, see AWS Glue Versions in the developer guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,


    /// 
    /// A list of AWS Glue table definitions used by the transform.
    /// 
    /// Required: Yes
    ///
    /// Type: InputRecordTables
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputRecordTables")]
    pub input_record_tables: InputRecordTables,


    /// 
    /// The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of    processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more    information, see the AWS Glue pricing     page.
    /// 
    /// MaxCapacity is a mutually exclusive option with NumberOfWorkers and WorkerType.
    /// 
    /// If either NumberOfWorkers or WorkerType is set, then MaxCapacity cannot be set.If MaxCapacity is set then neither NumberOfWorkers or WorkerType can be set.If WorkerType is set, then NumberOfWorkers is required (and vice versa).MaxCapacity and NumberOfWorkers must both be at least 1.
    /// 
    /// When the WorkerType field is set to a value other than Standard, the MaxCapacity field is set automatically and becomes read-only.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,


    /// 
    /// The maximum number of times to retry after an MLTaskRun of the machine    learning transform fails.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<i64>,


    /// 
    /// A user-defined name for the machine learning transform. Names are required to be unique. Name is optional:
    /// 
    /// If you supply Name, the stack cannot be repeatedly created.If Name is not provided, a randomly generated name will be used instead.
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The number of workers of a defined workerType that are allocated when a task of the transform runs.
    /// 
    /// If WorkerType is set, then NumberOfWorkers is required (and vice versa).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfWorkers")]
    pub number_of_workers: Option<i64>,


    /// 
    /// The name or Amazon Resource Name (ARN) of the IAM role with the required permissions. The required permissions include both AWS Glue service role permissions to AWS Glue resources, and Amazon S3 permissions required by the transform.
    /// 
    /// This role needs AWS Glue service role permissions to allow access to resources in AWS Glue. See Attach a Policy to IAM Users That Access AWS Glue.This role needs permission to your Amazon Simple Storage Service (Amazon S3) sources, targets, temporary directory, scripts, and any libraries used by the task run for this transform.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: String,


    /// 
    /// The tags to use with this machine learning transform. You may use tags to limit access to the machine learning transform. For more information about tags in AWS Glue, see AWS Tags in AWS Glue in the developer guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// The timeout in minutes of the machine learning transform.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


    /// 
    /// The encryption-at-rest settings of the transform that apply to accessing user data. Machine learning transforms can access user data encrypted in Amazon S3 using KMS.
    /// 
    /// Additionally, imported labels and trained transforms can now be encrypted using a customer provided KMS key.
    /// 
    /// Required: No
    ///
    /// Type: TransformEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransformEncryption")]
    pub transform_encryption: Option<TransformEncryption>,


    /// 
    /// The algorithm-specific parameters that are associated with the machine learning transform.
    /// 
    /// Required: Yes
    ///
    /// Type: TransformParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransformParameters")]
    pub transform_parameters: TransformParameters,


    /// 
    /// The type of predefined worker that is allocated when a task of this transform runs. Accepts a value of Standard, G.1X, or G.2X.
    /// 
    /// For the Standard worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.               For the G.1X worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.               For the G.2X worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.
    /// 
    /// MaxCapacity is a mutually exclusive option with NumberOfWorkers and WorkerType.
    /// 
    /// If either NumberOfWorkers or WorkerType is set, then MaxCapacity cannot be set.If MaxCapacity is set then neither NumberOfWorkers or WorkerType can be set.If WorkerType is set, then NumberOfWorkers is required (and vice versa).MaxCapacity and NumberOfWorkers must both be at least 1.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: G.025X | G.1X | G.2X | Standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<MLTransformWorkerTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum MLTransformWorkerTypeEnum {

    /// G.025X
    #[serde(rename = "G.025X")]
    G025x,

    /// G.1X
    #[serde(rename = "G.1X")]
    G1x,

    /// G.2X
    #[serde(rename = "G.2X")]
    G2x,

    /// Standard
    #[serde(rename = "Standard")]
    Standard,

}

impl Default for MLTransformWorkerTypeEnum {
    fn default() -> Self {
        MLTransformWorkerTypeEnum::G025x
    }
}


impl cfn_resources::CfnResource for CfnMLTransform {
    fn type_string() -> &'static str {
        "AWS::Glue::MLTransform"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The parameters to configure the find matches transform.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FindMatchesParameters {


    /// 
    /// The value that is selected when tuning your transform for a balance between accuracy and    cost. A value of 0.5 means that the system balances accuracy and cost concerns. A value of 1.0    means a bias purely for accuracy, which typically results in a higher cost, sometimes    substantially higher. A value of 0.0 means a bias purely for cost, which results in a less    accurate FindMatches transform, sometimes with unacceptable accuracy.
    /// 
    /// Accuracy measures how well the transform finds true positives and true negatives. Increasing accuracy requires more machine resources and cost. But it also results in increased recall.
    /// 
    /// Cost measures how many compute resources, and thus money, are consumed to run the    transform.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccuracyCostTradeoff")]
    pub accuracy_cost_tradeoff: Option<f64>,


    /// 
    /// The value to switch on or off to force the output to match the provided labels from users. If the value is True, the find matches transform forces the output to match the provided labels. The results override the normal conflation results. If the value is False, the find matches transform does not ensure all the labels provided are respected, and the results rely on the trained model.
    /// 
    /// Note that setting this value to true may increase the conflation execution time.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnforceProvidedLabels")]
    pub enforce_provided_labels: Option<bool>,


    /// 
    /// The value selected when tuning your transform for a balance between precision and recall.    A value of 0.5 means no preference; a value of 1.0 means a bias purely for precision, and a    value of 0.0 means a bias for recall. Because this is a tradeoff, choosing values close to 1.0    means very low recall, and choosing values close to 0.0 results in very low precision.
    /// 
    /// The precision metric indicates how often your model is correct when it predicts a match.
    /// 
    /// The recall metric indicates that for an actual match, how often your model predicts the    match.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrecisionRecallTradeoff")]
    pub precision_recall_tradeoff: Option<f64>,


    /// 
    /// The name of a column that uniquely identifies rows in the source table. Used to help identify matching records.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrimaryKeyColumnName")]
    pub primary_key_column_name: String,

}




/// The database and table in the AWS Glue Data Catalog that is used for input or output data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlueTables {


    /// A unique identifier for the AWS Glue Data Catalog.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// The name of the connection to the AWS Glue Data Catalog.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,


    /// A database name in the AWS Glue Data Catalog.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// A table name in the AWS Glue Data Catalog.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,

}




/// A list of AWS Glue table definitions used by the transform.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputRecordTables {


    /// The database and table in the AWS Glue Data Catalog that is used for input or output data.
    ///
    /// Required: No
    ///
    /// Type: List of GlueTables
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueTables")]
    pub glue_tables: Option<Vec<GlueTables>>,

}




/// The encryption-at-rest settings of the transform that apply to accessing user data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MLUserDataEncryption {


    /// 
    /// The ID for the customer-provided KMS key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The encryption mode applied to user data. Valid values are:
    /// 
    /// DISABLED: encryption is disabled.            SSEKMS: use of server-side encryption with AWS Key Management Service (SSE-KMS) for user data stored in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MLUserDataEncryptionMode")]
    pub mluser_data_encryption_mode: String,

}




/// The encryption-at-rest settings of the transform that apply to accessing user data. Machine learning transforms can access user data encrypted in Amazon S3 using KMS.
///
/// Additionally, imported labels and trained transforms can now be encrypted using a customer provided KMS key.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformEncryption {


    /// 
    /// The encryption-at-rest settings of the transform that apply to accessing user data.
    /// 
    /// Required: No
    ///
    /// Type: MLUserDataEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "MLUserDataEncryption")]
    pub mluser_data_encryption: Option<MLUserDataEncryption>,


    /// 
    /// The name of the security configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskRunSecurityConfigurationName")]
    pub task_run_security_configuration_name: Option<String>,

}




/// The algorithm-specific parameters that are associated with the machine learning    transform.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformParameters {


    /// 
    /// The parameters for the find matches algorithm.
    /// 
    /// Required: No
    ///
    /// Type: FindMatchesParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindMatchesParameters")]
    pub find_matches_parameters: Option<FindMatchesParameters>,


    /// 
    /// The type of machine learning transform. FIND_MATCHES is the only option.
    /// 
    /// For information about the types of machine learning transforms, see Creating Machine Learning Transforms.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FIND_MATCHES
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransformType")]
    pub transform_type: TransformParametersTransformTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TransformParametersTransformTypeEnum {

    /// FIND_MATCHES
    #[serde(rename = "FIND_MATCHES")]
    Findmatches,

}

impl Default for TransformParametersTransformTypeEnum {
    fn default() -> Self {
        TransformParametersTransformTypeEnum::Findmatches
    }
}

