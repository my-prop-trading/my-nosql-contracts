use serde::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum CfdCrossMarginAccountTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

impl CfdCrossMarginAccountTypeMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Demo => "Demo",
            Self::Live => "live",
        }
    }
}

impl ToString for CfdCrossMarginAccountTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for CfdCrossMarginAccountTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => CfdCrossMarginAccountTypeMyNoSql::Demo,
            1 => CfdCrossMarginAccountTypeMyNoSql::Live,
            _ => panic!("Invalid value '{}' for CfdCrossMarginAccountTypeMyNoSql", value,),
        }
    }
}
