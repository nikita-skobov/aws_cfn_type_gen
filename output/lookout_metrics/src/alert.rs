

/// The AWS::LookoutMetrics::Alert type creates an alert for an anomaly detector.
#[derive(Default, serde::Serialize)]
pub struct CfnAlert {


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
    pub alert_description: Option<String>,


    /// 
    /// The name of the alert.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlertName")]
    pub alert_name: Option<String>,


    /// 
    /// The ARN of the detector to which the alert is attached.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AnomalyDetectorArn")]
    pub anomaly_detector_arn: String,

}


/// Contains information about the SNS topic to which you want to send your alerts and the IAM role that has    access to that topic.
#[derive(Default, serde::Serialize)]
pub struct SNSConfiguration {


    /// 
    /// The ARN of the target SNS topic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,


    /// 
    /// The ARN of the IAM role that has access to the target SNS topic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


/// A configuration that specifies the action to perform when anomalies are detected.
#[derive(Default, serde::Serialize)]
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
    pub snsconfiguration: Option<SNSConfiguration>,

}


/// Contains information about a Lambda configuration.
#[derive(Default, serde::Serialize)]
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
    pub lambda_arn: String,


    /// 
    /// The ARN of an IAM role that has permission to invoke the Lambda function.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}