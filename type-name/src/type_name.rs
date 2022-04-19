use crate::TypeNameData;

/// This trait allows to extract some basic information about the type
pub trait TypeName {
    /// Extract name of type from an instance
    fn type_name(&self) -> TypeNameData;
    /// Extract name of type statically. Typically both functions share the same implementation
    fn type_name_static() -> TypeNameData
    where
        Self: Sized;
}
