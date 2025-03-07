#![allow(dead_code)]

use crate::deserialize::FromSqlRow;
use crate::expression::AsExpression;
use crate::sql_types::Json;
#[cfg(feature = "postgres")]
use crate::sql_types::Jsonb;

#[derive(AsExpression, FromSqlRow)]
#[diesel(foreign_derive)]
#[diesel(sql_type = Json)]
#[cfg_attr(feature = "postgres", diesel(sql_type = Jsonb))]
struct SerdeJsonValueProxy(serde_json::Value);
