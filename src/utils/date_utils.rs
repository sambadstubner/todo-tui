use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};

pub fn parse_date_input(input: &str) -> Option<DateTime<Local>> {
    let input = input.trim().to_lowercase();
    
    // Handle relative dates
    match input.as_str() {
        "today" => return Some(Local::now().date_naive().and_hms_opt(23, 59, 59)?.and_local_timezone(Local).single()?),
        "tomorrow" => return Some((Local::now().date_naive() + chrono::Duration::days(1)).and_hms_opt(23, 59, 59)?.and_local_timezone(Local).single()?),
        "next week" => return Some((Local::now().date_naive() + chrono::Duration::weeks(1)).and_hms_opt(23, 59, 59)?.and_local_timezone(Local).single()?),
        "next month" => {
            let next_month = Local::now().date_naive() + chrono::Duration::days(30);
            return Some(next_month.and_hms_opt(23, 59, 59)?.and_local_timezone(Local).single()?);
        }
        _ => {}
    }
    
    // Try parsing various date formats
    let formats = [
        "%Y-%m-%d",
        "%m/%d/%Y",
        "%d/%m/%Y",
        "%Y-%m-%d %H:%M",
        "%m/%d/%Y %H:%M",
        "%d/%m/%Y %H:%M",
    ];
    
    for format in &formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(&input, format) {
            return naive_dt.and_local_timezone(Local).single();
        }
    }
    
    // Try parsing date only
    for format in &["%Y-%m-%d", "%m/%d/%Y", "%d/%m/%Y"] {
        if let Ok(naive_date) = NaiveDate::parse_from_str(&input, format) {
            return naive_date.and_hms_opt(23, 59, 59)?.and_local_timezone(Local).single();
        }
    }
    
    None
}

