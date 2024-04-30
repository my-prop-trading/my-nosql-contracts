use serde::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(i32)]
pub enum CfdAccountTypeMyNoSql {
    Demo = 0,
    Live = 1,
}

impl CfdAccountTypeMyNoSql {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Demo => "Demo",
            Self::Live => "live",
        }
    }
}

impl ToString for CfdAccountTypeMyNoSql {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<i32> for CfdAccountTypeMyNoSql {
    fn from(value: i32) -> Self {
        match value {
            0 => CfdAccountTypeMyNoSql::Demo,
            1 => CfdAccountTypeMyNoSql::Live,
            _ => panic!("Invalid value '{}' for CfdAccountTypeMyNoSql", value,),
        }
    }
}
