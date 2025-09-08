fn main() {
    // Days in each month (non-leap year)
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    // Start from 1 Jan 1901
    // We know 1 Jan 1900 was a Monday (day 1)
    // So we need to calculate what day 1 Jan 1901 was
    
    // Days from 1 Jan 1900 to 31 Dec 1900 = 365 days (1900 is not a leap year)
    // 365 % 7 = 1, so 1 Jan 1901 was a Tuesday (day 2)
    let mut current_day = 2; // Tuesday (0=Sunday, 1=Monday, ..., 6=Saturday)
    let mut sunday_count = 0;
    
    for year in 1901..=2000 {
        let is_leap_year = is_leap(year);
        
        for month in 0..12 {
            // Check if the first day of this month is a Sunday
            if current_day == 0 {
                sunday_count += 1;
            }
            
            // Calculate days in this month
            let mut days_this_month = days_in_month[month];
            if month == 1 && is_leap_year { // February in leap year
                days_this_month = 29;
            }
            
            // Move to the first day of next month
            current_day = (current_day + days_this_month) % 7;
        }
    }
    
    println!("Number of Sundays that fell on the first of the month: {}", sunday_count);
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}
