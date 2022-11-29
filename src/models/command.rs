#[derive(Clone)]
pub enum Trigger {
    NONE,
    OCR,
    OCRINVERSE,
    ADDBOOK
}

impl Trigger {
    pub fn from_str(s: &str) -> Trigger {
        match s {
            "ocr" | "OCR" => Trigger::OCR,
            "ocrinverse" | "OCRINVERSE" => Trigger::OCRINVERSE,
            "addbook" | "ADDBOOK" => Trigger::ADDBOOK,
            _ => Trigger::NONE,
        }
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Trigger::NONE
    }
}