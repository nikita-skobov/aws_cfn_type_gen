

/// The AWS::GuardDuty::Detector resource specifies a new detector. A detector is an object that          represents the service. A detector is          required for to become operational.
///
/// Make sure you use either DataSources or          Features in a one request, and not both.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDetector {


    /// 
    /// Describes which data sources will be enabled for the detector.
    /// 
    /// Required: No
    ///
    /// Type: CFNDataSourceConfigurations
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSources")]
    pub data_sources: Option<CFNDataSourceConfigurations>,


    /// 
    /// Specifies whether the detector is to be enabled on creation.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: bool,


    /// 
    /// A list of features that will be configured for the detector.
    /// 
    /// Required: No
    ///
    /// Type: List of FeatureConfigurations
    ///
    /// Update requires: No interruption
    #[serde(rename = "Features")]
    pub features: Option<Vec<FeatureConfigurations>>,


    /// 
    /// Specifies how frequently updated findings are exported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FIFTEEN_MINUTES | ONE_HOUR | SIX_HOURS
    ///
    /// Update requires: No interruption
    #[serde(rename = "FindingPublishingFrequency")]
    pub finding_publishing_frequency: Option<DetectorFindingPublishingFrequencyEnum>,


    /// 
    /// Specifies tags added to a new detector resource. Each tag consists of a key and an          optional value, both of which you define.
    /// 
    /// Currently, support is available only for creating and deleting a tag. No support       exists for updating the tags.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DetectorFindingPublishingFrequencyEnum {

    /// FIFTEEN_MINUTES
    #[serde(rename = "FIFTEEN_MINUTES")]
    Fifteenminutes,

    /// ONE_HOUR
    #[serde(rename = "ONE_HOUR")]
    Onehour,

    /// SIX_HOURS
    #[serde(rename = "SIX_HOURS")]
    Sixhours,

}

impl Default for DetectorFindingPublishingFrequencyEnum {
    fn default() -> Self {
        DetectorFindingPublishingFrequencyEnum::Fifteenminutes
    }
}


impl cfn_resources::CfnResource for CfnDetector {
    fn type_string() -> &'static str {
        "AWS::GuardDuty::Detector"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.data_sources.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes whether S3 data event logs, Kubernetes audit logs, or Malware Protection          will be enabled as a data source when the detector is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNDataSourceConfigurations {


    /// 
    /// Describes which Kubernetes data sources are enabled for a detector.
    /// 
    /// Required: No
    ///
    /// Type: CFNKubernetesConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kubernetes")]
    pub kubernetes: Option<CFNKubernetesConfiguration>,


    /// 
    /// Describes whether Malware Protection will be enabled as a data source.
    /// 
    /// Required: No
    ///
    /// Type: CFNMalwareProtectionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "MalwareProtection")]
    pub malware_protection: Option<CFNMalwareProtectionConfiguration>,


    /// 
    /// Describes whether S3 data event logs are enabled as a data source.
    /// 
    /// Required: No
    ///
    /// Type: CFNS3LogsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<CFNS3LogsConfiguration>,

}



impl cfn_resources::CfnResource for CFNDataSourceConfigurations {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.kubernetes.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.malware_protection.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.s3_logs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes which optional data sources are enabled for a detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNKubernetesAuditLogsConfiguration {


    /// 
    /// Describes whether Kubernetes audit logs are enabled as a data source for the          detector.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,

}



impl cfn_resources::CfnResource for CFNKubernetesAuditLogsConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes which Kubernetes protection data sources are enabled for the          detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNKubernetesConfiguration {


    /// 
    /// Describes whether Kubernetes audit logs are enabled as a data source for the          detector.
    /// 
    /// Required: No
    ///
    /// Type: CFNKubernetesAuditLogsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditLogs")]
    pub audit_logs: Option<CFNKubernetesAuditLogsConfiguration>,

}



impl cfn_resources::CfnResource for CFNKubernetesConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.audit_logs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes whether Malware Protection will be enabled as a data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNMalwareProtectionConfiguration {


    /// 
    /// Describes the configuration of Malware Protection for EC2 instances with          findings.
    /// 
    /// Required: No
    ///
    /// Type: CFNScanEc2InstanceWithFindingsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScanEc2InstanceWithFindings")]
    pub scan_ec2_instance_with_findings: Option<CFNScanEc2InstanceWithFindingsConfiguration>,

}



impl cfn_resources::CfnResource for CFNMalwareProtectionConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.scan_ec2_instance_with_findings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes whether S3 data event logs will be enabled as a data source when the          detector is created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNS3LogsConfiguration {


    /// 
    /// The status of S3 data event logs as a data source.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,

}



impl cfn_resources::CfnResource for CFNS3LogsConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes whether Malware Protection for EC2 instances with findings will be          enabled as a data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CFNScanEc2InstanceWithFindingsConfiguration {


    /// 
    /// Describes the configuration for scanning EBS volumes as data source.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsVolumes")]
    pub ebs_volumes: Option<bool>,

}



impl cfn_resources::CfnResource for CFNScanEc2InstanceWithFindingsConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes the additional configuration for a feature. If you want to specify any additional          configuration for your feature, it is required to provide the Name and Status          for that additional configuration. For more information, see          DetectorAdditionalConfiguration.
///
/// If you're providing additional configuration, ensure to provide the corresponding       FeatureConfigurations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FeatureAdditionalConfiguration {


    /// 
    /// Name of the additional configuration of a feature.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Status of the additional configuration of a feature.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}



impl cfn_resources::CfnResource for FeatureAdditionalConfiguration {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes the configuration for a feature.
///
/// Although the Required field associated with the following properties specifies          No, if you provide information for Name, you will need to          provide the information for Status too. For information about the available feature configurations, see          DetectorFeatureConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FeatureConfigurations {


    /// 
    /// Additional configuration of the feature.
    /// 
    /// Required: No
    ///
    /// Type: List of FeatureAdditionalConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalConfiguration")]
    pub additional_configuration: Option<Vec<FeatureAdditionalConfiguration>>,


    /// 
    /// Name of the feature.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Status of the feature.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,

}



impl cfn_resources::CfnResource for FeatureConfigurations {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}