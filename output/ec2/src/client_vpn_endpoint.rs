

/// Specifies a Client VPN endpoint. A Client VPN endpoint is the resource you create and     configure to enable and manage client VPN sessions. It is the destination endpoint at which     all client VPN sessions are terminated.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClientVpnEndpoint {


    /// 
    /// Information about the authentication method to be used to authenticate clients.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ClientAuthenticationRequest
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationOptions")]
    pub authentication_options: Vec<ClientAuthenticationRequest>,


    /// 
    /// The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. Client CIDR range must have a size of at least /22 and must not be greater than /12.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientCidrBlock")]
    pub client_cidr_block: String,


    /// 
    /// The options for managing connection authorization for new client connections.
    /// 
    /// Required: No
    ///
    /// Type: ClientConnectOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientConnectOptions")]
    pub client_connect_options: Option<ClientConnectOptions>,


    /// 
    /// Options for enabling a customizable text banner that will be displayed on AWS provided clients when a VPN session is 			established.
    /// 
    /// Required: No
    ///
    /// Type: ClientLoginBannerOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientLoginBannerOptions")]
    pub client_login_banner_options: Option<ClientLoginBannerOptions>,


    /// 
    /// Information about the client connection logging options.
    /// 
    /// If you enable client connection logging, data about client connections is sent to a 			Cloudwatch Logs log stream. The following information is logged:
    /// 
    /// Client connection requests               Client connection results (successful and unsuccessful)               Reasons for unsuccessful client connection requests               Client connection termination time
    /// 
    /// Required: Yes
    ///
    /// Type: ConnectionLogOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionLogOptions")]
    pub connection_log_options: ConnectionLogOptions,


    /// 
    /// A brief description of the Client VPN endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can 			have up to two DNS servers. If no DNS server is specified, the DNS address configured on the device is used for the DNS server.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsServers")]
    pub dns_servers: Option<Vec<String>>,


    /// 
    /// The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// Specify whether to enable the self-service portal for the Client VPN endpoint.
    /// 
    /// Default Value: enabled
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disabled | enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfServicePortal")]
    pub self_service_portal: Option<ClientVpnEndpointSelfServicePortalEnum>,


    /// 
    /// The ARN of the server certificate. For more information, see 			the AWS Certificate Manager User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateArn")]
    pub server_certificate_arn: String,


    /// 
    /// The maximum VPN session duration time in hours.
    /// 
    /// Valid values: 8 | 10 | 12 | 24
    /// 
    /// Default value: 24
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeoutHours")]
    pub session_timeout_hours: Option<i64>,


    /// 
    /// Indicates whether split-tunnel is enabled on the AWS Client VPN endpoint.
    /// 
    /// By default, split-tunnel on a VPN endpoint is disabled.
    /// 
    /// For information about split-tunnel VPN endpoints, see Split-tunnel AWS Client VPN endpoint in the 			        AWS Client VPN Administrator Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SplitTunnel")]
    pub split_tunnel: Option<bool>,


    /// 
    /// The tags to apply to the Client VPN endpoint during creation.
    /// 
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,


    /// 
    /// The transport protocol to be used by the VPN session.
    /// 
    /// Default value: udp
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: tcp | udp
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransportProtocol")]
    pub transport_protocol: Option<ClientVpnEndpointTransportProtocolEnum>,


    /// 
    /// The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,


    /// 
    /// The port number to assign to the Client VPN endpoint for TCP and UDP traffic.
    /// 
    /// Valid Values: 443 | 1194
    /// 
    /// Default Value: 443
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnPort")]
    pub vpn_port: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ClientVpnEndpointSelfServicePortalEnum {

    /// disabled
    #[serde(rename = "disabled")]
    Disabled,

    /// enabled
    #[serde(rename = "enabled")]
    Enabled,

}

impl Default for ClientVpnEndpointSelfServicePortalEnum {
    fn default() -> Self {
        ClientVpnEndpointSelfServicePortalEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ClientVpnEndpointTransportProtocolEnum {

    /// tcp
    #[serde(rename = "tcp")]
    Tcp,

    /// udp
    #[serde(rename = "udp")]
    Udp,

}

impl Default for ClientVpnEndpointTransportProtocolEnum {
    fn default() -> Self {
        ClientVpnEndpointTransportProtocolEnum::Tcp
    }
}


impl cfn_resources::CfnResource for CfnClientVpnEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::ClientVpnEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.client_connect_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.client_login_banner_options.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.connection_log_options.validate()?;

        Ok(())
    }
}

/// Information about the client certificate to be used for authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CertificateAuthenticationRequest {


    /// 
    /// The ARN of the client certificate. The certificate must be signed by a certificate 			authority (CA) and it must be provisioned in AWS Certificate Manager (ACM).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientRootCertificateChainArn")]
    pub client_root_certificate_chain_arn: String,

}



impl cfn_resources::CfnResource for CertificateAuthenticationRequest {
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

/// Describes the authentication method to be used by a Client VPN endpoint. For more information, see Authentication 			in the         AWS Client VPN Administrator Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientAuthenticationRequest {


    /// 
    /// Information about the Active Directory to be used, if applicable. You must provide this information if Type is directory-service-authentication.
    /// 
    /// Required: No
    ///
    /// Type: DirectoryServiceAuthenticationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveDirectory")]
    pub active_directory: Option<DirectoryServiceAuthenticationRequest>,


    /// 
    /// Information about the IAM SAML identity provider, if applicable.
    /// 
    /// Required: No
    ///
    /// Type: FederatedAuthenticationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "FederatedAuthentication")]
    pub federated_authentication: Option<FederatedAuthenticationRequest>,


    /// 
    /// Information about the authentication certificates to be used, if applicable. You must provide this information if Type is certificate-authentication.
    /// 
    /// Required: No
    ///
    /// Type: CertificateAuthenticationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "MutualAuthentication")]
    pub mutual_authentication: Option<CertificateAuthenticationRequest>,


    /// 
    /// The type of client authentication to be used.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: certificate-authentication | directory-service-authentication | federated-authentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ClientAuthenticationRequestTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ClientAuthenticationRequestTypeEnum {

    /// certificate-authentication
    #[serde(rename = "certificate-authentication")]
    Certificateauthentication,

    /// directory-service-authentication
    #[serde(rename = "directory-service-authentication")]
    Directoryserviceauthentication,

    /// federated-authentication
    #[serde(rename = "federated-authentication")]
    Federatedauthentication,

}

impl Default for ClientAuthenticationRequestTypeEnum {
    fn default() -> Self {
        ClientAuthenticationRequestTypeEnum::Certificateauthentication
    }
}


impl cfn_resources::CfnResource for ClientAuthenticationRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.active_directory.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.federated_authentication.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.mutual_authentication.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Indicates whether client connect options are enabled. The default is false     (not enabled).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientConnectOptions {


    /// 
    /// Indicates whether client connect options are enabled. The default is false     (not enabled).
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Lambda function used for     connection authorization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: Option<String>,

}



impl cfn_resources::CfnResource for ClientConnectOptions {
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

/// Options for enabling a customizable text banner that will be displayed on 			AWS provided clients when a VPN session is established.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientLoginBannerOptions {


    /// 
    /// Customizable text that will be displayed in a banner on AWS provided 			clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 			1400 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BannerText")]
    pub banner_text: Option<String>,


    /// 
    /// Enable or disable a customizable text banner that will be displayed on 			AWS provided clients when a VPN session is established.
    /// 
    /// Valid values: true | false
    /// 
    /// Default value: false
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}



impl cfn_resources::CfnResource for ClientLoginBannerOptions {
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

/// Describes the client connection logging options for the Client VPN endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionLogOptions {


    /// 
    /// The name of the CloudWatch Logs log group. Required if connection logging is enabled.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchLogGroup")]
    pub cloudwatch_log_group: Option<String>,


    /// 
    /// The name of the CloudWatch Logs log stream to which the connection data is published.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchLogStream")]
    pub cloudwatch_log_stream: Option<String>,


    /// 
    /// Indicates whether connection logging is enabled.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}



impl cfn_resources::CfnResource for ConnectionLogOptions {
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

/// Describes the Active Directory to be used for client authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DirectoryServiceAuthenticationRequest {


    /// 
    /// The ID of the Active Directory to be used for authentication.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,

}



impl cfn_resources::CfnResource for DirectoryServiceAuthenticationRequest {
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

/// The IAM SAML identity provider used for federated authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FederatedAuthenticationRequest {


    /// 
    /// The Amazon Resource Name (ARN) of the IAM SAML identity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAMLProviderArn")]
    pub samlprovider_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM SAML identity provider for the self-service portal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfServiceSAMLProviderArn")]
    pub self_service_samlprovider_arn: Option<String>,

}



impl cfn_resources::CfnResource for FederatedAuthenticationRequest {
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

/// The tags to apply to a resource when the resource is being created. When you specify a tag, you must     specify the resource type to tag, otherwise the request will fail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagSpecification {


    /// 
    /// The type of resource to tag.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-reservation | capacity-reservation-fleet | carrier-gateway | client-vpn-endpoint | coip-pool | customer-gateway | dedicated-host | dhcp-options | egress-only-internet-gateway | elastic-gpu | elastic-ip | export-image-task | export-instance-task | fleet | fpga-image | host-reservation | image | import-image-task | import-snapshot-task | instance | instance-event-window | internet-gateway | ipam | ipam-pool | ipam-resource-discovery | ipam-resource-discovery-association | ipam-scope | ipv4pool-ec2 | ipv6pool-ec2 | key-pair | launch-template | local-gateway | local-gateway-route-table | local-gateway-route-table-virtual-interface-group-association | local-gateway-route-table-vpc-association | local-gateway-virtual-interface | local-gateway-virtual-interface-group | natgateway | network-acl | network-insights-access-scope | network-insights-access-scope-analysis | network-insights-analysis | network-insights-path | network-interface | placement-group | prefix-list | replace-root-volume-task | reserved-instances | route-table | security-group | security-group-rule | snapshot | spot-fleet-request | spot-instances-request | subnet | subnet-cidr-reservation | traffic-mirror-filter | traffic-mirror-filter-rule | traffic-mirror-session | traffic-mirror-target | transit-gateway | transit-gateway-attachment | transit-gateway-connect-peer | transit-gateway-multicast-domain | transit-gateway-policy-table | transit-gateway-route-table | transit-gateway-route-table-announcement | verified-access-endpoint | verified-access-group | verified-access-instance | verified-access-policy | verified-access-trust-provider | volume | vpc | vpc-block-public-access-exclusion | vpc-endpoint | vpc-endpoint-connection | vpc-endpoint-connection-device-type | vpc-endpoint-service | vpc-endpoint-service-permission | vpc-flow-log | vpc-peering-connection | vpn-connection | vpn-connection-device-type | vpn-gateway
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: TagSpecificationResourceTypeEnum,


    /// 
    /// The tags to apply to the resource.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TagSpecificationResourceTypeEnum {

    /// capacity-reservation
    #[serde(rename = "capacity-reservation")]
    Capacityreservation,

    /// capacity-reservation-fleet
    #[serde(rename = "capacity-reservation-fleet")]
    Capacityreservationfleet,

    /// carrier-gateway
    #[serde(rename = "carrier-gateway")]
    Carriergateway,

    /// client-vpn-endpoint
    #[serde(rename = "client-vpn-endpoint")]
    Clientvpnendpoint,

    /// coip-pool
    #[serde(rename = "coip-pool")]
    Coippool,

    /// customer-gateway
    #[serde(rename = "customer-gateway")]
    Customergateway,

    /// dedicated-host
    #[serde(rename = "dedicated-host")]
    Dedicatedhost,

    /// dhcp-options
    #[serde(rename = "dhcp-options")]
    Dhcpoptions,

    /// egress-only-internet-gateway
    #[serde(rename = "egress-only-internet-gateway")]
    Egressonlyinternetgateway,

    /// elastic-gpu
    #[serde(rename = "elastic-gpu")]
    Elasticgpu,

    /// elastic-ip
    #[serde(rename = "elastic-ip")]
    Elasticip,

    /// export-image-task
    #[serde(rename = "export-image-task")]
    Exportimagetask,

    /// export-instance-task
    #[serde(rename = "export-instance-task")]
    Exportinstancetask,

    /// fleet
    #[serde(rename = "fleet")]
    Fleet,

    /// fpga-image
    #[serde(rename = "fpga-image")]
    Fpgaimage,

    /// host-reservation
    #[serde(rename = "host-reservation")]
    Hostreservation,

    /// image
    #[serde(rename = "image")]
    Image,

    /// import-image-task
    #[serde(rename = "import-image-task")]
    Importimagetask,

    /// import-snapshot-task
    #[serde(rename = "import-snapshot-task")]
    Importsnapshottask,

    /// instance
    #[serde(rename = "instance")]
    Instance,

    /// instance-event-window
    #[serde(rename = "instance-event-window")]
    Instanceeventwindow,

    /// internet-gateway
    #[serde(rename = "internet-gateway")]
    Internetgateway,

    /// ipam
    #[serde(rename = "ipam")]
    Ipam,

    /// ipam-pool
    #[serde(rename = "ipam-pool")]
    Ipampool,

    /// ipam-resource-discovery
    #[serde(rename = "ipam-resource-discovery")]
    Ipamresourcediscovery,

    /// ipam-resource-discovery-association
    #[serde(rename = "ipam-resource-discovery-association")]
    Ipamresourcediscoveryassociation,

    /// ipam-scope
    #[serde(rename = "ipam-scope")]
    Ipamscope,

    /// ipv4pool-ec2
    #[serde(rename = "ipv4pool-ec2")]
    Ipv4poolec2,

    /// ipv6pool-ec2
    #[serde(rename = "ipv6pool-ec2")]
    Ipv6poolec2,

    /// key-pair
    #[serde(rename = "key-pair")]
    Keypair,

    /// launch-template
    #[serde(rename = "launch-template")]
    Launchtemplate,

    /// local-gateway
    #[serde(rename = "local-gateway")]
    Localgateway,

    /// local-gateway-route-table
    #[serde(rename = "local-gateway-route-table")]
    Localgatewayroutetable,

    /// local-gateway-route-table-virtual-interface-group-association
    #[serde(rename = "local-gateway-route-table-virtual-interface-group-association")]
    Localgatewayroutetablevirtualinterfacegroupassociation,

    /// local-gateway-route-table-vpc-association
    #[serde(rename = "local-gateway-route-table-vpc-association")]
    Localgatewayroutetablevpcassociation,

    /// local-gateway-virtual-interface
    #[serde(rename = "local-gateway-virtual-interface")]
    Localgatewayvirtualinterface,

    /// local-gateway-virtual-interface-group
    #[serde(rename = "local-gateway-virtual-interface-group")]
    Localgatewayvirtualinterfacegroup,

    /// natgateway
    #[serde(rename = "natgateway")]
    Natgateway,

    /// network-acl
    #[serde(rename = "network-acl")]
    Networkacl,

    /// network-insights-access-scope
    #[serde(rename = "network-insights-access-scope")]
    Networkinsightsaccessscope,

    /// network-insights-access-scope-analysis
    #[serde(rename = "network-insights-access-scope-analysis")]
    Networkinsightsaccessscopeanalysis,

    /// network-insights-analysis
    #[serde(rename = "network-insights-analysis")]
    Networkinsightsanalysis,

    /// network-insights-path
    #[serde(rename = "network-insights-path")]
    Networkinsightspath,

    /// network-interface
    #[serde(rename = "network-interface")]
    Networkinterface,

    /// placement-group
    #[serde(rename = "placement-group")]
    Placementgroup,

    /// prefix-list
    #[serde(rename = "prefix-list")]
    Prefixlist,

    /// replace-root-volume-task
    #[serde(rename = "replace-root-volume-task")]
    Replacerootvolumetask,

    /// reserved-instances
    #[serde(rename = "reserved-instances")]
    Reservedinstances,

    /// route-table
    #[serde(rename = "route-table")]
    Routetable,

    /// security-group
    #[serde(rename = "security-group")]
    Securitygroup,

    /// security-group-rule
    #[serde(rename = "security-group-rule")]
    Securitygrouprule,

    /// snapshot
    #[serde(rename = "snapshot")]
    Snapshot,

    /// spot-fleet-request
    #[serde(rename = "spot-fleet-request")]
    Spotfleetrequest,

    /// spot-instances-request
    #[serde(rename = "spot-instances-request")]
    Spotinstancesrequest,

    /// subnet
    #[serde(rename = "subnet")]
    Subnet,

    /// subnet-cidr-reservation
    #[serde(rename = "subnet-cidr-reservation")]
    Subnetcidrreservation,

    /// traffic-mirror-filter
    #[serde(rename = "traffic-mirror-filter")]
    Trafficmirrorfilter,

    /// traffic-mirror-filter-rule
    #[serde(rename = "traffic-mirror-filter-rule")]
    Trafficmirrorfilterrule,

    /// traffic-mirror-session
    #[serde(rename = "traffic-mirror-session")]
    Trafficmirrorsession,

    /// traffic-mirror-target
    #[serde(rename = "traffic-mirror-target")]
    Trafficmirrortarget,

    /// transit-gateway
    #[serde(rename = "transit-gateway")]
    Transitgateway,

    /// transit-gateway-attachment
    #[serde(rename = "transit-gateway-attachment")]
    Transitgatewayattachment,

    /// transit-gateway-connect-peer
    #[serde(rename = "transit-gateway-connect-peer")]
    Transitgatewayconnectpeer,

    /// transit-gateway-multicast-domain
    #[serde(rename = "transit-gateway-multicast-domain")]
    Transitgatewaymulticastdomain,

    /// transit-gateway-policy-table
    #[serde(rename = "transit-gateway-policy-table")]
    Transitgatewaypolicytable,

    /// transit-gateway-route-table
    #[serde(rename = "transit-gateway-route-table")]
    Transitgatewayroutetable,

    /// transit-gateway-route-table-announcement
    #[serde(rename = "transit-gateway-route-table-announcement")]
    Transitgatewayroutetableannouncement,

    /// verified-access-endpoint
    #[serde(rename = "verified-access-endpoint")]
    Verifiedaccessendpoint,

    /// verified-access-group
    #[serde(rename = "verified-access-group")]
    Verifiedaccessgroup,

    /// verified-access-instance
    #[serde(rename = "verified-access-instance")]
    Verifiedaccessinstance,

    /// verified-access-policy
    #[serde(rename = "verified-access-policy")]
    Verifiedaccesspolicy,

    /// verified-access-trust-provider
    #[serde(rename = "verified-access-trust-provider")]
    Verifiedaccesstrustprovider,

    /// volume
    #[serde(rename = "volume")]
    Volume,

    /// vpc
    #[serde(rename = "vpc")]
    Vpc,

    /// vpc-block-public-access-exclusion
    #[serde(rename = "vpc-block-public-access-exclusion")]
    Vpcblockpublicaccessexclusion,

    /// vpc-endpoint
    #[serde(rename = "vpc-endpoint")]
    Vpcendpoint,

    /// vpc-endpoint-connection
    #[serde(rename = "vpc-endpoint-connection")]
    Vpcendpointconnection,

    /// vpc-endpoint-connection-device-type
    #[serde(rename = "vpc-endpoint-connection-device-type")]
    Vpcendpointconnectiondevicetype,

    /// vpc-endpoint-service
    #[serde(rename = "vpc-endpoint-service")]
    Vpcendpointservice,

    /// vpc-endpoint-service-permission
    #[serde(rename = "vpc-endpoint-service-permission")]
    Vpcendpointservicepermission,

    /// vpc-flow-log
    #[serde(rename = "vpc-flow-log")]
    Vpcflowlog,

    /// vpc-peering-connection
    #[serde(rename = "vpc-peering-connection")]
    Vpcpeeringconnection,

    /// vpn-connection
    #[serde(rename = "vpn-connection")]
    Vpnconnection,

    /// vpn-connection-device-type
    #[serde(rename = "vpn-connection-device-type")]
    Vpnconnectiondevicetype,

    /// vpn-gateway
    #[serde(rename = "vpn-gateway")]
    Vpngateway,

}

impl Default for TagSpecificationResourceTypeEnum {
    fn default() -> Self {
        TagSpecificationResourceTypeEnum::Capacityreservation
    }
}


impl cfn_resources::CfnResource for TagSpecification {
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