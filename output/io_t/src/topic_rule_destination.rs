/// A topic rule destination.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTopicRuleDestination {
    ///
    /// Properties of the HTTP URL.
    ///
    /// Required: No
    ///
    /// Type: HttpUrlDestinationSummary
    ///
    /// Update requires: Replacement
    #[serde(rename = "HttpUrlProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_properties: Option<HttpUrlDestinationSummary>,

    ///
    /// IN_PROGRESS               A topic rule destination was created but has not been confirmed. You can set status to IN_PROGRESS by calling UpdateTopicRuleDestination. Calling UpdateTopicRuleDestination causes a new confirmation challenge to be sent to your confirmation endpoint.                   ENABLED               Confirmation was completed, and traffic to this destination is allowed. You can set status to DISABLED by calling UpdateTopicRuleDestination.                    DISABLED                  Confirmation was completed, and traffic to this destination is not allowed. You can set status to ENABLED by calling UpdateTopicRuleDestination.                       ERROR                  Confirmation could not be completed; for example, if the confirmation timed           out. You can call GetTopicRuleDestination for details about the           error. You can set status to IN_PROGRESS by calling             UpdateTopicRuleDestination. Calling             UpdateTopicRuleDestination causes a new confirmation challenge           to be sent to your confirmation endpoint.
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
    /// Properties of the virtual private cloud (VPC) connection.
    ///
    /// Required: No
    ///
    /// Type: VpcDestinationProperties
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_properties: Option<VpcDestinationProperties>,

    #[serde(skip_serializing)]
    pub att_arn: CfnTopicRuleDestinationarn,

    #[serde(skip_serializing)]
    pub att_status_reason: CfnTopicRuleDestinationstatusreason,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopicRuleDestinationarn;
impl CfnTopicRuleDestinationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopicRuleDestinationstatusreason;
impl CfnTopicRuleDestinationstatusreason {
    pub fn att_name(&self) -> &'static str {
        r#"StatusReason"#
    }
}

impl cfn_resources::CfnResource for CfnTopicRuleDestination {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::TopicRuleDestination"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.http_url_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// HTTP URL destination properties.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpUrlDestinationSummary {
    ///
    /// The URL used to confirm the HTTP topic rule destination URL.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfirmationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HttpUrlDestinationSummary {
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

/// The properties of a virtual private cloud (VPC) destination.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VpcDestinationProperties {
    ///
    /// The ARN of a role that has permission to create and attach to elastic network interfaces (ENIs).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The security groups of the VPC destination.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The subnet IDs of the VPC destination.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// The ID of the VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpcDestinationProperties {
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
