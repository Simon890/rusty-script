use std::{fs, io, path::Path};

use crate::lang::interpreter::RuntimeValue;

use super::function_registry::{Function, FunctionRegistry, ParamCount, RuntimeType};

pub fn load_native_functions(fr: &mut FunctionRegistry) {
    fr.add_function(
        Function::new(
            "print".to_string(),
            ParamCount::Fixed(1),
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
            ParamCount::Fixed(0),
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
    fr.add_function(Function::new(
        "random".to_string(),
        ParamCount::Fixed(0),
        vec![],
        Box::new(|_| {
            let random_number = rand::random::<f32>();
            RuntimeValue::Number(random_number)
        })
    ));
    fr.add_function(Function::new(
        "toNumber".to_string(),
        ParamCount::Fixed(1),
        vec![RuntimeType::String],
        Box::new(|args| {
            let str_value = args.as_str(0);
            match str_value.parse::<f32>() {
                Ok(n) => RuntimeValue::Number(n),
                Err(_) => RuntimeValue::Null
            }
        })
    ));
    fr.add_function(Function::new(
        "toString".to_string(),
        ParamCount::Fixed(1),
        vec![RuntimeType::Number],
        Box::new(|args| {
            let num_value = args.as_f32(0);
            RuntimeValue::String(num_value.to_string())
        })
    ));
    fr.add_function(Function::new(
        "substring".to_string(),
        ParamCount::Fixed(3),
        vec![RuntimeType::String, RuntimeType::Number, RuntimeType::Number],
        Box::new(|args| {
            let str_value = args.as_str(0);
            let idx_start = *args.as_f32(1) as usize;
            let idx_end = *args.as_f32(2) as usize;
            let sub_str = &str_value[idx_start..=idx_end];
            RuntimeValue::String(sub_str.to_string())
        })
    ));
    fr.add_function(Function::new(
        "writeFile".to_string(),
        ParamCount::Fixed(2),
        vec![RuntimeType::String, RuntimeType::String],
        Box::new(|args| {
            let path = args.as_str(0);
            let contents = args.as_str(1);
            match fs::write(path, contents) {
                Ok(_) => RuntimeValue::Bool(true),
                Err(_) => RuntimeValue::Bool(false)
            }
        })
    ));
    fr.add_function(Function::new(
        "readFile".to_string(),
        ParamCount::Fixed(1),
        vec![RuntimeType::String],
        Box::new(|args| {
            let path = args.as_str(0);
            match fs::read_to_string(path) {
                Ok(content) => RuntimeValue::String(content),
                Err(_) => RuntimeValue::Null
            }
        })
    ));
    fr.add_function(Function::new(
        "deleteFile".to_string(),
        ParamCount::Fixed(1),
        vec![RuntimeType::String],
        Box::new(|args| {
            let path = args.as_str(0);
            match fs::remove_file(path) {
                Ok(_) => RuntimeValue::Bool(true),
                Err(_) => RuntimeValue::Bool(false)
            }
        })
    ));
    fr.add_function(Function::new(
        "exists".to_string(),
        ParamCount::Fixed(1),
        vec![RuntimeType::String],
        Box::new(|args| {
            let path = args.as_str(0);
            RuntimeValue::Bool(Path::new(path).exists())
        })
    ));
}