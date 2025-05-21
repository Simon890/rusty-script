use std::collections::HashMap;

use super::{interpreter::RuntimeValue};

#[derive(Debug)]
pub struct Env {
    parent: Option<Box<Env>>,
    vars: HashMap<String, RuntimeValue>
}

impl Env {

    pub fn new(parent: Option<Env>) -> Self {
        let parent_box = parent.map(Box::new);
        Self { parent: parent_box, vars: HashMap::new() }
    }

    pub fn add(&mut self, var_name: String, value: RuntimeValue) {
        if self.vars.contains_key(&var_name) {
            panic!("Variable {} was already declared", var_name)
        }
        self.vars.insert(var_name, value);
    }

    pub fn update(&mut self, var_name: String, value: RuntimeValue) {
        if !self.vars.contains_key(&var_name) {
            panic!("Variable {} does not exist", var_name);
        }
        self.vars.insert(var_name, value);
    }

    pub fn get(&self, var_name: &str) -> &RuntimeValue {
        let mut current = Some(self);
        while current.is_some() {
            let c = current.unwrap();
            let variable = c.vars.get(var_name);
            if variable.is_some() {
                return variable.unwrap();
            }
            current = c.parent.as_deref();
        }
        panic!("Variable {} does not exist", var_name);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_environment() {
        let mut env1 = Env::new(None);
        env1.add("x".to_string(), RuntimeValue::Number(5.));
        env1.add("y".to_string(), RuntimeValue::String("simon".to_string()));

        let mut env2 = Env::new(Some(env1));
        env2.add("z".to_string(), RuntimeValue::Bool(false));

        let z = env2.get("z");
        dbg!(&z);
        assert_eq!(*z, RuntimeValue::Bool(false));
        let x = env2.get("x");
        dbg!(&x);
        assert_eq!(*x, RuntimeValue::Number(5.));
    }

    #[test]
    #[should_panic]
    fn test_panic_environment() {
        let mut env1 = Env::new(None);
        env1.add("x".to_string(), RuntimeValue::Number(5.));
        env1.add("y".to_string(), RuntimeValue::String("simon".to_string()));

        let mut env2 = Env::new(Some(env1));
        env2.add("z".to_string(), RuntimeValue::Bool(false));

        env2.get("a");
    }
}