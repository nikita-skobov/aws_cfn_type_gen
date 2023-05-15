
pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// Description of the Environment
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// KMS key used to encrypt customer data within FinSpace Environment infrastructure
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// Federation mode used with the Environment
    #[serde(rename = "FederationMode")]
    pub federation_mode: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Additional parameters to identify Federation mode
    #[serde(rename = "FederationParameters")]
    pub federation_parameters: Option<FederationParameters>,
    /// Parameters of the first Superuser for the FinSpace Environment
    #[serde(rename = "SuperuserParameters")]
    pub superuser_parameters: Option<SuperuserParameters>,
    /// Name of the Environment
    #[serde(rename = "Name")]
    pub name: String,
    /// ARNs of FinSpace Data Bundles to install
    #[serde(rename = "DataBundles")]
    pub data_bundles: Option<Vec<DataBundleArn>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type DataBundleArn = String;
#[derive(serde::Serialize, Default)]
pub struct FederationParameters {
    #[serde(rename = "FederationURN")]
    pub federation_urn: Option<String>,
    #[serde(rename = "AttributeMap")]
    pub attribute_map: Option<()>,
    #[serde(rename = "SamlMetadataURL")]
    pub saml_metadata_url: Option<String>,
    #[serde(rename = "ApplicationCallBackURL")]
    pub application_call_back_url: Option<String>,
    #[serde(rename = "SamlMetadataDocument")]
    pub saml_metadata_document: Option<String>,
    #[serde(rename = "FederationProviderName")]
    pub federation_provider_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SuperuserParameters {
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,

}


}
