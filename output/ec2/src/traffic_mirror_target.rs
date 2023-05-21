

/// Specifies a target for your Traffic Mirror session.
///
/// A Traffic Mirror target is the destination for mirrored traffic. The Traffic Mirror     source and the Traffic Mirror target (monitoring appliances) can be in the same VPC, or in     different VPCs connected via VPC peering or a transit gateway.
///
/// A Traffic Mirror target can be a network interface, a Network Load Balancer, or a Gateway Load Balancer endpoint.
///
/// To use the target in a Traffic Mirror session, use AWS::EC2::TrafficMirrorSession.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrafficMirrorTarget {


    /// 
    /// The description of the Traffic Mirror target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ID of the Gateway Load Balancer endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GatewayLoadBalancerEndpointId")]
    pub gateway_load_balancer_endpoint_id: Option<String>,


    /// 
    /// The network interface ID that is associated with the target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkLoadBalancerArn")]
    pub network_load_balancer_arn: Option<String>,


    /// 
    /// The tags to assign to the Traffic Mirror target.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnTrafficMirrorTarget {
    fn type_string() -> &'static str {
        "AWS::EC2::TrafficMirrorTarget"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


