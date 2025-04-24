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

        let mut adjusted_prices = Vec::new();
        let mut i = 0;

        while i < prices.len() {
            let chunk_size = std::cmp::min(3, prices.len() - i);
            let chunk = &prices[i..i + chunk_size];
            
            if chunk_size == 3 {
                // Calculate total and discount for three items
                let total = chunk.iter().sum::<f32>();
                let discount = chunk[0]; // Cheapest item is free
                let target_total = total - discount;
                
                // Calculate scaling factor to distribute discount
                let scale = if total > 0.0 { target_total / total } else { 1.0 };
                
                // Apply scaled prices, round to 2 decimal places
                for &price in chunk {
                    let adjusted = (price * scale * 100.0).round() / 100.0;
                    adjusted_prices.push(adjusted);
                }
            } else {
                // For remaining items (less than 3), keep original prices
                adjusted_prices.extend_from_slice(chunk);
            }
            
            i += chunk_size;
        }

        // Sort adjusted prices to match expected output
        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        // Store in receipt and return
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}