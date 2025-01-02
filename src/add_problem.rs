use std::fs;
use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::path::Path;

pub fn add_problem(id: u32, name: &str, tags: &[&str], difficulty: &str) {
    let difficulty_dir = match difficulty {
        "easy" => "src/problems/easy",
        "medium" => "src/problems/medium",
        "hard" => "src/problems/hard",
        _ => panic!("Invalid difficulty level. Use 'easy', 'medium', or 'hard'."),
    };

    // Create filename like p0001.rs
    let filename = format!("{}/p{:04}.rs", difficulty_dir, id);
    let file_path = Path::new(&filename);

    if file_path.exists() {
        eprintln!("Problem file already exists: {}", filename);
        return;
    }

    // Create the problem file
    let mut file = fs::File::create(file_path).expect("Failed to create file");
    writeln!(
        file,
        "// Problem: {}\n// Tags: {}\n\npub fn solve() {{\n    // Your solution here\n}}\n",
        name,
        tags.join(", ")
    )
    .expect("Failed to write to file");

    // Update the module's mod.rs
    let mod_file = format!("{}/mod.rs", difficulty_dir);
    let mut mod_content = fs::read_to_string(&mod_file).unwrap_or_else(|_| String::new());
    writeln!(mod_content, "pub mod p{:04};", id).expect("Failed to update mod.rs");
    fs::write(&mod_file, mod_content).expect("Failed to write to mod.rs");

    println!("Added problem {}: {} to {}", id, name, difficulty);
}
