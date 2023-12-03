#[cfg(feature = "swc")]
pub type String = swc_atoms::Atom;

#[cfg(not(feature = "swc"))]
pub type String = std::string::String;
