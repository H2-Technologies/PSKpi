pub fn maidenhead_to_lat_long(mut maidenhead: String) -> Result<(f64, f64), &'static str> {
    // Ensure the Maidenhead locator is truncated to a maximum length of 6 characters
    if maidenhead.len() > 6 {
        maidenhead.truncate(6);
    }
    // Append 'll' if the Maidenhead locator is shorter than 4 characters
    if maidenhead.len() < 4 {
        maidenhead += "ll";
    }
    // Check if the provided Maidenhead locator is valid
    if !is_valid_maidenhead(&maidenhead) {
        return Err("Input is not a valid Maidenhead locator");
    }

    // Define conversion factor for patch minutes to degrees
    let patch_minutes_to_degrees = 5.0 / 60.0;

    // Extract major and minor degrees for longitude
    let long_major_degrees = (maidenhead.chars().nth(0).unwrap() as u8 - b'A') as f64 * 20.0;
    let long_minor_degrees = (maidenhead.chars().nth(2).unwrap() as u8 - b'0') as f64 * 2.0;

    // Extract major and minor degrees for latitude
    let lat_major_degrees = (maidenhead.chars().nth(1).unwrap() as u8 - b'A') as f64 * 10.0;
    let lat_minor_degrees = (maidenhead.chars().nth(3).unwrap() as u8 - b'0') as f64 * 1.0;

    // Extract patch minutes for longitude and latitude
    let long_patch_minutes = (maidenhead.chars().nth(4).unwrap() as u8 - b'A') as f64;
    let lat_patch_minutes = (maidenhead.chars().nth(5).unwrap() as u8 - b'A') as f64;

    // Convert patch minutes to degrees for longitude and latitude
    let long_patch_degrees = long_patch_minutes * patch_minutes_to_degrees;
    let lat_patch_degrees = lat_patch_minutes * patch_minutes_to_degrees;

    // Calculate latitude and longitude
    let long = long_major_degrees + long_minor_degrees + long_patch_degrees - 180.0;
    let lat = lat_major_degrees + lat_minor_degrees + lat_patch_degrees - 90.0;

    //take the float number and truncate them to 4 decimal places
    let lat = (lat * 10000.0).trunc() / 10000.0;
    let long = (long * 10000.0).trunc() / 10000.0;

    Ok((lat, long))
}

fn is_valid_maidenhead(maidenhead: &str) -> bool {
    if maidenhead.len() < 4 || maidenhead.len() > 6 {
        return false;
    }

    // Validate the format of the provided Maidenhead locator
    for (i, c) in maidenhead.chars().enumerate() {
        match i {
            0 | 1  => {
                if !('A'..='R').contains(&c) {
                    return false;
                }
            }
            2 | 3 => {
                if !('0'..='9').contains(&c) {
                    return false;
                }
            }
            4 | 5 => {
                if !('A'..='X').contains(&c) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

pub fn callsign_country(callsign: String) -> Result<String, &'static str> {
    let mut country = String::new();
    let mut callsign = callsign.to_uppercase();
    let mut callsign = callsign.chars();
    let first_two = callsign.take(2);
    let country_code = match first_two.collect::<String>().as_str() {
        "A2" => "Botswana",
        "A3" => "Tonga",
        "A4" => "Oman",
        "A5" => "Bhutan",
        "A6" => "United Arab Emirates",
        "A7" => "Qatar",
        "A9" => "Bahrain",
        //AP-AS inclusive is pakistan
        "AP" | "AS" => "Pakistan",
        "B" => "China",
    };
    Ok(country)
}
