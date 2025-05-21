use std::{collections::HashMap};

use crate::lang::interpreter::RuntimeValue;

use super::native_functions::load_native_functions;

pub struct FunctionRegistry {
    functions: HashMap<String, Function>
}

impl FunctionRegistry {
    pub fn new() -> Self {
        let mut instance = Self {
            functions: HashMap::new()
        };
        load_native_functions(&mut instance);
        instance
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name.to_owned(), function);
    }

    pub fn call(&self, function_name: &str, args: Vec<RuntimeValue>) -> RuntimeValue {
        let function = self.functions.get(function_name).expect(&format!("Function {} does not exist", function_name));
        match function.expected_params {
            ParamCount::Fixed(num) => {
                if num != args.len() {
                    panic!("Function {} expects {} params, got {}", function.name, num, args.len())
                }
            },
            ParamCount::Dynamic(min) => {
                if args.len() < min {
                    panic!("Function {} expects {} params as minimum, got {}", function.name, min, args.len())
                }
            },
        }

        let mut index: usize = 0;
        for (i, arg) in args.iter().enumerate() {
            index = i;
            if !function.expected_params.is_fixed() && i >= function.param_types.len() {
                index = function.param_types.len() - 1;
            }
            if !arg.matches_type(&function.param_types[index]) {
                panic!("Param {} of function {} expected {:?}, got {:?}", i, function.name, function.param_types[index], arg)
            }
        }
        (function.implementation)(Arguments::new(args))
    }
}

#[derive(PartialEq)]
pub enum ParamCount {
    Fixed(usize),
    Dynamic(usize)
}

impl ParamCount {
    pub fn is_fixed(&self) -> bool {
        matches!(&self, Self::Fixed(..))
    }
}

#[derive(Debug)]
pub enum RuntimeType {
    Number,
    String,
    Bool,
    Any,
    Null,
}

impl RuntimeValue {
    fn matches_type(&self, runtime_type: &RuntimeType) -> bool {
        match (self.to_type(), runtime_type) {
            (RuntimeType::Number, RuntimeType::Number) => true,
            (RuntimeType::Bool, RuntimeType::Bool) => true,
            (RuntimeType::String, RuntimeType::String) => true,
            (RuntimeType::Null, RuntimeType::Null) => true,
            (_, RuntimeType::Any) => true,
            _ => false
        }
    }
}

pub struct Function {
    name: String,
    expected_params: ParamCount,
    param_types: Vec<RuntimeType>,
    implementation: Box<dyn Fn(Arguments) -> RuntimeValue>
}

impl Function {
    pub fn new(name: String, expected_params: ParamCount, param_types: Vec<RuntimeType>, implementation: Box<dyn Fn(Arguments) -> RuntimeValue>) -> Self {
        Self { name, expected_params, param_types, implementation }
    }
}


pub struct Arguments {
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

    pub fn has(&self, index: usize) -> bool {
        index < self.args.len()
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

    pub fn as_any(&self, index: usize) -> &RuntimeValue {
        self.get::<RuntimeValue>(index, |rv| {
            match rv {
                value => Some(value)
            }
        }, "Any")
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
            expected_params: ParamCount::Fixed(2),
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
            expected_params: ParamCount::Dynamic(2),
            param_types: vec![RuntimeType::String, RuntimeType::String],
            implementation: Box::new(|args: Arguments| -> RuntimeValue {
                let mut result = String::new();
                for i in 0..args.len() {
                    result.push_str(&args.as_str(i));
                }
                RuntimeValue::String(result)
            })
        });

        let result2 = registry.call("concat", vec![RuntimeValue::String("hello ".to_owned()), RuntimeValue::String("world".to_owned()), RuntimeValue::String(" simon".to_owned())]);
        
        assert_eq!(result2, RuntimeValue::String("hello world simon".to_owned()));

        let result3 = registry.call("print", vec![RuntimeValue::Number(34.)]);
        assert_eq!(result3, RuntimeValue::Null);

        registry.call("print", vec![RuntimeValue::String("Write 'hi!'".to_owned())]);
        let result3 = registry.call("read", vec![]);
        assert_eq!(result3, RuntimeValue::String("hi!".to_string()));
    }
}