

/// Creates a version of the SageMaker image specified by ImageName. The       version represents the Amazon Container Registry (ECR) container image specified by         BaseImage.
#[derive(Default, serde::Serialize)]
pub struct CfnImageVersion {


    /// 
    /// The container image that the SageMaker image version is based on.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of       255.
    /// 
    /// Pattern: .*
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaseImage")]
    pub base_image: String,


    /// 
    /// The name of the parent image.
    /// 
    /// Length Constraints: Minimum length of 1. Maximum length of       63.
    /// 
    /// Pattern:       ^[a-zA-Z0-9]([-.]?[a-zA-Z0-9]){0,62}$
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageName")]
    pub image_name: String,

}
