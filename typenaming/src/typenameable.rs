use crate::TypeInfo;

/// This trait allows to extract some basic information about the type
pub trait TypeNameable {
    /// Fetch name of type and related information
    fn type_info() -> TypeInfo;
}
