use std::fs;

pub fn get_keys(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect("Not found apikeys file!");
    file.lines().map(|s| s.to_string()).collect()
}
pub fn get_file_data(path: &str) -> String {
    let file = fs::read_to_string(path).expect("Not found file for update!");
    file
}
