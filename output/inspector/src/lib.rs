
pub mod cfn_assessment_target {

#[derive(serde::Serialize, Default)]
pub struct CfnAssessmentTarget {
    /// No documentation provided by AWS
    #[serde(rename = "AssessmentTargetName")]
    pub assessment_target_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceGroupArn")]
    pub resource_group_arn: Option<String>,

}



}

pub mod cfn_assessment_template {

#[derive(serde::Serialize, Default)]
pub struct CfnAssessmentTemplate {
    /// No documentation provided by AWS
    #[serde(rename = "AssessmentTargetArn")]
    pub assessment_target_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "AssessmentTemplateName")]
    pub assessment_template_name: Option<String>,
    /// List of Tag
    #[serde(rename = "UserAttributesForFindings")]
    pub user_attributes_for_findings: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: usize,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_resource_group {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceGroup {
    /// List of Tag
    #[serde(rename = "ResourceGroupTags")]
    pub resource_group_tags: Vec<Tag>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
