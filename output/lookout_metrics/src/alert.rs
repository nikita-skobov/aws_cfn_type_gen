/// The AWS::LookoutMetrics::Alert type creates an alert for an anomaly detector.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAlert {
    ///
    /// Action that will be triggered when there is an alert.
    ///
    /// Required: Yes
    ///
    /// Type: Action
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: Action,

    ///
    /// A description of the alert.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlertDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the alert.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_name: Option<cfn_resources::StrVal>,

    ///
    /// An integer from 0 to 100 specifying the alert sensitivity threshold.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlertSensitivityThreshold")]
    pub alert_sensitivity_threshold: i64,

    ///
    /// The ARN of the detector to which the alert is attached.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AnomalyDetectorArn")]
    pub anomaly_detector_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnAlertarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAlertarn;
impl CfnAlertarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAlert {
    fn type_string(&self) -> &'static str {
        "AWS::LookoutMetrics::Alert"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

        Ok(())
    }
}

/// A configuration that specifies the action to perform when anomalies are detected.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Action {
    ///
    /// A configuration for an AWS Lambda channel.
    ///
    /// Required: No
    ///
    /// Type: LambdaConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_configuration: Option<LambdaConfiguration>,

    ///
    /// A configuration for an Amazon SNS channel.
    ///
    /// Required: No
    ///
    /// Type: SNSConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "SNSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snsconfiguration: Option<SNSConfiguration>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.snsconfiguration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about a Lambda configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaConfiguration {
    ///
    /// The ARN of the Lambda function.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: cfn_resources::StrVal,

    ///
    /// The ARN of an IAM role that has permission to invoke the Lambda function.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LambdaConfiguration {
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

/// Contains information about the SNS topic to which you want to send your alerts and the IAM role that has    access to that topic.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SNSConfiguration {
    ///
    /// The ARN of the IAM role that has access to the target SNS topic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The ARN of the target SNS topic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SNSConfiguration {
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
