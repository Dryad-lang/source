// tests/oop_tests.rs
// Tests for Object-Oriented Programming features in Dryad

use crate::test_runner::TestRunner;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::env::Env;

#[cfg(test)]
mod oop_tests {
    use super::*;

    #[test]
    fn test_static_method_basic() {
        let code = r#"
            class Math {
                public static fun add(a, b) {
                    return a + b;
                }
            }
            
            let result = Math.add(5, 3);
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        // Check that result is 8
        if let Some(result_value) = env.get("result") {
            match result_value {
                crate::interpreter::env::Value::Number(n) => assert_eq!(*n, 8.0),
                _ => panic!("Expected number result"),
            }
        } else {
            panic!("Expected result variable");
        }
    }

    #[test]
    fn test_static_method_visibility() {
        let code = r#"
            class Utils {
                public static fun publicMethod() {
                    return "public";
                }
                
                private static fun privateMethod() {
                    return "private";
                }
            }
            
            let result = Utils.publicMethod();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        if let Some(result_value) = env.get("result") {
            match result_value {
                crate::interpreter::env::Value::String(s) => assert_eq!(s, "public"),
                _ => panic!("Expected string result"),
            }
        }
    }

    #[test]
    fn test_mixed_static_and_instance_methods() {
        let code = r#"
            class Calculator {
                let value;
                
                public fun constructor(initialValue) {
                    this.value = initialValue;
                }
                
                public fun getValue() {
                    return this.value;
                }
                
                public static fun pi() {
                    return 3.14159;
                }
            }
            
            let staticResult = Calculator.pi();
            let calc = new Calculator(42);
            let instanceResult = calc.getValue();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        // Check static method result
        if let Some(static_value) = env.get("staticResult") {
            match static_value {
                crate::interpreter::env::Value::Number(n) => assert_eq!(*n, 3.14159),
                _ => panic!("Expected number for static result"),
            }
        }
        
        // Check instance method result
        if let Some(instance_value) = env.get("instanceResult") {
            match instance_value {
                crate::interpreter::env::Value::Number(n) => assert_eq!(*n, 42.0),
                _ => panic!("Expected number for instance result"),
            }
        }
    }

    #[test]
    fn test_static_method_calling_static_method() {
        let code = r#"
            class Math {
                public static fun square(x) {
                    return Math.multiply(x, x);
                }
                
                public static fun multiply(a, b) {
                    return a * b;
                }
            }
            
            let result = Math.square(5);
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        if let Some(result_value) = env.get("result") {
            match result_value {
                crate::interpreter::env::Value::Number(n) => assert_eq!(*n, 25.0),
                _ => panic!("Expected number result"),
            }
        }
    }

    #[test]
    fn test_static_method_access_this_error() {
        let code = r#"
            class InvalidClass {
                let field;
                
                public static fun badMethod() {
                    return this.field;
                }
            }
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        // This should produce an error
        assert!(result.is_err() || runner.has_errors());
    }

    #[test]
    fn test_static_method_access_instance_field_error() {
        let code = r#"
            class InvalidClass {
                let instanceField;
                
                public static fun badMethod() {
                    return instanceField;
                }
            }
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        // This should produce an error
        assert!(result.is_err() || runner.has_errors());
    }

    #[test]
    fn test_static_method_called_on_instance_error() {
        let code = r#"
            class TestClass {
                public static fun staticMethod() {
                    return "static";
                }
            }
            
            let instance = new TestClass();
            let result = instance.staticMethod();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        // This should produce an error
        assert!(result.is_err() || runner.has_errors());
    }

    #[test]
    fn test_instance_method_called_as_static_error() {
        let code = r#"
            class TestClass {
                public fun instanceMethod() {
                    return "instance";
                }
            }
            
            let result = TestClass.instanceMethod();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        // This should produce an error
        assert!(result.is_err() || runner.has_errors());
    }

    #[test]
    fn test_basic_class_instantiation() {
        let code = r#"
            class Person {
                let name;
                
                public fun constructor(n) {
                    this.name = n;
                }
                
                public fun getName() {
                    return this.name;
                }
            }
            
            let person = new Person("Alice");
            let name = person.getName();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        if let Some(name_value) = env.get("name") {
            match name_value {
                crate::interpreter::env::Value::String(s) => assert_eq!(s, "Alice"),
                _ => panic!("Expected string name"),
            }
        }
    }

    #[test]
    fn test_method_visibility() {
        let code = r#"
            class TestClass {
                public fun publicMethod() {
                    return "public";
                }
                
                private fun privateMethod() {
                    return "private";
                }
                
                protected fun protectedMethod() {
                    return "protected";
                }
            }
            
            let obj = new TestClass();
            let result = obj.publicMethod();
        "#;
        
        let mut runner = TestRunner::new();
        let result = runner.run_code(code);
        
        assert!(result.is_ok());
        let env = result.unwrap();
        
        if let Some(result_value) = env.get("result") {
            match result_value {
                crate::interpreter::env::Value::String(s) => assert_eq!(s, "public"),
                _ => panic!("Expected string result"),
            }
        }
    }
}
