use std::collections::HashMap;
use std::rc::Rc;

/// StringInterner manages a pool of unique string values to reduce memory usage
/// and enable fast string equality comparisons via pointer equality.
///
/// # Overview
///
/// String interning is a memory optimization technique where identical string values
/// share the same memory location. The interner maintains a pool of unique strings
/// and returns references (Rc<str>) to these shared instances.
///
/// # Design Rationale
///
/// **HashMap-based storage**: The interner uses `HashMap<String, Rc<str>>` where:
/// - The key is a String for O(1) average-case lookups during interning
/// - The value is Rc<str> for efficient cloning (just incrementing reference count)
/// - This design trades some memory overhead for fast lookups and sharing
///
/// **Per-VM isolation**: Each VM instance has its own StringInterner because:
/// - Strings are specific to a VM's execution context
/// - Per-VM isolation avoids thread-safety concerns (single-threaded VMs)
/// - Enables independent VM lifecycle management without global state
/// - Prevents memory leaks when VMs are destroyed
///
/// # Performance Characteristics
///
/// - **Interning**: O(1) average case (HashMap lookup + optional insert)
/// - **String equality**: O(1) when both strings are interned (pointer comparison)
/// - **Memory deduplication**: Identical strings share one allocation
/// - **Reference counting overhead**: Small cost per Rc clone (atomic increment)
///
/// # Usage Pattern
///
/// All string creation in Neon flows through the interner:
/// - String literals from bytecode chunks
/// - Runtime string operations (concatenation, substring, etc.)
/// - Standard library functions that return strings
/// - User input and I/O operations
///
/// This ensures that string equality comparisons (==) benefit from pointer equality
/// checks in ObjString::PartialEq, providing O(1) comparison for identical strings.
///
/// # Example
///
/// ```ignore
/// let mut interner = StringInterner::new();
/// let s1 = interner.intern("hello");
/// let s2 = interner.intern("hello");
/// assert!(Rc::ptr_eq(&s1, &s2)); // Same pointer, O(1) equality
/// ```
#[derive(Debug, Default)]
pub struct StringInterner {
    /// Maps string content to its interned Rc<str> representation.
    /// The String key enables fast lookups, while Rc<str> values are cheap to clone.
    pool: HashMap<String, Rc<str>>,
}

impl StringInterner {
    /// Creates a new empty StringInterner
    pub fn new() -> Self {
        StringInterner {
            pool: HashMap::new(),
        }
    }

    /// Creates a new StringInterner with pre-allocated capacity for the specified
    /// number of unique strings.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity for the internal HashMap
    ///
    /// # Performance
    ///
    /// Pre-allocating capacity can reduce allocations and improve performance
    /// when you know approximately how many unique strings will be interned.
    /// This is particularly useful when loading large bytecode chunks with
    /// many string constants.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // If loading a chunk with 1000 string constants
    /// let mut interner = StringInterner::with_capacity(1000);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        StringInterner {
            pool: HashMap::with_capacity(capacity),
        }
    }

    /// Interns a string, returning an Rc<str> that may be shared with other
    /// identical strings that have been interned.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to intern
    ///
    /// # Returns
    ///
    /// An Rc<str> pointing to the interned string. If this string was already
    /// interned, the returned Rc will point to the same memory location as
    /// previous calls with the same string content.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut interner = StringInterner::new();
    /// let s1 = interner.intern("hello");
    /// let s2 = interner.intern("hello");
    /// assert!(Rc::ptr_eq(&s1, &s2)); // Same pointer
    /// ```
    pub fn intern(&mut self, s: &str) -> Rc<str> {
        // Fast path: check if string already exists in pool
        if let Some(existing) = self.pool.get(s) {
            return Rc::clone(existing);
        }

        // Slow path: insert new string into pool
        let rc_str: Rc<str> = Rc::from(s);
        self.pool.insert(s.to_string(), Rc::clone(&rc_str));
        rc_str
    }

    /// Returns the number of unique strings currently in the interner
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    /// Returns true if the interner contains no strings
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_new_string() {
        let mut interner = StringInterner::new();
        let s = interner.intern("hello");
        assert_eq!(s.as_ref(), "hello");
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn test_intern_duplicate_string() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");

        // Should return the same Rc (pointer equality)
        assert!(Rc::ptr_eq(&s1, &s2));

        // Should only have one entry in the pool
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn test_intern_multiple_strings() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("world");
        let s3 = interner.intern("hello");

        // s1 and s3 should be the same pointer
        assert!(Rc::ptr_eq(&s1, &s3));

        // s1 and s2 should be different pointers
        assert!(!Rc::ptr_eq(&s1, &s2));

        // Should have two unique strings
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn test_intern_empty_string() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("");
        let s2 = interner.intern("");

        assert!(Rc::ptr_eq(&s1, &s2));
        assert_eq!(s1.as_ref(), "");
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn test_intern_special_characters() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello\nworld");
        let s2 = interner.intern("hello\nworld");
        let s3 = interner.intern("hello world");

        assert!(Rc::ptr_eq(&s1, &s2));
        assert!(!Rc::ptr_eq(&s1, &s3));
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn test_interner_is_empty() {
        let mut interner = StringInterner::new();
        assert!(interner.is_empty());

        interner.intern("hello");
        assert!(!interner.is_empty());
    }

    #[test]
    fn test_intern_long_strings() {
        let mut interner = StringInterner::new();
        let long_str = "a".repeat(1000);
        let s1 = interner.intern(&long_str);
        let s2 = interner.intern(&long_str);

        assert!(Rc::ptr_eq(&s1, &s2));
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn test_intern_unicode() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("Hello, 世界!");
        let s2 = interner.intern("Hello, 世界!");
        let s3 = interner.intern("Привет, мир!");

        assert!(Rc::ptr_eq(&s1, &s2));
        assert!(!Rc::ptr_eq(&s1, &s3));
        assert_eq!(interner.len(), 2);
    }
}
