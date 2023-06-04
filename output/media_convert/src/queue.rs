/// The AWS::MediaConvert::Queue resource is an AWS Elemental MediaConvert resource type       that you can use to manage the resources that are available to your account for parallel       processing of jobs. For more information about queues, see Working with AWS Elemental MediaConvert Queues in the AWS Elemental MediaConvert User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnQueue {
    ///
    /// Optional. A description of the queue that you are creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the queue that you are creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// When you use AWS CloudFormation, you can create only on-demand queues. Therefore,       always set PricingPlan to the value "ON_DEMAND" when declaring an       AWS::MediaConvert::Queue in your AWS CloudFormation template.
    ///
    /// To create a reserved queue, use the AWS Elemental MediaConvert console at       https://console.aws.amazon.com/mediaconvert to set up a contract. For more information,       see Working with AWS Elemental MediaConvert Queues in the AWS Elemental MediaConvert User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<cfn_resources::StrVal>,

    ///
    /// Initial state of the queue. Queues can be either ACTIVE or PAUSED. If you create a       paused queue, then jobs that you send to that queue won't begin.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnQueuearn,

    #[serde(skip_serializing)]
    pub att_name: CfnQueuename,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnQueuearn;
impl CfnQueuearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnQueuename;
impl CfnQueuename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnQueue {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConvert::Queue"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
