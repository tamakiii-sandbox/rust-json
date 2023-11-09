use std::io::{self, Read};

#[allow(non_snake_case)]
mod data {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Owner {
        pub DisplayName: String,
        pub ID: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Version {
        pub ETag: String,
        pub Size: i32,
        pub StorageClass: String,
        pub Key: String,
        pub VersionId: String,
        pub IsLatest: bool,
        pub LastModified: String,
        pub Owner: Owner,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteMarker {
        pub Owner: Owner,
        pub Key: String,
        pub VersionId: String,
        pub IsLatest: bool,
        pub LastModified: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputData {
        pub Versions: Vec<Version>,
        pub DeleteMarkers: Vec<DeleteMarker>,
        pub RequestCharged: Option<String>,
    }
}

fn main() {
    // Read from stdin into a string
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    // Parse the JSON string
    let data: data::InputData = serde_json::from_str(&input).expect("Failed to parse JSON");

    // Now you can work with the `data`
    println!("{:#?}", data);
}
