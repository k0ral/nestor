use itertools::Itertools;
use std::fmt::{self, Display};

// A JSON path is essentially a sequence of steps, starting from the document root, to reach a JSON
// node. `PathStep` is one of the steps.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathStep {
    Key(String), // For objects
    Index(u32),  // For arrays
}

impl PathStep {
    pub fn to_json_path(steps: &Vec<PathStep>) -> String {
        (&steps)
            .iter()
            .rev()
            .map(|s| match s {
                PathStep::Key(k) => format!("[\"{}\"]", k),
                PathStep::Index(i) => format!("[{}]", i),
            })
            .join("")
    }
    pub fn to_json_pointer(steps: &Vec<PathStep>) -> String {
        (&steps)
            .iter()
            .rev()
            .map(|s| match s {
                PathStep::Key(k) => k.clone(),
                PathStep::Index(i) => i.to_string(),
            })
            .join("/")
    }
}

// A JSON scalar is a non-null leaf value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scalar {
    Bool(bool),
    Number(serde_json::value::Number),
    String(String),
}

impl Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scalar::Bool(value) => value.fmt(f),
            Scalar::Number(value) => value.fmt(f),
            Scalar::String(value) => value.fmt(f),
        }
    }
}

// Iterates over scalar (i.e. leaf) values in a given JSON document.
// Produces (path to leaf value, actual leaf value) tuples.
pub enum ScalarIterator {
    Scalar {
        value: Scalar,
        consumed: bool,
    },
    Array {
        index: u32,
        iterator: Box<dyn std::iter::Iterator<Item = serde_json::Value>>,
        child: Option<Box<ScalarIterator>>,
    },
    Object {
        key: String,
        iterator: Box<dyn std::iter::Iterator<Item = (String, serde_json::Value)>>,
        child: Option<Box<ScalarIterator>>,
    },
}

impl ScalarIterator {
    pub fn new(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Array(a) => {
                let mut iterator = Box::new(a.into_iter());
                let child = iterator.next().map(|v| Box::new(ScalarIterator::new(v)));
                Self::Array { index: 0, iterator, child }
            }
            serde_json::Value::Object(m) => {
                let mut iterator = Box::new(m.into_iter());
                if let Some((key, value)) = iterator.next() {
                    Self::Object {
                        key,
                        iterator,
                        child: Some(Box::new(ScalarIterator::new(value))),
                    }
                } else {
                    Self::Object {
                        key: String::new(),
                        iterator,
                        child: None,
                    }
                }
            }
            serde_json::Value::Bool(b) => Self::Scalar {
                value: Scalar::Bool(b),
                consumed: false,
            },
            serde_json::Value::Number(n) => Self::Scalar {
                value: Scalar::Number(n),
                consumed: false,
            },
            serde_json::Value::String(s) => Self::Scalar {
                value: Scalar::String(s),
                consumed: false,
            },
            serde_json::Value::Null => Self::Scalar {
                value: Scalar::Bool(false), // Dummy, unused value
                consumed: true,
            },
        }
    }
}

impl std::iter::Iterator for ScalarIterator {
    type Item = (Vec<PathStep>, Scalar);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Scalar { value, consumed } => {
                if *consumed {
                    None
                } else {
                    *consumed = true;
                    Some((vec![], value.clone()))
                }
            }

            Self::Array { index, iterator, child } => loop {
                if let Some(i) = child {
                    if let Some((mut path, scalar)) = i.next() {
                        path.push(PathStep::Index(*index));
                        return Some((path, scalar));
                    }

                    *index += 1;
                    *child = iterator.next().map(|v| Box::new(ScalarIterator::new(v)));
                } else {
                    break None;
                }
            },

            Self::Object { key, iterator, child } => loop {
                if let Some(i) = child {
                    if let Some((mut path, scalar)) = i.next() {
                        path.push(PathStep::Key(key.clone()));
                        return Some((path, scalar));
                    }

                    if let Some((k, v)) = iterator.next() {
                        *key = k;
                        *child = Some(Box::new(ScalarIterator::new(v)));
                    } else {
                        *key = String::new();
                        *child = None;
                    }
                } else {
                    break None;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_iterate_scalars() {
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        let iterator = ScalarIterator::new(john);
        let items: Vec<(Vec<PathStep>, Scalar)> = iterator.collect();

        assert_eq!(
            items,
            vec![
                (vec![PathStep::Key("age".to_string())], Scalar::Number(json!(43).as_number().unwrap().clone())),
                (vec![PathStep::Key("name".to_string())], Scalar::String("John Doe".to_string())),
                (vec![PathStep::Index(0), PathStep::Key("phones".to_string())], Scalar::String("+44 1234567".to_string())),
                (vec![PathStep::Index(1), PathStep::Key("phones".to_string())], Scalar::String("+44 2345678".to_string())),
            ]
        );
    }
}
