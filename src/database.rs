use crate::{error, types, state};
use mysql::prelude::Queryable;
use snafu::ResultExt; 
use mysql::params;

pub fn check_database(config : &types::Config) -> error::Result<()> {
  let _ = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
  Ok(())
}

pub fn insert_subscriptions(list_name: &str, 
                            addresses : std::vec::Vec<String>,
                            request: &str) -> error::Result<()> {
  let state = state::get_server_state()?;
  let config = state.config;

  let pool = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
  let mut connection = pool.get_conn().context(error::DbConnectionError {})?;
  let get_list_stmt = "SELECT id FROM mailing_lists WHERE email = :email";
  let prep_get_list_stmt = connection.prep(get_list_stmt).context(error::DbPrepareError { statement: get_list_stmt })?;
  let list_id : i32 = connection.exec_first(&prep_get_list_stmt, mysql::params! { "email" => list_name })
                                .context(error::DbExecuteError { statement: get_list_stmt })?
                                .ok_or(error::Error::DbMailingListDoesNotExist { list_name : list_name.to_string() })?;

  Ok(())
}