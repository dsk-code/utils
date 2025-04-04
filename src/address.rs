use crate::error::Error;

use regex::Regex;

/// #### 住所から市を抽出
/// ##### Example
/// ---
/// ```rust
/// use utils::address::extract_city;
///
/// fn main() {
///     let city = extract_city("北海道札幌市中央区大通西4丁目");
///
///     match city {
///        Ok(city) => println!("city = {}", city),
///        Err(error) => eprintln!("{}", error),
///     };
///
/// }
/// ```
pub fn extract_city(address: &str) -> Result<String, Error> {
    let re = Regex::new(r"^[^都道府県]*(都|道|府|県)(?<city>[^0-9]+市)")?;

    let city = match re.captures(address) {
        Some(cap) => cap["city"].to_string(),
        None => return Err(Error::NotFound("city".to_string())),
    };

    Ok(city)
}
