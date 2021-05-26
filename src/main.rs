extern crate csv;
extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::error::Error;
use csv::Reader;
//use csv::StringRecord;
//use std::collections::HashMap; 
use std::process;
//use std::str;


// Check a SampleSheet CSV file for multiple common lab errors. 
// Does not check duplicate indices or similar.

fn check_csv(csv_file_string: String) -> Result<(), Box<dyn Error>> {
    let mut sample_id_count : u16 = 0;

    // Setup file and read
    //let mut rdr = Reader::from_path("SampleSheet_2020_032049.csv")?;
    let mut rdr = Reader::from_path(csv_file_string)?;
    // Iterate through results.
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);


        for field in record.iter() {

            //Exit on Umlaut 
            if field.contains("ö") || field.contains("Ö") || field.contains("Ü") || field.contains("ü") || field.contains("ä") || field.contains("Ä") {
                println!("");
                println!("ERROR: Umlaut found, exiting. Field: {}", field);
                println!("Line containing error: {:?}", record);
                //let error_string = "ERROR: Umlaut found, exiting. Field: {}", field;
                //let error_record = record.clone();
                //print_errors(error_string, error_record);
                break;
            }

        //Exit on semicolon
        if field.contains(";") {
            println!("");
            println!("ERROR: Semicolon ; illegal found, Only commas , should be used! Exiting. Field: {}", field);
            println!("Line containing error: {:?}", record);
            break;
        }

            // Check sample ID counts
            //assert!(field == "Sample_ID");
            if field == "Sample_ID" {
                sample_id_count = sample_id_count +  1;
                if sample_id_count > 1 {
                    println!("");
                    println!("ERROR: Sample_ID present more than once: {}, {}, {:?}", sample_id_count, field, record);
                    println!("Line containing error: {:?}", record);
                }
            }


                
        }


    }
    Ok(())
}

fn print_errors(bad_string: String, record: String){
    println!("");
    println!("Line containing error: {:?}", record);
    println!("");

} 


fn report_checks_as_info()  {
    println!("INFO: Prints error on using ; anywhere instead of comma");
    println!("INFO: Prints error on finding German Umlaut");
    println!("INFO: Prints error if number of fields are not correct");
    println!("INFO: Prints error Sample_ID is present twice, should be Sample_ID, Sample_Name");
    println!("INFO: Prints error if more than three speech marks on line");
    println!("INFO: Prints error if Sample_ID is present twice, should be Sample_ID, Sample_Name");
}



// 
//  Main method    

fn main() {

    let version = "0.21";
    //0.21 - add args parsing
    //0.20 - cross compile for Windows
    //0.10 - first version

    println!("INFO: Welcome to Rusty Samplesheet version {} by Colin Davenport", version);
    //report_checks_as_info();

    //let _map: HashMap<u32, u32> = HashMap::new(); 
    
    ////////////////
    // Parse input arguments

    let mut input_file = "SampleSheet.csv".to_string();
    let mut name = "horse".to_string();
    {  // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();
        parser.refer(&mut name)
            .add_option(&["--name"], Store,
            "Name for the greeting");
        parser.refer(&mut input_file);    
        //parser.refer(&mut input_file)
        //    .add_option(&["-f", "--input_file"], Store,
        //                "Input file CSV");
        parser.parse_args_or_exit();
    } 
    let mut input_csv: String = str::to_string(&name);

    ////////////////
    // Parse and check the CSV

    if let Err(err) = check_csv(input_csv) {
        println!("ERROR running check_csv: {}", err);
        println!("You can probably solve this by opening in Libreoffice or Excel and deleting the unused whitespace columns, then saving as CSV again.");
        process::exit(1);
    }


}
