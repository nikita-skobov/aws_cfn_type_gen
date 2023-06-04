/// The AWS::AppRunner::ObservabilityConfiguration resource is an AWS App Runner resource type that specifies an App Runner    observability configuration.
///
/// App Runner requires this resource when you specify App Runner services and you want to enable non-default observability features.    You can share an observability configuration across multiple services.
///
/// Create multiple revisions of a configuration by specifying this resource multiple times using the same ObservabilityConfigurationName.     App Runner creates multiple resources with incremental ObservabilityConfigurationRevision values. When you specify a service and    configure an observability configuration resource, the service uses the latest active revision of the observability configuration by default. You can    optionally configure the service to use a specific revision.
///
/// The observability configuration resource is designed to configure multiple features (currently one feature, tracing). This resource takes optional    parameters that describe the configuration of these features (currently one parameter, TraceConfiguration). If you don't specify a feature      parameter, App Runner doesn't enable the feature.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnObservabilityConfiguration {
    ///
    /// A name for the observability configuration. When you use it for the first time in an AWS Region, App Runner creates revision number     1 of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.
    ///
    /// NoteThe name DefaultConfiguration is reserved. You can't use it to create a new observability configuration, and you can't create a     revision of it.When you want to use your own observability configuration for your App Runner service, create a configuration with a different name,     and then provide it when you create or update your service.
    ///
    /// If you don't specify a name, AWS CloudFormation generates a name for your observability configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 32
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9\-_]{3,31}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObservabilityConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of metadata items that you can associate with your observability configuration resource. A tag is a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing.
    ///
    /// Required: No
    ///
    /// Type: TraceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "TraceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_configuration: Option<TraceConfiguration>,

    #[serde(skip_serializing)]
    pub att_observability_configuration_arn:
        CfnObservabilityConfigurationobservabilityconfigurationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnObservabilityConfigurationobservabilityconfigurationarn;
impl CfnObservabilityConfigurationobservabilityconfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ObservabilityConfigurationArn"#
    }
}

impl cfn_resources::CfnResource for CfnObservabilityConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::AppRunner::ObservabilityConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.observability_configuration_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!("Max validation failed on field 'observability_configuration_name'. {} is greater than 32", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.observability_configuration_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 4 as _ {
                    return Err(format!("Min validation failed on field 'observability_configuration_name'. {} is less than 4", s.len()));
                }
            }
        }

        self.trace_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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

/// Describes the configuration of the tracing feature within an AWS App Runner observability configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TraceConfiguration {
    ///
    /// The implementation provider chosen for tracing App Runner services.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWSXRAY
    ///
    /// Update requires: Replacement
    #[serde(rename = "Vendor")]
    pub vendor: TraceConfigurationVendorEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TraceConfigurationVendorEnum {
    /// AWSXRAY
    #[serde(rename = "AWSXRAY")]
    Awsxray,
}

impl Default for TraceConfigurationVendorEnum {
    fn default() -> Self {
        TraceConfigurationVendorEnum::Awsxray
    }
}

impl cfn_resources::CfnResource for TraceConfiguration {
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
