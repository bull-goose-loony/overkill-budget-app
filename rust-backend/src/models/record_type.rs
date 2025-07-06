
use serde::{Serialize, Deserialize};
use std::fmt;
use rusqlite::types::{ToSql, ToSqlOutput, ValueRef, FromSql, FromSqlResult, FromSqlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    Income,
    Expense,
    Debt,
}

impl std::str::FromStr for RecordType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Income" => Ok(RecordType::Income),
            "Expense" => Ok(RecordType::Expense),
            "Debt" => Ok(RecordType::Debt),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RecordType::Income => "Income",
            RecordType::Expense => "Expense",
            RecordType::Debt => "Debt",
        };
        write!(f, "{}", s)
    }
}

impl ToSql for RecordType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for RecordType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s = value.as_str()?;
        s.parse::<RecordType>()
            .map_err(|_| FromSqlError::Other("invalid record_type".into()))
    }
}
