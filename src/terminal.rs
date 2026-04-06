use strum::{Display, EnumString};

#[derive(Debug,EnumString, Display)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
pub enum Terminal{
  Number,
  Identifier,
  Expression,
  BooleanExpression,
  Equal,
}