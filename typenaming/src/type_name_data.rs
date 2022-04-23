/// This type represents some basic information about a given type
#[derive(Debug)]
pub struct TypeNameData {
    type_name: String,
    crate_name: Option<String>,
    crate_module: Option<String>,
    crate_version: Option<::semver::Version>,
    rustc_version: Option<::semver::Version>,
    generics: Vec<TypeNameData>,
}

#[derive(Debug)]
enum Token<'a> {
    String(&'a str),
    GenericStart,
    GenericNext,
    GenericEnd,
    Comma,
}

impl TypeNameData {
    /// Get name of type
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
    /// Get name of crate, in which the type is contained
    pub fn crate_name(&self) -> &Option<String> {
        &self.crate_name
    }
    /// Get module of crate, in which the type is contained
    /// This typically shows the information deliviered by 'std::module_path::module_path!'
    pub fn crate_module(&self) -> &Option<String> {
        &self.crate_module
    }
    /// Get version of crate, in which the type is contained - if available
    pub fn crate_version(&self) -> &Option<::semver::Version> {
        &self.crate_version
    }
    /// Get version of rustc, which was used to compile the type's crate - if available
    pub fn rustc_version(&self) -> &Option<::semver::Version> {
        &self.rustc_version
    }
    /// Get list of generic type parameters of the given type
    pub fn generics(&self) -> &[TypeNameData] {
        &self.generics
    }
    /// Constructor
    pub const fn new(
        type_name: String,
        crate_name: Option<String>,
        crate_module: Option<String>,
        crate_version: Option<::semver::Version>,
        rustc_version: Option<::semver::Version>,
        generics: Vec<TypeNameData>,
    ) -> Self {
        Self {
            type_name,
            crate_name,
            crate_module,
            crate_version,
            rustc_version,
            generics,
        }
    }
    /// Generates a one-line version, in the form "{type_name}<{generics}>, Crate={crate_name}, Module={crate_module}, Version={crate_version}, Rustc={rustc_version}"
    /// Note that None-entries are skipped
    /// If there are no generics, the '<…>'-part is skipped
    /// If there are generics, the generics are recursively joined with ';', using this method
    pub fn to_one_line_string(&self) -> String {
        let Self {
            type_name,
            crate_name,
            crate_module,
            crate_version,
            rustc_version,
            generics,
        } = self;

        let generics = if generics.is_empty() {
            String::new()
        } else {
            let generics = generics
                .iter()
                .map(|x| x.to_one_line_string())
                .collect::<Vec<_>>();
            format!("<{}>", generics.join("; "))
        };
        let mut s = type_name.clone() + &generics;
        if let Some(crate_name) = crate_name {
            s += ", Crate=";
            s += crate_name;
        }
        if let Some(crate_module) = crate_module {
            s += ", Module=";
            s += crate_module;
        }
        if let Some(crate_version) = crate_version {
            s += ", Version=";
            s += &crate_version.to_string();
        }
        if let Some(rustc_version) = rustc_version {
            s += ", Rustc=";
            s += &rustc_version.to_string();
        }
        //    "{type_name}{generics}, Version={crate_version}, rustc={rustc_version}"
        s
    }

    fn try_from_one_line_string(s: &str) -> Result<Self, (ParseError, String)> {
        let mut tokens = vec![];
        split_string(s, &mut tokens);
        fn parse_type<'a, 'b>(
            tokens: &'a [Token<'b>],
        ) -> Result<(TypeNameData, &'a [Token<'b>]), (ParseError, String)> {
            let (type_name, tokens) =
                if let Some((Token::String(type_name), tokens)) = tokens.split_first() {
                    (type_name.to_string(), tokens)
                } else {
                    return Err((ParseError::TypeNameNotFound, tokens.to_string()));
                };
            let mut crate_name = None;
            let mut crate_module = None;
            let mut crate_version = None;
            let mut rustc_version = None;
            let mut generics = Vec::new();
            #[derive(PartialEq)]
            enum GenericState {
                Possible,
                Ongoing,
                Impossible,
            }
            let mut generic_state = GenericState::Possible;
            let mut tokens = tokens;
            while let Some((t, remaining)) = tokens.split_first() {
                tokens = remaining;
                match t {
                    Token::String(s) => {
                        generic_state = GenericState::Impossible;
                        if let Some((s, data)) = s.split_once("Crate=") {
                            if s.trim().is_empty() {
                                if crate_name.is_none() {
                                    crate_name = Some(data.trim().to_owned());
                                } else {
                                    return Err((
                                        ParseError::CrateNameSetTwice,
                                        tokens.to_string(),
                                    ));
                                }
                            } else {
                                return Err((
                                    ParseError::FailedToParseCrateName,
                                    tokens.to_string(),
                                ));
                            }
                        } else if let Some((s, data)) = s.split_once("Module=") {
                            if s.trim().is_empty() {
                                if crate_module.is_none() {
                                    crate_module = Some(data.trim().to_owned());
                                } else {
                                    return Err((
                                        ParseError::CrateModuleSetTwice,
                                        tokens.to_string(),
                                    ));
                                }
                            } else {
                                return Err((
                                    ParseError::FailedToParseCrateModule,
                                    tokens.to_string(),
                                ));
                            }
                        } else if let Some((s, data)) = s.split_once("Version=") {
                            if s.trim().is_empty() {
                                if crate_version.is_none() {
                                    crate_version = Some(data.trim().to_owned());
                                } else {
                                    return Err((
                                        ParseError::CrateVersionSetTwice,
                                        tokens.to_string(),
                                    ));
                                }
                            } else {
                                return Err((
                                    ParseError::FailedToParseCrateVersion,
                                    tokens.to_string(),
                                ));
                            }
                        } else if let Some((s, data)) = s.split_once("Rustc=") {
                            if s.trim().is_empty() {
                                if rustc_version.is_none() {
                                    rustc_version = Some(data.trim().to_owned());
                                } else {
                                    return Err((
                                        ParseError::RustcVersionSetTwice,
                                        tokens.to_string(),
                                    ));
                                }
                            } else {
                                return Err((
                                    ParseError::FailedToParseRustcVersion,
                                    tokens.to_string(),
                                ));
                            }
                        } else {
                            return Err((ParseError::UnexpectedData, tokens.to_string()));
                        }
                    }
                    Token::GenericStart => {
                        if generic_state != GenericState::Possible {
                            return Err((ParseError::UnexpectedGenericStart, tokens.to_string()));
                        }
                        let (data, remaining) = parse_type(tokens)?;
                        generics.push(data);
                        tokens = remaining;
                        generic_state = GenericState::Ongoing;
                    }
                    Token::GenericEnd => {
                        if generic_state == GenericState::Ongoing {
                            generic_state = GenericState::Impossible;
                        } else {
                            return Err((ParseError::UnexpectedGenericEnd, tokens.to_string()));
                        }
                    }
                    Token::GenericNext => {
                        if generic_state != GenericState::Ongoing {
                            return Err((
                                ParseError::UnexpectedGenericGenericNext,
                                tokens.to_string(),
                            ));
                        }
                        let (data, remaining) = parse_type(tokens)?;
                        generics.push(data);
                        tokens = remaining;
                    }
                    Token::Comma => generic_state = GenericState::Impossible,
                }

                if crate_name.is_some() && crate_version.is_some() && rustc_version.is_some() {
                    break;
                }
            }
            use std::str::FromStr;
            let crate_version = crate_version
                .map(|v| semver::Version::from_str(&v))
                .transpose()
                .map_err(|e| {
                    (
                        ParseError::FailedToParseCrateVersionSemver(e),
                        tokens.to_string(),
                    )
                })?;
            let rustc_version = rustc_version
                .map(|v| semver::Version::from_str(&v))
                .transpose()
                .map_err(|e| {
                    (
                        ParseError::FailedToParseRustcVersionSemver(e),
                        tokens.to_string(),
                    )
                })?;
            Ok((
                TypeNameData {
                    type_name,
                    crate_name,
                    crate_module,
                    crate_version,
                    rustc_version,
                    generics,
                },
                tokens,
            ))
        }

        let (data, remaining) = parse_type(&tokens)?;
        if remaining.is_empty() {
            Ok(data)
        } else {
            Err((ParseError::RemainingToken(data), remaining.to_string()))
        }
    }
}
impl serde::Serialize for TypeNameData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_one_line_string();
        serializer.serialize_str(&s)
    }
}
impl<'de> serde::Deserialize<'de> for TypeNameData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        TypeNameData::try_from_one_line_string(&s).map_err(|(e, tokens)| {
            D::Error::custom(format!(
                "Failed to parse due to error '{e:?}' with remaining tokens '{tokens}' and complete tokens '{s}'"
            ))
        })
    }
}

fn split_string<'a>(s: &'a str, v: &mut Vec<Token<'a>>) {
    if let Some((first, last)) = s.split_once(',') {
        split_string(first, v);
        v.push(Token::Comma);
        split_string(last, v);
    } else if let Some((first, last)) = s.split_once('<') {
        split_string(first, v);
        v.push(Token::GenericStart);
        split_string(last, v);
    } else if let Some((first, last)) = s.split_once('>') {
        split_string(first, v);
        v.push(Token::GenericEnd);
        split_string(last, v);
    } else if let Some((first, last)) = s.split_once(';') {
        split_string(first, v);
        v.push(Token::GenericNext);
        split_string(last, v);
    } else {
        let s = s.trim();
        if !s.is_empty() {
            v.push(Token::String(s));
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    TypeNameNotFound,
    RemainingToken(TypeNameData),
    UnexpectedGenericEnd,
    UnexpectedGenericGenericNext,
    UnexpectedGenericStart,
    FailedToParseCrateName,
    CrateNameSetTwice,
    FailedToParseCrateModule,
    CrateModuleSetTwice,
    FailedToParseCrateVersion,
    CrateVersionSetTwice,
    FailedToParseRustcVersion,
    RustcVersionSetTwice,
    UnexpectedData,
    FailedToParseCrateVersionSemver(semver::Error),
    FailedToParseRustcVersionSemver(semver::Error),
}
trait TokensToString {
    fn to_string(&self) -> String;
}
impl<'b> TokensToString for [Token<'b>] {
    fn to_string(&self) -> String {
        let mut tt = Default::default();
        for t in self {
            match t {
                Token::String(s) => tt += *s,
                Token::GenericStart => tt += "<",
                Token::GenericEnd => tt += ">",
                Token::Comma => tt += ", ",
                Token::GenericNext => tt += "; ",
            }
        }
        tt
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    fn simple_example() -> TypeNameData {
        use core::str::FromStr;
        TypeNameData {
            type_name: "Test".to_owned(),
            crate_name: Some("testing".to_owned()),
            crate_module: Some("test_module".to_owned()),
            crate_version: Some(semver::Version::new(1, 2, 3)),
            rustc_version: Some(semver::Version {
                major: 2,
                minor: 0,
                patch: 1,
                pre: semver::Prerelease::from_str("ieie").unwrap(),
                build: semver::BuildMetadata::from_str("ieieiei").unwrap(),
            }),
            generics: vec![],
        }
    }
    fn generic_example() -> TypeNameData {
        use core::str::FromStr;
        TypeNameData {
            type_name: "Gen".to_owned(),
            crate_name: Some("generic".to_owned()),
            crate_module: Some("test_module".to_owned()),
            crate_version: Some(semver::Version::new(1, 2, 3)),
            rustc_version: Some(semver::Version {
                major: 2,
                minor: 0,
                patch: 1,
                pre: semver::Prerelease::from_str("ieie").unwrap(),
                build: semver::BuildMetadata::from_str("ieieiei").unwrap(),
            }),
            generics: vec![simple_example()],
        }
    }
    fn generic2_example() -> TypeNameData {
        use core::str::FromStr;
        TypeNameData {
            type_name: "Gen".to_owned(),
            crate_name: Some("generic2".to_owned()),
            crate_module: Some("test_module".to_owned()),
            crate_version: Some(semver::Version::new(1, 2, 3)),
            rustc_version: Some(semver::Version {
                major: 33,
                minor: 22,
                patch: 11,
                pre: semver::Prerelease::from_str("ieie").unwrap(),
                build: semver::BuildMetadata::from_str("ieieiei").unwrap(),
            }),
            generics: vec![simple_example(), generic_example(), simple_example()],
        }
    }

    fn asserting(lhs: &TypeNameData, rhs: &TypeNameData) {
        assert_eq!(lhs.crate_name, rhs.crate_name);
        assert_eq!(lhs.type_name, rhs.type_name);
        assert_eq!(lhs.generics.len(), rhs.generics.len());
        assert_eq!(lhs.crate_version, rhs.crate_version);
        assert_eq!(lhs.rustc_version, rhs.rustc_version);
        for (lhs, rhs) in lhs.generics.iter().zip(rhs.generics.iter()) {
            asserting(lhs, rhs);
        }
    }

    #[test]
    fn simple_serialize() {
        let serialized = dbg!(serde_json::to_string_pretty(&simple_example())).unwrap();
        assert_eq!(
            serialized,
            "\"Test, Crate=testing, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei\""
        );
    }
    #[test]
    fn simple_deserialize() {
        let info = simple_example();
        let serialized = dbg!(serde_json::to_string_pretty(&info)).unwrap();
        let deserialized: TypeNameData = serde_json::from_str(&serialized).unwrap();
        asserting(&deserialized, &info);
    }
    #[test]
    fn generic_serialize() {
        let serialized = dbg!(serde_json::to_string_pretty(&generic_example())).unwrap();
        assert_eq!(
        serialized,
        "\"Gen<Test, Crate=testing, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei>, Crate=generic, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei\""
    );
    }
    #[test]
    fn generic_deserialize() {
        let info = generic_example();
        let serialized = dbg!(serde_json::to_string_pretty(&info)).unwrap();
        let deserialized: TypeNameData = serde_json::from_str(&serialized).unwrap();
        asserting(&deserialized, &info);
    }
    #[test]
    fn generic2_serialize() {
        let serialized = dbg!(serde_json::to_string_pretty(&generic2_example())).unwrap();
        assert_eq!(
        serialized,
        "\"Gen<Test, Crate=testing, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei; Gen<Test, Crate=testing, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei>, Crate=generic, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei; Test, Crate=testing, Module=test_module, Version=1.2.3, Rustc=2.0.1-ieie+ieieiei>, Crate=generic2, Module=test_module, Version=1.2.3, Rustc=33.22.11-ieie+ieieiei\"");
    }
    #[test]
    fn generic2_deserialize() {
        let info = generic2_example();
        let serialized = dbg!(serde_json::to_string_pretty(&info)).unwrap();
        let deserialized: TypeNameData = serde_json::from_str(&serialized).unwrap();
        dbg!(&info);
        dbg!(&deserialized);
        asserting(&deserialized, &info);
    }
}
