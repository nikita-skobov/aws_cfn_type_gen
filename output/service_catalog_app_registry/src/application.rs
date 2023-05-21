

/// Represents a AWS Service Catalog AppRegistry application that is the top-level node in a hierarchy of related    cloud resource abstractions.
#[derive(Default, serde::Serialize)]
pub struct CfnApplication {


    /// 
    /// The description of the application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the application. The name must be unique in the region in which you are creating the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [-.\w]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Key-value pairs you can use to associate with the application.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}
