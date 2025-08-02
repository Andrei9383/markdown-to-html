use std::fs;
use std::env;


fn main() {

    println!("\n----------Starting Markdown to HTML----------\n");

    let files = md_html::get_md_files_dir("./");
    
    files.iter().for_each(|f| { 
        // Get the content of the file
        let _content = md_html::get_content_from_md_file(f.to_string()); 

        // Process the file
        // TODO
    });
}
