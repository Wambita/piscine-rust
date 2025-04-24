#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.items.iter().map(|(_, price)| *price).collect::<Vec<f32>>();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Calculate total and discount
        let total = prices.iter().sum::<f32>();
        let mut discount = 0.0;
        let mut i = 0;
        while i + 2 < prices.len() {
            // Cheapest item in each group of three is free
            discount += prices[i];
            i += 3;
        }
        let target_total = total - discount;

        // Calculate scaling factor
        let scale = if total > 0.0 { target_total / total } else { 1.0 };

        // Apply scaling to all prices
        let mut adjusted_prices = prices
            .iter()
            .map(|&price| (price * scale * 100.0).round() / 100.0)
            .collect::<Vec<f32>>();

        // Sort adjusted prices
        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Store in receipt and return
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}