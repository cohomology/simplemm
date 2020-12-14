use crate::{error, state, types};
use mysql::{params, prelude::Queryable};
use snafu::ResultExt;

pub fn check_database(config: &types::Config) -> error::Result<()> {
    let _ = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
    Ok(())
}

pub fn insert_subscriptions(
    list_name: &str,
    subscriptions: std::vec::Vec<types::Subscription>,
    request: &str,
    process_subscription: fn(&types::Subscription) -> error::Result<()>,
) -> error::Result<()> {
    let state = state::get_server_state()?;
    let config = state.config;

    let pool = mysql::Pool::new(&config.db_url).context(error::DbConnectionError {})?;
    let mut connection = pool.get_conn().context(error::DbConnectionError {})?;
    let mut transaction = connection
        .start_transaction(mysql::TxOpts::default())
        .context(error::DbStartTransactionError {})?;
    let get_list_stmt = r"SELECT id FROM mailing_lists WHERE email = :email";
    let prep_get_list_stmt = transaction
        .prep(get_list_stmt)
        .context(error::DbPrepareError {
            statement: get_list_stmt,
        })?;
    let list_id: i32 = transaction
        .exec_first(&prep_get_list_stmt, params! { "email" => list_name })
        .context(error::DbExecuteError {
            statement: get_list_stmt,
        })?
        .ok_or(error::Error::DbMailingListDoesNotExist {
            list_name: list_name.to_string(),
        })?;
    let insert_statement = r"INSERT INTO subscriptions (uuid, list_id, email, request)
                           VALUES (:uuid, :list_id, :email, :request)";

    let insert_result = transaction.exec_batch(
        insert_statement,
        subscriptions.iter().map(|s| {
            params! { "uuid" => s.uuid.clone(),
                                            "list_id" => list_id,
                                            "email" => s.email.clone(),
                                            "request" => request,
            }
        }),
    );
    if let Err(err) = insert_result {
        transaction
            .rollback()
            .context(error::DbRollbackTransactionError {})?;
        return Err(err).context(error::DbExecuteError {
            statement: insert_statement,
        });
    }
    for subscription in subscriptions.iter() {
        if let Err(err) = process_subscription(&subscription) {
            transaction
                .rollback()
                .context(error::DbRollbackTransactionError {})?;
            return Err(err);
        }
    }
    transaction
        .commit()
        .context(error::DbCommitTransactionError {})?;
    Ok(())
}
