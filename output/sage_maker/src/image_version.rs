

/// Creates a version of the SageMaker image specified by ImageName. The       version represents the Amazon Container Registry (ECR) container image specified by         BaseImage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImageVersion {


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

}



impl cfn_resources::CfnResource for CfnImageVersion {
    fn type_string() -> &'static str {
        "AWS::SageMaker::ImageVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
