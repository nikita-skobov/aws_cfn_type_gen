/// Creates a new Capacity Reservation with the specified attributes. For more information,     see Capacity       Reservations in the Amazon EC2 User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCapacityReservation {
    ///
    /// The Availability Zone in which to create the Capacity Reservation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: cfn_resources::StrVal,

    ///
    /// Indicates whether the Capacity Reservation supports EBS-optimized instances. This optimization provides 			dedicated throughput to Amazon EBS and an optimized configuration stack to provide 			optimal I/O performance. This optimization isn't available with all instance types. 			Additional usage charges apply when using an EBS- optimized instance.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,

    ///
    /// The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity 			is released and you can no longer launch instances into it. The Capacity Reservation's state changes to 				expired when it reaches its end date and time.
    ///
    /// You must provide an EndDate value if EndDateType is 				limited. Omit EndDate if EndDateType is 				unlimited.
    ///
    /// If the EndDateType is limited, the Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 			5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<cfn_resources::StrVal>,

    ///
    /// Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end 			types:
    ///
    /// unlimited - The Capacity Reservation remains active until you explicitly cancel it. Do not 					provide an EndDate if the EndDateType is 						unlimited.                        limited - The Capacity Reservation expires automatically at a specified date and time. You must 					provide an EndDate value if the EndDateType value is 						limited.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: limited | unlimited
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndDateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_type: Option<CapacityReservationEndDateTypeEnum>,

    ///
    /// Deprecated.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<bool>,

    ///
    /// The number of instances for which to reserve capacity.
    ///
    /// Valid range: 1 - 1000
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

    ///
    /// Indicates the type of instance launches that the Capacity Reservation accepts. The options 			include:
    ///
    /// open - The Capacity Reservation automatically matches all instances that have matching attributes (instance type, platform, 				and Availability Zone). Instances that have matching attributes run in the Capacity Reservation automatically without specifying 				any additional parameters.                        targeted - The Capacity Reservation only accepts instances that have matching attributes 					(instance type, platform, and Availability Zone), and explicitly target the 					Capacity Reservation. This ensures that only permitted instances can use the reserved capacity.
    ///
    /// Default: open
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: open | targeted
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceMatchCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_match_criteria: Option<CapacityReservationInstanceMatchCriteriaEnum>,

    ///
    /// The type of operating system for which to reserve capacity.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Linux with SQL Server Enterprise | Linux with SQL Server Standard | Linux with SQL Server Web | Linux/UNIX | Red Hat Enterprise Linux | RHEL with HA | RHEL with HA and SQL Server Enterprise | RHEL with HA and SQL Server Standard | RHEL with SQL Server Enterprise | RHEL with SQL Server Standard | RHEL with SQL Server Web | SUSE Linux | Windows | Windows with SQL Server | Windows with SQL Server Enterprise | Windows with SQL Server Standard | Windows with SQL Server Web
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstancePlatform")]
    pub instance_platform: CapacityReservationInstancePlatformEnum,

    ///
    /// The instance type for which to reserve capacity. For more information, see Instance types in the Amazon EC2 User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws([a-z-]+)?:outposts:[a-z\d-]+:\d{12}:outpost/op-[a-f0-9]{17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutPostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_post_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the cluster placement group in which 			to create the Capacity Reservation. For more information, see 			 				Capacity Reservations for cluster placement groups in the 			Amazon EC2 User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws([a-z-]+)?:ec2:[a-z\d-]+:\d{12}:placement-group/([^\s].+[^\s]){1,255}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlacementGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_arn: Option<cfn_resources::StrVal>,

    ///
    /// The tags to apply to the Capacity Reservation during launch.
    ///
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<Vec<TagSpecification>>,

    ///
    /// Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one of the following tenancy settings:
    ///
    /// default - The Capacity Reservation is created on hardware that is shared with other AWS accounts.                        dedicated - The Capacity Reservation is created on single-tenant hardware that is dedicated to a single AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dedicated | default
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<CapacityReservationTenancyEnum>,

    #[serde(skip_serializing)]
    pub att_availability_zone: CfnCapacityReservationavailabilityzone,

    #[serde(skip_serializing)]
    pub att_id: CfnCapacityReservationid,

    #[serde(skip_serializing)]
    pub att_instance_type: CfnCapacityReservationinstancetype,

    #[serde(skip_serializing)]
    pub att_tenancy: CfnCapacityReservationtenancy,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CapacityReservationEndDateTypeEnum {
    /// limited
    #[serde(rename = "limited")]
    Limited,

    /// unlimited
    #[serde(rename = "unlimited")]
    Unlimited,
}

impl Default for CapacityReservationEndDateTypeEnum {
    fn default() -> Self {
        CapacityReservationEndDateTypeEnum::Limited
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CapacityReservationInstanceMatchCriteriaEnum {
    /// open
    #[serde(rename = "open")]
    Open,

    /// targeted
    #[serde(rename = "targeted")]
    Targeted,
}

impl Default for CapacityReservationInstanceMatchCriteriaEnum {
    fn default() -> Self {
        CapacityReservationInstanceMatchCriteriaEnum::Open
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CapacityReservationInstancePlatformEnum {
    /// Linux with SQL Server Enterprise
    #[serde(rename = "Linux with SQL Server Enterprise")]
    Linuxwithsqlserverenterprise,

    /// Linux with SQL Server Standard
    #[serde(rename = "Linux with SQL Server Standard")]
    Linuxwithsqlserverstandard,

    /// Linux with SQL Server Web
    #[serde(rename = "Linux with SQL Server Web")]
    Linuxwithsqlserverweb,

    /// Linux/UNIX
    #[serde(rename = "Linux/UNIX")]
    Linuxunix,

    /// Red Hat Enterprise Linux
    #[serde(rename = "Red Hat Enterprise Linux")]
    Redhatenterpriselinux,

    /// RHEL with HA
    #[serde(rename = "RHEL with HA")]
    Rhelwithha,

    /// RHEL with HA and SQL Server Enterprise
    #[serde(rename = "RHEL with HA and SQL Server Enterprise")]
    Rhelwithhaandsqlserverenterprise,

    /// RHEL with HA and SQL Server Standard
    #[serde(rename = "RHEL with HA and SQL Server Standard")]
    Rhelwithhaandsqlserverstandard,

    /// RHEL with SQL Server Enterprise
    #[serde(rename = "RHEL with SQL Server Enterprise")]
    Rhelwithsqlserverenterprise,

    /// RHEL with SQL Server Standard
    #[serde(rename = "RHEL with SQL Server Standard")]
    Rhelwithsqlserverstandard,

    /// RHEL with SQL Server Web
    #[serde(rename = "RHEL with SQL Server Web")]
    Rhelwithsqlserverweb,

    /// SUSE Linux
    #[serde(rename = "SUSE Linux")]
    Suselinux,

    /// Windows
    #[serde(rename = "Windows")]
    Windows,

    /// Windows with SQL Server
    #[serde(rename = "Windows with SQL Server")]
    Windowswithsqlserver,

    /// Windows with SQL Server Enterprise
    #[serde(rename = "Windows with SQL Server Enterprise")]
    Windowswithsqlserverenterprise,

    /// Windows with SQL Server Standard
    #[serde(rename = "Windows with SQL Server Standard")]
    Windowswithsqlserverstandard,

    /// Windows with SQL Server Web
    #[serde(rename = "Windows with SQL Server Web")]
    Windowswithsqlserverweb,
}

impl Default for CapacityReservationInstancePlatformEnum {
    fn default() -> Self {
        CapacityReservationInstancePlatformEnum::Linuxwithsqlserverenterprise
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CapacityReservationTenancyEnum {
    /// dedicated
    #[serde(rename = "dedicated")]
    Dedicated,

    /// default
    #[serde(rename = "default")]
    Default,
}

impl Default for CapacityReservationTenancyEnum {
    fn default() -> Self {
        CapacityReservationTenancyEnum::Dedicated
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCapacityReservationavailabilityzone;
impl CfnCapacityReservationavailabilityzone {
    pub fn att_name(&self) -> &'static str {
        r#"AvailabilityZone"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCapacityReservationid;
impl CfnCapacityReservationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCapacityReservationinstancetype;
impl CfnCapacityReservationinstancetype {
    pub fn att_name(&self) -> &'static str {
        r#"InstanceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCapacityReservationtenancy;
impl CfnCapacityReservationtenancy {
    pub fn att_name(&self) -> &'static str {
        r#"Tenancy"#
    }
}

impl cfn_resources::CfnResource for CfnCapacityReservation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::CapacityReservation"
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

/// An array of key-value pairs to apply to this resource.
///
/// For more information, see Tag.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TagSpecification {
    ///
    /// The type of resource to tag. Specify capacity-reservation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: capacity-reservation | capacity-reservation-fleet | carrier-gateway | client-vpn-endpoint | coip-pool | customer-gateway | dedicated-host | dhcp-options | egress-only-internet-gateway | elastic-gpu | elastic-ip | export-image-task | export-instance-task | fleet | fpga-image | host-reservation | image | import-image-task | import-snapshot-task | instance | instance-event-window | internet-gateway | ipam | ipam-pool | ipam-resource-discovery | ipam-resource-discovery-association | ipam-scope | ipv4pool-ec2 | ipv6pool-ec2 | key-pair | launch-template | local-gateway | local-gateway-route-table | local-gateway-route-table-virtual-interface-group-association | local-gateway-route-table-vpc-association | local-gateway-virtual-interface | local-gateway-virtual-interface-group | natgateway | network-acl | network-insights-access-scope | network-insights-access-scope-analysis | network-insights-analysis | network-insights-path | network-interface | placement-group | prefix-list | replace-root-volume-task | reserved-instances | route-table | security-group | security-group-rule | snapshot | spot-fleet-request | spot-instances-request | subnet | subnet-cidr-reservation | traffic-mirror-filter | traffic-mirror-filter-rule | traffic-mirror-session | traffic-mirror-target | transit-gateway | transit-gateway-attachment | transit-gateway-connect-peer | transit-gateway-multicast-domain | transit-gateway-policy-table | transit-gateway-route-table | transit-gateway-route-table-announcement | verified-access-endpoint | verified-access-group | verified-access-instance | verified-access-policy | verified-access-trust-provider | volume | vpc | vpc-block-public-access-exclusion | vpc-endpoint | vpc-endpoint-connection | vpc-endpoint-connection-device-type | vpc-endpoint-service | vpc-endpoint-service-permission | vpc-flow-log | vpc-peering-connection | vpn-connection | vpn-connection-device-type | vpn-gateway
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<TagSpecificationResourceTypeEnum>,

    ///
    /// The tags to apply to the resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
