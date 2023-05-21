/// Creates a node on the specified blockchain network.
///
/// Applies to Hyperledger Fabric and Ethereum.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNode {
    ///
    /// The unique identifier of the member to which the node belongs. Applies only to Hyperledger Fabric.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberId")]
    pub member_id: cfn_resources::StrVal,

    ///
    /// The unique identifier of the network for the node.
    ///
    /// Ethereum public networks have the following NetworkIds:
    ///
    /// n-ethereum-mainnet                                n-ethereum-goerli                                n-ethereum-rinkeby
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkId")]
    pub network_id: cfn_resources::StrVal,

    ///
    /// Configuration properties of a peer node.
    ///
    /// Required: Yes
    ///
    /// Type: NodeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeConfiguration")]
    pub node_configuration: NodeConfiguration,
}

impl cfn_resources::CfnResource for CfnNode {
    fn type_string(&self) -> &'static str {
        "AWS::ManagedBlockchain::Node"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.network_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'network_id'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.network_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'network_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.node_configuration.validate()?;

        Ok(())
    }
}

/// Configuration properties of a peer node within a membership.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeConfiguration {
    ///
    /// The Availability Zone in which the node exists. Required for Ethereum nodes.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: cfn_resources::StrVal,

    ///
    /// The Amazon Managed Blockchain instance type for the node.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NodeConfiguration {
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
