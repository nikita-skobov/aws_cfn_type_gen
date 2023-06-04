/// Specifies a network insights analysis.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNetworkInsightsAnalysis {
    ///
    /// The member accounts that contain resources that the path can traverse.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_accounts: Option<Vec<String>>,

    ///
    /// The Amazon Resource Names (ARN) of the resources that the path must traverse.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterInArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_in_arns: Option<Vec<String>>,

    ///
    /// The ID of the path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInsightsPathId")]
    pub network_insights_path_id: cfn_resources::StrVal,

    ///
    /// The tags to apply.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_network_insights_analysis_arn: CfnNetworkInsightsAnalysisnetworkinsightsanalysisarn,

    #[serde(skip_serializing)]
    pub att_network_insights_analysis_id: CfnNetworkInsightsAnalysisnetworkinsightsanalysisid,

    #[serde(skip_serializing)]
    pub att_start_date: CfnNetworkInsightsAnalysisstartdate,

    #[serde(skip_serializing)]
    pub att_status: CfnNetworkInsightsAnalysisstatus,

    #[serde(skip_serializing)]
    pub att_status_message: CfnNetworkInsightsAnalysisstatusmessage,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAnalysisnetworkinsightsanalysisarn;
impl CfnNetworkInsightsAnalysisnetworkinsightsanalysisarn {
    pub fn att_name(&self) -> &'static str {
        r#"NetworkInsightsAnalysisArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAnalysisnetworkinsightsanalysisid;
impl CfnNetworkInsightsAnalysisnetworkinsightsanalysisid {
    pub fn att_name(&self) -> &'static str {
        r#"NetworkInsightsAnalysisId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAnalysisstartdate;
impl CfnNetworkInsightsAnalysisstartdate {
    pub fn att_name(&self) -> &'static str {
        r#"StartDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAnalysisstatus;
impl CfnNetworkInsightsAnalysisstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInsightsAnalysisstatusmessage;
impl CfnNetworkInsightsAnalysisstatusmessage {
    pub fn att_name(&self) -> &'static str {
        r#"StatusMessage"#
    }
}

impl cfn_resources::CfnResource for CfnNetworkInsightsAnalysis {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkInsightsAnalysis"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes an additional detail for a path analysis. For more information, see Reachability Analyzer additional detail codes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AdditionalDetail {
    ///
    /// The additional detail code.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalDetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_detail_type: Option<cfn_resources::StrVal>,

    ///
    /// The path component.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AnalysisComponent>,

    ///
    /// The load balancers.
    ///
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<AnalysisComponent>>,

    ///
    /// The name of the VPC endpoint service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AdditionalDetail {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.component
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an potential intermediate component of a feasible path.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AlternatePathHint {
    ///
    /// The Amazon Resource Name (ARN) of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AlternatePathHint {
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

/// Describes a network access control (ACL) rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisAclRule {
    ///
    /// The IPv4 address range, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether the rule is an outbound rule.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Egress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<bool>,

    ///
    /// The range of ports.
    ///
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,

    ///
    /// The protocol.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether to allow or deny traffic that matches the rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action: Option<cfn_resources::StrVal>,

    ///
    /// The rule number.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i64>,
}

impl cfn_resources::CfnResource for AnalysisAclRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.port_range
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a path component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisComponent {
    ///
    /// The Amazon Resource Name (ARN) of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AnalysisComponent {
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

/// Describes a load balancer listener.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisLoadBalancerListener {
    ///
    /// [Classic Load Balancers] The back-end port for the listener.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i64>,

    ///
    /// The port on which the load balancer is listening.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_port: Option<i64>,
}

impl cfn_resources::CfnResource for AnalysisLoadBalancerListener {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.instance_port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.instance_port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_port'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.load_balancer_port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'load_balancer_port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.load_balancer_port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'load_balancer_port'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Describes a load balancer target.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisLoadBalancerTarget {
    ///
    /// The IP address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<cfn_resources::StrVal>,

    ///
    /// The Availability Zone.
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
    /// Information about the instance.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<AnalysisComponent>,

    ///
    /// The port on which the target is listening.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

impl cfn_resources::CfnResource for AnalysisLoadBalancerTarget {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 15 as _ {
                    return Err(format!(
                        "Max validation failed on field 'address'. {} is greater than 15",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'address'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.instance
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Describes a header. Reflects any changes made by a component as traffic passes through.     The fields of an inbound header are null except for the first component of a path.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisPacketHeader {
    ///
    /// The destination addresses.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_addresses: Option<Vec<String>>,

    ///
    /// The destination port ranges.
    ///
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_ranges: Option<Vec<PortRange>>,

    ///
    /// The protocol.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// The source addresses.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_addresses: Option<Vec<String>>,

    ///
    /// The source port ranges.
    ///
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_ranges: Option<Vec<PortRange>>,
}

impl cfn_resources::CfnResource for AnalysisPacketHeader {
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

/// Describes a route table route.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisRouteTableRoute {
    ///
    /// The ID of a NAT gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a network interface.
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
    /// Describes how the route was created. The following are the possible values:
    ///
    /// CreateRouteTable - The route was automatically created when the route table was created.               CreateRoute - The route was manually added to the route table.               EnableVgwRoutePropagation - The route was propagated by route propagation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<cfn_resources::StrVal>,

    ///
    /// The state. The following are the possible values:
    ///
    /// active               blackhole
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a transit gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a VPC peering connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<cfn_resources::StrVal>,

    ///
    /// The destination IPv4 address, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "destinationCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr: Option<cfn_resources::StrVal>,

    ///
    /// The prefix of the AWS service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "destinationPrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix_list_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of an egress-only internet gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "egressOnlyInternetGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_only_internet_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the gateway, such as an internet gateway or virtual private gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the instance, such as a NAT instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AnalysisRouteTableRoute {
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

/// Describes a security group rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AnalysisSecurityGroupRule {
    ///
    /// The IPv4 address range, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<cfn_resources::StrVal>,

    ///
    /// The direction. The following are the possible values:
    ///
    /// egress               ingress
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<cfn_resources::StrVal>,

    ///
    /// The port range.
    ///
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,

    ///
    /// The prefix list ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<cfn_resources::StrVal>,

    ///
    /// The protocol name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// The security group ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AnalysisSecurityGroupRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.port_range
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an explanation code for an unreachable path. For more information, see Reachability Analyzer explanation codes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Explanation {
    ///
    /// The network ACL.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Acl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<AnalysisComponent>,

    ///
    /// The network ACL rule.
    ///
    /// Required: No
    ///
    /// Type: AnalysisAclRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_rule: Option<AnalysisAclRule>,

    ///
    /// The IPv4 address, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<cfn_resources::StrVal>,

    ///
    /// The IPv4 addresses, in CIDR notation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,

    ///
    /// The resource to which the component is attached.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<AnalysisComponent>,

    ///
    /// The Availability Zones.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// The CIDR ranges.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,

    ///
    /// The listener for a Classic Load Balancer.
    ///
    /// Required: No
    ///
    /// Type: AnalysisLoadBalancerListener
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClassicLoadBalancerListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_load_balancer_listener: Option<AnalysisLoadBalancerListener>,

    ///
    /// The component.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AnalysisComponent>,

    ///
    /// The AWS account for the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \d{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_account: Option<cfn_resources::StrVal>,

    ///
    /// The Region for the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [a-z]{2}-[a-z]+-[1-9]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_region: Option<cfn_resources::StrVal>,

    ///
    /// The customer gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway: Option<AnalysisComponent>,

    ///
    /// The destination.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<AnalysisComponent>,

    ///
    /// The destination VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationVpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_vpc: Option<AnalysisComponent>,

    ///
    /// The direction. The following are the possible values:
    ///
    /// egress               ingress
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<cfn_resources::StrVal>,

    ///
    /// The load balancer listener.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticLoadBalancerListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,

    ///
    /// The explanation code.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplanationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_code: Option<cfn_resources::StrVal>,

    ///
    /// The route table.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressRouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_route_table: Option<AnalysisComponent>,

    ///
    /// The internet gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternetGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_gateway: Option<AnalysisComponent>,

    ///
    /// The Amazon Resource Name (ARN) of the load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1283
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<cfn_resources::StrVal>,

    ///
    /// The listener port of the load balancer.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerListenerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_listener_port: Option<i64>,

    ///
    /// The target.
    ///
    /// Required: No
    ///
    /// Type: AnalysisLoadBalancerTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target: Option<AnalysisLoadBalancerTarget>,

    ///
    /// The target group.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target_group: Option<AnalysisComponent>,

    ///
    /// The target groups.
    ///
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target_groups: Option<Vec<AnalysisComponent>>,

    ///
    /// The target port.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_target_port: Option<i64>,

    ///
    /// The missing component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MissingComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_component: Option<cfn_resources::StrVal>,

    ///
    /// The NAT gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway: Option<AnalysisComponent>,

    ///
    /// The network interface.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<AnalysisComponent>,

    ///
    /// The packet field.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PacketField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_field: Option<cfn_resources::StrVal>,

    ///
    /// The port.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The port ranges.
    ///
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,

    ///
    /// The prefix list.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list: Option<AnalysisComponent>,

    ///
    /// The protocols.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    ///
    /// The route table.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table: Option<AnalysisComponent>,

    ///
    /// The route table route.
    ///
    /// Required: No
    ///
    /// Type: AnalysisRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,

    ///
    /// The security group.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<AnalysisComponent>,

    ///
    /// The security group rule.
    ///
    /// Required: No
    ///
    /// Type: AnalysisSecurityGroupRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,

    ///
    /// The security groups.
    ///
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<AnalysisComponent>>,

    ///
    /// The source VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_vpc: Option<AnalysisComponent>,

    ///
    /// The state.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,

    ///
    /// The subnet.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<AnalysisComponent>,

    ///
    /// The route table for the subnet.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetRouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_route_table: Option<AnalysisComponent>,

    ///
    /// The transit gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway: Option<AnalysisComponent>,

    ///
    /// The transit gateway attachment.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment: Option<AnalysisComponent>,

    ///
    /// The transit gateway route table.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table: Option<AnalysisComponent>,

    ///
    /// The transit gateway route table route.
    ///
    /// Required: No
    ///
    /// Type: TransitGatewayRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,

    ///
    /// The component VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<AnalysisComponent>,

    ///
    /// The VPC peering connection.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection: Option<AnalysisComponent>,

    ///
    /// The VPN connection.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_connection: Option<AnalysisComponent>,

    ///
    /// The VPN gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_gateway: Option<AnalysisComponent>,

    ///
    /// The VPC endpoint.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "vpcEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<AnalysisComponent>,
}

impl cfn_resources::CfnResource for Explanation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acl.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.acl_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 15 as _ {
                    return Err(format!(
                        "Max validation failed on field 'address'. {} is greater than 15",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'address'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.attached_to
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.classic_load_balancer_listener
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.component
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.customer_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination_vpc
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.elastic_load_balancer_listener
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ingress_route_table
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.internet_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.load_balancer_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1283 as _ {
                    return Err(format!("Max validation failed on field 'load_balancer_arn'. {} is greater than 1283", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.load_balancer_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'load_balancer_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.load_balancer_listener_port {
            if *the_val > 65535 as _ {
                return Err(format!("Max validation failed on field 'load_balancer_listener_port'. {} is greater than 65535", the_val));
            }
        }

        if let Some(the_val) = &self.load_balancer_listener_port {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'load_balancer_listener_port'. {} is less than 0", the_val));
            }
        }

        self.load_balancer_target
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.load_balancer_target_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.load_balancer_target_port {
            if *the_val > 65535 as _ {
                return Err(format!("Max validation failed on field 'load_balancer_target_port'. {} is greater than 65535", the_val));
            }
        }

        if let Some(the_val) = &self.load_balancer_target_port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'load_balancer_target_port'. {} is less than 0",
                    the_val
                ));
            }
        }

        self.nat_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_interface
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 0",
                    the_val
                ));
            }
        }

        self.prefix_list
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.route_table
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.route_table_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.security_group
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.security_group_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.source_vpc
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.subnet.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.subnet_route_table
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway_attachment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway_route_table
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway_route_table_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.vpc_peering_connection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpn_connection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpn_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc_endpoint
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a path component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathComponent {
    ///
    /// The network ACL rule.
    ///
    /// Required: No
    ///
    /// Type: AnalysisAclRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_rule: Option<AnalysisAclRule>,

    ///
    /// The additional details.
    ///
    /// Required: No
    ///
    /// Type: List of AdditionalDetail
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<Vec<AdditionalDetail>>,

    ///
    /// The component.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AnalysisComponent>,

    ///
    /// The destination VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationVpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_vpc: Option<AnalysisComponent>,

    ///
    /// The load balancer listener.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticLoadBalancerListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,

    ///
    /// The explanation codes.
    ///
    /// Required: No
    ///
    /// Type: List of Explanation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Explanations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<Explanation>>,

    ///
    /// The inbound header.
    ///
    /// Required: No
    ///
    /// Type: AnalysisPacketHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "InboundHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_header: Option<AnalysisPacketHeader>,

    ///
    /// The outbound header.
    ///
    /// Required: No
    ///
    /// Type: AnalysisPacketHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutboundHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_header: Option<AnalysisPacketHeader>,

    ///
    /// The route table route.
    ///
    /// Required: No
    ///
    /// Type: AnalysisRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,

    ///
    /// The security group rule.
    ///
    /// Required: No
    ///
    /// Type: AnalysisSecurityGroupRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,

    ///
    /// The sequence number.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,

    ///
    /// The name of the VPC endpoint service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<cfn_resources::StrVal>,

    ///
    /// The source VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_vpc: Option<AnalysisComponent>,

    ///
    /// The subnet.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<AnalysisComponent>,

    ///
    /// The transit gateway.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway: Option<AnalysisComponent>,

    ///
    /// The route in a transit gateway route table.
    ///
    /// Required: No
    ///
    /// Type: TransitGatewayRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,

    ///
    /// The component VPC.
    ///
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<AnalysisComponent>,
}

impl cfn_resources::CfnResource for PathComponent {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acl_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.component
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.destination_vpc
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.elastic_load_balancer_listener
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.inbound_header
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.outbound_header
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.route_table_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.security_group_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.source_vpc
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.subnet.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.transit_gateway_route_table_route
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vpc.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a range of ports.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PortRange {
    ///
    /// The first port in the range.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    ///
    /// The last port in the range.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl cfn_resources::CfnResource for PortRange {
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

/// Describes a route in a transit gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TransitGatewayRouteTableRoute {
    ///
    /// The ID of the route attachment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<cfn_resources::StrVal>,

    ///
    /// The CIDR block used for destination matches.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the prefix list.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the resource for the route attachment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<cfn_resources::StrVal>,

    ///
    /// The resource type for the route attachment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<cfn_resources::StrVal>,

    ///
    /// The route origin. The following are the possible values:
    ///
    /// static               propagated
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_origin: Option<cfn_resources::StrVal>,

    ///
    /// The state of the route.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TransitGatewayRouteTableRoute {
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
