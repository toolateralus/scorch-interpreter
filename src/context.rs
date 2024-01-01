use std::{self};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::types::{Instance, Value};

pub struct Context {
    pub parent: Option<Rc<RefCell<Context>>>,
    pub variables: HashMap<String, Rc<RefCell<Instance>>>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("parent", &self.parent.as_ref().map(|p| p.as_ptr()))
            .field("variables", &self.variables)
            .finish()
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        let parent = self.parent.clone();
        let variables = self.variables.clone();
        Context { parent, variables }
    }
}

impl Context {
    pub fn new() -> Rc<RefCell<Context>> {
        Rc::new(RefCell::new(Context {
            parent: Option::None,
            variables: HashMap::new(),
        }))
    }
    pub fn find_variable(&self, name: &str) -> Option<Rc<RefCell<Instance>>> {
        match self.variables.get(name) {
            Some(var) => Some(Rc::clone(&var)),
            None => match &self.parent {
                Some(parent) => parent.borrow().find_variable(name),
                None => None,
            },
        }
    }
    pub fn insert_variable(&mut self, name: &str, value: Rc<RefCell<Instance>>) -> () {
        self.variables.insert(String::from(name), value);
    }
    pub fn seek_overwrite_in_parents<'ctx>(
        &mut self,
        name: &str,
        value: &'ctx Value,
    ) -> Result<(), ()> {
        if let Some(var) = self.variables.get_mut(name) {
            if let Ok(mut v) = var.try_borrow_mut() {
                v.set_value(value);
                Ok(())
            } else {
                panic!("INTERNAL_INTERPRETER_ERROR::\n\t error : ->-> double borrow on variable {}, value : {:?} <-<- : error ", name, value.clone());
            }
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().seek_overwrite_in_parents(name, value)
        } else {
            Err(())
        }
    }
}
