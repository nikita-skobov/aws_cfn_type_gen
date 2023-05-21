

/// The collection of compute resources in Amazon Redshift Serverless.
#[derive(Default, serde::Serialize)]
pub struct CfnWorkgroup {


    /// 
    /// The custom port to use when connecting to a workgroup. Valid port ranges are 5431-5455 and 8191-8215.    The default is 5439.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The base compute capacity of the workgroup in Redshift Processing Units (RPUs).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseCapacity")]
    pub base_capacity: Option<i64>,


    /// 
    /// The value that specifies whether to enable enhanced virtual   private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedVpcRouting")]
    pub enhanced_vpc_routing: Option<bool>,


    /// 
    /// The map of the key-value pairs used to tag the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A list of subnet IDs the workgroup is associated with.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The name of the workgroup.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: String,


    /// 
    /// The namespace the workgroup is associated with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,


    /// 
    /// A value that specifies whether the workgroup   can be accessible from a public network.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,


    /// 
    /// A list of parameters to set for finer control over a database. Available   options are datestyle, enable_user_activity_logging,   query_group, search_path, and max_query_execution_time.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfigParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigParameters")]
    pub config_parameters: Option<Vec<ConfigParameter>>,


    /// 
    /// A list of security group IDs to associate with the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// The collection of computing resources from which an endpoint is created.
#[derive(Default, serde::Serialize)]
pub struct Workgroup {


    /// 
    /// The namespace the workgroup is associated with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,


    /// 
    /// An array of subnet IDs the workgroup is associated with.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The creation date of the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) that links to the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupArn")]
    pub workgroup_arn: Option<String>,


    /// 
    /// An array of security group IDs to associate with the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The endpoint that is created from the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: Endpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,


    /// 
    /// An array of parameters to set for advanced control over a database. The     options are auto_mv, datestyle, enable_case_sensitivity_identifier, enable_user_activity_logging,     query_group, , search_path, and query monitoring metrics that let you define performance boundaries.     For more information about query monitoring rules and available metrics, see Query monitoring metrics for Amazon Redshift Serverless.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfigParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigParameters")]
    pub config_parameters: Option<Vec<ConfigParameter>>,


    /// 
    /// The unique identifier of the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupId")]
    pub workgroup_id: Option<String>,


    /// 
    /// The status of the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// A value that specifies whether the workgroup    can be accessible from a public network
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,


    /// 
    /// The value that specifies whether to enable enhanced virtual    private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedVpcRouting")]
    pub enhanced_vpc_routing: Option<bool>,


    /// 
    /// The base data warehouse capacity of the workgroup in Redshift Processing Units (RPUs).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseCapacity")]
    pub base_capacity: Option<i64>,


    /// 
    /// The name of the workgroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: Option<String>,

}


/// A array of parameters to set for more control over a serverless database.
#[derive(Default, serde::Serialize)]
pub struct ConfigParameter {


    /// 
    /// The value of the parameter to set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: Option<String>,


    /// 
    /// The key of the parameter. The   options are datestyle, enable_user_activity_logging,   query_group, search_path, and max_query_execution_time.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterKey")]
    pub parameter_key: Option<String>,

}


/// Contains information about a network interface      in an Amazon Redshift Serverless managed VPC endpoint.
#[derive(Default, serde::Serialize)]
pub struct NetworkInterface {


    /// 
    /// The unique identifier of the subnet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,


    /// 
    /// The IPv4 address of the network interface within the subnet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,


    /// 
    /// The unique identifier of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The availability Zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

}


/// The VPC endpoint object.
#[derive(Default, serde::Serialize)]
pub struct Endpoint {


    /// 
    /// The port that Amazon Redshift Serverless listens on.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// An array of VpcEndpoint objects.
    /// 
    /// Required: No
    ///
    /// Type: List of VpcEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpoints")]
    pub vpc_endpoints: Option<Vec<VpcEndpoint>>,


    /// 
    /// The DNS address of the VPC endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,

}


/// The connection endpoint for connecting to Amazon Redshift Serverless through the proxy.
#[derive(Default, serde::Serialize)]
pub struct VpcEndpoint {


    /// 
    /// One or more network interfaces of the endpoint. Also known as an interface endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of NetworkInterface
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,


    /// 
    /// The VPC identifier that the endpoint is associated with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,


    /// 
    /// The connection endpoint ID for connecting to Amazon Redshift Serverless.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,

}
