

/// Specifies a VPC endpoint for a service. An endpoint enables you to create a private     connection between your VPC and the service. The service may be provided by AWS,      an AWS Marketplace Partner, or another AWS account.     For more information, see the AWS PrivateLink User Guide.
///
/// An interface endpoint establishes connections between the subnets in your VPC and an     AWS service, your own service, or a service hosted by another AWS account.     You can specify the subnets in which to create the endpoint and the security groups to      associate with the endpoint network interface.
///
/// A gateway endpoint serves as a target for a route in your route table for traffic destined      for Amazon S3 or Amazon DynamoDB. You can specify an endpoint policy for the endpoint,      which controls access to the service from your VPC. You can also specify the VPC route tables      that use the endpoint. For information about connectivity to Amazon S3, see Why can’t I connect to an S3 bucket using a gateway VPC endpoint?
///
/// A Gateway Load Balancer endpoint provides private connectivity between your VPC and      virtual appliances from a service provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPCEndpoint {


    /// 
    /// The IDs of the security groups to associate with the endpoint network interfaces.      If this parameter is not specified, we use the default security group for the VPC.      Security groups are supported only for interface endpoints.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The IDs of the route tables. Routing is supported only for gateway endpoints.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Option<Vec<String>>,


    /// 
    /// The service name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceName")]
    pub service_name: String,


    /// 
    /// The ID of the VPC for the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


    /// 
    /// The type of endpoint.
    /// 
    /// Default: Gateway
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Gateway | GatewayLoadBalancer | Interface
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcEndpointType")]
    pub vpc_endpoint_type: Option<VPCEndpointVpcEndpointTypeEnum>,


    /// 
    /// A policy that controls access to the service from the VPC.      If this parameter is not specified, the default policy allows full access to the service.     Endpoint policies are supported only for gateway and interface endpoints.
    /// 
    /// For CloudFormation templates in YAML, you can provide the policy in JSON or YAML format.       AWS CloudFormation converts YAML policies to JSON format before calling the API to     create or modify the VPC endpoint.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<serde_json::Value>,


    /// 
    /// The IDs of the subnets in which to create endpoint network interfaces.      You must specify this property for an interface endpoint or a Gateway Load Balancer endpoint.      You can't specify this property for a gateway endpoint.      For a Gateway Load Balancer endpoint, you can specify only one subnet.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// Indicate whether to associate a private hosted zone with the specified VPC. The private hosted zone      contains a record set for the default public DNS name for the service for the Region (for example,       kinesis.us-east-1.amazonaws.com), which resolves to the private IP addresses     of the endpoint network interfaces in the VPC. This enables you to make requests to the     default public DNS name for the service instead of the public DNS names that are     automatically generated by the VPC endpoint service.
    /// 
    /// To use a private hosted zone, you must set the following VPC attributes to       true: enableDnsHostnames and     enableDnsSupport.
    /// 
    /// This property is supported only for interface endpoints.
    /// 
    /// Default: false
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateDnsEnabled")]
    pub private_dns_enabled: Option<bool>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum VPCEndpointVpcEndpointTypeEnum {

    /// Gateway
    #[serde(rename = "Gateway")]
    Gateway,

    /// GatewayLoadBalancer
    #[serde(rename = "GatewayLoadBalancer")]
    Gatewayloadbalancer,

    /// Interface
    #[serde(rename = "Interface")]
    Interface,

}

impl Default for VPCEndpointVpcEndpointTypeEnum {
    fn default() -> Self {
        VPCEndpointVpcEndpointTypeEnum::Gateway
    }
}


impl cfn_resources::CfnResource for CfnVPCEndpoint {
    fn type_string() -> &'static str {
        "AWS::EC2::VPCEndpoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
