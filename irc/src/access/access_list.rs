//! Access list implementation

use crate::enums::AccessLevel;
use crate::traits::{AccessEntry, AccessList as AccessListTrait};
use parking_lot::RwLock;
use regex::Regex;
use std::collections::HashMap;

/// Implementation of an access list
pub struct AccessListImpl {
    entries: RwLock<HashMap<AccessLevel, Vec<AccessEntry>>>,
}

impl Default for AccessListImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessListImpl {
    /// Create a new empty access list
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}

impl AccessListTrait for AccessListImpl {
    fn get_entries(&self, level: AccessLevel) -> Vec<AccessEntry> {
        self.entries
            .read()
            .get(&level)
            .cloned()
            .unwrap_or_default()
    }

    fn get_all_entries(&self) -> Vec<(AccessLevel, AccessEntry)> {
        let entries = self.entries.read();
        let mut result = Vec::new();
        for (level, list) in entries.iter() {
            for entry in list {
                result.push((*level, entry.clone()));
            }
        }
        result
    }

    fn add(&mut self, level: AccessLevel, entry: AccessEntry) -> bool {
        let mut entries = self.entries.write();
        let list = entries.entry(level).or_insert_with(Vec::new);

        // Check if mask already exists
        if list.iter().any(|e| e.mask == entry.mask) {
            return false;
        }

        list.push(entry);
        true
    }

    fn remove(&mut self, level: AccessLevel, mask: &str) -> bool {
        let mut entries = self.entries.write();
        if let Some(list) = entries.get_mut(&level) {
            let len_before = list.len();
            list.retain(|e| e.mask != mask);
            return list.len() < len_before;
        }
        false
    }

    fn clear(&mut self, level: AccessLevel) {
        let mut entries = self.entries.write();
        entries.remove(&level);
    }

    fn clear_all(&mut self) {
        let mut entries = self.entries.write();
        entries.clear();
    }

    fn check(&self, address: &str) -> AccessLevel {
        let entries = self.entries.read();
        let mut highest_level = AccessLevel::None;

        // Check all levels from highest to lowest
        for (&level, list) in entries.iter() {
            for entry in list {
                // Skip expired entries
                if entry.is_expired() {
                    continue;
                }

                // Convert mask to regex pattern
                let pattern = entry
                    .mask
                    .replace('.', r"\.")
                    .replace('*', ".*")
                    .replace('?', ".");

                if let Ok(regex) = Regex::new(&format!("(?i)^{}$", pattern)) {
                    if regex.is_match(address) && level > highest_level {
                        highest_level = level;
                    }
                }
            }
        }

        highest_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_check() {
        let mut list = AccessListImpl::new();

        list.add(AccessLevel::Voice, AccessEntry::new("*!*@*.example.com".to_string()));
        list.add(AccessLevel::Deny, AccessEntry::new("baduser!*@*".to_string()));

        assert_eq!(list.check("user!ident@host.example.com"), AccessLevel::Voice);
        assert_eq!(list.check("baduser!ident@anywhere.net"), AccessLevel::Deny);
        assert_eq!(list.check("other!user@other.net"), AccessLevel::None);
    }

    #[test]
    fn test_remove() {
        let mut list = AccessListImpl::new();

        list.add(AccessLevel::Voice, AccessEntry::new("*!*@*.example.com".to_string()));
        assert!(list.remove(AccessLevel::Voice, "*!*@*.example.com"));
        assert!(!list.remove(AccessLevel::Voice, "*!*@*.example.com"));
    }
}
