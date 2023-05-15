
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// Application Description, should be between 1 and 2048 characters.
    #[serde(rename = "ApplicationDescription")]
    pub application_description: Option<String>,
    /// A list of key-value pairs that contain metadata for the application.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ARN of the role that the web application assumes when it interacts with AWS IoT Core. For more info on configuring this attribute, see https://docs.aws.amazon.com/iot/latest/apireference/API_iotfleethub_CreateApplication.html#API_iotfleethub_CreateApplication_RequestSyntax
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Application Name, should be between 1 and 256 characters.
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
