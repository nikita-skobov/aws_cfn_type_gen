pub trait CfnResource: serde::Serialize {
    /// returns a string like 'AWS::CloudFront::Distribution'
    fn type_string(&self) -> &'static str;

    /// returns Err(string) if there is a validation error.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    /// validate first by the default validation function,
    /// and then add optional extra check via passing your own external validation function.
    fn validate_extern(&self, cb: fn(&Self) -> Result<(), String> ) -> Result<(), String>
        where Self: Sized
    {
        self.validate()?;
        cb(self)
    }

    /// like validate_extern, but does not use the default validation. This method
    /// only validates via your custom validation function.
    fn validate_override(&self, cb: fn(&Self) -> Result<(), String> ) -> Result<(), String>
        where Self: Sized
    {
        cb(self)
    }
}
