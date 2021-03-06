use std::cmp::Ordering;
use std::collections::BTreeMap;

/// Extends a tree map to provide additional operations.
pub trait TreeMapExtension<K, V>
where
    K: std::cmp::Ord,
    V: std::cmp::Ord,
{
    /// Checks if a tree map contains another tree map.
    fn contains(&self, other: &BTreeMap<K, V>) -> bool;

    /// Checks if a tree map contains a certain pair of values.
    fn contains_entry(&self, key: &K, value: &V) -> bool;
}

/// Implements [`TreeMapExtension`].
impl<K, V> TreeMapExtension<K, V> for BTreeMap<K, V>
where
    K: std::cmp::Ord,
    V: std::cmp::Ord,
{
    fn contains(&self, other: &BTreeMap<K, V>) -> bool {
        return other.iter().all(|(k, v)| self.contains_entry(k, v));
    }

    fn contains_entry(&self, key: &K, value: &V) -> bool {
        return self
            .iter()
            .any(|(k, v)| k.cmp(key) == Ordering::Equal && v.cmp(value) == Ordering::Equal);
    }
}

/// Extends a string based tree map to provide additional operations.
pub trait StringTreeMapExtension {
    /// Checks if a tree map contains another tree map while ignoring the case of the key.
    fn contains_with_case_insensitive_key(&self, other: &BTreeMap<String, String>) -> bool;

    /// Checks if a tree map contains a certain pair of values while ignoring the case of the key.
    fn contains_entry_with_case_insensitive_key(&self, key: &String, value: &String) -> bool;

    /// Checks if a tree map contains a key while ignoring the case of the key.
    fn contains_case_insensitive_key(&self, key: &String) -> bool;
}

/// Implements [`StringTreeMapExtension`].
impl StringTreeMapExtension for BTreeMap<String, String> {
    fn contains_with_case_insensitive_key(&self, other: &BTreeMap<String, String>) -> bool {
        return other
            .iter()
            .all(|(k, v)| self.contains_entry_with_case_insensitive_key(k, v));
    }

    fn contains_entry_with_case_insensitive_key(&self, key: &String, value: &String) -> bool {
        return self.iter().any(|(k, v)| {
            k.to_lowercase().cmp(&key.to_lowercase()) == Ordering::Equal
                && v.cmp(value) == Ordering::Equal
        });
    }

    fn contains_case_insensitive_key(&self, key: &String) -> bool {
        let key_lc = key.to_lowercase();
        return self
            .keys()
            .into_iter()
            .any(|k| k.to_lowercase().eq(&key_lc));
    }
}

#[cfg(test)]
mod test {
    use crate::server::util::{StringTreeMapExtension, TreeMapExtension};
    use std::collections::BTreeMap;

    #[test]
    fn tree_map_fully_contains_other() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");
        m1.insert("h2", "v2");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v1");
        m2.insert("h2", "v2");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(true, result);
    }

    #[test]
    fn tree_map_contains_subset() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");
        m1.insert("h2", "v2");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v1");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(true, result);
    }

    #[test]
    fn tree_map_does_not_contain_other() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v1");
        m2.insert("h2", "v2");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn tree_map_contains_all_keys_no_values() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");
        m1.insert("h2", "v2");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v3");
        m2.insert("h2", "v4");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn tree_map_contains_all_keys_some_values() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");
        m1.insert("h2", "v2");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v1");
        m2.insert("h2", "v2");
        m2.insert("h3", "v3");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn tree_map_contains_all_keys_some_values_equal_length() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1", "v1");
        m1.insert("h2", "v2");

        let mut m2 = BTreeMap::new();
        m2.insert("h1", "v1");
        m2.insert("h2", "v3");

        // Act
        let result = m1.contains(&m2);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn string_tree_map_contains_all_keys_some_values_equal_length() {
        // Arrange
        let mut m1 = BTreeMap::new();
        m1.insert("h1".to_string(), "v1".to_string());
        m1.insert("h2".to_string(), "v2".to_string());

        let mut m2 = BTreeMap::new();
        m2.insert("H1".to_string(), "v1".to_string());
        m2.insert("H2".to_string(), "v2".to_string());

        // Act
        let result = m1.contains_with_case_insensitive_key(&m2);

        // Assert
        assert_eq!(true, result);
    }
}
