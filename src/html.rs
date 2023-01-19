use std::{fmt::Display, ops::Add, str::FromStr};

use global_counter::primitive::exact::CounterU8;
use maud::{html, Render};

#[derive(Debug, Clone)]
pub(crate) struct Class(String);

impl FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|char| char.is_ascii_whitespace()) {
            return Err(s.to_owned());
        }
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<&str> for Class {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for Class {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Render for Class {
    fn render(&self) -> maud::Markup {
        maud::PreEscaped(self.0.clone())
    }
}

#[derive(Default, Clone)]
pub(crate) struct Classes(Vec<Class>);

impl Classes {
    pub(crate) fn push(&mut self, value: Class) {
        self.0.push(value)
    }
}

impl<T: AsRef<str>> From<Vec<T>> for Classes {
    fn from(classes: Vec<T>) -> Self {
        classes
            .into_iter()
            .map(|class| Class(class.as_ref().to_owned()))
            .collect()
    }
}

impl Add<Self> for Classes {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.0.append(&mut rhs.0);
        self
    }
}

impl Add<Class> for Classes {
    type Output = Self;

    fn add(mut self, rhs: Class) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl FromIterator<Class> for Classes {
    fn from_iter<T: IntoIterator<Item = Class>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Render for Classes {
    fn render(&self) -> maud::Markup {
        html!(
            (self
                .0
                .iter()
                .map(|class| class.0.clone())
                .collect::<Vec<String>>()
                .join(" "))
        )
    }
}

#[macro_export]
macro_rules! classes {
    () => { $crate::html::Classes::default() };
    ($($class:expr)*) => {{
        let mut classes = $crate::html::Classes::default();
        $(
            let class = <$crate::html::Class as ::std::convert::TryFrom<_>>::try_from($class)
                .unwrap();
            classes.push(class);
        )*
        classes
    }};
}

static CLASS_COUNTER: CounterU8 = CounterU8::new(0);
pub(crate) fn css_class() -> Class {
    let count = CLASS_COUNTER.inc().to_string();
    format!("_{count}").parse().unwrap()
}
