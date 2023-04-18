use std::{
    fs::{self, File, ReadDir},
    io::{stdin, Error, ErrorKind, stdout, Write},
    path::Path,
};

mod ui;
use ui::edit;

trait CheckBlank {
    fn is_blank(&self) -> bool;
}

impl CheckBlank for String {
    fn is_blank(&self) -> bool {
        let is_white: Vec<&str> = self.matches(char::is_whitespace).collect();
        if self.is_empty() || !is_white.is_empty() {
            true
        } else {
            false
        }
    }
}

fn main() -> Result<(), Error> {
    let path = Path::new("./texts");
    if !path.is_dir() {
        if let Err(x) = fs::create_dir("./texts") {
            return Err(x);
        }
    }
    let path_text = path.to_str().unwrap_or("").to_string();
    if path_text.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            String::from("File Path could not be evaluated"),
        ));
    }
    let args: Vec<String> = std::env::args().collect();
    //check if a command was given
    if args.len() <= 1 {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Please provide from the following options(add,edit)",
        ));
    }

    let command = &args[1];
    match command {
        x if x == "add" => on_add(&args, path, &path_text),
        x if x == "edit" => on_edit(&args, path, &path_text),
        x if x == "list" => on_list(path),
        x if x == "delete" => on_delete(&args, path, &path_text),

        _ => {
            return Err(Error::new(
                ErrorKind::Unsupported,
                "Please provide from the following options(add,edit)",
            ))
        }
    }
}

fn on_delete(args: &Vec<String>, path: &Path, path_text: &String) -> Result<(), Error> {
    let filename: String;
    if args.len() < 3 {
        print!("Give the name of the file you wish to find");
        stdout().flush();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        filename = buf.to_string();
    } else {
        filename = args[2].to_owned()
    }
    if let Ok(dir) = path.read_dir() {
        for file_result in dir {
            if let Ok(file) = file_result {
                if file
                    .file_name()
                    .to_str()
                    .unwrap_or_default()
                    .starts_with(&filename)
                {
                    if let Ok(_) =fs::remove_file(format!("{}/{}", path_text, file.file_name().to_str().unwrap_or_default())){
                        return Ok(())
                    }else {
                        return Err(Error::new(ErrorKind::InvalidData,"Could not complete deletion"))
                    }
                }
            }
        }
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidData,
            "Could not complete delete",
        ))
    }
}

fn on_list(path: &Path) -> Result<(), Error> {
    if let Ok(dir) = path.read_dir() {
        for file_result in dir {
            if let Ok(file) = file_result {
                println!("{}", file.file_name().clone().to_str().unwrap_or_default())
            }
        }
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Path name is not a dir"))
    }
}

fn on_add(args: &Vec<String>, path: &Path, path_text: &String) -> Result<(), Error> {
    let mut filename: String;
    if args.len() < 3 {
        filename = format!("{}{}", uuid::Uuid::new_v4().to_string(), ".md");
        let mut files = fs::read_dir(path);
        while check_if_exists(&mut files, &filename) {
            filename = format!("{}{}", uuid::Uuid::new_v4().to_string(), ".md");
        }
    } else {
        filename = args[2].clone();
    }
    filename = force_markdown(&filename);
    if let Ok(_) = File::create(format!("{}/{}", path_text, &filename)) {
        edit(
            filename.clone(),
            format!("{}/{}", path_text, &filename),
            String::new(),
        );
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Could not create file"))
    }
}

fn on_edit(args: &Vec<String>, path: &Path, path_text: &String) -> Result<(), Error> {
    let mut filename: String;
    if args.len() < 3 {
        print!("Give the name of the file you wish to find");
        stdout().flush();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        filename = buf.to_string();
    } else {
        filename = args[2].to_owned()
    }
    filename = force_markdown(&filename);
    let mut files = fs::read_dir(path);
    if !check_if_exists(&mut files, &filename) {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid note name"));
    }
    let file = fs::read_to_string(format!("{}/{}", path_text, &filename));
    if let Ok(content) = file {
        edit(
            filename.clone(),
            format!("{}/{}", path_text, &filename),
            content,
        );
        Ok(())
    } else {
        return Err(Error::new(ErrorKind::NotFound, "program not fount :("));
    }
}

fn force_markdown(filename: &String) -> String {
    let mut fname_clone = filename.clone();
    if fname_clone.contains(".") && !fname_clone.contains(".md") {
        let extension = fname_clone.find(".").unwrap();
        fname_clone.replace_range(extension.., ".md")
    } else if !fname_clone.contains(".") {
        fname_clone.push_str(".md")
    }
    String::from(fname_clone)
}

fn check_if_exists(files: &mut Result<ReadDir, Error>, filename: &str) -> bool {
    if let Ok(ref mut dir) = files {
        dir.any(|x| -> bool {
            if let Ok(file) = x {
                if let Some(name) = file.file_name().to_str() {
                    name.to_string() == filename
                } else {
                    false
                }
            } else {
                false
            }
        })
    } else {
        false
    }
}
