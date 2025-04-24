use json::JsonValue;
use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2], // [kJ, kcal]
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Parse kcal value from calories[1] (assuming format like "510kcal")
        let kcal_str = food.calories[1]
            .trim_end_matches("kcal")
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0);

        // Multiply each macro by number of portions
        total_cals += kcal_str * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Format numbers according to specifications
    let format_number = |value: f64| -> f64 {
        // Round to 2 decimal places
        let rounded = (value * 100.0).round() / 100.0;
        // Check if it ends in .x0
        if (rounded * 10.0).fract().abs() < 0.0001 {
            // Round to 1 decimal place
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    // Create JSON object with formatted values
    object! {
        "cals": format_number(total_cals),
        "carbs": format_number(total_carbs),
        "proteins": format_number(total_proteins),
        "fats": format_number(total_fats)
    }
}
