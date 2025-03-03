use serde::de;
use serde::de::Visitor;
 
use std::fmt;
use std::str::FromStr;
use serde::Serializer;
use serde::Deserializer;
use serde::Serialize;
use serde::Deserialize;
use bytes::BytesMut;
use ethers::types::U256;

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug,Clone,Eq,PartialEq) ]
pub struct DomainUint256(pub U256);

impl<'a> FromSql<'a> for DomainUint256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
      // Ensure the type is NUMERIC
        if *ty != Type::NUMERIC {
            return Err("Type mismatch: expected NUMERIC".into());
        }

        // Parse the raw bytes into a Decimal
        let decimal = Decimal::from_sql(ty, raw)?;

        // Convert Decimal to string
        let decimal_str = decimal.to_string();

        // Parse the string into U256
        let u256_value = U256::from_dec_str(&decimal_str)?;

        Ok(DomainUint256(u256_value))
    }

    fn accepts(sql_type: &Type) -> bool {
      

          sql_type == &Type::NUMERIC

    }
}

impl ToSql for DomainUint256 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {

       // Convert U256 to a string
        let uint_string = self.0.to_string();

        // Parse the string into a rust_decimal::Decimal
        let decimal_value = Decimal::from_str(&uint_string)
            .map_err(|e| format!("Failed to parse U256 as Decimal: {}", e))?;

        // Delegate to Decimal's ToSql implementation
        decimal_value.to_sql(ty, out)


    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::NUMERIC
    }

    to_sql_checked!();
}




impl Serialize for DomainUint256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize U256 as a hexadecimal string
        serializer.serialize_str(&format!("{}", self.0. to_string()  ))
    }
}


/*
impl<'de> Deserialize<'de> for DomainUint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a string into U256
         let s = String::deserialize(deserializer)?;
        U256::from_str(&s)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
 
 */




impl<'de> Deserialize<'de> for DomainUint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DomainUint256Visitor;

        impl<'de> Visitor<'de> for DomainUint256Visitor {
            type Value = DomainUint256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer representing a U256 value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                U256::from_dec_str(value)
                    .map(DomainUint256)
                    .map_err(de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DomainUint256(U256::from(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    Err(de::Error::custom("negative value cannot be converted to U256"))
                } else {
                    Ok(DomainUint256(U256::from(value as u64)))
                }
            }
        }

        deserializer.deserialize_any(DomainUint256Visitor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};
    use ethers::types::U256;

    #[test]
    fn test_deserialize_from_string() {
 
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(123456789u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::Str("123456789")];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }

    #[test]
    fn test_deserialize_from_integer() {
        // The U256 value we expect after deserialization
        let expected = DomainUint256(U256::from(987654321u64));

        // The sequence of tokens representing the serialized form
        let tokens = &[Token::U64(987654321)];

        // Assert that deserializing these tokens produces the expected value
        assert_de_tokens(&expected, tokens);
    }
}
