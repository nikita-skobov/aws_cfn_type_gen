

/// Use a SecurityConfiguration resource to configure data encryption, Kerberos authentication (available in Amazon EMR release version 5.10.0 and later), and Amazon S3 authorization for EMRFS (available in EMR 5.10.0 and later). You can re-use a security configuration for any number of clusters in your account. For more information and example security configuration JSON objects, see Create a Security Configuration in the Amazon EMR Management Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnSecurityConfiguration {


    /// 
    /// The security configuration details in JSON format.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: serde_json::Value,


    /// 
    /// The name of the security configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10280
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
