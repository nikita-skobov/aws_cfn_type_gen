

/// Specifies a Client VPN endpoint. A Client VPN endpoint is the resource you create and     configure to enable and manage client VPN sessions. It is the destination endpoint at which     all client VPN sessions are terminated.
#[derive(Default, serde::Serialize)]
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
    pub self_service_portal: Option<String>,


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
    pub transport_protocol: Option<String>,


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
    /// The options for managing connection authorization for new client connections.
    /// 
    /// Required: No
    ///
    /// Type: ClientConnectOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientConnectOptions")]
    pub client_connect_options: Option<ClientConnectOptions>,

}


/// The tags to apply to a resource when the resource is being created. When you specify a tag, you must     specify the resource type to tag, otherwise the request will fail.
#[derive(Default, serde::Serialize)]
pub struct TagSpecification {


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
    pub resource_type: String,

}


/// Options for enabling a customizable text banner that will be displayed on 			AWS provided clients when a VPN session is established.
#[derive(Default, serde::Serialize)]
pub struct ClientLoginBannerOptions {


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

}


/// Describes the Active Directory to be used for client authentication.
#[derive(Default, serde::Serialize)]
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


/// Describes the client connection logging options for the Client VPN endpoint.
#[derive(Default, serde::Serialize)]
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


/// Describes the authentication method to be used by a Client VPN endpoint. For more information, see Authentication 			in the         AWS Client VPN Administrator Guide.
#[derive(Default, serde::Serialize)]
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
    pub cfn_type: String,

}


/// The IAM SAML identity provider used for federated authentication.
#[derive(Default, serde::Serialize)]
pub struct FederatedAuthenticationRequest {


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

}


/// Information about the client certificate to be used for authentication.
#[derive(Default, serde::Serialize)]
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


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// Indicates whether client connect options are enabled. The default is false     (not enabled).
#[derive(Default, serde::Serialize)]
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
