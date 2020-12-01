use crate::{error, types};
use diesel::Connection;
use snafu::ResultExt; 

pub fn check_database(config : &types::Config) -> error::Result<()> {
  diesel::mysql::MysqlConnection::establish(&config.db_url).context(error::DbConnectionError {})?;
  Ok(())
}
