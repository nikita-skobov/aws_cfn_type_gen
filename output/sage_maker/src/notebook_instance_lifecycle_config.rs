

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
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::NotebookInstanceLifecycleConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.notebook_instance_lifecycle_config_name {

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'notebook_instance_lifecycle_config_name'. {} is greater than 63", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.on_create {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'on_create'. {} is greater than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.on_start {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'on_start'. {} is greater than 1", the_val.len()));
        }

        }
        
        Ok(())
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



impl cfn_resources::CfnResource for NotebookInstanceLifecycleHook {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.content {

        if the_val.len() > 16384 as _ {
            return Err(format!("Max validation failed on field 'content'. {} is greater than 16384", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.content {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'content'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}