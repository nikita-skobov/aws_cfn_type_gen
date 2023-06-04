/// Creates a version of the SageMaker image specified by ImageName. The       version represents the Amazon Container Registry (ECR) container image specified by         BaseImage.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub base_image: cfn_resources::StrVal,

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
    pub image_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_container_image: CfnImageVersioncontainerimage,

    #[serde(skip_serializing)]
    pub att_image_arn: CfnImageVersionimagearn,

    #[serde(skip_serializing)]
    pub att_image_version_arn: CfnImageVersionimageversionarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnImageVersioncontainerimage;
impl CfnImageVersioncontainerimage {
    pub fn att_name(&self) -> &'static str {
        r#"ContainerImage"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnImageVersionimagearn;
impl CfnImageVersionimagearn {
    pub fn att_name(&self) -> &'static str {
        r#"ImageArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnImageVersionimageversionarn;
impl CfnImageVersionimageversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"ImageVersionArn"#
    }
}

impl cfn_resources::CfnResource for CfnImageVersion {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::ImageVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
