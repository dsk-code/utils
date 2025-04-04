use utils::address::extract_city;

fn main() {
    let city = extract_city("北海道札幌市中央区大通西4丁目");

    match city {
        Ok(city) => println!("city = {}", city),
        Err(error) => eprintln!("{}", error),
    };

}
