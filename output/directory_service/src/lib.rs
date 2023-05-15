
pub mod cfn_microsoft_ad {

#[derive(serde::Serialize, Default)]
pub struct CfnMicrosoftAD {
    /// No documentation provided by AWS
    #[serde(rename = "Edition")]
    pub edition: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSettings")]
    pub vpc_settings: VpcSettings,
    /// No documentation provided by AWS
    #[serde(rename = "CreateAlias")]
    pub create_alias: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ShortName")]
    pub short_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableSso")]
    pub enable_sso: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Password")]
    pub password: String,

}


#[derive(serde::Serialize, Default)]
pub struct VpcSettings {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


}

pub mod cfn_simple_ad {

#[derive(serde::Serialize, Default)]
pub struct CfnSimpleAD {
    /// No documentation provided by AWS
    #[serde(rename = "CreateAlias")]
    pub create_alias: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ShortName")]
    pub short_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableSso")]
    pub enable_sso: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSettings")]
    pub vpc_settings: VpcSettings,
    /// No documentation provided by AWS
    #[serde(rename = "Size")]
    pub size: String,
    /// No documentation provided by AWS
    #[serde(rename = "Password")]
    pub password: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct VpcSettings {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


}
