use std::collections::HashMap;
use serde::Deserialize;

const HOLIDAY_FILES: &[(i32, &str)] = &[
    (2026, include_str!("../data/holidays/2026.json")),
    (2027, include_str!("../data/holidays/2027.json")),
];

#[derive(Debug, Deserialize)]
struct YearHolidays {
    year: i32,
    holidays: Vec<HolidayEntry>,
    workdays: Vec<HolidayEntry>,
}

#[derive(Debug, Deserialize)]
struct HolidayEntry {
    date: String,
    name: String,
}
impl Default for HolidayCalendar {
    fn default() -> Self {
        Self::load_all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind {
    Holiday,
    Workday,
}

pub struct HolidayCalendar {
    /// key: "2026-01-01"
    map: HashMap<String, DayKind>,
}

impl HolidayCalendar {
    /// 加载所有已内置年份的数据，合并到一个 map。
    pub fn load_all() -> Self {
        let mut map = HashMap::new();
        for (_, text) in HOLIDAY_FILES {
            if let Ok(data) = serde_json::from_str::<YearHolidays>(text) {
                for h in data.holidays {
                    map.insert(h.date, DayKind::Holiday);
                }
                for w in data.workdays {
                    map.insert(w.date, DayKind::Workday);
                }
            }
        }
        Self { map }
    }

    /// 查询某天是节假日、调休工作日，还是普通日。
    pub fn kind(&self, date: jiff::civil::Date) -> Option<DayKind> {
        let key = format!("{:04}-{:02}-{:02}", date.year(), date.month(), date.day());
        self.map.get(&key).copied()
    }
}
