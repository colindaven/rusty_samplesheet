extern crate csv;

use std::error::Error;
use csv::Reader;
//use csv::StringRecord;
use std::collections::HashMap; 
use std::process;
use std::str;

fn check_csv() -> Result<(), Box<dyn Error>> {
    let mut sample_id_count : u16 = 0;

    // Setup file and read
    let mut rdr = Reader::from_path("SampleSheet_2020_032049.csv")?;
    // Iterate through results.
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);


        for field in record.iter() {

            //Exit on Umlaut 
            assert!(field.contains("ö"));
            //assert!(field.contains "ö" || field != "Ö" || field != "Ü"|| field != "ü"|| field != "ä"|| field != "Ä");


            // Check sample ID counts
            //assert!(field == "Sample_ID");
            if field == "Sample_ID" {
                sample_id_count = sample_id_count +  1;
                if sample_id_count > 1 {
                    println!("ERROR: Sample_ID present more than once: {}, {}, {:?}", sample_id_count, field, record);
                }
            }


                
        }


    }
    Ok(())
}


//fn check_string() -> Result<(), Box<dyn Error>> {


fn main() {

    println!("Welcome to validate Samplesheet by Colin Davenport");
    println!("Warning: does not catch Umlaeute öäü at present");

    let map: HashMap<u32, u32> = HashMap::new(); 
    

    if let Err(err) = check_csv() {
        println!("error running check_csv: {}", err);
        println!("You can probably solve this by opening in Libreoffice or Excel and deleting the unused whitespace columns, then saving as CSV again.");
        process::exit(1);
    }

    //let record : StringRecord = check_csv();
    //for field in record.iter() {
    //    assert!(field == "a" || field == "b" || field == "c");
    //}

}
