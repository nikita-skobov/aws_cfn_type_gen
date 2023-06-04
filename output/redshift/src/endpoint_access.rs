/// Creates a Redshift-managed VPC endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccess {
    ///
    /// The cluster identifier of the cluster associated with the endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterIdentifier")]
    pub cluster_identifier: cfn_resources::StrVal,

    ///
    /// The name of the endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointName")]
    pub endpoint_name: cfn_resources::StrVal,

    ///
    /// The AWS account ID of the owner of the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<cfn_resources::StrVal>,

    ///
    /// The subnet group name where Amazon Redshift chooses to deploy the endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: cfn_resources::StrVal,

    ///
    /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,

    #[serde(skip_serializing)]
    pub att_address: CfnEndpointAccessaddress,

    #[serde(skip_serializing)]
    pub att_endpoint_create_time: CfnEndpointAccessendpointcreatetime,

    #[serde(skip_serializing)]
    pub att_endpoint_status: CfnEndpointAccessendpointstatus,

    #[serde(skip_serializing)]
    pub att_vpc_endpoint_vpc_endpoint_id: CfnEndpointAccessvpcendpointvpcendpointid,

    #[serde(skip_serializing)]
    pub att_vpc_endpoint_vpc_id: CfnEndpointAccessvpcendpointvpcid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccessaddress;
impl CfnEndpointAccessaddress {
    pub fn att_name(&self) -> &'static str {
        r#"Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccessendpointcreatetime;
impl CfnEndpointAccessendpointcreatetime {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointCreateTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccessendpointstatus;
impl CfnEndpointAccessendpointstatus {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccessvpcendpointvpcendpointid;
impl CfnEndpointAccessvpcendpointvpcendpointid {
    pub fn att_name(&self) -> &'static str {
        r#"VpcEndpoint.VpcEndpointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEndpointAccessvpcendpointvpcid;
impl CfnEndpointAccessvpcendpointvpcid {
    pub fn att_name(&self) -> &'static str {
        r#"VpcEndpoint.VpcId"#
    }
}

impl cfn_resources::CfnResource for CfnEndpointAccess {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::EndpointAccess"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cluster_identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", s.len()));
            }
        }

        let the_val = &self.endpoint_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 2147483647",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.resource_owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'resource_owner'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        let the_val = &self.subnet_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'subnet_group_name'. {} is greater than 2147483647", s.len()));
            }
        }

        Ok(())
    }
}

/// Describes a network interface.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NetworkInterface {
    ///
    /// The Availability Zone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// The network interface identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
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
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<cfn_resources::StrVal>,

    ///
    /// The subnet identifier.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
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
        if let Some(the_val) = &self.availability_zone {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'availability_zone'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.network_interface_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'network_interface_id'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.private_ip_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'private_ip_address'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subnet_id'. {} is greater than 2147483647",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The connection endpoint for connecting to an Amazon Redshift cluster through the proxy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    /// The connection endpoint ID for connecting an Amazon Redshift cluster through the proxy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<cfn_resources::StrVal>,

    ///
    /// The VPC identifier that the endpoint is associated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
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
        if let Some(the_val) = &self.vpc_endpoint_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'vpc_endpoint_id'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.vpc_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!(
                        "Max validation failed on field 'vpc_id'. {} is greater than 2147483647",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The security groups associated with the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct VpcSecurityGroup {
    ///
    /// The status of the endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// The identifier of the VPC security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VpcSecurityGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.status {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!(
                        "Max validation failed on field 'status'. {} is greater than 2147483647",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.vpc_security_group_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2147483647 as _ {
                    return Err(format!("Max validation failed on field 'vpc_security_group_id'. {} is greater than 2147483647", s.len()));
                }
            }
        }

        Ok(())
    }
}
