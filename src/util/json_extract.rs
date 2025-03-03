use serde_json::Value;
use ethers::types::U256;
use std::str::FromStr;

// Define a trait with an extraction method
pub trait ExtractableFromJson: Sized {
    fn extract(value: &Value) -> Option<Self>;
}

// Implement the trait for U256
impl ExtractableFromJson for U256 {
    fn extract(value: &Value) -> Option<Self> {

        match value {

            Value::Number( x ) => U256::from_dec_str( &x.to_string()  ).ok()  ,
            _ =>   value.as_str().and_then(|s| U256::from_str(s).ok())
        }
      
    }
}

// Implement the trait for String
impl ExtractableFromJson for String {
    fn extract(value: &Value) -> Option<Self> {
        value.as_str().map(|s| s.to_string())
    }
}

// Implement the trait for bool
impl ExtractableFromJson for bool {
    fn extract(value: &Value) -> Option<Self> {
        value.as_bool()
    }
}


impl ExtractableFromJson for u64 {
    fn extract(value: &Value) -> Option<Self> {
        value.as_u64() 
    }
}



impl ExtractableFromJson for i64 {
    fn extract(value: &Value) -> Option<Self> {
        value.as_i64() 
    }
}



// Generic function to fetch a value of type T from JSON
fn fetch_from_serde_json<T: ExtractableFromJson>(input: &Value, key: &str) -> Option<T> {
    input.get(key).and_then(T::extract)
}
