use odbc_api::{ConnectionOptions, Environment};

fn main() {
}

#[cfg(test)]
mod test {
    use odbc_api::{buffers::{BufferDesc, ColumnarAnyBuffer}, Environment, ConnectionOptions, IntoParameter, Cursor};

    #[test]
    fn test_conn() {
        use odbc_api::{ConnectionOptions, Environment};

        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P;Database=TicketingDB;Trusted_Connection=yes;";

        let mut conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default());

        assert!(!conn.is_err());
    }

    #[test]
    fn test_query() {
        use odbc_api::{ConnectionOptions, Environment, Connection, Error, IntoParameter, Cursor};

        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P;Database=TicketingDB;Trusted_Connection=yes;";

        let mut conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        let query = "select * from ProductTbl";

        let mut cursor = conn.execute(query, ()).expect("FAILED TO EXECUTE QUERY").expect("NO VALUE DETECTED");

        let mut buf = Vec::new();

        if let Some(mut row) = cursor.next_row().expect("NO ROWS TO ITERATE") {
            row.get_text(1, &mut buf).expect("NO VALUE TO GET");
        }

        let ret = String::from_utf8(buf);

        println!("{}", ret.clone().unwrap());

        assert!(!ret.is_err());
    }

    #[test]
    fn test_iterate_query() {
        use odbc_api::{ConnectionOptions, Environment, Cursor};

        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P;Database=TicketingDB;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        let query = "select * from ItemSubCategoryTbl";

        let mut cursor = conn.execute(query, ()).expect("FAILED TO EXECUTE QUERY").expect("NO VALUE DETECTED");

        loop {
            if let Ok(somthing) = cursor.next_row() {
                if let Some(mut obj) = somthing {
                    let mut item: i32 = 0;
                    obj.get_data(6, &mut item).unwrap();
                    println!("{}", item);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    #[test]
    fn test_query_using_buffer() {
        use odbc_api::{ConnectionOptions, Environment, Cursor};

        let batch_size = 1000;
        let buffer_description = [
            BufferDesc::Text { max_str_len: 255 },
            BufferDesc::I32 { nullable: false },
        ];
        let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);


        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P;Database=TicketingDB;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        if let Some(cursor) = conn.execute("select SubCatgID, Quantity from ItemSubCategoryTbl", ()).unwrap() {
            // bind cursor to buffer
            let mut row_set_cursor = cursor.bind_buffer(&mut buffer).unwrap();

            // loop over sets
            while let Some(row_set) = row_set_cursor.fetch().unwrap() {
                let item1 = row_set.column(0);
                for i in item1.as_text_view().unwrap().iter() {
                    if let Some(a) = i {
                        let str = String::from_utf8(a.to_vec()).unwrap();
                        println!("{:#?}", str);
                    } else {
                        println!("NONE");
                    }
                }

                println!("*******");

                let item2 = row_set.column(1);
                if let Some(i) = item2.as_slice::<i32>() {
                    for a in i {
                        println!("{}", a);
                    }
                }
            }
        };
    }

    #[test]
    fn test_create() {
        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        let to_insert = "prd5";
        conn.execute("insert into ProductTbl(Product) values(?)", (&to_insert.into_parameter())).expect("FAILED TO INSERT");
    }

    #[test]
    fn test_read() {
        let batch_size = 1000;
        let buffer_description = [
            BufferDesc::I32 { nullable: false },
            BufferDesc::Text { max_str_len: 255 },
        ];
        let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_description);

        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        if let Some(cursor) =  conn.execute("select * from ProductTbl", ()).expect("FAILED TO CREATE CURSOR FROM QUERY") {
            let mut row_set_cursor = cursor.bind_buffer(&mut buffer).expect("FAILED TO BIND CURSOR TO GIVEN BUFFER");

            while let Some(row_set) = row_set_cursor.fetch().expect("FAILED TO FETCH DATA FROM BUFFER") {
                let col1 = row_set.column(0);
                if let Some(a) = col1.as_slice::<i32>() {
                    for b in a {
                        println!("{}", b);
                    }
                }

                let col2 = row_set.column(1);
                if let Some(a) =  col2.as_text_view() {
                    for b in a.iter() {
                        if let Some(c) = b {
                            let str = String::from_utf8(c.to_vec()).unwrap();
                            println!("{}", str);
                        } else {
                            println!("NONE");
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn test_update() {
        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        let to_mod = "prd8";
        let to_rep = "prd7";
        conn.execute("update ProductTbl set Product=? where Product=?", (&to_rep.into_parameter(), &to_mod.into_parameter())).expect("FAILED TO UPDATE");
    }

    #[test]
    fn test_delete() {
        let env = Environment::new().expect("FAILED TO CREATE ENV");

        let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";

        let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default()).expect("FAILED TO CREATE CONNECTION");

        let to_del = "prd7";
        conn.execute("delete from ProductTbl where Product=?", (&to_del.into_parameter())).expect("FAILED TO DELETE");
    }
}