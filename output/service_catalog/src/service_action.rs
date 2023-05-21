

/// Creates a self-service action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceAction {


    /// The language code.
    /// 
    /// en - English (default)jp - Japanesezh - Chinese
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptLanguage")]
    pub accept_language: Option<String>,


    /// 
    /// A map that defines the self-service action.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DefinitionParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Vec<DefinitionParameter>,


    /// 
    /// The self-service action definition type. For example, SSM_AUTOMATION.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SSM_AUTOMATION
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionType")]
    pub definition_type: ServiceActionDefinitionTypeEnum,


    /// 
    /// The self-service action description.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The self-service action name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ServiceActionDefinitionTypeEnum {

    /// SSM_AUTOMATION
    #[serde(rename = "SSM_AUTOMATION")]
    Ssmautomation,

}

impl Default for ServiceActionDefinitionTypeEnum {
    fn default() -> Self {
        ServiceActionDefinitionTypeEnum::Ssmautomation
    }
}


impl cfn_resources::CfnResource for CfnServiceAction {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceCatalog::ServiceAction"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 1024", the_val.len()));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The list of parameters in JSON format.    For example: [{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}] or [{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}].
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefinitionParameter {


    /// 
    /// The parameter key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value of the parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for DefinitionParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}