
pub mod cfn_app {

#[derive(serde::Serialize, Default)]
pub struct CfnApp {
    /// A string containing full ResilienceHub app template body.
    #[serde(rename = "AppTemplateBody")]
    pub app_template_body: String,
    /// Amazon Resource Name (ARN) of the Resiliency Policy.
    #[serde(rename = "ResiliencyPolicyArn")]
    pub resiliency_policy_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// An array of ResourceMapping objects.
    #[serde(rename = "ResourceMappings")]
    pub resource_mappings: Vec<ResourceMapping>,
    /// Assessment execution schedule.
    #[serde(rename = "AppAssessmentSchedule")]
    pub app_assessment_schedule: Option<String>,
    /// App description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Name of the app.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct PhysicalResourceId {
    #[serde(rename = "AwsRegion")]
    pub aws_region: Option<String>,
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Identifier")]
    pub identifier: String,

}
pub type TagValue = String;
#[derive(serde::Serialize, Default)]
pub struct TagMap {

}

#[derive(serde::Serialize, Default)]
pub struct ResourceMapping {
    #[serde(rename = "TerraformSourceName")]
    pub terraform_source_name: Option<String>,
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "MappingType")]
    pub mapping_type: String,
    #[serde(rename = "LogicalStackName")]
    pub logical_stack_name: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    pub physical_resource_id: PhysicalResourceId,

}


}

pub mod cfn_resiliency_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResiliencyPolicy {
    /// Name of Resiliency Policy.
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// Resiliency Policy Tier.
    #[serde(rename = "Tier")]
    pub tier: String,
    /// Description of Resiliency Policy.
    #[serde(rename = "PolicyDescription")]
    pub policy_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: PolicyMap,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// Data Location Constraint of the Policy.
    #[serde(rename = "DataLocationConstraint")]
    pub data_location_constraint: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PolicyMap {

}

#[derive(serde::Serialize, Default)]
pub struct FailurePolicy {
    #[serde(rename = "RtoInSecs")]
    pub rto_in_secs: usize,
    #[serde(rename = "RpoInSecs")]
    pub rpo_in_secs: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TagMap {

}
pub type TagValue = String;

}
