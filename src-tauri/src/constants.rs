#[cfg(not(test))]
pub const DATA_DIRECTORY: &str = "data\\";
#[cfg(test)]
pub const DATA_DIRECTORY: &str = "test-resources\\data\\";
#[cfg(not(test))]
pub const PAYER_DATA_DIRECTORY: &str = "data\\payer_data\\";
#[cfg(test)]
pub const PAYER_DATA_DIRECTORY: &str = "test-resources\\data\\payer_data\\";
#[cfg(not(test))]
pub const TRANSACTION_DIRECTORY: &str = "data\\transaction_data\\";
#[cfg(test)]
pub const TRANSACTION_DIRECTORY: &str = "test-resources\\data\\transaction_data\\";
#[cfg(not(test))]
pub const PROGRAM_PROPERTIES: &str = "data\\properties.ini";
#[cfg(test)]
pub const PROGRAM_PROPERTIES: &str = "test-resources\\data\\properties.ini";
