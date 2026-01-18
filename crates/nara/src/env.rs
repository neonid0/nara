use crate::interner::StringInterner;
use crate::val::Val;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent: Option<&'parent Self>,
    interner: RefCell<StringInterner>,
}

impl<'parent> Default for Env<'parent> {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
            interner: RefCell::new(StringInterner::new()),
        }
    }
}

impl<'parent> Env<'parent> {
    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding_value_restrict(&self, name: &str) -> Result<Val, String> {
        self.get_binding_value(name)
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    fn get_binding_value(&self, name: &str) -> Option<Val> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value(name))
        })
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
            interner: RefCell::new(StringInterner::new()),
        }
    }

    fn get_root_interner(&self) -> &RefCell<StringInterner> {
        match self.parent {
            None => &self.interner,
            Some(parent) => parent.get_root_interner(),
        }
    }

    pub(crate) fn intern(&self, s: &str) -> Rc<str> {
        self.get_root_interner().borrow_mut().intern(s)
    }
}
