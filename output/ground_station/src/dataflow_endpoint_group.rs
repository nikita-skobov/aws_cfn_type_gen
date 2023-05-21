

/// Creates a Dataflow Endpoint Group request.
///
/// Dataflow endpoint groups contain a list of endpoints.       When the name of a dataflow endpoint group is specified in a mission profile, the Ground Station service will connect to the endpoints and flow data during a contact.
///
/// For more information about dataflow endpoint groups, see Dataflow Endpoint Groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataflowEndpointGroup {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactPostPassDurationSeconds")]
    pub contact_post_pass_duration_seconds: Option<i64>,


    /// 
    /// Tags assigned to a resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactPrePassDurationSeconds")]
    pub contact_pre_pass_duration_seconds: Option<i64>,


    /// 
    /// List of Endpoint Details, containing address and port for each endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of EndpointDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointDetails")]
    pub endpoint_details: Vec<EndpointDetails>,

}



impl cfn_resources::CfnResource for CfnDataflowEndpointGroup {
    fn type_string() -> &'static str {
        "AWS::GroundStation::DataflowEndpointGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The RangedConnectionDetails property type specifies Property description not available. for an AWS::GroundStation::DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RangedConnectionDetails {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mtu")]
    pub mtu: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: RangedSocketAddress
    ///
    /// Update requires: No interruption
    #[serde(rename = "SocketAddress")]
    pub socket_address: Option<RangedSocketAddress>,

}




/// Contains information such as socket address and name that defines an endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataflowEndpoint {


    /// 
    /// Maximum transmission unit (MTU) size in bytes of a dataflow endpoint.      Valid values are between 1400 and 1500. A default value of 1500 is used if not set.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mtu")]
    pub mtu: Option<i64>,


    /// 
    /// The address and port of an endpoint.
    /// 
    /// Required: No
    ///
    /// Type: SocketAddress
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<SocketAddress>,


    /// 
    /// The endpoint name.
    /// 
    /// When listing available contacts for a satellite, Ground Station searches for a dataflow endpoint whose name matches the value specified by the dataflow endpoint config of the selected mission profile. If no matching dataflow endpoints are found then Ground Station will not display any available contacts for the satellite.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}




/// The AwsGroundStationAgentEndpoint property type specifies Property description not available. for an AWS::GroundStation::DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsGroundStationAgentEndpoint {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ConnectionDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressAddress")]
    pub egress_address: Option<ConnectionDetails>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: RangedConnectionDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressAddress")]
    pub ingress_address: Option<RangedConnectionDetails>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditResults")]
    pub audit_results: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentStatus")]
    pub agent_status: Option<String>,

}




/// The security details and endpoint information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EndpointDetails {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AwsGroundStationAgentEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsGroundStationAgentEndpoint")]
    pub aws_ground_station_agent_endpoint: Option<AwsGroundStationAgentEndpoint>,


    /// 
    /// The role ARN, and IDs for security groups and subnets.
    /// 
    /// Required: No
    ///
    /// Type: SecurityDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityDetails")]
    pub security_details: Option<SecurityDetails>,


    /// 
    /// Information about the endpoint such as name and the endpoint address.
    /// 
    /// Required: No
    ///
    /// Type: DataflowEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<DataflowEndpoint>,

}




/// Information about IAM roles, subnets, and security groups needed for this DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecurityDetails {


    /// 
    /// The security group Ids of the security role, such as       sg-1234567890abcdef0.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The ARN of a role which Ground Station has permission to assume, such as       arn:aws:iam::1234567890:role/DataDeliveryServiceRole.
    /// 
    /// Ground Station will assume this role and create an ENI in your VPC on the specified subnet upon creation of a dataflow endpoint group. This ENI is used as the ingress/egress point for data streamed during a satellite contact.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The subnet Ids of the security details, such as       subnet-12345678.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}




/// The IntegerRange property type specifies Property description not available. for an AWS::GroundStation::DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IntegerRange {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Maximum")]
    pub maximum: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minimum")]
    pub minimum: Option<i64>,

}




/// The RangedSocketAddress property type specifies Property description not available. for an AWS::GroundStation::DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RangedSocketAddress {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: IntegerRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    pub port_range: Option<IntegerRange>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}




/// The address of the endpoint, such as       192.168.1.1.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SocketAddress {


    /// 
    /// The port of the endpoint, such as       55888.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The name of the endpoint, such as       Endpoint 1.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

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




/// The ConnectionDetails property type specifies Property description not available. for an AWS::GroundStation::DataflowEndpointGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionDetails {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: SocketAddress
    ///
    /// Update requires: No interruption
    #[serde(rename = "SocketAddress")]
    pub socket_address: Option<SocketAddress>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mtu")]
    pub mtu: Option<i64>,

}


