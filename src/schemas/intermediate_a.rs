use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaNode {
    pub super_node_id: Option<String>,
    pub deprecated: bool,
    pub title: String,
    pub description: String,
    pub examples: Vec<Value>,
    pub types: Vec<TypeEnum>,
    pub compounds: Vec<CompoundEnum>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub nodes: HashMap<String, SchemaNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
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
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
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
    pub options: Option<Vec<bool>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumberType {
    // numberType: PropertiesNumberType,
    pub options: Option<Vec<usize>>,
    pub minimum_inclusive: Option<usize>,
    pub minimum_exclusive: Option<usize>,
    pub maximum_inclusive: Option<usize>,
    pub maximum_exclusive: Option<usize>,
    pub multiple_of: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    pub options: Option<Vec<String>>,
    pub minimum_length: Option<usize>,
    pub maximum_length: Option<usize>,
    pub value_pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TupleType {
    pub item_type_node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    pub minimum_items: Option<usize>,
    pub maximum_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub item_type_node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceType {
    pub required_properties: Vec<String>,
    pub property_type_node_ids: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordType {
    pub required_properties: Vec<String>,
    pub minimum_properties: Option<usize>,
    pub maximum_properties: Option<usize>,
    pub property_type_node_id: String,
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
