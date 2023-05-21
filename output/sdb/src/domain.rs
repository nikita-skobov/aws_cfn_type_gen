

/// Use the AWS::SDB::Domain resource to declare a SimpleDB domain.     When you specify AWS::SDB::Domain as an argument in a Ref function,      AWS CloudFormation returns the value of the DomainName.
#[derive(Default, serde::Serialize)]
pub struct CfnDomain {


    /// 
    /// Information about the SimpleDB domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}