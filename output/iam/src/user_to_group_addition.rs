

/// Adds the specified user to the specified group.
#[derive(Default, serde::Serialize)]
pub struct CfnUserToGroupAddition {


    /// 
    /// A list of the names of the users that you want to add to the group.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Vec<String>,


    /// 
    /// The name of the group to update.
    /// 
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: String,

}
