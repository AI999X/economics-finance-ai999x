//! Every Economics and Finance capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use economics_finance_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/economics-finance-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("arc_price_elasticity", &["decision"]),
    ("beta_regression", &["fit"]),
    ("binomial_option", &["convergence", "tree"]),
    ("black_scholes", &["payoff", "value"]),
    ("bond_convexity", &["curve-2"]),
    ("bond_duration", &["stems"]),
    ("bond_price", &["family", "timeline"]),
    ("bond_price_change", &["bars"]),
    ("break_even", &["chart"]),
    ("bs_greeks", &["greeks"]),
    ("cagr", &["growth"]),
    ("capm", &["line"]),
    ("cobb_douglas_demand", &["budget"]),
    ("compound_interest", &["growth"]),
    ("continuous_compounding", &["limit"]),
    ("correlation", &["xy"]),
    ("cournot", &["reaction"]),
    ("cross_price_elasticity", &["decision"]),
    ("crossover_rate", &["profile"]),
    ("discounted_payback", &["cumulative"]),
    ("durbin_watson", &["residuals"]),
    ("effective_annual_rate", &["field", "stems"]),
    ("endowment_insurance", &["bars"]),
    ("equivalent_annual_annuity", &["bars"]),
    ("forward_price_dividends", &["bars"]),
    ("forward_rate", &["rates"]),
    ("future_value_annuity", &["bars"]),
    ("gdp_deflator", &["bars"]),
    ("gini_coefficient", &["lorenz"]),
    ("global_min_variance", &["heatmap", "matrix", "series"]),
    ("growing_annuity", &["bars"]),
    ("growing_perpetuity", &["limit"]),
    ("herfindahl_index", &["bars"]),
    ("historical_var", &["stems"]),
    ("income_elasticity", &["decision"]),
    ("inflation_adjustment", &["path"]),
    ("information_ratio", &["bars"]),
    ("irr", &["profile"]),
    ("life_annuity_due", &["bars"]),
    ("life_expectancy", &["survival"]),
    ("loan_cumulative_interest", &["area"]),
    ("loan_payment", &["bars"]),
    ("loan_payment_split", &["bars", "schedule"]),
    ("log_log_elasticity", &["fit"]),
    ("market_equilibrium", &["equilibrium", "shift"]),
    ("max_drawdown", &["drawdown"]),
    ("mean_returns", &["growth"]),
    ("mirr", &["growth"]),
    ("monopoly", &["diagram"]),
    ("multiple_regression", &["fit"]),
    ("net_premium_whole_life", &["bars"]),
    ("nominal_rate", &["stems"]),
    ("npv", &["discounted"]),
    ("ols_simple", &["xy"]),
    ("parametric_var", &["density"]),
    ("payback_period", &["cumulative"]),
    ("per_unit_tax", &["wedge"]),
    ("perpetuity", &["limit"]),
    ("portfolio_return_risk", &["frontier"]),
    ("present_value_annuity", &["bars"]),
    ("price_indices", &["bars"]),
    ("profitability_index", &["bars"]),
    ("put_call_parity", &["bars"]),
    ("scenario_return", &["bars"]),
    ("sharpe_ratio", &["stems"]),
    ("simple_interest", &["bars"]),
    ("sinking_fund", &["bars"]),
    ("sortino_ratio", &["stems"]),
    ("stackelberg", &["reaction"]),
    ("survival_probability", &["survival"]),
    ("tangency_portfolio", &["frontier"]),
    ("temporary_annuity_due", &["bars"]),
    ("term_insurance", &["bars"]),
    ("treynor_jensen", &["line"]),
    ("two_asset_portfolio", &["frontier", "frontier-2"]),
    ("two_stage_ddm", &["bars"]),
    ("wacc", &["bars"]),
    ("whole_life_insurance", &["bars"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("binomial_option", "option_type", "put"),
    ("durbin_watson", "residuals", "0.5, 0.4, 0.2, -0.1, -0.3"),
    ("life_expectancy", "mortality_rates", "0.05, 0.2, 0.5, 1"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    if !f.slug.contains("-live-") {
                        check_texts(f);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a growing perpetuity whose growth is not below the rate is refused by the model: no figure at all
    let p = pack();
    let given: Vec<(String, Input)> = [("first_payment", "100"), ("rate", "0.03"), ("growth", "0.05")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "growing_perpetuity", &given, None);
    let figs = figures(p, "growing_perpetuity", &given, None, None);
    assert!(!a.solved());
    assert!(figs.is_empty());
}
