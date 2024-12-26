pub fn calculate_income_tax(income: f64) -> f64 {
    let tax_rate = get_tax_rate(income);
    income * tax_rate
}
pub fn get_tax_rate(income: f64) -> f64 {
    if income <= 10000.0 {
        0.0
    } else if income <= 50000.0 {
        0.1
    } else {
        0.2
    }
}
pub fn calculate_compound_interest(principal: f64, rate: f64, time: f64, n: f64) -> f64 {
    principal * (1.0 + rate / n).powf(n * time)
}
pub fn calculate_simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    principal * rate * time
}