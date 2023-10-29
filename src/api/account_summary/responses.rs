use serde::Deserialize;

/// Currency position.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyPosition {
    /// Currency code.
    pub code: String,
    /// Value of position.
    pub value: String,
    /// Converted value of position if cross rates are available.
    pub converted_value: String,
}

/// Instrument position.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentPosition {
    /// Financial instrument identifier.
    pub symbol_id: String,
    /// Financial instrument type.
    pub symbol_type: String,
    /// Quantity on position.
    pub quantity: String,
    /// Currency code of the financial instrument.
    pub currency: String,
    /// Current financial instrument price.
    pub price: Option<String>,
    /// Average position opening price.
    pub average_price: Option<String>,
    /// Current position PnL.
    pub pnl: Option<String>,
    /// Current position PnL in the currency of the report.
    pub converted_pnl: Option<String>,
    /// Position value.
    pub value: Option<String>,
    /// Position value in the currency of the report.
    pub converted_value: Option<String>,
    /// Accrued interest in the currency of the report (bonds only).
    pub accrued_interest: Option<String>,
}

/// Account summary.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    /// User account ID.
    pub account_id: String,
    /// Currency of the report.
    pub currency: String,
    /// Session date of the report.
    pub session_date: Option<String>,
    /// Timestamp of the report.
    pub timestamp: u64,
    /// Total NAV of user in the currency of the report.
    pub net_asset_value: Option<String>,
    /// Free money in the currency of the report.
    pub free_money: Option<String>,
    /// Money used for margin in the currency of the report.
    pub money_used_for_margin: Option<String>,
    /// Margin utilization in fraction of NAV.
    pub margin_utilization: Option<String>,
    /// Currencies on position.
    pub currencies: Vec<CurrencyPosition>,
    /// Open positions.
    pub positions: Vec<InstrumentPosition>,
}
