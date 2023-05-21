

/// The AWS::MediaConnect::FlowVpcInterface resource is a connection between your AWS Elemental MediaConnect flow and a virtual private cloud (VPC) that you       created using the Amazon Virtual Private Cloud service.
///
/// To avoid streaming your content over the public internet, you can add up to two VPC       interfaces to your flow and use those connections to transfer content between your VPC       and MediaConnect.
///
/// You can update an existing flow to add a VPC interface. If you haven’t created the       flow yet, you must create the flow with a temporary standard source by doing the       following:
#[derive(Default, serde::Serialize)]
pub struct CfnFlowVpcInterface {


    /// 
    /// The Amazon Resource Name (ARN) of the role that you created when you set up        MediaConnect as a trusted service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FlowArn")]
    pub flow_arn: String,


    /// 
    /// The name of the VPC Interface. This value must be unique within the current        flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The subnet IDs that you want to use for your VPC interface.
    /// 
    /// A range of IP addresses in your VPC. When you create your VPC, you specify a range        of IPv4 addresses for the VPC in the form of a Classless Inter-Domain Routing (CIDR)        block; for example, 10.0.0.0/16. This is the primary CIDR block for your VPC. When        you create a subnet for your VPC, you specify the CIDR block for the subnet, which is        a subset of the VPC CIDR block.
    /// 
    /// The subnets that you use across all VPC interfaces on the flow must be in the same        Availability Zone as the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The VPC security groups that you want MediaConnect to use for your VPC        configuration. You must include at least one security group in the request.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}
