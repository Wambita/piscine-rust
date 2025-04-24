use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Parse kcal from calories tuple (second element)
        let kcal_str = food.calories.1.trim_end_matches("kcal");
        let kcal: f64 = kcal_str.parse().unwrap_or(0.0);
        
        // Accumulate totals, scaled by number of portions
        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Round to two decimal places, or one if second decimal is zero
    let round = |value: f64| {
        let rounded = (value * 100.0).round() / 100.0;
        let formatted = format!("{:.2}", rounded);
        if formatted.ends_with("0") {
            format!("{:.1}", rounded)
        } else {
            formatted
        }
        .parse::<f64>()
        .unwrap()
    };

    // Apply rounding to totals
    let total_cals = round(total_cals);
    let total_carbs = round(total_carbs);
    let total_proteins = round(total_proteins);
    let total_fats = round(total_fats);

    // Create JSON object
    let mut result = JsonValue::new_object();
    result["cals"] = JsonValue::Number(json::number::Number::from(total_cals));
    result["carbs"] = JsonValue::Number(json::number::Number::from(total_carbs));
    result["proteins"] = JsonValue::Number(json::number::Number::from(total_proteins));
    result["fats"] = JsonValue::Number(json::number::Number::from(total_fats));

    result
}