use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    // Prompt for the question name
    print!("Enter the question name (ex: in-memory-file-system): ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

    let mut question_name = String::new();
    io::stdin().read_line(&mut question_name).unwrap();
    let question_name = question_name.trim(); // Trim any whitespace or newlines

    // Prompt for the programming language
    print!("Enter the programming language: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

    let mut programming_language = String::new();
    io::stdin().read_line(&mut programming_language).unwrap();
    let programming_language = programming_language.trim(); // Trim any whitespace or newlines

    // Create the directory structure question_name/solutions/y/01
    let base_path = Path::new(question_name).join("solutions").join(programming_language).join("01");
    
    // Using `fs::create_dir_all` to create the directory structure
    match fs::create_dir_all(&base_path) {
        Ok(_) => println!("Successfully created directory structure: {}/solutions/{}/01", question_name, programming_language),
        Err(e) => {
            eprintln!("Error creating directories: {}", e);
            return;
        }
    }

    // Create README.md in the question folder (question_name)
    let readme_path = Path::new(question_name).join("README.md");
    match File::create(&readme_path) {
        Ok(_) => println!("Created README.md in {}/", question_name),
        Err(e) => eprintln!("Error creating README.md: {}", e),
    }

    // Create .keep files in solutions, programming_language, and 01
    let keep_files = vec![
        Path::new(question_name).join("solutions").join(".keep"),               // .keep in solutions
        Path::new(question_name).join("solutions").join(programming_language).join(".keep"),  // .keep in programming_language
        base_path.join(".keep"),                                               // .keep in 01
    ];

    for keep_file in keep_files {
        match File::create(&keep_file) {
            Ok(_) => println!("Created .keep file in {:?}", keep_file),
            Err(e) => eprintln!("Error creating .keep file in {:?}: {}", keep_file, e),
        }
    }

    // Run git checkout -b x (where x is the question name)
    let git_branch_name = question_name;

    match Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(git_branch_name)
        .status()
    {
        Ok(status) if status.success() => println!("Successfully created and switched to branch '{}'", question_name),
        Ok(_) => eprintln!("Failed to create or switch branches."),
        Err(e) => eprintln!("Error running git: {}", e),
    }
}
