use std::error::Error;
use pg_rs::protocol::connection::Connection;
use pg_rs::protocol::conn_params::ConnParamsBuilder;
use pg_rs::messages::back::BackTypes;
use pg_rs::messages::front_builder::StartupMessageBuilder;

#[test]
fn test_conn_new() -> Result<(), Box<dyn Error>> {
    let params = ConnParamsBuilder::new()
        .build();
    Connection::new(&params)?;
    Ok(())
}

#[test]
fn test_conn_auth() -> Result<(), Box<dyn Error>> {
    let params = ConnParamsBuilder::new()
        .build();
    let mut conn = Connection::new(&params)?;
    let mut auth_message = StartupMessageBuilder::new()
        .build();
    let test_response = conn.send_message(&mut auth_message)?;

    match test_response.msg_type() {
        BackTypes::AuthenticationOk => Ok(()),
        _ => Err("User did not log in".into()),
    }
}

