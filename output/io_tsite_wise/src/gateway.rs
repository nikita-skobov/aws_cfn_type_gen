/// Creates a gateway, which is a virtual or edge device that delivers industrial data streams       from local servers to AWS IoT SiteWise. For more information, see Ingesting data using a gateway in the       AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGateway {
    ///
    /// A list of gateway capability summaries that each contain a namespace and status. Each    gateway capability defines data sources for the gateway. To retrieve a capability    configuration's definition, use DescribeGatewayCapabilityConfiguration.
    ///
    /// Required: No
    ///
    /// Type: List of GatewayCapabilitySummary
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayCapabilitySummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_capability_summaries: Option<Vec<GatewayCapabilitySummary>>,

    ///
    /// A unique, friendly name for the gateway.
    ///
    /// The maximum length is 256 characters with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayName")]
    pub gateway_name: cfn_resources::StrVal,

    ///
    /// The gateway's platform. You can only specify one platform in a gateway.
    ///
    /// Required: Yes
    ///
    /// Type: GatewayPlatform
    ///
    /// Update requires: Replacement
    #[serde(rename = "GatewayPlatform")]
    pub gateway_platform: GatewayPlatform,

    ///
    /// A list of key-value pairs that contain metadata for the gateway. For more information, see       Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_gateway_id: CfnGatewaygatewayid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGatewaygatewayid;
impl CfnGatewaygatewayid {
    pub fn att_name(&self) -> &'static str {
        r#"GatewayId"#
    }
}

impl cfn_resources::CfnResource for CfnGateway {
    fn type_string(&self) -> &'static str {
        "AWS::IoTSiteWise::Gateway"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.gateway_platform.validate()?;

        Ok(())
    }
}

/// Contains a summary of a gateway capability configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GatewayCapabilitySummary {
    ///
    /// The JSON document that defines the configuration for the gateway capability. For more       information, see Configuring data sources (CLI) in the AWS IoT SiteWise User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapabilityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_configuration: Option<cfn_resources::StrVal>,

    ///
    /// The namespace of the capability configuration.    For example, if you configure OPC-UA    sources from the AWS IoT SiteWise console, your OPC-UA capability configuration has the namespace     iotsitewise:opcuacollector:version, where version is a number such as     1.
    ///
    /// The maximum length is 512 characters with the pattern ^[a-zA-Z]+:[a-zA-Z]+:[0-9]+$.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapabilityNamespace")]
    pub capability_namespace: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for GatewayCapabilitySummary {
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

/// Contains a gateway's platform information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GatewayPlatform {
    ///
    /// A gateway that runs on AWS IoT Greengrass.
    ///
    /// Required: No
    ///
    /// Type: Greengrass
    ///
    /// Update requires: Replacement
    #[serde(rename = "Greengrass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass: Option<Greengrass>,

    ///
    /// A gateway that runs on AWS IoT Greengrass V2.
    ///
    /// Required: No
    ///
    /// Type: GreengrassV2
    ///
    /// Update requires: Replacement
    #[serde(rename = "GreengrassV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greengrass_v2: Option<GreengrassV2>,
}

impl cfn_resources::CfnResource for GatewayPlatform {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.greengrass
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.greengrass_v2
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains details for a gateway that runs on AWS IoT Greengrass. To create a gateway that runs on AWS IoT Greengrass,    you must add the IoT SiteWise connector to a Greengrass group and deploy it. Your Greengrass    group must also have permissions to upload data to AWS IoT SiteWise. For more information, see Ingesting data using a gateway in the      AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Greengrass {
    ///
    /// The ARN of the Greengrass group. For more information about how to find a group's    ARN, see ListGroups and GetGroup in       the AWS IoT Greengrass API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupArn")]
    pub group_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Greengrass {
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

/// Contains details for a gateway that runs on AWS IoT Greengrass V2. To create a gateway that runs on AWS IoT Greengrass    V2, you must deploy the IoT SiteWise Edge component to your gateway device. Your Greengrass     device role must use the AWSIoTSiteWiseEdgeAccess policy. For more    information, see Using AWS IoT SiteWise at the edge in the             AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GreengrassV2 {
    ///
    /// The name of the AWS IoT thing for your AWS IoT Greengrass V2 core device.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreDeviceThingName")]
    pub core_device_thing_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for GreengrassV2 {
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
