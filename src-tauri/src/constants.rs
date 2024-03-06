#[cfg(not(test))]
pub const DATA_DIRECTORY: &str = "data\\";
#[cfg(test)]
pub const DATA_DIRECTORY: &str = "test-resources\\data\\";
#[cfg(not(test))]
pub const PROGRAM_PROPERTIES: &str = "data\\properties.ini";
#[cfg(test)]
pub const PROGRAM_PROPERTIES: &str = "test-resources\\data\\properties.ini";
#[cfg(not(test))]
pub const GROUP_DATA_DIRECTORY: &str = "data\\groups\\";
#[cfg(test)]
pub const GROUP_DATA_DIRECTORY: &str = "test-resources\\data\\groups\\";
pub const PAYER_DATA_DIRECTORY: &str = "payer_data\\";
pub const TRANSACTION_DIRECTORY: &str = "transaction_data\\";
pub const GROUP_DATA_FILE: &str = "g_data";
pub const TRANSACTION_BUNDLE_MAP_FILE: &str = "t_map";
pub const TRANSACTION_BUNDLE_FILE_SUFFIX: &str = ".bundle";
pub const TRANSACTION_BUNDLE_MAP_FILE_SUFFIX: &str = ".map";
pub const DATA_FILES_SUFFIX: &str = ".data";
