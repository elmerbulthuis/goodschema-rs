use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidatorEnum {
    Boolean(BooleanValidator),
    Integer(IntegerValidator),
    Number(NumberValidator),
    String(StringValidator),
    Array(ArrayValidator),
    Map(MapValidator),
}
