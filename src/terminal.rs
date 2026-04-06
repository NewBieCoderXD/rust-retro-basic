use strum::{Display, EnumString};

#[derive(Debug,EnumString, Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
pub enum ExpectedNode{
  Number,
  Identifier,
  Expression,
  BooleanExpression,
  Equal,
}