

/// The AWS::RDS::DBProxyEndpoint resource creates or updates a DB proxy endpoint. You can use custom proxy endpoints to access a proxy through a different       VPC than the proxy's default VPC.
///
/// For more information about RDS Proxy, see                AWS::RDS::DBProxy.
#[derive(Default, serde::Serialize)]
pub struct CfnDBProxyEndpoint {


    /// 
    /// An optional set of key-value pairs to associate arbitrary data of your choosing with the proxy.
    /// 
    /// Required: No
    ///
    /// Type: List of TagFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagFormat>>,


    /// 
    /// The VPC subnet IDs for the DB proxy endpoint that you create. You can specify a     different set of subnet IDs than for the original DB proxy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Vec<String>,


    /// 
    /// The name of the DB proxy endpoint to create.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyEndpointName")]
    pub dbproxy_endpoint_name: String,


    /// 
    /// A value that indicates whether the DB proxy endpoint can be used for read/write or read-only operations.
    /// 
    /// Valid Values: READ_WRITE | READ_ONLY
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetRole")]
    pub target_role: Option<String>,


    /// 
    /// The name of the DB proxy associated with the DB proxy endpoint that you create.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,


    /// 
    /// The VPC security group IDs for the DB proxy endpoint that you create. You can     specify a different set of security group IDs than for the original DB proxy.     The default is the default security group for the VPC.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,

}


/// Metadata assigned to a DB proxy endpoint consisting of a key-value pair.
#[derive(Default, serde::Serialize)]
pub struct TagFormat {


    /// 
    /// Metadata assigned to a DB instance consisting of a key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// A value is the optional value of the tag. The string value can be 1-256 Unicode       characters in length and can't be prefixed with aws:. The string can contain only the       set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex:       "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}
