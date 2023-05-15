
pub mod cfn_cidr_collection {

#[derive(serde::Serialize, Default)]
pub struct CfnCidrCollection {
    /// A complex type that contains information about the list of CIDR locations.
    #[serde(rename = "Locations")]
    pub locations: Option<Vec<Location>>,
    /// A unique name for the CIDR collection.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "CidrList")]
    pub cidr_list: Vec<String>,
    #[serde(rename = "LocationName")]
    pub location_name: String,

}


}

pub mod cfn_dnssec {

#[derive(serde::Serialize, Default)]
pub struct CfnDNSSEC {
    /// The unique string (ID) used to identify a hosted zone.
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,

}



}

pub mod cfn_health_check {

#[derive(serde::Serialize, Default)]
pub struct CfnHealthCheck {
    /// A complex type that contains information about the health check.
    #[serde(rename = "HealthCheckConfig")]
    pub health_check_config: (),
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "HealthCheckTags")]
    pub health_check_tags: Option<Vec<HealthCheckTag>>,

}


#[derive(serde::Serialize, Default)]
pub struct HealthCheckTag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmIdentifier {
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_hosted_zone {

#[derive(serde::Serialize, Default)]
pub struct CfnHostedZone {
    /// A complex type that contains information about a configuration for DNS query logging.
    #[serde(rename = "QueryLoggingConfig")]
    pub query_logging_config: Option<QueryLoggingConfig>,
    /// A complex type that contains an optional comment.
    /// 
    /// If you don't want to specify a comment, omit the HostedZoneConfig and Comment elements.
    #[serde(rename = "HostedZoneConfig")]
    pub hosted_zone_config: Option<HostedZoneConfig>,
    /// A complex type that contains information about the VPCs that are associated with the specified hosted zone.
    #[serde(rename = "VPCs")]
    pub vpcs: Option<Vec<VPC>>,
    /// The name of the domain. Specify a fully qualified domain name, for example, www.example.com. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that Route 53 treats www.example.com (without a trailing dot) and www.example.com. (with a trailing dot) as identical.
    /// 
    /// If you're creating a public hosted zone, this is the name you have registered with your DNS registrar. If your domain name is registered with a registrar other than Route 53, change the name servers for your domain to the set of NameServers that are returned by the Fn::GetAtt intrinsic function.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Adds, edits, or deletes tags for a health check or a hosted zone.
    /// 
    /// For information about using tags for cost allocation, see Using Cost Allocation Tags in the AWS Billing and Cost Management User Guide.
    #[serde(rename = "HostedZoneTags")]
    pub hosted_zone_tags: Option<Vec<HostedZoneTag>>,

}


#[derive(serde::Serialize, Default)]
pub struct VPC {
    #[serde(rename = "VPCId")]
    pub vpcid: String,
    #[serde(rename = "VPCRegion")]
    pub vpcregion: String,

}

#[derive(serde::Serialize, Default)]
pub struct HostedZoneConfig {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HostedZoneTag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryLoggingConfig {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: String,

}


}

pub mod cfn_key_signing_key {

#[derive(serde::Serialize, Default)]
pub struct CfnKeySigningKey {
    /// The Amazon resource name (ARN) for a customer managed key (CMK) in AWS Key Management Service (KMS). The KeyManagementServiceArn must be unique for each key signing key (KSK) in a single hosted zone.
    #[serde(rename = "KeyManagementServiceArn")]
    pub key_management_service_arn: String,
    /// A string specifying the initial status of the key signing key (KSK). You can set the value to ACTIVE or INACTIVE.
    #[serde(rename = "Status")]
    pub status: String,
    /// An alphanumeric string used to identify a key signing key (KSK). Name must be unique for each key signing key in the same hosted zone.
    #[serde(rename = "Name")]
    pub name: String,
    /// The unique string (ID) used to identify a hosted zone.
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,

}



}

pub mod cfn_record_set {

#[derive(serde::Serialize, Default)]
pub struct CfnRecordSet {
    /// No documentation provided by AWS
    #[serde(rename = "SetIdentifier")]
    pub set_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "GeoLocation")]
    pub geo_location: Option<GeoLocation>,
    /// No documentation provided by AWS
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Failover")]
    pub failover: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AliasTarget")]
    pub alias_target: Option<AliasTarget>,
    /// No documentation provided by AWS
    #[serde(rename = "HostedZoneName")]
    pub hosted_zone_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckId")]
    pub health_check_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceRecords")]
    pub resource_records: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrRoutingConfig")]
    pub cidr_routing_config: Option<CidrRoutingConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "MultiValueAnswer")]
    pub multi_value_answer: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct GeoLocation {
    #[serde(rename = "SubdivisionCode")]
    pub subdivision_code: Option<String>,
    #[serde(rename = "ContinentCode")]
    pub continent_code: Option<String>,
    #[serde(rename = "CountryCode")]
    pub country_code: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AliasTarget {
    #[serde(rename = "DNSName")]
    pub dnsname: String,
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,
    #[serde(rename = "EvaluateTargetHealth")]
    pub evaluate_target_health: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CidrRoutingConfig {
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    #[serde(rename = "LocationName")]
    pub location_name: String,

}


}

pub mod cfn_record_set_group {

#[derive(serde::Serialize, Default)]
pub struct CfnRecordSetGroup {
    /// List of RecordSet
    #[serde(rename = "RecordSets")]
    pub record_sets: Option<Vec<RecordSet>>,
    /// No documentation provided by AWS
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostedZoneName")]
    pub hosted_zone_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AliasTarget {
    #[serde(rename = "DNSName")]
    pub dnsname: String,
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,
    #[serde(rename = "EvaluateTargetHealth")]
    pub evaluate_target_health: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CidrRoutingConfig {
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    #[serde(rename = "LocationName")]
    pub location_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct RecordSet {
    #[serde(rename = "AliasTarget")]
    pub alias_target: Option<AliasTarget>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "HealthCheckId")]
    pub health_check_id: Option<String>,
    #[serde(rename = "ResourceRecords")]
    pub resource_records: Option<Vec<String>>,
    #[serde(rename = "MultiValueAnswer")]
    pub multi_value_answer: Option<bool>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "GeoLocation")]
    pub geo_location: Option<GeoLocation>,
    #[serde(rename = "SetIdentifier")]
    pub set_identifier: Option<String>,
    #[serde(rename = "Failover")]
    pub failover: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "CidrRoutingConfig")]
    pub cidr_routing_config: Option<CidrRoutingConfig>,
    #[serde(rename = "HostedZoneName")]
    pub hosted_zone_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeoLocation {
    #[serde(rename = "SubdivisionCode")]
    pub subdivision_code: Option<String>,
    #[serde(rename = "ContinentCode")]
    pub continent_code: Option<String>,
    #[serde(rename = "CountryCode")]
    pub country_code: Option<String>,

}


}
