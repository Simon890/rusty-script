use std::io;

use crate::lang::interpreter::RuntimeValue;

use super::function_registry::{Function, FunctionRegistry, RuntimeType};

pub fn load_native_functions(fr: &mut FunctionRegistry) {
    fr.add_function(
        Function::new(
            "print".to_string(),
            1,
            vec![RuntimeType::Any],
            Box::new(|args| {
                let value = args.as_any(0);
                match value {
                    RuntimeValue::Null => println!("null"),
                    RuntimeValue::Bool(value) => println!("{}", value),
                    RuntimeValue::Number(value) => println!("{}", value),
                    RuntimeValue::String(value) => println!("{}", value)
                }
                RuntimeValue::Null
            })
        )
    );
    fr.add_function(
        Function::new(
            "read".to_string(),
            0,
            vec![],
            Box::new(|_| {
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Error when reading from console");
                if value.ends_with("\n") {
                    value.pop();
                }
                RuntimeValue::String(value)
            })
        )
    );
}