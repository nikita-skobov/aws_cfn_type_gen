/// Creates an asset from an existing asset model. For more information, see Creating assets in the       AWS IoT SiteWise User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAsset {
    ///
    /// A description for the asset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_description: Option<cfn_resources::StrVal>,

    ///
    /// A list of asset hierarchies that each contain a hierarchyLogicalId. A hierarchy specifies allowed parent/child asset relationships.
    ///
    /// Required: No
    ///
    /// Type: List of AssetHierarchy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetHierarchies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_hierarchies: Option<Vec<AssetHierarchy>>,

    ///
    /// The ID of the asset model from which to create the asset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetModelId")]
    pub asset_model_id: cfn_resources::StrVal,

    ///
    /// A unique, friendly name for the asset.
    ///
    /// The maximum length is 256 characters with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetName")]
    pub asset_name: cfn_resources::StrVal,

    ///
    /// The list of asset properties for the asset.
    ///
    /// This object doesn't include properties that you define in composite models. You can find    composite model properties in the assetCompositeModels object.
    ///
    /// Required: No
    ///
    /// Type: List of AssetProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_properties: Option<Vec<AssetProperty>>,

    ///
    /// A list of key-value pairs that contain metadata for the asset. For more information, see       Tagging your AWS IoT SiteWise resources in the       AWS IoT SiteWise User Guide.
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
    pub att_asset_arn: CfnAssetassetarn,

    #[serde(skip_serializing)]
    pub att_asset_id: CfnAssetassetid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAssetassetarn;
impl CfnAssetassetarn {
    pub fn att_name(&self) -> &'static str {
        r#"AssetArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAssetassetid;
impl CfnAssetassetid {
    pub fn att_name(&self) -> &'static str {
        r#"AssetId"#
    }
}

impl cfn_resources::CfnResource for CfnAsset {
    fn type_string(&self) -> &'static str {
        "AWS::IoTSiteWise::Asset"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes an asset hierarchy that contains a childAssetId and hierarchyLogicalId.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AssetHierarchy {
    ///
    /// The Id of the child asset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChildAssetId")]
    pub child_asset_id: cfn_resources::StrVal,

    ///
    /// The LogicalID of the hierarchy. This ID is a hierarchyLogicalId.
    ///
    /// The maximum length is 256 characters, with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalId")]
    pub logical_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AssetHierarchy {
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

/// Contains asset property information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AssetProperty {
    ///
    /// The property alias that identifies the property, such as an OPC-UA server data stream path     (for example, /company/windfarm/3/turbine/7/temperature). For more information, see     Mapping industrial data streams to asset properties in the       AWS IoT SiteWise User Guide.
    ///
    /// The property alias must have 1-1000 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<cfn_resources::StrVal>,

    ///
    /// The LogicalID of the asset property.
    ///
    /// The maximum length is 256 characters, with the pattern [^\u0000-\u001F\u007F]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalId")]
    pub logical_id: cfn_resources::StrVal,

    ///
    /// The MQTT notification state (ENABLED or DISABLED) for this asset property.       When the notification state is ENABLED, AWS IoT SiteWise publishes property value       updates to a unique MQTT topic. For more information, see Interacting with other services in the AWS IoT SiteWise User Guide.
    ///
    /// If you omit this parameter, the notification state is set to DISABLED.
    ///
    /// NoteYou must use all caps for the NotificationState parameter. If you use lower case letters, you         will receive a schema validation error.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_state: Option<cfn_resources::StrVal>,

    ///
    /// The unit (such as Newtons or RPM) of the asset property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AssetProperty {
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
#[serde(default)]
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
