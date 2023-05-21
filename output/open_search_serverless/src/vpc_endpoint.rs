

/// Creates an OpenSearch Serverless-managed interface VPC endpoint. For more information, see Access         Amazon OpenSearch Serverless using an interface endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVpcEndpoint {


    /// 
    /// The name of the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The unique identifiers of the security groups that define the ports, protocols, and       sources for inbound traffic that you are authorizing into your endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The ID of the subnets from which you access OpenSearch Serverless.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The ID of the VPC from which you access OpenSearch Serverless.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}



impl cfn_resources::CfnResource for CfnVpcEndpoint {
    fn type_string() -> &'static str {
        "AWS::OpenSearchServerless::VpcEndpoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}