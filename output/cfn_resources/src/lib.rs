pub trait CfnResource {
    /// returns a string like 'AWS::CloudFront::Distribution'
    fn type_string() -> &'static str;

    fn properties(self) -> serde_json::Value;

    /// returns Some(string) if there is a validation error.
    fn validate(&self) -> Option<String> {
        None
    }

    /// validate first by the default validation function,
    /// and then add optional extra check via passing your own external validation function.
    fn validate_extern(&self, cb: fn(&Self) -> Option<String> ) -> Option<String> {
        let default_validation = self.validate();
        if default_validation.is_some() {
            return default_validation;
        }
        cb(self)
    }

    /// like validate_extern, but does not use the default validation. This method
    /// only validates via your custom validation function.
    fn validate_override(&self, cb: fn(&Self) -> Option<String> ) -> Option<String> {
        cb(self)
    }
}
