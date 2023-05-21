

/// Creates an Amazon SageMaker Model Card.
///
/// For information about how to use model cards, see Amazon SageMaker Model Card.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModelCard {


    /// 
    /// The security configuration used to protect model card data.
    /// 
    /// Required: No
    ///
    /// Type: SecurityConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfig")]
    pub security_config: Option<SecurityConfig>,


    /// 
    /// The approval status of the model card within your organization. Different organizations might have different criteria for model card review and approval.
    /// 
    /// Draft: The model card is a work in progress.                        PendingReview: The model card is pending review.                        Approved: The model card is approved.                        Archived: The model card is archived. No more updates should be made to the model        card, but it can still be exported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Approved | Archived | Draft | PendingReview
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelCardStatus")]
    pub model_card_status: ModelCardModelCardStatusEnum,


    /// 
    /// Information about the user who created or modified one or more of the       following:
    /// 
    /// Experiment               Trial               Trial component               Lineage group               Project               Model Card
    /// 
    /// Required: No
    ///
    /// Type: UserContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<UserContext>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: UserContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedBy")]
    pub last_modified_by: Option<UserContext>,


    /// 
    /// Key-value pairs used to manage metadata for the model card.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The content of the model card. Content uses the model card JSON         schema.
    /// 
    /// Required: Yes
    ///
    /// Type: Content
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: Content,


    /// 
    /// The unique name of the model card.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelCardName")]
    pub model_card_name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ModelCardModelCardStatusEnum {

    /// Approved
    #[serde(rename = "Approved")]
    Approved,

    /// Archived
    #[serde(rename = "Archived")]
    Archived,

    /// Draft
    #[serde(rename = "Draft")]
    Draft,

    /// PendingReview
    #[serde(rename = "PendingReview")]
    Pendingreview,

}

impl Default for ModelCardModelCardStatusEnum {
    fn default() -> Self {
        ModelCardModelCardStatusEnum::Approved
    }
}


impl cfn_resources::CfnResource for CfnModelCard {
    fn type_string() -> &'static str {
        "AWS::SageMaker::ModelCard"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A result from a SageMaker training job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrainingMetric {


    /// 
    /// The name of the result from the SageMaker training job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value of a result from the SageMaker training job.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,


    /// 
    /// Any additional notes describing the result of the training job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

}




/// Function details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Function {


    /// 
    /// An optional description of any conditions of your objective function metric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: Option<String>,


    /// 
    /// The optimization direction of the model's objective function. You must specify one of       the following values:
    /// 
    /// Maximize               Minimize
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Function")]
    pub function: Option<String>,


    /// 
    /// The metric of the model's objective function. For example, loss       or rmse. The following list shows examples of the values that you can specify for the metric:
    /// 
    /// ACCURACY               AUC               LOSS               MAE               RMSE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Facet")]
    pub facet: Option<String>,

}




/// The content of the model card. It follows the model card json         schema.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Content {


    /// 
    /// An overview about the model's evaluation.
    /// 
    /// Required: No
    ///
    /// Type: List of EvaluationDetail
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationDetails")]
    pub evaluation_details: Option<Vec<EvaluationDetail>>,


    /// 
    /// The intended usage of the model.
    /// 
    /// Required: No
    ///
    /// Type: IntendedUses
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntendedUses")]
    pub intended_uses: Option<IntendedUses>,


    /// 
    /// Information about how the model supports business goals.
    /// 
    /// Required: No
    ///
    /// Type: BusinessDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessDetails")]
    pub business_details: Option<BusinessDetails>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ModelPackageDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageDetails")]
    pub model_package_details: Option<ModelPackageDetails>,


    /// 
    /// An overview about model training.
    /// 
    /// Required: No
    ///
    /// Type: TrainingDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingDetails")]
    pub training_details: Option<TrainingDetails>,


    /// 
    /// An overview about the model
    /// 
    /// Required: No
    ///
    /// Type: ModelOverview
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelOverview")]
    pub model_overview: Option<ModelOverview>,


    /// 
    /// Additional information about the model.
    /// 
    /// Required: No
    ///
    /// Type: AdditionalInformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: Option<AdditionalInformation>,

}




/// Information about how the model supports business goals.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BusinessDetails {


    /// 
    /// The broader business need that the model is serving.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineOfBusiness")]
    pub line_of_business: Option<String>,


    /// 
    /// The relevant stakeholders for the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessStakeholders")]
    pub business_stakeholders: Option<String>,


    /// 
    /// The specific business problem that the model is trying to solve.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessProblem")]
    pub business_problem: Option<String>,

}




/// Information about the user who created or modified an experiment, trial, trial    component, lineage group, project, or model card.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserContext {


    /// 
    /// The domain associated with the user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainId")]
    pub domain_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the user's profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileArn")]
    pub user_profile_arn: Option<String>,


    /// 
    /// The name of the user's profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,

}




/// Additional information about the model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdditionalInformation {


    /// 
    /// Any additional information to document about the model.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomDetails")]
    pub custom_details: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Caveats and recommendations for those who might use this model in their       applications.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaveatsAndRecommendations")]
    pub caveats_and_recommendations: Option<String>,


    /// 
    /// Any ethical considerations documented by the model card author.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EthicalConsiderations")]
    pub ethical_considerations: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}




/// SageMaker training image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrainingEnvironment {


    /// 
    /// SageMaker inference image URI.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerImage")]
    pub container_image: Option<Vec<String>>,

}




/// The Container property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Container {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NearestModelName")]
    pub nearest_model_name: Option<String>,

}




/// The function that is optimized during model training.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ObjectiveFunction {


    /// 
    /// A function object that details optimization direction, metric, and additional       descriptions.
    /// 
    /// Required: No
    ///
    /// Type: Function
    ///
    /// Update requires: No interruption
    #[serde(rename = "Function")]
    pub function: Option<Function>,


    /// 
    /// Notes about the object function, including other considerations for possible objective       functions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

}




/// The ModelPackageCreator property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelPackageCreator {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,

}




/// The intended uses of a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IntendedUses {


    /// 
    /// Your organization's risk rating. You can specify one the following values as the risk rating:
    /// 
    /// High               Medium               Low               Unknown
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RiskRating")]
    pub risk_rating: Option<String>,


    /// 
    /// The intended use cases for the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntendedUses")]
    pub intended_uses: Option<String>,


    /// 
    /// An explanation of why your organization categorizes the model with its risk       rating.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplanationsForRiskRating")]
    pub explanations_for_risk_rating: Option<String>,


    /// 
    /// Factors affecting model efficacy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FactorsAffectingModelEfficiency")]
    pub factors_affecting_model_efficiency: Option<String>,


    /// 
    /// The general purpose of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PurposeOfModel")]
    pub purpose_of_model: Option<String>,

}




/// An overview about the model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelOverview {


    /// 
    /// A description of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDescription")]
    pub model_description: Option<String>,


    /// 
    /// The name of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,


    /// 
    /// The location of the model artifact.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelArtifact")]
    pub model_artifact: Option<Vec<String>>,


    /// 
    /// The creator of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelCreator")]
    pub model_creator: Option<String>,


    /// 
    /// An overview about model inference.
    /// 
    /// Required: No
    ///
    /// Type: InferenceEnvironment
    ///
    /// Update requires: No interruption
    #[serde(rename = "InferenceEnvironment")]
    pub inference_environment: Option<InferenceEnvironment>,


    /// 
    /// The algorithm used to solve the problem.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlgorithmType")]
    pub algorithm_type: Option<String>,


    /// 
    /// The owner of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelOwner")]
    pub model_owner: Option<String>,


    /// 
    /// The version of the model.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelVersion")]
    pub model_version: Option<f64>,


    /// 
    /// The SageMaker Model ARN or non-SageMaker Model ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelId")]
    pub model_id: Option<String>,


    /// 
    /// The problem being solved with the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProblemType")]
    pub problem_type: Option<String>,

}




/// An overview of a model's inference environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InferenceEnvironment {


    /// 
    /// The container used to run the inference environment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerImage")]
    pub container_image: Option<Vec<String>>,

}




/// The evaluation details of the model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EvaluationDetail {


    /// 
    /// The Amazon Resource Name (ARN) of the evaluation job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationJobArn")]
    pub evaluation_job_arn: Option<String>,


    /// 
    /// Any observations made during the model evaluation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationObservation")]
    pub evaluation_observation: Option<String>,


    /// 
    /// The evaluation job name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// An evaluation Metric Group object.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricGroups")]
    pub metric_groups: Option<Vec<MetricGroup>>,


    /// 
    /// Additional attributes associated with the evaluation results.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The location of the datasets used to evaluate the model.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datasets")]
    pub datasets: Option<Vec<String>>,

}




/// The InferenceSpecification property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InferenceSpecification {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of Container
    ///
    /// Update requires: No interruption
    #[serde(rename = "Containers")]
    pub containers: Vec<Container>,

}




/// Metric data. The type determines the data types that you specify for         value, XAxisName and YAxisName. For       information about specifying values for metrics, see model card JSON         schema.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDataItems {


    /// 
    /// You must specify one of the following data types:
    /// 
    /// Bar Chart –           bar_char               Boolean –           boolean               Linear Graph –           linear_graph               Matrix –           matrix               Number –           number               String –           string
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// Any notes to add to the metric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notes")]
    pub notes: Option<String>,


    /// 
    /// The datatype of the metric. The metric's value must be compatible       with the metric's type.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: serde_json::Value,


    /// 
    /// The names of the metrics.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The name of the x axis.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisName")]
    pub xaxis_name: Option<Vec<String>>,


    /// 
    /// The name of the y axis.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "YAxisName")]
    pub yaxis_name: Option<Vec<String>>,

}




/// A hyper parameter that was configured in training the model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrainingHyperParameter {


    /// 
    /// The name of the hyper parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value specified for the hyper parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}




/// A group of metric data that you use to initialize a metric group object.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricGroup {


    /// 
    /// The metric group name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A list of metric objects. The MetricDataItems list can have one of the following values:
    /// 
    /// bar_chart_metric               matrix_metric               simple_metric               linear_graph_metric
    /// 
    /// For more information about the metric schema, see the definition section of the model card JSON       schema.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricDataItems
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricData")]
    pub metric_data: Vec<MetricDataItems>,

}




/// The ModelPackageDetails property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelPackageDetails {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalDescription")]
    pub approval_description: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of SourceAlgorithm
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAlgorithms")]
    pub source_algorithms: Option<Vec<SourceAlgorithm>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageVersion")]
    pub model_package_version: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Task")]
    pub task: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelApprovalStatus")]
    pub model_approval_status: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: InferenceSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "InferenceSpecification")]
    pub inference_specification: Option<InferenceSpecification>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageStatus")]
    pub model_package_status: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageDescription")]
    pub model_package_description: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ModelPackageCreator
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<ModelPackageCreator>,

}




/// The SourceAlgorithm property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceAlgorithm {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,

}




/// The overview of a training job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrainingJobDetails {


    /// 
    /// The SageMaker training job image URI.
    /// 
    /// Required: No
    ///
    /// Type: TrainingEnvironment
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingEnvironment")]
    pub training_environment: Option<TrainingEnvironment>,


    /// 
    /// The SageMaker training job Amazon Resource Name (ARN)
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingArn")]
    pub training_arn: Option<String>,


    /// 
    /// Additional hyper parameters that you've specified when training the model.
    /// 
    /// Required: No
    ///
    /// Type: List of TrainingHyperParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProvidedHyperParameters")]
    pub user_provided_hyper_parameters: Option<Vec<TrainingHyperParameter>>,


    /// 
    /// The location of the datasets used to train the model.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingDatasets")]
    pub training_datasets: Option<Vec<String>>,


    /// 
    /// The hyper parameters used in the training job.
    /// 
    /// Required: No
    ///
    /// Type: List of TrainingHyperParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "HyperParameters")]
    pub hyper_parameters: Option<Vec<TrainingHyperParameter>>,


    /// 
    /// Custom training job results.
    /// 
    /// Required: No
    ///
    /// Type: List of TrainingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProvidedTrainingMetrics")]
    pub user_provided_training_metrics: Option<Vec<TrainingMetric>>,


    /// 
    /// The SageMaker training job results.
    /// 
    /// Required: No
    ///
    /// Type: List of TrainingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingMetrics")]
    pub training_metrics: Option<Vec<TrainingMetric>>,

}




/// The security configuration used to protect model card data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecurityConfig {


    /// 
    /// A AWS Key Management Service       key ID used to encrypt a model card.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}




/// The training details of the model
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrainingDetails {


    /// 
    /// The function that is optimized during model training.
    /// 
    /// Required: No
    ///
    /// Type: ObjectiveFunction
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectiveFunction")]
    pub objective_function: Option<ObjectiveFunction>,


    /// 
    /// Any observations about training.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingObservations")]
    pub training_observations: Option<String>,


    /// 
    /// Details about any associated training jobs.
    /// 
    /// Required: No
    ///
    /// Type: TrainingJobDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingJobDetails")]
    pub training_job_details: Option<TrainingJobDetails>,

}


