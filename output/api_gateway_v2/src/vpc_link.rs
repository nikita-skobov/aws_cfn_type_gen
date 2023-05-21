

/// The AWS::ApiGatewayV2::VpcLink resource creates a VPC link. Supported only for HTTP APIs. The VPC link status must transition          from PENDING to AVAILABLE to successfully create a VPC link, which can take up to 10 minutes. To learn more, see             Working with VPC Links for HTTP APIs          in the API Gateway Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnVpcLink {


    /// 
    /// The collection of tags. Each tag element is associated with a given resource.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


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
    pub security_group_ids: Option<Vec<String>>,

}