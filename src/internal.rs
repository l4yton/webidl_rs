#[cfg(feature = "hstr-atom")]
pub type String = hstr::Atom;

#[cfg(not(feature = "hstr-atom"))]
pub type String = std::string::String;
