

/// The AWS::SageMaker::NotebookInstanceLifecycleConfig resource creates       shell scripts that run when you create and/or start a notebook instance. For information       about notebook instance lifecycle configurations, see Customize a Notebook         Instance in the Amazon SageMaker Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNotebookInstanceLifecycleConfig {


    /// 
    /// The name of the lifecycle configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: Option<String>,


    /// 
    /// A shell script that runs only once, when you create a notebook instance. The shell       script must be a base64-encoded string.
    /// 
    /// Required: No
    ///
    /// Type: List of NotebookInstanceLifecycleHook
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnCreate")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,


    /// 
    /// A shell script that runs every time you start a notebook instance, including when you       create the notebook instance. The shell script must be a base64-encoded string.
    /// 
    /// Required: No
    ///
    /// Type: List of NotebookInstanceLifecycleHook
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnStart")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,

}



impl cfn_resources::CfnResource for CfnNotebookInstanceLifecycleConfig {
    fn type_string() -> &'static str {
        "AWS::SageMaker::NotebookInstanceLifecycleConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the notebook instance lifecycle configuration script. Each lifecycle       configuration script has a limit of 16384 characters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotebookInstanceLifecycleHook {


    /// 
    /// A base64-encoded string that contains a shell script for a notebook instance lifecycle       configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16384
    ///
    /// Pattern: [\S\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: Option<String>,

}


