/// The AWS::GlobalAccelerator::Listener resource is a Global Accelerator resource type that contains information about 	   		how you create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static 			IP addresses on a port, port range, or list of port ranges that you specify.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnListener {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_affinity: Option<ListenerClientAffinityEnum>,

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
    pub protocol: ListenerProtocolEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ListenerClientAffinityEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SOURCE_IP
    #[serde(rename = "SOURCE_IP")]
    Sourceip,
}

impl Default for ListenerClientAffinityEnum {
    fn default() -> Self {
        ListenerClientAffinityEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ListenerProtocolEnum {
    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

    /// UDP
    #[serde(rename = "UDP")]
    Udp,
}

impl Default for ListenerProtocolEnum {
    fn default() -> Self {
        ListenerProtocolEnum::Tcp
    }
}

impl cfn_resources::CfnResource for CfnListener {
    fn type_string(&self) -> &'static str {
        "AWS::GlobalAccelerator::Listener"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.accelerator_arn;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'accelerator_arn'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.port_ranges;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'port_ranges'. {} is greater than 10",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A complex type for a range of ports for a listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortRange {
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
}

impl cfn_resources::CfnResource for PortRange {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.from_port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'from_port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.from_port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'from_port'. {} is less than 1",
                the_val
            ));
        }

        let the_val = &self.to_port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'to_port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.to_port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'to_port'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}
