use chrono::{Datelike, Local};

/// #### 4月基準の年度を取得
/// ---
/// ##### 4月1日より前の場合は前年を取得
/// ##### Example
/// ---
/// ```rust
/// use utils::year::get_school_year;
/// 
/// fn main() {
///     let year = get_school_year();
/// 
///     println!("{}年度", year);
/// }
/// ```
pub fn get_school_year() -> i32 {
    let today = Local::now().date_naive(); // 現在の日付（ローカル時間）
    let current_year = today.year();
    let current_month = today.month();

    // 4月1日より前の場合は前年を取得
    let year_to_use = if current_month < 4 {
        current_year - 1
    } else {
        current_year
    };

    year_to_use
}
