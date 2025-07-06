
use serde::{Serialize, Deserialize};
use std::fmt;
use rusqlite::types::{ToSql, ToSqlOutput, ValueRef, FromSql, FromSqlResult, FromSqlError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

impl std::str::FromStr for Frequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Daily" => Ok(Frequency::Daily),
            "Weekly" => Ok(Frequency::Weekly),
            "Monthly" => Ok(Frequency::Monthly),
            "Quarterly" => Ok(Frequency::Quarterly),
            "Yearly" => Ok(Frequency::Yearly),
            _ => Err(()),
        }
    }
}

// Implement Display (gives you .to_string())
impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Frequency::Daily => "Daily",
            Frequency::Weekly => "Weekly",
            Frequency::Monthly => "Monthly",
            Frequency::Quarterly => "Quarterly",
            Frequency::Yearly => "Yearly",
        };
        write!(f, "{}", s)
    }
}

impl ToSql for Frequency {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for Frequency {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        // expect a TEXT column
        let s = value.as_str()?;             // if it's not TEXT, this errors
        s.parse::<Frequency>()               // use your FromStr
            .map_err(|_| FromSqlError::Other("invalid frequency".into()))
    }
}
