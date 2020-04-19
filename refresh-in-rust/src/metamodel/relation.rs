pub enum DataType {
    Text,
}

pub struct Column {
    name: String,
    type: DataType,
}

pub struct Connection {
    driver: String,
    connectionString: String,
}

pub struct Relation {
    connection: Connection,
    columns: Vec<Column>,
    rows_returned: u128,
}
