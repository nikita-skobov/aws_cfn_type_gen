/// Creates a Redshift-managed VPC endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub cluster_identifier: String,

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
    pub endpoint_name: String,

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
    pub resource_owner: Option<String>,

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
    pub subnet_group_name: String,

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
}

impl cfn_resources::CfnResource for CfnEndpointAccess {
    fn type_string(&self) -> &'static str {
        "AWS::Redshift::EndpointAccess"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cluster_identifier;

        if the_val.len() > 2147483647 as _ {
            return Err(format!("Max validation failed on field 'cluster_identifier'. {} is greater than 2147483647", the_val.len()));
        }

        let the_val = &self.endpoint_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'endpoint_name'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.resource_owner {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'resource_owner'. {} is greater than 2147483647", the_val.len()));
            }
        }

        let the_val = &self.subnet_group_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'subnet_group_name'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Describes a network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub availability_zone: Option<String>,

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
    pub network_interface_id: Option<String>,

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
    pub private_ip_address: Option<String>,

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
    pub subnet_id: Option<String>,
}

impl cfn_resources::CfnResource for NetworkInterface {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.availability_zone {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'availability_zone'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.network_interface_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'network_interface_id'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.private_ip_address {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'private_ip_address'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_id'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The connection endpoint for connecting to an Amazon Redshift cluster through the proxy.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub vpc_endpoint_id: Option<String>,

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
    pub vpc_id: Option<String>,
}

impl cfn_resources::CfnResource for VpcEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.vpc_endpoint_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'vpc_endpoint_id'. {} is greater than 2147483647", the_val.len()));
            }
        }

        if let Some(the_val) = &self.vpc_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpc_id'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The security groups associated with the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub status: Option<String>,

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
    pub vpc_security_group_id: Option<String>,
}

impl cfn_resources::CfnResource for VpcSecurityGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.status {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'status'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vpc_security_group_id {
            if the_val.len() > 2147483647 as _ {
                return Err(format!("Max validation failed on field 'vpc_security_group_id'. {} is greater than 2147483647", the_val.len()));
            }
        }

        Ok(())
    }
}
