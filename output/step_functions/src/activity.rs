

/// An activity is a task that you write in any programming language and host on any machine     that has access to AWS Step Functions. Activities must poll Step Functions using the       GetActivityTask API action and respond using SendTask* API     actions. This function lets Step Functions know the existence of your activity and returns     an identifier for use in a state machine and when polling from the activity.
///
/// For information about creating an activity, see Creating an     Activity State Machine in the AWS Step Functions Developer       Guide and CreateActivity     in the AWS Step Functions API Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnActivity {


    /// 
    /// The list of tags to add to a resource.
    /// 
    /// Tags may only contain Unicode letters, digits, white space, or these symbols: _ . : / = + - @.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,


    /// 
    /// The name of the activity.
    /// 
    /// A name must not contain:
    /// 
    /// white space               brackets < > { } [ ]                       wildcard characters ? *                       special characters " # % \ ^ | ~ ` $ & , ; : /                       control characters (U+0000-001F, U+007F-009F)
    /// 
    /// To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// The TagsEntry property specifies tags to identify an     activity.
#[derive(Default, serde::Serialize)]
pub struct TagsEntry {


    /// 
    /// The value for a key-value pair in a tag entry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key for a key-value pair in a tag entry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}