use cpp_type::CppType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CppOperator {
  /// (type) a
  Conversion(CppType),
  /// a = b
  Assignment,
  /// a + b
  Addition,
  /// a - b
  Subtraction,
  /// +a
  UnaryPlus,
  /// -a
  UnaryMinus,
  /// a * b
  Multiplication,
  /// a / b
  Division,
  /// a % b
  Modulo,
  /// ++a
  PrefixIncrement,
  /// a++
  PostfixIncrement,
  /// --a
  PrefixDecrement,
  /// a--
  PostfixDecrement,
  /// a == b
  EqualTo,
  /// a != b
  NotEqualTo,
  /// a > b
  GreaterThan,
  /// a < b
  LessThan,
  /// a >= b
  GreaterThanOrEqualTo,
  /// a <= b
  LessThanOrEqualTo,
  /// !a
  LogicalNot,
  /// a && b
  LogicalAnd,
  /// a || b
  LogicalOr,
  /// ~a
  BitwiseNot,
  /// a & b
  BitwiseAnd,
  /// a | b
  BitwiseOr,
  /// a ^ b
  BitwiseXor,
  /// a << b
  BitwiseLeftShift,
  /// a >> b
  BitwiseRightShift,

  /// a += b
  AdditionAssignment,
  /// a -= b
  SubtractionAssignment,
  /// a *= b
  MultiplicationAssignment,
  /// a /= b
  DivisionAssignment,
  /// a %= b
  ModuloAssignment,
  /// a &= b
  BitwiseAndAssignment,
  /// a |= b
  BitwiseOrAssignment,
  /// a ^= b
  BitwiseXorAssignment,
  /// a <<= b
  BitwiseLeftShiftAssignment,
  /// a >>= b
  BitwiseRightShiftAssignment,
  /// a[b]
  Subscript,
  /// *a
  Indirection,
  /// &a
  AddressOf,
  /// a->b
  StructureDereference,
  /// a->*b
  PointerToMember,
  /// a(a1, a2)
  FunctionCall,
  /// a, b
  Comma,
  /// new type
  New,
  /// new type[n]
  NewArray,
  /// delete a
  Delete,
  /// delete[] a
  DeleteArray,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OperatorInfo {
  pub function_name_suffix: Option<&'static str>,
  pub arguments_count: i32,
  pub allows_variable_arguments: bool,
}

impl CppOperator {
  pub fn info(&self) -> OperatorInfo {
    use self::CppOperator::*;
    fn oi(suffix: &'static str, count: i32) -> OperatorInfo {
      OperatorInfo {
        function_name_suffix: Some(suffix),
        arguments_count: count,
        allows_variable_arguments: false,
      }
    }

    match *self {
      Conversion(..) => {
        OperatorInfo {
          function_name_suffix: None,
          arguments_count: 1,
          allows_variable_arguments: false,
        }
      }
      Assignment => oi("=", 2),
      Addition => oi("+", 2),
      Subtraction => oi("-", 2),
      UnaryPlus => oi("+", 1),
      UnaryMinus => oi("-", 1),
      Multiplication => oi("*", 2),
      Division => oi("/", 2),
      Modulo => oi("%", 2),
      PrefixIncrement => oi("++", 1),
      PostfixIncrement => oi("++", 2),
      PrefixDecrement => oi("--", 1),
      PostfixDecrement => oi("--", 2),
      EqualTo => oi("==", 2),
      NotEqualTo => oi("!=", 2),
      GreaterThan => oi(">", 2),
      LessThan => oi("<", 2),
      GreaterThanOrEqualTo => oi(">=", 2),
      LessThanOrEqualTo => oi("<=", 2),
      LogicalNot => oi("!", 1),
      LogicalAnd => oi("&&", 2),
      LogicalOr => oi("||", 2),
      BitwiseNot => oi("~", 1),
      BitwiseAnd => oi("&", 2),
      BitwiseOr => oi("|", 2),
      BitwiseXor => oi("^", 2),
      BitwiseLeftShift => oi("<<", 2),
      BitwiseRightShift => oi(">>", 2),
      AdditionAssignment => oi("+=", 2),
      SubtractionAssignment => oi("-=", 2),
      MultiplicationAssignment => oi("*=", 2),
      DivisionAssignment => oi("/=", 2),
      ModuloAssignment => oi("%=", 2),
      BitwiseAndAssignment => oi("&=", 2),
      BitwiseOrAssignment => oi("|=", 2),
      BitwiseXorAssignment => oi("^=", 2),
      BitwiseLeftShiftAssignment => oi("<<=", 2),
      BitwiseRightShiftAssignment => oi(">>=", 2),
      Subscript => oi("[]", 2),
      Indirection => oi("*", 1),
      AddressOf => oi("&", 1),
      StructureDereference => oi("->", 1),
      PointerToMember => oi("->*", 2),
      FunctionCall => {
        OperatorInfo {
          function_name_suffix: Some("()"),
          arguments_count: 0,
          allows_variable_arguments: true,
        }
      }
      Comma => oi(",", 2),
      New => oi("new", 2),
      NewArray => oi("new[]", 2),
      Delete => oi("delete", 2),
      DeleteArray => oi("delete[]", 2),
    }
  }

  pub fn c_name(&self) -> &'static str {
    unimplemented!()
  }
}