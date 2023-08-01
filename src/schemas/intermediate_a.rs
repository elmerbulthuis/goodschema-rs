use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaNode {
    super_node_id: Option<String>,
    deprecated: bool,
    title: String,
    description: String,
    examples: Vec<Value>,
    types: Vec<TypeEnum>,
    compounds: Vec<CompoundEnum>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub nodes: HashMap<String, SchemaNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TypeEnum {
    Null(NullType),
    Any(AnyType),
    Never(NeverType),
    Boolean(BooleanType),
    Number(NumberType),
    String(StringType),
    Tuple(TupleType),
    Array(ArrayType),
    Interface(InterfaceType),
    Record(RecordType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CompoundEnum {
    OneOf(OneOfCompound),
    AnyOf(AnyOfCompound),
    AllOf(AllOfCompound),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NullType {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnyType {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NeverType {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BooleanType {
    options: Option<Vec<bool>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumberType {
    // numberType: PropertiesNumberType,
    options: Option<Vec<usize>>,
    minimum_inclusive: Option<usize>,
    minimum_exclusive: Option<usize>,
    maximum_inclusive: Option<usize>,
    maximum_exclusive: Option<usize>,
    multiple_of: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    options: Option<Vec<String>>,
    minimum_length: Option<usize>,
    maximum_length: Option<usize>,
    value_pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TupleType {
    item_type_node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    minimum_items: Option<usize>,
    maximum_items: Option<usize>,
    unique_items: Option<usize>,
    item_type_node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceType {
    required_properties: Vec<String>,
    property_type_node_ids: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordType {
    required_properties: Vec<String>,
    minimum_properties: Option<usize>,
    maximum_properties: Option<usize>,
    property_type_node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OneOfCompound {
    type_node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnyOfCompound {
    type_node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllOfCompound {
    type_node_ids: Vec<String>,
}
