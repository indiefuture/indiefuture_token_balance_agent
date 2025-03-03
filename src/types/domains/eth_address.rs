use serde::Serializer;
use serde::Deserializer;
use serde::Serialize;
use serde::Deserialize;
use bytes::BytesMut;
use ethabi::Address;
use ethers::utils::to_checksum;
use std::{error::Error, str::FromStr};
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug,Clone,Eq,PartialEq )]
pub struct DomainEthAddress(pub Address);

impl<'a> FromSql<'a> for DomainEthAddress {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        let address = Address::from_str(s)?;
        //   let address_string = to_checksum(s, None)

        Ok(DomainEthAddress(address))
    }

    fn accepts(sql_type: &Type) -> bool {
       

        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT

    }
}

impl ToSql for DomainEthAddress {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let address_string = format!("{}", to_checksum(&self.0, None));

        println!("inserting {}", address_string);

        <&str as ToSql>::to_sql(&address_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type == &Type::VARCHAR || sql_type == &Type::TEXT
    }

    to_sql_checked!();
}


impl DomainEthAddress {


    pub fn to_string_full(&self) -> String {

        format!( "{:?}" , self.0  )
    }

}



impl Serialize for DomainEthAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize Address as a checksum-encoded string
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}

impl<'de> Deserialize<'de> for DomainEthAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a string into Address
        let s = String::deserialize(deserializer)?;
        Address::from_str(&s)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}