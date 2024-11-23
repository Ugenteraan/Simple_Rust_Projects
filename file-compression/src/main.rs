use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

/// Compresses the input data using Run-Length Encoding (RLE).
fn compress(data: &str) -> String {
    let mut compressed = String::new();
    let mut chars = data.chars().peekable(); // Create a Peekable iterator
    while let Some(current) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&current) {
            count += 1;
            chars.next();
        }
        compressed.push(current);
        compressed.push_str(&count.to_string());
    }
    return compressed;
}


fn decompress(data: &str) -> Result<String, String> {

    let mut decompressed = String::new();
    let mut chars = data.chars().peekable();
    
    //Note that sometimes if the chars repeated for more than 10 times, it doesn't suffice to only
    //check for one char of integer after a non-integer character, we have to check for integer
    //characters until there's no non-integer character left.
    while let Some(current) = chars.next() {
        
        let mut count_string = String::new(); //to hold the integer values.
        
        //filter method is used on iterators to filter for specific conditions.
        while let Some(digit) = chars.peek().filter(|c| c.is_ascii_digit()) {
            count_string.push(*digit); //the .peek() method returns a reference. However, push
                                      //method expects a char. Hence the * to deference and get the
                                      //actual value of digit.
            chars.next(); //go to the next item.
        }
        
        //since .parse() returns Result<usize, ParseIntError>, we can handle the err with .map_err.
        let count: usize = count_string.parse().map_err(|_| format!("Invalid count in compressed data: {}", count_string))?;

        decompressed.push_str(&current.to_string().repeat(count));
        
    }

    return Ok(decompressed);

}


fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    return Ok(contents);

}

fn write_string_to_file(file_path: &Path, data: &str) -> io::Result<()> {
    
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(data.as_bytes())?;

    return Ok(());

}




fn main() -> io::Result<()>{

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        eprintln!("{} <compress|decompress> <input file> <output file>", args[0]);
        return Ok(());
    }

    let operation = &args[1];
    let input_file = Path::new(&args[2]); //Path makes sure the directory path is compatible with
                                          //cross-platform such as Unix-based and Windows.
    let output_file = Path::new(&args[3]);

    let input_data = read_file_to_string(input_file)?; //we don't have to explicitly pass
                                                       //input_file by reference since the function
                                                       //was defined to take reference as input.
                                                       //AND the variable is not being modified.
    match operation.as_str() {
        
        "compress" => {
           let compressed_data = compress(&input_data); 
           write_string_to_file(output_file, &compressed_data)?; //we can only pass output_file
                                                                 //without borrowing due to
                                                                 //output_file not getting
                                                                 //modified. While compressed data
                                                                 //is being modified. Hence the &.
        }
        "decompress" => {
            match decompress(&input_data) {
                Ok(decompressed_data) => {
                    write_string_to_file(output_file, &decompressed_data)?;
                    println!("File successfully compressed to {}", output_file.display());
                }
                Err(e) => eprintln!("Decompression failed: {}", e),

            }
        }
        _ => {
            eprintln!("Invalid operation: {}. Only 'compress' or 'decompress' is available for now.", operation);
        }


    }

    return Ok(());


}
