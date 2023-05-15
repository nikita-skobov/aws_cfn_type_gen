
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// List of Credential
    #[serde(rename = "Credentials")]
    pub credentials: Option<Vec<Credential>>,
    /// No documentation provided by AWS
    #[serde(rename = "Sid")]
    pub sid: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationType")]
    pub application_type: String,
    /// List of Instance
    #[serde(rename = "Instances")]
    pub instances: Option<Vec<Instance>>,
    /// No documentation provided by AWS
    #[serde(rename = "SapInstanceNumber")]
    pub sap_instance_number: Option<String>,
    /// The tags of a SystemsManagerSAP application.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Credential {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "SecretId")]
    pub secret_id: Option<String>,
    #[serde(rename = "CredentialType")]
    pub credential_type: Option<String>,

}
pub type Instance = String;

}
