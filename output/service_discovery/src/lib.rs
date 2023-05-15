
pub mod cfn_http_namespace {

#[derive(serde::Serialize, Default)]
pub struct CfnHttpNamespace {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnInstance {
    /// No documentation provided by AWS
    #[serde(rename = "ServiceId")]
    pub service_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceAttributes")]
    pub instance_attributes: (),

}



}

pub mod cfn_private_dns_namespace {

#[derive(serde::Serialize, Default)]
pub struct CfnPrivateDnsNamespace {
    /// No documentation provided by AWS
    #[serde(rename = "Vpc")]
    pub vpc: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Properties {
    #[serde(rename = "DnsProperties")]
    pub dns_properties: Option<PrivateDnsPropertiesMutable>,

}

#[derive(serde::Serialize, Default)]
pub struct SOA {
    #[serde(rename = "TTL")]
    pub ttl: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct PrivateDnsPropertiesMutable {
    #[serde(rename = "SOA")]
    pub soa: Option<SOA>,

}


}

pub mod cfn_public_dns_namespace {

#[derive(serde::Serialize, Default)]
pub struct CfnPublicDnsNamespace {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct PublicDnsPropertiesMutable {
    #[serde(rename = "SOA")]
    pub soa: Option<SOA>,

}

#[derive(serde::Serialize, Default)]
pub struct Properties {
    #[serde(rename = "DnsProperties")]
    pub dns_properties: Option<PublicDnsPropertiesMutable>,

}

#[derive(serde::Serialize, Default)]
pub struct SOA {
    #[serde(rename = "TTL")]
    pub ttl: Option<f64>,

}


}

pub mod cfn_service {

#[derive(serde::Serialize, Default)]
pub struct CfnService {
    /// No documentation provided by AWS
    #[serde(rename = "DnsConfig")]
    pub dns_config: Option<DnsConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckCustomConfig")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "NamespaceId")]
    pub namespace_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckConfig")]
    pub health_check_config: Option<HealthCheckConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct HealthCheckConfig {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "FailureThreshold")]
    pub failure_threshold: Option<f64>,
    #[serde(rename = "ResourcePath")]
    pub resource_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DnsConfig {
    #[serde(rename = "RoutingPolicy")]
    pub routing_policy: Option<String>,
    #[serde(rename = "NamespaceId")]
    pub namespace_id: Option<String>,
    #[serde(rename = "DnsRecords")]
    pub dns_records: Vec<DnsRecord>,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheckCustomConfig {
    #[serde(rename = "FailureThreshold")]
    pub failure_threshold: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct DnsRecord {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "TTL")]
    pub ttl: f64,

}


}
