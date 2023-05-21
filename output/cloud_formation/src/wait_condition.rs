

/// You can use a wait condition for situations like the following:
///
/// For these situations, we recommend that you associate a CreationPolicy attribute with the  wait condition so that you don't have to use a wait condition handle. For more information and an example, see Creating wait conditions   in a template. If you use a CreationPolicy with a wait condition, don't specify any of the wait condition's  properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWaitCondition {


    /// 
    /// The number of success signals that CloudFormation must receive before it continues the stack creation  process. When the wait condition receives the requisite number of success signals, CloudFormation resumes  the creation of the stack. If the wait condition doesn't receive the specified number of success signals before the  Timeout period expires, CloudFormation assumes that the wait condition has failed and rolls the stack  back.
    /// 
    /// Updates aren't supported.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// 
    /// A reference to the wait condition handle used to signal this wait condition. Use the Ref intrinsic  function to specify an AWS::CloudFormation::WaitConditionHandle resource.
    /// 
    /// Anytime you add a WaitCondition resource during a stack update, you must associate the wait  condition with a new WaitConditionHandle resource. Don't reuse an old wait condition handle that has already been  defined in the template. If you reuse a wait condition handle, the wait condition might evaluate old signals from a  previous create or update stack command.
    /// 
    /// Updates aren't supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Handle")]
    pub handle: Option<String>,


    /// 
    /// The length of time (in seconds) to wait for the number of signals that the Count property  specifies. Timeout is a minimum-bound property, meaning the timeout occurs no sooner than the time you  specify, but can occur shortly thereafter. The maximum time that can be specified for this property is 12 hours  (43200 seconds).
    /// 
    /// Updates aren't supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<String>,

}



impl cfn_resources::CfnResource for CfnWaitCondition {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::WaitCondition"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
