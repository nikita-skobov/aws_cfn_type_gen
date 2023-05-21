/// The AWS::Inspector::AssessmentTarget resource is used to create Amazon     Inspector assessment targets, which specify the Amazon EC2 instances that will be analyzed     during an assessment run.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAssessmentTarget {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_target_name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
}

impl cfn_resources::CfnResource for CfnAssessmentTarget {
    fn type_string(&self) -> &'static str {
        "AWS::Inspector::AssessmentTarget"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.assessment_target_name {
            if the_val.len() > 140 as _ {
                return Err(format!("Max validation failed on field 'assessment_target_name'. {} is greater than 140", the_val.len()));
            }
        }

        if let Some(the_val) = &self.assessment_target_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'assessment_target_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_group_arn {
            if the_val.len() > 300 as _ {
                return Err(format!(
                    "Max validation failed on field 'resource_group_arn'. {} is greater than 300",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_group_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'resource_group_arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
