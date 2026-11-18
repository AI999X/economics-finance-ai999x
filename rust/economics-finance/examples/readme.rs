//! Economics and Finance: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p economics-finance-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&economics_finance_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "financial_ratios", "inputs": {"sales": "500000", "net_income": "40000", "total_assets": "400000", "total_equity": "160000", "current_assets": "150000", "current_liabilities": "100000", "inventory": "60000"}}));
    println!();
    show(json!({"key": "endowment_insurance", "inputs": {"mortality_rates": [0.01, 0.02, 0.03], "interest_rate": "0.04", "term": "3", "benefit": "1000"}}));
    println!();
    show(json!({"key": "bond_price_change", "inputs": {"face": "1000", "coupon_rate": "0.05", "years": "10", "ytm": "0.06", "yield_change": "0.01", "frequency": "2"}}));
    println!();
    show(json!({"key": "discounted_payback", "inputs": {"rate": "0.1", "cash_flows": [-1000, 300, 400, 500, 200]}}));
    println!();
    show(json!({"key": "credit_card_finance_charge", "inputs": {"opening_balance": "1500", "apr": "0.18", "billing_days": "30", "rate_basis": "daily", "transaction_days": [11, 21], "transaction_amounts": [-500, 200]}}));
    println!();
    show(json!({"key": "log_log_elasticity", "inputs": {"prices": [1, 2, 4, 8], "quantities": [100, 50, 25, 12.5]}}));
    println!();
    show(json!({"key": "comparative_advantage", "inputs": {"a_good_x": "6", "a_good_y": "3", "b_good_x": "1", "b_good_y": "2", "basis": "output"}}));
    println!();
    show(json!({"key": "loan_cumulative_interest", "inputs": {"principal": "200000", "annual_rate": "0.06", "years": "30", "payments_made": "12"}}));
    println!();
    show(json!({"key": "inflation_adjustment", "inputs": {"price_levels": [100, 103, 107.12], "amount": "1000"}}));
    println!();
    show(json!({"key": "break_even", "inputs": {"fixed_costs": "10000", "price": "50", "variable_cost": "30"}}));
    println!();
    show(json!({"key": "mean_returns", "inputs": {"returns": [0.1, -0.05, 0.2]}}));
    println!();
    show(json!({"key": "historical_var", "inputs": {"returns": [-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015], "confidence": "0.9"}}));
    println!();
    show(json!({"key": "capm", "inputs": {"risk_free": "0.03", "beta": "1.2", "market_return": "0.09"}}));
    println!();
    show(json!({"key": "binomial_option", "inputs": {"spot": "100", "strike": "100", "time": "1", "rate": "0.05", "volatility": "0.2", "steps": "3"}}));
    println!();
    show(json!({"key": "cagr", "inputs": {"begin_value": "1000", "end_value": "2000", "years": "5"}}));
}
