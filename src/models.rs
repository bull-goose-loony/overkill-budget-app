use serde::{Serialize, Deserialize};
use std::fmt;
use uuid::Uuid;
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

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: Uuid,
    pub name: String, // i.e. Electric Bill
    pub amount: f64, // i.e. 16.42
    pub frequency: Frequency, // i.e. MONTHLY
    pub record_type: RecordType, // i.e. EXPENSE
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | ${:.2} | {:?} | {:?}",
            self.id,
            self.name,
            self.amount,
            self.frequency,
            self.record_type
        )
    }
}

