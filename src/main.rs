use odbc_api::{ConnectionOptions, Environment};

fn main() {
    let env = Environment::new()?;

    let connection_string = "";

    let mut conn =
        env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;
}
