
pub mod cfn_detector {

#[derive(serde::Serialize, Default)]
pub struct CfnDetector {
    /// No documentation provided by AWS
    #[serde(rename = "DataSources")]
    pub data_sources: Option<CFNDataSourceConfigurations>,
    /// List of FeatureConfigurations
    #[serde(rename = "Features")]
    pub features: Option<Vec<FeatureConfigurations>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Enable")]
    pub enable: bool,
    /// No documentation provided by AWS
    #[serde(rename = "FindingPublishingFrequency")]
    pub finding_publishing_frequency: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct CFNDataSourceConfigurations {
    #[serde(rename = "MalwareProtection")]
    pub malware_protection: Option<CFNMalwareProtectionConfiguration>,
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<CFNS3LogsConfiguration>,
    #[serde(rename = "Kubernetes")]
    pub kubernetes: Option<CFNKubernetesConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CFNMalwareProtectionConfiguration {
    #[serde(rename = "ScanEc2InstanceWithFindings")]
    pub scan_ec2_instance_with_findings: Option<CFNScanEc2InstanceWithFindingsConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct CFNScanEc2InstanceWithFindingsConfiguration {
    #[serde(rename = "EbsVolumes")]
    pub ebs_volumes: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct FeatureAdditionalConfiguration {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FeatureConfigurations {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "AdditionalConfiguration")]
    pub additional_configuration: Option<Vec<FeatureAdditionalConfiguration>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CFNKubernetesAuditLogsConfiguration {
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CFNS3LogsConfiguration {
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CFNKubernetesConfiguration {
    #[serde(rename = "AuditLogs")]
    pub audit_logs: Option<CFNKubernetesAuditLogsConfiguration>,

}


}

pub mod cfn_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnFilter {
    /// No documentation provided by AWS
    #[serde(rename = "Rank")]
    pub rank: usize,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: String,
    /// No documentation provided by AWS
    #[serde(rename = "FindingCriteria")]
    pub finding_criteria: FindingCriteria,
    /// No documentation provided by AWS
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Condition {
    #[serde(rename = "Lte")]
    pub lte: Option<usize>,
    #[serde(rename = "Gte")]
    pub gte: Option<usize>,
    #[serde(rename = "Lt")]
    pub lt: Option<usize>,
    #[serde(rename = "Neq")]
    pub neq: Option<Vec<String>>,
    #[serde(rename = "LessThan")]
    pub less_than: Option<usize>,
    #[serde(rename = "GreaterThan")]
    pub greater_than: Option<usize>,
    #[serde(rename = "LessThanOrEqual")]
    pub less_than_or_equal: Option<usize>,
    #[serde(rename = "Eq")]
    pub eq: Option<Vec<String>>,
    #[serde(rename = "Equals")]
    pub equals: Option<Vec<String>>,
    #[serde(rename = "GreaterThanOrEqual")]
    pub greater_than_or_equal: Option<usize>,
    #[serde(rename = "Gt")]
    pub gt: Option<usize>,
    #[serde(rename = "NotEquals")]
    pub not_equals: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct FindingCriteria {
    #[serde(rename = "Criterion")]
    pub criterion: Option<()>,
    #[serde(rename = "ItemType")]
    pub item_type: Option<Condition>,

}


}

pub mod cfn_ipset {

#[derive(serde::Serialize, Default)]
pub struct CfnIPSet {
    /// No documentation provided by AWS
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// No documentation provided by AWS
    #[serde(rename = "Location")]
    pub location: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Format")]
    pub format: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_master {

#[derive(serde::Serialize, Default)]
pub struct CfnMaster {
    /// No documentation provided by AWS
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "InvitationId")]
    pub invitation_id: Option<String>,

}



}

pub mod cfn_member {

#[derive(serde::Serialize, Default)]
pub struct CfnMember {
    /// No documentation provided by AWS
    #[serde(rename = "Email")]
    pub email: String,
    /// No documentation provided by AWS
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Message")]
    pub message: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableEmailNotification")]
    pub disable_email_notification: Option<bool>,

}



}

pub mod cfn_threat_intel_set {

#[derive(serde::Serialize, Default)]
pub struct CfnThreatIntelSet {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Activate")]
    pub activate: bool,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Format")]
    pub format: String,
    /// No documentation provided by AWS
    #[serde(rename = "Location")]
    pub location: String,
    /// No documentation provided by AWS
    #[serde(rename = "DetectorId")]
    pub detector_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
