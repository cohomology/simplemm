use crate::types::{Result, Config, DbConnectionError};
use diesel::mysql::MysqlConnection;
use diesel::Connection;
use snafu::ResultExt; 

pub fn check_database(config : &Config) -> Result<()> {
  MysqlConnection::establish(&config.db_url).context(DbConnectionError {})?;
  Ok(())
}
