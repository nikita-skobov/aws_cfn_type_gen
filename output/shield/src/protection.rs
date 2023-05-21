

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProtection {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration
    #[serde(rename = "ApplicationLayerAutomaticResponseConfiguration")]
    pub application_layer_automatic_response_configuration: Option<ApplicationLayerAutomaticResponseConfiguration>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-resourcearn
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-name
    #[serde(rename = "Name")]
    pub name: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html#cfn-shield-protection-healthcheckarns
    #[serde(rename = "HealthCheckArns")]
    pub health_check_arns: Option<Vec<String>>,

}

impl cfn_resources::CfnResource for CfnProtection {
    fn type_string() -> &'static str {
        "AWS::Shield::Protection"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationLayerAutomaticResponseConfiguration {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-status
    #[serde(rename = "Status")]
    pub status: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html#cfn-shield-protection-applicationlayerautomaticresponseconfiguration-action
    #[serde(rename = "Action")]
    pub action: Action,

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


/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-block
    #[serde(rename = "Block")]
    pub block: Option<serde_json::Value>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html#cfn-shield-protection-action-count
    #[serde(rename = "Count")]
    pub count: Option<serde_json::Value>,

}
