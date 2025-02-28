use core::fmt;
use std::cmp::Ordering;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use super::{Key, Node};

#[derive(Debug)]
pub enum InsertionResult {
    Inserted,
    AddToFater(Key, Node),
}

#[derive(Debug)]
pub enum SearchResult {
    Find(usize),
    GoDown(usize),
}

#[derive(Debug)]
pub enum RemovalResult{
    RemoveCompleted,
    LeafRemoveFail(Key),
    NotLeafRemoveFail(Key, Node)
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.keys[0].cmp(&other.keys[0])
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl Key {
    pub fn new(id: i32, nome: String, quantidade: usize) -> Self {
        Self {
            key: id,
            nome,
            quantidade,
        }
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a string in the format {id, "nome", quantidade}"#)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = value.trim().trim_start_matches('{').trim_end_matches('}');

                let parts: Vec<&str> = value.split(", ").collect();

                if parts.len() != 3 {
                    return Err(E::custom(format!("Expected 3 parts, got {}", parts.len())));
                }

                let key = parts[0]
                    .parse::<i32>()
                    .map_err(|_| E::custom("Invalid id format"))?;
                let nome = parts[1].trim_matches('"').to_string();
                let quantidade = parts[2]
                    .parse::<usize>()
                    .map_err(|_| E::custom("Invalid quantidade format"))?;

                Ok(Key {
                    key,
                    nome,
                    quantidade,
                })
            }
        }

        deserializer.deserialize_str(KeyVisitor)
    }
}
