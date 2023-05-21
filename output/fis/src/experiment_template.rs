

/// Specifies an experiment template.
///
/// An experiment template includes the following components:
///
/// For more information, see Experiment templates     in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnExperimentTemplate {


    /// 
    /// The tags to apply to the experiment template.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: std::collections::HashMap<String, String>,


    /// 
    /// The stop conditions.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ExperimentTemplateStopCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "StopConditions")]
    pub stop_conditions: Vec<ExperimentTemplateStopCondition>,


    /// 
    /// The configuration for experiment logging.
    /// 
    /// Required: No
    ///
    /// Type: ExperimentTemplateLogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<ExperimentTemplateLogConfiguration>,


    /// 
    /// The targets for the experiment.
    /// 
    /// Required: Yes
    ///
    /// Type: Map of ExperimentTemplateTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: std::collections::HashMap<String, ExperimentTemplateTarget>,


    /// 
    /// The actions for the experiment.
    /// 
    /// Required: No
    ///
    /// Type: Map of ExperimentTemplateAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<std::collections::HashMap<String, ExperimentTemplateAction>>,


    /// 
    /// The Amazon Resource Name (ARN) of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// A description for the experiment template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: String,

}



impl cfn_resources::CfnResource for CfnExperimentTemplate {
    fn type_string() -> &'static str {
        "AWS::FIS::ExperimentTemplate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies a stop condition for an experiment template.
///
/// For more information, see Stop conditions    in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExperimentTemplateStopCondition {


    /// 
    /// The Amazon Resource Name (ARN) of the CloudWatch alarm. This is required if the source is     a CloudWatch alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The source for the stop condition. Specify aws:cloudwatch:alarm if the stop     condition is defined by a CloudWatch alarm. Specify none if there is no stop     condition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: String,

}




/// The CloudWatchLogsConfiguration property type specifies Property description not available. for an AWS::FIS::ExperimentTemplate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogsConfiguration {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: String,

}




/// Specifies a target for an experiment. You must specify at least one Amazon Resource Name (ARN) or      at least one resource tag. You cannot specify both ARNs and tags.
///
/// For more information, see Targets     in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExperimentTemplateTarget {


    /// 
    /// The resource type. The resource type must be supported for the specified action.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: String,


    /// 
    /// The Amazon Resource Names (ARNs) of the resources.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Option<Vec<String>>,


    /// 
    /// Scopes the identified resources to a specific count of the resources at random, or a percentage of the resources. All identified resources are included in the target.
    /// 
    /// ALL - Run the action on all identified targets. This is the default.               COUNT(n) - Run the action on the specified number of targets, chosen from the identified targets at random.         For example, COUNT(1) selects one of the targets.               PERCENT(n) - Run the action on the specified percentage of targets, chosen from the identified targets         at random. For example, PERCENT(25) selects 25% of the targets.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectionMode")]
    pub selection_mode: String,


    /// 
    /// The tags for the target resources.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The parameters for the resource type.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The filters to apply to identify target resources using specific attributes.
    /// 
    /// Required: No
    ///
    /// Type: List of ExperimentTemplateTargetFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<ExperimentTemplateTargetFilter>>,

}




/// The S3Configuration property type specifies Property description not available. for an AWS::FIS::ExperimentTemplate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Configuration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}




/// Specifies the configuration for experiment logging.
///
/// For more information, see Experiment logging    in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExperimentTemplateLogConfiguration {


    /// 
    /// The schema version. The supported value is 1.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogSchemaVersion")]
    pub log_schema_version: i64,


    /// 
    /// The configuration for experiment logging to Amazon S3. The following fields are     supported:
    /// 
    /// bucketName - The name of the destination bucket.prefix - An optional bucket prefix.
    /// 
    /// For example:
    /// 
    /// {"BucketName": "my-s3-bucket", "Prefix": "log-folder"}
    /// 
    /// Required: No
    ///
    /// Type: S3Configuration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: Option<S3Configuration>,


    /// 
    /// The configuration for experiment logging to Amazon CloudWatch Logs. The supported field     is LogGroupArn. For example:
    /// 
    /// {"LogGroupArn": "aws:arn:logs:region_name:account_id:log-group:log_group_name"}
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLogsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsConfiguration")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfiguration>,

}




/// Specifies a filter used for the target resource input in an experiment template.
///
/// For more information, see Resource filters     in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExperimentTemplateTargetFilter {


    /// 
    /// The attribute path for the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: String,


    /// 
    /// The attribute values for the filter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}




/// Specifies an action for an experiment template.
///
/// For more information, see Actions     in the AWS Fault Injection Simulator User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExperimentTemplateAction {


    /// 
    /// The targets for the action.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The ID of the action. The format of the action ID is: aws:service-name:action-type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionId")]
    pub action_id: String,


    /// 
    /// The parameters for the action, if applicable.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// A description for the action.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the action that must be completed before the current action starts. Omit this parameter to run the action at the start of the experiment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartAfter")]
    pub start_after: Option<Vec<String>>,

}


