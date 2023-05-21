

/// Associates a resource      with an application.      Both the resource and the application can be specified either      by ID or name.
#[derive(Default, serde::Serialize)]
pub struct CfnResourceAssociation {


    /// 
    /// The type      of resource     of which the application will be associated.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: String,


    /// 
    /// The name or ID      of the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Application")]
    pub application: String,


    /// 
    /// The name or ID      of the resource      of which the application      will be associated.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resource")]
    pub resource: String,

}
