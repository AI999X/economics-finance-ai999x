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
    "compound_interest",
    "continuous_compounding",
    "correlation",
    "cournot",
    "cross_price_elasticity",
    "crossover_rate",
    "discounted_payback",
    "doubling_time",
    "durbin_watson",
    "effective_annual_rate",
    "endowment_insurance",
    "equivalent_annual_annuity",
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
    "inflation_adjustment",
    "information_ratio",
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
        "compound_interest" => {
            let r = f_compound_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_662(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "continuous_compounding" => {
            let r = f_continuous_compounding(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_690(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "correlation" => {
            let r = f_correlation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cournot" => {
            let r = f_cournot(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_758(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_price_elasticity" => {
            let r = f_cross_price_elasticity(match v.get("price_other_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_other_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crossover_rate" => {
            let r = f_crossover_rate(match v.get("cash_flows_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_crossover(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discounted_payback" => {
            let r = f_discounted_payback(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_898(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "doubling_time" => {
            let r = f_doubling_time(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_921(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "durbin_watson" => {
            let r = f_durbin_watson(match v.get("residuals") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_950(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "effective_annual_rate" => {
            let r = f_effective_annual_rate(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "endowment_insurance" => {
            let r = f_endowment_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1038(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equivalent_annual_annuity" => {
            let r = f_equivalent_annual_annuity(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fisher_real_rate" => {
            let r = f_fisher_real_rate(match v.get("nominal_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("inflation_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1095(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_price" => {
            let r = f_forward_price(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("storage_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_price_dividends" => {
            let r = f_forward_price_dividends(match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividends") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_times") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forward_rate" => {
            let r = f_forward_rate(match v.get("spot_rate_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("maturity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot_rate_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("maturity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "future_value_annuity" => {
            let r = f_future_value_annuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1238(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gdp_deflator" => {
            let r = f_gdp_deflator(match v.get("base_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1285(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gini_coefficient" => {
            let r = f_gini_coefficient(match v.get("incomes") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "global_min_variance" => {
            let r = f_global_min_variance(match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1381(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gordon_growth" => {
            let r = f_gordon_growth(match v.get("required_return") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("next_dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gordon_implied_return" => {
            let r = f_gordon_implied_return(match v.get("price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("next_dividend") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "growing_annuity" => {
            let r = f_growing_annuity(match v.get("first_payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1492(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "growing_perpetuity" => {
            let r = f_growing_perpetuity(match v.get("first_payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("growth") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1518(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "herfindahl_index" => {
            let r = f_herfindahl_index(match v.get("market_shares") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1550(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "historical_var" => {
            let r = f_historical_var(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "implied_volatility" => {
            let r = f_implied_volatility(match v.get("option_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("option_type") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("call")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_iv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "income_elasticity" => {
            let r = f_income_elasticity(match v.get("income_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("income_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantity_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inflation_adjustment" => {
            let r = f_inflation_adjustment(match v.get("price_levels") { Some(x) => py::from_val(x), None => { return None } }, match v.get("amount") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "information_ratio" => {
            let r = f_information_ratio(match v.get("portfolio_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benchmark_returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1718(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "irr" => {
            let r = f_irr(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_irr(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "life_annuity_due" => {
            let r = f_life_annuity_due(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1820(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "life_expectancy" => {
            let r = f_life_expectancy(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1852(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_balance" => {
            let r = f_loan_balance(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_made") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1904(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_cumulative_interest" => {
            let r = f_loan_cumulative_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_made") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_payment" => {
            let r = f_loan_payment(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2003(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "loan_payment_split" => {
            let r = f_loan_payment_split(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("annual_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment_number") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payments_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(12i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "log_log_elasticity" => {
            let r = f_log_log_elasticity(match v.get("prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2098(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "market_equilibrium" => {
            let r = f_market_equilibrium(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_slope") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_drawdown" => {
            let r = f_max_drawdown(match v.get("prices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2159(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_returns" => {
            let r = f_mean_returns(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2188(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_variance_two_asset" => {
            let r = f_min_variance_two_asset(match v.get("sd_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("correlation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2216(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mirr" => {
            let r = f_mirr(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } }, match v.get("finance_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reinvest_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2259(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "money_multiplier" => {
            let r = f_money_multiplier(match v.get("reserve_ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("currency_ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("monetary_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "monopoly" => {
            let r = f_monopoly(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("marginal_cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fixed_cost") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("cost_quadratic") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "multiple_regression" => {
            let r = f_multiple_regression(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2376(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "net_premium_whole_life" => {
            let r = f_net_premium_whole_life(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2417(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nominal_rate" => {
            let r = f_nominal_rate(match v.get("effective_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2453(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "npv" => {
            let r = f_npv(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "number_of_periods" => {
            let r = f_number_of_periods(match v.get("present_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2517(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ols_simple" => {
            let r = f_ols_simple(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2554(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_var" => {
            let r = f_parametric_var(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("std_dev") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2587(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "payback_period" => {
            let r = f_payback_period(match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2620(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "per_unit_tax" => {
            let r = f_per_unit_tax(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2659(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "perpetuity" => {
            let r = f_perpetuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_elasticity" => {
            let r = f_point_elasticity(match v.get("demand_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2712(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "portfolio_return_risk" => {
            let r = f_portfolio_return_risk(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("expected_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2775(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "portfolio_var" => {
            let r = f_portfolio_var(match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("confidence") { Some(x) => py::from_val(x), None => { return None } }, match v.get("portfolio_value") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("horizon") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "present_value" => {
            let r = f_present_value(match v.get("future_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2862(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "present_value_annuity" => {
            let r = f_present_value_annuity(match v.get("payment") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2904(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "price_indices" => {
            let r = f_price_indices(match v.get("base_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base_quantities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_prices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("current_quantities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "profitability_index" => {
            let r = f_profitability_index(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cash_flows") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2983(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "put_call_parity" => {
            let r = f_put_call_parity(match v.get("call_price") { Some(x) => py::from_val(x), None => { return None } }, match v.get("spot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strike") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dividend_yield") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "scenario_return" => {
            let r = f_scenario_return(match v.get("probabilities") { Some(x) => py::from_val(x), None => { return None } }, match v.get("returns") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sharpe_ratio" => {
            let r = f_sharpe_ratio(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("periods_per_year") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3105(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_interest" => {
            let r = f_simple_interest(match v.get("principal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sinking_fund" => {
            let r = f_sinking_fund(match v.get("future_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("periods") { Some(x) => py::from_val(x), None => { return None } }, match v.get("timing") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("ordinary")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solow_steady_state" => {
            let r = f_solow_steady_state(match v.get("saving_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("population_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("depreciation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("technology_growth") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("productivity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3213(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sortino_ratio" => {
            let r = f_sortino_ratio(match v.get("returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3247(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spending_multiplier" => {
            let r = f_spending_multiplier(match v.get("mpc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax_rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("import_propensity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("spending_change") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3279(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stackelberg" => {
            let r = f_stackelberg(match v.get("price_intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("price_slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_leader") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_follower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3310(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "survival_probability" => {
            let r = f_survival_probability(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("deferred_years") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangency_portfolio" => {
            let r = f_tangency_portfolio(match v.get("expected_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("covariance") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3437(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "temporary_annuity_due" => {
            let r = f_temporary_annuity_due(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("payment") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "term_insurance" => {
            let r = f_term_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("term") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "treynor_jensen" => {
            let r = f_treynor_jensen(match v.get("portfolio_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("market_returns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("risk_free") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_asset_portfolio" => {
            let r = f_two_asset_portfolio(match v.get("weight_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("return_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("return_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("correlation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3617(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_stage_ddm" => {
            let r = f_two_stage_ddm(match v.get("dividend") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("high_growth_years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("stable_growth") { Some(x) => py::from_val(x), None => { return None } }, match v.get("required_return") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3658(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wacc" => {
            let r = f_wacc(match v.get("equity_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("debt_value") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_of_equity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost_of_debt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tax_rate") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3690(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "whole_life_insurance" => {
            let r = f_whole_life_insurance(match v.get("mortality_rates") { Some(x) => py::from_val(x), None => { return None } }, match v.get("interest_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("benefit") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3729(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_coupon_bond" => {
            let r = f_zero_coupon_bond(match v.get("face") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ytm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("years") { Some(x) => py::from_val(x), None => { return None } }, match v.get("compounding") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("annual")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    return Ok(py::fsum(&{ let mut out = vec![]; for t3728 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3728;
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
    return Ok(py::fsum(&{ let mut out = vec![]; for t3487 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3487;
out.push(py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &py::getitem(&v_p.clone(), &v_k.clone())?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__annuity_fv(mut v_p: Py, mut v_r: Py, mut v_n: Py, mut v_due: Py) -> R {
    let mut v_f: Py = Py::None;
    v_f = (if ({ let t3175 = v_r.clone(); Py::Bool({ let t3176 = Py::Int(0i64); if py::eq(&t3175, &t3176) { true } else { false } }) }).truthy() { v_n.clone() } else { py::truediv(&py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?, &Py::Int(1i64))?, &v_r.clone())? });
    return Ok(py::mul(&py::mul(&v_p.clone(), &v_f.clone())?, &(if (v_due.clone()).truthy() { py::add(&Py::Int(1i64), &v_r.clone())? } else { Py::Int(1i64) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__annuity_pv(mut v_p: Py, mut v_r: Py, mut v_n: Py, mut v_due: Py) -> R {
    let mut v_f: Py = Py::None;
    v_f = (if ({ let t2902 = v_r.clone(); Py::Bool({ let t2903 = Py::Int(0i64); if py::eq(&t2902, &t2903) { true } else { false } }) }).truthy() { v_n.clone() } else { py::truediv(&py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &py::neg(&v_n.clone())?)?)?, &v_r.clone())? });
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
    let t1640 = (Py::tuple(vec![f__pos(py::add(&v_nx.clone(), &Py::str("_1"))?, v_x1.clone())?, f__pos(py::add(&v_nq.clone(), &Py::str("_1"))?, v_q1.clone())?, f__pos(py::add(&v_nx.clone(), &Py::str("_2"))?, v_x2.clone())?, f__pos(py::add(&v_nq.clone(), &Py::str("_2"))?, v_q2.clone())?])).items()?;
    if t1640.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1640[0].clone();
    v_b = t1640[1].clone();
    v_c = t1640[2].clone();
    v_d = t1640[3].clone();
    let _ = f__need({ let t1641 = v_a.clone(); Py::Bool({ let t1642 = v_c.clone(); if py::ne(&t1641, &t1642) { true } else { false } }) }, Py::str(&["the two ".to_string(), py::format(&py::method(&v_nx.clone(), "replace", &[Py::str("_"), Py::str(" ")])?, "")?, "s must differ".to_string()].concat()))?;
    v_e = py::truediv(&py::truediv(&py::sub(&v_d.clone(), &v_b.clone())?, &py::truediv(&py::add(&v_b.clone(), &v_d.clone())?, &Py::Int(2i64))?)?, &py::truediv(&py::sub(&v_c.clone(), &v_a.clone())?, &py::truediv(&py::add(&v_a.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?;
    v_ex = py::truediv(&py::truediv(&py::sub(&py::fraction(&[v_d.clone()])?, &py::fraction(&[v_b.clone()])?)?, &py::add(&py::fraction(&[v_b.clone()])?, &py::fraction(&[v_d.clone()])?)?)?, &py::truediv(&py::sub(&py::fraction(&[v_c.clone()])?, &py::fraction(&[v_a.clone()])?)?, &py::add(&py::fraction(&[v_a.clone()])?, &py::fraction(&[v_c.clone()])?)?)?)?;
    return Ok(Py::tuple(vec![v_e.clone(), v_ex.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__balance(mut v_P: Py, mut v_i: Py, mut v_pmt: Py, mut v_k: Py) -> R {
    return Ok((if ({ let t2051 = v_i.clone(); Py::Bool({ let t2052 = Py::Int(0i64); if py::eq(&t2051, &t2052) { true } else { false } }) }).truthy() { py::sub(&v_P.clone(), &py::mul(&v_pmt.clone(), &v_k.clone())?)? } else { py::sub(&py::mul(&v_P.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &v_k.clone())?)?, &py::truediv(&py::mul(&v_pmt.clone(), &py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &v_k.clone())?, &Py::Int(1i64))?)?, &v_i.clone())?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__baskets(mut v_p0: Py, mut v_q0: Py, mut v_p1: Py, mut v_q1: Py) -> R {
    let t2930 = ({ let mut out = vec![]; for t2922 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("base_prices"), v_p0.clone()]), Py::tuple(vec![Py::str("base_quantities"), v_q0.clone()]), Py::tuple(vec![Py::str("current_prices"), v_p1.clone()]), Py::tuple(vec![Py::str("current_quantities"), v_q1.clone()])])))? { let mut v_n: Py; let mut v_v: Py; 
let t2923 = (t2922).items()?;
if t2923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2923[0].clone();
v_v = t2923[1].clone();
out.push(f__vec(v_n.clone(), v_v.clone(), Py::Int(1i64))?); } Py::list(out) }).items()?;
    if t2930.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t2930[0].clone();
    v_q0 = t2930[1].clone();
    v_p1 = t2930[2].clone();
    v_q1 = t2930[3].clone();
    let _ = f__need({ let t2931 = py::len(&v_p0.clone())?; Py::Bool({ let t2932 = py::len(&v_q0.clone())?; if py::eq(&t2931, &t2932) { { let t2933 = py::len(&v_p1.clone())?; if py::eq(&t2932, &t2933) { { let t2934 = py::len(&v_q1.clone())?; if py::eq(&t2933, &t2934) { true } else { false } } } else { false } } } else { false } }) }, Py::str("one price and quantity per good in each year"))?;
    let _ = f__need({ let t2941 = Py::Bool(({ let mut out = vec![]; for t2935 in py::iter(&(py::add(&v_p0.clone(), &v_p1.clone())?))? { let mut v_x: Py; 
v_x = t2935;
out.push({ let t2936 = v_x.clone(); Py::Bool({ let t2937 = Py::Int(0i64); if py::gt(&t2936, &t2937)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2941.truthy() { t2941 } else { Py::Bool(({ let mut out = vec![]; for t2938 in py::iter(&(py::add(&v_q0.clone(), &v_q1.clone())?))? { let mut v_x: Py; 
v_x = t2938;
out.push({ let t2939 = v_x.clone(); Py::Bool({ let t2940 = Py::Int(0i64); if py::ge(&t2939, &t2940)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("prices must be positive and quantities non-negative"))?;
    let _ = f__need({ let t2946 = { let t2942 = py::sum(&v_q0.clone(), None)?; Py::Bool({ let t2943 = Py::Int(0i64); if py::gt(&t2942, &t2943)? { true } else { false } }) }; if !t2946.truthy() { t2946 } else { { let t2944 = py::sum(&v_q1.clone(), None)?; Py::Bool({ let t2945 = Py::Int(0i64); if py::gt(&t2944, &t2945)? { true } else { false } }) } } }, Py::str("each year needs a positive quantity"))?;
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
    let t1615 = (Py::tuple(vec![py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?, py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_T.clone())?)?])).items()?;
    if t1615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dq = t1615[0].clone();
    v_dr = t1615[1].clone();
    return Ok(Py::tuple(vec![v_d1.clone(), v_d2.clone(), py::sub(&py::mul(&py::mul(&v_S.clone(), &v_dq.clone())?, &f__N(v_d1.clone())?)?, &py::mul(&py::mul(&v_K.clone(), &v_dr.clone())?, &f__N(v_d2.clone())?)?)?, py::sub(&py::mul(&py::mul(&v_K.clone(), &v_dr.clone())?, &f__N(py::neg(&v_d2.clone())?)?)?, &py::mul(&py::mul(&v_S.clone(), &v_dq.clone())?, &f__N(py::neg(&v_d1.clone())?)?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_npv(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("npv"), py::sum(&{ let mut out = vec![]; for t2484 in py::iter(&(py::enumerate(&v_cash_flows.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t2485 = (t2484).items()?;
if t2485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2485[0].clone();
v_c = t2485[1].clone();
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
    for t3423 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3423;
        py::setitem(&v_y.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t3424 in py::range_iter(&[(v_i.clone()).i()?]) { let mut v_k: Py; 
v_k = t3424;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t3425 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t3425;
        py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t3426 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3426;
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
    v_L = { let mut out = vec![]; for t3412 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3412;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    v_tol = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3413 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3413;
out.push(py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }], true)?)?;
    for t3414 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3414;
        for t3415 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?]) {
            v_j = t3415;
            v_s = py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t3416 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t3416;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?;
            if ({ let t3417 = v_i.clone(); Py::Bool({ let t3418 = v_j.clone(); if py::eq(&t3417, &t3418) { true } else { false } }) }).truthy() {
                if ({ let t3419 = v_s.clone(); Py::Bool({ let t3420 = v_tol.clone(); if py::le(&t3419, &t3420)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t2824 = Py::Float(0.5); Py::Bool({ let t2825 = v_c.clone(); if py::lt(&t2824, &t2825)? { { let t2826 = Py::Int(1i64); if py::lt(&t2825, &t2826)? { true } else { false } } } else { false } }) }, Py::str("confidence is a decimal between 0.5 and 1 (95% = 0.95)"))?;
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cov(mut v_xs: Py, mut v_ys: Py, mut v_ddof: Py) -> R {
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let t3574 = (Py::tuple(vec![f__mean(v_xs.clone())?, f__mean(v_ys.clone())?])).items()?;
    if t3574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t3574[0].clone();
    v_my = t3574[1].clone();
    return Ok(py::truediv(&py::fsum(&{ let mut out = vec![]; for t3575 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3576 = (t3575).items()?;
if t3576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3576[0].clone();
v_y = t3576[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) })?, &py::sub(&py::len(&v_xs.clone())?, &v_ddof.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__d1(mut v_dividend: Py, mut v_next_dividend: Py, mut v_growth: Py) -> R {
    let _ = f__need({ let t1441 = { let t1443 = v_dividend.clone(); Py::Bool({ let t1444 = Py::None; if py::is(&t1443, &t1444) { true } else { false } }) }; Py::Bool({ let t1442 = { let t1445 = v_next_dividend.clone(); Py::Bool({ let t1446 = Py::None; if py::is(&t1445, &t1446) { true } else { false } }) }; if py::ne(&t1441, &t1442) { true } else { false } }) }, Py::str("give either the dividend just paid (D0) or the next dividend (D1), not both"))?;
    return Ok((if ({ let t1447 = v_next_dividend.clone(); Py::Bool({ let t1448 = Py::None; if !py::is(&t1447, &t1448) { true } else { false } }) }).truthy() { f__fin(Py::str("next_dividend"), v_next_dividend.clone())? } else { py::mul(&f__fin(Py::str("dividend"), v_dividend.clone())?, &py::add(&Py::Int(1i64), &v_growth.clone())?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dot(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t2948 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2949 = (t2948).items()?;
if t2949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2949[0].clone();
v_y = t2949[1].clone();
out.push(py::mul(&v_x.clone(), &v_y.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t3748 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3748.truthy() { t3748 } else { { let t3747 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3747.truthy() { t3747 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hnpv(mut v_rate: Py, mut v_cfs: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2981 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_rate.clone())?)?, Py::Float(0.0)])).items()?;
    if t2981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2981[0].clone();
    v_acc = t2981[1].clone();
    for t2982 in py::iter(&(py::reversed(&v_cfs.clone())?))? {
        v_c = t2982;
        v_acc = py::add(&py::mul(&v_acc.clone(), &v_v.clone())?, &v_c.clone())?;
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t3647 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3647.truthy() { t3647 } else { { let t3646 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3646.truthy() { t3646 } else { { let t3645 = py::math1("isfinite", &v_v.clone())?; if !t3645.truthy() { t3645 } else { { let t3643 = py::float(&v_v.clone())?; Py::Bool({ let t3644 = py::int(&v_v.clone())?; if py::eq(&t3643, &t3644) { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))?;
    v_v = py::int(&v_v.clone())?;
    let _ = f__need({ let t3648 = v_v.clone(); Py::Bool({ let t3649 = v_lo.clone(); if py::ge(&t3648, &t3649)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    let _ = f__need({ let t3654 = { let t3650 = v_hi.clone(); Py::Bool({ let t3651 = Py::None; if py::is(&t3650, &t3651) { true } else { false } }) }; if t3654.truthy() { t3654 } else { { let t3652 = v_v.clone(); Py::Bool({ let t3653 = v_hi.clone(); if py::le(&t3652, &t3653)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))?;
    return Ok(v_v.clone());
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
    let _ = f__need({ let t1747 = v_k.clone(); Py::Bool({ let t1748 = Py::Int(1i64); if py::ge(&t1747, &t1748)? { true } else { false } }) }, Py::str("the cash flows never change sign: there is no internal rate of return"))?;
    let _ = f__need({ let t1749 = v_k.clone(); Py::Bool({ let t1750 = Py::Int(1i64); if py::eq(&t1749, &t1750) { true } else { false } }) }, Py::str(&["the cash flows change sign ".to_string(), py::format(&v_k.clone(), "")?, " times: the internal rate of return is not unique, so none is given".to_string()].concat()))?;
    v_nz = { let mut out = vec![]; for t1751 in py::iter(&(v_cfs.clone()))? { let mut v_c: Py; 
v_c = t1751;
if ({ let t1752 = v_c.clone(); Py::Bool({ let t1753 = Py::Int(0i64); if py::ne(&t1752, &t1753) { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    v_first = { let t1754 = py::getitem(&v_nz.clone(), &Py::Int(0i64))?; Py::Bool({ let t1755 = Py::Int(0i64); if py::gt(&t1754, &t1755)? { true } else { false } }) };
    v_g = { let v_cfs = v_cfs.clone(); let v_first = v_first.clone(); let v_k = v_k.clone(); let v_nz = v_nz.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t1756 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t1757 = (t1756).items()?;
if t1757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t1757[0].clone();
v_c = t1757[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_v.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1758 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])).items()?;
    if t1758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1758[0].clone();
    v_hi = t1758[1].clone();
    while ({ let t1762 = { let t1764 = py::call(&v_g.clone(), &[v_hi.clone()])?; Py::Bool({ let t1765 = Py::Int(0i64); if py::gt(&t1764, &t1765)? { true } else { false } }) }; Py::Bool({ let t1763 = v_first.clone(); if py::eq(&t1762, &t1763) { true } else { false } }) }).truthy() {
        let t1759 = (Py::tuple(vec![v_hi.clone(), py::mul(&v_hi.clone(), &Py::Int(2i64))?])).items()?;
        if t1759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1759[0].clone();
        v_hi = t1759[1].clone();
        let _ = f__need({ let t1760 = v_hi.clone(); Py::Bool({ let t1761 = py::pow(&Py::Float(2.0), &Py::Int(60i64))?; if py::lt(&t1760, &t1761)? { true } else { false } }) }, Py::str("the internal rate of return is too close to -100%"))?;
    }
    for t1766 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t1766;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t1767 = v_mid.clone(); Py::Bool({ let t1768 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1768, &t1767)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t1769 = { let t1771 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t1772 = Py::Int(0i64); if py::gt(&t1771, &t1772)? { true } else { false } }) }; Py::Bool({ let t1770 = v_first.clone(); if py::eq(&t1769, &t1770) { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_r = py::sub(&py::truediv(&Py::Float(1.0), &py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?)?, &Py::Float(1.0))?;
    for t1773 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t1773;
        v_f = f__hnpv(v_r.clone(), v_cfs.clone())?;
        v_d = py::sum(&{ let mut out = vec![]; for t1776 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t1777 = (t1776).items()?;
if t1777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t1777[0].clone();
v_c = t1777[1].clone();
out.push(py::mul(&py::mul(&py::neg(&v_t.clone())?, &v_c.clone())?, &py::pow(&py::add(&Py::Float(1.0), &v_r.clone())?, &py::sub(&py::neg(&v_t.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?;
        if ({ let t1780 = { let t1778 = v_d.clone(); Py::Bool({ let t1779 = Py::Int(0i64); if py::eq(&t1778, &t1779) { true } else { false } }) }; if t1780.truthy() { t1780 } else { py::not(&py::math1("isfinite", &py::truediv(&v_f.clone(), &v_d.clone())?)?) } }).truthy() {
            break;
        }
        v_nr = py::sub(&v_r.clone(), &py::truediv(&v_f.clone(), &v_d.clone())?)?;
        if (py::not(&{ let t1785 = { let t1781 = v_nr.clone(); Py::Bool({ let t1782 = py::neg(&Py::Int(1i64))?; if py::gt(&t1781, &t1782)? { true } else { false } }) }; if !t1785.truthy() { t1785 } else { { let t1783 = py::abs(&f__hnpv(v_nr.clone(), v_cfs.clone())?)?; Py::Bool({ let t1784 = py::abs(&v_f.clone())?; if py::le(&t1783, &t1784)? { true } else { false } }) } } })).truthy() {
            break;
        }
        v_r = v_nr.clone();
    }
    return Ok(v_r.clone());
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
    let t2028 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("annual_rate"), v_annual_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t2028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2028[0].clone();
    v_r = t2028[1].clone();
    v_m = f__int(Py::str("payments_per_year"), v_payments_per_year.clone(), Py::Int(1i64), Py::Int(365i64))?;
    v_n = f__periods_of(f__pos(Py::str("years"), v_years.clone())?, v_m.clone(), Py::str("years"))?;
    v_i = py::truediv(&v_r.clone(), &v_m.clone())?;
    v_pmt = (if ({ let t2048 = v_i.clone(); Py::Bool({ let t2049 = Py::Int(0i64); if py::eq(&t2048, &t2049) { true } else { false } }) }).truthy() { py::truediv(&v_P.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_P.clone(), &v_i.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &py::neg(&v_n.clone())?)?)?)? });
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
    let _ = f__need({ let t3754 = v_v.clone(); Py::Bool({ let t3755 = Py::Int(0i64); if py::ge(&t3754, &t3755)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__npv_scale(mut v_rate: Py, mut v_cfs: Py) -> R {
    return Ok({ let t1798 = py::sum(&{ let mut out = vec![]; for t1796 in py::iter(&(py::enumerate(&v_cfs.clone(), 0)?))? { let mut v_c: Py; let mut v_t: Py; 
let t1797 = (t1796).items()?;
if t1797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t1797[0].clone();
v_c = t1797[1].clone();
out.push(py::mul(&py::abs(&v_c.clone())?, &py::pow(&py::add(&Py::Float(1.0), &v_rate.clone())?, &py::neg(&v_t.clone())?)?)?); } Py::list(out) }, None)?; if t1798.truthy() { t1798 } else { Py::Float(1.0) } });
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
    let t3005 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__pos(Py::str("strike"), v_strike.clone())?, f__pos(Py::str("time"), v_time.clone())?])).items()?;
    if t3005.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3005[0].clone();
    v_K = t3005[1].clone();
    v_T = t3005[2].clone();
    let t3009 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("dividend_yield"), v_dividend_yield.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3009.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3009[0].clone();
    v_q = t3009[1].clone();
    v_s = f__pos(Py::str("volatility"), v_volatility.clone())?;
    let _ = f__need({ let t3010 = v_s.clone(); Py::Bool({ let t3011 = Py::Int(5i64); if py::lt(&t3010, &t3011)? { true } else { false } }) }, Py::str("volatility is a decimal (20% = 0.2)"))?;
    return Ok(Py::tuple(vec![v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair(mut v_a: Py, mut v_b: Py, mut v_na: Py, mut v_nb: Py, mut v_n_min: Py) -> R {
    let t3567 = (Py::tuple(vec![f__returns(v_na.clone(), v_a.clone(), v_n_min.clone(), Py::Float(1.0))?, f__returns(v_nb.clone(), v_b.clone(), v_n_min.clone(), Py::Float(1.0))?])).items()?;
    if t3567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3567[0].clone();
    v_b = t3567[1].clone();
    let _ = f__need({ let t3568 = py::len(&v_a.clone())?; Py::Bool({ let t3569 = py::len(&v_b.clone())?; if py::eq(&t3568, &t3569) { true } else { false } }) }, Py::str(&[py::format(&v_na.clone(), "")?, " and ".to_string(), py::format(&v_nb.clone(), "")?, " must have the same number of observations".to_string()].concat()))?;
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__payback(mut v_flows: Py) -> R {
    let mut v_cum: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t2605 = py::getitem(&v_flows.clone(), &Py::Int(0i64))?; Py::Bool({ let t2606 = Py::Int(0i64); if py::lt(&t2605, &t2606)? { true } else { false } }) }, Py::str("the first cash flow (the investment at time 0) must be negative"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2607 in py::iter(&(py::slice(&v_flows.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2607;
out.push({ let t2608 = v_x.clone(); Py::Bool({ let t2609 = Py::Int(0i64); if py::gt(&t2608, &t2609)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the payback period is defined here for cash flows after time 0 that are all positive"))?;
    v_cum = py::getitem(&v_flows.clone(), &Py::Int(0i64))?;
    for t2610 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_flows.clone())?).i()?]) {
        v_t = t2610;
        let t2611 = (Py::tuple(vec![v_cum.clone(), py::add(&v_cum.clone(), &py::getitem(&v_flows.clone(), &v_t.clone())?)?])).items()?;
        if t2611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_prev = t2611[0].clone();
        v_cum = t2611[1].clone();
        if ({ let t2612 = v_cum.clone(); Py::Bool({ let t2613 = Py::Int(0i64); if py::ge(&t2612, &t2613)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t2041 = v_years.clone(); Py::Bool({ let t2042 = Py::Int(1000i64); if py::le(&t2041, &t2042)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be at most 1000".to_string()].concat()))?;
    v_n = py::mul(&v_years.clone(), &v_per_year.clone())?;
    let _ = f__need({ let t2047 = { let t2043 = py::abs(&py::sub(&v_n.clone(), &py::round(&v_n.clone(), None)?)?)?; Py::Bool({ let t2044 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_n.clone())?], true)?)?; if py::le(&t2043, &t2044)? { true } else { false } }) }; if !t2047.truthy() { t2047 } else { { let t2045 = py::round(&v_n.clone(), None)?; Py::Bool({ let t2046 = Py::Int(1i64); if py::ge(&t2045, &t2046)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " times the number of payments per year must be a whole number of periods (at least 1)".to_string()].concat()))?;
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
    let _ = f__need({ let t3749 = v_v.clone(); Py::Bool({ let t3750 = Py::Int(0i64); if py::gt(&t3749, &t3750)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prob(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t3687 = v_lo.clone(); Py::Bool({ let t3688 = v_v.clone(); if py::le(&t3687, &t3688)? { { let t3689 = v_hi.clone(); if py::le(&t3688, &t3689)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ps(mut v_c: Py, mut v_d: Py, mut v_P: Py) -> R {
    return Ok((if ({ let t2649 = v_c.clone(); Py::Bool({ let t2650 = Py::Int(0i64); if py::ge(&t2649, &t2650)? { true } else { false } }) }).truthy() { py::add(&py::mul(&v_c.clone(), &v_P.clone())?, &py::truediv(&py::mul(&py::mul(&v_d.clone(), &v_P.clone())?, &v_P.clone())?, &Py::Int(2i64))?)? } else { py::truediv(&py::mul(&v_d.clone(), &py::pow(&py::add(&v_P.clone(), &py::truediv(&v_c.clone(), &v_d.clone())?)?, &Py::Int(2i64))?)?, &Py::Int(2i64))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qtab(mut v_q: Py, mut v_terminal: Py) -> R {
    v_q = f__vec(Py::str("mortality_rates"), v_q.clone(), Py::Int(1i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3714 in py::iter(&(v_q.clone()))? { let mut v_x: Py; 
v_x = t3714;
out.push({ let t3715 = Py::Int(0i64); Py::Bool({ let t3716 = v_x.clone(); if py::le(&t3715, &t3716)? { { let t3717 = Py::Int(1i64); if py::le(&t3716, &t3717)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("each mortality rate q is a probability between 0 and 1"))?;
    let _ = f__need({ let t3720 = py::not(&v_terminal.clone()); if t3720.truthy() { t3720 } else { { let t3718 = py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3719 = Py::Int(1i64); if py::eq(&t3718, &t3719) { true } else { false } }) } } }, Py::str("the table must end with q = 1 (no one survives past its last age)"))?;
    return Ok(v_q.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quad(mut v_w: Py, mut v_S: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3434 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3434;
for t3435 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3435;
out.push(py::mul(&py::mul(&py::getitem(&v_w.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_S.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_w.clone(), &v_j.clone())?)?); }} Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rate(mut v_name: Py, mut v_v: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    v_v = f__fin(v_name.clone(), v_v.clone())?;
    let _ = f__need({ let t3751 = v_lo.clone(); Py::Bool({ let t3752 = v_v.clone(); if py::lt(&t3751, &t3752)? { { let t3753 = v_hi.clone(); if py::lt(&t3752, &t3753)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a decimal rate between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?, " (write 5% or 0.05)".to_string()].concat()))?;
    return Ok(v_v.clone());
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
    for t3762 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3777 = (t3762).items()?;
        if t3777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3777[0].clone();
        v_w = t3777[1].clone();
        if ({ let t3763 = v_k.clone(); Py::Bool({ let t3764 = v_got.clone(); if !py::contains(&t3764, &t3763)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_o = py::getitem(&v_got.clone(), &v_k.clone())?;
        if (Py::Bool(matches!(v_w.clone(), Py::Str(_)))).truthy() {
            if ({ let t3765 = v_o.clone(); Py::Bool({ let t3766 = v_w.clone(); if py::ne(&t3765, &t3766) { true } else { false } }) }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            continue;
        }
        v_pairs = (if (Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_)))).truthy() { Py::list((py::zip(&[v_o.clone(), v_w.clone()])?).items()?) } else { Py::list(vec![Py::tuple(vec![v_o.clone(), v_w.clone()])]) });
        if ({ let t3770 = Py::Bool(matches!(v_w.clone(), Py::List(_) | Py::Tuple(_))); if !t3770.truthy() { t3770 } else { { let t3769 = py::not(&Py::Bool(matches!(v_o.clone(), Py::List(_) | Py::Tuple(_)))); if t3769.truthy() { t3769 } else { { let t3767 = py::len(&v_o.clone())?; Py::Bool({ let t3768 = py::len(&v_w.clone())?; if py::ne(&t3767, &t3768) { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        for t3771 in py::iter(&(v_pairs.clone()))? {
            let t3776 = (t3771).items()?;
            if t3776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t3776[0].clone();
            v_b = t3776[1].clone();
            if ({ let t3773 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t3773.truthy() { t3773 } else { { let t3772 = py::not(&Py::Bool(matches!(v_a.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3772.truthy() { t3772 } else { py::not(&py::math1("isfinite", &v_a.clone())?) } } } }).truthy() {
                return Ok(Py::Float(f64::INFINITY));
            }
            v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
            v_worst = py::minmax(&[v_worst.clone(), (if ({ let t3774 = v_d.clone(); Py::Bool({ let t3775 = Py::Float(1e-15); if py::le(&t3774, &t3775)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? })], true)?;
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__returns(mut v_name: Py, mut v_xs: Py, mut v_n_min: Py, mut v_hi: Py) -> R {
    v_xs = f__vec(v_name.clone(), v_xs.clone(), v_n_min.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3606 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3606;
out.push({ let t3607 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t3608 = v_x.clone(); if py::lt(&t3607, &t3608)? { { let t3609 = v_hi.clone(); if py::lt(&t3608, &t3609)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, ": each return is a decimal between -100% and ".to_string(), py::format(&v_hi.clone(), ".0%")?, " (write 5% or 0.05)".to_string()].concat()))?;
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
    v_s = { let mut out = vec![]; for t1736 in py::iter(&(v_cfs.clone()))? { let mut v_c: Py; 
v_c = t1736;
if ({ let t1737 = v_c.clone(); Py::Bool({ let t1738 = Py::Int(0i64); if py::ne(&t1737, &t1738) { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    return Ok(py::sum(&{ let mut out = vec![]; for t1739 in py::iter(&(py::zip(&[v_s.clone(), py::slice(&v_s.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1740 = (t1739).items()?;
if t1740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1740[0].clone();
v_b = t1740[1].clone();
if ({ let t1741 = { let t1743 = v_a.clone(); Py::Bool({ let t1744 = Py::Int(0i64); if py::gt(&t1743, &t1744)? { true } else { false } }) }; Py::Bool({ let t1742 = { let t1745 = v_b.clone(); Py::Bool({ let t1746 = Py::Int(0i64); if py::gt(&t1745, &t1746)? { true } else { false } }) }; if py::ne(&t1741, &t1742) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__square(mut v_name: Py, mut v_A: Py, mut v_n: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let _ = f__need({ let t3388 = Py::Bool(matches!(v_A.clone(), Py::List(_) | Py::Tuple(_))); if !t3388.truthy() { t3388 } else { { let t3387 = { let t3384 = py::len(&v_A.clone())?; Py::Bool({ let t3385 = Py::Int(1i64); if py::ge(&t3384, &t3385)? { true } else { false } }) }; if !t3387.truthy() { t3387 } else { Py::Bool(({ let mut out = vec![]; for t3386 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3386;
out.push(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_)))); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix".to_string()].concat()))?;
    v_k = py::len(&v_A.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3389 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3389;
out.push({ let t3390 = py::len(&v_r.clone())?; Py::Bool({ let t3391 = v_k.clone(); if py::eq(&t3390, &t3391) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be square".to_string()].concat()))?;
    let _ = f__need({ let t3396 = { let t3392 = v_n.clone(); Py::Bool({ let t3393 = Py::None; if py::is(&t3392, &t3393) { true } else { false } }) }; if t3396.truthy() { t3396 } else { { let t3394 = v_k.clone(); Py::Bool({ let t3395 = v_n.clone(); if py::eq(&t3394, &t3395) { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be ".to_string(), py::format(&v_n.clone(), "")?, " x ".to_string(), py::format(&v_n.clone(), "")?, " (one row and column per asset)".to_string()].concat()))?;
    v_M = { let mut out = vec![]; for t3397 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3397;
out.push({ let mut out = vec![]; for t3398 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3398;
out.push(f__fin(v_name.clone(), v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_scale = { let t3401 = py::minmax(&[{ let mut out = vec![]; for t3399 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3399;
for t3400 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3400;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; if t3401.truthy() { t3401 } else { Py::Float(1.0) } };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3402 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t3402;
for t3403 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3403;
out.push({ let t3404 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_j.clone())?, &v_i.clone())?)?)?; Py::Bool({ let t3405 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t3404, &t3405)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3406 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t3406;
out.push({ let t3407 = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?; Py::Bool({ let t3408 = Py::Int(0i64); if py::ge(&t3407, &t3408)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, ": variances on the diagonal must not be negative".to_string()].concat()))?;
    return Ok(v_M.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__surv(mut v_q: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_p = Py::list(vec![Py::Float(1.0)]);
    for t3727 in py::iter(&(v_q.clone()))? {
        v_x = t3727;
        let _ = py::method(&v_p.clone(), "append", &[py::mul(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?])?;
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__timing(mut v_t: Py) -> R {
    let _ = f__need({ let t3170 = v_t.clone(); Py::Bool({ let t3171 = Py::tuple(vec![Py::str("ordinary"), Py::str("due")]); if py::contains(&t3171, &t3170)? { true } else { false } }) }, Py::str("timing is 'ordinary' (end of period) or 'due' (beginning of period)"))?;
    return Ok((if ({ let t3172 = v_t.clone(); Py::Bool({ let t3173 = Py::str("due"); if py::eq(&t3172, &t3173) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }));
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
    let _ = f__need({ let t3710 = Py::Bool(matches!(v_xs.clone(), Py::List(_) | Py::Tuple(_))); if !t3710.truthy() { t3710 } else { { let t3708 = py::len(&v_xs.clone())?; Py::Bool({ let t3709 = v_n_min.clone(); if py::ge(&t3708, &t3709)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " values".to_string()].concat()))?;
    return Ok({ let mut out = vec![]; for t3711 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3711;
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
    if ({ let t870 = py::not(&Py::Bool(matches!(v_r.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t870.truthy() { t870 } else { { let t869 = py::not(&py::math1("isfinite", &v_r.clone())?); if t869.truthy() { t869 } else { { let t867 = v_r.clone(); Py::Bool({ let t868 = py::neg(&Py::Int(1i64))?; if py::le(&t867, &t868)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t871 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("cash_flows_a"))?, py::getitem(&v_inputs.clone(), &Py::str("cash_flows_b"))?])).items()?;
    if t871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t871[0].clone();
    v_b = t871[1].clone();
    v_d = { let mut out = vec![]; for t872 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t873 = (t872).items()?;
if t873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t873[0].clone();
v_y = t873[1].clone();
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
    if ({ let t1795 = py::not(&Py::Bool(matches!(v_r.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1795.truthy() { t1795 } else { { let t1794 = py::not(&py::math1("isfinite", &v_r.clone())?); if t1794.truthy() { t1794 } else { { let t1792 = v_r.clone(); Py::Bool({ let t1793 = py::neg(&Py::Int(1i64))?; if py::le(&t1792, &t1793)? { true } else { false } }) } } } } }).truthy() {
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
    if ({ let t1632 = py::not(&Py::Bool(matches!(v_s.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1632.truthy() { t1632 } else { { let t1631 = py::not(&py::math1("isfinite", &v_s.clone())?); if t1631.truthy() { t1631 } else { { let t1629 = v_s.clone(); Py::Bool({ let t1630 = Py::Int(0i64); if py::le(&t1629, &t1630)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t1633 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("spot"))?, py::getitem(&v_inputs.clone(), &Py::str("strike"))?, py::getitem(&v_inputs.clone(), &Py::str("time"))?, py::getitem(&v_inputs.clone(), &Py::str("rate"))?])).items()?;
    if t1633.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1633[0].clone();
    v_K = t1633[1].clone();
    v_T = t1633[2].clone();
    v_r = t1633[3].clone();
    v_q = py::method(&v_inputs.clone(), "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?;
    v_v = py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone())?, &(if ({ let t1634 = py::method(&v_inputs.clone(), "get", &[Py::str("option_type"), Py::str("call")])?; Py::Bool({ let t1635 = Py::str("call"); if py::eq(&t1634, &t1635) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(3i64) }))?;
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
fn f_compound_interest(mut v_principal: Py, mut v_rate: Py, mut v_years: Py, mut v_periods_per_year: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t649 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t649.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t649[0].clone();
    v_r = t649[1].clone();
    v_t = t649[2].clone();
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
    let t689 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t689.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t689[0].clone();
    v_r = t689[1].clone();
    v_t = t689[2].clone();
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
    let t714 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t714[0].clone();
    v_Y = t714[1].clone();
    let _ = f__need({ let t715 = py::len(&v_X.clone())?; Py::Bool({ let t716 = py::len(&v_Y.clone())?; if py::eq(&t715, &t716) { true } else { false } }) }, Py::str("x and y need the same number of observations"))?;
    let t720 = (Py::tuple(vec![f__cov(v_X.clone(), v_X.clone(), Py::Int(1i64))?, f__cov(v_Y.clone(), v_Y.clone(), Py::Int(1i64))?])).items()?;
    if t720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t720[0].clone();
    v_syy = t720[1].clone();
    let _ = f__need({ let t725 = { let t721 = v_sxx.clone(); Py::Bool({ let t722 = Py::Int(0i64); if py::gt(&t721, &t722)? { true } else { false } }) }; if !t725.truthy() { t725 } else { { let t723 = v_syy.clone(); Py::Bool({ let t724 = Py::Int(0i64); if py::gt(&t723, &t724)? { true } else { false } }) } } }, Py::str("neither variable may be constant"))?;
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
    let t748 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t748[0].clone();
    v_B = t748[1].clone();
    let t751 = (Py::tuple(vec![f__nonneg(Py::str("cost_1"), v_cost_1.clone())?, f__nonneg(Py::str("cost_2"), v_cost_2.clone())?])).items()?;
    if t751.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c1 = t751[0].clone();
    v_c2 = t751[1].clone();
    let t752 = (Py::tuple(vec![py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_c1.clone())?)?, &v_c2.clone())?, &py::mul(&Py::Int(3i64), &v_B.clone())?)?, py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_c2.clone())?)?, &v_c1.clone())?, &py::mul(&Py::Int(3i64), &v_B.clone())?)?])).items()?;
    if t752.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q1 = t752[0].clone();
    v_q2 = t752[1].clone();
    let _ = f__need({ let t757 = { let t753 = v_q1.clone(); Py::Bool({ let t754 = Py::Int(0i64); if py::gt(&t753, &t754)? { true } else { false } }) }; if !t757.truthy() { t757 } else { { let t755 = v_q2.clone(); Py::Bool({ let t756 = Py::Int(0i64); if py::gt(&t755, &t756)? { true } else { false } }) } } }, Py::str("one firm would not produce: the costs are too different for an interior Cournot equilibrium"))?;
    v_P = py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &py::add(&v_q1.clone(), &v_q2.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("q1"), v_q1.clone()), (Py::str("q2"), v_q2.clone()), (Py::str("price"), v_P.clone()), (Py::str("profit_1"), py::mul(&py::sub(&v_P.clone(), &v_c1.clone())?, &v_q1.clone())?), (Py::str("profit_2"), py::mul(&py::sub(&v_P.clone(), &v_c2.clone())?, &v_q2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cross_price_elasticity(mut v_price_other_1: Py, mut v_quantity_1: Py, mut v_price_other_2: Py, mut v_quantity_2: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t783 = (f__arc(v_price_other_1.clone(), v_quantity_1.clone(), v_price_other_2.clone(), v_quantity_2.clone(), Py::str("price_other"), Py::str("quantity"))?).items()?;
    if t783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t783[0].clone();
    v_ex = t783[1].clone();
    return Ok(Py::dict(vec![(Py::str("elasticity"), v_e.clone()), (Py::str("relationship"), (if ({ let t784 = v_ex.clone(); Py::Bool({ let t785 = Py::Int(0i64); if py::gt(&t784, &t785)? { true } else { false } }) }).truthy() { Py::str("substitutes") } else { (if ({ let t786 = v_ex.clone(); Py::Bool({ let t787 = Py::Int(0i64); if py::lt(&t786, &t787)? { true } else { false } }) }).truthy() { Py::str("complements") } else { Py::str("independent") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crossover_rate(mut v_cash_flows_a: Py, mut v_cash_flows_b: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t812 = (Py::tuple(vec![f__vec(Py::str("cash_flows_a"), v_cash_flows_a.clone(), Py::Int(2i64))?, f__vec(Py::str("cash_flows_b"), v_cash_flows_b.clone(), Py::Int(2i64))?])).items()?;
    if t812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t812[0].clone();
    v_b = t812[1].clone();
    let _ = f__need({ let t813 = py::len(&v_a.clone())?; Py::Bool({ let t814 = py::len(&v_b.clone())?; if py::eq(&t813, &t814) { true } else { false } }) }, Py::str("the two projects need cash flows over the same periods (lists of equal length)"))?;
    v_d = { let mut out = vec![]; for t815 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t816 = (t815).items()?;
if t816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t816[0].clone();
v_y = t816[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("crossover_rate"), f__irr(v_d.clone())?), (Py::str("npv_at_crossover"), f__hnpv(f__irr(v_d.clone())?, v_a.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discounted_payback(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t886 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t886.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t886[0].clone();
    v_c = t886[1].clone();
    return Ok(Py::dict(vec![(Py::str("discounted_payback"), f__payback({ let mut out = vec![]; for t896 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t897 = (t896).items()?;
if t897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t897[0].clone();
v_x = t897[1].clone();
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
    v_den = py::fsum(&{ let mut out = vec![]; for t945 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t945;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    let _ = f__need({ let t946 = v_den.clone(); Py::Bool({ let t947 = Py::Int(0i64); if py::gt(&t946, &t947)? { true } else { false } }) }, Py::str("the residuals are all zero"))?;
    return Ok(Py::dict(vec![(Py::str("durbin_watson"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t948 in py::iter(&(py::zip(&[v_e.clone(), py::slice(&v_e.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t949 = (t948).items()?;
if t949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t949[0].clone();
v_b = t949[1].clone();
out.push(py::pow(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_den.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_annual_rate(mut v_rate: Py, mut v_periods_per_year: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t985 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t985[0].clone();
    v_m = t985[1].clone();
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
    let t1022 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t1022.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1022[0].clone();
    v_v = t1022[1].clone();
    v_B = t1022[2].clone();
    v_n = f__int(Py::str("term"), v_term.clone(), Py::Int(1i64), py::len(&v_q.clone())?)?;
    let t1037 = (Py::tuple(vec![f__A(v_q.clone(), v_v.clone(), v_n.clone())?, py::mul(&py::pow(&v_v.clone(), &v_n.clone())?, &py::getitem(&f__surv(v_q.clone())?, &v_n.clone())?)?])).items()?;
    if t1037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t1037[0].clone();
    v_pe = t1037[1].clone();
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
    let t1065 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t1065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1065[0].clone();
    v_c = t1065[1].clone();
    let t1068 = (Py::tuple(vec![py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?, f__hnpv(v_r.clone(), v_c.clone())?])).items()?;
    if t1068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1068[0].clone();
    v_v = t1068[1].clone();
    return Ok(Py::dict(vec![(Py::str("eaa"), (if ({ let t1069 = v_r.clone(); Py::Bool({ let t1070 = Py::Int(0i64); if py::eq(&t1069, &t1070) { true } else { false } }) }).truthy() { py::truediv(&v_v.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_v.clone(), &v_r.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &py::neg(&v_n.clone())?)?)?)? })), (Py::str("npv"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fisher_real_rate(mut v_nominal_rate: Py, mut v_inflation_rate: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let t1094 = (Py::tuple(vec![f__rate(Py::str("nominal_rate"), v_nominal_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("inflation_rate"), v_inflation_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i = t1094[0].clone();
    v_pi = t1094[1].clone();
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
    let t1119 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__nonneg(Py::str("time"), v_time.clone())?])).items()?;
    if t1119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1119[0].clone();
    v_T = t1119[1].clone();
    let t1123 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("dividend_yield"), v_dividend_yield.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("storage_cost"), v_storage_cost.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1123.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1123[0].clone();
    v_q = t1123[1].clone();
    v_u = t1123[2].clone();
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
    let t1149 = (Py::tuple(vec![f__pos(Py::str("spot"), v_spot.clone())?, f__pos(Py::str("time"), v_time.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1149.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1149[0].clone();
    v_T = t1149[1].clone();
    v_r = t1149[2].clone();
    let t1154 = (Py::tuple(vec![f__vec(Py::str("dividends"), v_dividends.clone(), Py::Int(1i64))?, f__vec(Py::str("dividend_times"), v_dividend_times.clone(), Py::Int(1i64))?])).items()?;
    if t1154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1154[0].clone();
    v_t = t1154[1].clone();
    let _ = f__need({ let t1155 = py::len(&v_D.clone())?; Py::Bool({ let t1156 = py::len(&v_t.clone())?; if py::eq(&t1155, &t1156) { true } else { false } }) }, Py::str("one time per dividend"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1157 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1157;
out.push({ let t1158 = Py::Int(0i64); Py::Bool({ let t1159 = v_x.clone(); if py::lt(&t1158, &t1159)? { { let t1160 = v_T.clone(); if py::le(&t1159, &t1160)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("each dividend time must lie in (0, time]"))?;
    v_pv = py::fsum(&{ let mut out = vec![]; for t1161 in py::iter(&(py::zip(&[v_D.clone(), v_t.clone()])?))? { let mut v_d: Py; let mut v_x: Py; 
let t1162 = (t1161).items()?;
if t1162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t1162[0].clone();
v_x = t1162[1].clone();
out.push(py::mul(&v_d.clone(), &py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_x.clone())?)?)?); } Py::list(out) })?;
    let _ = f__need({ let t1163 = v_pv.clone(); Py::Bool({ let t1164 = v_S.clone(); if py::lt(&t1163, &t1164)? { true } else { false } }) }, Py::str("the dividends are worth more than the asset"))?;
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
    let t1190 = (Py::tuple(vec![f__rate(Py::str("spot_rate_1"), v_spot_rate_1.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("maturity_1"), v_maturity_1.clone())?])).items()?;
    if t1190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t1190[0].clone();
    v_t1 = t1190[1].clone();
    let t1193 = (Py::tuple(vec![f__rate(Py::str("spot_rate_2"), v_spot_rate_2.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__pos(Py::str("maturity_2"), v_maturity_2.clone())?])).items()?;
    if t1193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s2 = t1193[0].clone();
    v_t2 = t1193[1].clone();
    let _ = f__need({ let t1194 = v_t2.clone(); Py::Bool({ let t1195 = v_t1.clone(); if py::gt(&t1194, &t1195)? { true } else { false } }) }, Py::str("maturity_2 must be later than maturity_1"))?;
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
    let t1235 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t1235.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1235[0].clone();
    v_r = t1235[1].clone();
    v_n = t1235[2].clone();
    v_due = t1235[3].clone();
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
    let t1281 = (f__baskets(v_base_prices.clone(), v_base_quantities.clone(), v_current_prices.clone(), v_current_quantities.clone())?).items()?;
    if t1281.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t1281[0].clone();
    v_q0 = t1281[1].clone();
    v_p1 = t1281[2].clone();
    v_q1 = t1281[3].clone();
    let t1284 = (Py::tuple(vec![f__dot(v_p1.clone(), v_q1.clone())?, f__dot(v_p0.clone(), v_q1.clone())?, f__dot(v_p0.clone(), v_q0.clone())?])).items()?;
    if t1284.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_nom = t1284[0].clone();
    v_real = t1284[1].clone();
    v_real0 = t1284[2].clone();
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
    let _ = f__need({ let t1314 = Py::Bool(({ let mut out = vec![]; for t1309 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1309;
out.push({ let t1310 = v_v.clone(); Py::Bool({ let t1311 = Py::Int(0i64); if py::ge(&t1310, &t1311)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1314.truthy() { t1314 } else { { let t1312 = py::sum(&v_x.clone(), None)?; Py::Bool({ let t1313 = Py::Int(0i64); if py::gt(&t1312, &t1313)? { true } else { false } }) } } }, Py::str("incomes must be non-negative with a positive total"))?;
    let t1315 = (Py::tuple(vec![py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?, py::len(&v_x.clone())?])).items()?;
    if t1315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1315[0].clone();
    v_n = t1315[1].clone();
    return Ok(Py::dict(vec![(Py::str("gini"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t1316 in py::iter(&(py::enumerate(&v_s.clone(), (Py::Int(1i64)).i()?)?))? { let mut v_i: Py; let mut v_v: Py; 
let t1317 = (t1316).items()?;
if t1317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t1317[0].clone();
v_v = t1317[1].clone();
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
    let _ = f__need({ let t1372 = v_L.clone(); Py::Bool({ let t1373 = Py::None; if !py::is(&t1372, &t1373) { true } else { false } }) }, Py::str("the covariance matrix must be positive definite (no riskless combination of the assets)"))?;
    v_x = f__chol_solve(v_L.clone(), py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_S.clone())?)?)?;
    v_w = { let mut out = vec![]; for t1378 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1378;
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
    let t1404 = (Py::tuple(vec![f__rate(Py::str("required_return"), v_required_return.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1404.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1404[0].clone();
    v_g = t1404[1].clone();
    let _ = f__need({ let t1405 = v_r.clone(); Py::Bool({ let t1406 = v_g.clone(); if py::gt(&t1405, &t1406)? { true } else { false } }) }, Py::str("the required return must exceed the growth rate (otherwise the value is infinite)"))?;
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
    let t1440 = (Py::tuple(vec![f__pos(Py::str("price"), v_price.clone())?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1440[0].clone();
    v_g = t1440[1].clone();
    v_D1 = f__d1(v_dividend.clone(), v_next_dividend.clone(), v_g.clone())?;
    let _ = f__need({ let t1449 = v_D1.clone(); Py::Bool({ let t1450 = Py::Int(0i64); if py::gt(&t1449, &t1450)? { true } else { false } }) }, Py::str("the dividend must be positive"))?;
    v_r = py::add(&py::truediv(&v_D1.clone(), &v_P.clone())?, &v_g.clone())?;
    let _ = f__need({ let t1451 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1452 = v_r.clone(); if py::lt(&t1451, &t1452)? { { let t1453 = Py::Int(1i64); if py::lt(&t1452, &t1453)? { true } else { false } } } else { false } }) }, Py::str("the implied required return is outside -100%..100%"))?;
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
    let t1489 = (Py::tuple(vec![f__fin(Py::str("first_payment"), v_first_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1489.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1489[0].clone();
    v_r = t1489[1].clone();
    v_g = t1489[2].clone();
    v_n = t1489[3].clone();
    v_pv = (if ({ let t1490 = py::abs(&py::sub(&v_r.clone(), &v_g.clone())?)?; Py::Bool({ let t1491 = Py::Float(1e-15); if py::lt(&t1490, &t1491)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_c.clone(), &v_n.clone())?, &py::add(&Py::Int(1i64), &v_r.clone())?)? } else { py::mul(&py::truediv(&v_c.clone(), &py::sub(&v_r.clone(), &v_g.clone())?)?, &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&py::add(&Py::Int(1i64), &v_g.clone())?, &py::add(&Py::Int(1i64), &v_r.clone())?)?, &v_n.clone())?)?)? });
    return Ok(Py::dict(vec![(Py::str("present_value"), v_pv.clone()), (Py::str("future_value"), py::mul(&v_pv.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_growing_perpetuity(mut v_first_payment: Py, mut v_rate: Py, mut v_growth: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1515 = (Py::tuple(vec![f__fin(Py::str("first_payment"), v_first_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("growth"), v_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t1515.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1515[0].clone();
    v_r = t1515[1].clone();
    v_g = t1515[2].clone();
    let _ = f__need({ let t1516 = v_r.clone(); Py::Bool({ let t1517 = v_g.clone(); if py::gt(&t1516, &t1517)? { true } else { false } }) }, Py::str("the discount rate must exceed the growth rate (otherwise the value is infinite)"))?;
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
    let _ = f__need({ let t1547 = Py::Bool(({ let mut out = vec![]; for t1542 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1542;
out.push({ let t1543 = v_v.clone(); Py::Bool({ let t1544 = Py::Int(0i64); if py::ge(&t1543, &t1544)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1547.truthy() { t1547 } else { { let t1545 = py::sum(&v_x.clone(), None)?; Py::Bool({ let t1546 = Py::Int(0i64); if py::gt(&t1545, &t1546)? { true } else { false } }) } } }, Py::str("shares (or sales) must be non-negative with a positive total"))?;
    v_tot = py::fsum(&v_x.clone())?;
    v_s = { let mut out = vec![]; for t1548 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1548;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_h = py::fsum(&{ let mut out = vec![]; for t1549 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t1549;
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
    let t1612 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), Py::Float(0.2), v_dividend_yield.clone())?).items()?;
    if t1612.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1612[0].clone();
    v_K = t1612[1].clone();
    v_T = t1612[2].clone();
    v_r = t1612[3].clone();
    v__s = t1612[4].clone();
    v_q = t1612[5].clone();
    v_p = f__pos(Py::str("option_price"), v_option_price.clone())?;
    let _ = f__need({ let t1613 = v_option_type.clone(); Py::Bool({ let t1614 = Py::tuple(vec![Py::str("call"), Py::str("put")]); if py::contains(&t1614, &t1613)? { true } else { false } }) }, Py::str("option_type is call or put"))?;
    v_f = { let v_K = v_K.clone(); let v_S = v_S.clone(); let v_T = v_T.clone(); let v__s = v__s.clone(); let v_dividend_yield = v_dividend_yield.clone(); let v_option_price = v_option_price.clone(); let v_option_type = v_option_type.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_rate = v_rate.clone(); let v_spot = v_spot.clone(); let v_strike = v_strike.clone(); let v_time = v_time.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_s.clone(), v_q.clone())?, &(if ({ let t1616 = v_option_type.clone(); Py::Bool({ let t1617 = Py::str("call"); if py::eq(&t1616, &t1617) { true } else { false } }) }).truthy() { Py::Int(2i64) } else { Py::Int(3i64) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1618 = (Py::tuple(vec![Py::Float(0.0001), Py::Float(5.0)])).items()?;
    if t1618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1618[0].clone();
    v_hi = t1618[1].clone();
    let _ = f__need({ let t1619 = py::call(&v_f.clone(), &[v_lo.clone()])?; Py::Bool({ let t1620 = v_p.clone(); if py::lt(&t1619, &t1620)? { { let t1621 = py::call(&v_f.clone(), &[v_hi.clone()])?; if py::lt(&t1620, &t1621)? { true } else { false } } } else { false } }) }, Py::str("the price is outside the range Black-Scholes can produce (volatility 0.01% to 500%): it violates the no-arbitrage bounds or implies an extreme volatility"))?;
    for t1622 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1622;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t1623 = v_mid.clone(); Py::Bool({ let t1624 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1624, &t1623)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t1625 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t1626 = v_p.clone(); if py::lt(&t1625, &t1626)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_sig = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    v_vega = py::mul(&py::mul(&py::mul(&v_S.clone(), &py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?)?, &f__phi(py::getitem(&f__bs(v_S.clone(), v_K.clone(), v_T.clone(), v_r.clone(), v_sig.clone(), v_q.clone())?, &Py::Int(0i64))?)?)?, &py::math1("sqrt", &v_T.clone())?)?;
    let _ = f__need({ let t1627 = v_vega.clone(); Py::Bool({ let t1628 = py::mul(&Py::Float(0.0001), &v_S.clone())?; if py::gt(&t1627, &t1628)? { true } else { false } }) }, Py::str("the option is too far from the money: its price barely depends on volatility, so the implied volatility is not well determined"))?;
    return Ok(Py::dict(vec![(Py::str("volatility"), v_sig.clone()), (Py::str("vega"), v_vega.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_income_elasticity(mut v_income_1: Py, mut v_quantity_1: Py, mut v_income_2: Py, mut v_quantity_2: Py) -> R {
    let mut v_cls: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let t1643 = (f__arc(v_income_1.clone(), v_quantity_1.clone(), v_income_2.clone(), v_quantity_2.clone(), Py::str("income"), Py::str("quantity"))?).items()?;
    if t1643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1643[0].clone();
    v_ex = t1643[1].clone();
    v_cls = (if ({ let t1644 = v_ex.clone(); Py::Bool({ let t1645 = Py::Int(0i64); if py::lt(&t1644, &t1645)? { true } else { false } }) }).truthy() { Py::str("inferior good") } else { (if ({ let t1646 = v_ex.clone(); Py::Bool({ let t1647 = Py::Int(0i64); if py::eq(&t1646, &t1647) { true } else { false } }) }).truthy() { Py::str("neutral") } else { (if ({ let t1648 = v_ex.clone(); Py::Bool({ let t1649 = Py::Int(1i64); if py::gt(&t1648, &t1649)? { true } else { false } }) }).truthy() { Py::str("normal good (luxury)") } else { Py::str("normal good (necessity)") }) }) });
    return Ok(Py::dict(vec![(Py::str("elasticity"), v_e.clone()), (Py::str("classification"), v_cls.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inflation_adjustment(mut v_price_levels: Py, mut v_amount: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__vec(Py::str("price_levels"), v_price_levels.clone(), Py::Int(2i64))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1674 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t1674;
out.push({ let t1675 = v_x.clone(); Py::Bool({ let t1676 = Py::Int(0i64); if py::gt(&t1675, &t1676)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("price levels must be positive"))?;
    v_n = py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?;
    v_out = Py::dict(vec![(Py::str("total_inflation"), py::sub(&py::truediv(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?), (Py::str("average_inflation"), py::sub(&py::pow(&py::truediv(&py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &py::truediv(&Py::Int(1i64), &v_n.clone())?)?, &Py::Int(1i64))?)]);
    if ({ let t1677 = v_amount.clone(); Py::Bool({ let t1678 = Py::None; if !py::is(&t1677, &t1678) { true } else { false } }) }).truthy() {
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
    let t1710 = (f__pair(v_portfolio_returns.clone(), v_benchmark_returns.clone(), Py::str("portfolio_returns"), Py::str("benchmark_returns"), Py::Int(2i64))?).items()?;
    if t1710.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1710[0].clone();
    v_b = t1710[1].clone();
    v_a = { let mut out = vec![]; for t1711 in py::iter(&(py::zip(&[v_p.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1712 = (t1711).items()?;
if t1712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1712[0].clone();
v_y = t1712[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) };
    v_te = f__sd(v_a.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t1716 = v_te.clone(); Py::Bool({ let t1717 = Py::Int(0i64); if py::gt(&t1716, &t1717)? { true } else { false } }) }, Py::str("the active returns are constant: the tracking error is zero"))?;
    return Ok(Py::dict(vec![(Py::str("information_ratio"), py::truediv(&f__mean(v_a.clone())?, &v_te.clone())?), (Py::str("tracking_error"), v_te.clone()), (Py::str("mean_active_return"), f__mean(v_a.clone())?)]));
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
    let t1817 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("payment"), v_payment.clone())?])).items()?;
    if t1817.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1817[0].clone();
    v_v = t1817[1].clone();
    v_P = t1817[2].clone();
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
    let t1899 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t1899.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1899[0].clone();
    v_i = t1899[1].clone();
    v_n = t1899[2].clone();
    v_pmt = t1899[3].clone();
    v_k = f__int(Py::str("payments_made"), v_payments_made.clone(), Py::Int(0i64), v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("balance"), (if ({ let t1900 = v_k.clone(); Py::Bool({ let t1901 = v_n.clone(); if py::lt(&t1900, &t1901)? { true } else { false } }) }).truthy() { py::minmax(&[Py::Float(0.0), f__balance(v_P.clone(), v_i.clone(), v_pmt.clone(), v_k.clone())?], true)? } else { Py::Float(0.0) })), (Py::str("payment"), v_pmt.clone())]));
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
    let t1951 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t1951.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1951[0].clone();
    v_i = t1951[1].clone();
    v_n = t1951[2].clone();
    v_pmt = t1951[3].clone();
    v_k = f__int(Py::str("payments_made"), v_payments_made.clone(), Py::Int(1i64), v_n.clone())?;
    v_repaid = py::sub(&v_P.clone(), &(if ({ let t1952 = v_k.clone(); Py::Bool({ let t1953 = v_n.clone(); if py::lt(&t1952, &t1953)? { true } else { false } }) }).truthy() { f__balance(v_P.clone(), v_i.clone(), v_pmt.clone(), v_k.clone())? } else { Py::Float(0.0) }))?;
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
    let t1981 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("annual_rate"), v_annual_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?])).items()?;
    if t1981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1981[0].clone();
    v_r = t1981[1].clone();
    v_m = f__int(Py::str("payments_per_year"), v_payments_per_year.clone(), Py::Int(1i64), Py::Int(365i64))?;
    v_n = f__periods_of(f__pos(Py::str("years"), v_years.clone())?, v_m.clone(), Py::str("years"))?;
    v_i = py::truediv(&v_r.clone(), &v_m.clone())?;
    v_pmt = (if ({ let t2001 = v_i.clone(); Py::Bool({ let t2002 = Py::Int(0i64); if py::eq(&t2001, &t2002) { true } else { false } }) }).truthy() { py::truediv(&v_P.clone(), &v_n.clone())? } else { py::truediv(&py::mul(&v_P.clone(), &v_i.clone())?, &py::sub(&Py::Int(1i64), &py::pow(&py::add(&Py::Int(1i64), &v_i.clone())?, &py::neg(&v_n.clone())?)?)?)? });
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
    let t2050 = (f__loan_state(v_principal.clone(), v_annual_rate.clone(), v_years.clone(), v_payments_per_year.clone())?).items()?;
    if t2050.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2050[0].clone();
    v_i = t2050[1].clone();
    v_n = t2050[2].clone();
    v_pmt = t2050[3].clone();
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
    let t2077 = (Py::tuple(vec![f__vec(Py::str("prices"), v_prices.clone(), Py::Int(3i64))?, f__vec(Py::str("quantities"), v_quantities.clone(), Py::Int(3i64))?])).items()?;
    if t2077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2077[0].clone();
    v_q = t2077[1].clone();
    let _ = f__need({ let t2078 = py::len(&v_p.clone())?; Py::Bool({ let t2079 = py::len(&v_q.clone())?; if py::eq(&t2078, &t2079) { true } else { false } }) }, Py::str("one quantity per price"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2080 in py::iter(&(py::add(&v_p.clone(), &v_q.clone())?))? { let mut v_v: Py; 
v_v = t2080;
out.push({ let t2081 = v_v.clone(); Py::Bool({ let t2082 = Py::Int(0i64); if py::gt(&t2081, &t2082)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("prices and quantities must be positive (logarithms)"))?;
    v_o = f_ols_simple({ let mut out = vec![]; for t2096 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2096;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t2097 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t2097;
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
    let t2642 = (Py::tuple(vec![f__pos(Py::str("demand_intercept"), v_demand_intercept.clone())?, f__pos(Py::str("demand_slope"), v_demand_slope.clone())?])).items()?;
    if t2642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2642[0].clone();
    v_b = t2642[1].clone();
    let t2643 = (Py::tuple(vec![f__fin(Py::str("supply_intercept"), v_supply_intercept.clone())?, f__pos(Py::str("supply_slope"), v_supply_slope.clone())?])).items()?;
    if t2643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2643[0].clone();
    v_d = t2643[1].clone();
    v_P = py::truediv(&py::sub(&v_a.clone(), &v_c.clone())?, &py::add(&v_b.clone(), &v_d.clone())?)?;
    v_Q = py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_P.clone())?)?;
    let _ = f__need({ let t2648 = { let t2644 = v_P.clone(); Py::Bool({ let t2645 = Py::Int(0i64); if py::gt(&t2644, &t2645)? { true } else { false } }) }; if !t2648.truthy() { t2648 } else { { let t2646 = v_Q.clone(); Py::Bool({ let t2647 = Py::Int(0i64); if py::gt(&t2646, &t2647)? { true } else { false } }) } } }, Py::str("the curves do not cross at a positive price and quantity"))?;
    let t2651 = (Py::tuple(vec![py::truediv(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::mul(&Py::Int(2i64), &v_b.clone())?)?, f__ps(v_c.clone(), v_d.clone(), v_P.clone())?])).items()?;
    if t2651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cs = t2651[0].clone();
    v_ps = t2651[1].clone();
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
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2154 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t2154;
out.push({ let t2155 = v_x.clone(); Py::Bool({ let t2156 = Py::Int(0i64); if py::gt(&t2155, &t2156)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("prices must be positive"))?;
    let t2157 = (Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, Py::Float(0.0)])).items()?;
    if t2157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_peak = t2157[0].clone();
    v_mdd = t2157[1].clone();
    for t2158 in py::iter(&(v_p.clone()))? {
        v_x = t2158;
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
    v_g = py::sub(&py::math1("exp", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2187 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2187;
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
    let t2210 = (Py::tuple(vec![f__pos(Py::str("sd_1"), v_sd_1.clone())?, f__pos(Py::str("sd_2"), v_sd_2.clone())?])).items()?;
    if t2210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2210[0].clone();
    v_s2 = t2210[1].clone();
    v_rho = f__prob(Py::str("correlation"), v_correlation.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_den = py::sub(&py::add(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_rho.clone())?, &v_s1.clone())?, &v_s2.clone())?)?;
    let _ = f__need({ let t2214 = v_den.clone(); Py::Bool({ let t2215 = py::mul(&Py::Float(1e-12), &py::add(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?)?; if py::gt(&t2214, &t2215)? { true } else { false } }) }, Py::str("the two assets are perfectly correlated with equal risk: every mix has the same variance"))?;
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
    let t2243 = (Py::tuple(vec![f__rate(Py::str("finance_rate"), v_finance_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("reinvest_rate"), v_reinvest_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t2243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t2243[0].clone();
    v_rr = t2243[1].clone();
    let _ = f__need({ let t2250 = Py::Bool(({ let mut out = vec![]; for t2244 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2244;
out.push({ let t2245 = v_x.clone(); Py::Bool({ let t2246 = Py::Int(0i64); if py::gt(&t2245, &t2246)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if !t2250.truthy() { t2250 } else { Py::Bool(({ let mut out = vec![]; for t2247 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2247;
out.push({ let t2248 = v_x.clone(); Py::Bool({ let t2249 = Py::Int(0i64); if py::lt(&t2248, &t2249)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }, Py::str("the cash flows need at least one inflow and one outflow"))?;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_fv_pos = py::sum(&{ let mut out = vec![]; for t2251 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t2252 = (t2251).items()?;
if t2252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2252[0].clone();
v_x = t2252[1].clone();
if ({ let t2253 = v_x.clone(); Py::Bool({ let t2254 = Py::Int(0i64); if py::gt(&t2253, &t2254)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_x.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_rr.clone())?, &py::sub(&v_n.clone(), &v_t.clone())?)?)?); }} Py::list(out) }, None)?;
    v_pv_neg = py::neg(&py::sum(&{ let mut out = vec![]; for t2255 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t2256 = (t2255).items()?;
if t2256.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2256[0].clone();
v_x = t2256[1].clone();
if ({ let t2257 = v_x.clone(); Py::Bool({ let t2258 = Py::Int(0i64); if py::lt(&t2257, &t2258)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_x.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_f.clone())?, &v_t.clone())?)?); }} Py::list(out) }, None)?)?;
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
    let t2284 = (Py::tuple(vec![f__prob(Py::str("reserve_ratio"), v_reserve_ratio.clone(), Py::Float(0.0), Py::Float(1.0))?, f__nonneg(Py::str("currency_ratio"), v_currency_ratio.clone())?])).items()?;
    if t2284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rr = t2284[0].clone();
    v_c = t2284[1].clone();
    let _ = f__need({ let t2285 = v_rr.clone(); Py::Bool({ let t2286 = Py::Int(0i64); if py::gt(&t2285, &t2286)? { true } else { false } }) }, Py::str("the reserve ratio must be positive"))?;
    v_m = py::truediv(&py::add(&Py::Int(1i64), &v_c.clone())?, &py::add(&v_rr.clone(), &v_c.clone())?)?;
    v_out = Py::dict(vec![(Py::str("simple_multiplier"), py::truediv(&Py::Int(1i64), &v_rr.clone())?), (Py::str("money_multiplier"), v_m.clone())]);
    if ({ let t2287 = v_monetary_base.clone(); Py::Bool({ let t2288 = Py::None; if !py::is(&t2287, &t2288) { true } else { false } }) }).truthy() {
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
    let t2313 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t2313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2313[0].clone();
    v_B = t2313[1].clone();
    let t2316 = (Py::tuple(vec![f__nonneg(Py::str("marginal_cost"), v_marginal_cost.clone())?, f__nonneg(Py::str("fixed_cost"), v_fixed_cost.clone())?, f__nonneg(Py::str("cost_quadratic"), v_cost_quadratic.clone())?])).items()?;
    if t2316.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2316[0].clone();
    v_F = t2316[1].clone();
    v_e = t2316[2].clone();
    let _ = f__need({ let t2317 = v_A.clone(); Py::Bool({ let t2318 = v_c.clone(); if py::gt(&t2317, &t2318)? { true } else { false } }) }, Py::str("the demand intercept must exceed marginal cost (otherwise nothing is produced)"))?;
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
    let t2343 = (Py::tuple(vec![v_X.clone(), f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t2343[0].clone();
    v_Y = t2343[1].clone();
    let _ = f__need({ let t2346 = Py::Bool(matches!(v_rows.clone(), Py::List(_) | Py::Tuple(_))); if !t2346.truthy() { t2346 } else { { let t2344 = py::len(&v_rows.clone())?; Py::Bool({ let t2345 = py::len(&v_Y.clone())?; if py::eq(&t2344, &t2345) { true } else { false } }) } } }, Py::str("X needs one row per observation of y"))?;
    v_k = py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?;
    v_n = py::len(&v_Y.clone())?;
    let _ = f__need({ let t2347 = v_n.clone(); Py::Bool({ let t2348 = py::add(&v_k.clone(), &Py::Int(1i64))?; if py::gt(&t2347, &t2348)? { true } else { false } }) }, Py::str("more observations than coefficients are needed (n > k + 1)"))?;
    v_cols = py::add(&Py::list(vec![py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?]), &{ let mut out = vec![]; for t2349 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2349;
out.push({ let mut out = vec![]; for t2350 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t2350;
out.push(f__fin(Py::str("X"), py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) })?;
    let t2352 = (Py::tuple(vec![Py::list(vec![]), { let mut out = vec![]; for t2351 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t2351;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) }])).items()?;
    if t2352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t2352[0].clone();
    v_R = t2352[1].clone();
    for t2353 in py::iter(&(py::enumerate(&v_cols.clone(), 0)?))? {
        let t2365 = (t2353).items()?;
        if t2365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2365[0].clone();
        v_v = t2365[1].clone();
        v_v = Py::list((v_v.clone()).items()?);
        for t2354 in py::iter(&(py::enumerate(&v_Q.clone(), 0)?))? {
            let t2359 = (t2354).items()?;
            if t2359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_i = t2359[0].clone();
            v_q = t2359[1].clone();
            py::setitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone(), py::fsum(&{ let mut out = vec![]; for t2355 in py::iter(&(py::zip(&[v_q.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2356 = (t2355).items()?;
if t2356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2356[0].clone();
v_b = t2356[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?;
            v_v = { let mut out = vec![]; for t2357 in py::iter(&(py::zip(&[v_v.clone(), v_q.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2358 = (t2357).items()?;
if t2358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2358[0].clone();
v_b = t2358[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &v_b.clone())?)?); } Py::list(out) };
        }
        py::setitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2360 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2360;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?)?;
        let _ = f__need({ let t2361 = py::getitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone())?; Py::Bool({ let t2362 = py::mul(&Py::Float(1e-10), &py::minmax(&[Py::Float(1.0), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2363 in py::iter(&(py::getitem(&v_cols.clone(), &v_j.clone())?))? { let mut v_a: Py; 
v_a = t2363;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) })?)?], true)?)?; if py::gt(&t2361, &t2362)? { true } else { false } }) }, Py::str("the columns of X are (nearly) collinear"))?;
        let _ = py::method(&v_Q.clone(), "append", &[{ let mut out = vec![]; for t2364 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2364;
out.push(py::truediv(&v_a.clone(), &py::getitem(&py::getitem(&v_R.clone(), &v_j.clone())?, &v_j.clone())?)?); } Py::list(out) }])?;
    }
    v_qty = { let mut out = vec![]; for t2366 in py::iter(&(v_Q.clone()))? { let mut v_q: Py; 
v_q = t2366;
out.push(py::fsum(&{ let mut out = vec![]; for t2367 in py::iter(&(py::zip(&[v_q.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2368 = (t2367).items()?;
if t2368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2368[0].clone();
v_b = t2368[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_beta = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
    for t2369 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t2369;
        py::setitem(&v_beta.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_qty.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t2370 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2370;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_beta.clone(), &v_j.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    v_my = f__mean(v_Y.clone())?;
    v_sst = py::fsum(&{ let mut out = vec![]; for t2371 in py::iter(&(v_Y.clone()))? { let mut v_v: Py; 
v_v = t2371;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    let _ = f__need({ let t2372 = v_sst.clone(); Py::Bool({ let t2373 = Py::Int(0i64); if py::gt(&t2372, &t2373)? { true } else { false } }) }, Py::str("y must not be constant"))?;
    v_ssr = py::fsum(&{ let mut out = vec![]; for t2374 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t2374;
out.push(py::pow(&py::sub(&py::getitem(&v_Y.clone(), &v_t.clone())?, &py::fsum(&{ let mut out = vec![]; for t2375 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2375;
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
    let t2412 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t2412.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2412[0].clone();
    v_v = t2412[1].clone();
    v_B = t2412[2].clone();
    let t2416 = (Py::tuple(vec![f__A(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?, f__a_due(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?])).items()?;
    if t2416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2416[0].clone();
    v_a = t2416[1].clone();
    return Ok(Py::dict(vec![(Py::str("premium"), py::truediv(&py::mul(&v_B.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("insurance_apv"), py::mul(&v_B.clone(), &v_A.clone())?), (Py::str("annuity_due"), v_a.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nominal_rate(mut v_effective_rate: Py, mut v_periods_per_year: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t2452 = (Py::tuple(vec![f__rate(Py::str("effective_rate"), v_effective_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods_per_year"), v_periods_per_year.clone(), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t2452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t2452[0].clone();
    v_m = t2452[1].clone();
    v_p = py::sub(&py::pow(&py::add(&Py::Int(1i64), &v_e.clone())?, &py::truediv(&Py::Int(1i64), &v_m.clone())?)?, &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("nominal_rate"), py::mul(&v_m.clone(), &v_p.clone())?), (Py::str("periodic_rate"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_npv(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2480 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t2480.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2480[0].clone();
    v_c = t2480[1].clone();
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
    let t2514 = (Py::tuple(vec![f__pos(Py::str("present_value"), v_present_value.clone())?, f__pos(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__timing(v_timing.clone())?])).items()?;
    if t2514.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t2514[0].clone();
    v_p = t2514[1].clone();
    v_r = t2514[2].clone();
    v_due = t2514[3].clone();
    v_eff = (if (v_due.clone()).truthy() { py::mul(&v_p.clone(), &py::add(&Py::Int(1i64), &v_r.clone())?)? } else { v_p.clone() });
    let _ = f__need({ let t2515 = v_eff.clone(); Py::Bool({ let t2516 = py::mul(&v_pv.clone(), &v_r.clone())?; if py::gt(&t2515, &t2516)? { true } else { false } }) }, Py::str("the payment does not even cover the interest: the loan is never repaid"))?;
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
    let t2541 = (Py::tuple(vec![f__vec(Py::str("x"), v_x.clone(), Py::Int(3i64))?, f__vec(Py::str("y"), v_y.clone(), Py::Int(3i64))?])).items()?;
    if t2541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2541[0].clone();
    v_Y = t2541[1].clone();
    let _ = f__need({ let t2542 = py::len(&v_X.clone())?; Py::Bool({ let t2543 = py::len(&v_Y.clone())?; if py::eq(&t2542, &t2543) { true } else { false } }) }, Py::str("x and y need the same number of observations"))?;
    let t2547 = (Py::tuple(vec![f__cov(v_X.clone(), v_X.clone(), Py::Int(1i64))?, f__cov(v_Y.clone(), v_Y.clone(), Py::Int(1i64))?, f__cov(v_X.clone(), v_Y.clone(), Py::Int(1i64))?])).items()?;
    if t2547.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t2547[0].clone();
    v_syy = t2547[1].clone();
    v_sxy = t2547[2].clone();
    let _ = f__need({ let t2548 = v_sxx.clone(); Py::Bool({ let t2549 = Py::Int(0i64); if py::gt(&t2548, &t2549)? { true } else { false } }) }, Py::str("x must not be constant"))?;
    let _ = f__need({ let t2550 = v_syy.clone(); Py::Bool({ let t2551 = Py::Int(0i64); if py::gt(&t2550, &t2551)? { true } else { false } }) }, Py::str("y must not be constant"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&f__mean(v_Y.clone())?, &py::mul(&v_b.clone(), &f__mean(v_X.clone())?)?)?;
    v_ssr = py::fsum(&{ let mut out = vec![]; for t2552 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_xv: Py; let mut v_yv: Py; 
let t2553 = (t2552).items()?;
if t2553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_xv = t2553[0].clone();
v_yv = t2553[1].clone();
out.push(py::pow(&py::sub(&py::sub(&v_yv.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_xv.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_n = py::len(&v_X.clone())?;
    return Ok(Py::dict(vec![(Py::str("slope"), v_b.clone()), (Py::str("intercept"), v_a.clone()), (Py::str("r_squared"), py::sub(&Py::Int(1i64), &py::truediv(&v_ssr.clone(), &py::mul(&v_syy.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?), (Py::str("slope_std_error"), py::math1("sqrt", &py::truediv(&py::truediv(&v_ssr.clone(), &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::mul(&v_sxx.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?), (Py::str("n"), v_n.clone())]));
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
    let t2579 = (Py::tuple(vec![f__rate(Py::str("mean"), v_mean.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__pos(Py::str("std_dev"), v_std_dev.clone())?])).items()?;
    if t2579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t2579[0].clone();
    v_s = t2579[1].clone();
    let _ = f__need({ let t2580 = v_s.clone(); Py::Bool({ let t2581 = Py::Int(5i64); if py::lt(&t2580, &t2581)? { true } else { false } }) }, Py::str("std_dev is a decimal (20% = 0.2)"))?;
    let t2585 = (Py::tuple(vec![f__confidence(v_confidence.clone())?, f__pos(Py::str("horizon"), v_horizon.clone())?, f__pos(Py::str("portfolio_value"), v_portfolio_value.clone())?])).items()?;
    if t2585.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2585[0].clone();
    v_h = t2585[1].clone();
    v_V = t2585[2].clone();
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_a.clone()])?;
    let t2586 = (Py::tuple(vec![py::mul(&v_s.clone(), &py::math1("sqrt", &v_h.clone())?)?, py::mul(&v_mu.clone(), &v_h.clone())?])).items()?;
    if t2586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sh = t2586[0].clone();
    v_mh = t2586[1].clone();
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
    let t2652 = (Py::tuple(vec![v_demand_intercept.clone(), v_demand_slope.clone(), v_supply_intercept.clone(), v_supply_slope.clone()])).items()?;
    if t2652.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2652[0].clone();
    v_b = t2652[1].clone();
    v_c = t2652[2].clone();
    v_d = t2652[3].clone();
    v_Pb = py::truediv(&py::add(&py::sub(&v_a.clone(), &v_c.clone())?, &py::mul(&v_d.clone(), &v_t.clone())?)?, &py::add(&v_b.clone(), &v_d.clone())?)?;
    let t2653 = (Py::tuple(vec![py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_Pb.clone())?)?, py::sub(&v_Pb.clone(), &v_t.clone())?])).items()?;
    if t2653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Qt = t2653[0].clone();
    v_Ps = t2653[1].clone();
    let _ = f__need({ let t2658 = { let t2654 = v_Qt.clone(); Py::Bool({ let t2655 = Py::Int(0i64); if py::gt(&t2654, &t2655)? { true } else { false } }) }; if !t2658.truthy() { t2658 } else { { let t2656 = v_Ps.clone(); Py::Bool({ let t2657 = Py::Int(0i64); if py::gt(&t2656, &t2657)? { true } else { false } }) } } }, Py::str("the tax is so large that the market closes (no positive quantity or seller price)"))?;
    return Ok(Py::dict(vec![(Py::str("quantity"), v_Qt.clone()), (Py::str("buyer_price"), v_Pb.clone()), (Py::str("seller_price"), v_Ps.clone()), (Py::str("tax_revenue"), py::mul(&v_t.clone(), &v_Qt.clone())?), (Py::str("deadweight_loss"), py::truediv(&py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_e.clone(), &Py::str("quantity"))?, &v_Qt.clone())?)?, &Py::Int(2i64))?), (Py::str("buyer_share"), py::truediv(&py::sub(&v_Pb.clone(), &py::getitem(&v_e.clone(), &Py::str("price"))?)?, &v_t.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_perpetuity(mut v_payment: Py, mut v_rate: Py, mut v_timing: Py) -> R {
    let mut v_due: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2686 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), Py::Float(0.0), Py::Float(1.0))?, f__timing(v_timing.clone())?])).items()?;
    if t2686.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2686[0].clone();
    v_r = t2686[1].clone();
    v_due = t2686[2].clone();
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
    let t2709 = (Py::tuple(vec![f__pos(Py::str("demand_intercept"), v_demand_intercept.clone())?, f__pos(Py::str("demand_slope"), v_demand_slope.clone())?, f__pos(Py::str("price"), v_price.clone())?])).items()?;
    if t2709.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2709[0].clone();
    v_b = t2709[1].clone();
    v_P = t2709[2].clone();
    v_Q = py::sub(&v_a.clone(), &py::mul(&v_b.clone(), &v_P.clone())?)?;
    let _ = f__need({ let t2710 = v_Q.clone(); Py::Bool({ let t2711 = Py::Int(0i64); if py::gt(&t2710, &t2711)? { true } else { false } }) }, Py::str("the price is at or above the choke price a/b: nothing is demanded"))?;
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
    let _ = f__need({ let t2740 = py::len(&v_w.clone())?; Py::Bool({ let t2741 = py::len(&v_mu.clone())?; if py::eq(&t2740, &t2741) { true } else { false } }) }, Py::str("weights and expected_returns need one entry per asset"))?;
    v_S = f__square(Py::str("covariance"), v_covariance.clone(), py::len(&v_w.clone())?)?;
    let _ = f__need({ let t2767 = py::abs(&py::sub(&py::fsum(&v_w.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t2768 = Py::Float(1e-09); if py::le(&t2767, &t2768)? { true } else { false } }) }, Py::str("the weights must add up to 1"))?;
    v_var = f__quad(v_w.clone(), v_S.clone())?;
    let _ = f__need({ let t2771 = v_var.clone(); Py::Bool({ let t2772 = py::neg(&Py::Float(1e-15))?; if py::ge(&t2771, &t2772)? { true } else { false } }) }, Py::str("the covariance matrix is not positive semidefinite (negative portfolio variance)"))?;
    v_var = py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?;
    return Ok(Py::dict(vec![(Py::str("expected_return"), py::fsum(&{ let mut out = vec![]; for t2773 in py::iter(&(py::zip(&[v_w.clone(), v_mu.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2774 = (t2773).items()?;
if t2774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2774[0].clone();
v_b = t2774[1].clone();
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
    let t2829 = (Py::tuple(vec![f__confidence(v_confidence.clone())?, f__pos(Py::str("portfolio_value"), v_portfolio_value.clone())?, f__pos(Py::str("horizon"), v_horizon.clone())?])).items()?;
    if t2829.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2829[0].clone();
    v_V = t2829[1].clone();
    v_h = t2829[2].clone();
    v_var = f__quad(v_w.clone(), v_S.clone())?;
    let _ = f__need({ let t2832 = v_var.clone(); Py::Bool({ let t2833 = py::neg(&Py::Float(1e-15))?; if py::ge(&t2832, &t2833)? { true } else { false } }) }, Py::str("the covariance matrix is not positive semidefinite"))?;
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_a.clone()])?;
    v_sd = py::math1("sqrt", &py::minmax(&[v_var.clone(), Py::Float(0.0)], true)?)?;
    v_undiv = py::mul(&py::mul(&py::mul(&v_z.clone(), &py::math1("sqrt", &v_h.clone())?)?, &v_V.clone())?, &py::fsum(&{ let mut out = vec![]; for t2834 in py::iter(&(py::enumerate(&v_w.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t2835 = (t2834).items()?;
if t2835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2835[0].clone();
v_x = t2835[1].clone();
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
    let t2861 = (Py::tuple(vec![f__fin(Py::str("future_value"), v_future_value.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("periods"), v_periods.clone())?])).items()?;
    if t2861.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2861[0].clone();
    v_r = t2861[1].clone();
    v_n = t2861[2].clone();
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
    let t2901 = (Py::tuple(vec![f__fin(Py::str("payment"), v_payment.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t2901.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2901[0].clone();
    v_r = t2901[1].clone();
    v_n = t2901[2].clone();
    v_due = t2901[3].clone();
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
    let t2947 = (f__baskets(v_base_prices.clone(), v_base_quantities.clone(), v_current_prices.clone(), v_current_quantities.clone())?).items()?;
    if t2947.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t2947[0].clone();
    v_q0 = t2947[1].clone();
    v_p1 = t2947[2].clone();
    v_q1 = t2947[3].clone();
    let t2950 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(100i64), &f__dot(v_p1.clone(), v_q0.clone())?)?, &f__dot(v_p0.clone(), v_q0.clone())?)?, py::truediv(&py::mul(&Py::Int(100i64), &f__dot(v_p1.clone(), v_q1.clone())?)?, &f__dot(v_p0.clone(), v_q1.clone())?)?])).items()?;
    if t2950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_L = t2950[0].clone();
    v_P = t2950[1].clone();
    return Ok(Py::dict(vec![(Py::str("laspeyres"), v_L.clone()), (Py::str("paasche"), v_P.clone()), (Py::str("fisher"), py::math1("sqrt", &py::mul(&v_L.clone(), &v_P.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_profitability_index(mut v_rate: Py, mut v_cash_flows: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2978 = (Py::tuple(vec![f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__vec(Py::str("cash_flows"), v_cash_flows.clone(), Py::Int(2i64))?])).items()?;
    if t2978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2978[0].clone();
    v_c = t2978[1].clone();
    let _ = f__need({ let t2979 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2980 = Py::Int(0i64); if py::lt(&t2979, &t2980)? { true } else { false } }) }, Py::str("the first cash flow (the investment at time 0) must be negative"))?;
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
    let t3012 = (f__opt(v_spot.clone(), v_strike.clone(), v_time.clone(), v_rate.clone(), Py::Float(0.2), v_dividend_yield.clone())?).items()?;
    if t3012.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3012[0].clone();
    v_K = t3012[1].clone();
    v_T = t3012[2].clone();
    v_r = t3012[3].clone();
    v__s = t3012[4].clone();
    v_q = t3012[5].clone();
    v_C = f__pos(Py::str("call_price"), v_call_price.clone())?;
    let t3013 = (Py::tuple(vec![py::mul(&v_S.clone(), &py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_T.clone())?)?)?, py::mul(&v_K.clone(), &py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_T.clone())?)?)?])).items()?;
    if t3013.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Sq = t3013[0].clone();
    v_Kr = t3013[1].clone();
    let _ = f__need({ let t3014 = py::minmax(&[Py::Float(0.0), py::sub(&v_Sq.clone(), &v_Kr.clone())?], true)?; Py::Bool({ let t3015 = v_C.clone(); if py::lt(&t3014, &t3015)? { { let t3016 = v_Sq.clone(); if py::lt(&t3015, &t3016)? { true } else { false } } } else { false } }) }, Py::str("the call price violates the no-arbitrage bounds max(0, S e^-qT - K e^-rT) < C < S e^-qT"))?;
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
    let t3041 = (Py::tuple(vec![f__vec(Py::str("probabilities"), v_probabilities.clone(), Py::Int(1i64))?, f__vec(Py::str("returns"), v_returns.clone(), Py::Int(1i64))?])).items()?;
    if t3041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3041[0].clone();
    v_r = t3041[1].clone();
    let _ = f__need({ let t3042 = py::len(&v_p.clone())?; Py::Bool({ let t3043 = py::len(&v_r.clone())?; if py::eq(&t3042, &t3043) { true } else { false } }) }, Py::str("one probability per scenario return"))?;
    let _ = f__need({ let t3049 = Py::Bool(({ let mut out = vec![]; for t3044 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t3044;
out.push({ let t3045 = v_x.clone(); Py::Bool({ let t3046 = Py::Int(0i64); if py::ge(&t3045, &t3046)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t3049.truthy() { t3049 } else { { let t3047 = py::abs(&py::sub(&py::fsum(&v_p.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t3048 = Py::Float(1e-09); if py::le(&t3047, &t3048)? { true } else { false } }) } } }, Py::str("the probabilities must be non-negative and add up to 1"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3050 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3050;
out.push({ let t3051 = v_x.clone(); Py::Bool({ let t3052 = py::neg(&Py::Int(1i64))?; if py::gt(&t3051, &t3052)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a return cannot be below -100%"))?;
    v_e = py::fsum(&{ let mut out = vec![]; for t3053 in py::iter(&(py::zip(&[v_p.clone(), v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3054 = (t3053).items()?;
if t3054.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3054[0].clone();
v_b = t3054[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    v_var = py::fsum(&{ let mut out = vec![]; for t3055 in py::iter(&(py::zip(&[v_p.clone(), v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3056 = (t3055).items()?;
if t3056.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3056[0].clone();
v_b = t3056[1].clone();
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
    let _ = f__need({ let t3103 = v_s.clone(); Py::Bool({ let t3104 = Py::Int(0i64); if py::gt(&t3103, &t3104)? { true } else { false } }) }, Py::str("the returns must not all be equal (zero standard deviation)"))?;
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
    let t3132 = (Py::tuple(vec![f__pos(Py::str("principal"), v_principal.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1e-300))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t3132.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t3132[0].clone();
    v_r = t3132[1].clone();
    v_t = t3132[2].clone();
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
    let t3174 = (Py::tuple(vec![f__pos(Py::str("future_value"), v_future_value.clone())?, f__rate(Py::str("rate"), v_rate.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__int(Py::str("periods"), v_periods.clone(), Py::Int(1i64), Py::Int(100000i64))?, f__timing(v_timing.clone())?])).items()?;
    if t3174.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3174[0].clone();
    v_r = t3174[1].clone();
    v_n = t3174[2].clone();
    v_due = t3174[3].clone();
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
    let t3197 = (Py::tuple(vec![f__fin(Py::str("saving_rate"), v_saving_rate.clone())?, f__fin(Py::str("alpha"), v_alpha.clone())?])).items()?;
    if t3197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3197[0].clone();
    v_a = t3197[1].clone();
    let _ = f__need({ let t3204 = { let t3198 = Py::Int(0i64); Py::Bool({ let t3199 = v_s.clone(); if py::lt(&t3198, &t3199)? { { let t3200 = Py::Int(1i64); if py::lt(&t3199, &t3200)? { true } else { false } } } else { false } }) }; if !t3204.truthy() { t3204 } else { { let t3201 = Py::Int(0i64); Py::Bool({ let t3202 = v_a.clone(); if py::lt(&t3201, &t3202)? { { let t3203 = Py::Int(1i64); if py::lt(&t3202, &t3203)? { true } else { false } } } else { false } }) } } }, Py::str("saving_rate and alpha are decimals between 0 and 1"))?;
    let t3208 = (Py::tuple(vec![f__rate(Py::str("population_growth"), v_population_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("depreciation"), v_depreciation.clone(), Py::Float(0.0), Py::Float(1.0))?, f__rate(Py::str("technology_growth"), v_technology_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3208.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3208[0].clone();
    v_d = t3208[1].clone();
    v_g = t3208[2].clone();
    v_A = f__pos(Py::str("productivity"), v_productivity.clone())?;
    v_br = py::add(&py::add(&v_n.clone(), &v_g.clone())?, &v_d.clone())?;
    let _ = f__need({ let t3211 = v_br.clone(); Py::Bool({ let t3212 = Py::Int(0i64); if py::gt(&t3211, &t3212)? { true } else { false } }) }, Py::str("population growth + technology growth + depreciation must be positive"))?;
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
    v_dd = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3244 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3244;
out.push(py::pow(&py::minmax(&[Py::Float(0.0), py::sub(&v_x.clone(), &v_T.clone())?], false)?, &Py::Int(2i64))?); } Py::list(out) })?, &py::len(&v_r.clone())?)?)?;
    let _ = f__need({ let t3245 = v_dd.clone(); Py::Bool({ let t3246 = Py::Int(0i64); if py::gt(&t3245, &t3246)? { true } else { false } }) }, Py::str("no return is below the target: the downside deviation is zero"))?;
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
    let _ = f__need({ let t3267 = Py::Int(0i64); Py::Bool({ let t3268 = v_b.clone(); if py::lt(&t3267, &t3268)? { { let t3269 = Py::Int(1i64); if py::lt(&t3268, &t3269)? { true } else { false } } } else { false } }) }, Py::str("the marginal propensity to consume is between 0 and 1"))?;
    let t3273 = (Py::tuple(vec![f__prob(Py::str("tax_rate"), v_tax_rate.clone(), Py::Int(0i64), Py::Float(0.99))?, f__prob(Py::str("import_propensity"), v_import_propensity.clone(), Py::Int(0i64), Py::Float(0.99))?])).items()?;
    if t3273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t3273[0].clone();
    v_m = t3273[1].clone();
    v_rho = py::sub(&py::mul(&v_b.clone(), &py::sub(&Py::Int(1i64), &v_t.clone())?)?, &v_m.clone())?;
    let _ = f__need({ let t3274 = Py::Int(0i64); Py::Bool({ let t3275 = v_rho.clone(); if py::le(&t3274, &t3275)? { { let t3276 = Py::Int(1i64); if py::lt(&t3275, &t3276)? { true } else { false } } } else { false } }) }, Py::str("the induced-spending ratio mpc(1 - t) - m must lie in [0, 1)"))?;
    v_k = py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_out = Py::dict(vec![(Py::str("spending_multiplier"), v_k.clone()), (Py::str("tax_multiplier"), py::mul(&py::neg(&v_b.clone())?, &v_k.clone())?)]);
    if ({ let t3277 = v_spending_change.clone(); Py::Bool({ let t3278 = Py::None; if !py::is(&t3277, &t3278) { true } else { false } }) }).truthy() {
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
    let t3301 = (Py::tuple(vec![f__pos(Py::str("price_intercept"), v_price_intercept.clone())?, f__pos(Py::str("price_slope"), v_price_slope.clone())?])).items()?;
    if t3301.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3301[0].clone();
    v_B = t3301[1].clone();
    let t3304 = (Py::tuple(vec![f__nonneg(Py::str("cost_leader"), v_cost_leader.clone())?, f__nonneg(Py::str("cost_follower"), v_cost_follower.clone())?])).items()?;
    if t3304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cl = t3304[0].clone();
    v_cf = t3304[1].clone();
    v_ql = py::truediv(&py::add(&py::sub(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_cl.clone())?)?, &v_cf.clone())?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    v_qf = py::truediv(&py::sub(&py::sub(&v_A.clone(), &v_cf.clone())?, &py::mul(&v_B.clone(), &v_ql.clone())?)?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    let _ = f__need({ let t3309 = { let t3305 = v_ql.clone(); Py::Bool({ let t3306 = Py::Int(0i64); if py::gt(&t3305, &t3306)? { true } else { false } }) }; if !t3309.truthy() { t3309 } else { { let t3307 = v_qf.clone(); Py::Bool({ let t3308 = Py::Int(0i64); if py::gt(&t3307, &t3308)? { true } else { false } }) } } }, Py::str("one firm would not produce: no interior Stackelberg equilibrium"))?;
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
    v_u = (if ({ let t3353 = v_t.clone(); Py::Bool({ let t3354 = py::len(&v_q.clone())?; if py::lt(&t3353, &t3354)? { true } else { false } }) }).truthy() { f__int(Py::str("deferred_years"), v_deferred_years.clone(), Py::Int(1i64), py::sub(&py::len(&v_q.clone())?, &v_t.clone())?)? } else { f__int(Py::str("deferred_years"), v_deferred_years.clone(), Py::Int(1i64), Py::Int(0i64))? });
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
    let _ = f__need({ let t3421 = v_L.clone(); Py::Bool({ let t3422 = Py::None; if !py::is(&t3421, &t3422) { true } else { false } }) }, Py::str("the covariance matrix must be positive definite"))?;
    v_z = f__chol_solve(v_L.clone(), { let mut out = vec![]; for t3427 in py::iter(&(v_mu.clone()))? { let mut v_m: Py; 
v_m = t3427;
out.push(py::sub(&v_m.clone(), &v_rf.clone())?); } Py::list(out) })?;
    v_tot = py::fsum(&v_z.clone())?;
    let _ = f__need({ let t3428 = v_tot.clone(); Py::Bool({ let t3429 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3430 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t3430;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3428, &t3429)? { true } else { false } }) }, Py::str("no portfolio of positive total weight has the highest Sharpe ratio (the excess returns do not favour a long position)"))?;
    v_w = { let mut out = vec![]; for t3431 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t3431;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    let t3436 = (Py::tuple(vec![py::fsum(&{ let mut out = vec![]; for t3432 in py::iter(&(py::zip(&[v_w.clone(), v_mu.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3433 = (t3432).items()?;
if t3433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3433[0].clone();
v_b = t3433[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, py::math1("sqrt", &py::minmax(&[f__quad(v_w.clone(), v_S.clone())?, Py::Float(0.0)], true)?)?])).items()?;
    if t3436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_er = t3436[0].clone();
    v_sd = t3436[1].clone();
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
    let t3473 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("payment"), v_payment.clone())?])).items()?;
    if t3473.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3473[0].clone();
    v_v = t3473[1].clone();
    v_P = t3473[2].clone();
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
    let t3524 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(false))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t3524.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3524[0].clone();
    v_v = t3524[1].clone();
    v_B = t3524[2].clone();
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
    let t3570 = (f__pair(v_portfolio_returns.clone(), v_market_returns.clone(), Py::str("portfolio_returns"), Py::str("market_returns"), Py::Int(3i64))?).items()?;
    if t3570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3570[0].clone();
    v_m = t3570[1].clone();
    v_rf = f__rate(Py::str("risk_free"), v_risk_free.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?;
    v_sxx = f__cov(v_m.clone(), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3577 = v_sxx.clone(); Py::Bool({ let t3578 = Py::Int(0i64); if py::gt(&t3577, &t3578)? { true } else { false } }) }, Py::str("the market returns must not all be equal"))?;
    v_b = py::truediv(&f__cov(v_p.clone(), v_m.clone(), Py::Int(1i64))?, &v_sxx.clone())?;
    let _ = f__need({ let t3579 = py::abs(&v_b.clone())?; Py::Bool({ let t3580 = Py::Float(1e-09); if py::gt(&t3579, &t3580)? { true } else { false } }) }, Py::str("beta is zero: the Treynor ratio is undefined"))?;
    let t3581 = (Py::tuple(vec![f__mean(v_p.clone())?, f__mean(v_m.clone())?])).items()?;
    if t3581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mp = t3581[0].clone();
    v_mm = t3581[1].clone();
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
    let t3610 = (Py::tuple(vec![py::getitem(&f__returns(Py::str("return_1"), Py::list(vec![v_return_1.clone()]), Py::Int(1i64), Py::Float(1.0))?, &Py::Int(0i64))?, py::getitem(&f__returns(Py::str("return_2"), Py::list(vec![v_return_2.clone()]), Py::Int(1i64), Py::Float(1.0))?, &Py::Int(0i64))?])).items()?;
    if t3610.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t3610[0].clone();
    v_r2 = t3610[1].clone();
    let t3613 = (Py::tuple(vec![f__nonneg(Py::str("sd_1"), v_sd_1.clone())?, f__nonneg(Py::str("sd_2"), v_sd_2.clone())?])).items()?;
    if t3613.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t3613[0].clone();
    v_s2 = t3613[1].clone();
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
    let t3642 = (Py::tuple(vec![f__rate(Py::str("high_growth"), v_high_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("stable_growth"), v_stable_growth.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g1 = t3642[0].clone();
    v_g2 = t3642[1].clone();
    v_n = f__int(Py::str("high_growth_years"), v_high_growth_years.clone(), Py::Int(1i64), Py::Int(200i64))?;
    v_r = f__rate(Py::str("required_return"), v_required_return.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let _ = f__need({ let t3655 = v_r.clone(); Py::Bool({ let t3656 = v_g2.clone(); if py::gt(&t3655, &t3656)? { true } else { false } }) }, Py::str("the required return must exceed the stable growth rate"))?;
    v_pv1 = py::fsum(&{ let mut out = vec![]; for t3657 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3657;
out.push(py::truediv(&py::mul(&v_D0.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_g1.clone())?, &v_k.clone())?)?, &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_k.clone())?)?); } Py::list(out) })?;
    v_tv = py::truediv(&py::mul(&py::mul(&v_D0.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_g1.clone())?, &v_n.clone())?)?, &py::add(&Py::Int(1i64), &v_g2.clone())?)?, &py::sub(&v_r.clone(), &v_g2.clone())?)?;
    v_pvt = py::truediv(&v_tv.clone(), &py::pow(&py::add(&Py::Int(1i64), &v_r.clone())?, &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("price"), py::add(&v_pv1.clone(), &v_pvt.clone())?), (Py::str("terminal_value"), v_tv.clone()), (Py::str("pv_terminal"), v_pvt.clone()), (Py::str("pv_high_growth_dividends"), v_pv1.clone())]));
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
    let t3680 = (Py::tuple(vec![f__nonneg(Py::str("equity_value"), v_equity_value.clone())?, f__nonneg(Py::str("debt_value"), v_debt_value.clone())?])).items()?;
    if t3680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3680[0].clone();
    v_D = t3680[1].clone();
    let _ = f__need({ let t3681 = py::add(&v_E.clone(), &v_D.clone())?; Py::Bool({ let t3682 = Py::Int(0i64); if py::gt(&t3681, &t3682)? { true } else { false } }) }, Py::str("the firm needs a positive total value"))?;
    let t3686 = (Py::tuple(vec![f__rate(Py::str("cost_of_equity"), v_cost_of_equity.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__rate(Py::str("cost_of_debt"), v_cost_of_debt.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?])).items()?;
    if t3686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t3686[0].clone();
    v_rd = t3686[1].clone();
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
    let t3726 = (Py::tuple(vec![f__qtab(v_mortality_rates.clone(), Py::Bool(true))?, f__v(v_interest_rate.clone())?, f__pos(Py::str("benefit"), v_benefit.clone())?])).items()?;
    if t3726.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3726[0].clone();
    v_v = t3726[1].clone();
    v_B = t3726[2].clone();
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
    let t3756 = (Py::tuple(vec![f__pos(Py::str("face"), v_face.clone())?, f__rate(Py::str("ytm"), v_ytm.clone(), py::neg(&Py::Float(1.0))?, Py::Float(1.0))?, f__nonneg(Py::str("years"), v_years.clone())?])).items()?;
    if t3756.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3756[0].clone();
    v_y = t3756[1].clone();
    v_t = t3756[2].clone();
    let _ = f__need({ let t3757 = v_compounding.clone(); Py::Bool({ let t3758 = c__ZC()?; if py::contains(&t3758, &t3757)? { true } else { false } }) }, Py::str("compounding is annual, semiannual, quarterly, monthly or continuous"))?;
    v_m = py::getitem(&c__ZC()?, &v_compounding.clone())?;
    v_d = (if ({ let t3759 = v_m.clone(); Py::Bool({ let t3760 = Py::Int(0i64); if py::eq(&t3759, &t3760) { true } else { false } }) }).truthy() { py::math1("exp", &py::mul(&py::neg(&v_y.clone())?, &v_t.clone())?)? } else { py::pow(&py::add(&Py::Int(1i64), &py::truediv(&v_y.clone(), &v_m.clone())?)?, &py::mul(&py::neg(&v_m.clone())?, &v_t.clone())?)? });
    return Ok(Py::dict(vec![(Py::str("price"), py::mul(&v_F.clone(), &v_d.clone())?), (Py::str("discount_factor"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1038(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1055 = v_inputs.clone(); f_endowment_insurance(py::getitem(&t1055, &Py::str("mortality_rates"))?, py::getitem(&t1055, &Py::str("interest_rate"))?, py::getitem(&t1055, &Py::str("term"))?, py::method(&t1055, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
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
fn s_1071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1088 = v_inputs.clone(); f_equivalent_annual_annuity(py::getitem(&t1088, &Py::str("rate"))?, py::getitem(&t1088, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1095(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1112 = v_inputs.clone(); f_fisher_real_rate(py::getitem(&t1112, &Py::str("nominal_rate"))?, py::getitem(&t1112, &Py::str("inflation_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1141 = v_inputs.clone(); f_forward_price(py::getitem(&t1141, &Py::str("spot"))?, py::getitem(&t1141, &Py::str("rate"))?, py::getitem(&t1141, &Py::str("time"))?, py::method(&t1141, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?, py::method(&t1141, "get", &[Py::str("storage_cost"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1165(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1182 = v_inputs.clone(); f_forward_price_dividends(py::getitem(&t1182, &Py::str("spot"))?, py::getitem(&t1182, &Py::str("rate"))?, py::getitem(&t1182, &Py::str("time"))?, py::getitem(&t1182, &Py::str("dividends"))?, py::getitem(&t1182, &Py::str("dividend_times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1213 = v_inputs.clone(); f_forward_rate(py::getitem(&t1213, &Py::str("spot_rate_1"))?, py::getitem(&t1213, &Py::str("maturity_1"))?, py::getitem(&t1213, &Py::str("spot_rate_2"))?, py::getitem(&t1213, &Py::str("maturity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1238(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1255 = v_inputs.clone(); f_future_value_annuity(py::getitem(&t1255, &Py::str("payment"))?, py::getitem(&t1255, &Py::str("rate"))?, py::getitem(&t1255, &Py::str("periods"))?, py::method(&t1255, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1285(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1302 = v_inputs.clone(); f_gdp_deflator(py::getitem(&t1302, &Py::str("base_prices"))?, py::getitem(&t1302, &Py::str("base_quantities"))?, py::getitem(&t1302, &Py::str("current_prices"))?, py::getitem(&t1302, &Py::str("current_quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1335 = v_inputs.clone(); f_gini_coefficient(py::getitem(&t1335, &Py::str("incomes"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1381(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1398 = v_inputs.clone(); f_global_min_variance(py::getitem(&t1398, &Py::str("covariance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1432 = v_inputs.clone(); f_gordon_growth(py::getitem(&t1432, &Py::str("required_return"))?, py::getitem(&t1432, &Py::str("growth"))?, py::method(&t1432, "get", &[Py::str("dividend"), Py::None])?, py::method(&t1432, "get", &[Py::str("next_dividend"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1471 = v_inputs.clone(); f_gordon_implied_return(py::getitem(&t1471, &Py::str("price"))?, py::getitem(&t1471, &Py::str("growth"))?, py::method(&t1471, "get", &[Py::str("dividend"), Py::None])?, py::method(&t1471, "get", &[Py::str("next_dividend"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1492(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1509 = v_inputs.clone(); f_growing_annuity(py::getitem(&t1509, &Py::str("first_payment"))?, py::getitem(&t1509, &Py::str("rate"))?, py::getitem(&t1509, &Py::str("growth"))?, py::getitem(&t1509, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1518(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1535 = v_inputs.clone(); f_growing_perpetuity(py::getitem(&t1535, &Py::str("first_payment"))?, py::getitem(&t1535, &Py::str("rate"))?, py::getitem(&t1535, &Py::str("growth"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1550(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1567 = v_inputs.clone(); f_herfindahl_index(py::getitem(&t1567, &Py::str("market_shares"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1600 = v_inputs.clone(); f_historical_var(py::getitem(&t1600, &Py::str("returns"))?, py::getitem(&t1600, &Py::str("confidence"))?, py::method(&t1600, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?)? })?);
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
fn s_1650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1667 = v_inputs.clone(); f_income_elasticity(py::getitem(&t1667, &Py::str("income_1"))?, py::getitem(&t1667, &Py::str("quantity_1"))?, py::getitem(&t1667, &Py::str("income_2"))?, py::getitem(&t1667, &Py::str("quantity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1696 = v_inputs.clone(); f_inflation_adjustment(py::getitem(&t1696, &Py::str("price_levels"))?, py::method(&t1696, "get", &[Py::str("amount"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1718(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1735 = v_inputs.clone(); f_information_ratio(py::getitem(&t1735, &Py::str("portfolio_returns"))?, py::getitem(&t1735, &Py::str("benchmark_returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1820(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1837 = v_inputs.clone(); f_life_annuity_due(py::getitem(&t1837, &Py::str("mortality_rates"))?, py::getitem(&t1837, &Py::str("interest_rate"))?, py::method(&t1837, "get", &[Py::str("payment"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1852(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1869 = v_inputs.clone(); f_life_expectancy(py::getitem(&t1869, &Py::str("mortality_rates"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1904(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1921 = v_inputs.clone(); f_loan_balance(py::getitem(&t1921, &Py::str("principal"))?, py::getitem(&t1921, &Py::str("annual_rate"))?, py::getitem(&t1921, &Py::str("years"))?, py::getitem(&t1921, &Py::str("payments_made"))?, py::method(&t1921, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
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
fn s_1956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1973 = v_inputs.clone(); f_loan_cumulative_interest(py::getitem(&t1973, &Py::str("principal"))?, py::getitem(&t1973, &Py::str("annual_rate"))?, py::getitem(&t1973, &Py::str("years"))?, py::getitem(&t1973, &Py::str("payments_made"))?, py::method(&t1973, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2003(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2020 = v_inputs.clone(); f_loan_payment(py::getitem(&t2020, &Py::str("principal"))?, py::getitem(&t2020, &Py::str("annual_rate"))?, py::getitem(&t2020, &Py::str("years"))?, py::method(&t2020, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2070 = v_inputs.clone(); f_loan_payment_split(py::getitem(&t2070, &Py::str("principal"))?, py::getitem(&t2070, &Py::str("annual_rate"))?, py::getitem(&t2070, &Py::str("years"))?, py::getitem(&t2070, &Py::str("payment_number"))?, py::method(&t2070, "get", &[Py::str("payments_per_year"), Py::Int(12i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2098(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2115 = v_inputs.clone(); f_log_log_elasticity(py::getitem(&t2115, &Py::str("prices"))?, py::getitem(&t2115, &Py::str("quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2147 = v_inputs.clone(); f_market_equilibrium(py::getitem(&t2147, &Py::str("demand_intercept"))?, py::getitem(&t2147, &Py::str("demand_slope"))?, py::getitem(&t2147, &Py::str("supply_intercept"))?, py::getitem(&t2147, &Py::str("supply_slope"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2159(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2176 = v_inputs.clone(); f_max_drawdown(py::getitem(&t2176, &Py::str("prices"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2188(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2205 = v_inputs.clone(); f_mean_returns(py::getitem(&t2205, &Py::str("returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2216(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2233 = v_inputs.clone(); f_min_variance_two_asset(py::getitem(&t2233, &Py::str("sd_1"))?, py::getitem(&t2233, &Py::str("sd_2"))?, py::getitem(&t2233, &Py::str("correlation"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2259(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2276 = v_inputs.clone(); f_mirr(py::getitem(&t2276, &Py::str("cash_flows"))?, py::getitem(&t2276, &Py::str("finance_rate"))?, py::getitem(&t2276, &Py::str("reinvest_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2291(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2308 = v_inputs.clone(); f_money_multiplier(py::getitem(&t2308, &Py::str("reserve_ratio"))?, py::method(&t2308, "get", &[Py::str("currency_ratio"), Py::Float(0.0)])?, py::method(&t2308, "get", &[Py::str("monetary_base"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2336 = v_inputs.clone(); f_monopoly(py::getitem(&t2336, &Py::str("price_intercept"))?, py::getitem(&t2336, &Py::str("price_slope"))?, py::getitem(&t2336, &Py::str("marginal_cost"))?, py::method(&t2336, "get", &[Py::str("fixed_cost"), Py::Float(0.0)])?, py::method(&t2336, "get", &[Py::str("cost_quadratic"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2376(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2393 = v_inputs.clone(); f_multiple_regression(py::getitem(&t2393, &Py::str("X"))?, py::getitem(&t2393, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2417(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2434 = v_inputs.clone(); f_net_premium_whole_life(py::getitem(&t2434, &Py::str("mortality_rates"))?, py::getitem(&t2434, &Py::str("interest_rate"))?, py::method(&t2434, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2453(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2470 = v_inputs.clone(); f_nominal_rate(py::getitem(&t2470, &Py::str("effective_rate"))?, py::getitem(&t2470, &Py::str("periods_per_year"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2502 = v_inputs.clone(); f__check_npv(py::getitem(&t2502, &Py::str("rate"))?, py::getitem(&t2502, &Py::str("cash_flows"))?)? })?);
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
fn s_2517(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2534 = v_inputs.clone(); f_number_of_periods(py::getitem(&t2534, &Py::str("present_value"))?, py::getitem(&t2534, &Py::str("payment"))?, py::getitem(&t2534, &Py::str("rate"))?, py::method(&t2534, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2554(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2571 = v_inputs.clone(); f_ols_simple(py::getitem(&t2571, &Py::str("x"))?, py::getitem(&t2571, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2587(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2604 = v_inputs.clone(); f_parametric_var(py::getitem(&t2604, &Py::str("mean"))?, py::getitem(&t2604, &Py::str("std_dev"))?, py::getitem(&t2604, &Py::str("confidence"))?, py::method(&t2604, "get", &[Py::str("horizon"), Py::Float(1.0)])?, py::method(&t2604, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2620(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2637 = v_inputs.clone(); f_payback_period(py::getitem(&t2637, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2659(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2676 = v_inputs.clone(); f_per_unit_tax(py::getitem(&t2676, &Py::str("demand_intercept"))?, py::getitem(&t2676, &Py::str("demand_slope"))?, py::getitem(&t2676, &Py::str("supply_intercept"))?, py::getitem(&t2676, &Py::str("supply_slope"))?, py::getitem(&t2676, &Py::str("tax"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2704 = v_inputs.clone(); f_perpetuity(py::getitem(&t2704, &Py::str("payment"))?, py::getitem(&t2704, &Py::str("rate"))?, py::method(&t2704, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2712(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2729 = v_inputs.clone(); f_point_elasticity(py::getitem(&t2729, &Py::str("demand_intercept"))?, py::getitem(&t2729, &Py::str("demand_slope"))?, py::getitem(&t2729, &Py::str("price"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2775(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2792 = v_inputs.clone(); f_portfolio_return_risk(py::getitem(&t2792, &Py::str("weights"))?, py::getitem(&t2792, &Py::str("expected_returns"))?, py::getitem(&t2792, &Py::str("covariance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2853 = v_inputs.clone(); f_portfolio_var(py::getitem(&t2853, &Py::str("weights"))?, py::getitem(&t2853, &Py::str("covariance"))?, py::getitem(&t2853, &Py::str("confidence"))?, py::method(&t2853, "get", &[Py::str("portfolio_value"), Py::Float(1.0)])?, py::method(&t2853, "get", &[Py::str("horizon"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2862(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2879 = v_inputs.clone(); f_present_value(py::getitem(&t2879, &Py::str("future_value"))?, py::getitem(&t2879, &Py::str("rate"))?, py::getitem(&t2879, &Py::str("periods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2904(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2921 = v_inputs.clone(); f_present_value_annuity(py::getitem(&t2921, &Py::str("payment"))?, py::getitem(&t2921, &Py::str("rate"))?, py::getitem(&t2921, &Py::str("periods"))?, py::method(&t2921, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t2968 = v_inputs.clone(); f_price_indices(py::getitem(&t2968, &Py::str("base_prices"))?, py::getitem(&t2968, &Py::str("base_quantities"))?, py::getitem(&t2968, &Py::str("current_prices"))?, py::getitem(&t2968, &Py::str("current_quantities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2983(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3000 = v_inputs.clone(); f_profitability_index(py::getitem(&t3000, &Py::str("rate"))?, py::getitem(&t3000, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3034 = v_inputs.clone(); f_put_call_parity(py::getitem(&t3034, &Py::str("call_price"))?, py::getitem(&t3034, &Py::str("spot"))?, py::getitem(&t3034, &Py::str("strike"))?, py::getitem(&t3034, &Py::str("time"))?, py::getitem(&t3034, &Py::str("rate"))?, py::method(&t3034, "get", &[Py::str("dividend_yield"), Py::Float(0.0)])?)? })?);
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
fn s_3057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3074 = v_inputs.clone(); f_scenario_return(py::getitem(&t3074, &Py::str("probabilities"))?, py::getitem(&t3074, &Py::str("returns"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3105(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3122 = v_inputs.clone(); f_sharpe_ratio(py::getitem(&t3122, &Py::str("returns"))?, py::method(&t3122, "get", &[Py::str("risk_free"), Py::Float(0.0)])?, py::method(&t3122, "get", &[Py::str("periods_per_year"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3150 = v_inputs.clone(); f_simple_interest(py::getitem(&t3150, &Py::str("principal"))?, py::getitem(&t3150, &Py::str("rate"))?, py::getitem(&t3150, &Py::str("years"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3194 = v_inputs.clone(); f_sinking_fund(py::getitem(&t3194, &Py::str("future_value"))?, py::getitem(&t3194, &Py::str("rate"))?, py::getitem(&t3194, &Py::str("periods"))?, py::method(&t3194, "get", &[Py::str("timing"), Py::str("ordinary")])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3213(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3230 = v_inputs.clone(); f_solow_steady_state(py::getitem(&t3230, &Py::str("saving_rate"))?, py::getitem(&t3230, &Py::str("population_growth"))?, py::getitem(&t3230, &Py::str("depreciation"))?, py::getitem(&t3230, &Py::str("alpha"))?, py::method(&t3230, "get", &[Py::str("technology_growth"), Py::Float(0.0)])?, py::method(&t3230, "get", &[Py::str("productivity"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3247(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3264 = v_inputs.clone(); f_sortino_ratio(py::getitem(&t3264, &Py::str("returns"))?, py::method(&t3264, "get", &[Py::str("target"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3279(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3296 = v_inputs.clone(); f_spending_multiplier(py::getitem(&t3296, &Py::str("mpc"))?, py::method(&t3296, "get", &[Py::str("tax_rate"), Py::Float(0.0)])?, py::method(&t3296, "get", &[Py::str("import_propensity"), Py::Float(0.0)])?, py::method(&t3296, "get", &[Py::str("spending_change"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3310(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3327 = v_inputs.clone(); f_stackelberg(py::getitem(&t3327, &Py::str("price_intercept"))?, py::getitem(&t3327, &Py::str("price_slope"))?, py::getitem(&t3327, &Py::str("cost_leader"))?, py::getitem(&t3327, &Py::str("cost_follower"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3373 = v_inputs.clone(); f_survival_probability(py::getitem(&t3373, &Py::str("mortality_rates"))?, py::getitem(&t3373, &Py::str("years"))?, py::method(&t3373, "get", &[Py::str("deferred_years"), Py::Int(1i64)])?)? })?);
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
fn s_3437(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3454 = v_inputs.clone(); f_tangency_portfolio(py::getitem(&t3454, &Py::str("expected_returns"))?, py::getitem(&t3454, &Py::str("covariance"))?, py::getitem(&t3454, &Py::str("risk_free"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3488(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3505 = v_inputs.clone(); f_temporary_annuity_due(py::getitem(&t3505, &Py::str("mortality_rates"))?, py::getitem(&t3505, &Py::str("interest_rate"))?, py::getitem(&t3505, &Py::str("term"))?, py::method(&t3505, "get", &[Py::str("payment"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3556 = v_inputs.clone(); f_term_insurance(py::getitem(&t3556, &Py::str("mortality_rates"))?, py::getitem(&t3556, &Py::str("interest_rate"))?, py::getitem(&t3556, &Py::str("term"))?, py::method(&t3556, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
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
    return Ok(f__residual(v_outputs.clone(), { let t3599 = v_inputs.clone(); f_treynor_jensen(py::getitem(&t3599, &Py::str("portfolio_returns"))?, py::getitem(&t3599, &Py::str("market_returns"))?, py::method(&t3599, "get", &[Py::str("risk_free"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3617(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3634 = v_inputs.clone(); f_two_asset_portfolio(py::getitem(&t3634, &Py::str("weight_1"))?, py::getitem(&t3634, &Py::str("return_1"))?, py::getitem(&t3634, &Py::str("return_2"))?, py::getitem(&t3634, &Py::str("sd_1"))?, py::getitem(&t3634, &Py::str("sd_2"))?, py::getitem(&t3634, &Py::str("correlation"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3658(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3675 = v_inputs.clone(); f_two_stage_ddm(py::getitem(&t3675, &Py::str("dividend"))?, py::getitem(&t3675, &Py::str("high_growth"))?, py::getitem(&t3675, &Py::str("high_growth_years"))?, py::getitem(&t3675, &Py::str("stable_growth"))?, py::getitem(&t3675, &Py::str("required_return"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3690(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3707 = v_inputs.clone(); f_wacc(py::getitem(&t3707, &Py::str("equity_value"))?, py::getitem(&t3707, &Py::str("debt_value"))?, py::getitem(&t3707, &Py::str("cost_of_equity"))?, py::getitem(&t3707, &Py::str("cost_of_debt"))?, py::getitem(&t3707, &Py::str("tax_rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3729(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3746 = v_inputs.clone(); f_whole_life_insurance(py::getitem(&t3746, &Py::str("mortality_rates"))?, py::getitem(&t3746, &Py::str("interest_rate"))?, py::method(&t3746, "get", &[Py::str("benefit"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t3778 = v_inputs.clone(); f_zero_coupon_bond(py::getitem(&t3778, &Py::str("face"))?, py::getitem(&t3778, &Py::str("ytm"))?, py::getitem(&t3778, &Py::str("years"))?, py::method(&t3778, "get", &[Py::str("compounding"), Py::str("annual")])?)? })?);
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
fn s_662(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t679 = v_inputs.clone(); f_compound_interest(py::getitem(&t679, &Py::str("principal"))?, py::getitem(&t679, &Py::str("rate"))?, py::getitem(&t679, &Py::str("years"))?, py::method(&t679, "get", &[Py::str("periods_per_year"), Py::Int(1i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_690(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t707 = v_inputs.clone(); f_continuous_compounding(py::getitem(&t707, &Py::str("principal"))?, py::getitem(&t707, &Py::str("rate"))?, py::getitem(&t707, &Py::str("years"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t743 = v_inputs.clone(); f_correlation(py::getitem(&t743, &Py::str("x"))?, py::getitem(&t743, &Py::str("y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_758(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t775 = v_inputs.clone(); f_cournot(py::getitem(&t775, &Py::str("price_intercept"))?, py::getitem(&t775, &Py::str("price_slope"))?, py::getitem(&t775, &Py::str("cost_1"))?, py::getitem(&t775, &Py::str("cost_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t805 = v_inputs.clone(); f_cross_price_elasticity(py::getitem(&t805, &Py::str("price_other_1"))?, py::getitem(&t805, &Py::str("quantity_1"))?, py::getitem(&t805, &Py::str("price_other_2"))?, py::getitem(&t805, &Py::str("quantity_2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_898(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t915 = v_inputs.clone(); f_discounted_payback(py::getitem(&t915, &Py::str("rate"))?, py::getitem(&t915, &Py::str("cash_flows"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_921(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t938 = v_inputs.clone(); f_doubling_time(py::getitem(&t938, &Py::str("rate"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_950(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t967 = v_inputs.clone(); f_durbin_watson(py::getitem(&t967, &Py::str("residuals"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_986(mut v_inputs: Py, mut v_outputs: Py) -> R {
    return Ok(f__residual(v_outputs.clone(), { let t1003 = v_inputs.clone(); f_effective_annual_rate(py::getitem(&t1003, &Py::str("rate"))?, py::getitem(&t1003, &Py::str("periods_per_year"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
