

/// The AWS::Inspector::AssessmentTarget resource is used to create Amazon     Inspector assessment targets, which specify the Amazon EC2 instances that will be analyzed     during an assessment run.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessmentTarget {


    /// 
    /// The ARN that specifies the resource group that is used to create the assessment     target. If resourceGroupArn is not specified, all EC2 instances in the current AWS account     and Region are included in the assessment target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceGroupArn")]
    pub resource_group_arn: Option<String>,


    /// 
    /// The name of the Amazon Inspector assessment target. The name must be unique within     the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssessmentTargetName")]
    pub assessment_target_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnAssessmentTarget {
    fn type_string() -> &'static str {
        "AWS::Inspector::AssessmentTarget"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
