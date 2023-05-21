

/// Creates a new instance profile. For information about instance profiles, see Using       instance profiles.
///
/// For information about the number of instance profiles you can create, see IAM object quotas in the IAM User       Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnInstanceProfile {


    /// 
    /// The name of the instance profile to create.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceProfileName")]
    pub instance_profile_name: Option<String>,


    /// 
    /// The name of the role to associate with the instance profile. Only one role can be     assigned to an EC2 instance at a time, and all applications on the instance share the same     role and permissions.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Vec<String>,


    /// 
    /// The path to the instance profile. For more information about paths, see IAM         Identifiers in the IAM User Guide.
    /// 
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: (\u002F)|(\u002F[\u0021-\u007E]+\u002F)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    pub path: Option<String>,

}
