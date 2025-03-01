use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    println!("Testing serde_v8 with Rust 2021 edition");
    
    // Initialize V8
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    
    {
        // Create a new isolate and context
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        
        // Create a handle scope
        let handle_scope = &mut v8::HandleScope::new(isolate);
        
        // Create a context
        let context = v8::Context::new(handle_scope);
        
        // Enter the context for compiling and running scripts
        let scope = &mut v8::ContextScope::new(handle_scope, context);
        
        // Get the handle scope from the context scope
        let handle_scope = &mut v8::HandleScope::new(scope);
        
        // Create a person object
        let person = Person {
            name: "John Doe".to_string(),
            age: 30,
        };
        
        // Serialize the person to a V8 value
        let v8_value = match serde_v8::to_v8(handle_scope, &person) {
            Ok(value) => value,
            Err(err) => {
                println!("Error serializing to V8: {:?}", err);
                return;
            }
        };
        
        println!("Successfully serialized Person to V8 value");
        
        // Deserialize the V8 value back to a Person
        let deserialized_person: Person = match serde_v8::from_v8(handle_scope, v8_value) {
            Ok(person) => person,
            Err(err) => {
                println!("Error deserializing from V8: {:?}", err);
                return;
            }
        };
        
        println!("Successfully deserialized V8 value to Person: {:?}", deserialized_person);
    }
    
    // Clean up V8
    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
} 