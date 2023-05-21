

/// The AWS::AppSync::GraphQLSchema resource is used for your AWS AppSync GraphQL     schema that controls the data model for your API. Schema files are text written in Schema Definition Language     (SDL) format. For more information about schema authoring, see Designing a GraphQL API in the        AWS AppSync Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGraphQLSchema {


    /// 
    /// The AWS AppSync GraphQL API identifier to which you want to apply this schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,


    /// 
    /// The text representation of a GraphQL schema in SDL format.
    /// 
    /// For more information about using the Ref function, see Ref.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<String>,


    /// 
    /// The location of a GraphQL schema file in an Amazon S3 bucket. Use this if you want to provision     with the schema living in Amazon S3 rather than embedding it in your CloudFormation     template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionS3Location")]
    pub definition_s3_location: Option<String>,

}



impl cfn_resources::CfnResource for CfnGraphQLSchema {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::GraphQLSchema"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}