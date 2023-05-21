pub trait CfnResource {
    /// returns a string like 'AWS::CloudFront::Distribution'
    fn type_string() -> &'static str;

    fn properties(&self) -> serde_json::Value;
}
