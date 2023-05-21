

/// The AWS::GlobalAccelerator::Listener resource is a Global Accelerator resource type that contains information about 	   		how you create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static 			IP addresses on a port, port range, or list of port ranges that you specify.
#[derive(Default, serde::Serialize)]
pub struct CfnListener {


    /// 
    /// The protocol for the connections from clients to the accelerator.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: TCP | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: String,


    /// 
    /// The Amazon Resource Name (ARN) of your accelerator.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,


    /// 
    /// Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, 			regardless of the port and protocol of the client request. Client affinity gives you control over whether to always 			route each client to the same specific endpoint.
    /// 
    /// AWS Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client 			affinity is NONE, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, 			destination IP address, destination port, and protocol—to select the hash value, and then chooses the best 			endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not 			be always routed to the same endpoint because the hash value changes.
    /// 
    /// If you want a given client to always be routed to the same endpoint, set client affinity to SOURCE_IP 			instead. When you use the SOURCE_IP setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— 			source (client) IP address and destination IP address—to select the hash value.
    /// 
    /// The default value is NONE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SOURCE_IP
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientAffinity")]
    pub client_affinity: Option<String>,


    /// 
    /// The list of port ranges for the connections from clients to the accelerator.
    /// 
    /// Required: Yes
    ///
    /// Type: List of PortRange
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRanges")]
    pub port_ranges: Vec<PortRange>,

}


/// A complex type for a range of ports for a listener.
#[derive(Default, serde::Serialize)]
pub struct PortRange {


    /// 
    /// The last port in the range of ports, inclusive.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    pub to_port: i64,


    /// 
    /// The first port in the range of ports, inclusive.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    pub from_port: i64,

}
