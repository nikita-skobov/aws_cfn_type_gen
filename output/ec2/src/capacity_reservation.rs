

/// Creates a new Capacity Reservation with the specified attributes. For more information,     see Capacity       Reservations in the Amazon EC2 User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnCapacityReservation {


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
    pub instance_match_criteria: Option<String>,


    /// 
    /// The Availability Zone in which to create the Capacity Reservation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,


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
    pub end_date: Option<String>,


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
    pub placement_group_arn: Option<String>,


    /// 
    /// The tags to apply to the Capacity Reservation during launch.
    /// 
    /// Required: No
    ///
    /// Type: List of TagSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,


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
    pub out_post_arn: Option<String>,


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
    pub instance_platform: String,


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
    pub tenancy: Option<String>,


    /// 
    /// Deprecated.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EphemeralStorage")]
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
    pub end_date_type: Option<String>,


    /// 
    /// The instance type for which to reserve capacity. For more information, see Instance types in the Amazon EC2 User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// Indicates whether the Capacity Reservation supports EBS-optimized instances. This optimization provides 			dedicated throughput to Amazon EBS and an optimized configuration stack to provide 			optimal I/O performance. This optimization isn't available with all instance types. 			Additional usage charges apply when using an EBS- optimized instance.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,

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


/// An array of key-value pairs to apply to this resource.
///
/// For more information, see Tag.
#[derive(Default, serde::Serialize)]
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
    pub resource_type: Option<String>,


    /// 
    /// The tags to apply to the resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}
