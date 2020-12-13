use crate::{error, types, state};
use mysql::{params, prelude::Queryable};
use snafu::ResultExt;

pub fn check_database(config : &types::Config) -> error::Result<()> {
  let _ = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
  Ok(())
}

pub fn insert_subscriptions(list_name: &str, 
                            subscriptions : std::vec::Vec<types::Subscription>,
                            request: &str,
                            process_subscription: fn(&types::Subscription) -> error::Result<()>) -> error::Result<()>  {
  let state = state::get_server_state()?;
  let config = state.config;

  let pool = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
  let mut connection = pool.get_conn().context(error::DbConnectionError {})?;
   let get_list_stmt = r"SELECT id FROM mailing_lists WHERE email = :email";
  let prep_get_list_stmt = connection.prep(get_list_stmt).context(error::DbPrepareError { statement: get_list_stmt })?;
  let list_id : i32 = connection.exec_first(&prep_get_list_stmt, mysql::params! { "email" => list_name })
                                 .context(error::DbExecuteError { statement: get_list_stmt })?
                                 .ok_or(error::Error::DbMailingListDoesNotExist { list_name : list_name.to_string() })?;
  let insert_statement = r"INSERT INTO subscriptions (uuid, list_id, email, request)
                           VALUES (:uuid, :list_id, :email, :request)";

  for subscription in subscriptions.iter() {                           
    let mut transaction = connection.start_transaction(mysql::TxOpts::default())
                                    .context(error::DbStartTransactionError {})?;
    let insert_result = transaction.exec_drop(insert_statement, mysql::params! { "uuid" => subscription.uuid.clone(),
                                                                                  "list_id" => list_id,
                                                                                  "email" => subscription.email.clone(),
                                                                                  "request" => request,
    });
    if let Err(_) = insert_result {
      transaction.rollback().context(error::DbRollbackTransactionError {})?;
    } else {  
      if let Err(err) = process_subscription(&subscription) {
        transaction.rollback().context(error::DbRollbackTransactionError {})?;
        return Err(err);
      } else {
        transaction.commit().context(error::DbCommitTransactionError {})?;
      }
    }   
  }
  Ok(())
}