
pub mod cfn_dataset {

#[derive(serde::Serialize, Default)]
pub struct CfnDataset {
    /// The Amazon Resource Name (ARN) of the dataset group to add the dataset to
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
    /// The type of dataset
    #[serde(rename = "DatasetType")]
    pub dataset_type: String,
    /// The ARN of the schema to associate with the dataset. The schema defines the dataset fields.
    #[serde(rename = "SchemaArn")]
    pub schema_arn: String,
    /// Initial DatasetImportJob for the created dataset
    #[serde(rename = "DatasetImportJob")]
    pub dataset_import_job: Option<DatasetImportJob>,
    /// The name for the dataset
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct DatasetImportJob {
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,
    #[serde(rename = "DataSource")]
    pub data_source: Option<()>,
    #[serde(rename = "DatasetImportJobArn")]
    pub dataset_import_job_arn: Option<String>,

}


}

pub mod cfn_dataset_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDatasetGroup {
    /// The ARN of the AWS Identity and Access Management (IAM) role that has permissions to access the AWS Key Management Service (KMS) key. Supplying an IAM role is only valid when also specifying a KMS key.
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// The Amazon Resource Name(ARN) of a AWS Key Management Service (KMS) key used to encrypt the datasets.
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    /// The name for the new dataset group.
    #[serde(rename = "Name")]
    pub name: String,
    /// The domain of a Domain dataset group.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

}



}

pub mod cfn_schema {

#[derive(serde::Serialize, Default)]
pub struct CfnSchema {
    /// Name for the schema.
    #[serde(rename = "Name")]
    pub name: String,
    /// The domain of a Domain dataset group.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// A schema in Avro JSON format.
    #[serde(rename = "Schema")]
    pub schema: String,

}



}

pub mod cfn_solution {

#[derive(serde::Serialize, Default)]
pub struct CfnSolution {
    /// When your have multiple event types (using an EVENT_TYPE schema field), this parameter specifies which event type (for example, 'click' or 'like') is used for training the model. If you do not provide an eventType, Amazon Personalize will use all interactions for training with equal weight regardless of type.
    #[serde(rename = "EventType")]
    pub event_type: Option<String>,
    /// The ARN of the dataset group that provides the training data.
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
    /// Whether to perform automated machine learning (AutoML). The default is false. For this case, you must specify recipeArn.
    #[serde(rename = "PerformAutoML")]
    pub perform_auto_ml: Option<bool>,
    /// Whether to perform hyperparameter optimization (HPO) on the specified or selected recipe. The default is false. When performing AutoML, this parameter is always true and you should not set it to false.
    #[serde(rename = "PerformHPO")]
    pub perform_hpo: Option<bool>,
    /// The configuration to use with the solution. When performAutoML is set to true, Amazon Personalize only evaluates the autoMLConfig section of the solution configuration.
    #[serde(rename = "SolutionConfig")]
    pub solution_config: Option<SolutionConfig>,
    /// The ARN of the recipe to use for model training. Only specified when performAutoML is false.
    #[serde(rename = "RecipeArn")]
    pub recipe_arn: Option<String>,
    /// The name for the solution
    #[serde(rename = "Name")]
    pub name: String,

}

pub type SolutionArn = String;
#[derive(serde::Serialize, Default)]
pub struct CategoricalHyperParameterRange {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct IntegerHyperParameterRange {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "MinValue")]
    pub min_value: Option<usize>,
    #[serde(rename = "MaxValue")]
    pub max_value: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SolutionConfig {
    #[serde(rename = "EventValueThreshold")]
    pub event_value_threshold: Option<String>,
    #[serde(rename = "AlgorithmHyperParameters")]
    pub algorithm_hyper_parameters: Option<()>,
    #[serde(rename = "FeatureTransformationParameters")]
    pub feature_transformation_parameters: Option<()>,
    #[serde(rename = "HpoConfig")]
    pub hpo_config: Option<()>,
    #[serde(rename = "AutoMLConfig")]
    pub auto_mlconfig: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct ContinuousHyperParameterRange {
    #[serde(rename = "MinValue")]
    pub min_value: Option<f64>,
    #[serde(rename = "MaxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}
