

/// The AWS::DataSync::Agent resource activates an AWS DataSync     agent that you've deployed for storage discovery or data transfers. The activation process     associates the agent with your AWS account.
///
/// For more information, see the following topics in the AWS DataSync       User Guide:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAgent {


    /// 
    /// The Amazon Resource Names (ARNs) of the security groups used to protect your data     transfer task subnets. See SecurityGroupArns.
    /// 
    /// Pattern:       ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Option<Vec<String>>,


    /// 
    /// The ID of the virtual private cloud (VPC) endpoint that the agent has access to. This is     the client-side VPC endpoint, powered by AWS PrivateLink. If you don't have an     AWS PrivateLink VPC endpoint, see AWS PrivateLink and VPC endpoints in the Amazon VPC User     Guide.
    /// 
    /// For more information about activating your agent in a private network based on a VPC,     see Using        AWS DataSync in a Virtual Private Cloud in the AWS DataSync User Guide.
    /// 
    /// A VPC endpoint ID looks like this: vpce-01234d5aff67890e1.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^vpce-[0-9a-f]{17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,


    /// 
    /// Specifies the ARN of the subnet where you want to run your DataSync task when    using a VPC endpoint. This is the subnet where DataSync creates and manages the     network     interfaces for your transfer.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetArns")]
    pub subnet_arns: Option<Vec<String>>,


    /// 
    /// Specifies your DataSync agent's activation key. If you don't have an    activation key, see Activate your agent.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 29
    ///
    /// Pattern: [A-Z0-9]{5}(-[A-Z0-9]{5}){4}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActivationKey")]
    pub activation_key: Option<String>,


    /// 
    /// Specifies a name for your agent. You can see this name in the DataSync    console.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9\s+=._:@/-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentName")]
    pub agent_name: Option<String>,


    /// 
    /// Specifies labels that help you categorize, filter, and search for your AWS resources.    We recommend creating at least one tag for your agent.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnAgent {
    fn type_string() -> &'static str {
        "AWS::DataSync::Agent"
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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


