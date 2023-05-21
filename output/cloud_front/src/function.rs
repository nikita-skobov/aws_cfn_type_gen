

/// Creates a CloudFront function.
///
/// To create a function, you provide the function code and some configuration information 				about the function. The response contains an Amazon Resource Name (ARN) that uniquely 				identifies the function, and the function’s stage.
///
/// By default, when you create a function, it’s in the DEVELOPMENT stage. In this   			stage, you can test the function in the CloudFront console (or with 		  	TestFunction in the CloudFront API).
///
/// When you’re ready to use your function with a CloudFront distribution, publish the 			  function to the LIVE stage. You can do this in the CloudFront console, with         PublishFunction in the CloudFront API, or by updating the         AWS::CloudFront::Function resource with the AutoPublish         property set to true. When the function is published to the         LIVE stage, you can attach it to a distribution’s cache behavior, using the         function’s ARN.
///
/// To automatically publish the function to the LIVE stage when it’s 				created, set the AutoPublish property to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFunction {


    /// 
    /// A name to identify the function.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9-_]{1,64}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// A flag that determines whether to automatically publish the function to the 			LIVE stage when it’s created. To automatically publish to the 			LIVE stage, set this property to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoPublish")]
    pub auto_publish: Option<bool>,


    /// 
    /// Contains configuration information about a CloudFront function.
    /// 
    /// Required: Yes
    ///
    /// Type: FunctionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionConfig")]
    pub function_config: FunctionConfig,


    /// 
    /// The function code. For more information about writing a CloudFront function, see Writing 				function code for CloudFront Functions in the 			Amazon CloudFront Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionCode")]
    pub function_code: String,


    /// 
    /// Contains metadata about a CloudFront function.
    /// 
    /// Required: No
    ///
    /// Type: FunctionMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionMetadata")]
    pub function_metadata: Option<FunctionMetadata>,

}

impl cfn_resources::CfnResource for CfnFunction {
    fn type_string() -> &'static str {
        "AWS::CloudFront::Function"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains metadata about a CloudFront function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FunctionMetadata {


    /// 
    /// The Amazon Resource Name (ARN) of the function. The ARN uniquely identifies the 			function.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionARN")]
    pub function_arn: Option<String>,

}


/// Contains configuration information about a CloudFront function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FunctionConfig {


    /// 
    /// A comment to describe the function.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: String,


    /// 
    /// The function's runtime environment. The only valid value is 				cloudfront-js-1.0.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: cloudfront-js-1.0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    pub runtime: String,

}
