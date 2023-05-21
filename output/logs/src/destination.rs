/// The AWS::Logs::Destination resource specifies a CloudWatch Logs destination. A destination encapsulates a physical resource (such    as an Amazon Kinesis data stream) and enables you to subscribe that resource to a stream of log events.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDestination {
    ///
    /// The name of the destination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationName")]
    pub destination_name: cfn_resources::StrVal,

    ///
    /// An IAM policy document that governs which AWS accounts can create subscription filters    against this destination.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_policy: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of an IAM role that permits CloudWatch Logs to send data to the specified AWS resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the physical target where the log events are    delivered (for example, a Kinesis stream).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    pub target_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnDestination {
    fn type_string(&self) -> &'static str {
        "AWS::Logs::Destination"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.destination_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination_name'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.destination_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.destination_policy {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'destination_policy'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
