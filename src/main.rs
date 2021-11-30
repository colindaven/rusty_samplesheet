extern crate csv;
extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::error::Error;
use csv::Reader;
//use csv::StringRecord;
//use std::collections::HashMap; 
use std::process;
//use std::str;

//global vars
static mut sample_id_count : u16 = 0;
//static mut errors_found : u16 = 0;

// Check a SampleSheet CSV file for multiple common lab errors. 
// Does not check duplicate indices or similar.
fn version() ->  String {
    let version: String = str::to_string("0.24");
    //0.24 - parse Sample_ID and Sample_Name fields. Check separately for duplication.
    //0.23 - improve error msgs, allow date to contain "."
    //0.22 - add windows batch file
    //0.21 - add args parsing
    //0.20 - cross compile for Windows
    //0.10 - first version
    version
} 



fn check_csv(csv_file_string: String) -> Result<(), Box<dyn Error>> {

    // Setup file and read
    //let mut rdr = Reader::from_path("SampleSheet_2020_032049.csv")?;
    let mut rdr = Reader::from_path(csv_file_string)?;
    let mut sample_id_vec = Vec::<String>::new();
    let mut sample_name_vec = Vec::<String>::new();
    let mut index1_vec = Vec::<String>::new();
    let mut index2_vec = Vec::<String>::new();

    let mut errors_found : u16 = 0;
    // Read only tabular data after field [Data] into array
    let mut read_into_vectors : bool = false;

    // Iterate through results. Record is a StringRecord.
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);
        
        // Setup field counter
        let mut i = 0;


        //last_field = field;
        for field in record.iter() {

            let fieldStr = field.to_string();
            if field.contains("[Data]"){
                read_into_vectors = true;
            } 

            if read_into_vectors{
                // parse columns to check for duplicates into vectors
                if i == 0 {
                    sample_id_vec.push(fieldStr.clone());
                } 
                if i == 1 {
                    sample_name_vec.push(fieldStr.clone());
                } 
                if i == 5 {
                    index1_vec.push(fieldStr.clone());
                } 
                if i == 7 {
                    index2_vec.push(fieldStr.clone());
                } 
            } 
            
            // the whole line, all fields, as a string slice, then converted to String
            let record_string = record.as_slice(); 
            let record_string2 = String::from(record_string);

            // Make detailed checks on each field
            errors_found = make_field_checks(fieldStr, record_string2, errors_found);

            // TODO print columns eg with S762 and S512 + names and seqs. Check if doubled?
            //last_field = &field;
            i = i + 1;
                
        }


    }

    check_vector_contents_unique(sample_id_vec);
    //check_vector_contents_unique(sample_name_vec);
    //check_vector_contents_unique(index1_vec);
    //check_vector_contents_unique(index2_vec);

    Ok(())
}



fn check_vector_contents_unique(vector1: Vec<String>){


    for i in &vector1 {
        println!("{}", i);
    }

    //let mut sorted_vector = vector1.clone();
    let mut sorted_vector = vector1.clone();
    //sorted_vector = sorted_vector.sort();
    //println!("Len: {} ", sorted_vector.len());

    // a sorted vector has had all duplicates removed, so we can check by length. 
    // if the sorted vector is shorter, then dups have been removed.
    if sorted_vector.len() < vector1.len(){
        println!("Duplicates were found in the following list!");
        println!("{:?}",vector1);
        for i in &vector1 {
            //println!("{}", i);
        }
    } 

} 

fn make_field_checks(field: String, record_string: String, mut errors_found: u16) -> u16 {

    
    //Exit on Umlaut 
    if field.contains("ö") || field.contains("Ö") || field.contains("Ü") || field.contains("ü") || field.contains("ä") || field.contains("Ä") {
        println!("");
        println!("ERROR: Umlaut found, exiting. Field: {}", field);
        println!("Line containing error: {:?}", record_string);
        println!("See help at http://hpc-web1.mh-hannover.local/doku.php?id=samplesheet");
        println!("");
        //break;
        errors_found = errors_found + 1;
    }

    //Exit on semicolon
    if field.contains(";") {
        println!("");
        println!("ERROR: Semicolon ; illegal found, Only commas ',' should be used! Exiting. Field: {}", field);
        println!("Line containing error: {:?}", record_string);
        println!("See help at http://hpc-web1.mh-hannover.local/doku.php?id=samplesheet");
        println!("");
        errors_found = errors_found + 1;
    }
    //Report Warning on dot . , but not if Date in the same line (dots allowed for Date)
    if field.contains(".") && !record_string.contains("Date") {
        println!("");
        println!("WARNING: Dot . is illegal in non Date lines. Only [A-Za-z][1-9] and '_', should be used! Exiting. Field: {}", field);
        println!("Line containing error: {:?}", record_string);
        println!("See help at http://hpc-web1.mh-hannover.local/doku.php?id=samplesheet");
        errors_found = errors_found + 1;
    }
    //Check lines with more than 3 speech marks
    if field.contains("\"\"\"\"") {
        println!("");
        println!("ERROR: More than 3 double quotes found. Illegal. Exiting. Field: {}", field);
        println!("Line containing error: {:?}", record_string);
        println!("See help at http://hpc-web1.mh-hannover.local/doku.php?id=samplesheet");
        println!("");
        errors_found = errors_found + 1;
    }                    

    // Check sample ID header counts. This checks the header only
    //assert!(field == "Sample_ID");
    if field == "Sample_ID" {
        unsafe{ 
            sample_id_count = sample_id_count +  1;
            if sample_id_count > 1 {
                println!("");
                println!("ERROR: Sample_ID header present more than once: {}, {}, {:?}", sample_id_count, field, record_string);
                println!("Line containing error: {:?}", record_string);
                println!("See help at http://hpc-web1.mh-hannover.local/doku.php?id=samplesheet");
                println!("");
                errors_found = errors_found + 1;
            }
        }
    }
    return errors_found
} 

fn print_errors(bad_string: String, record: String){
    println!("");
    println!("Line containing error: {:?}", record);
    println!("");

} 


fn report_checks_as_info()  {
    println!("INFO: Prints error on using ; anywhere instead of comma");
    println!("INFO: Prints error on using . anywhere instead of _");
    println!("INFO: Prints error on finding German Umlaut");
    println!("INFO: Prints error if number of fields are not correct");
    println!("INFO: Prints error Sample_ID is present twice, should be Sample_ID, Sample_Name");
    println!("INFO: Prints error if more than three speech marks on line");
}



// 
//  Main method    

fn main() {

    let version = version();
    println!("INFO: Welcome to Rusty Samplesheet version {} by Colin Davenport", &version);
    println!("Usage: Call your SampleSheet SampleSheet.csv in the same directory. Double click the .bat file to start.");
    println!("Remember to use Wordpad or Notepad++ to read the output.txt file");
    println!("If this file appears empty, the tools didn't find any errors - good job!");
    //println!("Errors found: {} ", errors_found);
    //report_checks_as_info();
    
    ////////////////
    // Parse input arguments

    let mut input_file = "SampleSheet.csv".to_string();
    {  // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();
        parser.refer(&mut input_file)
            .add_option(&["-f", "--input_file"], Store,
                    "Input file CSV");

        parser.parse_args_or_exit();
    } 
    let mut input_csv: String = str::to_string(&input_file);

    ////////////////
    // Parse and check the CSV

    if let Err(err) = check_csv(input_csv) {
        println!("");
        println!("Summary: ");
        println!("There were one or more errors while running check_csv: {}", err);
        println!("You can probably solve this by opening in Libreoffice or Excel and deleting the unused whitespace columns, then saving as CSV again.");
        process::exit(1);
    }


}
