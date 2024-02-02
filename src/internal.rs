#[cfg(feature = "swc-atoms")]
pub type String = swc_atoms::Atom;

#[cfg(not(feature = "swc-atoms"))]
pub type String = std::string::String;
