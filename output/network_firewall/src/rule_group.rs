

/// Use the AWS::NetworkFirewall::RuleGroup to define a reusable collection of stateless or stateful network traffic filtering rules.        You use rule groups in an AWS::NetworkFirewall::FirewallPolicy to specify the filtering behavior of an AWS::NetworkFirewall::Firewall.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRuleGroup {


    /// 
    /// The maximum operating resources that this rule group can use. You can't change a rule group's capacity setting         after you create the rule group. When you update a rule group, you are limited to this capacity. When you reference a rule group    from a firewall policy, Network Firewall reserves this capacity for the rule group.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Capacity")]
    pub capacity: i64,


    /// 
    /// A description of the rule group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// An object that defines the rule group rules.
    /// 
    /// Required: No
    ///
    /// Type: RuleGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleGroup")]
    pub rule_group: Option<Box<RuleGroup>>,


    /// 
    /// The descriptive name of the rule group. You can't change the name of a rule group after you create it.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleGroupName")]
    pub rule_group_name: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: STATEFUL | STATELESS
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: RuleGroupTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RuleGroupTypeEnum {

    /// STATEFUL
    #[serde(rename = "STATEFUL")]
    Stateful,

    /// STATELESS
    #[serde(rename = "STATELESS")]
    Stateless,

}

impl Default for RuleGroupTypeEnum {
    fn default() -> Self {
        RuleGroupTypeEnum::Stateful
    }
}


impl cfn_resources::CfnResource for CfnRuleGroup {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkFirewall::RuleGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 512 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 512", the_val.len()));
        }

        }
        
        self.rule_group.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.rule_group_name;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'rule_group_name'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.rule_group_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'rule_group_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 200 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 200", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A custom action to use in stateless rule actions settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ActionDefinition {


    /// 
    /// Stateless inspection criteria that publishes the specified metrics to Amazon CloudWatch for the     matching packet. This setting defines a CloudWatch dimension value to be published.
    /// 
    /// You can pair this custom action with any of the standard stateless rule actions. For     example, you could pair this in a rule action with the standard action that forwards the     packet for stateful inspection. Then, when a packet matches the rule, Network Firewall     publishes metrics for the packet and forwards it.
    /// 
    /// Required: No
    ///
    /// Type: PublishMetricAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublishMetricAction")]
    pub publish_metric_action: Option<PublishMetricAction>,

}



impl cfn_resources::CfnResource for ActionDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.publish_metric_action.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A single IP address specification. This is used in the AWS::NetworkFirewall::RuleGroup MatchAttributes     source and destination specifications.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Address {


    /// 
    /// Specify an IP address or a block of IP addresses in Classless Inter-Domain Routing (CIDR) notation. Network Firewall supports all address ranges for IPv4 and IPv6.
    /// 
    /// Examples:
    /// 
    /// To configure Network Firewall to inspect for the IP address 192.0.2.44, specify 192.0.2.44/32.               To configure Network Firewall to inspect for IP addresses from 192.0.2.0 to 192.0.2.255, specify 192.0.2.0/24.               To configure Network Firewall to inspect for the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify 1111:0000:0000:0000:0000:0000:0000:0111/128.               To configure Network Firewall to inspect for IP addresses from 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify 1111:0000:0000:0000:0000:0000:0000:0000/64.
    /// 
    /// For more information about CIDR notation, see the Wikipedia entry Classless     Inter-Domain Routing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^([a-fA-F\d:\.]+($|/\d{1,3}))$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddressDefinition")]
    pub address_definition: String,

}



impl cfn_resources::CfnResource for Address {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.address_definition;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'address_definition'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.address_definition;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'address_definition'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// An optional, non-standard action to use for stateless packet handling. You can define     this in addition to the standard action that you must specify.
///
/// You define and name the custom actions that you want to be able to use, and then you     reference them by name in your actions settings.
///
/// You can use custom actions in the following places:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomAction {


    /// 
    /// The custom action associated with the action name.
    /// 
    /// Required: Yes
    ///
    /// Type: ActionDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionDefinition")]
    pub action_definition: ActionDefinition,


    /// 
    /// The descriptive name of the custom action. You can't change the name of a custom action after you create it.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionName")]
    pub action_name: String,

}



impl cfn_resources::CfnResource for CustomAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.action_definition.validate()?;

        let the_val = &self.action_name;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'action_name'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.action_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'action_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The value to use in an Amazon CloudWatch custom metric dimension. This is used in the       PublishMetrics custom action. A CloudWatch custom metric dimension is a name/value pair that's     part of the identity of a metric.
///
/// AWS Network Firewall sets the dimension name to CustomAction and you provide the     dimension value.
///
/// For more information about CloudWatch custom metric dimensions, see      Publishing Custom Metrics in the Amazon CloudWatch User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Dimension {


    /// 
    /// The value to use in the custom metric dimension.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9-_ ]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Dimension {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.value;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'value'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'value'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The 5-tuple criteria for AWS Network Firewall to use to inspect packet headers in stateful     traffic flow inspection. Traffic flows that match the criteria are a match for the     corresponding stateful rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Header {


    /// 
    /// The destination IP address or address range to inspect for, in CIDR notation.      To match with any address, specify ANY.
    /// 
    /// Specify an IP address or a block of IP addresses in Classless Inter-Domain Routing (CIDR) notation. Network Firewall supports all address ranges for IPv4 and IPv6.
    /// 
    /// Examples:
    /// 
    /// To configure Network Firewall to inspect for the IP address 192.0.2.44, specify 192.0.2.44/32.               To configure Network Firewall to inspect for IP addresses from 192.0.2.0 to 192.0.2.255, specify 192.0.2.0/24.               To configure Network Firewall to inspect for the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify 1111:0000:0000:0000:0000:0000:0000:0111/128.               To configure Network Firewall to inspect for IP addresses from 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify 1111:0000:0000:0000:0000:0000:0000:0000/64.
    /// 
    /// For more information about CIDR notation, see the Wikipedia entry Classless     Inter-Domain Routing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: String,


    /// 
    /// The destination port to inspect for. You can specify an individual port, for      example 1994 and you can specify     a port range, for example 1990:1994.      To match with any port, specify ANY.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPort")]
    pub destination_port: String,


    /// 
    /// The direction of traffic flow to inspect. If set to ANY, the inspection     matches bidirectional traffic, both from the source to the destination and from the     destination to the source. If set to FORWARD, the inspection only matches     traffic going from the source to the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ANY | FORWARD
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: HeaderDirectionEnum,


    /// 
    /// The protocol to inspect for. To specify all, you can use IP, because all traffic on AWS and on the internet is IP.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DCERPC | DHCP | DNS | FTP | HTTP | ICMP | IKEV2 | IMAP | IP | KRB5 | MSN | NTP | SMB | SMTP | SSH | TCP | TFTP | TLS | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: HeaderProtocolEnum,


    /// 
    /// The source IP address or address range to inspect for, in CIDR notation.      To match with any address, specify ANY.
    /// 
    /// Specify an IP address or a block of IP addresses in Classless Inter-Domain Routing (CIDR) notation. Network Firewall supports all address ranges for IPv4 and IPv6.
    /// 
    /// Examples:
    /// 
    /// To configure Network Firewall to inspect for the IP address 192.0.2.44, specify 192.0.2.44/32.               To configure Network Firewall to inspect for IP addresses from 192.0.2.0 to 192.0.2.255, specify 192.0.2.0/24.               To configure Network Firewall to inspect for the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify 1111:0000:0000:0000:0000:0000:0000:0111/128.               To configure Network Firewall to inspect for IP addresses from 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify 1111:0000:0000:0000:0000:0000:0000:0000/64.
    /// 
    /// For more information about CIDR notation, see the Wikipedia entry Classless     Inter-Domain Routing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: String,


    /// 
    /// The source port to inspect for. You can specify an individual port, for      example 1994 and you can specify a port        range, for example 1990:1994.      To match with any port, specify ANY.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePort")]
    pub source_port: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HeaderDirectionEnum {

    /// ANY
    #[serde(rename = "ANY")]
    Any,

    /// FORWARD
    #[serde(rename = "FORWARD")]
    Forward,

}

impl Default for HeaderDirectionEnum {
    fn default() -> Self {
        HeaderDirectionEnum::Any
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum HeaderProtocolEnum {

    /// DCERPC
    #[serde(rename = "DCERPC")]
    Dcerpc,

    /// DHCP
    #[serde(rename = "DHCP")]
    Dhcp,

    /// DNS
    #[serde(rename = "DNS")]
    Dns,

    /// FTP
    #[serde(rename = "FTP")]
    Ftp,

    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// ICMP
    #[serde(rename = "ICMP")]
    Icmp,

    /// IKEV2
    #[serde(rename = "IKEV2")]
    Ikev2,

    /// IMAP
    #[serde(rename = "IMAP")]
    Imap,

    /// IP
    #[serde(rename = "IP")]
    Ip,

    /// KRB5
    #[serde(rename = "KRB5")]
    Krb5,

    /// MSN
    #[serde(rename = "MSN")]
    Msn,

    /// NTP
    #[serde(rename = "NTP")]
    Ntp,

    /// SMB
    #[serde(rename = "SMB")]
    Smb,

    /// SMTP
    #[serde(rename = "SMTP")]
    Smtp,

    /// SSH
    #[serde(rename = "SSH")]
    Ssh,

    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

    /// TFTP
    #[serde(rename = "TFTP")]
    Tftp,

    /// TLS
    #[serde(rename = "TLS")]
    Tls,

    /// UDP
    #[serde(rename = "UDP")]
    Udp,

}

impl Default for HeaderProtocolEnum {
    fn default() -> Self {
        HeaderProtocolEnum::Dcerpc
    }
}


impl cfn_resources::CfnResource for Header {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.destination;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'destination'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.destination;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'destination'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.destination_port;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'destination_port'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.destination_port;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'destination_port'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.source;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'source'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.source;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'source'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.source_port;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'source_port'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.source_port;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'source_port'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A list of IP addresses and address ranges, in CIDR notation. This is part of a AWS::NetworkFirewall::RuleGroup RuleVariables.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IPSet {


    /// 
    /// The list of IP addresses and address ranges, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for IPSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Configures one or more IPSetReferences for a Suricata-compatible rule group. An IP set reference is a rule variable that references a resource that you create and manage in another AWS service, such as an Amazon VPC prefix list. Network Firewall IP set references enable you to dynamically update the contents of your rules. When you create, update, or delete the IP set you are referencing in your rule, Network Firewall automatically updates the rule's content with the changes. For more information about IP set references in Network Firewall, see Using IP set references in the Network Firewall Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IPSetReference {


    /// 
    /// The Amazon Resource Name (ARN) of the resource to include in the AWS::NetworkFirewall::RuleGroup IPSetReference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceArn")]
    pub reference_arn: Option<String>,

}



impl cfn_resources::CfnResource for IPSetReference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.reference_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'reference_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.reference_arn {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'reference_arn'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Criteria for Network Firewall to use to inspect an individual packet in stateless rule inspection. Each match attributes set can include one or more items such as IP address, CIDR range, port number, protocol, and TCP flags.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MatchAttributes {


    /// 
    /// The destination ports to inspect for. If not specified, this matches with any     destination port. This setting is only used for protocols 6 (TCP) and 17 (UDP).
    /// 
    /// You can specify individual ports, for example 1994 and you can specify port     ranges, for example 1990:1994.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPorts")]
    pub destination_ports: Option<Vec<PortRange>>,


    /// 
    /// The destination IP addresses and address ranges to inspect for, in CIDR notation. If not     specified, this matches with any destination address.
    /// 
    /// Required: No
    ///
    /// Type: List of Address
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Address>>,


    /// 
    /// The protocols to inspect for, specified using each protocol's assigned internet protocol     number (IANA). If not specified, this matches with any protocol.
    /// 
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<i64>>,


    /// 
    /// The source ports to inspect for. If not specified, this matches with any source port.     This setting is only used for protocols 6 (TCP) and 17 (UDP).
    /// 
    /// You can specify individual ports, for example 1994 and you can specify port     ranges, for example 1990:1994.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePorts")]
    pub source_ports: Option<Vec<PortRange>>,


    /// 
    /// The source IP addresses and address ranges to inspect for, in CIDR notation. If not     specified, this matches with any source address.
    /// 
    /// Required: No
    ///
    /// Type: List of Address
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<Address>>,


    /// 
    /// The TCP flags and masks to inspect for. If not specified, this matches with any     settings. This setting is only used for protocol 6 (TCP).
    /// 
    /// Required: No
    ///
    /// Type: List of TCPFlagField
    ///
    /// Update requires: No interruption
    #[serde(rename = "TCPFlags")]
    pub tcpflags: Option<Vec<TCPFlagField>>,

}



impl cfn_resources::CfnResource for MatchAttributes {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A single port range specification. This is used for source and destination port ranges     in the stateless AWS::NetworkFirewall::RuleGroup MatchAttributes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortRange {


    /// 
    /// The lower limit of the port range. This must be less than or equal to the       ToPort specification.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    pub from_port: i64,


    /// 
    /// The upper limit of the port range. This must be greater than or equal to the       FromPort specification.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.from_port;

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'from_port'. {} is greater than 65535", the_val));
        }

        
        let the_val = &self.from_port;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'from_port'. {} is less than 0", the_val));
        }

        
        let the_val = &self.to_port;

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'to_port'. {} is greater than 65535", the_val));
        }

        
        let the_val = &self.to_port;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'to_port'. {} is less than 0", the_val));
        }

        
        Ok(())
    }
}

/// A set of port ranges for use in the rules in a rule group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortSet {


    /// 
    /// The set of port ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for PortSet {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Stateless inspection criteria that publishes the specified metrics to Amazon CloudWatch for the     matching packet. This setting defines a CloudWatch dimension value to be published.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PublishMetricAction {


    /// 
    /// 
    /// 
    /// Required: Yes
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<Dimension>,

}



impl cfn_resources::CfnResource for PublishMetricAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.dimensions;

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'dimensions'. {} is greater than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Configures the ReferenceSets for a stateful rule group. For more information, see the Using IP set references in Suricata compatible rule groups in the Network Firewall User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferenceSets {


    /// 
    /// The IP set references to use in the stateful rule group.
    /// 
    /// Required: No
    ///
    /// Type: Map of IPSetReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPSetReferences")]
    pub ipset_references: Option<std::collections::HashMap<String, IPSetReference>>,

}



impl cfn_resources::CfnResource for ReferenceSets {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The inspection criteria and action for a single stateless rule. AWS Network Firewall inspects each packet for the specified matching        criteria. When a packet matches the criteria, Network Firewall performs the rule's actions on     the packet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuleDefinition {


    /// 
    /// The actions to take on a packet that matches one of the stateless rule definition's     match attributes. You must specify a standard action and you can add custom actions.
    /// 
    /// NoteNetwork Firewall only forwards a packet for stateful rule inspection if you specify        aws:forward_to_sfe for a rule that the packet matches, or if the packet       doesn't match any stateless rule and you specify aws:forward_to_sfe for the        StatelessDefaultActions setting for the AWS::NetworkFirewall::FirewallPolicy.
    /// 
    /// For every rule, you must specify exactly one of the following standard actions.
    /// 
    /// aws:pass - Discontinues all inspection of        the packet and permits it to go to its intended destination.                        aws:drop - Discontinues all inspection of        the packet and blocks it from going to its intended destination.                        aws:forward_to_sfe - Discontinues        stateless inspection of the packet and forwards it to the stateful rule engine for        inspection.
    /// 
    /// Additionally, you can specify a custom action.     To do this, you define a custom action by name and type, then provide the name you've assigned     to the action in this Actions setting.
    /// 
    /// To provide more than one action in this setting, separate the settings with a comma. For     example, if you have a publish metrics custom action that you've named       MyMetricsAction, then you could specify the standard action       aws:pass combined with the custom action using [“aws:pass”,       “MyMetricsAction”].
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,


    /// 
    /// Criteria for Network Firewall to use to inspect an individual packet in stateless rule inspection. Each match attributes set can include one or more items such as IP address, CIDR range, port number, protocol, and TCP flags.
    /// 
    /// Required: Yes
    ///
    /// Type: MatchAttributes
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchAttributes")]
    pub match_attributes: MatchAttributes,

}



impl cfn_resources::CfnResource for RuleDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.match_attributes.validate()?;

        Ok(())
    }
}

/// The object that defines the rules in a rule group.
///
/// AWS Network Firewall uses a rule group to inspect and control network traffic.   You define stateless rule groups to inspect individual packets and you define stateful rule groups to inspect packets in the context of their   traffic flow.
///
/// To use a rule group, you include it by reference in an Network Firewall firewall policy, then you use the policy in a firewall. You can reference a rule group from   more than one firewall policy, and you can use a firewall policy in more than one firewall.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuleGroup {


    /// 
    /// The reference sets for the stateful rule group.
    /// 
    /// Required: No
    ///
    /// Type: ReferenceSets
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceSets")]
    pub reference_sets: Option<ReferenceSets>,


    /// 
    /// Settings that are available for use in the rules in the rule group. You can only use     these for stateful rule groups.
    /// 
    /// Required: No
    ///
    /// Type: RuleVariables
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleVariables")]
    pub rule_variables: Option<RuleVariables>,


    /// 
    /// The stateful rules or stateless rules for the rule group.
    /// 
    /// Required: Yes
    ///
    /// Type: RulesSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "RulesSource")]
    pub rules_source: RulesSource,


    /// 
    /// Additional options governing how Network Firewall handles stateful rules. The policies where you use your stateful    rule group must have stateful rule options settings that are compatible with these settings.
    /// 
    /// Required: No
    ///
    /// Type: StatefulRuleOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatefulRuleOptions")]
    pub stateful_rule_options: Option<StatefulRuleOptions>,

}



impl cfn_resources::CfnResource for RuleGroup {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.reference_sets.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.rule_variables.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.rules_source.validate()?;

        self.stateful_rule_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Additional settings for a stateful rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuleOption {


    /// 
    /// The Suricata rule option keywords. For Network Firewall, the keyword signature ID (sid) is required in the format sid:112233. The sid must be unique within the rule group. For information about Suricata rule option keywords, see Rule options.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Keyword")]
    pub keyword: String,


    /// 
    /// The Suricata rule option settings. Settings have zero or more values, and the number of possible settings and required settings depends on the keyword. The format for Settings is number. For information about Suricata rule option settings, see Rule options.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Settings")]
    pub settings: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for RuleOption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.keyword;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'keyword'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.keyword;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'keyword'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Settings that are available for use in the rules in the AWS::NetworkFirewall::RuleGroup     where this is defined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuleVariables {


    /// 
    /// A list of IP addresses and address ranges, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: Map of IPSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPSets")]
    pub ipsets: Option<std::collections::HashMap<String, IPSet>>,


    /// 
    /// A list of port ranges.
    /// 
    /// Required: No
    ///
    /// Type: Map of PortSet
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortSets")]
    pub port_sets: Option<std::collections::HashMap<String, PortSet>>,

}



impl cfn_resources::CfnResource for RuleVariables {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The stateless or stateful rules definitions for use in a single rule group. Each rule     group requires a single RulesSource. You can use an instance of this for     either stateless rules or stateful rules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RulesSource {


    /// 
    /// Stateful inspection criteria for a domain list rule group.
    /// 
    /// Required: No
    ///
    /// Type: RulesSourceList
    ///
    /// Update requires: No interruption
    #[serde(rename = "RulesSourceList")]
    pub rules_source_list: Option<RulesSourceList>,


    /// 
    /// Stateful inspection criteria, provided in Suricata compatible intrusion prevention     system (IPS) rules. Suricata is an open-source network IPS that includes a standard     rule-based language for network traffic inspection.
    /// 
    /// These rules contain the inspection criteria and the action to take for traffic that     matches the criteria, so this type of rule group doesn't have a separate action     setting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2000000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RulesString")]
    pub rules_string: Option<String>,


    /// 
    /// An array of individual stateful rules inspection criteria to be used together in a stateful rule group.    Use this option to specify simple Suricata rules with protocol, source and destination, ports, direction, and rule options.    For information about the Suricata Rules format, see                     Rules Format.
    /// 
    /// Required: No
    ///
    /// Type: List of StatefulRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatefulRules")]
    pub stateful_rules: Option<Vec<StatefulRule>>,


    /// 
    /// Stateless inspection criteria to be used in a stateless rule group.
    /// 
    /// Required: No
    ///
    /// Type: StatelessRulesAndCustomActions
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessRulesAndCustomActions")]
    pub stateless_rules_and_custom_actions: Option<StatelessRulesAndCustomActions>,

}



impl cfn_resources::CfnResource for RulesSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.rules_source_list.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.rules_string {

        if the_val.len() > 2000000 as _ {
            return Err(format!("Max validation failed on field 'rules_string'. {} is greater than 2000000", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.rules_string {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'rules_string'. {} is less than 0", the_val.len()));
        }

        }
        
        self.stateless_rules_and_custom_actions.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Stateful inspection criteria for a domain list rule group.
///
/// For HTTPS traffic, domain filtering is SNI-based. It uses the server name indicator extension of the TLS handshake.
///
/// By default, Network Firewall domain list inspection only includes traffic coming from the VPC where you deploy the firewall. To inspect traffic from IP addresses outside of the deployment VPC, you set the HOME_NET rule variable to include the CIDR range of the deployment VPC plus the other CIDR ranges. For more information, see AWS::NetworkFirewall::RuleGroup RuleVariables in this guide and Stateful domain list rule groups in AWS Network Firewall in the Network Firewall Developer Guide
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RulesSourceList {


    /// 
    /// Whether you want to allow or deny access to the domains in your target list.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOWLIST | DENYLIST
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeneratedRulesType")]
    pub generated_rules_type: RulesSourceListGeneratedRulesTypeEnum,


    /// 
    /// The types of targets to inspect for. Valid values are TLS_SNI and HTTP_HOST.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTypes")]
    pub target_types: Vec<String>,


    /// 
    /// The domains that you want to inspect for in your traffic flows. Valid domain specifications are the following:
    /// 
    /// Explicit names. For example, abc.example.com matches only the domain abc.example.com.               Names that use a domain wildcard, which you indicate with an initial '.'. For example,.example.com matches example.com and matches all subdomains of example.com, such as abc.example.com and www.example.com.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Vec<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RulesSourceListGeneratedRulesTypeEnum {

    /// ALLOWLIST
    #[serde(rename = "ALLOWLIST")]
    Allowlist,

    /// DENYLIST
    #[serde(rename = "DENYLIST")]
    Denylist,

}

impl Default for RulesSourceListGeneratedRulesTypeEnum {
    fn default() -> Self {
        RulesSourceListGeneratedRulesTypeEnum::Allowlist
    }
}


impl cfn_resources::CfnResource for RulesSourceList {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A single Suricata rules specification, for use in a stateful rule group.    Use this option to specify a simple Suricata rule with protocol, source and destination, ports, direction, and rule options.    For information about the Suricata Rules format, see                     Rules Format.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatefulRule {


    /// 
    /// Defines what Network Firewall should do with the packets in a traffic flow when the flow     matches the stateful rule criteria. For all actions, Network Firewall performs the specified     action and discontinues stateful inspection of the traffic flow.
    /// 
    /// The actions for a stateful rule are defined as follows:
    /// 
    /// PASS - Permits the packets to go to the        intended destination.                        DROP - Blocks the packets from going to        the intended destination and sends an alert log message, if alert logging is configured in the AWS::NetworkFirewall::Firewall          AWS::NetworkFirewall::LoggingConfiguration.                         REJECT - Drops traffic that matches the conditions of the stateful rule and sends a TCP reset packet back to sender of the packet. A TCP reset packet is a packet with no payload and a RST bit contained in the TCP header flags. REJECT is available only for TCP traffic.                        ALERT - Permits the packets to go to the        intended destination and sends an alert log message, if alert logging is configured in the AWS::NetworkFirewall::Firewall          AWS::NetworkFirewall::LoggingConfiguration.         You can use this action to test a rule that you intend to use to drop traffic. You        can enable the rule with ALERT action, verify in the logs that the rule        is filtering as you want, then change the action to DROP.                        REJECT - Drops TCP traffic that matches the conditions of the stateful rule, and sends a TCP reset packet back to sender of the packet. A TCP reset packet is a packet with no payload and a RST bit contained in the TCP header flags. Also sends an alert log mesage if alert logging is configured in the AWS::NetworkFirewall::Firewall          AWS::NetworkFirewall::LoggingConfiguration.        REJECT isn't currently available for use with IMAP and FTP protocols.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALERT | DROP | PASS | REJECT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: StatefulRuleActionEnum,


    /// 
    /// The stateful inspection criteria for this rule, used to inspect traffic flows.
    /// 
    /// Required: Yes
    ///
    /// Type: Header
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: Header,


    /// 
    /// Additional settings for a stateful rule, provided as keywords and settings.
    /// 
    /// Required: Yes
    ///
    /// Type: List of RuleOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleOptions")]
    pub rule_options: Vec<RuleOption>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StatefulRuleActionEnum {

    /// ALERT
    #[serde(rename = "ALERT")]
    Alert,

    /// DROP
    #[serde(rename = "DROP")]
    Drop,

    /// PASS
    #[serde(rename = "PASS")]
    Pass,

    /// REJECT
    #[serde(rename = "REJECT")]
    Reject,

}

impl Default for StatefulRuleActionEnum {
    fn default() -> Self {
        StatefulRuleActionEnum::Alert
    }
}


impl cfn_resources::CfnResource for StatefulRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.header.validate()?;

        Ok(())
    }
}

/// Additional options governing how Network Firewall handles the rule group. You can only use these for stateful rule groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatefulRuleOptions {


    /// 
    /// Indicates how to manage the order of the rule evaluation for the rule group. DEFAULT_ACTION_ORDER is       the default behavior. Stateful rules are provided to the rule engine as Suricata compatible strings, and Suricata evaluates them       based on certain settings. For more information, see      Evaluation order for stateful rules in the AWS Network Firewall Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT_ACTION_ORDER | STRICT_ORDER
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleOrder")]
    pub rule_order: Option<StatefulRuleOptionsRuleOrderEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StatefulRuleOptionsRuleOrderEnum {

    /// DEFAULT_ACTION_ORDER
    #[serde(rename = "DEFAULT_ACTION_ORDER")]
    Defaultactionorder,

    /// STRICT_ORDER
    #[serde(rename = "STRICT_ORDER")]
    Strictorder,

}

impl Default for StatefulRuleOptionsRuleOrderEnum {
    fn default() -> Self {
        StatefulRuleOptionsRuleOrderEnum::Defaultactionorder
    }
}


impl cfn_resources::CfnResource for StatefulRuleOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A single stateless rule. This is used in AWS::NetworkFirewall::RuleGroup StatelessRulesAndCustomActions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatelessRule {


    /// 
    /// Indicates the order in which to run this rule relative to all of the     rules that are defined for a stateless rule group. Network Firewall evaluates the rules in a     rule group starting with the lowest priority setting. You must ensure that the priority     settings are unique for the rule group.
    /// 
    /// Each stateless rule group uses exactly one StatelessRulesAndCustomActions     object, and each StatelessRulesAndCustomActions contains exactly one       StatelessRules object. To ensure unique priority settings for your rule     groups, set unique priorities for the stateless rules that you define inside any single       StatelessRules object.
    /// 
    /// You can change the priority settings of your rules at any time. To make it easier to     insert rules later, number them so there's a wide range in between, for example use 100,     200, and so on.
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
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// Defines the stateless 5-tuple packet inspection criteria and the action to take on a     packet that matches the criteria.
    /// 
    /// Required: Yes
    ///
    /// Type: RuleDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleDefinition")]
    pub rule_definition: RuleDefinition,

}



impl cfn_resources::CfnResource for StatelessRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.priority;

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'priority'. {} is greater than 65535", the_val));
        }

        
        let the_val = &self.priority;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'priority'. {} is less than 1", the_val));
        }

        
        self.rule_definition.validate()?;

        Ok(())
    }
}

/// Stateless inspection criteria. Each stateless rule group uses exactly one of these data     types to define its stateless rules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatelessRulesAndCustomActions {


    /// 
    /// Defines an array of individual custom action definitions that are available for use by     the stateless rules in this StatelessRulesAndCustomActions specification. You     name each custom action that you define, and then you can use it by name in your stateless rule       AWS::NetworkFirewall::RuleGroup RuleDefinition Actions specification.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomActions")]
    pub custom_actions: Option<Vec<CustomAction>>,


    /// 
    /// Defines the set of stateless rules for use in a stateless rule group.
    /// 
    /// Required: Yes
    ///
    /// Type: List of StatelessRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatelessRules")]
    pub stateless_rules: Vec<StatelessRule>,

}



impl cfn_resources::CfnResource for StatelessRulesAndCustomActions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// TCP flags and masks to inspect packets for. This is used in the AWS::NetworkFirewall::RuleGroup MatchAttributes       specification.
///
/// For example:
///
/// "TCPFlags": [     {       "Flags": [         "ECE",         "SYN"       ],       "Masks": [         "SYN",         "ECE"       ]     }       ]
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TCPFlagField {


    /// 
    /// Used in conjunction with the Masks setting to define the flags that must be set and flags that must not be set in order for the packet to match. This setting can only specify values that are also specified in the Masks setting.
    /// 
    /// For the flags that are specified in the masks setting, the following must be true for the packet to match:
    /// 
    /// The ones that are set in this flags setting must be set in the packet.               The ones that are not set in this flags setting must also not be set in the packet.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Flags")]
    pub flags: Vec<String>,


    /// 
    /// The set of flags to consider in the inspection. To inspect all flags in the valid values list, leave this with no setting.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Masks")]
    pub masks: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for TCPFlagField {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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



impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}