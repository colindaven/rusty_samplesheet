extern crate csv;

use std::error::Error;
use csv::Reader;
//use csv::StringRecord;
use std::collections::HashMap; 
use std::process;
//use std::str;


// Check a SampleSheet CSV file for multiple common lab errors. 
// Does not check duplicate indices or similar.

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
            if field.contains("ö") || field.contains("Ö") || field.contains("Ü") || field.contains("ü") || field.contains("ä") || field.contains("Ä") {
                println!("ERROR: Umlaut found, exiting. Field: {}", field);
                break;
            }

        //Exit on semicolon
        if field.contains(";") {
            println!("ERROR: Semicolon ; illegal found, exiting. Field: {}", field);
            break;
        }

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


// 
//  Main method    

fn main() {

    let version = "0.1";
    println!("INFO: Welcome to Validate Samplesheet v{} by Colin Davenport", version);
    println!("INFO: Prints error on using ; anywhere instead of comma");
    println!("INFO: Prints error on finding German Umlaut");
    println!("INFO: Prints error if number of fields are not correct");
    println!("INFO: Prints error Sample_ID is present twice, should be Sample_ID, Sample_Name");
    //println!("Warning: does not catch xxxxx at present");

    let map: HashMap<u32, u32> = HashMap::new(); 
    

    if let Err(err) = check_csv() {
        println!("ERROR running check_csv: {}", err);
        println!("You can probably solve this by opening in Libreoffice or Excel and deleting the unused whitespace columns, then saving as CSV again.");
        process::exit(1);
    }


}
