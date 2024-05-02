#[derive(Clone, Eq)]
pub enum SmartString<'a> {
    Ref(&'a str),
    Dyn(String),
}

impl SmartString<'_> {
    pub fn as_ref(&self) -> &str {
        return match self {
            Self::Ref(value) => value,
            Self::Dyn(value) => value.as_ref(),
        };
    }
}

impl Default for SmartString<'static> {
    fn default() -> Self {
        return Self::Ref("");
    }
}

impl<'a> From<&'a str> for SmartString<'a> {
    fn from(string: &'a str) -> Self {
        return Self::Ref(string);
    }
}

impl From<String> for SmartString<'_> {
    fn from(string: String) -> Self {
        return Self::Dyn(string);
    }
}

impl PartialEq for SmartString<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.as_ref() == other.as_ref();
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SmartString<'_> {
    fn to_string(&self) -> String {
        return match self {
            Self::Ref(value) => value.to_string(),
            Self::Dyn(value) => value.clone(),
        };
    }
}
