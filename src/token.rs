
#[derive(Debug)]
pub enum TokenCompare{
  Equal,
  LessThan,
  MoreThan
}

#[derive(Debug)]
pub enum Token{
  Number(u16),
  Iden(String),
  Compare(TokenCompare)
}