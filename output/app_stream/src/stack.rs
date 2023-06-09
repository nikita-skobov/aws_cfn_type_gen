/// The AWS::AppStream::Stack resource creates a stack to start streaming applications to Amazon AppStream 2.0 users. A stack consists of an associated fleet, user access policies, and storage configurations.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnStack {
    ///
    /// The list of virtual private cloud (VPC) interface endpoint objects. Users of the stack can connect to AppStream 2.0 only through the specified endpoints.
    ///
    /// Required: No
    ///
    /// Type: List of AccessEndpoint
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,

    ///
    /// The persistent application settings for users of the stack. When these settings are enabled, changes that users make to applications and Windows settings are automatically saved after each session and applied to the next session.
    ///
    /// Required: No
    ///
    /// Type: ApplicationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettings>,

    ///
    /// The stack attributes to delete.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributesToDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_delete: Option<Vec<String>>,

    ///
    /// This parameter has been deprecated.
    ///
    /// Deletes the storage connectors currently enabled for the stack.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteStorageConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_storage_connectors: Option<bool>,

    ///
    /// The description to display.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The stack name to display.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<cfn_resources::StrVal>,

    ///
    /// The domains where AppStream 2.0 streaming sessions can be embedded in an iframe. You must approve the domains that you want to host embedded AppStream 2.0 streaming sessions.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmbedHostDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_host_domains: Option<Vec<String>>,

    ///
    /// The URL that users are redirected to after they click the Send Feedback link. If no URL is specified, no Send Feedback link is displayed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeedbackURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_url: Option<cfn_resources::StrVal>,

    ///
    /// The name of the stack.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,100}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The URL that users are redirected to after their streaming session ends.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<cfn_resources::StrVal>,

    ///
    /// The storage connectors to enable.
    ///
    /// Required: No
    ///
    /// Type: List of StorageConnector
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageConnectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_connectors: Option<Vec<StorageConnector>>,

    /// The streaming protocol that you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
    ///
    /// Required: No
    ///
    /// Type: StreamingExperienceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamingExperienceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_experience_settings: Option<StreamingExperienceSettings>,

    ///
    /// An array of key-value pairs.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The actions that are enabled or disabled for users during their streaming sessions. By default, these actions are enabled.
    ///
    /// Required: No
    ///
    /// Type: List of UserSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSetting>>,
}

impl cfn_resources::CfnResource for CfnStack {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::Stack"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.access_endpoints {
            if the_val.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_endpoints'. {} is greater than 4",
                    the_val.len()
                ));
            }
        }

        self.application_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.display_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'display_name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.embed_host_domains {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'embed_host_domains'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.feedback_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'feedback_url'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.redirect_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'redirect_url'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        self.streaming_experience_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an interface VPC endpoint (interface endpoint) that lets you create a private connection between the virtual private cloud (VPC) that you specify and AppStream 2.0. When you specify an interface endpoint for a stack, users of the stack can connect to AppStream 2.0 only through that endpoint. When you specify an interface endpoint for an image builder, administrators can connect to the image builder only through that endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessEndpoint {
    ///
    /// The type of interface endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: STREAMING
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointType")]
    pub endpoint_type: AccessEndpointEndpointTypeEnum,

    ///
    /// The identifier (ID) of the VPC in which the interface endpoint is used.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpceId")]
    pub vpce_id: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AccessEndpointEndpointTypeEnum {
    /// STREAMING
    #[serde(rename = "STREAMING")]
    Streaming,
}

impl Default for AccessEndpointEndpointTypeEnum {
    fn default() -> Self {
        AccessEndpointEndpointTypeEnum::Streaming
    }
}

impl cfn_resources::CfnResource for AccessEndpoint {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.vpce_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpce_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The persistent application settings for users of a stack.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ApplicationSettings {
    ///
    /// Enables or disables persistent application settings for users during their streaming sessions.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The path prefix for the S3 bucket where users’ persistent application settings are stored. You can allow the same persistent application settings to be used across multiple stacks by specifying the same settings group for each stack.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SettingsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_group: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ApplicationSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.settings_group {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'settings_group'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A connector that enables persistent storage for users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StorageConnector {
    ///
    /// The type of storage connector.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GOOGLE_DRIVE | HOMEFOLDERS | ONE_DRIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorType")]
    pub connector_type: StorageConnectorConnectorTypeEnum,

    ///
    /// The names of the domains for the account.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,

    ///
    /// The ARN of the storage connector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StorageConnectorConnectorTypeEnum {
    /// GOOGLE_DRIVE
    #[serde(rename = "GOOGLE_DRIVE")]
    Googledrive,

    /// HOMEFOLDERS
    #[serde(rename = "HOMEFOLDERS")]
    Homefolders,

    /// ONE_DRIVE
    #[serde(rename = "ONE_DRIVE")]
    Onedrive,
}

impl Default for StorageConnectorConnectorTypeEnum {
    fn default() -> Self {
        StorageConnectorConnectorTypeEnum::Googledrive
    }
}

impl cfn_resources::CfnResource for StorageConnector {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.domains {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'domains'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The streaming protocol that you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StreamingExperienceSettings {
    /// The preferred protocol that you want to use while streaming your application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: TCP | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_protocol: Option<StreamingExperienceSettingsPreferredProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StreamingExperienceSettingsPreferredProtocolEnum {
    /// TCP
    #[serde(rename = "TCP")]
    Tcp,

    /// UDP
    #[serde(rename = "UDP")]
    Udp,
}

impl Default for StreamingExperienceSettingsPreferredProtocolEnum {
    fn default() -> Self {
        StreamingExperienceSettingsPreferredProtocolEnum::Tcp
    }
}

impl cfn_resources::CfnResource for StreamingExperienceSettings {
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

/// Specifies an action and whether the action is enabled or disabled for users during their streaming sessions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserSetting {
    ///
    /// The action that is enabled or disabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CLIPBOARD_COPY_FROM_LOCAL_DEVICE | CLIPBOARD_COPY_TO_LOCAL_DEVICE | DOMAIN_PASSWORD_SIGNIN | DOMAIN_SMART_CARD_SIGNIN | FILE_DOWNLOAD | FILE_UPLOAD | PRINTING_TO_LOCAL_DEVICE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: UserSettingActionEnum,

    ///
    /// Indicates whether the action is enabled or disabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permission")]
    pub permission: UserSettingPermissionEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserSettingActionEnum {
    /// CLIPBOARD_COPY_FROM_LOCAL_DEVICE
    #[serde(rename = "CLIPBOARD_COPY_FROM_LOCAL_DEVICE")]
    Clipboardcopyfromlocaldevice,

    /// CLIPBOARD_COPY_TO_LOCAL_DEVICE
    #[serde(rename = "CLIPBOARD_COPY_TO_LOCAL_DEVICE")]
    Clipboardcopytolocaldevice,

    /// DOMAIN_PASSWORD_SIGNIN
    #[serde(rename = "DOMAIN_PASSWORD_SIGNIN")]
    Domainpasswordsignin,

    /// DOMAIN_SMART_CARD_SIGNIN
    #[serde(rename = "DOMAIN_SMART_CARD_SIGNIN")]
    Domainsmartcardsignin,

    /// FILE_DOWNLOAD
    #[serde(rename = "FILE_DOWNLOAD")]
    Filedownload,

    /// FILE_UPLOAD
    #[serde(rename = "FILE_UPLOAD")]
    Fileupload,

    /// PRINTING_TO_LOCAL_DEVICE
    #[serde(rename = "PRINTING_TO_LOCAL_DEVICE")]
    Printingtolocaldevice,
}

impl Default for UserSettingActionEnum {
    fn default() -> Self {
        UserSettingActionEnum::Clipboardcopyfromlocaldevice
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserSettingPermissionEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for UserSettingPermissionEnum {
    fn default() -> Self {
        UserSettingPermissionEnum::Disabled
    }
}

impl cfn_resources::CfnResource for UserSetting {
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
