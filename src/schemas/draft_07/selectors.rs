pub trait Selectors {
    fn select_schema(&self) -> Option<&str>;
    fn select_id(&self) -> Option<&str>;
}

impl Selectors for serde_json::Value {
    fn select_schema(&self) -> Option<&str> {
        self.as_object()?.get("$schema")?.as_str()
    }

    fn select_id(&self) -> Option<&str> {
        self.as_object()?.get("$id")?.as_str()
    }
}
