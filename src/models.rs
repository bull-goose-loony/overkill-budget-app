use serde::{Serialize, Deserialize};
use rusqlite::types::{ToSql, FromSql};

#[derive(Debug, Serialize, Deserialize)]
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
            "Yearly" => Ok(Frequency::Yearly),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Record {
    name: String, // i.e. Electric Bill
    amount: f64, // i.e. 16.42
    frequency: Frequency, // i.e. MONTHLY
    record_type: RecordType, // i.e. EXPENSE
}
