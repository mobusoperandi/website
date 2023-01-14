use std::{ops::Add, str::FromStr};

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
            let class = <$crate::html::Class as ::std::str::FromStr>::from_str(&$class)
                .unwrap();
            classes.push(class);
        )*
        classes
    }};
}
