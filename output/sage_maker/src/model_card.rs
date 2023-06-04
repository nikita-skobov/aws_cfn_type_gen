/// Creates an Amazon SageMaker Model Card.
///
/// For information about how to use model cards, see Amazon SageMaker Model Card.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnModelCard {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserContext>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: UserContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<UserContext>,

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
    pub model_card_name: cfn_resources::StrVal,

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
    /// The security configuration used to protect model card data.
    ///
    /// Required: No
    ///
    /// Type: SecurityConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config: Option<SecurityConfig>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_created_by_domain_id: CfnModelCardcreatedbydomainid,

    #[serde(skip_serializing)]
    pub att_created_by_user_profile_arn: CfnModelCardcreatedbyuserprofilearn,

    #[serde(skip_serializing)]
    pub att_created_by_user_profile_name: CfnModelCardcreatedbyuserprofilename,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnModelCardcreationtime,

    #[serde(skip_serializing)]
    pub att_last_modified_by_domain_id: CfnModelCardlastmodifiedbydomainid,

    #[serde(skip_serializing)]
    pub att_last_modified_by_user_profile_arn: CfnModelCardlastmodifiedbyuserprofilearn,

    #[serde(skip_serializing)]
    pub att_last_modified_by_user_profile_name: CfnModelCardlastmodifiedbyuserprofilename,

    #[serde(skip_serializing)]
    pub att_last_modified_time: CfnModelCardlastmodifiedtime,

    #[serde(skip_serializing)]
    pub att_model_card_arn: CfnModelCardmodelcardarn,

    #[serde(skip_serializing)]
    pub att_model_card_processing_status: CfnModelCardmodelcardprocessingstatus,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardcreatedbydomainid;
impl CfnModelCardcreatedbydomainid {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedBy.DomainId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardcreatedbyuserprofilearn;
impl CfnModelCardcreatedbyuserprofilearn {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedBy.UserProfileArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardcreatedbyuserprofilename;
impl CfnModelCardcreatedbyuserprofilename {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedBy.UserProfileName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardcreationtime;
impl CfnModelCardcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardlastmodifiedbydomainid;
impl CfnModelCardlastmodifiedbydomainid {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedBy.DomainId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardlastmodifiedbyuserprofilearn;
impl CfnModelCardlastmodifiedbyuserprofilearn {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedBy.UserProfileArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardlastmodifiedbyuserprofilename;
impl CfnModelCardlastmodifiedbyuserprofilename {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedBy.UserProfileName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardlastmodifiedtime;
impl CfnModelCardlastmodifiedtime {
    pub fn att_name(&self) -> &'static str {
        r#"LastModifiedTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardmodelcardarn;
impl CfnModelCardmodelcardarn {
    pub fn att_name(&self) -> &'static str {
        r#"ModelCardArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelCardmodelcardprocessingstatus;
impl CfnModelCardmodelcardprocessingstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ModelCardProcessingStatus"#
    }
}

impl cfn_resources::CfnResource for CfnModelCard {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::ModelCard"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.content.validate()?;

        self.created_by
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.last_modified_by
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.model_card_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'model_card_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.model_card_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'model_card_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.security_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Additional information about the model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdditionalInformation {
    ///
    /// Caveats and recommendations for those who might use this model in their       applications.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaveatsAndRecommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caveats_and_recommendations: Option<cfn_resources::StrVal>,

    ///
    /// Any additional information to document about the model.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<std::collections::HashMap<String, String>>,

    ///
    /// Any ethical considerations documented by the model card author.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EthicalConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethical_considerations: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AdditionalInformation {
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

/// Information about how the model supports business goals.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BusinessDetails {
    ///
    /// The specific business problem that the model is trying to solve.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessProblem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_problem: Option<cfn_resources::StrVal>,

    ///
    /// The relevant stakeholders for the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessStakeholders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_stakeholders: Option<cfn_resources::StrVal>,

    ///
    /// The broader business need that the model is serving.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LineOfBusiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_business: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BusinessDetails {
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

/// The Container property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Container {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NearestModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearest_model_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Container {
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

/// The content of the model card. It follows the model card json         schema.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Content {
    ///
    /// Additional information about the model.
    ///
    /// Required: No
    ///
    /// Type: AdditionalInformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<AdditionalInformation>,

    ///
    /// Information about how the model supports business goals.
    ///
    /// Required: No
    ///
    /// Type: BusinessDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "BusinessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_details: Option<BusinessDetails>,

    ///
    /// An overview about the model's evaluation.
    ///
    /// Required: No
    ///
    /// Type: List of EvaluationDetail
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_uses: Option<IntendedUses>,

    ///
    /// An overview about the model
    ///
    /// Required: No
    ///
    /// Type: ModelOverview
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelOverview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_overview: Option<ModelOverview>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ModelPackageDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_details: Option<TrainingDetails>,
}

impl cfn_resources::CfnResource for Content {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.additional_information
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.business_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.intended_uses
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.model_overview
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.model_package_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.training_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The evaluation details of the model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EvaluationDetail {
    ///
    /// The location of the datasets used to evaluate the model.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<String>>,

    ///
    /// The Amazon Resource Name (ARN) of the evaluation job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_job_arn: Option<cfn_resources::StrVal>,

    ///
    /// Any observations made during the model evaluation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationObservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_observation: Option<cfn_resources::StrVal>,

    ///
    /// Additional attributes associated with the evaluation results.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    ///
    /// An evaluation Metric Group object.
    ///
    /// Required: No
    ///
    /// Type: List of MetricGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_groups: Option<Vec<MetricGroup>>,

    ///
    /// The evaluation job name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EvaluationDetail {
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

/// Function details.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Function {
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

/// An overview of a model's inference environment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_image: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for InferenceEnvironment {
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

/// The InferenceSpecification property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for InferenceSpecification {
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

/// The intended uses of a model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IntendedUses {
    ///
    /// An explanation of why your organization categorizes the model with its risk       rating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplanationsForRiskRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanations_for_risk_rating: Option<cfn_resources::StrVal>,

    ///
    /// Factors affecting model efficacy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FactorsAffectingModelEfficiency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors_affecting_model_efficiency: Option<cfn_resources::StrVal>,

    ///
    /// The intended use cases for the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntendedUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_uses: Option<cfn_resources::StrVal>,

    ///
    /// The general purpose of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PurposeOfModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose_of_model: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_rating: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for IntendedUses {
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

/// Metric data. The type determines the data types that you specify for         value, XAxisName and YAxisName. For       information about specifying values for metrics, see model card JSON         schema.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricDataItems {
    ///
    /// The names of the metrics.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Any notes to add to the metric.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<cfn_resources::StrVal>,

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
    pub cfn_type: cfn_resources::StrVal,

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
    /// The name of the x axis.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "XAxisName")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaxis_name: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for MetricDataItems {
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

/// A group of metric data that you use to initialize a metric group object.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricGroup {
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

    ///
    /// The metric group name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MetricGroup {
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

/// An overview about the model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ModelOverview {
    ///
    /// The algorithm used to solve the problem.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlgorithmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_type: Option<cfn_resources::StrVal>,

    ///
    /// An overview about model inference.
    ///
    /// Required: No
    ///
    /// Type: InferenceEnvironment
    ///
    /// Update requires: No interruption
    #[serde(rename = "InferenceEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_environment: Option<InferenceEnvironment>,

    ///
    /// The location of the model artifact.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelArtifact")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_creator: Option<cfn_resources::StrVal>,

    ///
    /// A description of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_description: Option<cfn_resources::StrVal>,

    ///
    /// The SageMaker Model ARN or non-SageMaker Model ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<cfn_resources::StrVal>,

    ///
    /// The owner of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_owner: Option<cfn_resources::StrVal>,

    ///
    /// The version of the model.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<f64>,

    ///
    /// The problem being solved with the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProblemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ModelOverview {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.inference_environment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The ModelPackageCreator property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ModelPackageCreator {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ModelPackageCreator {
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

/// The ModelPackageDetails property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ModelPackageDetails {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApprovalDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_description: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ModelPackageCreator
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ModelPackageCreator>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: InferenceSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "InferenceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_specification: Option<InferenceSpecification>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelApprovalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_approval_status: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_arn: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_description: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_group_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_status: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_package_version: Option<f64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of SourceAlgorithm
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_algorithms: Option<Vec<SourceAlgorithm>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ModelPackageDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.created_by
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.inference_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The function that is optimized during model training.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ObjectiveFunction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.function
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The security configuration used to protect model card data.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SecurityConfig {
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

/// The SourceAlgorithm property type specifies Property description not available. for an AWS::SageMaker::ModelCard.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceAlgorithm {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelDataUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SourceAlgorithm {
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

/// The training details of the model
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_function: Option<ObjectiveFunction>,

    ///
    /// Details about any associated training jobs.
    ///
    /// Required: No
    ///
    /// Type: TrainingJobDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingJobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_job_details: Option<TrainingJobDetails>,

    ///
    /// Any observations about training.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingObservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_observations: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TrainingDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.objective_function
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.training_job_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// SageMaker training image.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_image: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for TrainingEnvironment {
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

/// A hyper parameter that was configured in training the model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub name: cfn_resources::StrVal,

    ///
    /// The value specified for the hyper parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TrainingHyperParameter {
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

/// The overview of a training job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrainingJobDetails {
    ///
    /// The hyper parameters used in the training job.
    ///
    /// Required: No
    ///
    /// Type: List of TrainingHyperParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "HyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<Vec<TrainingHyperParameter>>,

    ///
    /// The SageMaker training job Amazon Resource Name (ARN)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_arn: Option<cfn_resources::StrVal>,

    ///
    /// The location of the datasets used to train the model.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingDatasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_datasets: Option<Vec<String>>,

    ///
    /// The SageMaker training job image URI.
    ///
    /// Required: No
    ///
    /// Type: TrainingEnvironment
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_environment: Option<TrainingEnvironment>,

    ///
    /// The SageMaker training job results.
    ///
    /// Required: No
    ///
    /// Type: List of TrainingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrainingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_metrics: Option<Vec<TrainingMetric>>,

    ///
    /// Additional hyper parameters that you've specified when training the model.
    ///
    /// Required: No
    ///
    /// Type: List of TrainingHyperParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProvidedHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_provided_hyper_parameters: Option<Vec<TrainingHyperParameter>>,

    ///
    /// Custom training job results.
    ///
    /// Required: No
    ///
    /// Type: List of TrainingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProvidedTrainingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_provided_training_metrics: Option<Vec<TrainingMetric>>,
}

impl cfn_resources::CfnResource for TrainingJobDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.training_environment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A result from a SageMaker training job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub name: cfn_resources::StrVal,

    ///
    /// Any additional notes describing the result of the training job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<cfn_resources::StrVal>,

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
}

impl cfn_resources::CfnResource for TrainingMetric {
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

/// Information about the user who created or modified an experiment, trial, trial    component, lineage group, project, or model card.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the user's profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the user's profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_profile_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for UserContext {
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
