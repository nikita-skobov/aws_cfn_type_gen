

/// A public key that you can use with signed URLs and signed cookies, or with field-level encryption.
#[derive(Default, serde::Serialize)]
pub struct CfnPublicKey {


    /// 
    /// Configuration information about a public key that you can use with signed URLs and signed cookies, or with field-level encryption.
    /// 
    /// Required: Yes
    ///
    /// Type: PublicKeyConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicKeyConfig")]
    pub public_key_config: PublicKeyConfig,

}


/// Configuration information about a public key that you can use with signed URLs and signed cookies, or with field-level encryption.
#[derive(Default, serde::Serialize)]
pub struct PublicKeyConfig {


    /// 
    /// A name to help identify the public key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The public key that you can use with signed URLs and signed cookies, or with field-level encryption.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncodedKey")]
    pub encoded_key: String,


    /// 
    /// A string included in the request to help make sure that the request can't be 			replayed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CallerReference")]
    pub caller_reference: String,


    /// 
    /// A comment to describe the public key. The comment cannot be longer than 128 			characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}
