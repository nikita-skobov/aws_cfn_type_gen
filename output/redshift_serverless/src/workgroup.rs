/// The collection of compute resources in Amazon Redshift Serverless.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnWorkgroup {
    ///
    /// The base compute capacity of the workgroup in Redshift Processing Units (RPUs).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_capacity: Option<i64>,

    ///
    /// A list of parameters to set for finer control over a database. Available   options are datestyle, enable_user_activity_logging,   query_group, search_path, and max_query_execution_time.
    ///
    /// Required: No
    ///
    /// Type: List of ConfigParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_parameters: Option<Vec<ConfigParameter>>,

    ///
    /// The value that specifies whether to enable enhanced virtual   private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,

    ///
    /// The namespace the workgroup is associated with.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<cfn_resources::StrVal>,

    ///
    /// The custom port to use when connecting to a workgroup. Valid port ranges are 5431-5455 and 8191-8215.    The default is 5439.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// A value that specifies whether the workgroup   can be accessible from a public network.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    ///
    /// A list of security group IDs to associate with the workgroup.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of subnet IDs the workgroup is associated with.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// The map of the key-value pairs used to tag the workgroup.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The name of the workgroup.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkgroupName")]
    pub workgroup_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_workgroup_creation_date: CfnWorkgroupworkgroupcreationdate,

    #[serde(skip_serializing)]
    pub att_workgroup_endpoint_address: CfnWorkgroupworkgroupendpointaddress,

    #[serde(skip_serializing)]
    pub att_workgroup_namespace_name: CfnWorkgroupworkgroupnamespacename,

    #[serde(skip_serializing)]
    pub att_workgroup_status: CfnWorkgroupworkgroupstatus,

    #[serde(skip_serializing)]
    pub att_workgroup_workgroup_arn: CfnWorkgroupworkgroupworkgrouparn,

    #[serde(skip_serializing)]
    pub att_workgroup_workgroup_id: CfnWorkgroupworkgroupworkgroupid,

    #[serde(skip_serializing)]
    pub att_workgroup_workgroup_name: CfnWorkgroupworkgroupworkgroupname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupcreationdate;
impl CfnWorkgroupworkgroupcreationdate {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.CreationDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupendpointaddress;
impl CfnWorkgroupworkgroupendpointaddress {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.Endpoint.Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupnamespacename;
impl CfnWorkgroupworkgroupnamespacename {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.NamespaceName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupstatus;
impl CfnWorkgroupworkgroupstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupworkgrouparn;
impl CfnWorkgroupworkgroupworkgrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.WorkgroupArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupworkgroupid;
impl CfnWorkgroupworkgroupworkgroupid {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.WorkgroupId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnWorkgroupworkgroupworkgroupname;
impl CfnWorkgroupworkgroupworkgroupname {
    pub fn att_name(&self) -> &'static str {
        r#"Workgroup.WorkgroupName"#
    }
}

impl cfn_resources::CfnResource for CfnWorkgroup {
    fn type_string(&self) -> &'static str {
        "AWS::RedshiftServerless::Workgroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A array of parameters to set for more control over a serverless database.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigParameter {
    ///
    /// The key of the parameter. The   options are datestyle, enable_user_activity_logging,   query_group, search_path, and max_query_execution_time.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<cfn_resources::StrVal>,

    ///
    /// The value of the parameter to set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ConfigParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The VPC endpoint object.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Endpoint {
    ///
    /// The DNS address of the VPC endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<cfn_resources::StrVal>,

    ///
    /// The port that Amazon Redshift Serverless listens on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<Vec<VpcEndpoint>>,
}

impl cfn_resources::CfnResource for Endpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Contains information about a network interface      in an Amazon Redshift Serverless managed VPC endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkInterface {
    ///
    /// The availability Zone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The unique identifier of the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<cfn_resources::StrVal>,

    ///
    /// The IPv4 address of the network interface within the subnet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<cfn_resources::StrVal>,

    ///
    /// The unique identifier of the subnet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NetworkInterface {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The connection endpoint for connecting to Amazon Redshift Serverless through the proxy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,

    ///
    /// The connection endpoint ID for connecting to Amazon Redshift Serverless.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<cfn_resources::StrVal>,

    ///
    /// The VPC identifier that the endpoint is associated with.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpcEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The collection of computing resources from which an endpoint is created.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Workgroup {
    ///
    /// The base data warehouse capacity of the workgroup in Redshift Processing Units (RPUs).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_capacity: Option<i64>,

    ///
    /// An array of parameters to set for advanced control over a database. The     options are auto_mv, datestyle, enable_case_sensitivity_identifier, enable_user_activity_logging,     query_group, , search_path, and query monitoring metrics that let you define performance boundaries.     For more information about query monitoring rules and available metrics, see Query monitoring metrics for Amazon Redshift Serverless.
    ///
    /// Required: No
    ///
    /// Type: List of ConfigParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfigParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_parameters: Option<Vec<ConfigParameter>>,

    ///
    /// The creation date of the workgroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<cfn_resources::StrVal>,

    ///
    /// The endpoint that is created from the workgroup.
    ///
    /// Required: No
    ///
    /// Type: Endpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,

    ///
    /// The value that specifies whether to enable enhanced virtual    private cloud (VPC) routing, which forces Amazon Redshift Serverless to route traffic through your VPC.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,

    ///
    /// The namespace the workgroup is associated with.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<cfn_resources::StrVal>,

    ///
    /// A value that specifies whether the workgroup    can be accessible from a public network
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,

    ///
    /// An array of security group IDs to associate with the workgroup.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The status of the workgroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// An array of subnet IDs the workgroup is associated with.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// The Amazon Resource Name (ARN) that links to the workgroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_arn: Option<cfn_resources::StrVal>,

    ///
    /// The unique identifier of the workgroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the workgroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkgroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Workgroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
