/// Specifies a route calculator resource in your AWS account.
///
/// You can send requests to a route calculator resource to estimate travel time,       distance, and get directions. A route calculator sources traffic and road network data       from your chosen data provider.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub calculator_name: cfn_resources::StrVal,

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
    pub data_source: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<RouteCalculatorPricingPlanEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnRouteCalculatorarn,

    #[serde(skip_serializing)]
    pub att_calculator_arn: CfnRouteCalculatorcalculatorarn,

    #[serde(skip_serializing)]
    pub att_create_time: CfnRouteCalculatorcreatetime,

    #[serde(skip_serializing)]
    pub att_update_time: CfnRouteCalculatorupdatetime,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRouteCalculatorarn;
impl CfnRouteCalculatorarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRouteCalculatorcalculatorarn;
impl CfnRouteCalculatorcalculatorarn {
    pub fn att_name(&self) -> &'static str {
        r#"CalculatorArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRouteCalculatorcreatetime;
impl CfnRouteCalculatorcreatetime {
    pub fn att_name(&self) -> &'static str {
        r#"CreateTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRouteCalculatorupdatetime;
impl CfnRouteCalculatorupdatetime {
    pub fn att_name(&self) -> &'static str {
        r#"UpdateTime"#
    }
}

impl cfn_resources::CfnResource for CfnRouteCalculator {
    fn type_string(&self) -> &'static str {
        "AWS::Location::RouteCalculator"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.calculator_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'calculator_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.calculator_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'calculator_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
