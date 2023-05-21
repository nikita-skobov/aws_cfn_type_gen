

/// The AWS::Lambda::LayerVersion resource creates a Lambda layer from a ZIP archive.
#[derive(Default, serde::Serialize)]
pub struct CfnLayerVersion {


    /// 
    /// The layer's software license. It can be any of the following:
    /// 
    /// An SPDX license identifier. For example,      MIT.               The URL of a license hosted on the internet. For example,      https://opensource.org/licenses/MIT.               The full text of the license.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: Replacement
    #[serde(rename = "LicenseInfo")]
    pub license_info: Option<String>,


    /// 
    /// The function layer archive.
    /// 
    /// Required: Yes
    ///
    /// Type: Content
    ///
    /// Update requires: Replacement
    #[serde(rename = "Content")]
    pub content: Content,


    /// 
    /// The name or Amazon Resource Name (ARN) of the layer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LayerName")]
    pub layer_name: Option<String>,


    /// 
    /// A list of compatible  instruction set architectures.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 2
    ///
    /// Update requires: Replacement
    #[serde(rename = "CompatibleArchitectures")]
    pub compatible_architectures: Option<Vec<String>>,


    /// 
    /// The description of the version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A list of compatible function     runtimes. Used for filtering with ListLayers and ListLayerVersions.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 15
    ///
    /// Update requires: Replacement
    #[serde(rename = "CompatibleRuntimes")]
    pub compatible_runtimes: Option<Vec<String>>,

}


/// A ZIP archive that contains the contents of an Lambda layer.
#[derive(Default, serde::Serialize)]
pub struct Content {


    /// 
    /// For versioned objects, the version of the layer archive object to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,


    /// 
    /// The Amazon S3 bucket of the layer archive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[0-9A-Za-z\.\-_]*(?<!\.)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,


    /// 
    /// The Amazon S3 key of the layer archive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}
