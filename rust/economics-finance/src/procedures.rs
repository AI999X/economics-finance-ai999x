//! economics_finance: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/economics_finance/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "annuity_rate",
    "arc_price_elasticity",
    "beta_regression",
    "binomial_option",
    "black_scholes",
    "bond_convexity",
    "bond_duration",
    "bond_price",
    "bond_price_change",
    "bond_ytm",
    "break_even",
    "bs_greeks",
    "cagr",
    "capm",
    "cobb_douglas_demand",
    "comparative_advantage",
    "competitive_firm_cost_curves",
    "compound_interest",
    "continuous_compounding",
    "correlation",
    "cournot",
    "credit_card_finance_charge",
    "cross_price_elasticity",
    "crossover_rate",
    "direct_cost_variances",
    "discounted_payback",
    "doubling_time",
    "durbin_watson",
    "effective_annual_rate",
    "endowment_insurance",
    "equivalent_annual_annuity",
    "financial_ratios",
    "fisher_real_rate",
    "forward_price",
    "forward_price_dividends",
    "forward_rate",
    "future_value_annuity",
    "gdp_deflator",
    "gini_coefficient",
    "global_min_variance",
    "gordon_growth",
    "gordon_implied_return",
    "growing_annuity",
    "growing_perpetuity",
    "herfindahl_index",
    "historical_var",
    "implied_volatility",
    "income_elasticity",
    "income_tax",
    "inflation_adjustment",
    "information_ratio",
    "inventory_costing",
    "irr",
    "life_annuity_due",
    "life_expectancy",
    "loan_balance",
    "loan_cumulative_interest",
    "loan_payment",
    "loan_payment_split",
    "log_log_elasticity",
    "market_equilibrium",
    "max_drawdown",
    "mean_returns",
    "min_variance_two_asset",
    "mirr",
    "money_multiplier",
    "monopoly",
    "multiple_regression",
    "net_premium_whole_life",
    "nominal_rate",
    "npv",
    "number_of_periods",
    "ols_simple",
    "overhead_cost_variances",
    "parametric_var",
    "payback_period",
    "per_unit_tax",
    "perpetuity",
    "point_elasticity",
    "portfolio_return_risk",
    "portfolio_var",
    "present_value",
    "present_value_annuity",
    "price_indices",
    "profitability_index",
    "put_call_parity",
    "scenario_return",
    "sharpe_ratio",
    "simple_interest",
    "sinking_fund",
    "solow_steady_state",
    "sortino_ratio",
    "spending_multiplier",
    "stackelberg",
    "survival_probability",
    "tangency_portfolio",
    "temporary_annuity_due",
    "term_insurance",
    "treynor_jensen",
    "two_asset_portfolio",
    "two_stage_ddm",
    "unemployment_rate",
    "wacc",
    "whole_life_insurance",
    "zero_coupon_bond",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "annuity_rate" => {
            let r = f_annuity_rate(match v.get("present_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_34(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arc_price_elasticity" => {
            let r = f_arc_price_elasticity(match v.get("price_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_66(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_regression" => {
            let r = f_beta_regression(match v.get("asset_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("market_returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_106(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "binomial_option" => {
            let r = f_binomial_option(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("volatility") { Some(x) => py::from_val(x), None => { return None } }, match v.get("steps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("option_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("call")) })().ok()? } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_162(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "black_scholes" => {
            let r = f_black_scholes(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("volatility") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_193(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_convexity" => {
            let r = f_bond_convexity(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coupon_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("frequency") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_duration" => {
            let r = f_bond_duration(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coupon_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("frequency") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_304(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_price" => {
            let r = f_bond_price(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coupon_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("frequency") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_price_change" => {
            let r = f_bond_price_change(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coupon_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yield_change") { Some(x) => py::from_val(x), None => { return None } }, match v.get("frequency") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_428(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_ytm" => {
            let r = f_bond_ytm(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("coupon_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("frequency") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ytm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "break_even" => {
            let r = f_break_even(match v.get("fixed_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("variable_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target_profit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_517(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bs_greeks" => {
            let r = f_bs_greeks(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("volatility") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_550(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cagr" => {
            let r = f_cagr(match v.get("begin_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("end_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_573(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "capm" => {
            let r = f_capm(match v.get("risk_free") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("market_return") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cobb_douglas_demand" => {
            let r = f_cobb_douglas_demand(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("income") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_622(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "comparative_advantage" => {
            let r = f_comparative_advantage(match v.get("a_good_x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_good_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b_good_x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b_good_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("basis") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_672(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "competitive_firm_cost_curves" => {
            let r = f_competitive_firm_cost_curves(match v.get("quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_728(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "compound_interest" => {
            let r = f_compound_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_768(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "continuous_compounding" => {
            let r = f_continuous_compounding(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "correlation" => {
            let r = f_correlation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_832(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cournot" => {
            let r = f_cournot(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_864(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "credit_card_finance_charge" => {
            let r = f_credit_card_finance_charge(match v.get("opening_balance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("apr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("billing_days") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate_basis") { Some(x) => py::from_val(x), None => { return None } }, match v.get("transaction_days") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("transaction_amounts") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_930(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_price_elasticity" => {
            let r = f_cross_price_elasticity(match v.get("price_other_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_other_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_966(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crossover_rate" => {
            let r = f_crossover_rate(match v.get("cash_flows_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_crossover(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "direct_cost_variances" => {
            let r = f_direct_cost_variances(match v.get("standard_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("standard_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_purchased") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1070(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discounted_payback" => {
            let r = f_discounted_payback(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "doubling_time" => {
            let r = f_doubling_time(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "durbin_watson" => {
            let r = f_durbin_watson(match v.get("residuals") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1161(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "effective_annual_rate" => {
            let r = f_effective_annual_rate(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1197(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "endowment_insurance" => {
            let r = f_endowment_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equivalent_annual_annuity" => {
            let r = f_equivalent_annual_annuity(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1282(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "financial_ratios" => {
            let r = f_financial_ratios(match v.get("sales") { Some(x) => py::from_val(x), None => { return None } }, match v.get("net_income") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_assets") { Some(x) => py::from_val(x), None => { return None } }, match v.get("total_equity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_assets") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_liabilities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("inventory") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1315(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fisher_real_rate" => {
            let r = f_fisher_real_rate(match v.get("nominal_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("inflation_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1339(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_price" => {
            let r = f_forward_price(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("storage_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1368(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_price_dividends" => {
            let r = f_forward_price_dividends(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1409(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_rate" => {
            let r = f_forward_rate(match v.get("spot_rate_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("maturity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot_rate_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("maturity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1440(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "future_value_annuity" => {
            let r = f_future_value_annuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1482(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gdp_deflator" => {
            let r = f_gdp_deflator(match v.get("base_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gini_coefficient" => {
            let r = f_gini_coefficient(match v.get("incomes") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1562(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "global_min_variance" => {
            let r = f_global_min_variance(match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1625(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gordon_growth" => {
            let r = f_gordon_growth(match v.get("required_return") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("next_dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gordon_implied_return" => {
            let r = f_gordon_implied_return(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("next_dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "growing_annuity" => {
            let r = f_growing_annuity(match v.get("first_payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1736(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "growing_perpetuity" => {
            let r = f_growing_perpetuity(match v.get("first_payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1762(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "herfindahl_index" => {
            let r = f_herfindahl_index(match v.get("market_shares") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1794(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "historical_var" => {
            let r = f_historical_var(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1827(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "implied_volatility" => {
            let r = f_implied_volatility(match v.get("option_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("option_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("call")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_iv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "income_elasticity" => {
            let r = f_income_elasticity(match v.get("income_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("income_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1894(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "income_tax" => {
            let r = f_income_tax(match v.get("taxable_income") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bracket_thresholds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bracket_rates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1936(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inflation_adjustment" => {
            let r = f_inflation_adjustment(match v.get("price_levels") { Some(x) => py::from_val(x), None => { return None } }, match v.get("amount") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "information_ratio" => {
            let r = f_information_ratio(match v.get("portfolio_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benchmark_returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inventory_costing" => {
            let r = f_inventory_costing(match v.get("layer_units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("layer_costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units_sold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("selling_price") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "irr" => {
            let r = f_irr(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_irr(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "life_annuity_due" => {
            let r = f_life_annuity_due(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2158(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "life_expectancy" => {
            let r = f_life_expectancy(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_balance" => {
            let r = f_loan_balance(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_made") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2242(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_cumulative_interest" => {
            let r = f_loan_cumulative_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_made") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2294(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_payment" => {
            let r = f_loan_payment(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2341(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_payment_split" => {
            let r = f_loan_payment_split(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment_number") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2391(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_log_elasticity" => {
            let r = f_log_log_elasticity(match v.get("prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "market_equilibrium" => {
            let r = f_market_equilibrium(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_slope") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2468(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_drawdown" => {
            let r = f_max_drawdown(match v.get("prices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_returns" => {
            let r = f_mean_returns(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2526(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_variance_two_asset" => {
            let r = f_min_variance_two_asset(match v.get("sd_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("correlation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2554(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mirr" => {
            let r = f_mirr(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } }, match v.get("finance_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reinvest_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "money_multiplier" => {
            let r = f_money_multiplier(match v.get("reserve_ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("currency_ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("monetary_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2629(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "monopoly" => {
            let r = f_monopoly(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("marginal_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("cost_quadratic") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2657(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multiple_regression" => {
            let r = f_multiple_regression(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2714(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "net_premium_whole_life" => {
            let r = f_net_premium_whole_life(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2755(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nominal_rate" => {
            let r = f_nominal_rate(match v.get("effective_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2791(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "npv" => {
            let r = f_npv(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2821(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "number_of_periods" => {
            let r = f_number_of_periods(match v.get("present_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2855(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ols_simple" => {
            let r = f_ols_simple(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "overhead_cost_variances" => {
            let r = f_overhead_cost_variances(match v.get("variable_overhead_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_variable_overhead") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_hours") { Some(x) => py::from_val(x), None => { return None } }, match v.get("standard_hours") { Some(x) => py::from_val(x), None => { return None } }, match v.get("budgeted_fixed_overhead") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual_fixed_overhead") { Some(x) => py::from_val(x), None => { return None } }, match v.get("denominator_hours") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2923(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_var" => {
            let r = f_parametric_var(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("std_dev") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "payback_period" => {
            let r = f_payback_period(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2989(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "per_unit_tax" => {
            let r = f_per_unit_tax(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3028(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "perpetuity" => {
            let r = f_perpetuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3056(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_elasticity" => {
            let r = f_point_elasticity(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "portfolio_return_risk" => {
            let r = f_portfolio_return_risk(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("expected_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "portfolio_var" => {
            let r = f_portfolio_var(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "present_value" => {
            let r = f_present_value(match v.get("future_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3231(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "present_value_annuity" => {
            let r = f_present_value_annuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3273(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "price_indices" => {
            let r = f_price_indices(match v.get("base_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3320(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "profitability_index" => {
            let r = f_profitability_index(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3352(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "put_call_parity" => {
            let r = f_put_call_parity(match v.get("call_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "scenario_return" => {
            let r = f_scenario_return(match v.get("probabilities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sharpe_ratio" => {
            let r = f_sharpe_ratio(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_interest" => {
            let r = f_simple_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3502(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sinking_fund" => {
            let r = f_sinking_fund(match v.get("future_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3546(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solow_steady_state" => {
            let r = f_solow_steady_state(match v.get("saving_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("population_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("depreciation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("technology_growth") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("productivity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sortino_ratio" => {
            let r = f_sortino_ratio(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3616(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spending_multiplier" => {
            let r = f_spending_multiplier(match v.get("mpc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax_rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("import_propensity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("spending_change") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3648(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stackelberg" => {
            let r = f_stackelberg(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_leader") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_follower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "survival_probability" => {
            let r = f_survival_probability(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("deferred_years") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3725(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangency_portfolio" => {
            let r = f_tangency_portfolio(match v.get("expected_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "temporary_annuity_due" => {
            let r = f_temporary_annuity_due(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "term_insurance" => {
            let r = f_term_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3908(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "treynor_jensen" => {
            let r = f_treynor_jensen(match v.get("portfolio_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("market_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_asset_portfolio" => {
            let r = f_two_asset_portfolio(match v.get("weight_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("return_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("return_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("correlation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_stage_ddm" => {
            let r = f_two_stage_ddm(match v.get("dividend") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high_growth_years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stable_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("required_return") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4027(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unemployment_rate" => {
            let r = f_unemployment_rate(match v.get("employed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("unemployed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("working_age_population") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4059(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wacc" => {
            let r = f_wacc(match v.get("equity_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("debt_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_of_equity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_of_debt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4091(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "whole_life_insurance" => {
            let r = f_whole_life_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_coupon_bond" => {
            let r = f_zero_coupon_bond(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("compounding") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("annual")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4162(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__ZC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("annual"), Py::Int(1i64)), (Py::str("semiannual"), Py::Int(2i64)), (Py::str("quarterly"), Py::Int(4i64)), (Py::str("monthly"), Py::Int(12i64)), (Py::str("continuous"), Py::Int(0i64))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__A(mut v_q: Py, mut v_v: Py, mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    v_p = f__surv(v_q.clone())?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t4129 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4129;
out.push(py::mul(&py::mul(&py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_p.clone(), &v_k.clone())?)?, &py::getitem(&v_q.clone(), &v_k.clone())?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__N(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_x.clone())?, &py::math1("sqrt", &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__a_due(mut v_q: Py, mut v_v: Py, mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    v_p = f__surv(v_q.clone())?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t3856 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3856;
out.push(py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &py::getitem(&v_p.clone(), &v_k.clone())?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__annuity_fv(mut v_p: Py, mut v_r: Py, mut v_n: Py, mut v_due: Py) -> R {
    let mut v_f: Py = Py::None;
    v_f = (if ({ let t3544 = v_r.clone(); Py::Bool({ let t3545 = Py::Int(0i64); if py::eq(&t3544, &t3545) { true } else { false } }) }).truthy() { v_n.clone() } else { py::truediv(&py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?, &Py::Int(1i64))?, &v_r.clone())? });
    return Ok(py::mul(&py::mul(&v_p.clone(), &v_f.clone())?, &(if (v_due.clone()).truthy() { py::add(&Py::Int(1i64), &v_r.clone())? } else { Py::Int(1i64) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__annuity_pv(mut v_p: Py, mut v_r: Py, mut v_n: Py, mut v_due: Py) -> R {
    let mut v_f: Py = Py::None;
    v_f = (if ({ let t3271 = v_r.clone(); Py::Bool({ let t3272 = Py::Int(0i64); if py::eq(&t3271, &t3272) { true } else { false } }) }).truthy() { v_n.clone() } else { py::truediv(&py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &py::neg(&v_n.clone())?)?)?, &v_r.clone())? });
    return Ok(py::mul(&py::mul(&v_p.clone(), &v_f.clone())?, &(if (v_due.clone()).truthy() { py::add(&Py::Int(1i64), &v_r.clone())? } else { Py::Int(1i64) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__arc(mut v_x1: Py, mut v_q1: Py, mut v_x2: Py, mut v_q2: Py, mut v_nx: Py, mut v_nq: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t1884 = (Py::tuple(vec![f__pos(py::add(&v_nx.clone(), &Py::str("_1"))?, v_x1.clone())?, f__pos(py::add(&v_nq.clone(), &Py::str("_1"))?, v_q1.clone())?, f__pos(py::add(&v_nx.clone(), &Py::str("_2"))?, v_x2.clone())?, f__pos(py::add(&v_nq.clone(), &Py::str("_2"))?, v_q2.clone())?])).items()?;
    if t1884.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1884[0].clone();
    v_b = t1884[1].clone();
    v_c = t1884[2].clone();
    v_d = t1884[3].clone();
    let _ = f__need({ let t1885 = v_a.clone(); Py::Bool({ let t1886 = v_c.clone(); if py::ne(&t1885, &t1886) { true } else { false } }) }, Py::str(&["the two ".to_string(), py::format(&py::method(&v_nx.clone(), "replace", &[Py::str("_"), Py::str(" ")])?, "")?, "s must differ".to_string()].concat()))?;
    v_e = py::truediv(&py::truediv(&py::sub(&v_d.clone(), &v_b.clone())?, &py::truediv(&py::add(&v_b.clone(), &v_d.clone())?, &Py::Int(2i64))?)?, &py::truediv(&py::sub(&v_c.clone(), &v_a.clone())?, &py::truediv(&py::add(&v_a.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?;
    v_ex = py::truediv(&py::truediv(&py::sub(&py::fraction(&[v_d.clone()])?, &py::fraction(&[v_b.clone()])?)?, &py::add(&py::fraction(&[v_b.clone()])?, &py::fraction(&[v_d.clone()])?)?)?, &py::truediv(&py::sub(&py::fraction(&[v_c.clone()])?, &py::fraction(&[v_a.clone()])?)?, &py::add(&py::fraction(&[v_a.clone()])?, &py::fraction(&[v_c.clone()])?)?)?)?;
    return Ok(Py::tuple(vec![v_e.clone(), v_ex.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__balance(mut v_P: Py, mut v_i: Py, mut v_pmt: Py, mut v_k: Py) -> R {
    return Ok((if ({ let t2389 = v_i.clone(); Py::Bool({ let t2390 = Py::Int(0i64); if py::eq(&t2389, &t2390) { true } else { false } }) }).truthy() { py::sub(&v_P.clone(), &py::mul(&v_pmt.clone(), &v_k.clone())?)? } else { py::sub(&py::mul(&v_P.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &v_k.clone())?)?, &py::truediv(&py::mul(&v_pmt.clone(), &py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &v_k.clone())?, &Py::Int(1i64))?)?, &v_i.clone())?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__baskets(mut v_p0: Py, mut v_q0: Py, mut v_p1: Py, mut v_q1: Py) -> R {
    let t3299 = ({ let mut out = vec![]; for t3291 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("base_prices"), v_p0.clone()]), Py::tuple(vec![Py::str("base_quantities"), v_q0.clone()]), Py::tuple(vec![Py::str("current_prices"), v_p1.clone()]), Py::tuple(vec![Py::str("current_quantities"), v_q1.clone()])])))? { let mut v_n: Py; let mut v_v: Py; 
let t3292 = (t3291).items()?;
if t3292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t3292[0].clone();
v_v = t3292[1].clone();
out.push(f__vec(v_n.clone(), v_v.clone(), Py::Int(1i64))?); } Py::list(out) }).items()?;
    if t3299.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t3299[0].clone();
    v_q0 = t3299[1].clone();
    v_p1 = t3299[2].clone();
    v_q1 = t3299[3].clone();
    let _ = f__need({ let t3300 = py::len(&v_p0.clone())?; Py::Bool({ let t3301 = py::len(&v_q0.clone())?; if py::eq(&t3300, &t3301) { { let t3302 = py::len(&v_p1.clone())?; if py::eq(&t3301, &t3302) { { let t3303 = py::len(&v_q1.clone())?; if py::eq(&t3302, &t3303) { true } else { false } } } else { false } } } else { false } }) }, Py::str("one price and quantity per good in each year"))?;
    let _ = f__need({ let t3310 = Py::Bool(({ let mut out = vec![]; for t3304 in py::iter(&(py::add(&v_p0.clone(), &v_p1.clone())?))? { let mut v_x: Py; 
v_x = t3304;
out.push({ let t3305 = v_x.clone(); Py::Bool({ let t3306 = Py::Int(0i64); if py::gt(&t3305, &t3306)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t3310.truthy() { t3310 } else { Py::Bool(({ let mut out = vec![]; for t3307 in py::iter(&(py::add(&v_q0.clone(), &v_q1.clone())?))? { let mut v_x: Py; 
v_x = t3307;
out.push({ let t3308 = v_x.clone(); Py::Bool({ let t3309 = Py::Int(0i64); if py::ge(&t3308, &t3309)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("prices must be positive and quantities non-negative"))?;
    let _ = f__need({ let t3315 = { let t3311 = py::sum(&v_q0.clone(), None)?; Py::Bool({ let t3312 = Py::Int(0i64); if py::gt(&t3311, &t3312)? { true } else { false } }) }; if !t3315.truthy() { t3315 } else { { let t3313 = py::sum(&v_q1.clone(), None)?; Py::Bool({ let t3314 = Py::Int(0i64); if py::gt(&t3313, &t3314)? { true } else { false } }) } } }, Py::str("each year needs a positive quantity"))?;
    return Ok(Py::tuple(vec![v_p0.clone(), v_q0.clone(), v_p1.clone(), v_q1.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bond(mut v_face: Py, mut v_coupon_rate: Py, mut v_years: Py, mut v_frequency: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    v_F = f__pos(Py::str("face"), v_face.clone())?;
    v_c = f__fin(Py::str("coupon_rate"), v_coupon_rate.clone())?;
    let _ = f__need({ let t450 = Py::Int(0i64); Py::Bool({ let t451 = v_c.clone(); if py::le(&t450, &t451)? { { let t452 = Py::Int(1i64); if py::lt(&t451, &t452)? { true } else { false } } } else { false } }) }, Py::str("coupon_rate must be a decimal between 0 and 1 (write 5% or 0.05)"))?;
    v_f = f__int(Py::str("frequency"), v_frequency.clone(), Py::Int(1i64), Py::Int(12i64))?;
    let _ = f__need({ let t465 = py::modulo(&Py::Int(12i64), &v_f.clone())?; Py::Bool({ let t466 = Py::Int(0i64); if py::eq(&t465, &t466) { true } else { false } }) }, Py::str("frequency is 1, 2, 3, 4, 6 or 12 coupons a year"))?;
    v_N = f__periods_of(f__pos(Py::str("years"), v_years.clone())?, v_f.clone(), Py::str("years"))?;
    return Ok(Py::tuple(vec![v_F.clone(), v_c.clone(), v_f.clone(), v_N.clone(), py::truediv(&py::mul(&v_F.clone(), &v_c.clone())?, &v_f.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bprice(mut v_F: Py, mut v_cpn: Py, mut v_N: Py, mut v_j: Py) -> R {
    let mut v_v: Py = Py::None;
    if ({ let t475 = v_j.clone(); Py::Bool({ let t476 = Py::Int(0i64); if py::eq(&t475, &t476) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::mul(&v_cpn.clone(), &v_N.clone())?, &v_F.clone())?);
    }
    v_v = py::pow(&py::add(&Py::Int(1i64), &v_j.clone())?, &py::neg(&v_N.clone())?)?;
    return Ok(py::add(&py::truediv(&py::mul(&v_cpn.clone(), &py::sub(&Py::Int(1i64), &v_v.clone())?)?, &v_j.clone())?, &py::mul(&v_F.clone(), &v_v.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bs(mut v_S: Py, mut v_K: Py, mut v_T: Py, mut v_r: Py, mut v_s: Py, mut v_q: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_dq: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_sq: Py = Py::None;
    v_sq = py::mul(&v_s.clone(), &py::math1("sqrt", &v_T.clone())?)?;
    v_d1 = py::truediv(&py::add(&py::math1("log", &py::truediv(&v_S.clone(), &v_K.clone())?)?, &py::mul(&py::add(&py::sub(&v_r.clone(), &v_q.clone())?, &py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &Py::Int(2i64))?)?, &v_T.clone())?)?, &v_sq.clone())?;
    v_d2 = py::sub(&v_d1.clone(), &v_sq.clone())?;
    let t1859 = (Py::tuple(vec![py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?, py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_T.clone())?)?])).items()?;
    if t1859.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dq = t1859[0].clone();
    v_dr = t1859[1].clone();
    return Ok(Py::tuple(vec![v_d1.clone(), v_d2.clone(), py::sub(&py::mul(&py::mul(&v_S.clone(), &v_dq.clone())?, &f__N(v_d1.clone())?)?, &py::mul(&py::mul(&v_K.clone(), &v_dr.clone())?, &f__N(v_d2.clone())?)?)?, py::sub(&py::mul(&py::mul(&v_K.clone(), &v_dr.clone())?, &f__N(py::neg(&v_d2.clone())?)?)?, &py::mul(&py::mul(&v_S.clone(), &v_dq.clone())?, &f__N(py::neg(&v_d1.clone())?)?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cc_inputs(mut v_opening_balance: Py, mut v_apr: Py, mut v_billing_days: Py, mut v_rate_basis: Py, mut v_transaction_days: Py, mut v_transaction_amounts: Py) -> R {
    let mut v_B0: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_amts: Py = Py::None;
    let mut v_cycle: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_days: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_B0 = f__nonneg(Py::str("opening_balance"), v_opening_balance.clone())?;
    v_r = f__rate(Py::str("apr"), v_apr.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?;
    v_D = f__int(Py::str("billing_days"), v_billing_days.clone(), Py::Int(1i64), Py::Int(366i64))?;
    let _ = f__need({ let t901 = v_rate_basis.clone(); Py::Bool({ let t902 = Py::tuple(vec![Py::str("daily"), Py::str("monthly")]); if py::contains(&t902, &t901)? { true } else { false } }) }, Py::str("rate basis is 'daily' (APR/365 for each day of the cycle) or 'monthly' (APR/12)"))?;
    v_days = (if ({ let t903 = v_transaction_days.clone(); Py::Bool({ let t904 = Py::None; if py::is(&t903, &t904) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__vec(Py::str("transaction_days"), v_transaction_days.clone(), Py::Int(1i64))? });
    v_amts = (if ({ let t909 = v_transaction_amounts.clone(); Py::Bool({ let t910 = Py::None; if py::is(&t909, &t910) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__vec(Py::str("transaction_amounts"), v_transaction_amounts.clone(), Py::Int(1i64))? });
    let _ = f__need({ let t911 = py::len(&v_days.clone())?; Py::Bool({ let t912 = py::len(&v_amts.clone())?; if py::eq(&t911, &t912) { true } else { false } }) }, Py::str("transaction days and transaction amounts need the same number of entries"))?;
    v_out = Py::list(vec![]);
    for t913 in py::range_iter(&[(py::len(&v_days.clone())?).i()?]) {
        v_k = t913;
        v_d = py::getitem(&v_days.clone(), &v_k.clone())?;
        let _ = f__need({ let t919 = { let t914 = v_d.clone(); Py::Bool({ let t915 = py::math1("floor", &v_d.clone())?; if py::eq(&t914, &t915) { true } else { false } }) }; if !t919.truthy() { t919 } else { { let t916 = Py::Int(1i64); Py::Bool({ let t917 = v_d.clone(); if py::le(&t916, &t917)? { { let t918 = v_D.clone(); if py::le(&t917, &t918)? { true } else { false } } } else { false } }) } } }, Py::str("each transaction day must be a whole day of the cycle, from 1 to the number of billing days"))?;
        let _ = py::method(&v_out.clone(), "append", &[py::int(&v_d.clone())?])?;
    }
    v_cycle = (if ({ let t920 = v_rate_basis.clone(); Py::Bool({ let t921 = Py::str("daily"); if py::eq(&t920, &t921) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_r.clone(), &v_D.clone())?, &Py::Float(365.0))? } else { py::truediv(&v_r.clone(), &Py::Float(12.0))? });
    return Ok(Py::tuple(vec![v_B0.clone(), v_D.clone(), v_out.clone(), v_amts.clone(), v_cycle.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_cc(mut v_opening_balance: Py, mut v_apr: Py, mut v_billing_days: Py, mut v_rate_basis: Py, mut v_transaction_days: Py, mut v_transaction_amounts: Py) -> R {
    let mut v_B0: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_adb: Py = Py::None;
    let mut v_amts: Py = Py::None;
    let mut v_bal: Py = Py::None;
    let mut v_cycle: Py = Py::None;
    let mut v_day: Py = Py::None;
    let mut v_days: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_total: Py = Py::None;
    let t931 = (f__cc_inputs(v_opening_balance.clone(), v_apr.clone(), v_billing_days.clone(), v_rate_basis.clone(), v_transaction_days.clone(), v_transaction_amounts.clone())?).items()?;
    if t931.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_B0 = t931[0].clone();
    v_D = t931[1].clone();
    v_days = t931[2].clone();
    v_amts = t931[3].clone();
    v_cycle = t931[4].clone();
    let t932 = (Py::tuple(vec![v_B0.clone(), Py::Float(0.0)])).items()?;
    if t932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bal = t932[0].clone();
    v_total = t932[1].clone();
    for t933 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_D.clone(), &Py::Int(1i64))?).i()?]) {
        v_day = t933;
        for t934 in py::range_iter(&[(py::len(&v_days.clone())?).i()?]) {
            v_k = t934;
            if ({ let t935 = py::getitem(&v_days.clone(), &v_k.clone())?; Py::Bool({ let t936 = v_day.clone(); if py::eq(&t935, &t936) { true } else { false } }) }).truthy() {
                v_bal = py::iadd(&v_bal, &py::getitem(&v_amts.clone(), &v_k.clone())?)?;
            }
        }
        v_total = py::iadd(&v_total, &v_bal.clone())?;
    }
    v_adb = py::truediv(&v_total.clone(), &v_D.clone())?;
    v_fc = py::mul(&v_adb.clone(), &v_cycle.clone())?;
    return Ok(Py::dict(vec![(Py::str("average_daily_balance"), v_adb.clone()), (Py::str("finance_charge"), v_fc.clone()), (Py::str("cycle_rate"), v_cycle.clone()), (Py::str("closing_balance"), v_bal.clone()), (Py::str("new_balance"), py::add(&v_bal.clone(), &v_fc.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_income_tax(mut v_taxable_income: Py, mut v_bracket_thresholds: Py, mut v_bracket_rates: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tax: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1937 = (f__tax_inputs(v_taxable_income.clone(), v_bracket_thresholds.clone(), v_bracket_rates.clone())?).items()?;
    if t1937.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1937[0].clone();
    v_t = t1937[1].clone();
    v_rs = t1937[2].clone();
    let t1938 = (Py::tuple(vec![Py::Float(0.0), py::getitem(&v_rs.clone(), &Py::Int(0i64))?])).items()?;
    if t1938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tax = t1938[0].clone();
    v_m = t1938[1].clone();
    for t1939 in py::range_iter(&[(py::len(&v_t.clone())?).i()?]) {
        v_k = t1939;
        v_top = (if ({ let t1940 = v_k.clone(); Py::Bool({ let t1941 = py::sub(&py::len(&v_t.clone())?, &Py::Int(1i64))?; if py::eq(&t1940, &t1941) { true } else { false } }) }).truthy() { v_x.clone() } else { py::minmax(&[v_x.clone(), py::getitem(&v_t.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?], false)? });
        if ({ let t1942 = v_top.clone(); Py::Bool({ let t1943 = py::getitem(&v_t.clone(), &v_k.clone())?; if py::gt(&t1942, &t1943)? { true } else { false } }) }).truthy() {
            v_tax = py::iadd(&v_tax, &py::mul(&py::getitem(&v_rs.clone(), &v_k.clone())?, &py::sub(&v_top.clone(), &py::getitem(&v_t.clone(), &v_k.clone())?)?)?)?;
        }
        if ({ let t1944 = v_x.clone(); Py::Bool({ let t1945 = py::getitem(&v_t.clone(), &v_k.clone())?; if py::ge(&t1944, &t1945)? { true } else { false } }) }).truthy() {
            v_m = py::getitem(&v_rs.clone(), &v_k.clone())?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("tax"), v_tax.clone()), (Py::str("marginal_rate"), v_m.clone()), (Py::str("effective_rate"), py::truediv(&v_tax.clone(), &v_x.clone())?), (Py::str("after_tax_income"), py::sub(&v_x.clone(), &v_tax.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_inventory(mut v_layer_units: Py, mut v_layer_costs: Py, mut v_units_sold: Py, mut v_selling_price: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cum: Py = Py::None;
    let mut v_f_cogs: Py = Py::None;
    let mut v_f_end: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_keep: Py = Py::None;
    let mut v_kept_k: Py = Py::None;
    let mut v_l_cogs: Py = Py::None;
    let mut v_l_end: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sold_k: Py = Py::None;
    let mut v_total_c: Py = Py::None;
    let mut v_total_u: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t2054 = (f__layers(v_layer_units.clone(), v_layer_costs.clone(), v_units_sold.clone())?).items()?;
    if t2054.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2054[0].clone();
    v_c = t2054[1].clone();
    v_total_u = t2054[2].clone();
    v_total_c = t2054[3].clone();
    v_q = t2054[4].clone();
    v_keep = py::sub(&v_total_u.clone(), &v_q.clone())?;
    v_cum = Py::Float(0.0);
    let t2055 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2055.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_f_cogs = t2055[0].clone();
    v_f_end = t2055[1].clone();
    v_l_cogs = t2055[2].clone();
    v_l_end = t2055[3].clone();
    for t2056 in py::range_iter(&[(py::len(&v_u.clone())?).i()?]) {
        v_k = t2056;
        v_sold_k = py::minmax(&[py::minmax(&[py::sub(&v_q.clone(), &v_cum.clone())?, Py::Float(0.0)], true)?, py::getitem(&v_u.clone(), &v_k.clone())?], false)?;
        v_kept_k = py::minmax(&[py::minmax(&[py::sub(&v_keep.clone(), &v_cum.clone())?, Py::Float(0.0)], true)?, py::getitem(&v_u.clone(), &v_k.clone())?], false)?;
        v_f_cogs = py::iadd(&v_f_cogs, &py::mul(&v_sold_k.clone(), &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_f_end = py::iadd(&v_f_end, &py::mul(&py::sub(&py::getitem(&v_u.clone(), &v_k.clone())?, &v_sold_k.clone())?, &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_l_end = py::iadd(&v_l_end, &py::mul(&v_kept_k.clone(), &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_l_cogs = py::iadd(&v_l_cogs, &py::mul(&py::sub(&py::getitem(&v_u.clone(), &v_k.clone())?, &v_kept_k.clone())?, &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_cum = py::iadd(&v_cum, &py::getitem(&v_u.clone(), &v_k.clone())?)?;
    }
    return Ok(f__inventory_out(v_total_u.clone(), v_total_c.clone(), v_q.clone(), Py::tuple(vec![v_f_cogs.clone(), v_f_end.clone()]), Py::tuple(vec![v_l_cogs.clone(), v_l_end.clone()]), v_selling_price.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_npv(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("npv"), py::sum(&{ let mut out = vec![]; for t2822 in py::iter(&(py::enumerate(&v_cash_flows.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t2823 = (t2822).items()?;
if t2823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2823[0].clone();
v_c = t2823[1].clone();
out.push(py::truediv(&v_c.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_rate.clone())?, &v_t.clone())?)?); } Py::list(out) }, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chol_solve(mut v_L: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_L.clone())?;
    v_y = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t3792 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3792;
        py::setitem(&v_y.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t3793 in py::range_iter(&[(v_i.clone()).i()?]) { let mut v_k: Py; 
v_k = t3793;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t3794 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t3794;
        py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t3795 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3795;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cholesky(mut v_M: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tol: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_L = { let mut out = vec![]; for t3781 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3781;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    v_tol = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3782 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3782;
out.push(py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }], true)?)?;
    for t3783 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3783;
        for t3784 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?]) {
            v_j = t3784;
            v_s = py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t3785 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t3785;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?;
            if ({ let t3786 = v_i.clone(); Py::Bool({ let t3787 = v_j.clone(); if py::eq(&t3786, &t3787) { true } else { false } }) }).truthy() {
                if ({ let t3788 = v_s.clone(); Py::Bool({ let t3789 = v_tol.clone(); if py::le(&t3788, &t3789)? { true } else { false } }) }).truthy() {
                    return Ok(Py::None);
                }
                py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_i.clone(), py::math1("sqrt", &v_s.clone())?)?;
            } else {
                py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_j.clone(), py::truediv(&v_s.clone(), &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?)?)?;
            }
        }
    }
    return Ok(v_L.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__confidence(mut v_c: Py) -> R {
    v_c = f__fin(Py::str("confidence"), v_c.clone())?;
    let _ = f__need({ let t3193 = Py::Float(0.5); Py::Bool({ let t3194 = v_c.clone(); if py::lt(&t3193, &t3194)? { { let t3195 = Py::Int(1i64); if py::lt(&t3194, &t3195)? { true } else { false } } } else { false } }) }, Py::str("confidence is a decimal between 0.5 and 1 (95% = 0.95)"))?;
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__consume(mut v_u: Py, mut v_c: Py, mut v_q: Py, mut v_order: Py) -> R {
    let mut v_cogs: Py = Py::None;
    let mut v_ending: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_take: Py = Py::None;
    v_left = v_q.clone();
    v_cogs = Py::Float(0.0);
    v_ending = Py::Float(0.0);
    for t2052 in py::iter(&(v_order.clone()))? {
        v_k = t2052;
        v_take = py::minmax(&[v_left.clone(), py::getitem(&v_u.clone(), &v_k.clone())?], false)?;
        v_cogs = py::iadd(&v_cogs, &py::mul(&v_take.clone(), &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_ending = py::iadd(&v_ending, &py::mul(&py::sub(&py::getitem(&v_u.clone(), &v_k.clone())?, &v_take.clone())?, &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
        v_left = py::sub(&v_left, &v_take.clone())?;
    }
    return Ok(Py::tuple(vec![v_cogs.clone(), v_ending.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cost_table(mut v_quantities: Py, mut v_total_costs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_Q = f__vec(Py::str("quantities"), v_quantities.clone(), Py::Int(3i64))?;
    v_C = f__vec(Py::str("total_costs"), v_total_costs.clone(), Py::Int(3i64))?;
    let _ = f__need({ let t696 = py::len(&v_Q.clone())?; Py::Bool({ let t697 = py::len(&v_C.clone())?; if py::eq(&t696, &t697) { true } else { false } }) }, Py::str("give one total cost for every output level"))?;
    let _ = f__need({ let t698 = py::getitem(&v_Q.clone(), &Py::Int(0i64))?; Py::Bool({ let t699 = Py::Int(0i64); if py::eq(&t698, &t699) { true } else { false } }) }, Py::str("the table starts at output 0 (its total cost is the fixed cost)"))?;
    let _ = f__need({ let t700 = py::getitem(&v_C.clone(), &Py::Int(0i64))?; Py::Bool({ let t701 = Py::Int(0i64); if py::ge(&t700, &t701)? { true } else { false } }) }, Py::str("the fixed cost must not be negative"))?;
    for t702 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_Q.clone())?).i()?]) {
        v_k = t702;
        let _ = f__need({ let t703 = py::getitem(&v_Q.clone(), &v_k.clone())?; Py::Bool({ let t704 = py::getitem(&v_Q.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t703, &t704)? { true } else { false } }) }, Py::str("output levels must increase"))?;
        let _ = f__need({ let t705 = py::getitem(&v_C.clone(), &v_k.clone())?; Py::Bool({ let t706 = py::getitem(&v_C.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::ge(&t705, &t706)? { true } else { false } }) }, Py::str("total cost must not fall as output rises"))?;
    }
    return Ok(Py::tuple(vec![v_Q.clone(), v_C.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cov(mut v_xs: Py, mut v_ys: Py, mut v_ddof: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let t3943 = (Py::tuple(vec![f__mean(v_xs.clone())?, f__mean(v_ys.clone())?])).items()?;
    if t3943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t3943[0].clone();
    v_my = t3943[1].clone();
    return Ok(py::truediv(&py::fsum(&{ let mut out = vec![]; for t3944 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3945 = (t3944).items()?;
if t3945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3945[0].clone();
v_y = t3945[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) })?, &py::sub(&py::len(&v_xs.clone())?, &v_ddof.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__d1(mut v_dividend: Py, mut v_next_dividend: Py, mut v_growth: Py) -> R {
    let _ = f__need({ let t1685 = { let t1687 = v_dividend.clone(); Py::Bool({ let t1688 = Py::None; if py::is(&t1687, &t1688) { true } else { false } }) }; Py::Bool({ let t1686 = { let t1689 = v_next_dividend.clone(); Py::Bool({ let t1690 = Py::None; if py::is(&t1689, &t1690) { true } else { false } }) }; if py::ne(&t1685, &t1686) { true } else { false } }) }, Py::str("give either the dividend just paid (D0) or the next dividend (D1), not both"))?;
    return Ok((if ({ let t1691 = v_next_dividend.clone(); Py::Bool({ let t1692 = Py::None; if !py::is(&t1691, &t1692) { true } else { false } }) }).truthy() { f__fin(Py::str("next_dividend"), v_next_dividend.clone())? } else { py::mul(&f__fin(Py::str("dividend"), v_dividend.clone())?, &py::add(&Py::Int(1i64), &v_growth.clone())?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__direct_var_inputs(mut v_standard_price: Py, mut v_standard_quantity: Py, mut v_actual_price: Py, mut v_actual_quantity: Py, mut v_quantity_purchased: Py) -> R {
    let mut v_AP: Py = Py::None;
    let mut v_AQ: Py = Py::None;
    let mut v_QP: Py = Py::None;
    let mut v_SP: Py = Py::None;
    let mut v_SQ: Py = Py::None;
    v_SP = f__pos(Py::str("standard_price"), v_standard_price.clone())?;
    v_SQ = f__pos(Py::str("standard_quantity"), v_standard_quantity.clone())?;
    v_AP = f__nonneg(Py::str("actual_price"), v_actual_price.clone())?;
    v_AQ = f__nonneg(Py::str("actual_quantity"), v_actual_quantity.clone())?;
    v_QP = (if ({ let t1061 = v_quantity_purchased.clone(); Py::Bool({ let t1062 = Py::None; if py::is(&t1061, &t1062) { true } else { false } }) }).truthy() { v_AQ.clone() } else { f__nonneg(Py::str("quantity_purchased"), v_quantity_purchased.clone())? });
    return Ok(Py::tuple(vec![v_SP.clone(), v_SQ.clone(), v_AP.clone(), v_AQ.clone(), v_QP.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dot(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3317 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3318 = (t3317).items()?;
if t3318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3318[0].clone();
v_y = t3318[1].clone();
out.push(py::mul(&v_x.clone(), &v_y.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fav(mut v_v: Py) -> R {
    if ({ let t2919 = v_v.clone(); Py::Bool({ let t2920 = Py::Int(0i64); if py::gt(&t2919, &t2920)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("unfavourable"));
    }
    if ({ let t2921 = v_v.clone(); Py::Bool({ let t2922 = Py::Int(0i64); if py::lt(&t2921, &t2922)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("favourable"));
    }
    return Ok(Py::str("none"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t4149 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4149.truthy() { t4149 } else { { let t4148 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4148.truthy() { t4148 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hnpv(mut v_rate: Py, mut v_cfs: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3350 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_rate.clone())?)?, Py::Float(0.0)])).items()?;
    if t3350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t3350[0].clone();
    v_acc = t3350[1].clone();
    for t3351 in py::iter(&(py::reversed(&v_cfs.clone())?))? {
        v_c = t3351;
        v_acc = py::add(&py::mul(&v_acc.clone(), &v_v.clone())?, &v_c.clone())?;
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t4016 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4016.truthy() { t4016 } else { { let t4015 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4015.truthy() { t4015 } else { { let t4014 = py::math1("isfinite", &v_v.clone())?; if !t4014.truthy() { t4014 } else { { let t4012 = py::float(&v_v.clone())?; Py::Bool({ let t4013 = py::int(&v_v.clone())?; if py::eq(&t4012, &t4013) { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))?;
    v_v = py::int(&v_v.clone())?;
    let _ = f__need({ let t4017 = v_v.clone(); Py::Bool({ let t4018 = v_lo.clone(); if py::ge(&t4017, &t4018)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    let _ = f__need({ let t4023 = { let t4019 = v_hi.clone(); Py::Bool({ let t4020 = Py::None; if py::is(&t4019, &t4020) { true } else { false } }) }; if t4023.truthy() { t4023 } else { { let t4021 = v_v.clone(); Py::Bool({ let t4022 = v_hi.clone(); if py::le(&t4021, &t4022)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inventory_out(mut v_total_u: Py, mut v_total_c: Py, mut v_q: Py, mut v_fifo: Py, mut v_lifo: Py, mut v_selling_price: Py) -> R {
    let mut v_avg: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rev: Py = Py::None;
    v_avg = py::truediv(&v_total_c.clone(), &v_total_u.clone())?;
    v_out = Py::dict(vec![(Py::str("units_available"), v_total_u.clone()), (Py::str("cost_of_goods_available"), v_total_c.clone()), (Py::str("fifo_cogs"), py::getitem(&v_fifo.clone(), &Py::Int(0i64))?), (Py::str("fifo_ending_inventory"), py::getitem(&v_fifo.clone(), &Py::Int(1i64))?), (Py::str("lifo_cogs"), py::getitem(&v_lifo.clone(), &Py::Int(0i64))?), (Py::str("lifo_ending_inventory"), py::getitem(&v_lifo.clone(), &Py::Int(1i64))?), (Py::str("average_unit_cost"), v_avg.clone()), (Py::str("average_cogs"), py::mul(&v_q.clone(), &v_avg.clone())?), (Py::str("average_ending_inventory"), py::mul(&py::sub(&v_total_u.clone(), &v_q.clone())?, &v_avg.clone())?)]);
    if ({ let t2050 = v_selling_price.clone(); Py::Bool({ let t2051 = Py::None; if !py::is(&t2050, &t2051) { true } else { false } }) }).truthy() {
        v_p = f__nonneg(Py::str("selling_price"), v_selling_price.clone())?;
        v_rev = py::mul(&v_p.clone(), &v_q.clone())?;
        py::setitem(&v_out.clone(), &Py::str("sales_revenue"), v_rev.clone())?;
        py::setitem(&v_out.clone(), &Py::str("fifo_gross_margin"), py::sub(&v_rev.clone(), &py::getitem(&v_fifo.clone(), &Py::Int(0i64))?)?)?;
        py::setitem(&v_out.clone(), &Py::str("lifo_gross_margin"), py::sub(&v_rev.clone(), &py::getitem(&v_lifo.clone(), &Py::Int(0i64))?)?)?;
        py::setitem(&v_out.clone(), &Py::str("average_gross_margin"), py::sub(&v_rev.clone(), &py::mul(&v_q.clone(), &v_avg.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__irr(mut v_cfs: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_nz: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_k = f__sign_changes(v_cfs.clone())?;
    let _ = f__need({ let t2085 = v_k.clone(); Py::Bool({ let t2086 = Py::Int(1i64); if py::ge(&t2085, &t2086)? { true } else { false } }) }, Py::str("the cash flows never change sign: there is no internal rate of return"))?;
    let _ = f__need({ let t2087 = v_k.clone(); Py::Bool({ let t2088 = Py::Int(1i64); if py::eq(&t2087, &t2088) { true } else { false } }) }, Py::str(&["the cash flows change sign ".to_string(), py::format(&v_k.clone(), "")?, " times: the internal rate of return is not unique, so none is given".to_string()].concat()))?;
    v_nz = { let mut out = vec![]; for t2089 in py::iter(&(v_cfs.clone()))? { let mut v_c: Py; 
v_c = t2089;
if ({ let t2090 = v_c.clone(); Py::Bool({ let t2091 = Py::Int(0i64); if py::ne(&t2090, &t2091) { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    v_first = { let t2092 = py::getitem(&v_nz.clone(), &Py::Int(0i64))?; Py::Bool({ let t2093 = Py::Int(0i64); if py::gt(&t2092, &t2093)? { true } else { false } }) };
    v_g = { let v_cfs = v_cfs.clone(); let v_first = v_first.clone(); let v_k = v_k.clone(); let v_nz = v_nz.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2094 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t2095 = (t2094).items()?;
if t2095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2095[0].clone();
v_c = t2095[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_v.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2096 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])).items()?;
    if t2096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2096[0].clone();
    v_hi = t2096[1].clone();
    while ({ let t2100 = { let t2102 = py::call(&v_g.clone(), &[v_hi.clone()])?; Py::Bool({ let t2103 = Py::Int(0i64); if py::gt(&t2102, &t2103)? { true } else { false } }) }; Py::Bool({ let t2101 = v_first.clone(); if py::eq(&t2100, &t2101) { true } else { false } }) }).truthy() {
        let t2097 = (Py::tuple(vec![v_hi.clone(), py::mul(&v_hi.clone(), &Py::Int(2i64))?])).items()?;
        if t2097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t2097[0].clone();
        v_hi = t2097[1].clone();
        let _ = f__need({ let t2098 = v_hi.clone(); Py::Bool({ let t2099 = py::pow(&Py::Float(2.0), &Py::Int(60i64))?; if py::lt(&t2098, &t2099)? { true } else { false } }) }, Py::str("the internal rate of return is too close to -100%"))?;
    }
    for t2104 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t2104;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t2105 = v_mid.clone(); Py::Bool({ let t2106 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t2106, &t2105)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t2107 = { let t2109 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t2110 = Py::Int(0i64); if py::gt(&t2109, &t2110)? { true } else { false } }) }; Py::Bool({ let t2108 = v_first.clone(); if py::eq(&t2107, &t2108) { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_r = py::sub(&py::truediv(&Py::Float(1.0), &py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?)?, &Py::Float(1.0))?;
    for t2111 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t2111;
        v_f = f__hnpv(v_r.clone(), v_cfs.clone())?;
        v_d = py::sum(&{ let mut out = vec![]; for t2114 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t2115 = (t2114).items()?;
if t2115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2115[0].clone();
v_c = t2115[1].clone();
out.push(py::mul(&py::mul(&py::neg(&v_t.clone())?, &v_c.clone())?, &py::pow(&py::add(&Py::Float(1.0), &v_r.clone())?, &py::sub(&py::neg(&v_t.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?;
        if ({ let t2118 = { let t2116 = v_d.clone(); Py::Bool({ let t2117 = Py::Int(0i64); if py::eq(&t2116, &t2117) { true } else { false } }) }; if t2118.truthy() { t2118 } else { py::not(&py::math1("isfinite", &py::truediv(&v_f.clone(), &v_d.clone())?)?) } }).truthy() {
            break;
        }
        v_nr = py::sub(&v_r.clone(), &py::truediv(&v_f.clone(), &v_d.clone())?)?;
        if (py::not(&{ let t2123 = { let t2119 = v_nr.clone(); Py::Bool({ let t2120 = py::neg(&Py::Int(1i64))?; if py::gt(&t2119, &t2120)? { true } else { false } }) }; if !t2123.truthy() { t2123 } else { { let t2121 = py::abs(&f__hnpv(v_nr.clone(), v_cfs.clone())?)?; Py::Bool({ let t2122 = py::abs(&v_f.clone())?; if py::le(&t2121, &t2122)? { true } else { false } }) } } })).truthy() {
            break;
        }
        v_r = v_nr.clone();
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__layers(mut v_layer_units: Py, mut v_layer_costs: Py, mut v_units_sold: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_total_c: Py = Py::None;
    let mut v_total_u: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_u = f__vec(Py::str("layer_units"), v_layer_units.clone(), Py::Int(1i64))?;
    v_c = f__vec(Py::str("layer_costs"), v_layer_costs.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2037 = py::len(&v_u.clone())?; Py::Bool({ let t2038 = py::len(&v_c.clone())?; if py::eq(&t2037, &t2038) { true } else { false } }) }, Py::str("give one unit cost for every layer"))?;
    v_total_u = Py::Float(0.0);
    v_total_c = Py::Float(0.0);
    for t2039 in py::range_iter(&[(py::len(&v_u.clone())?).i()?]) {
        v_k = t2039;
        let _ = f__need({ let t2040 = py::getitem(&v_u.clone(), &v_k.clone())?; Py::Bool({ let t2041 = Py::Int(0i64); if py::gt(&t2040, &t2041)? { true } else { false } }) }, Py::str("every layer needs a positive number of units"))?;
        let _ = f__need({ let t2042 = py::getitem(&v_c.clone(), &v_k.clone())?; Py::Bool({ let t2043 = Py::Int(0i64); if py::ge(&t2042, &t2043)? { true } else { false } }) }, Py::str("a unit cost must not be negative"))?;
        v_total_u = py::iadd(&v_total_u, &py::getitem(&v_u.clone(), &v_k.clone())?)?;
        v_total_c = py::iadd(&v_total_c, &py::mul(&py::getitem(&v_u.clone(), &v_k.clone())?, &py::getitem(&v_c.clone(), &v_k.clone())?)?)?;
    }
    v_q = f__nonneg(Py::str("units_sold"), v_units_sold.clone())?;
    let _ = f__need({ let t2046 = v_q.clone(); Py::Bool({ let t2047 = py::mul(&v_total_u.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t2046, &t2047)? { true } else { false } }) }, Py::str("more units were sold than were available for sale"))?;
    return Ok(Py::tuple(vec![v_u.clone(), v_c.clone(), v_total_u.clone(), v_total_c.clone(), v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__loan_state(mut v_principal: Py, mut v_annual_rate: Py, mut v_years: Py, mut v_payments_per_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pmt: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2366 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("annual_rate"), v_annual_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t2366.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2366[0].clone();
    v_r = t2366[1].clone();
    v_m = f__int(Py::str("payments_per_year"), v_payments_per_year.clone(), Py::Int(1i64), Py::Int(365i64))?;
    v_n = f__periods_of(f__pos(Py::str("years"), v_years.clone())?, v_m.clone(), Py::str("years"))?;
    v_i = py::truediv(&v_r.clone(), &v_m.clone())?;
    v_pmt = (if ({ let t2386 = v_i.clone(); Py::Bool({ let t2387 = Py::Int(0i64); if py::eq(&t2386, &t2387) { true } else { false } }) }).truthy() { py::truediv(&v_P.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_P.clone(), &v_i.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &py::neg(&v_n.clone())?)?)?)? });
    return Ok(Py::tuple(vec![v_P.clone(), v_i.clone(), v_n.clone(), v_pmt.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_xs: Py) -> R {
    return Ok(py::truediv(&py::fsum(&v_xs.clone())?, &py::len(&v_xs.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_v: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t4155 = v_v.clone(); Py::Bool({ let t4156 = Py::Int(0i64); if py::ge(&t4155, &t4156)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__npv_scale(mut v_rate: Py, mut v_cfs: Py) -> R {
    return Ok({ let t2136 = py::sum(&{ let mut out = vec![]; for t2134 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t2135 = (t2134).items()?;
if t2135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2135[0].clone();
v_c = t2135[1].clone();
out.push(py::mul(&py::abs(&v_c.clone())?, &py::pow(&py::add(&Py::Float(1.0), &v_rate.clone())?, &py::neg(&v_t.clone())?)?)?); } Py::list(out) }, None)?; if t2136.truthy() { t2136 } else { Py::Float(1.0) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__opt(mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_volatility: Py, mut v_dividend_yield: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3374 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__pos(Py::str("strike"), v_strike.clone())?, f__pos(Py::str("time"), v_time.clone())?])).items()?;
    if t3374.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3374[0].clone();
    v_K = t3374[1].clone();
    v_T = t3374[2].clone();
    let t3378 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("dividend_yield"), v_dividend_yield.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3378[0].clone();
    v_q = t3378[1].clone();
    v_s = f__pos(Py::str("volatility"), v_volatility.clone())?;
    let _ = f__need({ let t3379 = v_s.clone(); Py::Bool({ let t3380 = Py::Int(5i64); if py::lt(&t3379, &t3380)? { true } else { false } }) }, Py::str("volatility is a decimal (20% = 0.2)"))?;
    return Ok(Py::tuple(vec![v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__overhead_inputs(mut v_variable_overhead_rate: Py, mut v_actual_variable_overhead: Py, mut v_actual_hours: Py, mut v_standard_hours: Py, mut v_budgeted_fixed_overhead: Py, mut v_actual_fixed_overhead: Py, mut v_denominator_hours: Py) -> R {
    return Ok(Py::tuple(vec![f__nonneg(Py::str("variable_overhead_rate"), v_variable_overhead_rate.clone())?, f__nonneg(Py::str("actual_variable_overhead"), v_actual_variable_overhead.clone())?, f__nonneg(Py::str("actual_hours"), v_actual_hours.clone())?, f__nonneg(Py::str("standard_hours"), v_standard_hours.clone())?, f__nonneg(Py::str("budgeted_fixed_overhead"), v_budgeted_fixed_overhead.clone())?, f__nonneg(Py::str("actual_fixed_overhead"), v_actual_fixed_overhead.clone())?, f__pos(Py::str("denominator_hours"), v_denominator_hours.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair(mut v_a: Py, mut v_b: Py, mut v_na: Py, mut v_nb: Py, mut v_n_min: Py) -> R {
    let t3936 = (Py::tuple(vec![f__returns(v_na.clone(), v_a.clone(), v_n_min.clone(), Py::Float(1.0))?, f__returns(v_nb.clone(), v_b.clone(), v_n_min.clone(), Py::Float(1.0))?])).items()?;
    if t3936.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3936[0].clone();
    v_b = t3936[1].clone();
    let _ = f__need({ let t3937 = py::len(&v_a.clone())?; Py::Bool({ let t3938 = py::len(&v_b.clone())?; if py::eq(&t3937, &t3938) { true } else { false } }) }, Py::str(&[py::format(&v_na.clone(), "")?, " and ".to_string(), py::format(&v_nb.clone(), "")?, " must have the same number of observations".to_string()].concat()))?;
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__payback(mut v_flows: Py) -> R {
    let mut v_cum: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t2974 = py::getitem(&v_flows.clone(), &Py::Int(0i64))?; Py::Bool({ let t2975 = Py::Int(0i64); if py::lt(&t2974, &t2975)? { true } else { false } }) }, Py::str("the first cash flow (the investment at time 0) must be negative"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2976 in py::iter(&(py::slice(&v_flows.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2976;
out.push({ let t2977 = v_x.clone(); Py::Bool({ let t2978 = Py::Int(0i64); if py::gt(&t2977, &t2978)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the payback period is defined here for cash flows after time 0 that are all positive"))?;
    v_cum = py::getitem(&v_flows.clone(), &Py::Int(0i64))?;
    for t2979 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_flows.clone())?).i()?]) {
        v_t = t2979;
        let t2980 = (Py::tuple(vec![v_cum.clone(), py::add(&v_cum.clone(), &py::getitem(&v_flows.clone(), &v_t.clone())?)?])).items()?;
        if t2980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_prev = t2980[0].clone();
        v_cum = t2980[1].clone();
        if ({ let t2981 = v_cum.clone(); Py::Bool({ let t2982 = Py::Int(0i64); if py::ge(&t2981, &t2982)? { true } else { false } }) }).truthy() {
            return Ok(py::add(&py::sub(&v_t.clone(), &Py::Int(1i64))?, &py::truediv(&py::neg(&v_prev.clone())?, &py::getitem(&v_flows.clone(), &v_t.clone())?)?)?);
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the investment is not recovered within the cash flows given"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__periods_of(mut v_years: Py, mut v_per_year: Py, mut v_name: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__need({ let t2379 = v_years.clone(); Py::Bool({ let t2380 = Py::Int(1000i64); if py::le(&t2379, &t2380)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most 1000".to_string()].concat()))?;
    v_n = py::mul(&v_years.clone(), &v_per_year.clone())?;
    let _ = f__need({ let t2385 = { let t2381 = py::abs(&py::sub(&v_n.clone(), &py::round(&v_n.clone(), None)?)?)?; Py::Bool({ let t2382 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_n.clone())?], true)?)?; if py::le(&t2381, &t2382)? { true } else { false } }) }; if !t2385.truthy() { t2385 } else { { let t2383 = py::round(&v_n.clone(), None)?; Py::Bool({ let t2384 = Py::Int(1i64); if py::ge(&t2383, &t2384)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " times the number of payments per year must be a whole number of periods (at least 1)".to_string()].concat()))?;
    return Ok(py::int(&py::round(&v_n.clone(), None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phi(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_x.clone())?, &v_x.clone())?, &Py::Int(2i64))?)?, &py::math1("sqrt", &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_v: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t4150 = v_v.clone(); Py::Bool({ let t4151 = Py::Int(0i64); if py::gt(&t4150, &t4151)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prob(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t4088 = v_lo.clone(); Py::Bool({ let t4089 = v_v.clone(); if py::le(&t4088, &t4089)? { { let t4090 = v_hi.clone(); if py::le(&t4089, &t4090)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ps(mut v_c: Py, mut v_d: Py, mut v_P: Py) -> R {
    return Ok((if ({ let t3018 = v_c.clone(); Py::Bool({ let t3019 = Py::Int(0i64); if py::ge(&t3018, &t3019)? { true } else { false } }) }).truthy() { py::add(&py::mul(&v_c.clone(), &v_P.clone())?, &py::truediv(&py::mul(&py::mul(&v_d.clone(), &v_P.clone())?, &v_P.clone())?, &Py::Int(2i64))?)? } else { py::truediv(&py::mul(&v_d.clone(), &py::pow(&py::add(&v_P.clone(), &py::truediv(&v_c.clone(), &v_d.clone())?)?, &Py::Int(2i64))?)?, &Py::Int(2i64))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qtab(mut v_q: Py, mut v_terminal: Py) -> R {
    v_q = f__vec(Py::str("mortality_rates"), v_q.clone(), Py::Int(1i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4115 in py::iter(&(v_q.clone()))? { let mut v_x: Py; 
v_x = t4115;
out.push({ let t4116 = Py::Int(0i64); Py::Bool({ let t4117 = v_x.clone(); if py::le(&t4116, &t4117)? { { let t4118 = Py::Int(1i64); if py::le(&t4117, &t4118)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("each mortality rate q is a probability between 0 and 1"))?;
    let _ = f__need({ let t4121 = py::not(&v_terminal.clone()); if t4121.truthy() { t4121 } else { { let t4119 = py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t4120 = Py::Int(1i64); if py::eq(&t4119, &t4120) { true } else { false } }) } } }, Py::str("the table must end with q = 1 (no one survives past its last age)"))?;
    return Ok(v_q.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quad(mut v_w: Py, mut v_S: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3803 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3803;
for t3804 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3804;
out.push(py::mul(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_S.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_w.clone(), &v_j.clone())?)?); }} Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rate(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t4152 = v_lo.clone(); Py::Bool({ let t4153 = v_v.clone(); if py::lt(&t4152, &t4153)? { { let t4154 = v_hi.clone(); if py::lt(&t4153, &t4154)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a decimal rate between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?, " (write 5% or 0.05)".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ratio_inputs(mut v_sales: Py, mut v_net_income: Py, mut v_total_assets: Py, mut v_total_equity: Py, mut v_current_assets: Py, mut v_current_liabilities: Py, mut v_inventory: Py) -> R {
    let mut v_CA: Py = Py::None;
    let mut v_CL: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_INV: Py = Py::None;
    let mut v_NI: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_TA: Py = Py::None;
    v_S = f__pos(Py::str("sales"), v_sales.clone())?;
    v_NI = f__fin(Py::str("net_income"), v_net_income.clone())?;
    v_TA = f__pos(Py::str("total_assets"), v_total_assets.clone())?;
    v_E = f__pos(Py::str("total_equity"), v_total_equity.clone())?;
    v_CA = f__nonneg(Py::str("current_assets"), v_current_assets.clone())?;
    v_CL = f__pos(Py::str("current_liabilities"), v_current_liabilities.clone())?;
    v_INV = f__nonneg(Py::str("inventory"), v_inventory.clone())?;
    let _ = f__need({ let t1306 = v_E.clone(); Py::Bool({ let t1307 = v_TA.clone(); if py::le(&t1306, &t1307)? { true } else { false } }) }, Py::str("total equity cannot exceed total assets (the liabilities would be negative)"))?;
    let _ = f__need({ let t1308 = v_CA.clone(); Py::Bool({ let t1309 = v_TA.clone(); if py::le(&t1308, &t1309)? { true } else { false } }) }, Py::str("current assets cannot exceed total assets"))?;
    let _ = f__need({ let t1310 = v_INV.clone(); Py::Bool({ let t1311 = v_CA.clone(); if py::le(&t1310, &t1311)? { true } else { false } }) }, Py::str("inventory is part of current assets and cannot exceed them"))?;
    let _ = f__need({ let t1312 = py::add(&v_CL.clone(), &v_E.clone())?; Py::Bool({ let t1313 = py::mul(&v_TA.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t1312, &t1313)? { true } else { false } }) }, Py::str("current liabilities cannot exceed total liabilities (total assets - equity)"))?;
    return Ok(Py::tuple(vec![v_S.clone(), v_NI.clone(), v_TA.clone(), v_E.clone(), v_CA.clone(), v_CL.clone(), v_INV.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__residual(mut v_got: Py, mut v_want: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_worst = Py::Float(0.0);
    for t4163 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4178 = (t4163).items()?;
        if t4178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4178[0].clone();
        v_w = t4178[1].clone();
        if ({ let t4164 = v_k.clone(); Py::Bool({ let t4165 = v_got.clone(); if !py::contains(&t4165, &t4164)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_o = py::getitem(&v_got.clone(), &v_k.clone())?;
        if (Py::Bool(matches!(v_w.clone(), Py::Str(_)))).truthy() {
            if ({ let t4166 = v_o.clone(); Py::Bool({ let t4167 = v_w.clone(); if py::ne(&t4166, &t4167) { true } else { false } }) }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            continue;
        }
        v_pairs = (if (Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_)))).truthy() { Py::list((py::zip(&[v_o.clone(), v_w.clone()])?).items()?) } else { Py::list(vec![Py::tuple(vec![v_o.clone(), v_w.clone()])]) });
        if ({ let t4171 = Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_))); if !t4171.truthy() { t4171 } else { { let t4170 = py::not(&Py::Bool(matches!(v_o.clone(), Py::List(_) | Py::Tuple(_)))); if t4170.truthy() { t4170 } else { { let t4168 = py::len(&v_o.clone())?; Py::Bool({ let t4169 = py::len(&v_w.clone())?; if py::ne(&t4168, &t4169) { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        for t4172 in py::iter(&(v_pairs.clone()))? {
            let t4177 = (t4172).items()?;
            if t4177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4177[0].clone();
            v_b = t4177[1].clone();
            if ({ let t4174 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t4174.truthy() { t4174 } else { { let t4173 = py::not(&Py::Bool(matches!(v_a.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t4173.truthy() { t4173 } else { py::not(&py::math1("isfinite", &v_a.clone())?) } } } }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
            v_worst = py::minmax(&[v_worst.clone(), (if ({ let t4175 = v_d.clone(); Py::Bool({ let t4176 = Py::Float(1e-15); if py::le(&t4175, &t4176)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? })], true)?;
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__returns(mut v_name: Py, mut v_xs: Py, mut v_n_min: Py, mut v_hi: Py) -> R {
    v_xs = f__vec(v_name.clone(), v_xs.clone(), v_n_min.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3975 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3975;
out.push({ let t3976 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t3977 = v_x.clone(); if py::lt(&t3976, &t3977)? { { let t3978 = v_hi.clone(); if py::lt(&t3977, &t3978)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, ": each return is a decimal between -100% and ".to_string(), py::format(&v_hi.clone(), ".0%")?, " (write 5% or 0.05)".to_string()].concat()))?;
    return Ok(v_xs.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sd(mut v_xs: Py, mut v_ddof: Py) -> R {
    return Ok(py::math1("sqrt", &f__cov(v_xs.clone(), v_xs.clone(), v_ddof.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_changes(mut v_cfs: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = { let mut out = vec![]; for t2074 in py::iter(&(v_cfs.clone()))? { let mut v_c: Py; 
v_c = t2074;
if ({ let t2075 = v_c.clone(); Py::Bool({ let t2076 = Py::Int(0i64); if py::ne(&t2075, &t2076) { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    return Ok(py::sum(&{ let mut out = vec![]; for t2077 in py::iter(&(py::zip(&[v_s.clone(), py::slice(&v_s.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2078 = (t2077).items()?;
if t2078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2078[0].clone();
v_b = t2078[1].clone();
if ({ let t2079 = { let t2081 = v_a.clone(); Py::Bool({ let t2082 = Py::Int(0i64); if py::gt(&t2081, &t2082)? { true } else { false } }) }; Py::Bool({ let t2080 = { let t2083 = v_b.clone(); Py::Bool({ let t2084 = Py::Int(0i64); if py::gt(&t2083, &t2084)? { true } else { false } }) }; if py::ne(&t2079, &t2080) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_v: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t2917 = py::abs(&v_v.clone())?; Py::Bool({ let t2918 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2917, &t2918)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__square(mut v_name: Py, mut v_A: Py, mut v_n: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let _ = f__need({ let t3757 = Py::Bool(matches!(v_A.clone(), Py::List(_) | Py::Tuple(_))); if !t3757.truthy() { t3757 } else { { let t3756 = { let t3753 = py::len(&v_A.clone())?; Py::Bool({ let t3754 = Py::Int(1i64); if py::ge(&t3753, &t3754)? { true } else { false } }) }; if !t3756.truthy() { t3756 } else { Py::Bool(({ let mut out = vec![]; for t3755 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3755;
out.push(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_)))); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix".to_string()].concat()))?;
    v_k = py::len(&v_A.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3758 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3758;
out.push({ let t3759 = py::len(&v_r.clone())?; Py::Bool({ let t3760 = v_k.clone(); if py::eq(&t3759, &t3760) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be square".to_string()].concat()))?;
    let _ = f__need({ let t3765 = { let t3761 = v_n.clone(); Py::Bool({ let t3762 = Py::None; if py::is(&t3761, &t3762) { true } else { false } }) }; if t3765.truthy() { t3765 } else { { let t3763 = v_k.clone(); Py::Bool({ let t3764 = v_n.clone(); if py::eq(&t3763, &t3764) { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be ".to_string(), py::format(&v_n.clone(), "")?, " x ".to_string(), py::format(&v_n.clone(), "")?, " (one row and column per asset)".to_string()].concat()))?;
    v_M = { let mut out = vec![]; for t3766 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3766;
out.push({ let mut out = vec![]; for t3767 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3767;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_scale = { let t3770 = py::minmax(&[{ let mut out = vec![]; for t3768 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3768;
for t3769 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3769;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t3770.truthy() { t3770 } else { Py::Float(1.0) } };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3771 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t3771;
for t3772 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3772;
out.push({ let t3773 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_j.clone())?, &v_i.clone())?)?)?; Py::Bool({ let t3774 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t3773, &t3774)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3775 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t3775;
out.push({ let t3776 = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?; Py::Bool({ let t3777 = Py::Int(0i64); if py::ge(&t3776, &t3777)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, ": variances on the diagonal must not be negative".to_string()].concat()))?;
    return Ok(v_M.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__surv(mut v_q: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_p = Py::list(vec![Py::Float(1.0)]);
    for t4128 in py::iter(&(v_q.clone()))? {
        v_x = t4128;
        let _ = py::method(&v_p.clone(), "append", &[py::mul(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?])?;
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tax_inputs(mut v_taxable_income: Py, mut v_bracket_thresholds: Py, mut v_bracket_rates: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__pos(Py::str("taxable_income"), v_taxable_income.clone())?;
    v_t = f__vec(Py::str("bracket_thresholds"), v_bracket_thresholds.clone(), Py::Int(1i64))?;
    v_rs = f__vec(Py::str("bracket_rates"), v_bracket_rates.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t1920 = py::len(&v_t.clone())?; Py::Bool({ let t1921 = py::len(&v_rs.clone())?; if py::eq(&t1920, &t1921) { true } else { false } }) }, Py::str("bracket thresholds and bracket rates need the same number of entries"))?;
    let _ = f__need({ let t1922 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t1923 = Py::Int(0i64); if py::eq(&t1922, &t1923) { true } else { false } }) }, Py::str("the first bracket must start at 0"))?;
    for t1924 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_t.clone())?).i()?]) {
        v_k = t1924;
        let _ = f__need({ let t1925 = py::getitem(&v_t.clone(), &v_k.clone())?; Py::Bool({ let t1926 = py::getitem(&v_t.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t1925, &t1926)? { true } else { false } }) }, Py::str("bracket thresholds must be strictly increasing"))?;
    }
    for t1927 in py::iter(&(v_rs.clone()))? {
        v_v = t1927;
        let _ = f__need({ let t1928 = Py::Int(0i64); Py::Bool({ let t1929 = v_v.clone(); if py::le(&t1928, &t1929)? { { let t1930 = Py::Int(1i64); if py::lt(&t1929, &t1930)? { true } else { false } } } else { false } }) }, Py::str("each bracket rate is a decimal from 0 to below 1 (write 22% or 0.22)"))?;
    }
    return Ok(Py::tuple(vec![v_x.clone(), v_t.clone(), v_rs.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__timing(mut v_t: Py) -> R {
    let _ = f__need({ let t3539 = v_t.clone(); Py::Bool({ let t3540 = Py::tuple(vec![Py::str("ordinary"), Py::str("due")]); if py::contains(&t3540, &t3539)? { true } else { false } }) }, Py::str("timing is 'ordinary' (end of period) or 'due' (beginning of period)"))?;
    return Ok((if ({ let t3541 = v_t.clone(); Py::Bool({ let t3542 = Py::str("due"); if py::eq(&t3541, &t3542) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__v(mut v_i: Py) -> R {
    return Ok(py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &f__rate(Py::str("interest_rate"), v_i.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_name: Py, mut v_xs: Py, mut v_n_min: Py) -> R {
    let _ = f__need({ let t4111 = Py::Bool(matches!(v_xs.clone(), Py::List(_) | Py::Tuple(_))); if !t4111.truthy() { t4111 } else { { let t4109 = py::len(&v_xs.clone())?; Py::Bool({ let t4110 = v_n_min.clone(); if py::ge(&t4109, &t4110)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " values".to_string()].concat()))?;
    return Ok({ let mut out = vec![]; for t4112 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t4112;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_crossover(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = py::method(&v_outputs.clone(), "get", &[Py::str("crossover_rate")])?;
    if ({ let t1048 = py::not(&Py::Bool(matches!(v_r.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1048.truthy() { t1048 } else { { let t1047 = py::not(&py::math1("isfinite", &v_r.clone())?); if t1047.truthy() { t1047 } else { { let t1045 = v_r.clone(); Py::Bool({ let t1046 = py::neg(&Py::Int(1i64))?; if py::le(&t1045, &t1046)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t1049 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("cash_flows_a"))?, py::getitem(&v_inputs.clone(), &Py::str("cash_flows_b"))?])).items()?;
    if t1049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1049[0].clone();
    v_b = t1049[1].clone();
    v_d = { let mut out = vec![]; for t1050 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1051 = (t1050).items()?;
if t1051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1051[0].clone();
v_y = t1051[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    return Ok(py::minmax(&[py::truediv(&py::abs(&f__hnpv(v_r.clone(), v_d.clone())?)?, &f__npv_scale(v_r.clone(), v_d.clone())?)?, py::truediv(&py::abs(&py::sub(&py::method(&v_outputs.clone(), "get", &[Py::str("npv_at_crossover"), Py::Float(f64::INFINITY)])?, &f__hnpv(v_r.clone(), v_a.clone())?)?)?, &py::minmax(&[Py::Float(1.0), f__npv_scale(v_r.clone(), v_a.clone())?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_irr(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = py::method(&v_outputs.clone(), "get", &[Py::str("irr")])?;
    if ({ let t2133 = py::not(&Py::Bool(matches!(v_r.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t2133.truthy() { t2133 } else { { let t2132 = py::not(&py::math1("isfinite", &v_r.clone())?); if t2132.truthy() { t2132 } else { { let t2130 = v_r.clone(); Py::Bool({ let t2131 = py::neg(&Py::Int(1i64))?; if py::le(&t2130, &t2131)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_c = py::getitem(&v_inputs.clone(), &Py::str("cash_flows"))?;
    return Ok(py::truediv(&py::abs(&f__hnpv(v_r.clone(), v_c.clone())?)?, &f__npv_scale(v_r.clone(), v_c.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_iv(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_s = py::method(&v_outputs.clone(), "get", &[Py::str("volatility")])?;
    if ({ let t1876 = py::not(&Py::Bool(matches!(v_s.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1876.truthy() { t1876 } else { { let t1875 = py::not(&py::math1("isfinite", &v_s.clone())?); if t1875.truthy() { t1875 } else { { let t1873 = v_s.clone(); Py::Bool({ let t1874 = Py::Int(0i64); if py::le(&t1873, &t1874)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t1877 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("spot"))?, py::getitem(&v_inputs.clone(), &Py::str("strike"))?, py::getitem(&v_inputs.clone(), &Py::str("time"))?, py::getitem(&v_inputs.clone(), &Py::str("rate"))?])).items()?;
    if t1877.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1877[0].clone();
    v_K = t1877[1].clone();
    v_T = t1877[2].clone();
    v_r = t1877[3].clone();
    v_q = py::method(&v_inputs.clone(), "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?;
    v_v = py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone())?, &(if ({ let t1878 = py::method(&v_inputs.clone(), "get", &[Py::str("option_type"), Py::str("call")])?; Py::Bool({ let t1879 = Py::str("call"); if py::eq(&t1878, &t1879) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(3i64) }))?;
    return Ok(py::truediv(&py::abs(&py::sub(&v_v.clone(), &py::getitem(&v_inputs.clone(), &Py::str("option_price"))?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_inputs.clone(), &Py::str("option_price"))?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ytm(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_y = py::method(&v_outputs.clone(), "get", &[Py::str("ytm")])?;
    if ({ let t487 = py::not(&Py::Bool(matches!(v_y.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t487.truthy() { t487 } else { py::not(&py::math1("isfinite", &v_y.clone())?) } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t488 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("face"))?, py::getitem(&v_inputs.clone(), &Py::str("coupon_rate"))?, py::method(&v_inputs.clone(), "get", &[Py::str("frequency"), Py::Int(2i64)])?])).items()?;
    if t488.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t488[0].clone();
    v_c = t488[1].clone();
    v_f = t488[2].clone();
    v_N = py::round(&py::mul(&py::getitem(&v_inputs.clone(), &Py::str("years"))?, &v_f.clone())?, None)?;
    v_P = py::getitem(&v_inputs.clone(), &Py::str("price"))?;
    let t489: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r1 = py::truediv(&py::abs(&py::sub(&f__bprice(v_F.clone(), py::truediv(&py::mul(&v_F.clone(), &v_c.clone())?, &v_f.clone())?, v_N.clone(), py::truediv(&v_y.clone(), &v_f.clone())?)?, &v_P.clone())?)?, &py::minmax(&[Py::Float(1.0), v_P.clone()], true)?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t489 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Overflow)) || (matches!(e, PyErr::ZeroDivision)) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    return Ok(py::minmax(&[v_r1.clone(), f__residual(v_outputs.clone(), Py::dict(vec![(Py::str("current_yield"), py::truediv(&py::mul(&v_F.clone(), &v_c.clone())?, &v_P.clone())?)]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__who(mut v_a: Py, mut v_b: Py, mut v_more_is_better: Py) -> R {
    if ({ let t652 = v_a.clone(); Py::Bool({ let t653 = v_b.clone(); if py::eq(&t652, &t653) { true } else { false } }) }).truthy() {
        return Ok(Py::str("equal"));
    }
    if ({ let t654 = { let t656 = v_a.clone(); Py::Bool({ let t657 = v_b.clone(); if py::gt(&t656, &t657)? { true } else { false } }) }; Py::Bool({ let t655 = v_more_is_better.clone(); if py::eq(&t654, &t655) { true } else { false } }) }).truthy() {
        return Ok(Py::str("country A"));
    }
    return Ok(Py::str("country B"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_annuity_rate(mut v_present_value: Py, mut v_payment: Py, mut v_periods: Py, mut v_timing: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_due: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let t21 = (Py::tuple(vec![f__pos(Py::str("present_value"), v_present_value.clone())?, f__pos(Py::str("payment"), v_payment.clone())?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t21.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t21[0].clone();
    v_p = t21[1].clone();
    v_n = t21[2].clone();
    v_due = t21[3].clone();
    v_f = { let v_due = v_due.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); let v_p = v_p.clone(); let v_payment = v_payment.clone(); let v_periods = v_periods.clone(); let v_present_value = v_present_value.clone(); let v_pv = v_pv.clone(); let v_timing = v_timing.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let t24: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    return Ok(Some(py::sub(&f__annuity_pv(v_p.clone(), v_r.clone(), v_n.clone(), v_due.clone())?, &v_pv.clone())?));
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t24 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Overflow)) => {
                    return Ok(Py::Float(f64::INFINITY));
                }
                Err(e) => return Err(e),
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t25 = (Py::tuple(vec![py::neg(&Py::Float(0.99))?, Py::Float(0.99)])).items()?;
    if t25.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t25[0].clone();
    v_hi = t25[1].clone();
    let _ = f__need({ let t26 = py::call(&v_f.clone(), &[v_lo.clone()])?; Py::Bool({ let t27 = Py::Int(0i64); if py::gt(&t26, &t27)? { { let t28 = py::call(&v_f.clone(), &[v_hi.clone()])?; if py::gt(&t27, &t28)? { true } else { false } } } else { false } }) }, Py::str("no rate between -99% and 99% per period gives this present value"))?;
    for t29 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t29;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t30 = v_mid.clone(); Py::Bool({ let t31 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t31, &t30)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t32 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t33 = Py::Int(0i64); if py::gt(&t32, &t33)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("rate"), py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arc_price_elasticity(mut v_price_1: Py, mut v_quantity_1: Py, mut v_price_2: Py, mut v_quantity_2: Py) -> R {
    let mut v_cls: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t59 = (f__arc(v_price_1.clone(), v_quantity_1.clone(), v_price_2.clone(), v_quantity_2.clone(), Py::str("price"), Py::str("quantity"))?).items()?;
    if t59.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t59[0].clone();
    v_ex = t59[1].clone();
    v_cls = (if ({ let t60 = v_ex.clone(); Py::Bool({ let t61 = Py::Int(0i64); if py::eq(&t60, &t61) { true } else { false } }) }).truthy() { Py::str("perfectly inelastic") } else { (if ({ let t62 = py::abs(&v_ex.clone())?; Py::Bool({ let t63 = Py::Int(1i64); if py::eq(&t62, &t63) { true } else { false } }) }).truthy() { Py::str("unit elastic") } else { (if ({ let t64 = py::abs(&v_ex.clone())?; Py::Bool({ let t65 = Py::Int(1i64); if py::gt(&t64, &t65)? { true } else { false } }) }).truthy() { Py::str("elastic") } else { Py::str("inelastic") }) }) });
    return Ok(Py::dict(vec![(Py::str("elasticity"), v_e.clone()), (Py::str("abs_elasticity"), py::abs(&v_e.clone())?), (Py::str("classification"), v_cls.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beta_regression(mut v_asset_returns: Py, mut v_market_returns: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t97 = (f__pair(v_asset_returns.clone(), v_market_returns.clone(), Py::str("asset_returns"), Py::str("market_returns"), Py::Int(3i64))?).items()?;
    if t97.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_y = t97[0].clone();
    v_x = t97[1].clone();
    let t101 = (Py::tuple(vec![f__cov(v_x.clone(), v_x.clone(), Py::Int(1i64))?, f__cov(v_y.clone(), v_y.clone(), Py::Int(1i64))?, f__cov(v_x.clone(), v_y.clone(), Py::Int(1i64))?])).items()?;
    if t101.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t101[0].clone();
    v_syy = t101[1].clone();
    v_sxy = t101[2].clone();
    let _ = f__need({ let t102 = v_sxx.clone(); Py::Bool({ let t103 = Py::Int(0i64); if py::gt(&t102, &t103)? { true } else { false } }) }, Py::str("the market returns must not all be equal"))?;
    let _ = f__need({ let t104 = v_syy.clone(); Py::Bool({ let t105 = Py::Int(0i64); if py::gt(&t104, &t105)? { true } else { false } }) }, Py::str("the asset returns must not all be equal"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::dict(vec![(Py::str("beta"), v_b.clone()), (Py::str("alpha"), py::sub(&f__mean(v_y.clone())?, &py::mul(&v_b.clone(), &f__mean(v_x.clone())?)?)?), (Py::str("correlation"), py::truediv(&v_sxy.clone(), &py::math1("sqrt", &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?), (Py::str("r_squared"), py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binomial_option(mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_volatility: Py, mut v_steps: Py, mut v_option_type: Py, mut v_dividend_yield: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pay: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vd: Py = Py::None;
    let mut v_vu: Py = Py::None;
    let t135 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), v_volatility.clone(), v_dividend_yield.clone())?).items()?;
    if t135.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t135[0].clone();
    v_K = t135[1].clone();
    v_T = t135[2].clone();
    v_r = t135[3].clone();
    v_s = t135[4].clone();
    v_q = t135[5].clone();
    v_n = f__int(Py::str("steps"), v_steps.clone(), Py::Int(1i64), Py::Int(1000i64))?;
    let _ = f__need({ let t148 = v_option_type.clone(); Py::Bool({ let t149 = Py::tuple(vec![Py::str("call"), Py::str("put")]); if py::contains(&t149, &t148)? { true } else { false } }) }, Py::str("option_type is call or put"))?;
    v_dt = py::truediv(&v_T.clone(), &v_n.clone())?;
    v_u = py::math1("exp", &py::mul(&v_s.clone(), &py::math1("sqrt", &v_dt.clone())?)?)?;
    v_d = py::truediv(&Py::Int(1i64), &v_u.clone())?;
    v_p = py::truediv(&py::sub(&py::math1("exp", &py::mul(&py::sub(&v_r.clone(), &v_q.clone())?, &v_dt.clone())?)?, &v_d.clone())?, &py::sub(&v_u.clone(), &v_d.clone())?)?;
    let _ = f__need({ let t150 = Py::Int(0i64); Py::Bool({ let t151 = v_p.clone(); if py::lt(&t150, &t151)? { { let t152 = Py::Int(1i64); if py::lt(&t151, &t152)? { true } else { false } } } else { false } }) }, Py::str("the risk-neutral probability is outside 0..1: use more steps"))?;
    v_disc = py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_dt.clone())?)?;
    v_pay = (if ({ let t153 = v_option_type.clone(); Py::Bool({ let t154 = Py::str("call"); if py::eq(&t153, &t154) { true } else { false } }) }).truthy() { { let v_K = v_K.clone(); let v_S = v_S.clone(); let v_T = v_T.clone(); let v_d = v_d.clone(); let v_disc = v_disc.clone(); let v_dividend_yield = v_dividend_yield.clone(); let v_dt = v_dt.clone(); let v_n = v_n.clone(); let v_option_type = v_option_type.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_rate = v_rate.clone(); let v_s = v_s.clone(); let v_spot = v_spot.clone(); let v_steps = v_steps.clone(); let v_strike = v_strike.clone(); let v_time = v_time.clone(); let v_u = v_u.clone(); let v_volatility = v_volatility.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[py::sub(&v_x.clone(), &v_K.clone())?, Py::Float(0.0)], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) } } else { { let v_K = v_K.clone(); let v_S = v_S.clone(); let v_T = v_T.clone(); let v_d = v_d.clone(); let v_disc = v_disc.clone(); let v_dividend_yield = v_dividend_yield.clone(); let v_dt = v_dt.clone(); let v_n = v_n.clone(); let v_option_type = v_option_type.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_rate = v_rate.clone(); let v_s = v_s.clone(); let v_spot = v_spot.clone(); let v_steps = v_steps.clone(); let v_strike = v_strike.clone(); let v_time = v_time.clone(); let v_u = v_u.clone(); let v_volatility = v_volatility.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[py::sub(&v_K.clone(), &v_x.clone())?, Py::Float(0.0)], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) } });
    v_v = { let mut out = vec![]; for t155 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t155;
out.push(py::call(&v_pay.clone(), &[py::mul(&py::mul(&v_S.clone(), &py::pow(&v_u.clone(), &v_j.clone())?)?, &py::pow(&v_d.clone(), &py::sub(&v_n.clone(), &v_j.clone())?)?)?])?); } Py::list(out) };
    let t156 = Py::None;
    v_vu = t156.clone();
    v_vd = t156.clone();
    for t157 in py::range_iter(&[(v_n.clone()).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_step = t157;
        if ({ let t158 = v_step.clone(); Py::Bool({ let t159 = Py::Int(1i64); if py::eq(&t158, &t159) { true } else { false } }) }).truthy() {
            let t160 = (Py::tuple(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?])).items()?;
            if t160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_vd = t160[0].clone();
            v_vu = t160[1].clone();
        }
        v_v = { let mut out = vec![]; for t161 in py::range_iter(&[(v_step.clone()).i()?]) { let mut v_j: Py; 
v_j = t161;
out.push(py::mul(&v_disc.clone(), &py::add(&py::mul(&v_p.clone(), &py::getitem(&v_v.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &py::getitem(&v_v.clone(), &v_j.clone())?)?)?)?); } Py::list(out) };
    }
    return Ok(Py::dict(vec![(Py::str("price"), py::getitem(&v_v.clone(), &Py::Int(0i64))?), (Py::str("delta"), py::truediv(&py::sub(&v_vu.clone(), &v_vd.clone())?, &py::sub(&py::mul(&v_S.clone(), &v_u.clone())?, &py::mul(&v_S.clone(), &v_d.clone())?)?)?), (Py::str("up_factor"), v_u.clone()), (Py::str("probability_up"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_black_scholes(mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_volatility: Py, mut v_dividend_yield: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t192 = ({ let mut v: Vec<Py> = vec![]; v.extend((f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), v_volatility.clone(), v_dividend_yield.clone())?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "S") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument S".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "K") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument K".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "T") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument T".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "r") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument r".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "s") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument s".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "q") { Some(x) => x, None => { return Err(PyErr::Type("_bs() missing argument q".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("_bs() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_bs() got an unexpected keyword argument".into())) } f__bs(a0, a1, a2, a3, a4, a5)? }).items()?;
    if t192.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t192[0].clone();
    v_d2 = t192[1].clone();
    v_c = t192[2].clone();
    v_p = t192[3].clone();
    return Ok(Py::dict(vec![(Py::str("call"), v_c.clone()), (Py::str("put"), v_p.clone()), (Py::str("d1"), v_d1.clone()), (Py::str("d2"), v_d2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_convexity(mut v_face: Py, mut v_coupon_rate: Py, mut v_ytm: Py, mut v_years: Py, mut v_frequency: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_conv: Py = Py::None;
    let mut v_cpn: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t411 = (f__bond(v_face.clone(), v_coupon_rate.clone(), v_years.clone(), v_frequency.clone())?).items()?;
    if t411.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_F = t411[0].clone();
    v_c = t411[1].clone();
    v_f = t411[2].clone();
    v_N = t411[3].clone();
    v_cpn = t411[4].clone();
    v_y = f__rate(Py::str("ytm"), v_ytm.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_v = py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::truediv(&v_y.clone(), &v_f.clone())?)?)?;
    v_cf = py::mul(&Py::list(vec![v_cpn.clone()]), &v_N.clone())?;
    let t412 = v_cf.clone(); let t413 = py::neg(&Py::Int(1i64))?;
    py::setitem(&t412, &t413, py::iadd(&py::getitem(&t412, &t413)?, &v_F.clone())?)?;
    v_P = py::fsum(&{ let mut out = vec![]; for t414 in py::iter(&(py::enumerate(&v_cf.clone(), (Py::Int(1i64)).i()?)?))? { let mut v_k: Py; let mut v_x: Py; 
let t415 = (t414).items()?;
if t415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t415[0].clone();
v_x = t415[1].clone();
out.push(py::mul(&v_x.clone(), &py::pow(&v_v.clone(), &v_k.clone())?)?); } Py::list(out) })?;
    v_conv = py::truediv(&py::fsum(&{ let mut out = vec![]; for t416 in py::iter(&(py::enumerate(&v_cf.clone(), (Py::Int(1i64)).i()?)?))? { let mut v_k: Py; let mut v_x: Py; 
let t417 = (t416).items()?;
if t417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t417[0].clone();
v_x = t417[1].clone();
out.push(py::mul(&py::mul(&py::mul(&v_x.clone(), &v_k.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(2i64))?)?)?); } Py::list(out) })?, &py::mul(&py::mul(&v_f.clone(), &v_f.clone())?, &v_P.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("convexity"), v_conv.clone()), (Py::str("price"), v_P.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_duration(mut v_face: Py, mut v_coupon_rate: Py, mut v_ytm: Py, mut v_years: Py, mut v_frequency: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cpn: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_mac: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t402 = (f__bond(v_face.clone(), v_coupon_rate.clone(), v_years.clone(), v_frequency.clone())?).items()?;
    if t402.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_F = t402[0].clone();
    v_c = t402[1].clone();
    v_f = t402[2].clone();
    v_N = t402[3].clone();
    v_cpn = t402[4].clone();
    v_y = f__rate(Py::str("ytm"), v_ytm.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_v = py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::truediv(&v_y.clone(), &v_f.clone())?)?)?;
    v_pv = { let mut out = vec![]; for t406 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t406;
out.push(py::mul(&v_cpn.clone(), &py::pow(&v_v.clone(), &v_k.clone())?)?); } Py::list(out) };
    let t407 = v_pv.clone(); let t408 = py::neg(&Py::Int(1i64))?;
    py::setitem(&t407, &t408, py::iadd(&py::getitem(&t407, &t408)?, &py::mul(&v_F.clone(), &py::pow(&v_v.clone(), &v_N.clone())?)?)?)?;
    v_P = py::fsum(&v_pv.clone())?;
    v_mac = py::truediv(&py::fsum(&{ let mut out = vec![]; for t409 in py::iter(&(py::enumerate(&v_pv.clone(), (Py::Int(1i64)).i()?)?))? { let mut v_k: Py; let mut v_x: Py; 
let t410 = (t409).items()?;
if t410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t410[0].clone();
v_x = t410[1].clone();
out.push(py::mul(&py::truediv(&v_k.clone(), &v_f.clone())?, &v_x.clone())?); } Py::list(out) })?, &v_P.clone())?;
    return Ok(Py::dict(vec![(Py::str("macaulay_duration"), v_mac.clone()), (Py::str("modified_duration"), py::truediv(&v_mac.clone(), &py::add(&Py::Int(1i64), &py::truediv(&v_y.clone(), &v_f.clone())?)?)?), (Py::str("price"), v_P.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_price(mut v_face: Py, mut v_coupon_rate: Py, mut v_ytm: Py, mut v_years: Py, mut v_frequency: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cpn: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t425 = (f__bond(v_face.clone(), v_coupon_rate.clone(), v_years.clone(), v_frequency.clone())?).items()?;
    if t425.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_F = t425[0].clone();
    v_c = t425[1].clone();
    v_f = t425[2].clone();
    v_N = t425[3].clone();
    v_cpn = t425[4].clone();
    v_y = f__rate(Py::str("ytm"), v_ytm.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_p = f__bprice(v_F.clone(), v_cpn.clone(), v_N.clone(), py::truediv(&v_y.clone(), &v_f.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("price"), v_p.clone()), (Py::str("current_yield"), py::truediv(&py::mul(&v_F.clone(), &v_c.clone())?, &v_p.clone())?), (Py::str("coupon"), v_cpn.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_price_change(mut v_face: Py, mut v_coupon_rate: Py, mut v_ytm: Py, mut v_years: Py, mut v_yield_change: Py, mut v_frequency: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_new: Py = Py::None;
    v_d = f_bond_duration(v_face.clone(), v_coupon_rate.clone(), v_ytm.clone(), v_years.clone(), v_frequency.clone())?;
    v_cv = py::getitem(&f_bond_convexity(v_face.clone(), v_coupon_rate.clone(), v_ytm.clone(), v_years.clone(), v_frequency.clone())?, &Py::str("convexity"))?;
    v_dy = f__fin(Py::str("yield_change"), v_yield_change.clone())?;
    let _ = f__need({ let t423 = { let t418 = py::abs(&v_dy.clone())?; Py::Bool({ let t419 = Py::Float(0.5); if py::lt(&t418, &t419)? { true } else { false } }) }; if !t423.truthy() { t423 } else { { let t420 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t421 = py::add(&v_ytm.clone(), &v_dy.clone())?; if py::lt(&t420, &t421)? { { let t422 = Py::Int(1i64); if py::lt(&t421, &t422)? { true } else { false } } } else { false } }) } } }, Py::str("yield_change must keep the yield between -100% and 100% (and |change| < 50%)"))?;
    let t424 = (Py::tuple(vec![py::getitem(&v_d.clone(), &Py::str("price"))?, py::getitem(&v_d.clone(), &Py::str("modified_duration"))?])).items()?;
    if t424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t424[0].clone();
    v_D = t424[1].clone();
    v_new = py::getitem(&f_bond_price(v_face.clone(), v_coupon_rate.clone(), py::add(&v_ytm.clone(), &v_dy.clone())?, v_years.clone(), v_frequency.clone())?, &Py::str("price"))?;
    return Ok(Py::dict(vec![(Py::str("duration_estimate"), py::mul(&py::mul(&py::neg(&v_D.clone())?, &v_P.clone())?, &v_dy.clone())?), (Py::str("duration_convexity_estimate"), py::add(&py::mul(&py::mul(&py::neg(&v_D.clone())?, &v_P.clone())?, &v_dy.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_cv.clone())?, &v_P.clone())?, &v_dy.clone())?, &v_dy.clone())?)?), (Py::str("actual_change"), py::sub(&v_new.clone(), &v_P.clone())?), (Py::str("new_price"), v_new.clone()), (Py::str("price"), v_P.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_ytm(mut v_price: Py, mut v_face: Py, mut v_coupon_rate: Py, mut v_years: Py, mut v_frequency: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cpn: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let t474 = (f__bond(v_face.clone(), v_coupon_rate.clone(), v_years.clone(), v_frequency.clone())?).items()?;
    if t474.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_F = t474[0].clone();
    v_c = t474[1].clone();
    v_f = t474[2].clone();
    v_N = t474[3].clone();
    v_cpn = t474[4].clone();
    v_P = f__pos(Py::str("price"), v_price.clone())?;
    v_g = { let v_F = v_F.clone(); let v_N = v_N.clone(); let v_P = v_P.clone(); let v_c = v_c.clone(); let v_coupon_rate = v_coupon_rate.clone(); let v_cpn = v_cpn.clone(); let v_f = v_f.clone(); let v_face = v_face.clone(); let v_frequency = v_frequency.clone(); let v_g = v_g.clone(); let v_price = v_price.clone(); let v_years = v_years.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            let t477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    return Ok(Some(py::sub(&f__bprice(v_F.clone(), v_cpn.clone(), v_N.clone(), v_j.clone())?, &v_P.clone())?));
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t477 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Overflow)) => {
                    return Ok(Py::Float(f64::INFINITY));
                }
                Err(e) => return Err(e),
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t478 = (Py::tuple(vec![py::neg(&Py::Float(0.5))?, Py::Float(1.0)])).items()?;
    if t478.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t478[0].clone();
    v_hi = t478[1].clone();
    let _ = f__need({ let t479 = py::call(&v_g.clone(), &[v_lo.clone()])?; Py::Bool({ let t480 = Py::Int(0i64); if py::gt(&t479, &t480)? { { let t481 = py::call(&v_g.clone(), &[v_hi.clone()])?; if py::gt(&t480, &t481)? { true } else { false } } } else { false } }) }, Py::str("no yield between -50% and 100% per period gives this price"))?;
    for t482 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t482;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t483 = v_mid.clone(); Py::Bool({ let t484 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t484, &t483)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t485 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t486 = Py::Int(0i64); if py::gt(&t485, &t486)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_j = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("ytm"), py::mul(&v_j.clone(), &v_f.clone())?), (Py::str("current_yield"), py::truediv(&py::mul(&v_F.clone(), &v_c.clone())?, &v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_break_even(mut v_fixed_costs: Py, mut v_price: Py, mut v_variable_cost: Py, mut v_target_profit: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t512 = (Py::tuple(vec![f__nonneg(Py::str("fixed_costs"), v_fixed_costs.clone())?, f__pos(Py::str("price"), v_price.clone())?, f__nonneg(Py::str("variable_cost"), v_variable_cost.clone())?])).items()?;
    if t512.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t512[0].clone();
    v_p = t512[1].clone();
    v_v = t512[2].clone();
    v_T = f__fin(Py::str("target_profit"), v_target_profit.clone())?;
    let _ = f__need({ let t513 = v_p.clone(); Py::Bool({ let t514 = v_v.clone(); if py::gt(&t513, &t514)? { true } else { false } }) }, Py::str("the price must exceed the variable cost per unit (no positive contribution margin)"))?;
    let _ = f__need({ let t515 = py::add(&v_F.clone(), &v_T.clone())?; Py::Bool({ let t516 = Py::Int(0i64); if py::ge(&t515, &t516)? { true } else { false } }) }, Py::str("fixed costs plus the target profit must not be negative"))?;
    v_Q = py::truediv(&py::add(&v_F.clone(), &v_T.clone())?, &py::sub(&v_p.clone(), &v_v.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("quantity"), v_Q.clone()), (Py::str("revenue"), py::mul(&v_p.clone(), &v_Q.clone())?), (Py::str("contribution_margin"), py::sub(&v_p.clone(), &v_v.clone())?), (Py::str("contribution_margin_ratio"), py::truediv(&py::sub(&v_p.clone(), &v_v.clone())?, &v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bs_greeks(mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_volatility: Py, mut v_dividend_yield: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v__c: Py = Py::None;
    let mut v__p: Py = Py::None;
    let mut v_common: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_dq: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let t546 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), v_volatility.clone(), v_dividend_yield.clone())?).items()?;
    if t546.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t546[0].clone();
    v_K = t546[1].clone();
    v_T = t546[2].clone();
    v_r = t546[3].clone();
    v_s = t546[4].clone();
    v_q = t546[5].clone();
    let t548 = (f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone())?).items()?;
    if t548.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t548[0].clone();
    v_d2 = t548[1].clone();
    v__c = t548[2].clone();
    v__p = t548[3].clone();
    let t549 = (Py::tuple(vec![py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?, py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_T.clone())?)?, py::math1("sqrt", &v_T.clone())?])).items()?;
    if t549.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dq = t549[0].clone();
    v_dr = t549[1].clone();
    v_sq = t549[2].clone();
    v_common = py::truediv(&py::mul(&py::mul(&py::mul(&py::neg(&v_S.clone())?, &v_dq.clone())?, &f__phi(v_d1.clone())?)?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_sq.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("call_delta"), py::mul(&v_dq.clone(), &f__N(v_d1.clone())?)?), (Py::str("put_delta"), py::mul(&py::neg(&v_dq.clone())?, &f__N(py::neg(&v_d1.clone())?)?)?), (Py::str("gamma"), py::truediv(&py::mul(&v_dq.clone(), &f__phi(v_d1.clone())?)?, &py::mul(&py::mul(&v_S.clone(), &v_s.clone())?, &v_sq.clone())?)?), (Py::str("vega"), py::mul(&py::mul(&py::mul(&v_S.clone(), &v_dq.clone())?, &f__phi(v_d1.clone())?)?, &v_sq.clone())?), (Py::str("call_theta"), py::add(&py::sub(&v_common.clone(), &py::mul(&py::mul(&py::mul(&v_r.clone(), &v_K.clone())?, &v_dr.clone())?, &f__N(v_d2.clone())?)?)?, &py::mul(&py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_dq.clone())?, &f__N(v_d1.clone())?)?)?), (Py::str("put_theta"), py::sub(&py::add(&v_common.clone(), &py::mul(&py::mul(&py::mul(&v_r.clone(), &v_K.clone())?, &v_dr.clone())?, &f__N(py::neg(&v_d2.clone())?)?)?)?, &py::mul(&py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_dq.clone())?, &f__N(py::neg(&v_d1.clone())?)?)?)?), (Py::str("call_rho"), py::mul(&py::mul(&py::mul(&v_K.clone(), &v_T.clone())?, &v_dr.clone())?, &f__N(v_d2.clone())?)?), (Py::str("put_rho"), py::mul(&py::mul(&py::mul(&py::neg(&v_K.clone())?, &v_T.clone())?, &v_dr.clone())?, &f__N(py::neg(&v_d2.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cagr(mut v_begin_value: Py, mut v_end_value: Py, mut v_years: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t572 = (Py::tuple(vec![f__pos(Py::str("begin_value"), v_begin_value.clone())?, f__pos(Py::str("end_value"), v_end_value.clone())?, f__pos(Py::str("years"), v_years.clone())?])).items()?;
    if t572.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t572[0].clone();
    v_e = t572[1].clone();
    v_t = t572[2].clone();
    return Ok(Py::dict(vec![(Py::str("cagr"), py::sub(&py::pow(&py::truediv(&v_e.clone(), &v_b.clone())?, &py::truediv(&Py::Int(1i64), &v_t.clone())?)?, &Py::Int(1i64))?), (Py::str("total_growth"), py::sub(&py::truediv(&v_e.clone(), &v_b.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capm(mut v_risk_free: Py, mut v_beta: Py, mut v_market_return: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_rf: Py = Py::None;
    let mut v_rm: Py = Py::None;
    let t596 = (Py::tuple(vec![f__rate(Py::str("risk_free"), v_risk_free.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__fin(Py::str("beta"), v_beta.clone())?, f__rate(Py::str("market_return"), v_market_return.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t596.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_rf = t596[0].clone();
    v_b = t596[1].clone();
    v_rm = t596[2].clone();
    return Ok(Py::dict(vec![(Py::str("expected_return"), py::add(&v_rf.clone(), &py::mul(&v_b.clone(), &py::sub(&v_rm.clone(), &v_rf.clone())?)?)?), (Py::str("market_risk_premium"), py::sub(&v_rm.clone(), &v_rf.clone())?), (Py::str("risk_premium"), py::mul(&v_b.clone(), &py::sub(&v_rm.clone(), &v_rf.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cobb_douglas_demand(mut v_alpha: Py, mut v_beta: Py, mut v_income: Py, mut v_price_x: Py, mut v_price_y: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t619 = (Py::tuple(vec![f__pos(Py::str("alpha"), v_alpha.clone())?, f__pos(Py::str("beta"), v_beta.clone())?, f__pos(Py::str("income"), v_income.clone())?])).items()?;
    if t619.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t619[0].clone();
    v_b = t619[1].clone();
    v_M = t619[2].clone();
    let t620 = (Py::tuple(vec![f__pos(Py::str("price_x"), v_price_x.clone())?, f__pos(Py::str("price_y"), v_price_y.clone())?])).items()?;
    if t620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_px = t620[0].clone();
    v_py = t620[1].clone();
    let t621 = (Py::tuple(vec![py::truediv(&py::mul(&py::truediv(&v_a.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?, &v_M.clone())?, &v_px.clone())?, py::truediv(&py::mul(&py::truediv(&v_b.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?, &v_M.clone())?, &v_py.clone())?])).items()?;
    if t621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t621[0].clone();
    v_y = t621[1].clone();
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone()), (Py::str("utility"), py::mul(&py::pow(&v_x.clone(), &v_a.clone())?, &py::pow(&v_y.clone(), &v_b.clone())?)?), (Py::str("share_x"), py::truediv(&v_a.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_comparative_advantage(mut v_a_good_x: Py, mut v_a_good_y: Py, mut v_b_good_x: Py, mut v_b_good_y: Py, mut v_basis: Py) -> R {
    let mut v_abs_x: Py = Py::None;
    let mut v_abs_y: Py = Py::None;
    let mut v_ax: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cross_a: Py = Py::None;
    let mut v_cross_b: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_oxa: Py = Py::None;
    let mut v_oxb: Py = Py::None;
    let mut v_oya: Py = Py::None;
    let mut v_oyb: Py = Py::None;
    let t644 = (Py::tuple(vec![f__pos(Py::str("a_good_x"), v_a_good_x.clone())?, f__pos(Py::str("a_good_y"), v_a_good_y.clone())?])).items()?;
    if t644.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ax = t644[0].clone();
    v_ay = t644[1].clone();
    let t645 = (Py::tuple(vec![f__pos(Py::str("b_good_x"), v_b_good_x.clone())?, f__pos(Py::str("b_good_y"), v_b_good_y.clone())?])).items()?;
    if t645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t645[0].clone();
    v_by = t645[1].clone();
    let _ = f__need({ let t646 = v_basis.clone(); Py::Bool({ let t647 = Py::tuple(vec![Py::str("output"), Py::str("hours")]); if py::contains(&t647, &t646)? { true } else { false } }) }, Py::str("basis is 'output' (units made per worker or hour) or 'hours' (hours needed per unit)"))?;
    if ({ let t648 = v_basis.clone(); Py::Bool({ let t649 = Py::str("output"); if py::eq(&t648, &t649) { true } else { false } }) }).truthy() {
        let t650 = (Py::tuple(vec![py::truediv(&v_ay.clone(), &v_ax.clone())?, py::truediv(&v_ax.clone(), &v_ay.clone())?, py::truediv(&v_by.clone(), &v_bx.clone())?, py::truediv(&v_bx.clone(), &v_by.clone())?])).items()?;
        if t650.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_oxa = t650[0].clone();
        v_oya = t650[1].clone();
        v_oxb = t650[2].clone();
        v_oyb = t650[3].clone();
        let t651 = (Py::tuple(vec![py::mul(&v_ay.clone(), &v_bx.clone())?, py::mul(&v_by.clone(), &v_ax.clone())?])).items()?;
        if t651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cross_a = t651[0].clone();
        v_cross_b = t651[1].clone();
        let t658 = (Py::tuple(vec![f__who(v_ax.clone(), v_bx.clone(), Py::Bool(true))?, f__who(v_ay.clone(), v_by.clone(), Py::Bool(true))?])).items()?;
        if t658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_abs_x = t658[0].clone();
        v_abs_y = t658[1].clone();
    } else {
        let t659 = (Py::tuple(vec![py::truediv(&v_ax.clone(), &v_ay.clone())?, py::truediv(&v_ay.clone(), &v_ax.clone())?, py::truediv(&v_bx.clone(), &v_by.clone())?, py::truediv(&v_by.clone(), &v_bx.clone())?])).items()?;
        if t659.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_oxa = t659[0].clone();
        v_oya = t659[1].clone();
        v_oxb = t659[2].clone();
        v_oyb = t659[3].clone();
        let t660 = (Py::tuple(vec![py::mul(&v_ax.clone(), &v_by.clone())?, py::mul(&v_bx.clone(), &v_ay.clone())?])).items()?;
        if t660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cross_a = t660[0].clone();
        v_cross_b = t660[1].clone();
        let t661 = (Py::tuple(vec![f__who(v_ax.clone(), v_bx.clone(), Py::Bool(false))?, f__who(v_ay.clone(), v_by.clone(), Py::Bool(false))?])).items()?;
        if t661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_abs_x = t661[0].clone();
        v_abs_y = t661[1].clone();
    }
    if ({ let t662 = py::abs(&py::sub(&v_cross_a.clone(), &v_cross_b.clone())?)?; Py::Bool({ let t663 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_cross_a.clone(), v_cross_b.clone()], true)?)?; if py::le(&t662, &t663)? { true } else { false } }) }).truthy() {
        let t664 = (Py::tuple(vec![Py::str("neither"), Py::str("neither")])).items()?;
        if t664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cx = t664[0].clone();
        v_cy = t664[1].clone();
        let t665 = (Py::tuple(vec![v_oxa.clone(), v_oxa.clone()])).items()?;
        if t665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t665[0].clone();
        v_hi = t665[1].clone();
    } else {
        if ({ let t666 = v_cross_a.clone(); Py::Bool({ let t667 = v_cross_b.clone(); if py::lt(&t666, &t667)? { true } else { false } }) }).truthy() {
            let t668 = (Py::tuple(vec![Py::str("country A"), Py::str("country B")])).items()?;
            if t668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cx = t668[0].clone();
            v_cy = t668[1].clone();
            let t669 = (Py::tuple(vec![v_oxa.clone(), v_oxb.clone()])).items()?;
            if t669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t669[0].clone();
            v_hi = t669[1].clone();
        } else {
            let t670 = (Py::tuple(vec![Py::str("country B"), Py::str("country A")])).items()?;
            if t670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cx = t670[0].clone();
            v_cy = t670[1].clone();
            let t671 = (Py::tuple(vec![v_oxb.clone(), v_oxa.clone()])).items()?;
            if t671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t671[0].clone();
            v_hi = t671[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("opportunity_cost_x_a"), v_oxa.clone()), (Py::str("opportunity_cost_y_a"), v_oya.clone()), (Py::str("opportunity_cost_x_b"), v_oxb.clone()), (Py::str("opportunity_cost_y_b"), v_oyb.clone()), (Py::str("comparative_advantage_x"), v_cx.clone()), (Py::str("comparative_advantage_y"), v_cy.clone()), (Py::str("absolute_advantage_x"), v_abs_x.clone()), (Py::str("absolute_advantage_y"), v_abs_y.clone()), (Py::str("terms_low"), v_lo.clone()), (Py::str("terms_high"), v_hi.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_competitive_firm_cost_curves(mut v_quantities: Py, mut v_total_costs: Py, mut v_price: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_afc: Py = Py::None;
    let mut v_atc: Py = Py::None;
    let mut v_avc: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_even: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_profit: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_shut: Py = Py::None;
    let t707 = (f__cost_table(v_quantities.clone(), v_total_costs.clone())?).items()?;
    if t707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t707[0].clone();
    v_C = t707[1].clone();
    v_P = f__pos(Py::str("price"), v_price.clone())?;
    v_n = py::len(&v_Q.clone())?;
    v_F = py::getitem(&v_C.clone(), &Py::Int(0i64))?;
    let t710 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t710.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_afc = t710[0].clone();
    v_avc = t710[1].clone();
    v_atc = t710[2].clone();
    v_mc = t710[3].clone();
    for t711 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t711;
        let _ = py::method(&v_afc.clone(), "append", &[py::truediv(&v_F.clone(), &py::getitem(&v_Q.clone(), &v_k.clone())?)?])?;
        let _ = py::method(&v_avc.clone(), "append", &[py::truediv(&py::sub(&py::getitem(&v_C.clone(), &v_k.clone())?, &v_F.clone())?, &py::getitem(&v_Q.clone(), &v_k.clone())?)?])?;
        let _ = py::method(&v_atc.clone(), "append", &[py::truediv(&py::getitem(&v_C.clone(), &v_k.clone())?, &py::getitem(&v_Q.clone(), &v_k.clone())?)?])?;
        let _ = py::method(&v_mc.clone(), "append", &[py::truediv(&py::sub(&py::getitem(&v_C.clone(), &v_k.clone())?, &py::getitem(&v_C.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?, &py::sub(&py::getitem(&v_Q.clone(), &v_k.clone())?, &py::getitem(&v_Q.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?])?;
    }
    v_profit = Py::list(vec![]);
    for t712 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t712;
        let _ = py::method(&v_profit.clone(), "append", &[py::sub(&py::mul(&v_P.clone(), &py::getitem(&v_Q.clone(), &v_k.clone())?)?, &py::getitem(&v_C.clone(), &v_k.clone())?)?])?;
    }
    v_best = py::getitem(&v_profit.clone(), &Py::Int(0i64))?;
    for t713 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t713;
        if ({ let t714 = py::getitem(&v_profit.clone(), &v_k.clone())?; Py::Bool({ let t715 = v_best.clone(); if py::gt(&t714, &t715)? { true } else { false } }) }).truthy() {
            v_best = py::getitem(&v_profit.clone(), &v_k.clone())?;
        }
    }
    v_scale = py::add(&py::mul(&v_P.clone(), &py::getitem(&v_Q.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_C.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_j = Py::Int(0i64);
    for t716 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t716;
        if ({ let t717 = py::getitem(&v_profit.clone(), &v_k.clone())?; Py::Bool({ let t718 = py::sub(&v_best.clone(), &py::mul(&Py::Float(1e-12), &v_scale.clone())?)?; if py::ge(&t717, &t718)? { true } else { false } }) }).truthy() {
            v_j = v_k.clone();
        }
    }
    v_shut = py::getitem(&v_avc.clone(), &Py::Int(0i64))?;
    v_even = py::getitem(&v_atc.clone(), &Py::Int(0i64))?;
    for t719 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t719;
        if ({ let t720 = py::getitem(&v_avc.clone(), &v_k.clone())?; Py::Bool({ let t721 = v_shut.clone(); if py::lt(&t720, &t721)? { true } else { false } }) }).truthy() {
            v_shut = py::getitem(&v_avc.clone(), &v_k.clone())?;
        }
        if ({ let t722 = py::getitem(&v_atc.clone(), &v_k.clone())?; Py::Bool({ let t723 = v_even.clone(); if py::lt(&t722, &t723)? { true } else { false } }) }).truthy() {
            v_even = py::getitem(&v_atc.clone(), &v_k.clone())?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("fixed_cost"), v_F.clone()), (Py::str("average_fixed_cost"), v_afc.clone()), (Py::str("average_variable_cost"), v_avc.clone()), (Py::str("average_total_cost"), v_atc.clone()), (Py::str("marginal_cost"), v_mc.clone()), (Py::str("best_output"), py::getitem(&v_Q.clone(), &v_j.clone())?), (Py::str("profit"), f__snap(py::getitem(&v_profit.clone(), &v_j.clone())?, v_scale.clone())?), (Py::str("shutdown_price"), v_shut.clone()), (Py::str("break_even_price"), v_even.clone()), (Py::str("decision"), (if ({ let t726 = v_j.clone(); Py::Bool({ let t727 = Py::Int(0i64); if py::gt(&t726, &t727)? { true } else { false } }) }).truthy() { Py::str("produce") } else { Py::str("shut down") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compound_interest(mut v_principal: Py, mut v_rate: Py, mut v_years: Py, mut v_periods_per_year: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t755 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t755.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t755[0].clone();
    v_r = t755[1].clone();
    v_t = t755[2].clone();
    v_m = f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?;
    v_A = py::mul(&v_P.clone(), &py::pow(&py::add(&Py::Int(1i64), &py::truediv(&v_r.clone(), &v_m.clone())?)?, &py::mul(&v_m.clone(), &v_t.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("amount"), v_A.clone()), (Py::str("interest"), py::sub(&v_A.clone(), &v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_continuous_compounding(mut v_principal: Py, mut v_rate: Py, mut v_years: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t795 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t795.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t795[0].clone();
    v_r = t795[1].clone();
    v_t = t795[2].clone();
    v_A = py::mul(&v_P.clone(), &py::math1("exp", &py::mul(&v_r.clone(), &v_t.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("amount"), v_A.clone()), (Py::str("interest"), py::sub(&v_A.clone(), &v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_correlation(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let t820 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t820[0].clone();
    v_Y = t820[1].clone();
    let _ = f__need({ let t821 = py::len(&v_X.clone())?; Py::Bool({ let t822 = py::len(&v_Y.clone())?; if py::eq(&t821, &t822) { true } else { false } }) }, Py::str("x and y need the same number of observations"))?;
    let t826 = (Py::tuple(vec![f__cov(v_X.clone(), v_X.clone(), Py::Int(1i64))?, f__cov(v_Y.clone(), v_Y.clone(), Py::Int(1i64))?])).items()?;
    if t826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t826[0].clone();
    v_syy = t826[1].clone();
    let _ = f__need({ let t831 = { let t827 = v_sxx.clone(); Py::Bool({ let t828 = Py::Int(0i64); if py::gt(&t827, &t828)? { true } else { false } }) }; if !t831.truthy() { t831 } else { { let t829 = v_syy.clone(); Py::Bool({ let t830 = Py::Int(0i64); if py::gt(&t829, &t830)? { true } else { false } }) } } }, Py::str("neither variable may be constant"))?;
    v_c = f__cov(v_X.clone(), v_Y.clone(), Py::Int(1i64))?;
    v_r = py::truediv(&v_c.clone(), &py::math1("sqrt", &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("correlation"), v_r.clone()), (Py::str("covariance"), v_c.clone()), (Py::str("r_squared"), py::mul(&v_r.clone(), &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cournot(mut v_price_intercept: Py, mut v_price_slope: Py, mut v_cost_1: Py, mut v_cost_2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let mut v_q2: Py = Py::None;
    let t854 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t854[0].clone();
    v_B = t854[1].clone();
    let t857 = (Py::tuple(vec![f__nonneg(Py::str("cost_1"), v_cost_1.clone())?, f__nonneg(Py::str("cost_2"), v_cost_2.clone())?])).items()?;
    if t857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c1 = t857[0].clone();
    v_c2 = t857[1].clone();
    let t858 = (Py::tuple(vec![py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_c1.clone())?)?, &v_c2.clone())?, &py::mul(&Py::Int(3i64), &v_B.clone())?)?, py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_c2.clone())?)?, &v_c1.clone())?, &py::mul(&Py::Int(3i64), &v_B.clone())?)?])).items()?;
    if t858.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q1 = t858[0].clone();
    v_q2 = t858[1].clone();
    let _ = f__need({ let t863 = { let t859 = v_q1.clone(); Py::Bool({ let t860 = Py::Int(0i64); if py::gt(&t859, &t860)? { true } else { false } }) }; if !t863.truthy() { t863 } else { { let t861 = v_q2.clone(); Py::Bool({ let t862 = Py::Int(0i64); if py::gt(&t861, &t862)? { true } else { false } }) } } }, Py::str("one firm would not produce: the costs are too different for an interior Cournot equilibrium"))?;
    v_P = py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &py::add(&v_q1.clone(), &v_q2.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("q1"), v_q1.clone()), (Py::str("q2"), v_q2.clone()), (Py::str("price"), v_P.clone()), (Py::str("profit_1"), py::mul(&py::sub(&v_P.clone(), &v_c1.clone())?, &v_q1.clone())?), (Py::str("profit_2"), py::mul(&py::sub(&v_P.clone(), &v_c2.clone())?, &v_q2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_credit_card_finance_charge(mut v_opening_balance: Py, mut v_apr: Py, mut v_billing_days: Py, mut v_rate_basis: Py, mut v_transaction_days: Py, mut v_transaction_amounts: Py) -> R {
    let mut v_B0: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_adb: Py = Py::None;
    let mut v_amts: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_closing: Py = Py::None;
    let mut v_cycle: Py = Py::None;
    let mut v_days: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_total: Py = Py::None;
    let t922 = (f__cc_inputs(v_opening_balance.clone(), v_apr.clone(), v_billing_days.clone(), v_rate_basis.clone(), v_transaction_days.clone(), v_transaction_amounts.clone())?).items()?;
    if t922.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_B0 = t922[0].clone();
    v_D = t922[1].clone();
    v_days = t922[2].clone();
    v_amts = t922[3].clone();
    v_cycle = t922[4].clone();
    for t923 in py::range_iter(&[(py::len(&v_days.clone())?).i()?]) {
        v_k = t923;
        v_b = v_B0.clone();
        for t924 in py::range_iter(&[(py::len(&v_days.clone())?).i()?]) {
            v_j = t924;
            if ({ let t925 = py::getitem(&v_days.clone(), &v_j.clone())?; Py::Bool({ let t926 = py::getitem(&v_days.clone(), &v_k.clone())?; if py::le(&t925, &t926)? { true } else { false } }) }).truthy() {
                v_b = py::iadd(&v_b, &py::getitem(&v_amts.clone(), &v_j.clone())?)?;
            }
        }
        let _ = f__need({ let t927 = v_b.clone(); Py::Bool({ let t928 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[Py::Float(1.0), v_B0.clone()], true)?)?; if py::ge(&t927, &t928)? { true } else { false } }) }, Py::str("the balance becomes negative (a credit balance): the average daily balance method charges a balance owed"))?;
    }
    v_total = py::mul(&v_D.clone(), &v_B0.clone())?;
    v_closing = v_B0.clone();
    for t929 in py::range_iter(&[(py::len(&v_days.clone())?).i()?]) {
        v_k = t929;
        v_total = py::iadd(&v_total, &py::mul(&py::getitem(&v_amts.clone(), &v_k.clone())?, &py::add(&py::sub(&v_D.clone(), &py::getitem(&v_days.clone(), &v_k.clone())?)?, &Py::Int(1i64))?)?)?;
        v_closing = py::iadd(&v_closing, &py::getitem(&v_amts.clone(), &v_k.clone())?)?;
    }
    v_adb = py::truediv(&v_total.clone(), &v_D.clone())?;
    v_fc = py::mul(&v_adb.clone(), &v_cycle.clone())?;
    return Ok(Py::dict(vec![(Py::str("average_daily_balance"), v_adb.clone()), (Py::str("finance_charge"), v_fc.clone()), (Py::str("cycle_rate"), v_cycle.clone()), (Py::str("closing_balance"), v_closing.clone()), (Py::str("new_balance"), py::add(&v_closing.clone(), &v_fc.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cross_price_elasticity(mut v_price_other_1: Py, mut v_quantity_1: Py, mut v_price_other_2: Py, mut v_quantity_2: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t961 = (f__arc(v_price_other_1.clone(), v_quantity_1.clone(), v_price_other_2.clone(), v_quantity_2.clone(), Py::str("price_other"), Py::str("quantity"))?).items()?;
    if t961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t961[0].clone();
    v_ex = t961[1].clone();
    return Ok(Py::dict(vec![(Py::str("elasticity"), v_e.clone()), (Py::str("relationship"), (if ({ let t962 = v_ex.clone(); Py::Bool({ let t963 = Py::Int(0i64); if py::gt(&t962, &t963)? { true } else { false } }) }).truthy() { Py::str("substitutes") } else { (if ({ let t964 = v_ex.clone(); Py::Bool({ let t965 = Py::Int(0i64); if py::lt(&t964, &t965)? { true } else { false } }) }).truthy() { Py::str("complements") } else { Py::str("independent") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crossover_rate(mut v_cash_flows_a: Py, mut v_cash_flows_b: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t990 = (Py::tuple(vec![f__vec(Py::str("cash_flows_a"), v_cash_flows_a.clone(), Py::Int(2i64))?, f__vec(Py::str("cash_flows_b"), v_cash_flows_b.clone(), Py::Int(2i64))?])).items()?;
    if t990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t990[0].clone();
    v_b = t990[1].clone();
    let _ = f__need({ let t991 = py::len(&v_a.clone())?; Py::Bool({ let t992 = py::len(&v_b.clone())?; if py::eq(&t991, &t992) { true } else { false } }) }, Py::str("the two projects need cash flows over the same periods (lists of equal length)"))?;
    v_d = { let mut out = vec![]; for t993 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t994 = (t993).items()?;
if t994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t994[0].clone();
v_y = t994[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("crossover_rate"), f__irr(v_d.clone())?), (Py::str("npv_at_crossover"), f__hnpv(f__irr(v_d.clone())?, v_a.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_direct_cost_variances(mut v_standard_price: Py, mut v_standard_quantity: Py, mut v_actual_price: Py, mut v_actual_quantity: Py, mut v_quantity_purchased: Py) -> R {
    let mut v_AP: Py = Py::None;
    let mut v_AQ: Py = Py::None;
    let mut v_QP: Py = Py::None;
    let mut v_SP: Py = Py::None;
    let mut v_SQ: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_qv: Py = Py::None;
    let mut v_tv: Py = Py::None;
    let t1063 = (f__direct_var_inputs(v_standard_price.clone(), v_standard_quantity.clone(), v_actual_price.clone(), v_actual_quantity.clone(), v_quantity_purchased.clone())?).items()?;
    if t1063.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_SP = t1063[0].clone();
    v_SQ = t1063[1].clone();
    v_AP = t1063[2].clone();
    v_AQ = t1063[3].clone();
    v_QP = t1063[4].clone();
    v_pv = py::mul(&v_QP.clone(), &py::sub(&v_AP.clone(), &v_SP.clone())?)?;
    v_qv = py::mul(&v_SP.clone(), &py::sub(&v_AQ.clone(), &v_SQ.clone())?)?;
    v_tv = f__snap(py::add(&v_pv.clone(), &v_qv.clone())?, py::add(&py::add(&py::add(&py::mul(&v_QP.clone(), &v_AP.clone())?, &py::mul(&v_QP.clone(), &v_SP.clone())?)?, &py::mul(&v_AQ.clone(), &v_SP.clone())?)?, &py::mul(&v_SQ.clone(), &v_SP.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("price_variance"), v_pv.clone()), (Py::str("quantity_variance"), v_qv.clone()), (Py::str("total_variance"), v_tv.clone()), (Py::str("price_variance_type"), f__fav(v_pv.clone())?), (Py::str("quantity_variance_type"), f__fav(v_qv.clone())?), (Py::str("total_variance_type"), f__fav(v_tv.clone())?), (Py::str("standard_cost_allowed"), py::mul(&v_SQ.clone(), &v_SP.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discounted_payback(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1097 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t1097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1097[0].clone();
    v_c = t1097[1].clone();
    return Ok(Py::dict(vec![(Py::str("discounted_payback"), f__payback({ let mut out = vec![]; for t1107 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t1108 = (t1107).items()?;
if t1108.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t1108[0].clone();
v_x = t1108[1].clone();
out.push(py::truediv(&v_x.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_t.clone())?)?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_doubling_time(mut v_rate: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?;
    return Ok(Py::dict(vec![(Py::str("periods"), py::truediv(&py::math1("log", &Py::Int(2i64))?, &py::math1("log", &py::add(&Py::Int(1i64), &v_r.clone())?)?)?), (Py::str("rule_of_72"), py::truediv(&Py::Int(72i64), &py::mul(&Py::Int(100i64), &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_durbin_watson(mut v_residuals: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_e = f__vec(Py::str("residuals"), v_residuals.clone(), Py::Int(2i64))?;
    v_den = py::fsum(&{ let mut out = vec![]; for t1156 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t1156;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    let _ = f__need({ let t1157 = v_den.clone(); Py::Bool({ let t1158 = Py::Int(0i64); if py::gt(&t1157, &t1158)? { true } else { false } }) }, Py::str("the residuals are all zero"))?;
    return Ok(Py::dict(vec![(Py::str("durbin_watson"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t1159 in py::iter(&(py::zip(&[v_e.clone(), py::slice(&v_e.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1160 = (t1159).items()?;
if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1160[0].clone();
v_b = t1160[1].clone();
out.push(py::pow(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_den.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_annual_rate(mut v_rate: Py, mut v_periods_per_year: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1196 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1196[0].clone();
    v_m = t1196[1].clone();
    return Ok(Py::dict(vec![(Py::str("effective_rate"), py::sub(&py::pow(&py::add(&Py::Int(1i64), &py::truediv(&v_r.clone(), &v_m.clone())?)?, &v_m.clone())?, &Py::Int(1i64))?), (Py::str("periodic_rate"), py::truediv(&v_r.clone(), &v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_endowment_insurance(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_term: Py, mut v_benefit: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pe: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1233 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t1233.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1233[0].clone();
    v_v = t1233[1].clone();
    v_B = t1233[2].clone();
    v_n = f__int(Py::str("term"), v_term.clone(), Py::Int(1i64), py::len(&v_q.clone())?)?;
    let t1248 = (Py::tuple(vec![f__A(v_q.clone(), v_v.clone(), v_n.clone())?, py::mul(&py::pow(&v_v.clone(), &v_n.clone())?, &py::getitem(&f__surv(v_q.clone())?, &v_n.clone())?)?])).items()?;
    if t1248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t1248[0].clone();
    v_pe = t1248[1].clone();
    return Ok(Py::dict(vec![(Py::str("apv"), py::mul(&v_B.clone(), &py::add(&v_t.clone(), &v_pe.clone())?)?), (Py::str("term_part"), py::mul(&v_B.clone(), &v_t.clone())?), (Py::str("pure_endowment"), py::mul(&v_B.clone(), &v_pe.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equivalent_annual_annuity(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1276 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t1276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1276[0].clone();
    v_c = t1276[1].clone();
    let t1279 = (Py::tuple(vec![py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?, f__hnpv(v_r.clone(), v_c.clone())?])).items()?;
    if t1279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1279[0].clone();
    v_v = t1279[1].clone();
    return Ok(Py::dict(vec![(Py::str("eaa"), (if ({ let t1280 = v_r.clone(); Py::Bool({ let t1281 = Py::Int(0i64); if py::eq(&t1280, &t1281) { true } else { false } }) }).truthy() { py::truediv(&v_v.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_v.clone(), &v_r.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &py::neg(&v_n.clone())?)?)?)? })), (Py::str("npv"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_financial_ratios(mut v_sales: Py, mut v_net_income: Py, mut v_total_assets: Py, mut v_total_equity: Py, mut v_current_assets: Py, mut v_current_liabilities: Py, mut v_inventory: Py) -> R {
    let mut v_CA: Py = Py::None;
    let mut v_CL: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_INV: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_NI: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_TA: Py = Py::None;
    let t1314 = (f__ratio_inputs(v_sales.clone(), v_net_income.clone(), v_total_assets.clone(), v_total_equity.clone(), v_current_assets.clone(), v_current_liabilities.clone(), v_inventory.clone())?).items()?;
    if t1314.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1314[0].clone();
    v_NI = t1314[1].clone();
    v_TA = t1314[2].clone();
    v_E = t1314[3].clone();
    v_CA = t1314[4].clone();
    v_CL = t1314[5].clone();
    v_INV = t1314[6].clone();
    v_L = py::sub(&v_TA.clone(), &v_E.clone())?;
    return Ok(Py::dict(vec![(Py::str("current_ratio"), py::truediv(&v_CA.clone(), &v_CL.clone())?), (Py::str("quick_ratio"), py::truediv(&py::sub(&v_CA.clone(), &v_INV.clone())?, &v_CL.clone())?), (Py::str("debt_ratio"), py::truediv(&v_L.clone(), &v_TA.clone())?), (Py::str("debt_to_equity"), py::truediv(&v_L.clone(), &v_E.clone())?), (Py::str("net_profit_margin"), py::truediv(&v_NI.clone(), &v_S.clone())?), (Py::str("asset_turnover"), py::truediv(&v_S.clone(), &v_TA.clone())?), (Py::str("equity_multiplier"), py::truediv(&v_TA.clone(), &v_E.clone())?), (Py::str("roa"), py::truediv(&v_NI.clone(), &v_TA.clone())?), (Py::str("roe"), py::truediv(&v_NI.clone(), &v_E.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fisher_real_rate(mut v_nominal_rate: Py, mut v_inflation_rate: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let t1338 = (Py::tuple(vec![f__rate(Py::str("nominal_rate"), v_nominal_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("inflation_rate"), v_inflation_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i = t1338[0].clone();
    v_pi = t1338[1].clone();
    return Ok(Py::dict(vec![(Py::str("real_rate"), py::sub(&py::truediv(&py::add(&Py::Int(1i64), &v_i.clone())?, &py::add(&Py::Int(1i64), &v_pi.clone())?)?, &Py::Int(1i64))?), (Py::str("approximate_real_rate"), py::sub(&v_i.clone(), &v_pi.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forward_price(mut v_spot: Py, mut v_rate: Py, mut v_time: Py, mut v_dividend_yield: Py, mut v_storage_cost: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t1363 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__nonneg(Py::str("time"), v_time.clone())?])).items()?;
    if t1363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1363[0].clone();
    v_T = t1363[1].clone();
    let t1367 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("dividend_yield"), v_dividend_yield.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("storage_cost"), v_storage_cost.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1367.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1367[0].clone();
    v_q = t1367[1].clone();
    v_u = t1367[2].clone();
    return Ok(Py::dict(vec![(Py::str("forward"), py::mul(&v_S.clone(), &py::math1("exp", &py::mul(&py::sub(&py::add(&v_r.clone(), &v_u.clone())?, &v_q.clone())?, &v_T.clone())?)?)?), (Py::str("cost_of_carry"), py::sub(&py::add(&v_r.clone(), &v_u.clone())?, &v_q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forward_price_dividends(mut v_spot: Py, mut v_rate: Py, mut v_time: Py, mut v_dividends: Py, mut v_dividend_times: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1393 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__pos(Py::str("time"), v_time.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1393.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1393[0].clone();
    v_T = t1393[1].clone();
    v_r = t1393[2].clone();
    let t1398 = (Py::tuple(vec![f__vec(Py::str("dividends"), v_dividends.clone(), Py::Int(1i64))?, f__vec(Py::str("dividend_times"), v_dividend_times.clone(), Py::Int(1i64))?])).items()?;
    if t1398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1398[0].clone();
    v_t = t1398[1].clone();
    let _ = f__need({ let t1399 = py::len(&v_D.clone())?; Py::Bool({ let t1400 = py::len(&v_t.clone())?; if py::eq(&t1399, &t1400) { true } else { false } }) }, Py::str("one time per dividend"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1401 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1401;
out.push({ let t1402 = Py::Int(0i64); Py::Bool({ let t1403 = v_x.clone(); if py::lt(&t1402, &t1403)? { { let t1404 = v_T.clone(); if py::le(&t1403, &t1404)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("each dividend time must lie in (0, time]"))?;
    v_pv = py::fsum(&{ let mut out = vec![]; for t1405 in py::iter(&(py::zip(&[v_D.clone(), v_t.clone()])?))? { let mut v_d: Py; let mut v_x: Py; 
let t1406 = (t1405).items()?;
if t1406.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t1406[0].clone();
v_x = t1406[1].clone();
out.push(py::mul(&v_d.clone(), &py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_x.clone())?)?)?); } Py::list(out) })?;
    let _ = f__need({ let t1407 = v_pv.clone(); Py::Bool({ let t1408 = v_S.clone(); if py::lt(&t1407, &t1408)? { true } else { false } }) }, Py::str("the dividends are worth more than the asset"))?;
    return Ok(Py::dict(vec![(Py::str("forward"), py::mul(&py::sub(&v_S.clone(), &v_pv.clone())?, &py::math1("exp", &py::mul(&v_r.clone(), &v_T.clone())?)?)?), (Py::str("pv_dividends"), v_pv.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forward_rate(mut v_spot_rate_1: Py, mut v_maturity_1: Py, mut v_spot_rate_2: Py, mut v_maturity_2: Py) -> R {
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t1434 = (Py::tuple(vec![f__rate(Py::str("spot_rate_1"), v_spot_rate_1.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("maturity_1"), v_maturity_1.clone())?])).items()?;
    if t1434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t1434[0].clone();
    v_t1 = t1434[1].clone();
    let t1437 = (Py::tuple(vec![f__rate(Py::str("spot_rate_2"), v_spot_rate_2.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__pos(Py::str("maturity_2"), v_maturity_2.clone())?])).items()?;
    if t1437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s2 = t1437[0].clone();
    v_t2 = t1437[1].clone();
    let _ = f__need({ let t1438 = v_t2.clone(); Py::Bool({ let t1439 = v_t1.clone(); if py::gt(&t1438, &t1439)? { true } else { false } }) }, Py::str("maturity_2 must be later than maturity_1"))?;
    return Ok(Py::dict(vec![(Py::str("forward_rate"), py::sub(&py::pow(&py::truediv(&py::pow(&py::add(&Py::Int(1i64), &v_s2.clone())?, &v_t2.clone())?, &py::pow(&py::add(&Py::Int(1i64), &v_s1.clone())?, &v_t1.clone())?)?, &py::truediv(&Py::Int(1i64), &py::sub(&v_t2.clone(), &v_t1.clone())?)?)?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_future_value_annuity(mut v_payment: Py, mut v_rate: Py, mut v_periods: Py, mut v_timing: Py) -> R {
    let mut v_due: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1479 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t1479.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1479[0].clone();
    v_r = t1479[1].clone();
    v_n = t1479[2].clone();
    v_due = t1479[3].clone();
    return Ok(Py::dict(vec![(Py::str("future_value"), f__annuity_fv(v_p.clone(), v_r.clone(), v_n.clone(), v_due.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gdp_deflator(mut v_base_prices: Py, mut v_base_quantities: Py, mut v_current_prices: Py, mut v_current_quantities: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_nom: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let mut v_real: Py = Py::None;
    let mut v_real0: Py = Py::None;
    let t1525 = (f__baskets(v_base_prices.clone(), v_base_quantities.clone(), v_current_prices.clone(), v_current_quantities.clone())?).items()?;
    if t1525.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t1525[0].clone();
    v_q0 = t1525[1].clone();
    v_p1 = t1525[2].clone();
    v_q1 = t1525[3].clone();
    let t1528 = (Py::tuple(vec![f__dot(v_p1.clone(), v_q1.clone())?, f__dot(v_p0.clone(), v_q1.clone())?, f__dot(v_p0.clone(), v_q0.clone())?])).items()?;
    if t1528.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_nom = t1528[0].clone();
    v_real = t1528[1].clone();
    v_real0 = t1528[2].clone();
    v_d = py::truediv(&py::mul(&Py::Int(100i64), &v_nom.clone())?, &v_real.clone())?;
    return Ok(Py::dict(vec![(Py::str("nominal_gdp"), v_nom.clone()), (Py::str("real_gdp"), v_real.clone()), (Py::str("deflator"), v_d.clone()), (Py::str("inflation"), py::sub(&py::truediv(&v_d.clone(), &Py::Int(100i64))?, &Py::Int(1i64))?), (Py::str("real_growth"), py::sub(&py::truediv(&v_real.clone(), &v_real0.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gini_coefficient(mut v_incomes: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("incomes"), v_incomes.clone(), Py::Int(2i64))?;
    let _ = f__need({ let t1558 = Py::Bool(({ let mut out = vec![]; for t1553 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1553;
out.push({ let t1554 = v_v.clone(); Py::Bool({ let t1555 = Py::Int(0i64); if py::ge(&t1554, &t1555)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1558.truthy() { t1558 } else { { let t1556 = py::sum(&v_x.clone(), None)?; Py::Bool({ let t1557 = Py::Int(0i64); if py::gt(&t1556, &t1557)? { true } else { false } }) } } }, Py::str("incomes must be non-negative with a positive total"))?;
    let t1559 = (Py::tuple(vec![py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?, py::len(&v_x.clone())?])).items()?;
    if t1559.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1559[0].clone();
    v_n = t1559[1].clone();
    return Ok(Py::dict(vec![(Py::str("gini"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t1560 in py::iter(&(py::enumerate(&v_s.clone(), (Py::Int(1i64)).i()?)?))? { let mut v_i: Py; let mut v_v: Py; 
let t1561 = (t1560).items()?;
if t1561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1561[0].clone();
v_v = t1561[1].clone();
out.push(py::mul(&py::sub(&py::sub(&py::mul(&Py::Int(2i64), &v_i.clone())?, &v_n.clone())?, &Py::Int(1i64))?, &v_v.clone())?); } Py::list(out) })?, &py::mul(&v_n.clone(), &py::fsum(&v_s.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_global_min_variance(mut v_covariance: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_S = f__square(Py::str("covariance"), v_covariance.clone(), Py::None)?;
    v_L = f__cholesky(v_S.clone())?;
    let _ = f__need({ let t1616 = v_L.clone(); Py::Bool({ let t1617 = Py::None; if !py::is(&t1616, &t1617) { true } else { false } }) }, Py::str("the covariance matrix must be positive definite (no riskless combination of the assets)"))?;
    v_x = f__chol_solve(v_L.clone(), py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_S.clone())?)?)?;
    v_w = { let mut out = vec![]; for t1622 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1622;
out.push(py::truediv(&v_v.clone(), &py::fsum(&v_x.clone())?)?); } Py::list(out) };
    v_var = f__quad(v_w.clone(), v_S.clone())?;
    return Ok(Py::dict(vec![(Py::str("weights"), v_w.clone()), (Py::str("variance"), v_var.clone()), (Py::str("std_dev"), py::math1("sqrt", &py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gordon_growth(mut v_required_return: Py, mut v_growth: Py, mut v_dividend: Py, mut v_next_dividend: Py) -> R {
    let mut v_D1: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1648 = (Py::tuple(vec![f__rate(Py::str("required_return"), v_required_return.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1648[0].clone();
    v_g = t1648[1].clone();
    let _ = f__need({ let t1649 = v_r.clone(); Py::Bool({ let t1650 = v_g.clone(); if py::gt(&t1649, &t1650)? { true } else { false } }) }, Py::str("the required return must exceed the growth rate (otherwise the value is infinite)"))?;
    v_D1 = f__d1(v_dividend.clone(), v_next_dividend.clone(), v_g.clone())?;
    return Ok(Py::dict(vec![(Py::str("price"), py::truediv(&v_D1.clone(), &py::sub(&v_r.clone(), &v_g.clone())?)?), (Py::str("next_dividend"), v_D1.clone()), (Py::str("dividend_yield"), py::sub(&v_r.clone(), &v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gordon_implied_return(mut v_price: Py, mut v_growth: Py, mut v_dividend: Py, mut v_next_dividend: Py) -> R {
    let mut v_D1: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1684 = (Py::tuple(vec![f__pos(Py::str("price"), v_price.clone())?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1684[0].clone();
    v_g = t1684[1].clone();
    v_D1 = f__d1(v_dividend.clone(), v_next_dividend.clone(), v_g.clone())?;
    let _ = f__need({ let t1693 = v_D1.clone(); Py::Bool({ let t1694 = Py::Int(0i64); if py::gt(&t1693, &t1694)? { true } else { false } }) }, Py::str("the dividend must be positive"))?;
    v_r = py::add(&py::truediv(&v_D1.clone(), &v_P.clone())?, &v_g.clone())?;
    let _ = f__need({ let t1695 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1696 = v_r.clone(); if py::lt(&t1695, &t1696)? { { let t1697 = Py::Int(1i64); if py::lt(&t1696, &t1697)? { true } else { false } } } else { false } }) }, Py::str("the implied required return is outside -100%..100%"))?;
    return Ok(Py::dict(vec![(Py::str("required_return"), v_r.clone()), (Py::str("dividend_yield"), py::truediv(&v_D1.clone(), &v_P.clone())?), (Py::str("next_dividend"), v_D1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_growing_annuity(mut v_first_payment: Py, mut v_rate: Py, mut v_growth: Py, mut v_periods: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1733 = (Py::tuple(vec![f__fin(Py::str("first_payment"), v_first_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1733.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1733[0].clone();
    v_r = t1733[1].clone();
    v_g = t1733[2].clone();
    v_n = t1733[3].clone();
    v_pv = (if ({ let t1734 = py::abs(&py::sub(&v_r.clone(), &v_g.clone())?)?; Py::Bool({ let t1735 = Py::Float(1e-15); if py::lt(&t1734, &t1735)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_c.clone(), &v_n.clone())?, &py::add(&Py::Int(1i64), &v_r.clone())?)? } else { py::mul(&py::truediv(&v_c.clone(), &py::sub(&v_r.clone(), &v_g.clone())?)?, &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&py::add(&Py::Int(1i64), &v_g.clone())?, &py::add(&Py::Int(1i64), &v_r.clone())?)?, &v_n.clone())?)?)? });
    return Ok(Py::dict(vec![(Py::str("present_value"), v_pv.clone()), (Py::str("future_value"), py::mul(&v_pv.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_growing_perpetuity(mut v_first_payment: Py, mut v_rate: Py, mut v_growth: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1759 = (Py::tuple(vec![f__fin(Py::str("first_payment"), v_first_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1759.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1759[0].clone();
    v_r = t1759[1].clone();
    v_g = t1759[2].clone();
    let _ = f__need({ let t1760 = v_r.clone(); Py::Bool({ let t1761 = v_g.clone(); if py::gt(&t1760, &t1761)? { true } else { false } }) }, Py::str("the discount rate must exceed the growth rate (otherwise the value is infinite)"))?;
    return Ok(Py::dict(vec![(Py::str("present_value"), py::truediv(&v_c.clone(), &py::sub(&v_r.clone(), &v_g.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_herfindahl_index(mut v_market_shares: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("market_shares"), v_market_shares.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t1791 = Py::Bool(({ let mut out = vec![]; for t1786 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1786;
out.push({ let t1787 = v_v.clone(); Py::Bool({ let t1788 = Py::Int(0i64); if py::ge(&t1787, &t1788)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1791.truthy() { t1791 } else { { let t1789 = py::sum(&v_x.clone(), None)?; Py::Bool({ let t1790 = Py::Int(0i64); if py::gt(&t1789, &t1790)? { true } else { false } }) } } }, Py::str("shares (or sales) must be non-negative with a positive total"))?;
    v_tot = py::fsum(&v_x.clone())?;
    v_s = { let mut out = vec![]; for t1792 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1792;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_h = py::fsum(&{ let mut out = vec![]; for t1793 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t1793;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_n = py::len(&v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("hhi"), py::mul(&Py::Int(10000i64), &v_h.clone())?), (Py::str("hhi_fraction"), v_h.clone()), (Py::str("cr4"), py::mul(&Py::Int(100i64), &py::fsum(&py::slice(&py::sorted(&v_s.clone(), (Py::Bool(true)).truthy())?, None, Some((Py::Int(4i64)).i()?), None)?)?)?), (Py::str("equivalent_firms"), py::truediv(&Py::Int(1i64), &v_h.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_historical_var(mut v_returns: Py, mut v_confidence: Py, mut v_portfolio_value: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_r = f__returns(Py::str("returns"), v_returns.clone(), Py::Int(2i64), Py::Float(1.0))?;
    v_a = f__confidence(v_confidence.clone())?;
    v_V = f__pos(Py::str("portfolio_value"), v_portfolio_value.clone())?;
    v_k = py::math1("ceil", &py::mul(&py::len(&v_r.clone())?, &py::sub(&Py::Int(1i64), &py::fraction(&[Py::str(&py::repr(&v_a.clone()))])?)?)?)?;
    v_s = py::sorted(&v_r.clone(), (Py::Bool(false)).truthy())?;
    return Ok(Py::dict(vec![(Py::str("var"), py::mul(&py::neg(&py::getitem(&v_s.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?, &v_V.clone())?), (Py::str("cvar"), py::mul(&py::truediv(&py::neg(&py::fsum(&py::slice(&v_s.clone(), None, Some((v_k.clone()).i()?), None)?)?)?, &v_k.clone())?, &v_V.clone())?), (Py::str("tail_observations"), v_k.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_implied_volatility(mut v_option_price: Py, mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_dividend_yield: Py, mut v_option_type: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v__s: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_vega: Py = Py::None;
    let t1856 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), Py::Float(0.2), v_dividend_yield.clone())?).items()?;
    if t1856.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1856[0].clone();
    v_K = t1856[1].clone();
    v_T = t1856[2].clone();
    v_r = t1856[3].clone();
    v__s = t1856[4].clone();
    v_q = t1856[5].clone();
    v_p = f__pos(Py::str("option_price"), v_option_price.clone())?;
    let _ = f__need({ let t1857 = v_option_type.clone(); Py::Bool({ let t1858 = Py::tuple(vec![Py::str("call"), Py::str("put")]); if py::contains(&t1858, &t1857)? { true } else { false } }) }, Py::str("option_type is call or put"))?;
    v_f = { let v_K = v_K.clone(); let v_S = v_S.clone(); let v_T = v_T.clone(); let v__s = v__s.clone(); let v_dividend_yield = v_dividend_yield.clone(); let v_option_price = v_option_price.clone(); let v_option_type = v_option_type.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_rate = v_rate.clone(); let v_spot = v_spot.clone(); let v_strike = v_strike.clone(); let v_time = v_time.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone())?, &(if ({ let t1860 = v_option_type.clone(); Py::Bool({ let t1861 = Py::str("call"); if py::eq(&t1860, &t1861) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(3i64) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1862 = (Py::tuple(vec![Py::Float(0.0001), Py::Float(5.0)])).items()?;
    if t1862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1862[0].clone();
    v_hi = t1862[1].clone();
    let _ = f__need({ let t1863 = py::call(&v_f.clone(), &[v_lo.clone()])?; Py::Bool({ let t1864 = v_p.clone(); if py::lt(&t1863, &t1864)? { { let t1865 = py::call(&v_f.clone(), &[v_hi.clone()])?; if py::lt(&t1864, &t1865)? { true } else { false } } } else { false } }) }, Py::str("the price is outside the range Black-Scholes can produce (volatility 0.01% to 500%): it violates the no-arbitrage bounds or implies an extreme volatility"))?;
    for t1866 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1866;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t1867 = v_mid.clone(); Py::Bool({ let t1868 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1868, &t1867)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t1869 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t1870 = v_p.clone(); if py::lt(&t1869, &t1870)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_sig = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    v_vega = py::mul(&py::mul(&py::mul(&v_S.clone(), &py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?)?, &f__phi(py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_sig.clone(), v_q.clone())?, &Py::Int(0i64))?)?)?, &py::math1("sqrt", &v_T.clone())?)?;
    let _ = f__need({ let t1871 = v_vega.clone(); Py::Bool({ let t1872 = py::mul(&Py::Float(0.0001), &v_S.clone())?; if py::gt(&t1871, &t1872)? { true } else { false } }) }, Py::str("the option is too far from the money: its price barely depends on volatility, so the implied volatility is not well determined"))?;
    return Ok(Py::dict(vec![(Py::str("volatility"), v_sig.clone()), (Py::str("vega"), v_vega.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_income_elasticity(mut v_income_1: Py, mut v_quantity_1: Py, mut v_income_2: Py, mut v_quantity_2: Py) -> R {
    let mut v_cls: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t1887 = (f__arc(v_income_1.clone(), v_quantity_1.clone(), v_income_2.clone(), v_quantity_2.clone(), Py::str("income"), Py::str("quantity"))?).items()?;
    if t1887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1887[0].clone();
    v_ex = t1887[1].clone();
    v_cls = (if ({ let t1888 = v_ex.clone(); Py::Bool({ let t1889 = Py::Int(0i64); if py::lt(&t1888, &t1889)? { true } else { false } }) }).truthy() { Py::str("inferior good") } else { (if ({ let t1890 = v_ex.clone(); Py::Bool({ let t1891 = Py::Int(0i64); if py::eq(&t1890, &t1891) { true } else { false } }) }).truthy() { Py::str("neutral") } else { (if ({ let t1892 = v_ex.clone(); Py::Bool({ let t1893 = Py::Int(1i64); if py::gt(&t1892, &t1893)? { true } else { false } }) }).truthy() { Py::str("normal good (luxury)") } else { Py::str("normal good (necessity)") }) }) });
    return Ok(Py::dict(vec![(Py::str("elasticity"), v_e.clone()), (Py::str("classification"), v_cls.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_income_tax(mut v_taxable_income: Py, mut v_bracket_thresholds: Py, mut v_bracket_rates: Py) -> R {
    let mut v_full: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tax: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1931 = (f__tax_inputs(v_taxable_income.clone(), v_bracket_thresholds.clone(), v_bracket_rates.clone())?).items()?;
    if t1931.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1931[0].clone();
    v_t = t1931[1].clone();
    v_rs = t1931[2].clone();
    v_j = Py::Int(0i64);
    for t1932 in py::range_iter(&[(py::len(&v_t.clone())?).i()?]) {
        v_k = t1932;
        if ({ let t1933 = py::getitem(&v_t.clone(), &v_k.clone())?; Py::Bool({ let t1934 = v_x.clone(); if py::le(&t1933, &t1934)? { true } else { false } }) }).truthy() {
            v_j = v_k.clone();
        }
    }
    v_full = Py::Float(0.0);
    for t1935 in py::range_iter(&[(v_j.clone()).i()?]) {
        v_k = t1935;
        v_full = py::iadd(&v_full, &py::mul(&py::getitem(&v_rs.clone(), &v_k.clone())?, &py::sub(&py::getitem(&v_t.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_t.clone(), &v_k.clone())?)?)?)?;
    }
    v_tax = py::add(&v_full.clone(), &py::mul(&py::getitem(&v_rs.clone(), &v_j.clone())?, &py::sub(&v_x.clone(), &py::getitem(&v_t.clone(), &v_j.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("tax"), v_tax.clone()), (Py::str("marginal_rate"), py::getitem(&v_rs.clone(), &v_j.clone())?), (Py::str("effective_rate"), py::truediv(&v_tax.clone(), &v_x.clone())?), (Py::str("after_tax_income"), py::sub(&v_x.clone(), &v_tax.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inflation_adjustment(mut v_price_levels: Py, mut v_amount: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__vec(Py::str("price_levels"), v_price_levels.clone(), Py::Int(2i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1969 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t1969;
out.push({ let t1970 = v_x.clone(); Py::Bool({ let t1971 = Py::Int(0i64); if py::gt(&t1970, &t1971)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("price levels must be positive"))?;
    v_n = py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?;
    v_out = Py::dict(vec![(Py::str("total_inflation"), py::sub(&py::truediv(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?), (Py::str("average_inflation"), py::sub(&py::pow(&py::truediv(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &py::truediv(&Py::Int(1i64), &v_n.clone())?)?, &Py::Int(1i64))?)]);
    if ({ let t1972 = v_amount.clone(); Py::Bool({ let t1973 = Py::None; if !py::is(&t1972, &t1973) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("real_value"), py::truediv(&py::mul(&f__fin(Py::str("amount"), v_amount.clone())?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_information_ratio(mut v_portfolio_returns: Py, mut v_benchmark_returns: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_te: Py = Py::None;
    let t2005 = (f__pair(v_portfolio_returns.clone(), v_benchmark_returns.clone(), Py::str("portfolio_returns"), Py::str("benchmark_returns"), Py::Int(2i64))?).items()?;
    if t2005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2005[0].clone();
    v_b = t2005[1].clone();
    v_a = { let mut out = vec![]; for t2006 in py::iter(&(py::zip(&[v_p.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2007 = (t2006).items()?;
if t2007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2007[0].clone();
v_y = t2007[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    v_te = f__sd(v_a.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2011 = v_te.clone(); Py::Bool({ let t2012 = Py::Int(0i64); if py::gt(&t2011, &t2012)? { true } else { false } }) }, Py::str("the active returns are constant: the tracking error is zero"))?;
    return Ok(Py::dict(vec![(Py::str("information_ratio"), py::truediv(&f__mean(v_a.clone())?, &v_te.clone())?), (Py::str("tracking_error"), v_te.clone()), (Py::str("mean_active_return"), f__mean(v_a.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inventory_costing(mut v_layer_units: Py, mut v_layer_costs: Py, mut v_units_sold: Py, mut v_selling_price: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_total_c: Py = Py::None;
    let mut v_total_u: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t2048 = (f__layers(v_layer_units.clone(), v_layer_costs.clone(), v_units_sold.clone())?).items()?;
    if t2048.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2048[0].clone();
    v_c = t2048[1].clone();
    v_total_u = t2048[2].clone();
    v_total_c = t2048[3].clone();
    v_q = t2048[4].clone();
    v_n = py::len(&v_u.clone())?;
    v_first = Py::list(vec![]);
    v_last = Py::list(vec![]);
    for t2049 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2049;
        let _ = py::method(&v_first.clone(), "append", &[v_k.clone()])?;
        let _ = py::method(&v_last.clone(), "append", &[py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?])?;
    }
    return Ok(f__inventory_out(v_total_u.clone(), v_total_c.clone(), v_q.clone(), f__consume(v_u.clone(), v_c.clone(), v_q.clone(), v_first.clone())?, f__consume(v_u.clone(), v_c.clone(), v_q.clone(), v_last.clone())?, v_selling_price.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_irr(mut v_cash_flows: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("irr"), f__irr(f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_life_annuity_due(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_payment: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2155 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("payment"), v_payment.clone())?])).items()?;
    if t2155.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2155[0].clone();
    v_v = t2155[1].clone();
    v_P = t2155[2].clone();
    return Ok(Py::dict(vec![(Py::str("apv"), py::mul(&v_P.clone(), &f__a_due(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_life_expectancy(mut v_mortality_rates: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_q = f__qtab(v_mortality_rates.clone(), Py::Bool(true))?;
    v_e = py::fsum(&py::slice(&f__surv(v_q.clone())?, Some((Py::Int(1i64)).i()?), None, None)?)?;
    return Ok(Py::dict(vec![(Py::str("curtate"), v_e.clone()), (Py::str("complete"), py::add(&v_e.clone(), &Py::Float(0.5))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loan_balance(mut v_principal: Py, mut v_annual_rate: Py, mut v_years: Py, mut v_payments_made: Py, mut v_payments_per_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pmt: Py = Py::None;
    let t2237 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t2237.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2237[0].clone();
    v_i = t2237[1].clone();
    v_n = t2237[2].clone();
    v_pmt = t2237[3].clone();
    v_k = f__int(Py::str("payments_made"), v_payments_made.clone(), Py::Int(0i64), v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("balance"), (if ({ let t2238 = v_k.clone(); Py::Bool({ let t2239 = v_n.clone(); if py::lt(&t2238, &t2239)? { true } else { false } }) }).truthy() { py::minmax(&[Py::Float(0.0), f__balance(v_P.clone(), v_i.clone(), v_pmt.clone(), v_k.clone())?], true)? } else { Py::Float(0.0) })), (Py::str("payment"), v_pmt.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loan_cumulative_interest(mut v_principal: Py, mut v_annual_rate: Py, mut v_years: Py, mut v_payments_made: Py, mut v_payments_per_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pmt: Py = Py::None;
    let mut v_repaid: Py = Py::None;
    let t2289 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t2289.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2289[0].clone();
    v_i = t2289[1].clone();
    v_n = t2289[2].clone();
    v_pmt = t2289[3].clone();
    v_k = f__int(Py::str("payments_made"), v_payments_made.clone(), Py::Int(1i64), v_n.clone())?;
    v_repaid = py::sub(&v_P.clone(), &(if ({ let t2290 = v_k.clone(); Py::Bool({ let t2291 = v_n.clone(); if py::lt(&t2290, &t2291)? { true } else { false } }) }).truthy() { f__balance(v_P.clone(), v_i.clone(), v_pmt.clone(), v_k.clone())? } else { Py::Float(0.0) }))?;
    return Ok(Py::dict(vec![(Py::str("cumulative_interest"), py::sub(&py::mul(&v_pmt.clone(), &v_k.clone())?, &v_repaid.clone())?), (Py::str("principal_repaid"), v_repaid.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loan_payment(mut v_principal: Py, mut v_annual_rate: Py, mut v_years: Py, mut v_payments_per_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pmt: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2319 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("annual_rate"), v_annual_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t2319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2319[0].clone();
    v_r = t2319[1].clone();
    v_m = f__int(Py::str("payments_per_year"), v_payments_per_year.clone(), Py::Int(1i64), Py::Int(365i64))?;
    v_n = f__periods_of(f__pos(Py::str("years"), v_years.clone())?, v_m.clone(), Py::str("years"))?;
    v_i = py::truediv(&v_r.clone(), &v_m.clone())?;
    v_pmt = (if ({ let t2339 = v_i.clone(); Py::Bool({ let t2340 = Py::Int(0i64); if py::eq(&t2339, &t2340) { true } else { false } }) }).truthy() { py::truediv(&v_P.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_P.clone(), &v_i.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &py::neg(&v_n.clone())?)?)?)? });
    return Ok(Py::dict(vec![(Py::str("payment"), v_pmt.clone()), (Py::str("number_of_payments"), v_n.clone()), (Py::str("total_paid"), py::mul(&v_pmt.clone(), &v_n.clone())?), (Py::str("total_interest"), py::sub(&py::mul(&v_pmt.clone(), &v_n.clone())?, &v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loan_payment_split(mut v_principal: Py, mut v_annual_rate: Py, mut v_years: Py, mut v_payment_number: Py, mut v_payments_per_year: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_interest: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pmt: Py = Py::None;
    let t2388 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t2388.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2388[0].clone();
    v_i = t2388[1].clone();
    v_n = t2388[2].clone();
    v_pmt = t2388[3].clone();
    v_k = f__int(Py::str("payment_number"), v_payment_number.clone(), Py::Int(1i64), v_n.clone())?;
    v_interest = py::mul(&f__balance(v_P.clone(), v_i.clone(), v_pmt.clone(), py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &v_i.clone())?;
    return Ok(Py::dict(vec![(Py::str("interest_part"), v_interest.clone()), (Py::str("principal_part"), py::sub(&v_pmt.clone(), &v_interest.clone())?), (Py::str("payment"), v_pmt.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_log_elasticity(mut v_prices: Py, mut v_quantities: Py) -> R {
    let mut v_o: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let t2415 = (Py::tuple(vec![f__vec(Py::str("prices"), v_prices.clone(), Py::Int(3i64))?, f__vec(Py::str("quantities"), v_quantities.clone(), Py::Int(3i64))?])).items()?;
    if t2415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2415[0].clone();
    v_q = t2415[1].clone();
    let _ = f__need({ let t2416 = py::len(&v_p.clone())?; Py::Bool({ let t2417 = py::len(&v_q.clone())?; if py::eq(&t2416, &t2417) { true } else { false } }) }, Py::str("one quantity per price"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2418 in py::iter(&(py::add(&v_p.clone(), &v_q.clone())?))? { let mut v_v: Py; 
v_v = t2418;
out.push({ let t2419 = v_v.clone(); Py::Bool({ let t2420 = Py::Int(0i64); if py::gt(&t2419, &t2420)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("prices and quantities must be positive (logarithms)"))?;
    v_o = f_ols_simple({ let mut out = vec![]; for t2434 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2434;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2435 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t2435;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("elasticity"), py::getitem(&v_o.clone(), &Py::str("slope"))?), (Py::str("intercept"), py::getitem(&v_o.clone(), &Py::str("intercept"))?), (Py::str("r_squared"), py::getitem(&v_o.clone(), &Py::str("r_squared"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_market_equilibrium(mut v_demand_intercept: Py, mut v_demand_slope: Py, mut v_supply_intercept: Py, mut v_supply_slope: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let t3011 = (Py::tuple(vec![f__pos(Py::str("demand_intercept"), v_demand_intercept.clone())?, f__pos(Py::str("demand_slope"), v_demand_slope.clone())?])).items()?;
    if t3011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3011[0].clone();
    v_b = t3011[1].clone();
    let t3012 = (Py::tuple(vec![f__fin(Py::str("supply_intercept"), v_supply_intercept.clone())?, f__pos(Py::str("supply_slope"), v_supply_slope.clone())?])).items()?;
    if t3012.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3012[0].clone();
    v_d = t3012[1].clone();
    v_P = py::truediv(&py::sub(&v_a.clone(), &v_c.clone())?, &py::add(&v_b.clone(), &v_d.clone())?)?;
    v_Q = py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_P.clone())?)?;
    let _ = f__need({ let t3017 = { let t3013 = v_P.clone(); Py::Bool({ let t3014 = Py::Int(0i64); if py::gt(&t3013, &t3014)? { true } else { false } }) }; if !t3017.truthy() { t3017 } else { { let t3015 = v_Q.clone(); Py::Bool({ let t3016 = Py::Int(0i64); if py::gt(&t3015, &t3016)? { true } else { false } }) } } }, Py::str("the curves do not cross at a positive price and quantity"))?;
    let t3020 = (Py::tuple(vec![py::truediv(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::mul(&Py::Int(2i64), &v_b.clone())?)?, f__ps(v_c.clone(), v_d.clone(), v_P.clone())?])).items()?;
    if t3020.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cs = t3020[0].clone();
    v_ps = t3020[1].clone();
    return Ok(Py::dict(vec![(Py::str("price"), v_P.clone()), (Py::str("quantity"), v_Q.clone()), (Py::str("consumer_surplus"), v_cs.clone()), (Py::str("producer_surplus"), v_ps.clone()), (Py::str("total_surplus"), py::add(&v_cs.clone(), &v_ps.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_drawdown(mut v_prices: Py) -> R {
    let mut v_mdd: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_p = f__vec(Py::str("prices"), v_prices.clone(), Py::Int(2i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2492 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t2492;
out.push({ let t2493 = v_x.clone(); Py::Bool({ let t2494 = Py::Int(0i64); if py::gt(&t2493, &t2494)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("prices must be positive"))?;
    let t2495 = (Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, Py::Float(0.0)])).items()?;
    if t2495.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_peak = t2495[0].clone();
    v_mdd = t2495[1].clone();
    for t2496 in py::iter(&(v_p.clone()))? {
        v_x = t2496;
        v_peak = py::minmax(&[v_peak.clone(), v_x.clone()], true)?;
        v_mdd = py::minmax(&[v_mdd.clone(), py::sub(&Py::Int(1i64), &py::truediv(&v_x.clone(), &v_peak.clone())?)?], true)?;
    }
    return Ok(Py::dict(vec![(Py::str("max_drawdown"), v_mdd.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_returns(mut v_returns: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = f__returns(Py::str("returns"), v_returns.clone(), Py::Int(1i64), Py::Float(1.0))?;
    v_g = py::sub(&py::math1("exp", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2525 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2525;
out.push(py::math1("log1p", &v_x.clone())?); } Py::list(out) })?, &py::len(&v_r.clone())?)?)?, &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("arithmetic_mean"), f__mean(v_r.clone())?), (Py::str("geometric_mean"), v_g.clone()), (Py::str("cumulative_return"), py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_g.clone())?, &py::len(&v_r.clone())?)?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_variance_two_asset(mut v_sd_1: Py, mut v_sd_2: Py, mut v_correlation: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_w1: Py = Py::None;
    let t2548 = (Py::tuple(vec![f__pos(Py::str("sd_1"), v_sd_1.clone())?, f__pos(Py::str("sd_2"), v_sd_2.clone())?])).items()?;
    if t2548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2548[0].clone();
    v_s2 = t2548[1].clone();
    v_rho = f__prob(Py::str("correlation"), v_correlation.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_den = py::sub(&py::add(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_rho.clone())?, &v_s1.clone())?, &v_s2.clone())?)?;
    let _ = f__need({ let t2552 = v_den.clone(); Py::Bool({ let t2553 = py::mul(&Py::Float(1e-12), &py::add(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?)?; if py::gt(&t2552, &t2553)? { true } else { false } }) }, Py::str("the two assets are perfectly correlated with equal risk: every mix has the same variance"))?;
    v_w1 = py::truediv(&py::sub(&py::mul(&v_s2.clone(), &v_s2.clone())?, &py::mul(&py::mul(&v_rho.clone(), &v_s1.clone())?, &v_s2.clone())?)?, &v_den.clone())?;
    v_var = py::minmax(&[py::add(&py::add(&py::mul(&py::mul(&py::mul(&v_w1.clone(), &v_w1.clone())?, &v_s1.clone())?, &v_s1.clone())?, &py::mul(&py::mul(&py::pow(&py::sub(&Py::Int(1i64), &v_w1.clone())?, &Py::Int(2i64))?, &v_s2.clone())?, &v_s2.clone())?)?, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_w1.clone())?, &py::sub(&Py::Int(1i64), &v_w1.clone())?)?, &v_rho.clone())?, &v_s1.clone())?, &v_s2.clone())?)?, Py::Float(0.0)], true)?;
    return Ok(Py::dict(vec![(Py::str("weight_1"), v_w1.clone()), (Py::str("weight_2"), py::sub(&Py::Int(1i64), &v_w1.clone())?), (Py::str("std_dev"), py::math1("sqrt", &v_var.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mirr(mut v_cash_flows: Py, mut v_finance_rate: Py, mut v_reinvest_rate: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fv_pos: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pv_neg: Py = Py::None;
    let mut v_rr: Py = Py::None;
    v_c = f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?;
    let t2581 = (Py::tuple(vec![f__rate(Py::str("finance_rate"), v_finance_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("reinvest_rate"), v_reinvest_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t2581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t2581[0].clone();
    v_rr = t2581[1].clone();
    let _ = f__need({ let t2588 = Py::Bool(({ let mut out = vec![]; for t2582 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2582;
out.push({ let t2583 = v_x.clone(); Py::Bool({ let t2584 = Py::Int(0i64); if py::gt(&t2583, &t2584)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if !t2588.truthy() { t2588 } else { Py::Bool(({ let mut out = vec![]; for t2585 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2585;
out.push({ let t2586 = v_x.clone(); Py::Bool({ let t2587 = Py::Int(0i64); if py::lt(&t2586, &t2587)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }, Py::str("the cash flows need at least one inflow and one outflow"))?;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_fv_pos = py::sum(&{ let mut out = vec![]; for t2589 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t2590 = (t2589).items()?;
if t2590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2590[0].clone();
v_x = t2590[1].clone();
if ({ let t2591 = v_x.clone(); Py::Bool({ let t2592 = Py::Int(0i64); if py::gt(&t2591, &t2592)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_x.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_rr.clone())?, &py::sub(&v_n.clone(), &v_t.clone())?)?)?); }} Py::list(out) }, None)?;
    v_pv_neg = py::neg(&py::sum(&{ let mut out = vec![]; for t2593 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t2594 = (t2593).items()?;
if t2594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2594[0].clone();
v_x = t2594[1].clone();
if ({ let t2595 = v_x.clone(); Py::Bool({ let t2596 = Py::Int(0i64); if py::lt(&t2595, &t2596)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_x.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_f.clone())?, &v_t.clone())?)?); }} Py::list(out) }, None)?)?;
    return Ok(Py::dict(vec![(Py::str("mirr"), py::sub(&py::pow(&py::truediv(&v_fv_pos.clone(), &v_pv_neg.clone())?, &py::truediv(&Py::Int(1i64), &v_n.clone())?)?, &Py::Int(1i64))?), (Py::str("terminal_value"), v_fv_pos.clone()), (Py::str("pv_outflows"), v_pv_neg.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_money_multiplier(mut v_reserve_ratio: Py, mut v_currency_ratio: Py, mut v_monetary_base: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let t2622 = (Py::tuple(vec![f__prob(Py::str("reserve_ratio"), v_reserve_ratio.clone(), Py::Float(0.0), Py::Float(1.0))?, f__nonneg(Py::str("currency_ratio"), v_currency_ratio.clone())?])).items()?;
    if t2622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t2622[0].clone();
    v_c = t2622[1].clone();
    let _ = f__need({ let t2623 = v_rr.clone(); Py::Bool({ let t2624 = Py::Int(0i64); if py::gt(&t2623, &t2624)? { true } else { false } }) }, Py::str("the reserve ratio must be positive"))?;
    v_m = py::truediv(&py::add(&Py::Int(1i64), &v_c.clone())?, &py::add(&v_rr.clone(), &v_c.clone())?)?;
    v_out = Py::dict(vec![(Py::str("simple_multiplier"), py::truediv(&Py::Int(1i64), &v_rr.clone())?), (Py::str("money_multiplier"), v_m.clone())]);
    if ({ let t2625 = v_monetary_base.clone(); Py::Bool({ let t2626 = Py::None; if !py::is(&t2625, &t2626) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("money_supply"), py::mul(&v_m.clone(), &f__pos(Py::str("monetary_base"), v_monetary_base.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_monopoly(mut v_price_intercept: Py, mut v_price_slope: Py, mut v_marginal_cost: Py, mut v_fixed_cost: Py, mut v_cost_quadratic: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Qc: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_mc: Py = Py::None;
    let t2651 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t2651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2651[0].clone();
    v_B = t2651[1].clone();
    let t2654 = (Py::tuple(vec![f__nonneg(Py::str("marginal_cost"), v_marginal_cost.clone())?, f__nonneg(Py::str("fixed_cost"), v_fixed_cost.clone())?, f__nonneg(Py::str("cost_quadratic"), v_cost_quadratic.clone())?])).items()?;
    if t2654.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2654[0].clone();
    v_F = t2654[1].clone();
    v_e = t2654[2].clone();
    let _ = f__need({ let t2655 = v_A.clone(); Py::Bool({ let t2656 = v_c.clone(); if py::gt(&t2655, &t2656)? { true } else { false } }) }, Py::str("the demand intercept must exceed marginal cost (otherwise nothing is produced)"))?;
    v_Q = py::truediv(&py::sub(&v_A.clone(), &v_c.clone())?, &py::add(&py::mul(&Py::Int(2i64), &v_B.clone())?, &py::mul(&Py::Int(2i64), &v_e.clone())?)?)?;
    v_P = py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &v_Q.clone())?)?;
    v_mc = py::add(&v_c.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_e.clone())?, &v_Q.clone())?)?;
    v_Qc = py::truediv(&py::sub(&v_A.clone(), &v_c.clone())?, &py::add(&v_B.clone(), &py::mul(&Py::Int(2i64), &v_e.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("quantity"), v_Q.clone()), (Py::str("price"), v_P.clone()), (Py::str("profit"), py::sub(&py::mul(&v_P.clone(), &v_Q.clone())?, &py::add(&py::add(&v_F.clone(), &py::mul(&v_c.clone(), &v_Q.clone())?)?, &py::mul(&py::mul(&v_e.clone(), &v_Q.clone())?, &v_Q.clone())?)?)?), (Py::str("lerner_index"), py::truediv(&py::sub(&v_P.clone(), &v_mc.clone())?, &v_P.clone())?), (Py::str("deadweight_loss"), py::truediv(&py::mul(&py::add(&v_B.clone(), &py::mul(&Py::Int(2i64), &v_e.clone())?)?, &py::pow(&py::sub(&v_Qc.clone(), &v_Q.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multiple_regression(mut v_X: Py, mut v_y: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qty: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_ssr: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2681 = (Py::tuple(vec![v_X.clone(), f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t2681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t2681[0].clone();
    v_Y = t2681[1].clone();
    let _ = f__need({ let t2684 = Py::Bool(matches!(v_rows.clone(), Py::List(_) | Py::Tuple(_))); if !t2684.truthy() { t2684 } else { { let t2682 = py::len(&v_rows.clone())?; Py::Bool({ let t2683 = py::len(&v_Y.clone())?; if py::eq(&t2682, &t2683) { true } else { false } }) } } }, Py::str("X needs one row per observation of y"))?;
    v_k = py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?;
    v_n = py::len(&v_Y.clone())?;
    let _ = f__need({ let t2685 = v_n.clone(); Py::Bool({ let t2686 = py::add(&v_k.clone(), &Py::Int(1i64))?; if py::gt(&t2685, &t2686)? { true } else { false } }) }, Py::str("more observations than coefficients are needed (n > k + 1)"))?;
    v_cols = py::add(&Py::list(vec![py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?]), &{ let mut out = vec![]; for t2687 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2687;
out.push({ let mut out = vec![]; for t2688 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t2688;
out.push(f__fin(Py::str("X"), py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) })?;
    let t2690 = (Py::tuple(vec![Py::list(vec![]), { let mut out = vec![]; for t2689 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t2689;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) }])).items()?;
    if t2690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t2690[0].clone();
    v_R = t2690[1].clone();
    for t2691 in py::iter(&(py::enumerate(&v_cols.clone(), 0)?))? {
        let t2703 = (t2691).items()?;
        if t2703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2703[0].clone();
        v_v = t2703[1].clone();
        v_v = Py::list((v_v.clone()).items()?);
        for t2692 in py::iter(&(py::enumerate(&v_Q.clone(), 0)?))? {
            let t2697 = (t2692).items()?;
            if t2697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_i = t2697[0].clone();
            v_q = t2697[1].clone();
            py::setitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone(), py::fsum(&{ let mut out = vec![]; for t2693 in py::iter(&(py::zip(&[v_q.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2694 = (t2693).items()?;
if t2694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2694[0].clone();
v_b = t2694[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?;
            v_v = { let mut out = vec![]; for t2695 in py::iter(&(py::zip(&[v_v.clone(), v_q.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2696 = (t2695).items()?;
if t2696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2696[0].clone();
v_b = t2696[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &v_b.clone())?)?); } Py::list(out) };
        }
        py::setitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2698 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2698;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?)?;
        let _ = f__need({ let t2699 = py::getitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone())?; Py::Bool({ let t2700 = py::mul(&Py::Float(1e-10), &py::minmax(&[Py::Float(1.0), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2701 in py::iter(&(py::getitem(&v_cols.clone(), &v_j.clone())?))? { let mut v_a: Py; 
v_a = t2701;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?], true)?)?; if py::gt(&t2699, &t2700)? { true } else { false } }) }, Py::str("the columns of X are (nearly) collinear"))?;
        let _ = py::method(&v_Q.clone(), "append", &[{ let mut out = vec![]; for t2702 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2702;
out.push(py::truediv(&v_a.clone(), &py::getitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone())?)?); } Py::list(out) }])?;
    }
    v_qty = { let mut out = vec![]; for t2704 in py::iter(&(v_Q.clone()))? { let mut v_q: Py; 
v_q = t2704;
out.push(py::fsum(&{ let mut out = vec![]; for t2705 in py::iter(&(py::zip(&[v_q.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2706 = (t2705).items()?;
if t2706.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2706[0].clone();
v_b = t2706[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_beta = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
    for t2707 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t2707;
        py::setitem(&v_beta.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_qty.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t2708 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2708;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_beta.clone(), &v_j.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    v_my = f__mean(v_Y.clone())?;
    v_sst = py::fsum(&{ let mut out = vec![]; for t2709 in py::iter(&(v_Y.clone()))? { let mut v_v: Py; 
v_v = t2709;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    let _ = f__need({ let t2710 = v_sst.clone(); Py::Bool({ let t2711 = Py::Int(0i64); if py::gt(&t2710, &t2711)? { true } else { false } }) }, Py::str("y must not be constant"))?;
    v_ssr = py::fsum(&{ let mut out = vec![]; for t2712 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t2712;
out.push(py::pow(&py::sub(&py::getitem(&v_Y.clone(), &v_t.clone())?, &py::fsum(&{ let mut out = vec![]; for t2713 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2713;
out.push(py::mul(&py::getitem(&v_beta.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_cols.clone(), &v_j.clone())?, &v_t.clone())?)?); } Py::list(out) })?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_r2 = py::sub(&Py::Int(1i64), &py::truediv(&v_ssr.clone(), &v_sst.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("coefficients"), v_beta.clone()), (Py::str("r_squared"), v_r2.clone()), (Py::str("adj_r_squared"), py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&py::sub(&Py::Int(1i64), &v_r2.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::sub(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_net_premium_whole_life(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_benefit: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2750 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t2750.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2750[0].clone();
    v_v = t2750[1].clone();
    v_B = t2750[2].clone();
    let t2754 = (Py::tuple(vec![f__A(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?, f__a_due(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?])).items()?;
    if t2754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2754[0].clone();
    v_a = t2754[1].clone();
    return Ok(Py::dict(vec![(Py::str("premium"), py::truediv(&py::mul(&v_B.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("insurance_apv"), py::mul(&v_B.clone(), &v_A.clone())?), (Py::str("annuity_due"), v_a.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nominal_rate(mut v_effective_rate: Py, mut v_periods_per_year: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t2790 = (Py::tuple(vec![f__rate(Py::str("effective_rate"), v_effective_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t2790.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t2790[0].clone();
    v_m = t2790[1].clone();
    v_p = py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_e.clone())?, &py::truediv(&Py::Int(1i64), &v_m.clone())?)?, &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("nominal_rate"), py::mul(&v_m.clone(), &v_p.clone())?), (Py::str("periodic_rate"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_npv(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2818 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t2818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2818[0].clone();
    v_c = t2818[1].clone();
    return Ok(Py::dict(vec![(Py::str("npv"), f__hnpv(v_r.clone(), v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_number_of_periods(mut v_present_value: Py, mut v_payment: Py, mut v_rate: Py, mut v_timing: Py) -> R {
    let mut v_due: Py = Py::None;
    let mut v_eff: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2852 = (Py::tuple(vec![f__pos(Py::str("present_value"), v_present_value.clone())?, f__pos(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__timing(v_timing.clone())?])).items()?;
    if t2852.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t2852[0].clone();
    v_p = t2852[1].clone();
    v_r = t2852[2].clone();
    v_due = t2852[3].clone();
    v_eff = (if (v_due.clone()).truthy() { py::mul(&v_p.clone(), &py::add(&Py::Int(1i64), &v_r.clone())?)? } else { v_p.clone() });
    let _ = f__need({ let t2853 = v_eff.clone(); Py::Bool({ let t2854 = py::mul(&v_pv.clone(), &v_r.clone())?; if py::gt(&t2853, &t2854)? { true } else { false } }) }, Py::str("the payment does not even cover the interest: the loan is never repaid"))?;
    return Ok(Py::dict(vec![(Py::str("periods"), py::truediv(&py::neg(&py::math1("log", &py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&v_pv.clone(), &v_r.clone())?, &v_eff.clone())?)?)?)?, &py::math1("log", &py::add(&Py::Int(1i64), &v_r.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ols_simple(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ssr: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let t2879 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t2879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2879[0].clone();
    v_Y = t2879[1].clone();
    let _ = f__need({ let t2880 = py::len(&v_X.clone())?; Py::Bool({ let t2881 = py::len(&v_Y.clone())?; if py::eq(&t2880, &t2881) { true } else { false } }) }, Py::str("x and y need the same number of observations"))?;
    let t2885 = (Py::tuple(vec![f__cov(v_X.clone(), v_X.clone(), Py::Int(1i64))?, f__cov(v_Y.clone(), v_Y.clone(), Py::Int(1i64))?, f__cov(v_X.clone(), v_Y.clone(), Py::Int(1i64))?])).items()?;
    if t2885.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t2885[0].clone();
    v_syy = t2885[1].clone();
    v_sxy = t2885[2].clone();
    let _ = f__need({ let t2886 = v_sxx.clone(); Py::Bool({ let t2887 = Py::Int(0i64); if py::gt(&t2886, &t2887)? { true } else { false } }) }, Py::str("x must not be constant"))?;
    let _ = f__need({ let t2888 = v_syy.clone(); Py::Bool({ let t2889 = Py::Int(0i64); if py::gt(&t2888, &t2889)? { true } else { false } }) }, Py::str("y must not be constant"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&f__mean(v_Y.clone())?, &py::mul(&v_b.clone(), &f__mean(v_X.clone())?)?)?;
    v_ssr = py::fsum(&{ let mut out = vec![]; for t2890 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_xv: Py; let mut v_yv: Py; 
let t2891 = (t2890).items()?;
if t2891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_xv = t2891[0].clone();
v_yv = t2891[1].clone();
out.push(py::pow(&py::sub(&py::sub(&v_yv.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_xv.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_n = py::len(&v_X.clone())?;
    return Ok(Py::dict(vec![(Py::str("slope"), v_b.clone()), (Py::str("intercept"), v_a.clone()), (Py::str("r_squared"), py::sub(&Py::Int(1i64), &py::truediv(&v_ssr.clone(), &py::mul(&v_syy.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?), (Py::str("slope_std_error"), py::math1("sqrt", &py::truediv(&py::truediv(&v_ssr.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::mul(&v_sxx.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_overhead_cost_variances(mut v_variable_overhead_rate: Py, mut v_actual_variable_overhead: Py, mut v_actual_hours: Py, mut v_standard_hours: Py, mut v_budgeted_fixed_overhead: Py, mut v_actual_fixed_overhead: Py, mut v_denominator_hours: Py) -> R {
    let mut v_AF: Py = Py::None;
    let mut v_AH: Py = Py::None;
    let mut v_AV: Py = Py::None;
    let mut v_BF: Py = Py::None;
    let mut v_DH: Py = Py::None;
    let mut v_FR: Py = Py::None;
    let mut v_SH: Py = Py::None;
    let mut v_VR: Py = Py::None;
    let mut v_applied: Py = Py::None;
    let mut v_budget: Py = Py::None;
    let mut v_eff: Py = Py::None;
    let mut v_spend: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_volume: Py = Py::None;
    let t2916 = (f__overhead_inputs(v_variable_overhead_rate.clone(), v_actual_variable_overhead.clone(), v_actual_hours.clone(), v_standard_hours.clone(), v_budgeted_fixed_overhead.clone(), v_actual_fixed_overhead.clone(), v_denominator_hours.clone())?).items()?;
    if t2916.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_VR = t2916[0].clone();
    v_AV = t2916[1].clone();
    v_AH = t2916[2].clone();
    v_SH = t2916[3].clone();
    v_BF = t2916[4].clone();
    v_AF = t2916[5].clone();
    v_DH = t2916[6].clone();
    v_FR = py::truediv(&v_BF.clone(), &v_DH.clone())?;
    v_spend = f__snap(py::sub(&v_AV.clone(), &py::mul(&v_AH.clone(), &v_VR.clone())?)?, py::add(&v_AV.clone(), &py::mul(&v_AH.clone(), &v_VR.clone())?)?)?;
    v_eff = py::mul(&v_VR.clone(), &py::sub(&v_AH.clone(), &v_SH.clone())?)?;
    v_budget = py::sub(&v_AF.clone(), &v_BF.clone())?;
    v_volume = py::truediv(&py::mul(&v_BF.clone(), &py::sub(&v_DH.clone(), &v_SH.clone())?)?, &v_DH.clone())?;
    v_applied = py::add(&py::mul(&v_SH.clone(), &v_VR.clone())?, &py::mul(&v_SH.clone(), &v_FR.clone())?)?;
    v_total = f__snap(py::add(&py::add(&py::add(&v_spend.clone(), &v_eff.clone())?, &v_budget.clone())?, &v_volume.clone())?, py::add(&py::add(&v_AV.clone(), &v_AF.clone())?, &v_applied.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("variable_spending_variance"), v_spend.clone()), (Py::str("variable_efficiency_variance"), v_eff.clone()), (Py::str("fixed_budget_variance"), v_budget.clone()), (Py::str("fixed_volume_variance"), v_volume.clone()), (Py::str("total_overhead_variance"), v_total.clone()), (Py::str("fixed_overhead_rate"), v_FR.clone()), (Py::str("applied_overhead"), v_applied.clone()), (Py::str("variable_spending_type"), f__fav(v_spend.clone())?), (Py::str("variable_efficiency_type"), f__fav(v_eff.clone())?), (Py::str("fixed_budget_type"), f__fav(v_budget.clone())?), (Py::str("fixed_volume_type"), f__fav(v_volume.clone())?), (Py::str("total_type"), f__fav(v_total.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parametric_var(mut v_mean: Py, mut v_std_dev: Py, mut v_confidence: Py, mut v_horizon: Py, mut v_portfolio_value: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_mh: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sh: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2948 = (Py::tuple(vec![f__rate(Py::str("mean"), v_mean.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__pos(Py::str("std_dev"), v_std_dev.clone())?])).items()?;
    if t2948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t2948[0].clone();
    v_s = t2948[1].clone();
    let _ = f__need({ let t2949 = v_s.clone(); Py::Bool({ let t2950 = Py::Int(5i64); if py::lt(&t2949, &t2950)? { true } else { false } }) }, Py::str("std_dev is a decimal (20% = 0.2)"))?;
    let t2954 = (Py::tuple(vec![f__confidence(v_confidence.clone())?, f__pos(Py::str("horizon"), v_horizon.clone())?, f__pos(Py::str("portfolio_value"), v_portfolio_value.clone())?])).items()?;
    if t2954.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2954[0].clone();
    v_h = t2954[1].clone();
    v_V = t2954[2].clone();
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_a.clone()])?;
    let t2955 = (Py::tuple(vec![py::mul(&v_s.clone(), &py::math1("sqrt", &v_h.clone())?)?, py::mul(&v_mu.clone(), &v_h.clone())?])).items()?;
    if t2955.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sh = t2955[0].clone();
    v_mh = t2955[1].clone();
    v_phi = py::truediv(&py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_z.clone())?, &v_z.clone())?, &Py::Int(2i64))?)?, &py::math1("sqrt", &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?;
    return Ok(Py::dict(vec![(Py::str("var"), py::mul(&v_V.clone(), &py::sub(&py::mul(&v_z.clone(), &v_sh.clone())?, &v_mh.clone())?)?), (Py::str("cvar"), py::mul(&v_V.clone(), &py::sub(&py::truediv(&py::mul(&v_sh.clone(), &v_phi.clone())?, &py::sub(&Py::Int(1i64), &v_a.clone())?)?, &v_mh.clone())?)?), (Py::str("z"), v_z.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_payback_period(mut v_cash_flows: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("payback"), f__payback(f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_per_unit_tax(mut v_demand_intercept: Py, mut v_demand_slope: Py, mut v_supply_intercept: Py, mut v_supply_slope: Py, mut v_tax: Py) -> R {
    let mut v_Pb: Py = Py::None;
    let mut v_Ps: Py = Py::None;
    let mut v_Qt: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_e = f_market_equilibrium(v_demand_intercept.clone(), v_demand_slope.clone(), v_supply_intercept.clone(), v_supply_slope.clone())?;
    v_t = f__pos(Py::str("tax"), v_tax.clone())?;
    let t3021 = (Py::tuple(vec![v_demand_intercept.clone(), v_demand_slope.clone(), v_supply_intercept.clone(), v_supply_slope.clone()])).items()?;
    if t3021.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3021[0].clone();
    v_b = t3021[1].clone();
    v_c = t3021[2].clone();
    v_d = t3021[3].clone();
    v_Pb = py::truediv(&py::add(&py::sub(&v_a.clone(), &v_c.clone())?, &py::mul(&v_d.clone(), &v_t.clone())?)?, &py::add(&v_b.clone(), &v_d.clone())?)?;
    let t3022 = (Py::tuple(vec![py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_Pb.clone())?)?, py::sub(&v_Pb.clone(), &v_t.clone())?])).items()?;
    if t3022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Qt = t3022[0].clone();
    v_Ps = t3022[1].clone();
    let _ = f__need({ let t3027 = { let t3023 = v_Qt.clone(); Py::Bool({ let t3024 = Py::Int(0i64); if py::gt(&t3023, &t3024)? { true } else { false } }) }; if !t3027.truthy() { t3027 } else { { let t3025 = v_Ps.clone(); Py::Bool({ let t3026 = Py::Int(0i64); if py::gt(&t3025, &t3026)? { true } else { false } }) } } }, Py::str("the tax is so large that the market closes (no positive quantity or seller price)"))?;
    return Ok(Py::dict(vec![(Py::str("quantity"), v_Qt.clone()), (Py::str("buyer_price"), v_Pb.clone()), (Py::str("seller_price"), v_Ps.clone()), (Py::str("tax_revenue"), py::mul(&v_t.clone(), &v_Qt.clone())?), (Py::str("deadweight_loss"), py::truediv(&py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_e.clone(), &Py::str("quantity"))?, &v_Qt.clone())?)?, &Py::Int(2i64))?), (Py::str("buyer_share"), py::truediv(&py::sub(&v_Pb.clone(), &py::getitem(&v_e.clone(), &Py::str("price"))?)?, &v_t.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_perpetuity(mut v_payment: Py, mut v_rate: Py, mut v_timing: Py) -> R {
    let mut v_due: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3055 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__timing(v_timing.clone())?])).items()?;
    if t3055.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3055[0].clone();
    v_r = t3055[1].clone();
    v_due = t3055[2].clone();
    return Ok(Py::dict(vec![(Py::str("present_value"), py::mul(&py::truediv(&v_p.clone(), &v_r.clone())?, &(if (v_due.clone()).truthy() { py::add(&Py::Int(1i64), &v_r.clone())? } else { Py::Int(1i64) }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_elasticity(mut v_demand_intercept: Py, mut v_demand_slope: Py, mut v_price: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t3078 = (Py::tuple(vec![f__pos(Py::str("demand_intercept"), v_demand_intercept.clone())?, f__pos(Py::str("demand_slope"), v_demand_slope.clone())?, f__pos(Py::str("price"), v_price.clone())?])).items()?;
    if t3078.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3078[0].clone();
    v_b = t3078[1].clone();
    v_P = t3078[2].clone();
    v_Q = py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_P.clone())?)?;
    let _ = f__need({ let t3079 = v_Q.clone(); Py::Bool({ let t3080 = Py::Int(0i64); if py::gt(&t3079, &t3080)? { true } else { false } }) }, Py::str("the price is at or above the choke price a/b: nothing is demanded"))?;
    return Ok(Py::dict(vec![(Py::str("quantity"), v_Q.clone()), (Py::str("elasticity"), py::truediv(&py::mul(&py::neg(&v_b.clone())?, &v_P.clone())?, &v_Q.clone())?), (Py::str("revenue"), py::mul(&v_P.clone(), &v_Q.clone())?), (Py::str("revenue_maximizing_price"), py::truediv(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_b.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_portfolio_return_risk(mut v_weights: Py, mut v_expected_returns: Py, mut v_covariance: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64))?;
    v_mu = f__returns(Py::str("expected_returns"), v_expected_returns.clone(), Py::Int(1i64), Py::Float(1.0))?;
    let _ = f__need({ let t3109 = py::len(&v_w.clone())?; Py::Bool({ let t3110 = py::len(&v_mu.clone())?; if py::eq(&t3109, &t3110) { true } else { false } }) }, Py::str("weights and expected_returns need one entry per asset"))?;
    v_S = f__square(Py::str("covariance"), v_covariance.clone(), py::len(&v_w.clone())?)?;
    let _ = f__need({ let t3136 = py::abs(&py::sub(&py::fsum(&v_w.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t3137 = Py::Float(1e-09); if py::le(&t3136, &t3137)? { true } else { false } }) }, Py::str("the weights must add up to 1"))?;
    v_var = f__quad(v_w.clone(), v_S.clone())?;
    let _ = f__need({ let t3140 = v_var.clone(); Py::Bool({ let t3141 = py::neg(&Py::Float(1e-15))?; if py::ge(&t3140, &t3141)? { true } else { false } }) }, Py::str("the covariance matrix is not positive semidefinite (negative portfolio variance)"))?;
    v_var = py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?;
    return Ok(Py::dict(vec![(Py::str("expected_return"), py::fsum(&{ let mut out = vec![]; for t3142 in py::iter(&(py::zip(&[v_w.clone(), v_mu.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3143 = (t3142).items()?;
if t3143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3143[0].clone();
v_b = t3143[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?), (Py::str("variance"), v_var.clone()), (Py::str("std_dev"), py::math1("sqrt", &v_var.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_portfolio_var(mut v_weights: Py, mut v_covariance: Py, mut v_confidence: Py, mut v_portfolio_value: Py, mut v_horizon: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_undiv: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_w = f__vec(Py::str("weights"), v_weights.clone(), Py::Int(1i64))?;
    v_S = f__square(Py::str("covariance"), v_covariance.clone(), py::len(&v_w.clone())?)?;
    let t3198 = (Py::tuple(vec![f__confidence(v_confidence.clone())?, f__pos(Py::str("portfolio_value"), v_portfolio_value.clone())?, f__pos(Py::str("horizon"), v_horizon.clone())?])).items()?;
    if t3198.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3198[0].clone();
    v_V = t3198[1].clone();
    v_h = t3198[2].clone();
    v_var = f__quad(v_w.clone(), v_S.clone())?;
    let _ = f__need({ let t3201 = v_var.clone(); Py::Bool({ let t3202 = py::neg(&Py::Float(1e-15))?; if py::ge(&t3201, &t3202)? { true } else { false } }) }, Py::str("the covariance matrix is not positive semidefinite"))?;
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_a.clone()])?;
    v_sd = py::math1("sqrt", &py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?)?;
    v_undiv = py::mul(&py::mul(&py::mul(&v_z.clone(), &py::math1("sqrt", &v_h.clone())?)?, &v_V.clone())?, &py::fsum(&{ let mut out = vec![]; for t3203 in py::iter(&(py::enumerate(&v_w.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t3204 = (t3203).items()?;
if t3204.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t3204[0].clone();
v_x = t3204[1].clone();
out.push(py::mul(&py::abs(&v_x.clone())?, &py::math1("sqrt", &py::getitem(&py::getitem(&v_S.clone(), &v_i.clone())?, &v_i.clone())?)?)?); } Py::list(out) })?)?;
    v_pv = py::mul(&py::mul(&py::mul(&v_z.clone(), &v_sd.clone())?, &py::math1("sqrt", &v_h.clone())?)?, &v_V.clone())?;
    return Ok(Py::dict(vec![(Py::str("var"), v_pv.clone()), (Py::str("portfolio_sd"), v_sd.clone()), (Py::str("undiversified_var"), v_undiv.clone()), (Py::str("diversification_benefit"), py::sub(&v_undiv.clone(), &v_pv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_present_value(mut v_future_value: Py, mut v_rate: Py, mut v_periods: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3230 = (Py::tuple(vec![f__fin(Py::str("future_value"), v_future_value.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("periods"), v_periods.clone())?])).items()?;
    if t3230.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3230[0].clone();
    v_r = t3230[1].clone();
    v_n = t3230[2].clone();
    v_d = py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &py::neg(&v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("present_value"), py::mul(&v_F.clone(), &v_d.clone())?), (Py::str("discount_factor"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_present_value_annuity(mut v_payment: Py, mut v_rate: Py, mut v_periods: Py, mut v_timing: Py) -> R {
    let mut v_due: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3270 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t3270.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3270[0].clone();
    v_r = t3270[1].clone();
    v_n = t3270[2].clone();
    v_due = t3270[3].clone();
    return Ok(Py::dict(vec![(Py::str("present_value"), f__annuity_pv(v_p.clone(), v_r.clone(), v_n.clone(), v_due.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_price_indices(mut v_base_prices: Py, mut v_base_quantities: Py, mut v_current_prices: Py, mut v_current_quantities: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_q1: Py = Py::None;
    let t3316 = (f__baskets(v_base_prices.clone(), v_base_quantities.clone(), v_current_prices.clone(), v_current_quantities.clone())?).items()?;
    if t3316.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t3316[0].clone();
    v_q0 = t3316[1].clone();
    v_p1 = t3316[2].clone();
    v_q1 = t3316[3].clone();
    let t3319 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(100i64), &f__dot(v_p1.clone(), v_q0.clone())?)?, &f__dot(v_p0.clone(), v_q0.clone())?)?, py::truediv(&py::mul(&Py::Int(100i64), &f__dot(v_p1.clone(), v_q1.clone())?)?, &f__dot(v_p0.clone(), v_q1.clone())?)?])).items()?;
    if t3319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t3319[0].clone();
    v_P = t3319[1].clone();
    return Ok(Py::dict(vec![(Py::str("laspeyres"), v_L.clone()), (Py::str("paasche"), v_P.clone()), (Py::str("fisher"), py::math1("sqrt", &py::mul(&v_L.clone(), &v_P.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_profitability_index(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3347 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t3347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3347[0].clone();
    v_c = t3347[1].clone();
    let _ = f__need({ let t3348 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t3349 = Py::Int(0i64); if py::lt(&t3348, &t3349)? { true } else { false } }) }, Py::str("the first cash flow (the investment at time 0) must be negative"))?;
    v_pv = f__hnpv(v_r.clone(), py::add(&Py::list(vec![Py::Float(0.0)]), &py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?)?)?;
    return Ok(Py::dict(vec![(Py::str("profitability_index"), py::truediv(&v_pv.clone(), &py::neg(&py::getitem(&v_c.clone(), &Py::Int(0i64))?)?)?), (Py::str("pv_future"), v_pv.clone()), (Py::str("npv"), py::add(&v_pv.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_put_call_parity(mut v_call_price: Py, mut v_spot: Py, mut v_strike: Py, mut v_time: Py, mut v_rate: Py, mut v_dividend_yield: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_Kr: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_Sq: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v__s: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3381 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), Py::Float(0.2), v_dividend_yield.clone())?).items()?;
    if t3381.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3381[0].clone();
    v_K = t3381[1].clone();
    v_T = t3381[2].clone();
    v_r = t3381[3].clone();
    v__s = t3381[4].clone();
    v_q = t3381[5].clone();
    v_C = f__pos(Py::str("call_price"), v_call_price.clone())?;
    let t3382 = (Py::tuple(vec![py::mul(&v_S.clone(), &py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?)?, py::mul(&v_K.clone(), &py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_T.clone())?)?)?])).items()?;
    if t3382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Sq = t3382[0].clone();
    v_Kr = t3382[1].clone();
    let _ = f__need({ let t3383 = py::minmax(&[Py::Float(0.0), py::sub(&v_Sq.clone(), &v_Kr.clone())?], true)?; Py::Bool({ let t3384 = v_C.clone(); if py::lt(&t3383, &t3384)? { { let t3385 = v_Sq.clone(); if py::lt(&t3384, &t3385)? { true } else { false } } } else { false } }) }, Py::str("the call price violates the no-arbitrage bounds max(0, S e^-qT - K e^-rT) < C < S e^-qT"))?;
    return Ok(Py::dict(vec![(Py::str("put_price"), py::add(&py::sub(&v_C.clone(), &v_Sq.clone())?, &v_Kr.clone())?), (Py::str("pv_strike"), v_Kr.clone()), (Py::str("pv_spot"), v_Sq.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scenario_return(mut v_probabilities: Py, mut v_returns: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_var: Py = Py::None;
    let t3410 = (Py::tuple(vec![f__vec(Py::str("probabilities"), v_probabilities.clone(), Py::Int(1i64))?, f__vec(Py::str("returns"), v_returns.clone(), Py::Int(1i64))?])).items()?;
    if t3410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3410[0].clone();
    v_r = t3410[1].clone();
    let _ = f__need({ let t3411 = py::len(&v_p.clone())?; Py::Bool({ let t3412 = py::len(&v_r.clone())?; if py::eq(&t3411, &t3412) { true } else { false } }) }, Py::str("one probability per scenario return"))?;
    let _ = f__need({ let t3418 = Py::Bool(({ let mut out = vec![]; for t3413 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t3413;
out.push({ let t3414 = v_x.clone(); Py::Bool({ let t3415 = Py::Int(0i64); if py::ge(&t3414, &t3415)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t3418.truthy() { t3418 } else { { let t3416 = py::abs(&py::sub(&py::fsum(&v_p.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t3417 = Py::Float(1e-09); if py::le(&t3416, &t3417)? { true } else { false } }) } } }, Py::str("the probabilities must be non-negative and add up to 1"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3419 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3419;
out.push({ let t3420 = v_x.clone(); Py::Bool({ let t3421 = py::neg(&Py::Int(1i64))?; if py::gt(&t3420, &t3421)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a return cannot be below -100%"))?;
    v_e = py::fsum(&{ let mut out = vec![]; for t3422 in py::iter(&(py::zip(&[v_p.clone(), v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3423 = (t3422).items()?;
if t3423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3423[0].clone();
v_b = t3423[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    v_var = py::fsum(&{ let mut out = vec![]; for t3424 in py::iter(&(py::zip(&[v_p.clone(), v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3425 = (t3424).items()?;
if t3425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3425[0].clone();
v_b = t3425[1].clone();
out.push(py::mul(&v_a.clone(), &py::pow(&py::sub(&v_b.clone(), &v_e.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("expected_return"), v_e.clone()), (Py::str("variance"), v_var.clone()), (Py::str("std_dev"), py::math1("sqrt", &v_var.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sharpe_ratio(mut v_returns: Py, mut v_risk_free: Py, mut v_periods_per_year: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rf: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sh: Py = Py::None;
    v_r = f__returns(Py::str("returns"), v_returns.clone(), Py::Int(2i64), Py::Float(1.0))?;
    v_rf = f__rate(Py::str("risk_free"), v_risk_free.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_m = f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?;
    v_s = f__sd(v_r.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3472 = v_s.clone(); Py::Bool({ let t3473 = Py::Int(0i64); if py::gt(&t3472, &t3473)? { true } else { false } }) }, Py::str("the returns must not all be equal (zero standard deviation)"))?;
    v_sh = py::truediv(&py::sub(&f__mean(v_r.clone())?, &v_rf.clone())?, &v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("sharpe"), v_sh.clone()), (Py::str("annualized_sharpe"), py::mul(&v_sh.clone(), &py::math1("sqrt", &v_m.clone())?)?), (Py::str("mean_excess_return"), py::sub(&f__mean(v_r.clone())?, &v_rf.clone())?), (Py::str("std_dev"), v_s.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_simple_interest(mut v_principal: Py, mut v_rate: Py, mut v_years: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t3501 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t3501.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t3501[0].clone();
    v_r = t3501[1].clone();
    v_t = t3501[2].clone();
    v_I = py::mul(&py::mul(&v_P.clone(), &v_r.clone())?, &v_t.clone())?;
    return Ok(Py::dict(vec![(Py::str("interest"), v_I.clone()), (Py::str("amount"), py::add(&v_P.clone(), &v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sinking_fund(mut v_future_value: Py, mut v_rate: Py, mut v_periods: Py, mut v_timing: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_due: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3543 = (Py::tuple(vec![f__pos(Py::str("future_value"), v_future_value.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t3543.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3543[0].clone();
    v_r = t3543[1].clone();
    v_n = t3543[2].clone();
    v_due = t3543[3].clone();
    return Ok(Py::dict(vec![(Py::str("payment"), py::truediv(&v_F.clone(), &f__annuity_fv(Py::Float(1.0), v_r.clone(), v_n.clone(), v_due.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solow_steady_state(mut v_saving_rate: Py, mut v_population_growth: Py, mut v_depreciation: Py, mut v_alpha: Py, mut v_technology_growth: Py, mut v_productivity: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_br: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t3566 = (Py::tuple(vec![f__fin(Py::str("saving_rate"), v_saving_rate.clone())?, f__fin(Py::str("alpha"), v_alpha.clone())?])).items()?;
    if t3566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3566[0].clone();
    v_a = t3566[1].clone();
    let _ = f__need({ let t3573 = { let t3567 = Py::Int(0i64); Py::Bool({ let t3568 = v_s.clone(); if py::lt(&t3567, &t3568)? { { let t3569 = Py::Int(1i64); if py::lt(&t3568, &t3569)? { true } else { false } } } else { false } }) }; if !t3573.truthy() { t3573 } else { { let t3570 = Py::Int(0i64); Py::Bool({ let t3571 = v_a.clone(); if py::lt(&t3570, &t3571)? { { let t3572 = Py::Int(1i64); if py::lt(&t3571, &t3572)? { true } else { false } } } else { false } }) } } }, Py::str("saving_rate and alpha are decimals between 0 and 1"))?;
    let t3577 = (Py::tuple(vec![f__rate(Py::str("population_growth"), v_population_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("depreciation"), v_depreciation.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("technology_growth"), v_technology_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3577.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3577[0].clone();
    v_d = t3577[1].clone();
    v_g = t3577[2].clone();
    v_A = f__pos(Py::str("productivity"), v_productivity.clone())?;
    v_br = py::add(&py::add(&v_n.clone(), &v_g.clone())?, &v_d.clone())?;
    let _ = f__need({ let t3580 = v_br.clone(); Py::Bool({ let t3581 = Py::Int(0i64); if py::gt(&t3580, &t3581)? { true } else { false } }) }, Py::str("population growth + technology growth + depreciation must be positive"))?;
    v_k = py::pow(&py::truediv(&py::mul(&v_s.clone(), &v_A.clone())?, &v_br.clone())?, &py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_a.clone())?)?)?;
    v_y = py::mul(&v_A.clone(), &py::pow(&v_k.clone(), &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("capital"), v_k.clone()), (Py::str("output"), v_y.clone()), (Py::str("consumption"), py::mul(&py::sub(&Py::Int(1i64), &v_s.clone())?, &v_y.clone())?), (Py::str("golden_rule_saving"), v_a.clone()), (Py::str("golden_rule_capital"), py::pow(&py::truediv(&py::mul(&v_a.clone(), &v_A.clone())?, &v_br.clone())?, &py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_a.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sortino_ratio(mut v_returns: Py, mut v_target: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = f__returns(Py::str("returns"), v_returns.clone(), Py::Int(2i64), Py::Float(1.0))?;
    v_T = f__rate(Py::str("target"), v_target.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_dd = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3613 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3613;
out.push(py::pow(&py::minmax(&[Py::Float(0.0), py::sub(&v_x.clone(), &v_T.clone())?], false)?, &Py::Int(2i64))?); } Py::list(out) })?, &py::len(&v_r.clone())?)?)?;
    let _ = f__need({ let t3614 = v_dd.clone(); Py::Bool({ let t3615 = Py::Int(0i64); if py::gt(&t3614, &t3615)? { true } else { false } }) }, Py::str("no return is below the target: the downside deviation is zero"))?;
    return Ok(Py::dict(vec![(Py::str("sortino"), py::truediv(&py::sub(&f__mean(v_r.clone())?, &v_T.clone())?, &v_dd.clone())?), (Py::str("downside_deviation"), v_dd.clone()), (Py::str("mean_return"), f__mean(v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spending_multiplier(mut v_mpc: Py, mut v_tax_rate: Py, mut v_import_propensity: Py, mut v_spending_change: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_b = f__fin(Py::str("mpc"), v_mpc.clone())?;
    let _ = f__need({ let t3636 = Py::Int(0i64); Py::Bool({ let t3637 = v_b.clone(); if py::lt(&t3636, &t3637)? { { let t3638 = Py::Int(1i64); if py::lt(&t3637, &t3638)? { true } else { false } } } else { false } }) }, Py::str("the marginal propensity to consume is between 0 and 1"))?;
    let t3642 = (Py::tuple(vec![f__prob(Py::str("tax_rate"), v_tax_rate.clone(), Py::Int(0i64), Py::Float(0.99))?, f__prob(Py::str("import_propensity"), v_import_propensity.clone(), Py::Int(0i64), Py::Float(0.99))?])).items()?;
    if t3642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t3642[0].clone();
    v_m = t3642[1].clone();
    v_rho = py::sub(&py::mul(&v_b.clone(), &py::sub(&Py::Int(1i64), &v_t.clone())?)?, &v_m.clone())?;
    let _ = f__need({ let t3643 = Py::Int(0i64); Py::Bool({ let t3644 = v_rho.clone(); if py::le(&t3643, &t3644)? { { let t3645 = Py::Int(1i64); if py::lt(&t3644, &t3645)? { true } else { false } } } else { false } }) }, Py::str("the induced-spending ratio mpc(1 - t) - m must lie in [0, 1)"))?;
    v_k = py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_out = Py::dict(vec![(Py::str("spending_multiplier"), v_k.clone()), (Py::str("tax_multiplier"), py::mul(&py::neg(&v_b.clone())?, &v_k.clone())?)]);
    if ({ let t3646 = v_spending_change.clone(); Py::Bool({ let t3647 = Py::None; if !py::is(&t3646, &t3647) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("output_change"), py::mul(&v_k.clone(), &f__fin(Py::str("spending_change"), v_spending_change.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stackelberg(mut v_price_intercept: Py, mut v_price_slope: Py, mut v_cost_leader: Py, mut v_cost_follower: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_qf: Py = Py::None;
    let mut v_ql: Py = Py::None;
    let t3670 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t3670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3670[0].clone();
    v_B = t3670[1].clone();
    let t3673 = (Py::tuple(vec![f__nonneg(Py::str("cost_leader"), v_cost_leader.clone())?, f__nonneg(Py::str("cost_follower"), v_cost_follower.clone())?])).items()?;
    if t3673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cl = t3673[0].clone();
    v_cf = t3673[1].clone();
    v_ql = py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_cl.clone())?)?, &v_cf.clone())?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    v_qf = py::truediv(&py::sub(&py::sub(&v_A.clone(), &v_cf.clone())?, &py::mul(&v_B.clone(), &v_ql.clone())?)?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    let _ = f__need({ let t3678 = { let t3674 = v_ql.clone(); Py::Bool({ let t3675 = Py::Int(0i64); if py::gt(&t3674, &t3675)? { true } else { false } }) }; if !t3678.truthy() { t3678 } else { { let t3676 = v_qf.clone(); Py::Bool({ let t3677 = Py::Int(0i64); if py::gt(&t3676, &t3677)? { true } else { false } }) } } }, Py::str("one firm would not produce: no interior Stackelberg equilibrium"))?;
    v_P = py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &py::add(&v_ql.clone(), &v_qf.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("q_leader"), v_ql.clone()), (Py::str("q_follower"), v_qf.clone()), (Py::str("price"), v_P.clone()), (Py::str("profit_leader"), py::mul(&py::sub(&v_P.clone(), &v_cl.clone())?, &v_ql.clone())?), (Py::str("profit_follower"), py::mul(&py::sub(&v_P.clone(), &v_cf.clone())?, &v_qf.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_survival_probability(mut v_mortality_rates: Py, mut v_years: Py, mut v_deferred_years: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_q = f__qtab(v_mortality_rates.clone(), Py::Bool(false))?;
    v_t = f__int(Py::str("years"), v_years.clone(), Py::Int(0i64), py::len(&v_q.clone())?)?;
    v_u = (if ({ let t3722 = v_t.clone(); Py::Bool({ let t3723 = py::len(&v_q.clone())?; if py::lt(&t3722, &t3723)? { true } else { false } }) }).truthy() { f__int(Py::str("deferred_years"), v_deferred_years.clone(), Py::Int(1i64), py::sub(&py::len(&v_q.clone())?, &v_t.clone())?)? } else { f__int(Py::str("deferred_years"), v_deferred_years.clone(), Py::Int(1i64), Py::Int(0i64))? });
    v_p = f__surv(v_q.clone())?;
    return Ok(Py::dict(vec![(Py::str("survival"), py::getitem(&v_p.clone(), &v_t.clone())?), (Py::str("death_probability"), py::sub(&Py::Int(1i64), &py::getitem(&v_p.clone(), &v_t.clone())?)?), (Py::str("deferred_death_probability"), py::sub(&py::getitem(&v_p.clone(), &v_t.clone())?, &py::getitem(&v_p.clone(), &py::add(&v_t.clone(), &v_u.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tangency_portfolio(mut v_expected_returns: Py, mut v_covariance: Py, mut v_risk_free: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_er: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_rf: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_mu = f__returns(Py::str("expected_returns"), v_expected_returns.clone(), Py::Int(1i64), Py::Float(1.0))?;
    v_S = f__square(Py::str("covariance"), v_covariance.clone(), py::len(&v_mu.clone())?)?;
    v_rf = f__rate(Py::str("risk_free"), v_risk_free.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_L = f__cholesky(v_S.clone())?;
    let _ = f__need({ let t3790 = v_L.clone(); Py::Bool({ let t3791 = Py::None; if !py::is(&t3790, &t3791) { true } else { false } }) }, Py::str("the covariance matrix must be positive definite"))?;
    v_z = f__chol_solve(v_L.clone(), { let mut out = vec![]; for t3796 in py::iter(&(v_mu.clone()))? { let mut v_m: Py; 
v_m = t3796;
out.push(py::sub(&v_m.clone(), &v_rf.clone())?); } Py::list(out) })?;
    v_tot = py::fsum(&v_z.clone())?;
    let _ = f__need({ let t3797 = v_tot.clone(); Py::Bool({ let t3798 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3799 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t3799;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3797, &t3798)? { true } else { false } }) }, Py::str("no portfolio of positive total weight has the highest Sharpe ratio (the excess returns do not favour a long position)"))?;
    v_w = { let mut out = vec![]; for t3800 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t3800;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    let t3805 = (Py::tuple(vec![py::fsum(&{ let mut out = vec![]; for t3801 in py::iter(&(py::zip(&[v_w.clone(), v_mu.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3802 = (t3801).items()?;
if t3802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3802[0].clone();
v_b = t3802[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, py::math1("sqrt", &py::minmax(&[f__quad(v_w.clone(), v_S.clone())?, Py::Float(0.0)], true)?)?])).items()?;
    if t3805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_er = t3805[0].clone();
    v_sd = t3805[1].clone();
    return Ok(Py::dict(vec![(Py::str("weights"), v_w.clone()), (Py::str("expected_return"), v_er.clone()), (Py::str("std_dev"), v_sd.clone()), (Py::str("sharpe"), py::truediv(&py::sub(&v_er.clone(), &v_rf.clone())?, &v_sd.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_temporary_annuity_due(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_term: Py, mut v_payment: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3842 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("payment"), v_payment.clone())?])).items()?;
    if t3842.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3842[0].clone();
    v_v = t3842[1].clone();
    v_P = t3842[2].clone();
    v_n = f__int(Py::str("term"), v_term.clone(), Py::Int(1i64), py::len(&v_q.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("apv"), py::mul(&v_P.clone(), &f__a_due(v_q.clone(), v_v.clone(), v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_term_insurance(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_term: Py, mut v_benefit: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3893 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t3893.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3893[0].clone();
    v_v = t3893[1].clone();
    v_B = t3893[2].clone();
    v_n = f__int(Py::str("term"), v_term.clone(), Py::Int(1i64), py::len(&v_q.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("apv"), py::mul(&v_B.clone(), &f__A(v_q.clone(), v_v.clone(), v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_treynor_jensen(mut v_portfolio_returns: Py, mut v_market_returns: Py, mut v_risk_free: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mm: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rf: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let t3939 = (f__pair(v_portfolio_returns.clone(), v_market_returns.clone(), Py::str("portfolio_returns"), Py::str("market_returns"), Py::Int(3i64))?).items()?;
    if t3939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3939[0].clone();
    v_m = t3939[1].clone();
    v_rf = f__rate(Py::str("risk_free"), v_risk_free.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_sxx = f__cov(v_m.clone(), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3946 = v_sxx.clone(); Py::Bool({ let t3947 = Py::Int(0i64); if py::gt(&t3946, &t3947)? { true } else { false } }) }, Py::str("the market returns must not all be equal"))?;
    v_b = py::truediv(&f__cov(v_p.clone(), v_m.clone(), Py::Int(1i64))?, &v_sxx.clone())?;
    let _ = f__need({ let t3948 = py::abs(&v_b.clone())?; Py::Bool({ let t3949 = Py::Float(1e-09); if py::gt(&t3948, &t3949)? { true } else { false } }) }, Py::str("beta is zero: the Treynor ratio is undefined"))?;
    let t3950 = (Py::tuple(vec![f__mean(v_p.clone())?, f__mean(v_m.clone())?])).items()?;
    if t3950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mp = t3950[0].clone();
    v_mm = t3950[1].clone();
    return Ok(Py::dict(vec![(Py::str("beta"), v_b.clone()), (Py::str("treynor"), py::truediv(&py::sub(&v_mp.clone(), &v_rf.clone())?, &v_b.clone())?), (Py::str("jensen_alpha"), py::sub(&v_mp.clone(), &py::add(&v_rf.clone(), &py::mul(&v_b.clone(), &py::sub(&v_mm.clone(), &v_rf.clone())?)?)?)?), (Py::str("mean_portfolio_return"), v_mp.clone()), (Py::str("mean_market_return"), v_mm.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_asset_portfolio(mut v_weight_1: Py, mut v_return_1: Py, mut v_return_2: Py, mut v_sd_1: Py, mut v_sd_2: Py, mut v_correlation: Py) -> R {
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = f__fin(Py::str("weight_1"), v_weight_1.clone())?;
    let t3979 = (Py::tuple(vec![py::getitem(&f__returns(Py::str("return_1"), Py::list(vec![v_return_1.clone()]), Py::Int(1i64), Py::Float(1.0))?, &Py::Int(0i64))?, py::getitem(&f__returns(Py::str("return_2"), Py::list(vec![v_return_2.clone()]), Py::Int(1i64), Py::Float(1.0))?, &Py::Int(0i64))?])).items()?;
    if t3979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t3979[0].clone();
    v_r2 = t3979[1].clone();
    let t3982 = (Py::tuple(vec![f__nonneg(Py::str("sd_1"), v_sd_1.clone())?, f__nonneg(Py::str("sd_2"), v_sd_2.clone())?])).items()?;
    if t3982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t3982[0].clone();
    v_s2 = t3982[1].clone();
    v_rho = f__prob(Py::str("correlation"), v_correlation.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_var = py::add(&py::add(&py::mul(&py::mul(&py::mul(&v_w.clone(), &v_w.clone())?, &v_s1.clone())?, &v_s1.clone())?, &py::mul(&py::mul(&py::pow(&py::sub(&Py::Int(1i64), &v_w.clone())?, &Py::Int(2i64))?, &v_s2.clone())?, &v_s2.clone())?)?, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_w.clone())?, &py::sub(&Py::Int(1i64), &v_w.clone())?)?, &v_rho.clone())?, &v_s1.clone())?, &v_s2.clone())?)?;
    v_var = py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?;
    return Ok(Py::dict(vec![(Py::str("expected_return"), py::add(&py::mul(&v_w.clone(), &v_r1.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_w.clone())?, &v_r2.clone())?)?), (Py::str("std_dev"), py::math1("sqrt", &v_var.clone())?), (Py::str("variance"), v_var.clone()), (Py::str("weight_2"), py::sub(&Py::Int(1i64), &v_w.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_stage_ddm(mut v_dividend: Py, mut v_high_growth: Py, mut v_high_growth_years: Py, mut v_stable_growth: Py, mut v_required_return: Py) -> R {
    let mut v_D0: Py = Py::None;
    let mut v_g1: Py = Py::None;
    let mut v_g2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pv1: Py = Py::None;
    let mut v_pvt: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tv: Py = Py::None;
    v_D0 = f__pos(Py::str("dividend"), v_dividend.clone())?;
    let t4011 = (Py::tuple(vec![f__rate(Py::str("high_growth"), v_high_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("stable_growth"), v_stable_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t4011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g1 = t4011[0].clone();
    v_g2 = t4011[1].clone();
    v_n = f__int(Py::str("high_growth_years"), v_high_growth_years.clone(), Py::Int(1i64), Py::Int(200i64))?;
    v_r = f__rate(Py::str("required_return"), v_required_return.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let _ = f__need({ let t4024 = v_r.clone(); Py::Bool({ let t4025 = v_g2.clone(); if py::gt(&t4024, &t4025)? { true } else { false } }) }, Py::str("the required return must exceed the stable growth rate"))?;
    v_pv1 = py::fsum(&{ let mut out = vec![]; for t4026 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t4026;
out.push(py::truediv(&py::mul(&v_D0.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_g1.clone())?, &v_k.clone())?)?, &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_k.clone())?)?); } Py::list(out) })?;
    v_tv = py::truediv(&py::mul(&py::mul(&v_D0.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_g1.clone())?, &v_n.clone())?)?, &py::add(&Py::Int(1i64), &v_g2.clone())?)?, &py::sub(&v_r.clone(), &v_g2.clone())?)?;
    v_pvt = py::truediv(&v_tv.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("price"), py::add(&v_pv1.clone(), &v_pvt.clone())?), (Py::str("terminal_value"), v_tv.clone()), (Py::str("pv_terminal"), v_pvt.clone()), (Py::str("pv_high_growth_dividends"), v_pv1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unemployment_rate(mut v_employed: Py, mut v_unemployed: Py, mut v_working_age_population: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_LF: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_nilf: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_E = f__nonneg(Py::str("employed"), v_employed.clone())?;
    v_U = f__nonneg(Py::str("unemployed"), v_unemployed.clone())?;
    v_LF = py::add(&v_E.clone(), &v_U.clone())?;
    let _ = f__need({ let t4049 = v_LF.clone(); Py::Bool({ let t4050 = Py::Int(0i64); if py::gt(&t4049, &t4050)? { true } else { false } }) }, Py::str("the labour force (employed plus unemployed) must be positive"))?;
    v_out = Py::dict(vec![(Py::str("labour_force"), v_LF.clone()), (Py::str("unemployment_rate"), py::truediv(&v_U.clone(), &v_LF.clone())?)]);
    if ({ let t4051 = v_working_age_population.clone(); Py::Bool({ let t4052 = Py::None; if !py::is(&t4051, &t4052) { true } else { false } }) }).truthy() {
        v_N = f__pos(Py::str("working_age_population"), v_working_age_population.clone())?;
        let _ = f__need({ let t4055 = v_LF.clone(); Py::Bool({ let t4056 = py::mul(&v_N.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t4055, &t4056)? { true } else { false } }) }, Py::str("the labour force cannot exceed the working-age population"))?;
        v_nilf = py::sub(&v_N.clone(), &v_LF.clone())?;
        if ({ let t4057 = v_nilf.clone(); Py::Bool({ let t4058 = Py::Int(0i64); if py::lt(&t4057, &t4058)? { true } else { false } }) }).truthy() {
            v_nilf = Py::Float(0.0);
        }
        py::setitem(&v_out.clone(), &Py::str("participation_rate"), py::truediv(&v_LF.clone(), &v_N.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("employment_population_ratio"), py::truediv(&v_E.clone(), &v_N.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("not_in_labour_force"), v_nilf.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wacc(mut v_equity_value: Py, mut v_debt_value: Py, mut v_cost_of_equity: Py, mut v_cost_of_debt: Py, mut v_tax_rate: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_rd: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t4081 = (Py::tuple(vec![f__nonneg(Py::str("equity_value"), v_equity_value.clone())?, f__nonneg(Py::str("debt_value"), v_debt_value.clone())?])).items()?;
    if t4081.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E = t4081[0].clone();
    v_D = t4081[1].clone();
    let _ = f__need({ let t4082 = py::add(&v_E.clone(), &v_D.clone())?; Py::Bool({ let t4083 = Py::Int(0i64); if py::gt(&t4082, &t4083)? { true } else { false } }) }, Py::str("the firm needs a positive total value"))?;
    let t4087 = (Py::tuple(vec![f__rate(Py::str("cost_of_equity"), v_cost_of_equity.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("cost_of_debt"), v_cost_of_debt.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t4087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t4087[0].clone();
    v_rd = t4087[1].clone();
    v_t = f__prob(Py::str("tax_rate"), v_tax_rate.clone(), Py::Float(0.0), Py::Float(0.99))?;
    v_V = py::add(&v_E.clone(), &v_D.clone())?;
    return Ok(Py::dict(vec![(Py::str("wacc"), py::add(&py::mul(&py::truediv(&v_E.clone(), &v_V.clone())?, &v_re_.clone())?, &py::mul(&py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &v_rd.clone())?, &py::sub(&Py::Int(1i64), &v_t.clone())?)?)?), (Py::str("after_tax_cost_of_debt"), py::mul(&v_rd.clone(), &py::sub(&Py::Int(1i64), &v_t.clone())?)?), (Py::str("equity_weight"), py::truediv(&v_E.clone(), &v_V.clone())?), (Py::str("debt_weight"), py::truediv(&v_D.clone(), &v_V.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_whole_life_insurance(mut v_mortality_rates: Py, mut v_interest_rate: Py, mut v_benefit: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t4127 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t4127.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t4127[0].clone();
    v_v = t4127[1].clone();
    v_B = t4127[2].clone();
    return Ok(Py::dict(vec![(Py::str("apv"), py::mul(&v_B.clone(), &f__A(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zero_coupon_bond(mut v_face: Py, mut v_ytm: Py, mut v_years: Py, mut v_compounding: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t4157 = (Py::tuple(vec![f__pos(Py::str("face"), v_face.clone())?, f__rate(Py::str("ytm"), v_ytm.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t4157.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t4157[0].clone();
    v_y = t4157[1].clone();
    v_t = t4157[2].clone();
    let _ = f__need({ let t4158 = v_compounding.clone(); Py::Bool({ let t4159 = c__ZC()?; if py::contains(&t4159, &t4158)? { true } else { false } }) }, Py::str("compounding is annual, semiannual, quarterly, monthly or continuous"))?;
    v_m = py::getitem(&c__ZC()?, &v_compounding.clone())?;
    v_d = (if ({ let t4160 = v_m.clone(); Py::Bool({ let t4161 = Py::Int(0i64); if py::eq(&t4160, &t4161) { true } else { false } }) }).truthy() { py::math1("exp", &py::mul(&py::neg(&v_y.clone())?, &v_t.clone())?)? } else { py::pow(&py::add(&Py::Int(1i64), &py::truediv(&v_y.clone(), &v_m.clone())?)?, &py::mul(&py::neg(&v_m.clone())?, &v_t.clone())?)? });
    return Ok(Py::dict(vec![(Py::str("price"), py::mul(&v_F.clone(), &v_d.clone())?), (Py::str("discount_factor"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_106(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t123 = v_inputs.clone(); f_beta_regression(py::getitem(&t123, &Py::str("asset_returns"))?, py::getitem(&t123, &Py::str("market_returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1070(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1087 = v_inputs.clone(); f_direct_cost_variances(py::getitem(&t1087, &Py::str("standard_price"))?, py::getitem(&t1087, &Py::str("standard_quantity"))?, py::getitem(&t1087, &Py::str("actual_price"))?, py::getitem(&t1087, &Py::str("actual_quantity"))?, py::method(&t1087, "get", &[Py::str("quantity_purchased"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1126 = v_inputs.clone(); f_discounted_payback(py::getitem(&t1126, &Py::str("rate"))?, py::getitem(&t1126, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1149 = v_inputs.clone(); f_doubling_time(py::getitem(&t1149, &Py::str("rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1161(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1178 = v_inputs.clone(); f_durbin_watson(py::getitem(&t1178, &Py::str("residuals"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1197(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1214 = v_inputs.clone(); f_effective_annual_rate(py::getitem(&t1214, &Py::str("rate"))?, py::getitem(&t1214, &Py::str("periods_per_year"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1266 = v_inputs.clone(); f_endowment_insurance(py::getitem(&t1266, &Py::str("mortality_rates"))?, py::getitem(&t1266, &Py::str("interest_rate"))?, py::getitem(&t1266, &Py::str("term"))?, py::method(&t1266, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1282(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1299 = v_inputs.clone(); f_equivalent_annual_annuity(py::getitem(&t1299, &Py::str("rate"))?, py::getitem(&t1299, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1315(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1332 = v_inputs.clone(); f_financial_ratios(py::getitem(&t1332, &Py::str("sales"))?, py::getitem(&t1332, &Py::str("net_income"))?, py::getitem(&t1332, &Py::str("total_assets"))?, py::getitem(&t1332, &Py::str("total_equity"))?, py::getitem(&t1332, &Py::str("current_assets"))?, py::getitem(&t1332, &Py::str("current_liabilities"))?, py::getitem(&t1332, &Py::str("inventory"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1339(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1356 = v_inputs.clone(); f_fisher_real_rate(py::getitem(&t1356, &Py::str("nominal_rate"))?, py::getitem(&t1356, &Py::str("inflation_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1368(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1385 = v_inputs.clone(); f_forward_price(py::getitem(&t1385, &Py::str("spot"))?, py::getitem(&t1385, &Py::str("rate"))?, py::getitem(&t1385, &Py::str("time"))?, py::method(&t1385, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?, py::method(&t1385, "get", &[Py::str("storage_cost"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1409(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1426 = v_inputs.clone(); f_forward_price_dividends(py::getitem(&t1426, &Py::str("spot"))?, py::getitem(&t1426, &Py::str("rate"))?, py::getitem(&t1426, &Py::str("time"))?, py::getitem(&t1426, &Py::str("dividends"))?, py::getitem(&t1426, &Py::str("dividend_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1440(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1457 = v_inputs.clone(); f_forward_rate(py::getitem(&t1457, &Py::str("spot_rate_1"))?, py::getitem(&t1457, &Py::str("maturity_1"))?, py::getitem(&t1457, &Py::str("spot_rate_2"))?, py::getitem(&t1457, &Py::str("maturity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1482(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1499 = v_inputs.clone(); f_future_value_annuity(py::getitem(&t1499, &Py::str("payment"))?, py::getitem(&t1499, &Py::str("rate"))?, py::getitem(&t1499, &Py::str("periods"))?, py::method(&t1499, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1546 = v_inputs.clone(); f_gdp_deflator(py::getitem(&t1546, &Py::str("base_prices"))?, py::getitem(&t1546, &Py::str("base_quantities"))?, py::getitem(&t1546, &Py::str("current_prices"))?, py::getitem(&t1546, &Py::str("current_quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1562(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1579 = v_inputs.clone(); f_gini_coefficient(py::getitem(&t1579, &Py::str("incomes"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_162(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t179 = v_inputs.clone(); f_binomial_option(py::getitem(&t179, &Py::str("spot"))?, py::getitem(&t179, &Py::str("strike"))?, py::getitem(&t179, &Py::str("time"))?, py::getitem(&t179, &Py::str("rate"))?, py::getitem(&t179, &Py::str("volatility"))?, py::getitem(&t179, &Py::str("steps"))?, py::method(&t179, "get", &[Py::str("option_type"), Py::str("call")])?, py::method(&t179, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1625(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1642 = v_inputs.clone(); f_global_min_variance(py::getitem(&t1642, &Py::str("covariance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1659(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1676 = v_inputs.clone(); f_gordon_growth(py::getitem(&t1676, &Py::str("required_return"))?, py::getitem(&t1676, &Py::str("growth"))?, py::method(&t1676, "get", &[Py::str("dividend"), Py::None])?, py::method(&t1676, "get", &[Py::str("next_dividend"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1715 = v_inputs.clone(); f_gordon_implied_return(py::getitem(&t1715, &Py::str("price"))?, py::getitem(&t1715, &Py::str("growth"))?, py::method(&t1715, "get", &[Py::str("dividend"), Py::None])?, py::method(&t1715, "get", &[Py::str("next_dividend"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1736(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1753 = v_inputs.clone(); f_growing_annuity(py::getitem(&t1753, &Py::str("first_payment"))?, py::getitem(&t1753, &Py::str("rate"))?, py::getitem(&t1753, &Py::str("growth"))?, py::getitem(&t1753, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1762(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1779 = v_inputs.clone(); f_growing_perpetuity(py::getitem(&t1779, &Py::str("first_payment"))?, py::getitem(&t1779, &Py::str("rate"))?, py::getitem(&t1779, &Py::str("growth"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1794(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1811 = v_inputs.clone(); f_herfindahl_index(py::getitem(&t1811, &Py::str("market_shares"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1827(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1844 = v_inputs.clone(); f_historical_var(py::getitem(&t1844, &Py::str("returns"))?, py::getitem(&t1844, &Py::str("confidence"))?, py::method(&t1844, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1894(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1911 = v_inputs.clone(); f_income_elasticity(py::getitem(&t1911, &Py::str("income_1"))?, py::getitem(&t1911, &Py::str("quantity_1"))?, py::getitem(&t1911, &Py::str("income_2"))?, py::getitem(&t1911, &Py::str("quantity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_193(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t210 = v_inputs.clone(); f_black_scholes(py::getitem(&t210, &Py::str("spot"))?, py::getitem(&t210, &Py::str("strike"))?, py::getitem(&t210, &Py::str("time"))?, py::getitem(&t210, &Py::str("rate"))?, py::getitem(&t210, &Py::str("volatility"))?, py::method(&t210, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1936(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1962 = v_inputs.clone(); f__check_income_tax(py::getitem(&t1962, &Py::str("taxable_income"))?, py::getitem(&t1962, &Py::str("bracket_thresholds"))?, py::getitem(&t1962, &Py::str("bracket_rates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1991 = v_inputs.clone(); f_inflation_adjustment(py::getitem(&t1991, &Py::str("price_levels"))?, py::method(&t1991, "get", &[Py::str("amount"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2030 = v_inputs.clone(); f_information_ratio(py::getitem(&t2030, &Py::str("portfolio_returns"))?, py::getitem(&t2030, &Py::str("benchmark_returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2073 = v_inputs.clone(); f__check_inventory(py::getitem(&t2073, &Py::str("layer_units"))?, py::getitem(&t2073, &Py::str("layer_costs"))?, py::getitem(&t2073, &Py::str("units_sold"))?, py::method(&t2073, "get", &[Py::str("selling_price"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2158(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2175 = v_inputs.clone(); f_life_annuity_due(py::getitem(&t2175, &Py::str("mortality_rates"))?, py::getitem(&t2175, &Py::str("interest_rate"))?, py::method(&t2175, "get", &[Py::str("payment"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2190(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2207 = v_inputs.clone(); f_life_expectancy(py::getitem(&t2207, &Py::str("mortality_rates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2242(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2259 = v_inputs.clone(); f_loan_balance(py::getitem(&t2259, &Py::str("principal"))?, py::getitem(&t2259, &Py::str("annual_rate"))?, py::getitem(&t2259, &Py::str("years"))?, py::getitem(&t2259, &Py::str("payments_made"))?, py::method(&t2259, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2294(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2311 = v_inputs.clone(); f_loan_cumulative_interest(py::getitem(&t2311, &Py::str("principal"))?, py::getitem(&t2311, &Py::str("annual_rate"))?, py::getitem(&t2311, &Py::str("years"))?, py::getitem(&t2311, &Py::str("payments_made"))?, py::method(&t2311, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2341(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2358 = v_inputs.clone(); f_loan_payment(py::getitem(&t2358, &Py::str("principal"))?, py::getitem(&t2358, &Py::str("annual_rate"))?, py::getitem(&t2358, &Py::str("years"))?, py::method(&t2358, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2391(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2408 = v_inputs.clone(); f_loan_payment_split(py::getitem(&t2408, &Py::str("principal"))?, py::getitem(&t2408, &Py::str("annual_rate"))?, py::getitem(&t2408, &Py::str("years"))?, py::getitem(&t2408, &Py::str("payment_number"))?, py::method(&t2408, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2453 = v_inputs.clone(); f_log_log_elasticity(py::getitem(&t2453, &Py::str("prices"))?, py::getitem(&t2453, &Py::str("quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2468(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2485 = v_inputs.clone(); f_market_equilibrium(py::getitem(&t2485, &Py::str("demand_intercept"))?, py::getitem(&t2485, &Py::str("demand_slope"))?, py::getitem(&t2485, &Py::str("supply_intercept"))?, py::getitem(&t2485, &Py::str("supply_slope"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t266 = v_inputs.clone(); f_bond_convexity(py::getitem(&t266, &Py::str("face"))?, py::getitem(&t266, &Py::str("coupon_rate"))?, py::getitem(&t266, &Py::str("ytm"))?, py::getitem(&t266, &Py::str("years"))?, py::method(&t266, "get", &[Py::str("frequency"), Py::Int(2i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2497(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2514 = v_inputs.clone(); f_max_drawdown(py::getitem(&t2514, &Py::str("prices"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2526(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2543 = v_inputs.clone(); f_mean_returns(py::getitem(&t2543, &Py::str("returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2554(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2571 = v_inputs.clone(); f_min_variance_two_asset(py::getitem(&t2571, &Py::str("sd_1"))?, py::getitem(&t2571, &Py::str("sd_2"))?, py::getitem(&t2571, &Py::str("correlation"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2614 = v_inputs.clone(); f_mirr(py::getitem(&t2614, &Py::str("cash_flows"))?, py::getitem(&t2614, &Py::str("finance_rate"))?, py::getitem(&t2614, &Py::str("reinvest_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2629(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2646 = v_inputs.clone(); f_money_multiplier(py::getitem(&t2646, &Py::str("reserve_ratio"))?, py::method(&t2646, "get", &[Py::str("currency_ratio"), Py::Float(0.0)])?, py::method(&t2646, "get", &[Py::str("monetary_base"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2657(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2674 = v_inputs.clone(); f_monopoly(py::getitem(&t2674, &Py::str("price_intercept"))?, py::getitem(&t2674, &Py::str("price_slope"))?, py::getitem(&t2674, &Py::str("marginal_cost"))?, py::method(&t2674, "get", &[Py::str("fixed_cost"), Py::Float(0.0)])?, py::method(&t2674, "get", &[Py::str("cost_quadratic"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2714(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2731 = v_inputs.clone(); f_multiple_regression(py::getitem(&t2731, &Py::str("X"))?, py::getitem(&t2731, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2755(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2772 = v_inputs.clone(); f_net_premium_whole_life(py::getitem(&t2772, &Py::str("mortality_rates"))?, py::getitem(&t2772, &Py::str("interest_rate"))?, py::method(&t2772, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2791(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2808 = v_inputs.clone(); f_nominal_rate(py::getitem(&t2808, &Py::str("effective_rate"))?, py::getitem(&t2808, &Py::str("periods_per_year"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2821(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2840 = v_inputs.clone(); f__check_npv(py::getitem(&t2840, &Py::str("rate"))?, py::getitem(&t2840, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2855(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2872 = v_inputs.clone(); f_number_of_periods(py::getitem(&t2872, &Py::str("present_value"))?, py::getitem(&t2872, &Py::str("payment"))?, py::getitem(&t2872, &Py::str("rate"))?, py::method(&t2872, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2909 = v_inputs.clone(); f_ols_simple(py::getitem(&t2909, &Py::str("x"))?, py::getitem(&t2909, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2923(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2940 = v_inputs.clone(); f_overhead_cost_variances(py::getitem(&t2940, &Py::str("variable_overhead_rate"))?, py::getitem(&t2940, &Py::str("actual_variable_overhead"))?, py::getitem(&t2940, &Py::str("actual_hours"))?, py::getitem(&t2940, &Py::str("standard_hours"))?, py::getitem(&t2940, &Py::str("budgeted_fixed_overhead"))?, py::getitem(&t2940, &Py::str("actual_fixed_overhead"))?, py::getitem(&t2940, &Py::str("denominator_hours"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2973 = v_inputs.clone(); f_parametric_var(py::getitem(&t2973, &Py::str("mean"))?, py::getitem(&t2973, &Py::str("std_dev"))?, py::getitem(&t2973, &Py::str("confidence"))?, py::method(&t2973, "get", &[Py::str("horizon"), Py::Float(1.0)])?, py::method(&t2973, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2989(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3006 = v_inputs.clone(); f_payback_period(py::getitem(&t3006, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3028(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3045 = v_inputs.clone(); f_per_unit_tax(py::getitem(&t3045, &Py::str("demand_intercept"))?, py::getitem(&t3045, &Py::str("demand_slope"))?, py::getitem(&t3045, &Py::str("supply_intercept"))?, py::getitem(&t3045, &Py::str("supply_slope"))?, py::getitem(&t3045, &Py::str("tax"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_304(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t321 = v_inputs.clone(); f_bond_duration(py::getitem(&t321, &Py::str("face"))?, py::getitem(&t321, &Py::str("coupon_rate"))?, py::getitem(&t321, &Py::str("ytm"))?, py::getitem(&t321, &Py::str("years"))?, py::method(&t321, "get", &[Py::str("frequency"), Py::Int(2i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3056(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3073 = v_inputs.clone(); f_perpetuity(py::getitem(&t3073, &Py::str("payment"))?, py::getitem(&t3073, &Py::str("rate"))?, py::method(&t3073, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3098 = v_inputs.clone(); f_point_elasticity(py::getitem(&t3098, &Py::str("demand_intercept"))?, py::getitem(&t3098, &Py::str("demand_slope"))?, py::getitem(&t3098, &Py::str("price"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3161 = v_inputs.clone(); f_portfolio_return_risk(py::getitem(&t3161, &Py::str("weights"))?, py::getitem(&t3161, &Py::str("expected_returns"))?, py::getitem(&t3161, &Py::str("covariance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3222 = v_inputs.clone(); f_portfolio_var(py::getitem(&t3222, &Py::str("weights"))?, py::getitem(&t3222, &Py::str("covariance"))?, py::getitem(&t3222, &Py::str("confidence"))?, py::method(&t3222, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?, py::method(&t3222, "get", &[Py::str("horizon"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3231(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3248 = v_inputs.clone(); f_present_value(py::getitem(&t3248, &Py::str("future_value"))?, py::getitem(&t3248, &Py::str("rate"))?, py::getitem(&t3248, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3273(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3290 = v_inputs.clone(); f_present_value_annuity(py::getitem(&t3290, &Py::str("payment"))?, py::getitem(&t3290, &Py::str("rate"))?, py::getitem(&t3290, &Py::str("periods"))?, py::method(&t3290, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3320(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3337 = v_inputs.clone(); f_price_indices(py::getitem(&t3337, &Py::str("base_prices"))?, py::getitem(&t3337, &Py::str("base_quantities"))?, py::getitem(&t3337, &Py::str("current_prices"))?, py::getitem(&t3337, &Py::str("current_quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3352(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3369 = v_inputs.clone(); f_profitability_index(py::getitem(&t3369, &Py::str("rate"))?, py::getitem(&t3369, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3403 = v_inputs.clone(); f_put_call_parity(py::getitem(&t3403, &Py::str("call_price"))?, py::getitem(&t3403, &Py::str("spot"))?, py::getitem(&t3403, &Py::str("strike"))?, py::getitem(&t3403, &Py::str("time"))?, py::getitem(&t3403, &Py::str("rate"))?, py::method(&t3403, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_34(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t51 = v_inputs.clone(); f_annuity_rate(py::getitem(&t51, &Py::str("present_value"))?, py::getitem(&t51, &Py::str("payment"))?, py::getitem(&t51, &Py::str("periods"))?, py::method(&t51, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3443 = v_inputs.clone(); f_scenario_return(py::getitem(&t3443, &Py::str("probabilities"))?, py::getitem(&t3443, &Py::str("returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3491 = v_inputs.clone(); f_sharpe_ratio(py::getitem(&t3491, &Py::str("returns"))?, py::method(&t3491, "get", &[Py::str("risk_free"), Py::Float(0.0)])?, py::method(&t3491, "get", &[Py::str("periods_per_year"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3502(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3519 = v_inputs.clone(); f_simple_interest(py::getitem(&t3519, &Py::str("principal"))?, py::getitem(&t3519, &Py::str("rate"))?, py::getitem(&t3519, &Py::str("years"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3546(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3563 = v_inputs.clone(); f_sinking_fund(py::getitem(&t3563, &Py::str("future_value"))?, py::getitem(&t3563, &Py::str("rate"))?, py::getitem(&t3563, &Py::str("periods"))?, py::method(&t3563, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t373 = v_inputs.clone(); f_bond_price(py::getitem(&t373, &Py::str("face"))?, py::getitem(&t373, &Py::str("coupon_rate"))?, py::getitem(&t373, &Py::str("ytm"))?, py::getitem(&t373, &Py::str("years"))?, py::method(&t373, "get", &[Py::str("frequency"), Py::Int(2i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3599 = v_inputs.clone(); f_solow_steady_state(py::getitem(&t3599, &Py::str("saving_rate"))?, py::getitem(&t3599, &Py::str("population_growth"))?, py::getitem(&t3599, &Py::str("depreciation"))?, py::getitem(&t3599, &Py::str("alpha"))?, py::method(&t3599, "get", &[Py::str("technology_growth"), Py::Float(0.0)])?, py::method(&t3599, "get", &[Py::str("productivity"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3616(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3633 = v_inputs.clone(); f_sortino_ratio(py::getitem(&t3633, &Py::str("returns"))?, py::method(&t3633, "get", &[Py::str("target"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3648(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3665 = v_inputs.clone(); f_spending_multiplier(py::getitem(&t3665, &Py::str("mpc"))?, py::method(&t3665, "get", &[Py::str("tax_rate"), Py::Float(0.0)])?, py::method(&t3665, "get", &[Py::str("import_propensity"), Py::Float(0.0)])?, py::method(&t3665, "get", &[Py::str("spending_change"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3696 = v_inputs.clone(); f_stackelberg(py::getitem(&t3696, &Py::str("price_intercept"))?, py::getitem(&t3696, &Py::str("price_slope"))?, py::getitem(&t3696, &Py::str("cost_leader"))?, py::getitem(&t3696, &Py::str("cost_follower"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3725(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3742 = v_inputs.clone(); f_survival_probability(py::getitem(&t3742, &Py::str("mortality_rates"))?, py::getitem(&t3742, &Py::str("years"))?, py::method(&t3742, "get", &[Py::str("deferred_years"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3823 = v_inputs.clone(); f_tangency_portfolio(py::getitem(&t3823, &Py::str("expected_returns"))?, py::getitem(&t3823, &Py::str("covariance"))?, py::getitem(&t3823, &Py::str("risk_free"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3874 = v_inputs.clone(); f_temporary_annuity_due(py::getitem(&t3874, &Py::str("mortality_rates"))?, py::getitem(&t3874, &Py::str("interest_rate"))?, py::getitem(&t3874, &Py::str("term"))?, py::method(&t3874, "get", &[Py::str("payment"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3908(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3925 = v_inputs.clone(); f_term_insurance(py::getitem(&t3925, &Py::str("mortality_rates"))?, py::getitem(&t3925, &Py::str("interest_rate"))?, py::getitem(&t3925, &Py::str("term"))?, py::method(&t3925, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3968 = v_inputs.clone(); f_treynor_jensen(py::getitem(&t3968, &Py::str("portfolio_returns"))?, py::getitem(&t3968, &Py::str("market_returns"))?, py::method(&t3968, "get", &[Py::str("risk_free"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3986(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4003 = v_inputs.clone(); f_two_asset_portfolio(py::getitem(&t4003, &Py::str("weight_1"))?, py::getitem(&t4003, &Py::str("return_1"))?, py::getitem(&t4003, &Py::str("return_2"))?, py::getitem(&t4003, &Py::str("sd_1"))?, py::getitem(&t4003, &Py::str("sd_2"))?, py::getitem(&t4003, &Py::str("correlation"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4027(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4044 = v_inputs.clone(); f_two_stage_ddm(py::getitem(&t4044, &Py::str("dividend"))?, py::getitem(&t4044, &Py::str("high_growth"))?, py::getitem(&t4044, &Py::str("high_growth_years"))?, py::getitem(&t4044, &Py::str("stable_growth"))?, py::getitem(&t4044, &Py::str("required_return"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4059(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4076 = v_inputs.clone(); f_unemployment_rate(py::getitem(&t4076, &Py::str("employed"))?, py::getitem(&t4076, &Py::str("unemployed"))?, py::method(&t4076, "get", &[Py::str("working_age_population"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4091(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4108 = v_inputs.clone(); f_wacc(py::getitem(&t4108, &Py::str("equity_value"))?, py::getitem(&t4108, &Py::str("debt_value"))?, py::getitem(&t4108, &Py::str("cost_of_equity"))?, py::getitem(&t4108, &Py::str("cost_of_debt"))?, py::getitem(&t4108, &Py::str("tax_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4147 = v_inputs.clone(); f_whole_life_insurance(py::getitem(&t4147, &Py::str("mortality_rates"))?, py::getitem(&t4147, &Py::str("interest_rate"))?, py::method(&t4147, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4162(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t4179 = v_inputs.clone(); f_zero_coupon_bond(py::getitem(&t4179, &Py::str("face"))?, py::getitem(&t4179, &Py::str("ytm"))?, py::getitem(&t4179, &Py::str("years"))?, py::method(&t4179, "get", &[Py::str("compounding"), Py::str("annual")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_428(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t445 = v_inputs.clone(); f_bond_price_change(py::getitem(&t445, &Py::str("face"))?, py::getitem(&t445, &Py::str("coupon_rate"))?, py::getitem(&t445, &Py::str("ytm"))?, py::getitem(&t445, &Py::str("years"))?, py::getitem(&t445, &Py::str("yield_change"))?, py::method(&t445, "get", &[Py::str("frequency"), Py::Int(2i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_517(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t534 = v_inputs.clone(); f_break_even(py::getitem(&t534, &Py::str("fixed_costs"))?, py::getitem(&t534, &Py::str("price"))?, py::getitem(&t534, &Py::str("variable_cost"))?, py::method(&t534, "get", &[Py::str("target_profit"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_550(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t567 = v_inputs.clone(); f_bs_greeks(py::getitem(&t567, &Py::str("spot"))?, py::getitem(&t567, &Py::str("strike"))?, py::getitem(&t567, &Py::str("time"))?, py::getitem(&t567, &Py::str("rate"))?, py::getitem(&t567, &Py::str("volatility"))?, py::method(&t567, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_573(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t590 = v_inputs.clone(); f_cagr(py::getitem(&t590, &Py::str("begin_value"))?, py::getitem(&t590, &Py::str("end_value"))?, py::getitem(&t590, &Py::str("years"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t614 = v_inputs.clone(); f_capm(py::getitem(&t614, &Py::str("risk_free"))?, py::getitem(&t614, &Py::str("beta"))?, py::getitem(&t614, &Py::str("market_return"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_622(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t639 = v_inputs.clone(); f_cobb_douglas_demand(py::getitem(&t639, &Py::str("alpha"))?, py::getitem(&t639, &Py::str("beta"))?, py::getitem(&t639, &Py::str("income"))?, py::getitem(&t639, &Py::str("price_x"))?, py::getitem(&t639, &Py::str("price_y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_66(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t83 = v_inputs.clone(); f_arc_price_elasticity(py::getitem(&t83, &Py::str("price_1"))?, py::getitem(&t83, &Py::str("quantity_1"))?, py::getitem(&t83, &Py::str("price_2"))?, py::getitem(&t83, &Py::str("quantity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_672(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t689 = v_inputs.clone(); f_comparative_advantage(py::getitem(&t689, &Py::str("a_good_x"))?, py::getitem(&t689, &Py::str("a_good_y"))?, py::getitem(&t689, &Py::str("b_good_x"))?, py::getitem(&t689, &Py::str("b_good_y"))?, py::getitem(&t689, &Py::str("basis"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_728(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t745 = v_inputs.clone(); f_competitive_firm_cost_curves(py::getitem(&t745, &Py::str("quantities"))?, py::getitem(&t745, &Py::str("total_costs"))?, py::getitem(&t745, &Py::str("price"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_768(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t785 = v_inputs.clone(); f_compound_interest(py::getitem(&t785, &Py::str("principal"))?, py::getitem(&t785, &Py::str("rate"))?, py::getitem(&t785, &Py::str("years"))?, py::method(&t785, "get", &[Py::str("periods_per_year"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t813 = v_inputs.clone(); f_continuous_compounding(py::getitem(&t813, &Py::str("principal"))?, py::getitem(&t813, &Py::str("rate"))?, py::getitem(&t813, &Py::str("years"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_832(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t849 = v_inputs.clone(); f_correlation(py::getitem(&t849, &Py::str("x"))?, py::getitem(&t849, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_864(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t881 = v_inputs.clone(); f_cournot(py::getitem(&t881, &Py::str("price_intercept"))?, py::getitem(&t881, &Py::str("price_slope"))?, py::getitem(&t881, &Py::str("cost_1"))?, py::getitem(&t881, &Py::str("cost_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_930(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t953 = v_inputs.clone(); f__check_cc(py::getitem(&t953, &Py::str("opening_balance"))?, py::getitem(&t953, &Py::str("apr"))?, py::getitem(&t953, &Py::str("billing_days"))?, py::getitem(&t953, &Py::str("rate_basis"))?, py::method(&t953, "get", &[Py::str("transaction_days"), Py::None])?, py::method(&t953, "get", &[Py::str("transaction_amounts"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_966(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t983 = v_inputs.clone(); f_cross_price_elasticity(py::getitem(&t983, &Py::str("price_other_1"))?, py::getitem(&t983, &Py::str("quantity_1"))?, py::getitem(&t983, &Py::str("price_other_2"))?, py::getitem(&t983, &Py::str("quantity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
