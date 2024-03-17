use chrono::prelude::*;
use chrono::{Days, Months};
use colored::*;

struct CorporateCoordinates {
    generation_time: DateTime<FixedOffset>,
    year: String,
    quarter: u32,
    start_of_quarter: DateTime<FixedOffset>,
    end_of_quarter: DateTime<FixedOffset>,
    full_week_of_quarter_done: u32,
    weeks_in_quarter: u32,
    days_left_in_quarter: u32,
    days_in_quarter: u32,
}

fn generate_coordinates(now: &DateTime<FixedOffset>) -> CorporateCoordinates {
    let quarter = (now.month() as f64 / 3.0).ceil() as u32;
    let start_of_year = NaiveDate::from_ymd_opt(now.year(), 1, 1)
        .unwrap()
        .and_hms_nano_opt(0, 0, 0, 0)
        .unwrap();
    let start_of_quarter = now
        .offset()
        .from_local_datetime(
            &start_of_year
                .checked_add_months(Months::new((quarter - 1) * 3))
                .unwrap(),
        )
        .unwrap();

    let end_of_quarter = now
        .offset()
        .from_local_datetime(
            &start_of_year
                .checked_add_months(Months::new((quarter) * 3))
                .unwrap()
                .checked_sub_days(Days::new(1))
                .unwrap(),
        )
        .unwrap();

    CorporateCoordinates {
        generation_time: now.clone(),
        year: format!("{}", now.year()),
        quarter: quarter as u32,
        start_of_quarter: start_of_quarter,
        end_of_quarter: end_of_quarter,
        full_week_of_quarter_done: (now.signed_duration_since(start_of_quarter).num_days() as f64
            / 7.0)
            .floor() as u32,
        weeks_in_quarter: 13,
        days_left_in_quarter: (end_of_quarter.signed_duration_since(now).num_days() + 1) as u32,
        days_in_quarter: (end_of_quarter
            .signed_duration_since(start_of_quarter)
            .num_days()) as u32,
    }
}

fn local_to_fixed(local_date_time: &DateTime<Local>) -> DateTime<FixedOffset> {
    local_date_time.with_timezone(local_date_time.offset())
}

fn print_summary(coordinates: &CorporateCoordinates) {
    println!(
        "We are {} into {}.",
        format!("{} weeks", coordinates.full_week_of_quarter_done)
            .red()
            .bold(),
        format!("Q{}, {}", coordinates.quarter, coordinates.year)
            .red()
            .bold()
    );
    println!(
        "The quarter started {} and will end {} (each quarter is {} weeks).",
        format!("{}", coordinates.start_of_quarter.format("%A, %d %B"))
            .red()
            .bold(),
        format!("{}", coordinates.end_of_quarter.format("%A, %d %B"))
            .red()
            .bold(),
        format!("{}", coordinates.weeks_in_quarter).red().bold()
    );
    println!(
        "There is {} of the quarter remaining ({} calendar days).",
        format!(
            "{:.2}%",
            (coordinates.days_left_in_quarter as f64 / coordinates.days_in_quarter as f64) * 100.0
        )
        .red()
        .bold(),
        format!("{}", (coordinates.days_left_in_quarter))
            .red()
            .bold()
    );
    println!(
        "The time and date now is {}.",
        format!("{}", coordinates.generation_time.format("%+"))
            .red()
            .bold()
    );
}

fn main() {
    let coordinates = generate_coordinates(&local_to_fixed(&Local::now()));
    print_summary(&coordinates);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> DateTime<FixedOffset> {
        local_to_fixed(&Local::now())
    }

    #[test]
    fn test_generation_time() {
        let t = now();
        assert_eq!(t, generate_coordinates(&t).generation_time)
    }

    #[test]
    fn test_year_correct() {
        let t = now();
        assert_eq!(format!("{}", t.year()), generate_coordinates(&t).year)
    }

    #[test]
    fn test_quarters_correct() {
        let q1_jan = DateTime::parse_from_rfc3339("1999-01-01T16:39:57+00:00").unwrap();
        let q1_feb = DateTime::parse_from_rfc3339("1999-02-01T16:39:57+00:00").unwrap();
        let q1_mar = DateTime::parse_from_rfc3339("1999-03-01T16:39:57+00:00").unwrap();

        let q2_apr = DateTime::parse_from_rfc3339("1999-04-01T16:39:57+00:00").unwrap();
        let q2_may = DateTime::parse_from_rfc3339("1999-05-01T16:39:57+00:00").unwrap();
        let q2_jun = DateTime::parse_from_rfc3339("1999-06-01T16:39:57+00:00").unwrap();

        let q3_jul = DateTime::parse_from_rfc3339("1999-07-01T16:39:57+00:00").unwrap();
        let q3_aug = DateTime::parse_from_rfc3339("1999-08-01T16:39:57+00:00").unwrap();
        let q3_sep = DateTime::parse_from_rfc3339("1999-09-01T16:39:57+00:00").unwrap();

        let q4_oct = DateTime::parse_from_rfc3339("1999-10-01T16:39:57+00:00").unwrap();
        let q4_nov = DateTime::parse_from_rfc3339("1999-11-01T16:39:57+00:00").unwrap();
        let q4_dec = DateTime::parse_from_rfc3339("1999-12-01T16:39:57+00:00").unwrap();

        assert_eq!(1, generate_coordinates(&q1_jan).quarter);
        assert_eq!(1, generate_coordinates(&q1_feb).quarter);
        assert_eq!(1, generate_coordinates(&q1_mar).quarter);

        assert_eq!(2, generate_coordinates(&q2_apr).quarter);
        assert_eq!(2, generate_coordinates(&q2_may).quarter);
        assert_eq!(2, generate_coordinates(&q2_jun).quarter);

        assert_eq!(3, generate_coordinates(&q3_jul).quarter);
        assert_eq!(3, generate_coordinates(&q3_aug).quarter);
        assert_eq!(3, generate_coordinates(&q3_sep).quarter);

        assert_eq!(4, generate_coordinates(&q4_oct).quarter);
        assert_eq!(4, generate_coordinates(&q4_nov).quarter);
        assert_eq!(4, generate_coordinates(&q4_dec).quarter);
    }

    #[test]
    fn test_start_end_quarter() {
        let q1 = DateTime::parse_from_rfc3339("1999-02-01T16:39:57+00:00").unwrap();
        let q2 = DateTime::parse_from_rfc3339("1999-05-01T16:39:57+00:00").unwrap();
        let q3 = DateTime::parse_from_rfc3339("1999-08-01T16:39:57+00:00").unwrap();
        let q4 = DateTime::parse_from_rfc3339("1999-11-01T16:39:57+00:00").unwrap();

        let start_of_q1 = DateTime::parse_from_rfc3339("1999-01-01T00:00:00+00:00").unwrap();
        let end_of_q1 = DateTime::parse_from_rfc3339("1999-03-31T00:00:00+00:00").unwrap();
        assert_eq!(start_of_q1, generate_coordinates(&q1).start_of_quarter);
        assert_eq!(end_of_q1, generate_coordinates(&q1).end_of_quarter);

        let start_of_q2 = DateTime::parse_from_rfc3339("1999-04-01T00:00:00+00:00").unwrap();
        let end_of_q2 = DateTime::parse_from_rfc3339("1999-06-30T00:00:00+00:00").unwrap();
        assert_eq!(start_of_q2, generate_coordinates(&q2).start_of_quarter);
        assert_eq!(end_of_q2, generate_coordinates(&q2).end_of_quarter);

        let start_of_q3 = DateTime::parse_from_rfc3339("1999-07-01T00:00:00+00:00").unwrap();
        let end_of_q3 = DateTime::parse_from_rfc3339("1999-09-30T00:00:00+00:00").unwrap();
        assert_eq!(start_of_q3, generate_coordinates(&q3).start_of_quarter);
        assert_eq!(end_of_q3, generate_coordinates(&q3).end_of_quarter);

        let start_of_q4 = DateTime::parse_from_rfc3339("1999-10-01T00:00:00+00:00").unwrap();
        let end_of_q4 = DateTime::parse_from_rfc3339("1999-12-31T00:00:00+00:00").unwrap();
        assert_eq!(start_of_q4, generate_coordinates(&q4).start_of_quarter);
        assert_eq!(end_of_q4, generate_coordinates(&q4).end_of_quarter);
    }

    #[test]
    fn test_weeks_per_quarter() {
        assert_eq!(52 / 4, generate_coordinates(&now()).weeks_in_quarter)
    }

    #[test]
    fn test_completed_weeks_quarter() {
        let start_of_year = DateTime::parse_from_rfc3339("1999-01-01T16:39:57+00:00").unwrap();
        assert_eq!(
            generate_coordinates(&start_of_year).full_week_of_quarter_done,
            0
        );

        let first_week_feb = DateTime::parse_from_rfc3339("1999-02-01T16:39:57+00:00").unwrap();
        assert_eq!(
            generate_coordinates(&first_week_feb).full_week_of_quarter_done,
            4
        );

        let first_day_q2 = DateTime::parse_from_rfc3339("1999-04-01T16:39:57+00:00").unwrap();
        assert_eq!(
            generate_coordinates(&first_day_q2).full_week_of_quarter_done,
            0
        );

        let last_day_q2 = DateTime::parse_from_rfc3339("1999-06-30T16:39:57+00:00").unwrap();
        assert_eq!(
            generate_coordinates(&last_day_q2).full_week_of_quarter_done,
            12
        );
    }

    #[test]
    fn test_days_left_in_quarter() {
        let first_day_q2 = DateTime::parse_from_rfc3339("1999-04-01T16:39:57+00:00").unwrap();
        let last_day_q2 = DateTime::parse_from_rfc3339("1999-06-30T16:39:57+00:00").unwrap();
        assert_eq!(generate_coordinates(&first_day_q2).quarter, 2);
        assert_eq!(
            generate_coordinates(&first_day_q2).days_left_in_quarter as i64,
            last_day_q2.signed_duration_since(first_day_q2).num_days()
        );
        assert_eq!(generate_coordinates(&last_day_q2).days_left_in_quarter, 1);
    }

    #[test]
    fn test_days_in_quarter() {
        let first_day_q2 = DateTime::parse_from_rfc3339("1999-04-01T16:39:57+00:00").unwrap();
        assert_eq!(generate_coordinates(&first_day_q2).days_in_quarter, 90);
    }
}
