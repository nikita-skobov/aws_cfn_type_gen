

/// Use the AWS::IoT::Thing resource to declare an AWS IoT thing.
///
/// For information about working with things, see How AWS IoT Works and       Device       Registry for AWS IoT in the AWS IoT Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnThing {


    /// 
    /// A string that contains up to three key value pairs. Maximum length of 800. Duplicates     not allowed.
    /// 
    /// Required: No
    ///
    /// Type: AttributePayload
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributePayload")]
    pub attribute_payload: Option<AttributePayload>,


    /// 
    /// The name of the thing to update.
    /// 
    /// You can't change a thing's name. To change a thing's name, you must create a 			new thing, give it the new name, and then delete the old thing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingName")]
    pub thing_name: Option<String>,

}


/// The AttributePayload property specifies up to three attributes for an AWS IoT as     key-value pairs. AttributePayload is a property of the AWS::IoT::Thing resource.
#[derive(Default, serde::Serialize)]
pub struct AttributePayload {


    /// 
    /// A JSON string containing up to three key-value pair in JSON format. For example:
    /// 
    /// {\"attributes\":{\"string1\":\"string2\"}}
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

}