#[cfg(feature = "swc-atoms")]
pub type String = hstr::Atom;

#[cfg(not(feature = "swc-atoms"))]
pub type String = std::string::String;
