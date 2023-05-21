

/// The AWS::FinSpace::Environment resource represents an Amazon FinSpace     environment.
#[derive(Default, serde::Serialize)]
pub struct CfnEnvironment {


    /// 
    /// Configuration information when authentication mode is FEDERATED.
    /// 
    /// Required: No
    ///
    /// Type: FederationParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationParameters")]
    pub federation_parameters: Option<FederationParameters>,


    /// 
    /// The description of the FinSpace environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^[a-zA-Z0-9. ]{1,1000}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The authentication mode for the environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FEDERATED | LOCAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "FederationMode")]
    pub federation_mode: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The KMS key id used to encrypt in the FinSpace environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^[a-zA-Z-0-9-:\/]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The name of the FinSpace environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9]+[a-zA-Z0-9-]*[a-zA-Z0-9]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Configuration information for the superuser.
    /// 
    /// Required: No
    ///
    /// Type: SuperuserParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "SuperuserParameters")]
    pub superuser_parameters: Option<SuperuserParameters>,

}


/// Configuration information when authentication mode is FEDERATED.
#[derive(Default, serde::Serialize)]
pub struct FederationParameters {


    /// 
    /// SAML 2.0 Metadata document from identity provider (IdP).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1000
    ///
    /// Maximum: 10000000
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "SamlMetadataDocument")]
    pub saml_metadata_document: Option<String>,


    /// 
    /// The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration    (IdP).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^https?://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationCallBackURL")]
    pub application_call_back_url: Option<String>,


    /// 
    /// Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: ^https?://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]
    ///
    /// Update requires: Replacement
    #[serde(rename = "SamlMetadataURL")]
    pub saml_metadata_url: Option<String>,


    /// 
    /// The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[A-Za-z0-9._\-:\/#\+]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationURN")]
    pub federation_urn: Option<String>,


    /// 
    /// Name of the identity provider (IdP).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^_\p{Z}][\p{L}\p{M}\p{S}\p{N}\p{P}][^_\p{Z}]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "FederationProviderName")]
    pub federation_provider_name: Option<String>,


    /// 
    /// SAML attribute name and value. The name must always be Email and the value should be set to     the attribute definition in which user email is set. For example, name would be Email and     value http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress.     Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.
    /// 
    /// Required: No
    ///
    /// Type: List of AttributeMapItems
    ///
    /// Update requires: Replacement
    #[serde(rename = "AttributeMap")]
    pub attribute_map: Option<Vec<AttributeMapItems>>,

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


/// The AttributeMapItems property type specifies Property description not available. for an AWS::FinSpace::Environment.
#[derive(Default, serde::Serialize)]
pub struct AttributeMapItems {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


/// Configuration information for the superuser.
#[derive(Default, serde::Serialize)]
pub struct SuperuserParameters {


    /// 
    /// The email address of the superuser.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+[.]+[A-Za-z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,


    /// 
    /// The first name of the superuser.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[a-zA-Z0-9]{1,50}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,


    /// 
    /// The last name of the superuser.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[a-zA-Z0-9]{1,50}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,

}