use std::fs;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_files() {
        let path = "./tests/get_files";
        
        let files = "./tests/get_files/markdown.md";

        assert_eq!(vec![files], get_md_files_dir(path));
    }

    #[test]
    fn get_content() {
        let path = "./tests/get_files/markdown.md";

        assert_eq!("Hello World\n", get_content_from_md_file(path.to_string()));
    }
}

// Get the visible (unhidden) markdown files from the current directory
pub fn get_md_files_dir(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {

        // Get the file name of the current path
        let file_name = path.unwrap().path();
        
        // Get the extension of the current file with the file_name
        let file_extension = file_name.extension();

        match file_extension {
            Some(extension) => {
                if extension == "md" {
                    // this is safe
                    files.push(file_name.to_str().unwrap().to_string());
                }
            },
            None => {},
        }
    }
    files
}


pub fn get_content_from_md_file(file_path: String) -> String {
    let content = fs::read_to_string(file_path).expect("unable to get the content"); 

    content
}




