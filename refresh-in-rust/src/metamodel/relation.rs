// Enumerable intrinsic types allowable
// TODO: Add more types
#[derive(PartialEq, Debug)]
pub enum Type {
    Text,
    Boolean(String, String),
}

impl Type {
    fn as_string(&self) -> String {
        match self {
            Type::Text => String::from("Text"),
            Type::Boolean(when_true, when_false) => format!(
                "Boolean(True: \"{}\", False: \"{}\")",
                when_true.as_str(),
                when_false.as_str()
            ),
        }
    }
}

// pub enum StringValue {
//     String,
// }

// pub enum BooleanValue {
//     bool,
// }

// pub struct TypeValue<T, V> {
//     r#type: T,
//     value: V,
// }

// pub enum Value {
//     Text
// }

// pub enum Value {
//     Text(String),
//     Boolean(bool),
//     Boolean(bool, String, String),
// }

// impl TypeValue<Type> {
//     fn as_string(&self) => String {
//         self
//     }
// }

// Enumerable values for allowable intrinsic types
// TODO: Add an enumerable value for each Value Type
// pub enum Value {
//     Type::Text!(String), // this needs to be an owned String
//     Type::Boolean(String, String)(bool),
// }

// impl Value {
//     fn as_str(&self) -> &str {
//         match self {
//             Type::Text(value) => self,
//             Type::Boolean(value, when_true, when_false) => {
//                 if value {
//                     when_true
//                 } else {
//                     when_false
//                 }
//             },
//         }
//     }

//     fn as_string(&self) -> String {
//         String::from(self.as_str())
//     }
// }

// pub struct Column {
//     name: String,
//     type: Type,
// }

// pub struct Connection {
//     driver: String,
//     connectionString: String,
// }

// pub struct Cell<T> {
//     type: T,
//     value: Value<T>,
// }

// pub struct Row {
//     cells: Vec<Cell>,
// }

// pub struct Relation {
//     connection: Connection,
//     columns: Vec<Column>,
// }

// pub struct DataSet {
//     relation: Relation,
//     rows: Vec<Row>,
// }

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn inspect_type_text() {
        let expected_type: Type = Type::Text;
        const EXPECTED_TYPE_STRING: &'static str = "Text";
        let actual_type: Type = Type::Text;
        let actual_type_string: String = actual_type.as_string();
        assert_eq!(actual_type, expected_type);
        assert_eq!(actual_type_string, String::from(EXPECTED_TYPE_STRING));
    }

    #[test]
    fn inspect_type_boolean() {
        let expected_type_1: Type = Type::Boolean(String::from("true"), String::from("false"));
        let expected_type_2: Type = Type::Boolean(String::from("yes"), String::from("no"));
        let expected_type_3: Type = Type::Boolean(String::from("on"), String::from("off"));
        let expected_type_4: Type = Type::Boolean(String::from("1"), String::from("0"));
        const EXPECTED_TYPE_1_STRING: &'static str = "Boolean(True: \"true\", False: \"false\")";
        const EXPECTED_TYPE_2_STRING: &'static str = "Boolean(True: \"yes\", False: \"no\")";
        const EXPECTED_TYPE_3_STRING: &'static str = "Boolean(True: \"on\", False: \"off\")";
        const EXPECTED_TYPE_4_STRING: &'static str = "Boolean(True: \"1\", False: \"0\")";
        let actual_type_1: Type = Type::Boolean(String::from("true"), String::from("false"));
        let actual_type_2: Type = Type::Boolean(String::from("yes"), String::from("no"));
        let actual_type_3: Type = Type::Boolean(String::from("on"), String::from("off"));
        let actual_type_4: Type = Type::Boolean(String::from("1"), String::from("0"));
        let actual_type_1_string: String = actual_type_1.as_string();
        let actual_type_2_string: String = actual_type_2.as_string();
        let actual_type_3_string: String = actual_type_3.as_string();
        let actual_type_4_string: String = actual_type_4.as_string();
        assert_eq!(actual_type_1, expected_type_1);
        assert_eq!(actual_type_2, expected_type_2);
        assert_eq!(actual_type_3, expected_type_3);
        assert_eq!(actual_type_4, expected_type_4);
        assert_eq!(actual_type_1_string, String::from(EXPECTED_TYPE_1_STRING));
        assert_eq!(actual_type_2_string, String::from(EXPECTED_TYPE_2_STRING));
        assert_eq!(actual_type_3_string, String::from(EXPECTED_TYPE_3_STRING));
        assert_eq!(actual_type_4_string, String::from(EXPECTED_TYPE_4_STRING));
    }
}
