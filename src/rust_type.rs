use cpp_type::CppType;
use cpp_ffi_type::CppToFfiTypeConversion;
use utils::JoinWithString;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum RustTypeIndirection {
  None,
  Ptr,
  Ref { lifetime: Option<String>, },
  PtrPtr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RustName {
  pub parts: Vec<String>,
}

impl RustName {
  pub fn new(parts: Vec<String>) -> RustName {
    assert!(parts.len() > 0);
    RustName { parts: parts }
  }

  pub fn crate_name(&self) -> Option<&String> {
    assert!(self.parts.len() > 0);
    if self.parts.len() > 1 {
      Some(&self.parts[0])
    } else {
      None
    }
  }
  pub fn last_name(&self) -> &String {
    self.parts.last().unwrap()
  }
  pub fn full_name(&self, current_crate: Option<&String>) -> String {
    if current_crate.is_some() && self.crate_name().is_some() &&
       current_crate.unwrap() == self.crate_name().unwrap() {
      format!("::{}", self.parts[1..].join("::"))
    } else {
      self.parts.join("::")
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
  Void,
  NonVoid {
    base: RustName,
    generic_arguments: Option<Vec<RustType>>,
    is_const: bool,
    indirection: RustTypeIndirection,
    is_option: bool,
  },
}

impl RustType {
  pub fn caption(&self) -> Option<String> {
    match *self {
      RustType::Void => None,
      RustType::NonVoid { ref base, ref generic_arguments, .. } => {
        let mut name = base.last_name().clone();
        if let &Some(ref args) = generic_arguments {
          name = format!("{}_{}", name, args.iter().map(|x| x.caption().unwrap_or(String::new())).join("_"));
        }
        Some(name)
      }
    }
  }

  pub fn with_lifetime(&self, new_lifetime: String) -> RustType {
    let mut r = self.clone();
    if let RustType::NonVoid { ref mut indirection, .. } = r {
      if let RustTypeIndirection::Ref { ref mut lifetime } = *indirection {
        assert!(lifetime.is_none());
        *lifetime = Some(new_lifetime);
      }
    }
    r
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum RustToCTypeConversion {
  None,
  RefToPtr,
  ValueToPtr,
  QFlagsToUInt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompleteType {
  pub cpp_type: CppType,
  pub cpp_ffi_type: CppType,
  pub cpp_to_ffi_conversion: CppToFfiTypeConversion,
  pub rust_ffi_type: RustType,
  pub rust_api_type: RustType,
  pub rust_api_to_c_conversion: RustToCTypeConversion,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustFFIArgument {
  pub name: String,
  pub argument_type: RustType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustFFIFunction {
  pub return_type: RustType,
  pub name: String,
  pub arguments: Vec<RustFFIArgument>,
}
