pub static SCHEMA_ID: &str = "https://schema.JsonSchema42.org/jns42-intermediate-a/schema.json";

// @generated by
//     __             _____     _                 ___ ___
//  _ |  |___ ___ ___|   __|___| |_ ___ _____  __| | |_  |
// | |_| |_ -| . |   |__   |  _|   | -_|     ||. |_  |  _|
// |_____|___|___|_|_|_____|___|_|_|___|_|_|_|___| |_|___|
//                                 -- www.JsonSchema42.org

pub type r#NeverType = r#NeverTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#NeverTypeInterface {
    #[serde(rename = "type")]
    pub r#type: r#NeverTypeType,
}
pub type r#OneOf7 = r#ArrayType;
pub type r#MaximumInclusive = r#MaximumInclusiveNumber;
pub type r#MaximumInclusiveNumber = i64;
pub type r#OneOf4 = r#DefsNumberType;
pub type r#BooleanTypeOptions = r#BooleanTypeOptionsArray;
pub type r#BooleanTypeOptionsArray = Vec<r#BooleanTypeOptionsItems>;
pub type r#AllOfCompoundTypeNodeIdsItems = r#AllOfCompoundTypeNodeIdsItemsString;
pub type r#AllOfCompoundTypeNodeIdsItemsString = String;
pub type r#PropertyTypeNodeIdsAdditionalProperties =
    r#PropertyTypeNodeIdsAdditionalPropertiesString;
pub type r#PropertyTypeNodeIdsAdditionalPropertiesString = String;
pub type r#StringTypeOptions = r#StringTypeOptionsArray;
pub type r#StringTypeOptionsArray = Vec<r#StringTypeOptionsItems>;
pub type r#NumberTypeType = r#NumberTypeTypeString;
pub type r#NumberTypeTypeString = String;
pub type r#Compounds = r#CompoundsArray;
pub type r#CompoundsArray = Vec<r#CompoundsItems>;
pub type r#Nodes = r#NodesRecord;
pub type r#NodesRecord = std::collections::HashMap<String, r#NodesAdditionalProperties>;
pub type r#MinimumInclusive = r#MinimumInclusiveNumber;
pub type r#MinimumInclusiveNumber = i64;
pub type r#StringTypeType = r#StringTypeTypeString;
pub type r#StringTypeTypeString = String;
pub type r#Types = r#TypesArray;
pub type r#TypesArray = Vec<r#TypesItems>;
pub type r#OneOf8 = r#InterfaceType;
pub type r#NullTypeType = r#NullTypeTypeString;
pub type r#NullTypeTypeString = String;
pub type r#OneOf5 = r#StringType;
pub type r#PropertyTypeNodeIds = r#PropertyTypeNodeIdsRecord;
pub type r#PropertyTypeNodeIdsRecord =
    std::collections::HashMap<String, r#PropertyTypeNodeIdsAdditionalProperties>;
pub type r#DefsNumberType = r#DefsNumberTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#DefsNumberTypeInterface {
    #[serde(rename = "options")]
    pub r#options: Option<r#NumberTypeOptions>,
    #[serde(rename = "minimumInclusive")]
    pub r#minimum_inclusive: Option<r#MinimumInclusive>,
    #[serde(rename = "multipleOf")]
    pub r#multiple_of: Option<r#MultipleOf>,
    #[serde(rename = "maximumInclusive")]
    pub r#maximum_inclusive: Option<r#MaximumInclusive>,
    #[serde(rename = "numberType")]
    pub r#number_type: Option<r#PropertiesNumberType>,
    #[serde(rename = "maximumExclusive")]
    pub r#maximum_exclusive: Option<r#MaximumExclusive>,
    #[serde(rename = "minimumExclusive")]
    pub r#minimum_exclusive: Option<r#MinimumExclusive>,
    #[serde(rename = "type")]
    pub r#type: r#NumberTypeType,
}
pub type r#TypeUnionOneOf1 = r#AnyType;
pub type r#SchemaJson = r#SchemaJsonInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#SchemaJsonInterface {
    #[serde(rename = "nodes")]
    pub r#nodes: r#Nodes,
    #[serde(rename = "$schema")]
    pub r#schema: Option<r#Schema>,
}
pub type r#ItemTypeNodeIds = r#ItemTypeNodeIdsArray;
pub type r#ItemTypeNodeIdsArray = Vec<r#ItemTypeNodeIdsItems>;
pub type r#CompoundUnionOneOf1 = r#AnyOfCompound;
pub type r#ItemTypeNodeId = r#ItemTypeNodeIdString;
pub type r#ItemTypeNodeIdString = String;
pub type r#RecordTypeRequiredPropertiesItems = r#RecordTypeRequiredPropertiesItemsString;
pub type r#RecordTypeRequiredPropertiesItemsString = String;
pub type r#AnyOfCompoundTypeNodeIds = r#AnyOfCompoundTypeNodeIdsArray;
pub type r#AnyOfCompoundTypeNodeIdsArray = Vec<r#AnyOfCompoundTypeNodeIdsItems>;
pub type r#TypeUnionOneOf2 = r#NeverType;
pub type r#OneOfCompoundTypeNodeIds = r#OneOfCompoundTypeNodeIdsArray;
pub type r#OneOfCompoundTypeNodeIdsArray = Vec<r#OneOfCompoundTypeNodeIdsItems>;
pub type r#StringType = r#StringTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#StringTypeInterface {
    #[serde(rename = "options")]
    pub r#options: Option<r#StringTypeOptions>,
    #[serde(rename = "minimumLength")]
    pub r#minimum_length: Option<r#MinimumLength>,
    #[serde(rename = "type")]
    pub r#type: r#StringTypeType,
    #[serde(rename = "maximumLength")]
    pub r#maximum_length: Option<r#MaximumLength>,
    #[serde(rename = "valuePattern")]
    pub r#value_pattern: Option<r#ValuePattern>,
}
pub type r#InterfaceTypeType = r#InterfaceTypeTypeString;
pub type r#InterfaceTypeTypeString = String;
pub type r#InterfaceTypeRequiredPropertiesItems = r#InterfaceTypeRequiredPropertiesItemsString;
pub type r#InterfaceTypeRequiredPropertiesItemsString = String;
pub type r#NeverTypeType = r#NeverTypeTypeString;
pub type r#NeverTypeTypeString = String;
pub type r#ValuePattern = r#ValuePatternString;
pub type r#ValuePatternString = String;
pub type r#BooleanTypeOptionsItems = r#BooleanTypeOptionsItemsBoolean;
pub type r#BooleanTypeOptionsItemsBoolean = bool;
pub type r#Description = r#DescriptionString;
pub type r#DescriptionString = String;
pub type r#MinimumProperties = r#MinimumPropertiesNumber;
pub type r#MinimumPropertiesNumber = i64;
pub type r#TupleType = r#TupleTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#TupleTypeInterface {
    #[serde(rename = "itemTypeNodeIds")]
    pub r#item_type_node_ids: Option<r#ItemTypeNodeIds>,
    #[serde(rename = "type")]
    pub r#type: r#TupleTypeType,
}
pub type r#MaximumItems = r#MaximumItemsNumber;
pub type r#MaximumItemsNumber = i64;
pub type r#AnyType = r#AnyTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#AnyTypeInterface {
    #[serde(rename = "type")]
    pub r#type: r#AnyTypeType,
}
pub type r#PropertyTypeNodeId = r#PropertyTypeNodeIdString;
pub type r#PropertyTypeNodeIdString = String;
pub type r#OneOfCompoundTypeNodeIdsItems = r#OneOfCompoundTypeNodeIdsItemsString;
pub type r#OneOfCompoundTypeNodeIdsItemsString = String;
pub type r#CompoundUnionOneOf0 = r#OneOfCompound;
pub type r#ItemTypeNodeIdsItems = r#ItemTypeNodeIdsItemsString;
pub type r#ItemTypeNodeIdsItemsString = String;
pub type r#MinimumExclusive = r#MinimumExclusiveNumber;
pub type r#MinimumExclusiveNumber = i64;
pub type r#MaximumProperties = r#MaximumPropertiesNumber;
pub type r#MaximumPropertiesNumber = i64;
pub type r#Schema = r#SchemaString;
pub type r#SchemaString = String;
pub type r#ExamplesItems = r#ExamplesItemsAny;
pub type r#ExamplesItemsAny = serde_json::Value;
pub type r#MaximumLength = r#MaximumLengthNumber;
pub type r#MaximumLengthNumber = i64;
pub type r#MinimumItems = r#MinimumItemsNumber;
pub type r#MinimumItemsNumber = i64;
pub type r#OneOf3 = r#BooleanType;
pub type r#AnyOfCompoundType = r#AnyOfCompoundTypeString;
pub type r#AnyOfCompoundTypeString = String;
pub type r#ArrayTypeType = r#ArrayTypeTypeString;
pub type r#ArrayTypeTypeString = String;
pub type r#CompoundsItems = r#CompoundUnion;
pub type r#OneOf6 = r#TupleType;
pub type r#TypesItems = r#TypeUnion;
pub type r#RecordTypeRequiredProperties = r#RecordTypeRequiredPropertiesArray;
pub type r#RecordTypeRequiredPropertiesArray = Vec<r#RecordTypeRequiredPropertiesItems>;
pub type r#Node = r#NodeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#NodeInterface {
    #[serde(rename = "superNodeId")]
    pub r#super_node_id: Option<r#SuperNodeId>,
    #[serde(rename = "description")]
    pub r#description: r#Description,
    #[serde(rename = "examples")]
    pub r#examples: r#Examples,
    #[serde(rename = "title")]
    pub r#title: r#Title,
    #[serde(rename = "types")]
    pub r#types: r#Types,
    #[serde(rename = "compounds")]
    pub r#compounds: r#Compounds,
    #[serde(rename = "deprecated")]
    pub r#deprecated: r#Deprecated,
}
pub type r#AllOfCompoundTypeNodeIds = r#AllOfCompoundTypeNodeIdsArray;
pub type r#AllOfCompoundTypeNodeIdsArray = Vec<r#AllOfCompoundTypeNodeIdsItems>;
pub type r#NullType = r#NullTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#NullTypeInterface {
    #[serde(rename = "type")]
    pub r#type: r#NullTypeType,
}
pub type r#OneOfCompound = r#OneOfCompoundInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#OneOfCompoundInterface {
    #[serde(rename = "typeNodeIds")]
    pub r#type_node_ids: Option<r#OneOfCompoundTypeNodeIds>,
    #[serde(rename = "type")]
    pub r#type: r#OneOfCompoundType,
}
pub type r#BooleanType = r#BooleanTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#BooleanTypeInterface {
    #[serde(rename = "options")]
    pub r#options: Option<r#BooleanTypeOptions>,
    #[serde(rename = "type")]
    pub r#type: r#BooleanTypeType,
}
pub type r#AllOfCompoundType = r#AllOfCompoundTypeString;
pub type r#AllOfCompoundTypeString = String;
pub type r#MaximumExclusive = r#MaximumExclusiveNumber;
pub type r#MaximumExclusiveNumber = i64;
pub type r#OneOfCompoundType = r#OneOfCompoundTypeString;
pub type r#OneOfCompoundTypeString = String;
pub type r#Title = r#TitleString;
pub type r#TitleString = String;
pub type r#ArrayType = r#ArrayTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#ArrayTypeInterface {
    #[serde(rename = "minimumItems")]
    pub r#minimum_items: Option<r#MinimumItems>,
    #[serde(rename = "maximumItems")]
    pub r#maximum_items: Option<r#MaximumItems>,
    #[serde(rename = "itemTypeNodeId")]
    pub r#item_type_node_id: Option<r#ItemTypeNodeId>,
    #[serde(rename = "type")]
    pub r#type: r#ArrayTypeType,
    #[serde(rename = "uniqueItems")]
    pub r#unique_items: Option<r#UniqueItems>,
}
pub type r#MultipleOf = r#MultipleOfNumber;
pub type r#MultipleOfNumber = i64;
pub type r#OneOf9 = r#RecordType;
pub type r#InterfaceTypeRequiredProperties = r#InterfaceTypeRequiredPropertiesArray;
pub type r#InterfaceTypeRequiredPropertiesArray = Vec<r#InterfaceTypeRequiredPropertiesItems>;
pub type r#Deprecated = r#DeprecatedBoolean;
pub type r#DeprecatedBoolean = bool;
pub type r#AnyOfCompound = r#AnyOfCompoundInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#AnyOfCompoundInterface {
    #[serde(rename = "type")]
    pub r#type: r#AnyOfCompoundType,
    #[serde(rename = "typeNodeIds")]
    pub r#type_node_ids: Option<r#AnyOfCompoundTypeNodeIds>,
}
pub type r#AllOfCompound = r#AllOfCompoundInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#AllOfCompoundInterface {
    #[serde(rename = "type")]
    pub r#type: r#AllOfCompoundType,
    #[serde(rename = "typeNodeIds")]
    pub r#type_node_ids: Option<r#AllOfCompoundTypeNodeIds>,
}
pub type r#AnyTypeType = r#AnyTypeTypeString;
pub type r#AnyTypeTypeString = String;
pub type r#TupleTypeType = r#TupleTypeTypeString;
pub type r#TupleTypeTypeString = String;
pub type r#CompoundUnionOneOf2 = r#AllOfCompound;
pub type r#Examples = r#ExamplesArray;
pub type r#ExamplesArray = Vec<r#ExamplesItems>;
pub type r#RecordType = r#RecordTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#RecordTypeInterface {
    #[serde(rename = "maximumProperties")]
    pub r#maximum_properties: Option<r#MaximumProperties>,
    #[serde(rename = "requiredProperties")]
    pub r#required_properties: Option<r#RecordTypeRequiredProperties>,
    #[serde(rename = "minimumProperties")]
    pub r#minimum_properties: Option<r#MinimumProperties>,
    #[serde(rename = "propertyTypeNodeId")]
    pub r#property_type_node_id: Option<r#PropertyTypeNodeId>,
    #[serde(rename = "type")]
    pub r#type: r#RecordTypeType,
}
pub type r#StringTypeOptionsItems = r#StringTypeOptionsItemsString;
pub type r#StringTypeOptionsItemsString = String;
pub type r#BooleanTypeType = r#BooleanTypeTypeString;
pub type r#BooleanTypeTypeString = String;
pub type r#PropertiesNumberType = r#PropertiesNumberTypeString;
pub type r#PropertiesNumberTypeString = String;
pub type r#MinimumLength = r#MinimumLengthNumber;
pub type r#MinimumLengthNumber = i64;
pub type r#NumberTypeOptionsItems = r#NumberTypeOptionsItemsNumber;
pub type r#NumberTypeOptionsItemsNumber = i64;
pub type r#TypeUnion = r#TypeUnionOneOf;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum r#TypeUnionOneOf {
    r#TypeUnionOneOf0(r#TypeUnionOneOf0),
    r#TypeUnionOneOf1(r#TypeUnionOneOf1),
    r#TypeUnionOneOf2(r#TypeUnionOneOf2),
    r#OneOf3(r#OneOf3),
    r#OneOf4(r#OneOf4),
    r#OneOf5(r#OneOf5),
    r#OneOf6(r#OneOf6),
    r#OneOf7(r#OneOf7),
    r#OneOf8(r#OneOf8),
    r#OneOf9(r#OneOf9),
}
pub type r#RecordTypeType = r#RecordTypeTypeString;
pub type r#RecordTypeTypeString = String;
pub type r#AnyOfCompoundTypeNodeIdsItems = r#AnyOfCompoundTypeNodeIdsItemsString;
pub type r#AnyOfCompoundTypeNodeIdsItemsString = String;
pub type r#NodesAdditionalProperties = r#Node;
pub type r#InterfaceType = r#InterfaceTypeInterface;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct r#InterfaceTypeInterface {
    #[serde(rename = "requiredProperties")]
    pub r#required_properties: Option<r#InterfaceTypeRequiredProperties>,
    #[serde(rename = "type")]
    pub r#type: r#InterfaceTypeType,
    #[serde(rename = "propertyTypeNodeIds")]
    pub r#property_type_node_ids: Option<r#PropertyTypeNodeIds>,
}
pub type r#UniqueItems = r#UniqueItemsBoolean;
pub type r#UniqueItemsBoolean = bool;
pub type r#NumberTypeOptions = r#NumberTypeOptionsArray;
pub type r#NumberTypeOptionsArray = Vec<r#NumberTypeOptionsItems>;
pub type r#SuperNodeId = r#SuperNodeIdString;
pub type r#SuperNodeIdString = String;
pub type r#TypeUnionOneOf0 = r#NullType;
pub type r#CompoundUnion = r#CompoundUnionOneOf;
#[derive(serde :: Serialize, serde :: Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum r#CompoundUnionOneOf {
    r#CompoundUnionOneOf0(r#CompoundUnionOneOf0),
    r#CompoundUnionOneOf1(r#CompoundUnionOneOf1),
    r#CompoundUnionOneOf2(r#CompoundUnionOneOf2),
}
