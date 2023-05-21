

/// Creates a CIDR collection in the current AWS account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCidrCollection {


    /// 
    /// A complex type that contains information about the list of CIDR locations.
    /// 
    /// Required: No
    ///
    /// Type: List of Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "Locations")]
    pub locations: Option<Vec<Location>>,


    /// 
    /// The name of a CIDR collection.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [0-9A-Za-z_\-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for CfnCidrCollection {
    fn type_string() -> &'static str {
        "AWS::Route53::CidrCollection"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the list of CIDR blocks for a CIDR location.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Location {


    /// 
    /// List of CIDR blocks.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrList")]
    pub cidr_list: Vec<String>,


    /// 
    /// The CIDR collection location name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16
    ///
    /// Pattern: [0-9A-Za-z_\-\*]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationName")]
    pub location_name: String,

}


