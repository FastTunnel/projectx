use chrono::{DateTime, TimeZone, Utc};

pub trait ToTimestamp {
    fn to_timestamp(&self) -> i64;
}

impl ToTimestamp for DateTime<Utc> {
    fn to_timestamp(&self) -> i64 {
        self.timestamp()
    }
}

pub trait ToDateTime {
    fn to_date_time(&self) -> DateTime<Utc>;
}

impl ToDateTime for i64 {
    fn to_date_time(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(*self, 0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::Utc;

    #[test]
    fn test_to_timestamp() {
        let dt = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        println!("{}", dt.to_timestamp());
        println!("{}", dt.timestamp_millis());
    }

    #[test]
    fn test_to_date_time() {
        let dt = -400;
        let dt = dt.to_date_time();
        println!("{}", dt);
    }
}
