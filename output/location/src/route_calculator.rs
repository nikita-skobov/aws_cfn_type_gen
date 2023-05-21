

/// Specifies a route calculator resource in your AWS account.
///
/// You can send requests to a route calculator resource to estimate travel time,       distance, and get directions. A route calculator sources traffic and road network data       from your chosen data provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRouteCalculator {


    /// 
    /// The name of the route calculator resource.
    /// 
    /// Requirements:
    /// 
    /// Can use alphanumeric characters (A–Z, a–z, 0–9) , hyphens (-), periods (.),           and underscores (_).               Must be a unique Route calculator resource name.               No spaces allowed. For example, ExampleRouteCalculator.
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
    #[serde(rename = "CalculatorName")]
    pub calculator_name: String,


    /// 
    /// Specifies the data provider of traffic and road network data.
    /// 
    /// NoteThis field is case-sensitive. Enter the valid values as shown. For example,         entering HERE returns an error.
    /// 
    /// Valid values include:
    /// 
    /// Esri – For additional information about Esri's coverage in your region of interest, see Esri details on street networks and traffic coverage.         Route calculators that use Esri as a           data source only calculate routes that are shorter than 400 km.               Grab – Grab provides routing functionality for Southeast Asia.           For additional information about GrabMaps' coverage,           see GrabMaps             countries and areas covered.               Here – For additional information about HERE             Technologies' coverage in your region of interest, see HERE car routing coverage and HERE truck routing coverage.
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
    /// The optional description for the route calculator resource.
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
    pub pricing_plan: Option<RouteCalculatorPricingPlanEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RouteCalculatorPricingPlanEnum {

    /// RequestBasedUsage
    #[serde(rename = "RequestBasedUsage")]
    Requestbasedusage,

}

impl Default for RouteCalculatorPricingPlanEnum {
    fn default() -> Self {
        RouteCalculatorPricingPlanEnum::Requestbasedusage
    }
}


impl cfn_resources::CfnResource for CfnRouteCalculator {
    fn type_string() -> &'static str {
        "AWS::Location::RouteCalculator"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.calculator_name;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'calculator_name'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.calculator_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'calculator_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.description {

        if the_val.len() > 1000 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 1000", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 0", the_val.len()));
        }

        }
        
        Ok(())
    }
}