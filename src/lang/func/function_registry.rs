use std::{any::TypeId, collections::HashMap};

use crate::lang::interpreter::RuntimeValue;

pub struct FunctionRegistry {
    functions: HashMap<String, Function>
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new()
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name.to_owned(), function);
    }

    pub fn call(&self, function_name: &str, args: Vec<RuntimeValue>) -> RuntimeValue {
        let function = self.functions.get(function_name).expect(&format!("Function {} does not exist", function_name));
        if function.expected_params != args.len() {
            panic!("Function {} expects {} params, got {}", function.name, function.expected_params, args.len())
        }

        for (i, arg) in args.iter().enumerate() {
            if !arg.matches_type(&function.param_types[i]) {
                panic!("Param {} of function {} expected {:?}, got {:?}", i, function.name, function.param_types[i], arg)
            }
        }
        (function.implementation)(Arguments::new(args))
    }
}

#[derive(Debug)]
enum RuntimeType {
    Number,
    String,
    Bool,
}

impl RuntimeValue {
    fn matches_type(&self, runtime_type: &RuntimeType) -> bool {
        match (self, runtime_type) {
            (RuntimeValue::Number(..), RuntimeType::Number) => true,
            (RuntimeValue::Bool(..), RuntimeType::Bool) => true,
            (RuntimeValue::String(..), RuntimeType::String) => true,
            _ => false
        }
    }
}

struct Function {
    name: String,
    expected_params: usize,
    param_types: Vec<RuntimeType>,
    implementation: Box<dyn Fn(Arguments) -> RuntimeValue>
}


struct Arguments {
    args: Vec<RuntimeValue>,
    index: usize
}

impl Arguments {

    pub fn new(args: Vec<RuntimeValue>) -> Self {
        Arguments { args, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn args(&self) -> &Vec<RuntimeValue> {
        &self.args
    }

    fn get<T>(&self, index: usize, extractor: fn(&RuntimeValue) -> Option<&T>, expected: &str) -> &T {
        let runtime_value = self.args.get(index).expect(&format!("Missing argument at position {}", index));
        extractor(&runtime_value).unwrap_or_else(|| {
            panic!("Expected argument at position {} to be a {} but got something else", index, expected);
        })
    }

    pub fn as_str(&self, index: usize) -> &String {
        self.get(index, |rv| {
            match rv {
                RuntimeValue::String(value) => Some(value),
                _ => None
            }
        }, "String")
    }
    pub fn as_bool(&self, index: usize) -> &bool {
        self.get(index, |rv| {
            match rv {
                RuntimeValue::Bool(value) => Some(value),
                _ => None
            }
        }, "Bool")
    }

    pub fn as_f32(&self, index: usize) -> &f32 {
        self.get(index, |rv| {
            match rv {
                RuntimeValue::Number(value) => Some(value),
                _ => None
            }
        }, "Number")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_callback() {
        let mut registry = FunctionRegistry::new();
        registry.add_function(Function {
            name: "sum".to_string(),
            expected_params: 2,
            param_types: vec![RuntimeType::Number, RuntimeType::Number],
            implementation: Box::new(|args: Arguments| -> RuntimeValue {
                let num1 = args.as_f32(0);
                let num2 = args.as_f32(1);
                RuntimeValue::Number(num1 + num2)
            })
        });
        let result = registry.call("sum", vec![RuntimeValue::Number(5.), RuntimeValue::Number(2.)]);
        assert_eq!(result, RuntimeValue::Number(7.));

        registry.add_function(Function {
            name: "concat".to_string(),
            expected_params: 2,
            param_types: vec![RuntimeType::String, RuntimeType::String],
            implementation: Box::new(|args: Arguments| -> RuntimeValue {
                let str1 = args.as_str(0);
                let str2 = args.as_str(1);
                let result = format!("{}{}", str1, str2);
                RuntimeValue::String(result)
            })
        });

        let result2 = registry.call("concat", vec![RuntimeValue::String("hello ".to_owned()), RuntimeValue::String("world".to_owned())]);
        assert_eq!(result2, RuntimeValue::String("hello world".to_owned()));
    }
}