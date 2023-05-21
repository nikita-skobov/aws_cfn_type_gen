/// The AWS::ApiGatewayV2::VpcLink resource creates a VPC link. Supported only for HTTP APIs. The VPC link status must transition          from PENDING to AVAILABLE to successfully create a VPC link, which can take up to 10 minutes. To learn more, see             Working with VPC Links for HTTP APIs          in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVpcLink {
    ///
    /// The name of the VPC link.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// A list of security group IDs for the VPC link.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of subnet IDs to include in the VPC link.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

    ///
    /// The collection of tags. Each tag element is associated with a given resource.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnVpcLink {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::VpcLink"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
