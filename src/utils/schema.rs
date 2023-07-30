use serde_json::Value;

pub fn discover_schema_id(node: &Value) -> Option<&str> {
    node.as_object()?.get("$schema")?.as_str()
}
