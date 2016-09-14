#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Version {
    Version: String,
    Repo: usize,
    Commit: String,
}
