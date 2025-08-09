use std::fmt;
use uuid::Uuid;
use ::serde::{Serialize, Deserialize};
use super::frequency::Frequency;
use super::record_type::RecordType;

/// ——————————————————————————————————————————————
/// Financial Record: income, expense, debt,
/// ——————————————————————————————————————————————
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialRecord {
    pub id: Uuid,

    pub name: String,
    pub amount: f64,

    pub frequency: Frequency,
    pub record_type: RecordType,
}

impl FinancialRecord {
    pub fn new(
        name: impl Into<String>,
        amount: f64,
        frequency: Frequency,
        record_type: RecordType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            amount,
            frequency,
            record_type,
        }
    }
}

impl fmt::Display for FinancialRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | ${:.2} | {} | {}",
            self.id, self.name, self.amount, self.frequency, self.record_type
        )
    }
}


