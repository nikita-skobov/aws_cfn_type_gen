/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProtection {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration
    #[serde(rename = "ApplicationLayerAutomaticResponseConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_layer_automatic_response_configuration:
        Option<ApplicationLayerAutomaticResponseConfiguration>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-healthcheckarns
    #[serde(rename = "HealthCheckArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_arns: Option<Vec<String>>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-name
    #[serde(rename = "Name")]
    pub name: String,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-resourcearn
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-tags
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnProtection {
    fn type_string(&self) -> &'static str {
        "AWS::Shield::Protection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.application_layer_automatic_response_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-block
    #[serde(rename = "Block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<serde_json::Value>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-count
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for Action {
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

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationLayerAutomaticResponseConfiguration {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-action
    #[serde(rename = "Action")]
    pub action: Action,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-status
    #[serde(rename = "Status")]
    pub status: String,
}

impl cfn_resources::CfnResource for ApplicationLayerAutomaticResponseConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

        Ok(())
    }
}

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-key
    #[serde(rename = "Key")]
    pub key: String,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html#cfn-resource-tags-value
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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
