use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct StringInterner {
    pool: HashMap<String, Rc<str>>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> Rc<str> {
        if let Some(interned) = self.pool.get(s) {
            // String already in pool, return existing Rc
            Rc::clone(interned)
        } else {
            // New string, add to pool
            let rc: Rc<str> = Rc::from(s);
            self.pool.insert(s.to_string(), Rc::clone(&rc));
            rc
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_same_string() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");

        assert_eq!(s1, s2);
        assert!(Rc::ptr_eq(&s1, &s2));
    }

    #[test]
    fn test_intern_different_strings() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("world");

        assert_ne!(s1, s2);
        assert!(!Rc::ptr_eq(&s1, &s2));
    }

    #[test]
    fn test_interner_len() {
        let mut interner = StringInterner::new();
        assert_eq!(interner.len(), 0);

        interner.intern("hello");
        assert_eq!(interner.len(), 1);

        interner.intern("hello");
        assert_eq!(interner.len(), 1);

        interner.intern("world");
        assert_eq!(interner.len(), 2);
    }
}
