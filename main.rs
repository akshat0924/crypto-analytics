fn main() {
    println!("🚀 Starting RSI Calculation...");

    let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42];
    let rsi = calculate_rsi(&prices, 14);

    println!("RSI value: {}", rsi);
}

fn calculate_rsi(prices: &Vec<f64>, period: usize) -> f64 {
    if prices.len() <= period {
        return 0.0;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=period {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            gains += diff;
        } else {
            losses -= diff;
        }
    }

    let avg_gain = gains / period as f64;
    let avg_loss = losses / period as f64;

    if avg_loss == 0.0 {
        return 100.0;
    }

    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
