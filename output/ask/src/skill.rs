

/// The Alexa::ASK::Skill resource creates an Alexa skill that enables       customers to access new abilities. For more information about developing a skill, see       the Build Skills with the Alexa Skills Kit developer documentation.
#[derive(Default, serde::Serialize)]
pub struct CfnSkill {


    /// 
    /// Configuration for the skill package that contains the components of the Alexa       skill. Skill packages are retrieved from an Amazon S3 bucket and key and used to create       and update the skill. For more information about the skill package format, see the       Skill Package API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: SkillPackage
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkillPackage")]
    pub skill_package: SkillPackage,


    /// 
    /// The vendor ID associated with the Amazon developer account that will host the       skill. Details for retrieving the vendor ID are in How to get your vendor ID. The provided LWA credentials must be linked to the       developer account associated with this vendor ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VendorId")]
    pub vendor_id: String,


    /// 
    /// Login with Amazon (LWA) configuration used to authenticate with the Alexa service.       Only Login with Amazon clients created through the Amazon Developer Console are supported. The client ID, client secret, and refresh token are       required.
    /// 
    /// Required: Yes
    ///
    /// Type: AuthenticationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: AuthenticationConfiguration,

}


/// The SkillPackage property type contains configuration details for the       skill package that contains the components of the Alexa skill. Skill packages are       retrieved from an Amazon S3 bucket and key and used to create and update the skill. More       details about the skill package format are located in the Skill Package API Reference.
///
/// SkillPackage is a property of the Alexa::ASK::Skill       resource.
#[derive(Default, serde::Serialize)]
pub struct SkillPackage {


    /// 
    /// ARN of the IAM role that grants the Alexa service (alexa-appkit.amazon.com) permission to access the bucket and       retrieve the skill package. This property is optional. If you do not provide it, the bucket       must be publicly accessible or configured with a policy that allows this access.       Otherwise, AWS CloudFormation cannot create the skill.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketRole")]
    pub s3_bucket_role: Option<String>,


    /// 
    /// The name of the Amazon S3 bucket where the .zip file that contains the skill       package is stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// Overrides to the skill package to apply when creating or updating the skill. Values       provided here do not modify the contents of the original skill package. Currently, only       overriding values inside of the skill manifest component of the package is       supported.
    /// 
    /// Required: No
    ///
    /// Type: Overrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: Option<Overrides>,


    /// 
    /// The location and name of the skill package .zip file.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Key")]
    pub s3_key: String,


    /// 
    /// If you have S3 versioning enabled, the version ID of the skill package.zip       file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,

}


/// The Overrides property type provides overrides to the skill package to       apply when creating or updating the skill. Values provided here do not modify the       contents of the original skill package. Currently, only overriding values inside of the       skill manifest component of the package is supported.
///
/// Overrides is a property of the Alexa::ASK::Skill SkillPackage       property type.
#[derive(Default, serde::Serialize)]
pub struct Overrides {


    /// 
    /// Overrides to apply to the skill manifest inside of the skill package. The skill       manifest contains metadata about the skill. For more information, see Skill Manifest Schemas.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Manifest")]
    pub manifest: Option<serde_json::Value>,

}


/// The AuthenticationConfiguration property type specifies the Login with       Amazon (LWA) configuration used to authenticate with the Alexa service. Only Login with       Amazon security profiles created through the Build Skills with the Alexa Skills Kit developer documentation are supported for authentication. A client ID, client secret, and       refresh token are required. You can generate a client ID and client secret by creating a       new security profile on the Amazon Developer Portal or you can retrieve them       from an existing profile. You can then retrieve the refresh token using the Alexa Skills       Kit CLI. For instructions, see util-command       in the ASK CLI Command Reference.
///
/// AuthenticationConfiguration is a property of the         Alexa::ASK::Skill resource.
#[derive(Default, serde::Serialize)]
pub struct AuthenticationConfiguration {


    /// 
    /// Client ID from Login with Amazon (LWA).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// Refresh token from Login with Amazon (LWA). This token is secret.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,


    /// 
    /// Client secret from Login with Amazon (LWA).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,

}
