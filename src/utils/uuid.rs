use uuid::Uuid;

pub fn create() -> Uuid {
    Uuid::now_v7()
}

pub fn create_string() -> String {
    create().to_string()
}

pub fn verify(value: &str) -> bool {
    Uuid::parse_str(value).is_ok()
}
