

/// The AWS::AppStream::Stack resource creates a stack to start streaming applications to Amazon AppStream 2.0 users. A stack consists of an associated fleet, user access policies, and storage configurations.
#[derive(Default, serde::Serialize)]
pub struct CfnStack {


    /// The streaming protocol that you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
    ///
    /// Required: No
    ///
    /// Type: StreamingExperienceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamingExperienceSettings")]
    pub streaming_experience_settings: Option<StreamingExperienceSettings>,


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
    pub redirect_url: Option<String>,


    /// 
    /// The stack attributes to delete.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributesToDelete")]
    pub attributes_to_delete: Option<Vec<String>>,


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
    pub description: Option<String>,


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
    pub delete_storage_connectors: Option<bool>,


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
    pub access_endpoints: Option<Vec<AccessEndpoint>>,


    /// 
    /// The actions that are enabled or disabled for users during their streaming sessions. By default, these actions are enabled.
    /// 
    /// Required: No
    ///
    /// Type: List of UserSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSettings")]
    pub user_settings: Option<Vec<UserSetting>>,


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
    pub feedback_url: Option<String>,


    /// 
    /// An array of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


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
    pub name: Option<String>,


    /// 
    /// The storage connectors to enable.
    /// 
    /// Required: No
    ///
    /// Type: List of StorageConnector
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageConnectors")]
    pub storage_connectors: Option<Vec<StorageConnector>>,


    /// 
    /// The persistent application settings for users of the stack. When these settings are enabled, changes that users make to applications and Windows settings are automatically saved after each session and applied to the next session.
    /// 
    /// Required: No
    ///
    /// Type: ApplicationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationSettings")]
    pub application_settings: Option<ApplicationSettings>,


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
    pub embed_host_domains: Option<Vec<String>>,


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
    pub display_name: Option<String>,

}


/// Specifies an action and whether the action is enabled or disabled for users during their streaming sessions.
#[derive(Default, serde::Serialize)]
pub struct UserSetting {


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
    pub permission: String,


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
    pub action: String,

}


/// Describes an interface VPC endpoint (interface endpoint) that lets you create a private connection between the virtual private cloud (VPC) that you specify and AppStream 2.0. When you specify an interface endpoint for a stack, users of the stack can connect to AppStream 2.0 only through that endpoint. When you specify an interface endpoint for an image builder, administrators can connect to the image builder only through that endpoint.
#[derive(Default, serde::Serialize)]
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
    pub endpoint_type: String,


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
    pub vpce_id: String,

}


/// The persistent application settings for users of a stack.
#[derive(Default, serde::Serialize)]
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
    pub settings_group: Option<String>,

}


/// The streaming protocol that you want your stack to prefer. This can be UDP or TCP. Currently, UDP is only supported in the Windows native client.
#[derive(Default, serde::Serialize)]
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
    pub preferred_protocol: Option<String>,

}


/// A connector that enables persistent storage for users.
#[derive(Default, serde::Serialize)]
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
    pub connector_type: String,


    /// 
    /// The ARN of the storage connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,


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
    pub domains: Option<Vec<String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
