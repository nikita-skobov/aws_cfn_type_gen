

/// Creates a node on the specified blockchain network.
///
/// Applies to Hyperledger Fabric and Ethereum.
#[derive(Default, serde::Serialize)]
pub struct CfnNode {


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


    /// 
    /// The unique identifier of the member to which the node belongs. Applies only to Hyperledger Fabric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberId")]
    pub member_id: String,


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
    pub network_id: String,

}


/// Configuration properties of a peer node within a membership.
#[derive(Default, serde::Serialize)]
pub struct NodeConfiguration {


    /// 
    /// The Amazon Managed Blockchain instance type for the node.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The Availability Zone in which the node exists. Required for Ethereum nodes.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,

}
