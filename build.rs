use std::fs;

fn main() {
    create_color_hashmaps();
}

fn create_color_hashmaps() {
    let mut color_file_output = r#"/// Automatically generated file. Cannot directly alter.
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::color::{generate_color_data_from_string, ColorTable};

pub static COLOR_TABLES: LazyLock<HashMap<&str, ColorTable>> = LazyLock::new(|| {
    let mut map = HashMap::new();"#.to_string();

    let file_list = fs::read_dir("./src/color_files/").unwrap();
    for color_file in file_list {
        let file = &color_file.unwrap();
        if file.path().to_str().unwrap().ends_with(".col") {
            let file_data = fs::read_to_string(file.path()).unwrap();
            let mut filename = file.file_name().to_str().unwrap().to_string();
            filename = filename.trim_end_matches(".col").into();

            color_file_output += r#"
            
    map.insert(""#;
            color_file_output += &filename;
            color_file_output += r#"", generate_color_data_from_string(r#"
"#;
            color_file_output += &file_data;
            color_file_output += r##""#));"##;
        }
    }

    color_file_output += r#"

    map
});
"#;

    fs::write("./src/color_files.rs", color_file_output).unwrap();
}