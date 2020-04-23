

#[cfg(test)]
mod tests {
    use std::error::Error;
    use csv::ReaderBuilder;

    #[test]
    fn test_csv_commas() -> Result<(), Box<dyn Error>> {
        let data = "city,country,pop\nBoston,United States,4628910\n";
        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(data.as_bytes());
        if let Some(result) = rdr.records().next() {
            let record = result?;
            assert_eq!(record, vec!["Boston", "United States", "4628910"]);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }

    #[test]
    fn test_csv_semicolons() -> Result<(), Box<dyn Error>> {
        let data = "city;country;pop\nBoston;United States;4628910\n";
        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(data.as_bytes());
        if let Some(result) = rdr.records().next() {
            let record = result?;
            assert_eq!(record, vec!["Boston", "United States", "4628910"]);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }

    #[test]
    fn test_csv_tabs() -> Result<(), Box<dyn Error>> {
        let data = "city\tcountry\tpop\nBoston\tUnited States\t4628910\n";
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(data.as_bytes());
        if let Some(result) = rdr.records().next() {
            let record = result?;
            assert_eq!(record, vec!["Boston", "United States", "4628910"]);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }

    #[test]
    fn test_csv_tildes() -> Result<(), Box<dyn Error>> {
        let data = "city~country~pop\nBoston~United States~4628910\n";
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'~')
            .from_reader(data.as_bytes());
        if let Some(result) = rdr.records().next() {
            let record = result?;
            assert_eq!(record, vec!["Boston", "United States", "4628910"]);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
}
