

/// Creates a view that users can query by using the Search       operation. Results from queries that you make using this view include only resources       that match the view's Filters.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnView {


    /// 
    /// A list of fields that provide additional information about the view.
    /// 
    /// Required: No
    ///
    /// Type: List of IncludedProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedProperties")]
    pub included_properties: Option<Vec<IncludedProperty>>,


    /// 
    /// The name of the new view.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9\-]{1,64}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ViewName")]
    pub view_name: String,


    /// 
    /// An array of strings that include search keywords, prefixes, and operators that filter       the results that are returned for queries made using this view. When you use this view       in a Search       operation, the filter string is combined with the search's QueryString       parameter using a logical AND operator.
    /// 
    /// For information about the supported syntax, see Search query         reference for Resource Explorer in the AWS         Resource Explorer User Guide.
    /// 
    /// ImportantThis query string in the context of this operation supports only filter prefixes with optional operators. It doesn't support free-form text. For example, the string           region:us* service:ec2 -tag:stage=prod includes all Amazon EC2 resources in any AWS Region that begin with the         letters us and are not tagged with a key           Stage that has the value prod.
    /// 
    /// Required: No
    ///
    /// Type: Filters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Filters>,


    /// 
    /// Tag key and value pairs that are attached to the view.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}



impl cfn_resources::CfnResource for CfnView {
    fn type_string() -> &'static str {
        "AWS::ResourceExplorer2::View"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information about an additional property that describes a resource, that you can       optionally include in a view.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IncludedProperty {


    /// 
    /// The name of the property that is included in this view.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1011
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}




/// An object with a FilterString that specifies which resources to include       in the results of queries made using this view.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Filters {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterString")]
    pub filter_string: String,

}


