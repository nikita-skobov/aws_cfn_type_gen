

/// A conformance pack is a collection of AWS Config rules and remediation actions         that can be easily deployed in an account and a region.         ConformancePack creates a service linked role in your account.         The service linked role is created only when the role does not exist in your account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConformancePack {


    /// 
    /// A list of ConformancePackInputParameter objects.
    /// 
    /// Required: No
    ///
    /// Type: List of ConformancePackInputParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConformancePackInputParameters")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,


    /// 
    /// Name of the conformance pack you want to create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,


    /// 
    /// The name of the Amazon S3 bucket where AWS Config stores conformance pack templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: Option<String>,


    /// 
    /// The prefix for the Amazon S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryS3KeyPrefix")]
    pub delivery_s3_key_prefix: Option<String>,


    /// 
    /// A string containing full conformance pack template body. Structure containing the template body with a     minimum length of 1 byte and a maximum length of 51,200 bytes.
    /// 
    /// NoteYou can only use a YAML template with two resource types: config rule (AWS::Config::ConfigRule) and a remediation action (AWS::Config::RemediationConfiguration).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,


    /// 
    /// Location of file containing the template body (s3://bucketname/prefix). The uri must point to the conformance pack template (max size: 300 KB)       that is located in an Amazon S3 bucket.
    /// 
    /// NoteYou must have access to read Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateS3Uri")]
    pub template_s3_uri: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TemplateSSMDocumentDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateSSMDocumentDetails")]
    pub template_ssmdocument_details: Option<TemplateSSMDocumentDetails>,

}



impl cfn_resources::CfnResource for CfnConformancePack {
    fn type_string() -> &'static str {
        "AWS::Config::ConformancePack"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.template_ssmdocument_details.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Input parameters in the form of key-value pairs for the conformance pack, both of which you define. 			Keys can have a maximum character length of 255 characters, and values can have a maximum length of 4096 characters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConformancePackInputParameter {


    /// 
    /// One part of a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,


    /// 
    /// Another part of the key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}



impl cfn_resources::CfnResource for ConformancePackInputParameter {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.parameter_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'parameter_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.parameter_name;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'parameter_name'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.parameter_value;

        if the_val.len() > 4096 as _ {
            return Err(format!("Max validation failed on field 'parameter_value'. {} is greater than 4096", the_val.len()));
        }

        
        let the_val = &self.parameter_value;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'parameter_value'. {} is less than 0", the_val.len()));
        }

        
        Ok(())
    }
}

/// This API allows you to create a conformance pack template with an AWS Systems Manager document (SSM document). 			To deploy a conformance pack using an SSM document, first create an SSM document with conformance pack content, and then provide the DocumentName in the PutConformancePack API. You can also provide the DocumentVersion.
///
/// The TemplateSSMDocumentDetails object contains the name of the SSM document and the version of the SSM document.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TemplateSSMDocumentDetails {


    /// 
    /// The name or Amazon Resource Name (ARN) of the SSM document to use to create a conformance pack. 			If you use the document name, AWS Config checks only your account and AWS Region for the SSM document. If you want to use an SSM document from another Region or account, you must provide the ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9_\-.:/]{3,200}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentName")]
    pub document_name: Option<String>,


    /// 
    /// The version of the SSM document to use to create a conformance pack. By default, AWS Config uses the latest version.
    /// 
    /// NoteThis field is optional.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ([$]LATEST|[$]DEFAULT|^[1-9][0-9]*$)
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,

}



impl cfn_resources::CfnResource for TemplateSSMDocumentDetails {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}