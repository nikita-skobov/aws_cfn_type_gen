

/// Creates an Amazon Personalize schema from the specified schema string. The schema you create    must be in Avro JSON format.
///
/// Amazon Personalize recognizes three schema variants. Each schema is associated with a dataset    type and has a set of required field and keywords. If you are creating a schema for a dataset in a Domain dataset group, you   provide the domain of the Domain dataset group.   You specify a schema when you call CreateDataset.
///
/// For more information on schemas, see    Datasets and schemas.
#[derive(Default, serde::Serialize)]
pub struct CfnSchema {


    /// 
    /// The domain of a schema that you created for a dataset in a Domain dataset group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECOMMERCE | VIDEO_ON_DEMAND
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// 
    /// The schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Schema")]
    pub schema: String,


    /// 
    /// The name of the schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9][a-zA-Z0-9\-_]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}
