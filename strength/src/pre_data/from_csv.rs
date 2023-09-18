pub trait FromCSV {
    fn from_csv(file_path: String) -> Result<Self, String>
    where Self: Sized;
}