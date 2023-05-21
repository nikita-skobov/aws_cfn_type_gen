

/// Creates an empty dataset and adds it to the specified dataset group.    Use CreateDatasetImportJob to import your training data to a    dataset.
///
/// There are three types of datasets:
///
/// Each dataset type has an associated schema with required field types.    Only the Interactions dataset is required in order to train a    model (also referred to as creating a solution).
///
/// A dataset can be in one of the following states:
///
/// To get the status of the dataset, call DescribeDataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataset {


    /// 
    /// The name of the dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the dataset group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,


    /// 
    /// The ARN of the associated schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,


    /// 
    /// One of the following values:
    /// 
    /// Interactions               Items               Users
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetType")]
    pub dataset_type: String,


    /// 
    /// Describes a job that imports training data from a data source (Amazon S3 bucket) to an       Amazon Personalize dataset.
    ///
    /// Required: No
    ///
    /// Type: DatasetImportJob
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetImportJob")]
    pub dataset_import_job: Option<DatasetImportJob>,

}



impl cfn_resources::CfnResource for CfnDataset {
    fn type_string() -> &'static str {
        "AWS::Personalize::Dataset"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The DataSource property type specifies Property description not available. for an AWS::Personalize::Dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSource {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLocation")]
    pub data_location: Option<String>,

}




/// Describes a job that imports training data from a data source (Amazon S3    bucket) to an Amazon Personalize dataset. For more information, see CreateDatasetImportJob.
///
/// A dataset import job can be in one of the following states:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetImportJob {


    /// 
    /// The ARN of the IAM role that has permissions to read from the Amazon S3    data source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the dataset that receives the    imported data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: Option<String>,


    /// 
    /// The Amazon S3 bucket that contains the training data to import.
    /// 
    /// Required: No
    ///
    /// Type: DataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSource")]
    pub data_source: Option<DataSource>,


    /// 
    /// The name of the import job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,


    /// 
    /// The ARN of the dataset import job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:([a-z\d-]+):personalize:.*:.*:.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetImportJobArn")]
    pub dataset_import_job_arn: Option<String>,

}


