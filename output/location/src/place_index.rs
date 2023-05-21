

/// Specifies a place index resource in your AWS account. Use a place index resource to       geocode addresses and other text queries by using the         SearchPlaceIndexForText operation, and reverse geocode coordinates by       using the SearchPlaceIndexForPosition operation, and enable autosuggestions       by using the SearchPlaceIndexForSuggestions operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPlaceIndex {


    /// 
    /// Specifies the geospatial data provider for the new place index.
    /// 
    /// NoteThis field is case-sensitive. Enter the valid values as shown. For example,         entering HERE returns an error.
    /// 
    /// Valid values include:
    /// 
    /// Esri – For additional information about Esri's coverage in your region of interest, see Esri details on geocoding coverage.               Grab – Grab provides place index functionality for Southeast           Asia. For additional information about GrabMaps' coverage, see GrabMaps countries and areas covered.               Here – For additional information about HERE             Technologies' coverage in your region of interest, see HERE details on goecoding coverage.         ImportantIf you specify HERE Technologies (Here) as the data provider,             you may not store results for locations in Japan. For more information, see             the AWS Service               Terms for Amazon Location Service.
    /// 
    /// For additional information , see Data         providers on the Amazon Location Service Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSource")]
    pub data_source: String,


    /// 
    /// Specifies the data storage option requesting Places.
    /// 
    /// Required: No
    ///
    /// Type: DataSourceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSourceConfiguration")]
    pub data_source_configuration: Option<DataSourceConfiguration>,


    /// 
    /// The optional description for the place index resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the place index resource.
    /// 
    /// Requirements:
    /// 
    /// Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods           (.), and underscores (_).               Must be a unique place index resource name.               No spaces allowed. For example, ExamplePlaceIndex.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[-._\w]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// No longer used. If included, the only allowed value is       RequestBasedUsage.
    /// 
    /// Allowed Values: RequestBasedUsage
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: Option<PlaceIndexPricingPlanEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PlaceIndexPricingPlanEnum {

    /// RequestBasedUsage
    #[serde(rename = "RequestBasedUsage")]
    Requestbasedusage,

}

impl Default for PlaceIndexPricingPlanEnum {
    fn default() -> Self {
        PlaceIndexPricingPlanEnum::Requestbasedusage
    }
}


impl cfn_resources::CfnResource for CfnPlaceIndex {
    fn type_string() -> &'static str {
        "AWS::Location::PlaceIndex"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the data storage option requesting Places.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSourceConfiguration {


    /// 
    /// Specifies how the results of an operation will be stored by the caller.
    /// 
    /// Valid values include:
    /// 
    /// SingleUse specifies that the results won't be stored.                Storage specifies that the result can be cached or stored in a           database.
    /// 
    /// Default value: SingleUse
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SingleUse | Storage
    ///
    /// Update requires: Replacement
    #[serde(rename = "IntendedUse")]
    pub intended_use: Option<DataSourceConfigurationIntendedUseEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DataSourceConfigurationIntendedUseEnum {

    /// SingleUse
    #[serde(rename = "SingleUse")]
    Singleuse,

    /// Storage
    #[serde(rename = "Storage")]
    Storage,

}

impl Default for DataSourceConfigurationIntendedUseEnum {
    fn default() -> Self {
        DataSourceConfigurationIntendedUseEnum::Singleuse
    }
}

