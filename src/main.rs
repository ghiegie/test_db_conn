fn main() {

}

#[cfg(test)]
mod test {
    //testing connection
    use odbc_api::{Environment, Error, Connection, ConnectionOptions, buffers::{BufferDesc, ColumnarAnyBuffer}, Cursor};
    #[test]
    fn test_conn() {
        // create an odbc env; unwrap for err
        let env = Environment::new().unwrap();

        // create conn str
        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDataBase;Trusted_Connection=yes;";

        // connect_with_connection_string creates a handle for connection
        // takes the constr and connection options with default props
        let mut conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default());

        assert!(!conn.is_err());
    }

    #[test]
    fn test_exec() {
        use odbc_api::{Environment, Error, Connection, ConnectionOptions, buffers::{BufferDesc, ColumnarAnyBuffer}, Cursor};

        let env = Environment::new().expect("ENVIRONMENT NOT CREATED");
        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDataBase;Trusted_Connection=yes;";
        let mut conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("CONNECTION NOT ESTABLISHED");

        let mut all_ints = Vec::new();
        // Batch size determines how many values we fetch at once.
        let batch_size = 100;
        // We expect the first column to hold INTEGERs (or a type convertible to INTEGER). Use
        // the metadata on the result set, if you want to investige the types of the columns at
        // runtime.
        let description = BufferDesc::I32 { nullable: false };
        // This is the buffer we bind to the driver, and repeatedly use to fetch each batch
        let buffer = ColumnarAnyBuffer::from_descs(batch_size, [description]);
        // Bind buffer to cursor
        let mut row_set_buffer = cursor.bind_buffer(buffer)?;
        // Fetch data batch by batch
        while let Some(batch) = row_set_buffer.fetch()? {
            all_ints.extend_from_slice(batch.column(0).as_slice().unwrap())
        }
        Ok(all_ints)
    }

    fn fetch_all_ints(cursor: impl Cursor) -> Result<Vec<i32>, Error> {
        let mut all_ints = Vec::new();
        // Batch size determines how many values we fetch at once.
        let batch_size = 100;
        // We expect the first column to hold INTEGERs (or a type convertible to INTEGER). Use
        // the metadata on the result set, if you want to investige the types of the columns at
        // runtime.
        let description = BufferDesc::I32 { nullable: false };
        // This is the buffer we bind to the driver, and repeatedly use to fetch each batch
        let buffer = ColumnarAnyBuffer::from_descs(batch_size, [description]);
        // Bind buffer to cursor
        let mut row_set_buffer = cursor.bind_buffer(buffer)?;
        // Fetch data batch by batch
        while let Some(batch) = row_set_buffer.fetch()? {
            all_ints.extend_from_slice(batch.column(0).as_slice().unwrap())
        }
        Ok(all_ints)
    }
}