//! Live figures of options, portfolios and risk, markets and firms, and the statistics capabilities: every value drawn
//! is an output of the model or a further solve of it (figures.rs helpers).

use crate::figures::{again, again1, again_in, agrees, bars, cat_chart, cat_labels, chart, fig, finish, fm, it, list_text,
                     lrange, lst, n, other, other_in, panel, pct, range, text, Key};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "binomial_option" => vec![binomial_convergence(s), binomial_tree(s)],
        "black_scholes" => {
            let c = bs_curves(s);
            vec![bs_payoff(s, c.as_ref()), bs_value(s, c.as_ref())]
        }
        "bs_greeks" => vec![greeks(s)],
        "put_call_parity" => vec![parity_bars(s)],
        "forward_price_dividends" => vec![forward_div_bars(s)],
        "capm" => vec![capm_line(s)],
        "treynor_jensen" => vec![jensen_line(s)],
        "beta_regression" => vec![beta_fit(s)],
        "correlation" => vec![scatter_xy(s, false)],
        "ols_simple" => vec![scatter_xy(s, true)],
        "log_log_elasticity" => vec![loglog_fit(s)],
        "multiple_regression" => vec![multi_fit(s)],
        "durbin_watson" => vec![dw_residuals(s)],
        "sharpe_ratio" => vec![sharpe_stems(s)],
        "sortino_ratio" => vec![sortino_stems(s)],
        "historical_var" => vec![hvar_stems(s)],
        "parametric_var" => vec![var_density(s)],
        "max_drawdown" => vec![drawdown(s)],
        "information_ratio" => vec![info_bars(s)],
        "scenario_return" => vec![scenario_bars(s)],
        "portfolio_return_risk" => vec![portfolio_frontier(s)],
        "tangency_portfolio" => vec![tangency_frontier(s)],
        "two_asset_portfolio" => vec![two_asset_frontier(s), two_asset_family(s)],
        "global_min_variance" => vec![gmv_heatmap(s), gmv_matrix(s)],
        "cournot" => vec![reaction(s, false)],
        "stackelberg" => vec![reaction(s, true)],
        "market_equilibrium" => vec![equilibrium(s), demand_shift(s)],
        "per_unit_tax" => vec![tax_wedge(s)],
        "monopoly" => vec![monopoly_diagram(s)],
        "cobb_douglas_demand" => vec![budget(s)],
        "break_even" => vec![break_even_chart(s)],
        "gini_coefficient" => vec![lorenz(s)],
        "herfindahl_index" => vec![hhi_bars(s)],
        "gdp_deflator" => vec![gdp_bars(s)],
        "price_indices" => vec![index_bars(s)],
        _ => vec![],
    }
}

fn ok2(v: &[(f64, f64)]) -> bool {
    v.len() >= 2
}

fn p100(v: &[f64]) -> Vec<f64> {
    v.iter().map(|x| x * 100.0).collect()
}

// ------------------------------------------------------------------------------------------------ options
fn opt_type(s: &Solved) -> String {
    s.txt("option_type").unwrap_or_else(|| "call".into())
}

fn bs_inputs(s: &Solved, spot: f64, time: f64) -> Option<Vec<(&'static str, Input)>> {
    let mut v = vec![("spot", Input::Float(spot)), ("strike", Input::Float(n(s, "strike")?)), ("time", Input::Float(time)),
                     ("rate", Input::Float(n(s, "rate")?)), ("volatility", Input::Float(n(s, "volatility")?))];
    if let Some(q) = n(s, "dividend_yield") {
        v.push(("dividend_yield", Input::Float(q)));
    }
    Some(v)
}

fn binomial_convergence(s: &Solved) -> Option<Figure> {
    let (steps, price) = (n(s, "steps")?, s.out("price")?);
    let ty = opt_type(s);
    let top = (steps.max(60.0)).min(150.0) as usize;
    let mut ks: Vec<usize> = (1..=top).collect();
    if top > 80 {
        ks = (1..=top).filter(|x| x % 2 == 1 || *x == steps as usize).collect();
    }
    let pts: Vec<(f64, f64)> = ks.iter().filter_map(|kk| again1(s, &[("steps", *kk as f64)], "price").map(|p| (*kk as f64, p))).collect();
    if !ok2(&pts) {
        return None;
    }
    let bs = other_in(s, "black_scholes", &bs_inputs(s, n(s, "spot")?, n(s, "time")?)?, &ty);
    let all: Vec<f64> = pts.iter().map(|q| q.1).chain([price]).chain(bs).collect();
    let mut h = String::new();
    let mut items = vec![it("binomial tree price", "axp-c", Key::Line)];
    if let Some(b) = bs {
        items.push(it(format!("Black-Scholes {}", fm(b)), "axp-c2", Key::Dash));
    }
    let mut pl = chart(&mut h, 340.0, &format!("{} steps: {}", fm(steps), fm(price)), &items, (0.0, top as f64),
                       range(&all, 0.08, false), false, false);
    if let Some(b) = bs {
        pl.hline(b, "axp-c2");
    }
    pl.line(&pts, "axp-c");
    pl.mark(steps, price, &format!("{} steps: {}", fm(steps), fm(price)));
    pl.axes("number of time steps in the tree", &format!("{} price", ty));
    fig(s, "convergence", "Binomial option price converging as the tree grows",
        &format!("Price of a European {} (spot {}, strike {}, {} year(s), rate {}, volatility {}) from binomial trees \
                  of 1 to {} steps, each a run of the model; the prices oscillate towards the Black-Scholes value{}. \
                  Marked: {} steps, {}.", ty, fm(n(s, "spot")?), fm(n(s, "strike")?), fm(n(s, "time")?),
                 pct(n(s, "rate")?), pct(n(s, "volatility")?), top,
                 bs.map(|b| format!(" the model returns, {} (dashed)", fm(b))).unwrap_or_default(), fm(steps), fm(price)),
        finish(h, pl), 340.0)
}

fn binomial_tree(s: &Solved) -> Option<Figure> {
    let (sp, kx, t, steps) = (n(s, "spot")?, n(s, "strike")?, n(s, "time")?, n(s, "steps")?);
    let (price, delta, u, pu) = (s.out("price")?, s.out("delta")?, s.out("up_factor")?, s.out("probability_up")?);
    let nn = steps.round() as usize;
    if nn < 1 || nn > 10 || !(u > 1.0) {
        return None;
    }
    let call = opt_type(s) == "call";
    let d = 1.0 / u;
    let node = |i: usize, j: usize| sp * u.powi(j as i32) * d.powi((i - j) as i32);
    let all: Vec<f64> = (0..=nn).flat_map(|j| [node(nn, j)]).chain([sp, kx]).collect();
    let mut h = String::new();
    let items = [it("price path of the underlying", "axp-c4", Key::Line), it(format!("strike {}", fm(kx)), "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("price {}, delta {}", fm(price), fm(delta)), &items,
                       (-t * 0.04, t * 1.32), range(&all, 0.08, false), false, false);
    pl.hline(kx, "axp-c2");
    for i in 0..nn {
        for j in 0..=i {
            let (x0, x1) = (t * i as f64 / nn as f64, t * (i + 1) as f64 / nn as f64);
            pl.line(&[(x0, node(i, j)), (x1, node(i + 1, j + 1))], "axp-c4");
            pl.line(&[(x0, node(i, j)), (x1, node(i + 1, j))], "axp-c4");
        }
    }
    for i in 0..=nn {
        for j in 0..=i {
            pl.dot(t * i as f64 / nn as f64, node(i, j), "axp-d3", 3.5);
        }
    }
    for j in 0..=nn {
        let st = node(nn, j);
        let pay = if call { (st - kx).max(0.0) } else { (kx - st).max(0.0) };
        pl.label(t * 1.01, st, 4.0, "start", "axp-vt", &format!("{} → {}", fm(st), fm(pay)));
    }
    pl.dot(0.0, sp, "axp-d", 5.0);
    pl.axes("time (years)", "price of the underlying");
    fig(s, "tree", "The binomial price tree and payoffs at expiry",
        &format!("The {}-step price tree behind the binomial {} (spot {}, strike {}, {} year(s)): each step multiplies \
                  the price by the up factor the model returns, u = {}, or by 1/u; the labels at expiry give each price \
                  and the payoff max(S - K, 0) for a call (max(K - S, 0) for a put). Risk-neutral up probability {}; \
                  option price {}, delta {}, as returned.", nn, opt_type(s), fm(sp), fm(kx), fm(t), fm(u), fm(pu),
                 fm(price), fm(delta)), finish(h, pl), 340.0)
}

/// Call and put values over underlying prices: now and with 1e-6 year left (the payoff), each a run of the model.
struct BsCurves {
    ss: Vec<f64>,
    call: Vec<f64>,
    put: Vec<f64>,
    call_x: Vec<f64>,
    put_x: Vec<f64>,
}

fn bs_curves(s: &Solved) -> Option<BsCurves> {
    let (kx, t) = (n(s, "strike")?, n(s, "time")?);
    if !(kx > 0.0) {
        return None;
    }
    let ss = k::span(kx * 0.5, kx * 1.5, 61, false);
    let mut c = BsCurves { ss: vec![], call: vec![], put: vec![], call_x: vec![], put_x: vec![] };
    for sv in &ss {
        let (Some(a), Some(b)) = (s.solve_other(s.key, &bs_inputs(s, *sv, t)?), s.solve_other(s.key, &bs_inputs(s, *sv, 1e-6)?)) else { continue };
        let (Some(c1), Some(p1), Some(c2), Some(p2)) = (k::num_of(&a, "call"), k::num_of(&a, "put"), k::num_of(&b, "call"), k::num_of(&b, "put")) else { continue };
        c.ss.push(*sv);
        c.call.push(c1);
        c.put.push(p1);
        c.call_x.push(c2);
        c.put_x.push(p2);
    }
    (c.ss.len() >= 2).then_some(c)
}

fn zip(a: &[f64], b: &[f64]) -> Vec<(f64, f64)> {
    a.iter().zip(b).map(|(x, y)| (*x, *y)).collect()
}

fn bs_payoff(s: &Solved, c: Option<&BsCurves>) -> Option<Figure> {
    let c = c?;
    let (kx, t, prem) = (n(s, "strike")?, n(s, "time")?, s.out("call")?);
    let profit: Vec<(f64, f64)> = c.ss.iter().zip(&c.call_x).map(|(x, v)| (*x, v - prem)).collect();
    let before: Vec<(f64, f64)> = c.ss.iter().zip(&c.call).map(|(x, v)| (*x, v - prem)).collect();
    let be = profit.windows(2).find(|w| w[0].1 < 0.0 && w[1].1 >= 0.0)
        .map(|w| w[0].0 + (w[1].0 - w[0].0) * (-w[0].1) / (w[1].1 - w[0].1));
    let all: Vec<f64> = profit.iter().chain(&before).map(|q| q.1).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("profit at expiry", "axp-c", Key::Line), it(format!("value - premium with {} year(s) left", fm(t)), "axp-c4", Key::Dash),
                 it("loss", "axp-s", Key::Box), it("profit", "axp-s2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("premium {}", fm(prem)), &items, (c.ss[0], c.ss[c.ss.len() - 1]),
                       range(&all, 0.06, false), false, false);
    let mut loss: Vec<(f64, f64)> = profit.iter().map(|(x, y)| (*x, y.min(0.0))).collect();
    loss.push((c.ss[c.ss.len() - 1], 0.0));
    loss.push((c.ss[0], 0.0));
    pl.fill(&loss, "axp-s");
    let mut gain: Vec<(f64, f64)> = profit.iter().map(|(x, y)| (*x, y.max(0.0))).collect();
    gain.push((c.ss[c.ss.len() - 1], 0.0));
    gain.push((c.ss[0], 0.0));
    pl.fill(&gain, "axp-s2");
    pl.hline(0.0, "axp-c4");
    pl.dash(&before, "axp-c4");
    pl.line(&profit, "axp-c");
    pl.vline(kx, "axp-c4");
    if let Some(b) = be {
        pl.mark(b, 0.0, &format!("break-even {}", fm(b)));
    }
    pl.axes("price of the underlying at expiry", "profit of one long call");
    fig(s, "payoff", "Call option payoff: profit and loss at expiry",
        &format!("Profit and loss at expiry of a European call (strike {}) bought for the Black-Scholes price the model \
                  returns, {} (rate {}, volatility {}, {} year(s)): the payoff is the model's value with 1e-6 year left, \
                  at {} prices, minus the premium. The loss (shaded) is capped at the premium{}. Dashed: value before \
                  expiry minus the premium.", fm(kx), fm(prem), pct(n(s, "rate")?), pct(n(s, "volatility")?), fm(t),
                 c.ss.len(), be.map(|b| format!("; profit starts above the break-even price S = {}", fm(b))).unwrap_or_default()),
        finish(h, pl), 340.0)
}

fn bs_value(s: &Solved, c: Option<&BsCurves>) -> Option<Figure> {
    let c = c?;
    let (sp, kx, t, call, put) = (n(s, "spot")?, n(s, "strike")?, n(s, "time")?, s.out("call")?, s.out("put")?);
    let all: Vec<f64> = c.call.iter().chain(&c.put).chain(&c.call_x).chain(&c.put_x).copied().chain([0.0, call, put]).collect();
    let mut h = String::new();
    let items = [it("call", "axp-c", Key::Line), it("put", "axp-c3", Key::Line), it("at expiry", "axp-c4", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("S = {}: call {}, put {}", fm(sp), fm(call), fm(put)), &items,
                       (c.ss[0], c.ss[c.ss.len() - 1]), range(&all, 0.05, true), false, false);
    pl.dash(&zip(&c.ss, &c.call_x), "axp-c4");
    pl.dash(&zip(&c.ss, &c.put_x), "axp-c4");
    pl.line(&zip(&c.ss, &c.call), "axp-c");
    pl.line(&zip(&c.ss, &c.put), "axp-c3");
    pl.vline(kx, "axp-c4");
    pl.mark(sp, call, &format!("call {}", fm(call)));
    pl.dot(sp, put, "axp-d3", 5.0);
    pl.label(sp, put, 18.0, "start", "axp-vt", &format!("  put {}", fm(put)));
    pl.axes("price of the underlying now", "option value");
    fig(s, "value", "Call and put values against the underlying price",
        &format!("Black-Scholes values of a European call and put (strike {}, rate {}, volatility {}, {} year(s) to \
                  expiry) at {} underlying prices, each a run of the model; dashed: the values with 1e-6 year left (the \
                  payoff at expiry). The gap is the time value. Marked: S = {}, call {}, put {}.", fm(kx),
                 pct(n(s, "rate")?), pct(n(s, "volatility")?), fm(t), c.ss.len(), fm(sp), fm(call), fm(put)),
        finish(h, pl), 340.0)
}

fn greeks(s: &Solved) -> Option<Figure> {
    let (sp, kx, t) = (n(s, "spot")?, n(s, "strike")?, n(s, "time")?);
    let (cd, pd, g, v) = (s.out("call_delta")?, s.out("put_delta")?, s.out("gamma")?, s.out("vega")?);
    if !(kx > 0.0) {
        return None;
    }
    let lo = (kx * 0.5).min(sp * 0.9);
    let hi = (kx * 1.5).max(sp * 1.1);
    let ss = k::span(lo, hi, 51, false);
    let (mut a, mut b, mut c, mut d) = (vec![], vec![], vec![], vec![]);
    for x in &ss {
        let Some(r) = again(s, &[("spot", *x)]) else { continue };
        let (Some(p), Some(q), Some(gg), Some(vv)) = (k::num_of(&r, "call_delta"), k::num_of(&r, "put_delta"), k::num_of(&r, "gamma"), k::num_of(&r, "vega")) else { continue };
        a.push((*x, p));
        b.push((*x, q));
        c.push((*x, gg));
        d.push((*x, vv));
    }
    if !ok2(&a) {
        return None;
    }
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("S = {}: delta {} / {}", fm(sp), fm(cd), fm(pd)));
    let lg = [it("call delta", "axp-c", Key::Line), it("put delta", "axp-c3", Key::Line), it("gamma", "axp-c2", Key::Line),
              it("vega", "axp-c4", Key::Line)];
    let top = crate::figures::legend(&mut h, &lg, 40.0) + 6.0;
    let xr = (lo, hi);
    let mut p1 = panel(top, top + 140.0, xr, (-1.08, 1.08), false, false, 1);
    p1.line(&a, "axp-c");
    p1.line(&b, "axp-c3");
    p1.vline(kx, "axp-c4");
    p1.mark(sp, cd, &fm(cd));
    p1.dot(sp, pd, "axp-d3", 5.0);
    p1.axes_with("", "delta", true, true);
    let mut p2 = panel(top + 170.0, top + 310.0, xr, range(&c.iter().map(|q| q.1).chain([g, 0.0]).collect::<Vec<_>>(), 0.08, true), false, false, 2);
    p2.line(&c, "axp-c2");
    p2.vline(kx, "axp-c4");
    p2.mark(sp, g, &fm(g));
    p2.axes_with("", "gamma", true, true);
    let mut p3 = panel(top + 340.0, top + 480.0, xr, range(&d.iter().map(|q| q.1).chain([v, 0.0]).collect::<Vec<_>>(), 0.08, true), false, false, 3);
    p3.line(&d, "axp-c4");
    p3.vline(kx, "axp-c4");
    p3.mark(sp, v, &fm(v));
    p3.axes("price of the underlying", "vega");
    let body = h + &p1.s + &p2.s + &p3.s;
    fig(s, "greeks", "Delta, gamma and vega against the underlying price",
        &format!("Delta (call and put), gamma and vega of a European option (strike {}, {} year(s), rate {}, \
                  volatility {}) at {} underlying prices, each a run of the model; the vertical line is the strike. \
                  Gamma and vega peak near the money. Marked at S = {}: call delta {}, put delta {}, gamma {}, vega {}.",
                 fm(kx), fm(t), pct(n(s, "rate")?), pct(n(s, "volatility")?), a.len(), fm(sp), fm(cd), fm(pd), fm(g), fm(v)),
        body, top + 534.0)
}

fn parity_bars(s: &Solved) -> Option<Figure> {
    let (c, kx, t, r) = (n(s, "call_price")?, n(s, "strike")?, n(s, "time")?, n(s, "rate")?);
    let (p, pvk, pvs) = (s.out("put_price")?, s.out("pv_strike")?, s.out("pv_spot")?);
    let all = [c + pvk, p + pvs, 0.0];
    let mut h = String::new();
    let items = [it("call", "axp-b", Key::Box), it("present value of the strike", "axp-b4", Key::Box),
                 it("put", "axp-b3", Key::Box), it("underlying (present value)", "axp-b2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("both portfolios worth {}", fm(c + pvk)), &items, (-0.7, 1.7), range(&all, 0.1, true), false, false);
    pl.rect(-0.3, 0.3, 0.0, pvk, "axp-b4");
    pl.rect(-0.3, 0.3, pvk, pvk + c, "axp-b");
    pl.rect(0.7, 1.3, 0.0, pvs, "axp-b2");
    pl.rect(0.7, 1.3, pvs, pvs + p, "axp-b3");
    pl.label(0.0, pvk / 2.0, 5.0, "middle", "axp-bt", &fm(pvk));
    pl.label(0.0, pvk + c, -6.0, "middle", "axp-vt", &format!("call {}", fm(c)));
    pl.label(1.0, pvs / 2.0, 5.0, "middle", "axp-bt", &fm(pvs));
    pl.label(1.0, pvs + p, -6.0, "middle", "axp-vt", &format!("put {}", fm(p)));
    for (i, l) in ["call + PV(strike)", "put + underlying"].iter().enumerate() {
        let x = pl.x(i as f64);
        let y = pl.y1 + 19.0;
        text(&mut pl.s, x, y, "middle", "axp-lt", l);
    }
    pl.axes_with("", "value", false, true);
    fig(s, "bars", "Put-call parity as two equal portfolios",
        &format!("Put-call parity: a call priced {} plus the present value of the strike ({}, returned) is worth the \
                  same as the put plus the underlying ({} + {}, both returned), for strike {}, {} year(s) at {}; the \
                  two bars have equal height.", fm(c), fm(pvk), fm(p), fm(pvs), fm(kx), fm(t), pct(r)), finish(h, pl), 340.0)
}

fn forward_div_bars(s: &Solved) -> Option<Figure> {
    let (sp, r, t) = (n(s, "spot")?, n(s, "rate")?, n(s, "time")?);
    let (fw, pvd) = (s.out("forward")?, s.out("pv_dividends")?);
    let dv = s.vec("dividends").unwrap_or_default();
    let dt = s.vec("dividend_times").unwrap_or_default();
    let all = [sp, sp - pvd, fw];
    let (lo, hi) = range(&all, 0.4, false);
    let mut h = String::new();
    let items = [it("spot price", "axp-b4", Key::Box), it("present value of the dividends", "axp-b", Key::Box), it("forward price", "axp-b2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("forward {}", fm(fw)), &items, (-0.6, 2.6), (lo, hi), false, false);
    pl.rect(-0.3, 0.3, lo, sp, "axp-b4");
    pl.rect(0.7, 1.3, sp - pvd, sp, "axp-b");
    pl.rect(1.7, 2.3, lo, fw, "axp-b2");
    pl.dash(&[(0.3, sp), (0.7, sp)], "axp-c4");
    pl.dash(&[(1.3, sp - pvd), (1.7, sp - pvd)], "axp-c4");
    pl.label(0.0, sp, -6.0, "middle", "axp-vt", &format!("spot {}", fm(sp)));
    pl.label(1.0, sp - pvd, 15.0, "middle", "axp-vt", &format!("- {}", fm(pvd)));
    pl.label(2.0, fw, -6.0, "middle", "axp-vt", &format!("forward {}", fm(fw)));
    for (i, l) in ["spot", "less dividends", "carried at the rate"].iter().enumerate() {
        let x = pl.x(i as f64);
        let y = pl.y1 + 19.0;
        text(&mut pl.s, x, y, "middle", "axp-lt", l);
    }
    pl.axes_with("", "amount", false, true);
    fig(s, "bars", "Forward price of a dividend-paying asset",
        &format!("Forward price for delivery in {} year(s) of an asset priced {} that pays dividends {} at times {}: \
                  the model returns the present value of the dividends, {}, which is taken off the spot, and the forward \
                  price {}, the remainder carried at {}. The value axis does not start at zero.", fm(t), fm(sp),
                 list_text(&dv), list_text(&dt), fm(pvd), fm(fw), pct(r)), finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ CAPM and regressions
fn capm_line(s: &Solved) -> Option<Figure> {
    let (rf, b, rm, er) = (n(s, "risk_free")?, n(s, "beta")?, n(s, "market_return")?, s.out("expected_return")?);
    let bmax = (b * 1.3).max(2.0);
    let bmin = b.min(0.0) * 1.3;
    let pts: Vec<(f64, f64)> = k::span(bmin, bmax, 41, false).iter().filter_map(|x| again1(s, &[("beta", *x)], "expected_return").map(|y| (*x, y * 100.0))).collect();
    if !ok2(&pts) {
        return None;
    }
    let all: Vec<f64> = pts.iter().map(|q| q.1).chain([rf * 100.0, rm * 100.0]).collect();
    let mut h = String::new();
    let items = [it("security market line", "axp-c", Key::Line), it("risk-free asset and the market", "axp-d2", Key::Dot)];
    let mut pl = chart(&mut h, 340.0, &format!("beta {}: {}", fm(b), pct(er)), &items, (bmin, bmax), range(&all, 0.08, false), false, false);
    pl.line(&pts, "axp-c");
    pl.dot(0.0, rf * 100.0, "axp-d2", 5.0);
    pl.dot(1.0, rm * 100.0, "axp-d2", 5.0);
    pl.label(0.0, rf * 100.0, 18.0, "start", "axp-lt", &format!("risk-free {}", pct(rf)));
    pl.label(1.0, rm * 100.0, 18.0, "start", "axp-lt", &format!("market {}", pct(rm)));
    pl.mark(b, er * 100.0, &format!("beta {}: {}", fm(b), pct(er)));
    pl.axes("beta", "expected return (%)");
    fig(s, "line", "Security market line (CAPM)",
        &format!("The security market line for a risk-free rate of {} and a market return of {}: the expected return \
                  the model returns at {} betas from {} to {}. Marked: beta = {}, expected return {} (risk premium {}).",
                 pct(rf), pct(rm), pts.len(), fm(bmin), fm(bmax), fm(b), pct(er), pct(s.out("risk_premium").unwrap_or(er - rf))),
        finish(h, pl), 340.0)
}

fn jensen_line(s: &Solved) -> Option<Figure> {
    let rf = n(s, "risk_free")?;
    let (b, tr, al, mp, mm) = (s.out("beta")?, s.out("treynor")?, s.out("jensen_alpha")?, s.out("mean_portfolio_return")?, s.out("mean_market_return")?);
    let bmax = (b * 1.4).max(1.6);
    let bmin = b.min(0.0) * 1.3;
    let sml: Vec<(f64, f64)> = k::span(bmin, bmax, 21, false).iter().filter_map(|x| other(s, "capm", &[("risk_free", rf), ("beta", *x), ("market_return", mm)], "expected_return").map(|y| (*x, y * 100.0))).collect();
    let on = other(s, "capm", &[("risk_free", rf), ("beta", b), ("market_return", mm)], "expected_return")?;
    if !ok2(&sml) {
        return None;
    }
    let all: Vec<f64> = sml.iter().map(|q| q.1).chain([mp * 100.0, rf * 100.0]).collect();
    let mut h = String::new();
    let items = [it("security market line", "axp-c4", Key::Line), it("risk-free and market", "axp-d2", Key::Dot),
                 it(format!("Jensen's alpha {}", pct(al)), "axp-c", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("beta {}, Treynor {}", fm(b), fm(tr)), &items, (bmin, bmax), range(&all, 0.12, false), false, false);
    pl.line(&sml, "axp-c4");
    pl.dot(0.0, rf * 100.0, "axp-d2", 5.0);
    pl.dot(1.0, mm * 100.0, "axp-d2", 5.0);
    pl.dash(&[(b, on * 100.0), (b, mp * 100.0)], "axp-c");
    pl.mark(b, mp * 100.0, &format!("portfolio {}", pct(mp)));
    pl.axes("beta", "mean return (%)");
    fig(s, "line", "Jensen's alpha above the security market line",
        &format!("The portfolio against the market with a risk-free rate of {}: the model returns beta {}, mean returns \
                  {} (portfolio) and {} (market). The line through the risk-free and market points is the security \
                  market line (CAPM runs of the model); the portfolio's height above it is the returned Jensen's alpha \
                  {}; Treynor ratio {}.", pct(rf), fm(b), pct(mp), pct(mm), pct(al), fm(tr)), finish(h, pl), 340.0)
}

fn scatter_chart(s: &Solved, kind: &str, title: &str, cap: String, x: &[f64], y: &[f64], line: Option<(f64, f64)>,
                 xl: &str, yl: &str, header: &str, logs: bool) -> Option<Figure> {
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }
    let xr = if logs { lrange(x, 1.3) } else { range(x, 0.08, false) };
    let mut yv: Vec<f64> = y.to_vec();
    if let Some((a, b)) = line {
        for xx in [xr.0, xr.1] {
            yv.push(if logs { (a + b * xx.ln()).exp() } else { a + b * xx });
        }
    }
    let yr = if logs { lrange(&yv, 1.3) } else { range(&yv, 0.08, false) };
    let mut h = String::new();
    let mut items = vec![it("data", "axp-d", Key::Dot)];
    if line.is_some() {
        items.push(it("fitted line from the returned values", "axp-c2", Key::Line));
    }
    let mut pl = chart(&mut h, 340.0, header, &items, xr, yr, logs, logs);
    if let Some((a, b)) = line {
        let xs = k::span(xr.0, xr.1, 30, logs);
        let pts: Vec<(f64, f64)> = xs.iter().map(|xx| (*xx, if logs { (a + b * xx.ln()).exp() } else { a + b * xx })).collect();
        pl.line(&pts, "axp-c2");
    }
    for (a, b) in x.iter().zip(y) {
        pl.dot(*a, *b, "axp-d", 4.5);
    }
    pl.axes(xl, yl);
    fig(s, kind, title, &cap, finish(h, pl), 340.0)
}

fn beta_fit(s: &Solved) -> Option<Figure> {
    let (a, m) = (s.vec("asset_returns")?, s.vec("market_returns")?);
    let (b, al, c, r2) = (s.out("beta")?, s.out("alpha")?, s.out("correlation")?, s.out("r_squared")?);
    let cap = format!("Asset returns {} against market returns {}: the model returns the regression slope beta = {} and \
                       intercept alpha = {} (the line), correlation {}, R squared = {}.", list_text(&a), list_text(&m),
                      fm(b), fm(al), fm(c), fm(r2));
    scatter_chart(s, "fit", "Beta as the slope of asset against market returns", cap, &p100(&m), &p100(&a),
                  Some((al * 100.0, b)), "market return (%)", "asset return (%)", &format!("slope beta = {}", fm(b)), false)
}

fn scatter_xy(s: &Solved, ols: bool) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    if ols {
        let (b, a, r2) = (s.out("slope")?, s.out("intercept")?, s.out("r_squared")?);
        let se = s.out("slope_std_error");
        let cap = format!("The data y against x as given ({} points); drawn on it from what the model returned: the line \
                           y = {} + {} x from the returned intercept and slope. Also returned: R squared = {}{}.", x.len(),
                          fm(a), fm(b), fm(r2), se.map(|v| format!(", slope standard error {}", fm(v))).unwrap_or_default());
        scatter_chart(s, "xy", "Simple linear regression: the data and the fitted line", cap, &x, &y, Some((a, b)),
                      "x (explanatory)", "y (dependent)", &format!("y = {} + {} x", fm(a), fm(b)), false)
    } else {
        let (r, cv, r2) = (s.out("correlation")?, s.out("covariance")?, s.out("r_squared")?);
        let cap = format!("The data y (second series) against x (first series) as given ({} points). Returned: \
                           correlation = {}, covariance = {}, R squared = {}.", x.len(), fm(r), fm(cv), fm(r2));
        scatter_chart(s, "xy", "Correlation and covariance of two series: the data and what was returned", cap, &x, &y,
                      None, "x (first series)", "y (second series)", &format!("r = {}", fm(r)), false)
    }
}

fn loglog_fit(s: &Solved) -> Option<Figure> {
    let (p, q) = (s.vec("prices")?, s.vec("quantities")?);
    let (e, a, r2) = (s.out("elasticity")?, s.out("intercept")?, s.out("r_squared")?);
    if p.iter().chain(&q).any(|v| *v <= 0.0) {
        return None;
    }
    let cap = format!("Quantities {} at prices {} on logarithmic axes: the model fits ln Q = a + e ln P and returns the \
                       elasticity e = {}, intercept a = {} and R squared = {}; the line Q = exp(a) P^e is drawn from those \
                       returned values.", list_text(&q), list_text(&p), fm(e), fm(a), fm(r2));
    scatter_chart(s, "fit", "Constant elasticity as a straight line on log-log axes", cap, &p, &q, Some((a, e)),
                  "price P", "quantity Q", &format!("elasticity {}", fm(e)), true)
}

fn multi_fit(s: &Solved) -> Option<Figure> {
    let (xm, y) = (s.mat("X")?, s.vec("y")?);
    let (c, r2) = (s.out_list("coefficients")?, s.out("r_squared")?);
    let adj = s.out("adj_r_squared");
    if xm.len() != y.len() || c.len() != xm[0].len() + 1 {
        return None;
    }
    let fit: Vec<f64> = xm.iter().map(|row| c[0] + row.iter().zip(&c[1..]).map(|(a, b)| a * b).sum::<f64>()).collect();
    let all: Vec<f64> = y.iter().chain(&fit).copied().collect();
    let r = range(&all, 0.08, false);
    let mut h = String::new();
    let items = [it("observation", "axp-d", Key::Dot), it("fitted exactly", "axp-c4", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("R squared = {}", fm(r2)), &items, r, r, false, false);
    pl.dash(&[(r.0, r.0), (r.1, r.1)], "axp-c4");
    for (a, b) in fit.iter().zip(&y) {
        pl.dot(*a, *b, "axp-d", 4.5);
    }
    pl.axes("fitted value", "observed y");
    fig(s, "fit", "Multiple regression: observed against fitted",
        &format!("Observed y = {} against the values fitted from X with the coefficients the model returns (intercept \
                  first: {}), R squared = {}{}. Points on the dashed line are fitted exactly.", list_text(&y),
                 list_text(&c), fm(r2), adj.map(|v| format!(", adjusted {}", fm(v))).unwrap_or_default()),
        finish(h, pl), 340.0)
}

fn dw_residuals(s: &Solved) -> Option<Figure> {
    let (e, d) = (s.vec("residuals")?, s.out("durbin_watson")?);
    if e.len() < 2 {
        return None;
    }
    let nn = e.len();
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("d = {}", fm(d)));
    let mut p1 = panel(36.0, 200.0, (0.4, nn as f64 + 0.6), range(&e, 0.1, true), false, false, 1);
    p1.hline(0.0, "axp-c4");
    p1.line(&crate::figures::indexed(&e, 1.0), "axp-c4");
    for (i, v) in e.iter().enumerate() {
        p1.stem(i as f64 + 1.0, 0.0, *v, "axp-c", if *v >= 0.0 { "axp-d" } else { "axp-d3" });
    }
    p1.axes("observation", "residual");
    let mut p2 = panel(286.0, 318.0, (0.0, 4.0), (0.0, 1.0), false, false, 2);
    p2.rect(0.0, 1.5, 0.0, 1.0, "axp-s");
    p2.rect(1.5, 2.5, 0.0, 1.0, "axp-s4");
    p2.rect(2.5, 4.0, 0.0, 1.0, "axp-s3");
    p2.label(0.75, 0.5, 5.0, "middle", "axp-lt", "positive");
    p2.label(2.0, 0.5, 5.0, "middle", "axp-lt", "none");
    p2.label(3.25, 0.5, 5.0, "middle", "axp-lt", "negative");
    p2.vline(d.clamp(0.0, 4.0), "axp-c");
    p2.axes_with("", "", true, false);
    p2.note(204.0, -8.0, "middle", "axp-lt", "Durbin-Watson scale: autocorrelation of the residuals");
    let x = p2.x(d.clamp(0.0, 4.0));
    p2.s.push_str(&format!("<circle class=\"axp-m\" cx=\"{x:.1}\" cy=\"302\" r=\"6\"/>"));
    text(&mut p2.s, x, 362.0, "middle", "axp-mt", &format!("d = {}", fm(d)));
    let body = h + &p1.s + &p2.s;
    fig(s, "residuals", "Durbin-Watson statistic of the residuals",
        &format!("The residuals {} in time order (top) and the Durbin-Watson statistic the model returns, d = {}, on its \
                  scale from 0 to 4 (bottom): near 2 there is no first-order autocorrelation; residuals that keep their \
                  sign push d below 2, residuals that alternate push it above 2.", list_text(&e), fm(d)), body, 380.0)
}

// ------------------------------------------------------------------------------------------------ risk
fn stem_frame(s: &Solved, r: &[f64], extra: &[f64], header: &str, items: &[(String, &'static str, Key)]) -> (String, crate::figures::Plot) {
    let all: Vec<f64> = p100(r).into_iter().chain(extra.iter().copied()).chain([0.0]).collect();
    let mut h = String::new();
    let pl = chart(&mut h, 340.0, header, items, (0.4, r.len() as f64 + 0.6), range(&all, 0.1, false), false, false);
    let _ = s;
    (h, pl)
}

fn sharpe_stems(s: &Solved) -> Option<Figure> {
    let (r, rf) = (s.vec("returns")?, n(s, "risk_free")?);
    let (sh, ash, mx, sd) = (s.out("sharpe")?, s.out("annualized_sharpe")?, s.out("mean_excess_return")?, s.out("std_dev")?);
    if r.len() > 200 {
        return None;
    }
    let mean = (rf + mx) * 100.0;
    let items = [it("return", "axp-d", Key::Dot), it(format!("risk-free {}", pct(rf)), "axp-c4", Key::Dash),
                 it("mean +/- one standard deviation", "axp-s2", Key::Box)];
    let (h, mut pl) = stem_frame(s, &r, &[mean + sd * 100.0, mean - sd * 100.0, rf * 100.0], &format!("Sharpe ratio {}", fm(sh)), &items);
    let nn = r.len() as f64;
    pl.rect(0.4, nn + 0.6, mean - sd * 100.0, mean + sd * 100.0, "axp-s2");
    pl.line(&[(0.4, mean), (nn + 0.6, mean)], "axp-c2");
    pl.hline(rf * 100.0, "axp-c4");
    pl.hline(0.0, "axp-c4");
    for (i, v) in r.iter().enumerate() {
        pl.stem(i as f64 + 1.0, 0.0, v * 100.0, "axp-c", "axp-d");
    }
    pl.label(nn + 0.6, mean, -6.0, "end", "axp-vt", &format!("mean {} %", fm(mean)));
    pl.axes("period", "return (%)");
    fig(s, "stems", "Sharpe ratio: excess return per unit of volatility",
        &format!("Returns {} against a risk-free rate of {}: the model returns the mean excess return {} and the \
                  standard deviation {} (the band around the mean), whose ratio is the Sharpe ratio {} (annualised {}).",
                 list_text(&r), pct(rf), pct(mx), pct(sd), fm(sh), fm(ash)), finish(h, pl), 340.0)
}

fn sortino_stems(s: &Solved) -> Option<Figure> {
    let (r, tg) = (s.vec("returns")?, n(s, "target")?);
    let (so, dd, mr) = (s.out("sortino")?, s.out("downside_deviation")?, s.out("mean_return")?);
    if r.len() > 200 {
        return None;
    }
    let items = [it("shortfall below the target", "axp-d", Key::Dot), it("other return", "axp-d4", Key::Dot),
                 it(format!("target {}", pct(tg)), "axp-c4", Key::Dash), it(format!("downside deviation {}", pct(dd)), "axp-s", Key::Box)];
    let (h, mut pl) = stem_frame(s, &r, &[(tg - dd) * 100.0, mr * 100.0], &format!("Sortino ratio {}", fm(so)), &items);
    let nn = r.len() as f64;
    pl.rect(0.4, nn + 0.6, (tg - dd) * 100.0, tg * 100.0, "axp-s");
    pl.hline(tg * 100.0, "axp-c4");
    pl.line(&[(0.4, mr * 100.0), (nn + 0.6, mr * 100.0)], "axp-c2");
    pl.label(nn + 0.6, mr * 100.0, -6.0, "end", "axp-vt", &format!("mean {}", pct(mr)));
    for (i, v) in r.iter().enumerate() {
        let short = *v < tg;
        pl.stem(i as f64 + 1.0, tg * 100.0, v * 100.0, if short { "axp-c" } else { "axp-c4" }, if short { "axp-d" } else { "axp-d4" });
    }
    pl.axes("period", "return (%)");
    fig(s, "stems", "Sortino ratio: only shortfalls below the target count",
        &format!("Returns {} measured from the target {}: only the shortfalls (highlighted) count as risk; the model \
                  returns their downside deviation {}, the mean return {} and the Sortino ratio {}.", list_text(&r),
                 pct(tg), pct(dd), pct(mr), fm(so)), finish(h, pl), 340.0)
}

fn hvar_stems(s: &Solved) -> Option<Figure> {
    let (r, conf) = (s.vec("returns")?, n(s, "confidence")?);
    let (var, cvar, nt) = (s.out("var")?, s.out("cvar")?, s.out("tail_observations")?);
    if r.len() > 300 {
        return None;
    }
    let pv = n(s, "portfolio_value");
    // as a fraction of the value when a portfolio value is given
    let (fv, fc) = match pv { Some(v) if v > 0.0 => (var / v, cvar / v), _ => (var, cvar) };
    let mut idx: Vec<usize> = (0..r.len()).collect();
    idx.sort_by(|a, b| r[*a].partial_cmp(&r[*b]).unwrap());
    let tail: Vec<usize> = idx.into_iter().take(nt.round() as usize).collect();
    let items = [it("return in the tail", "axp-d", Key::Dot), it("other return", "axp-d4", Key::Dot),
                 it(format!("VaR: loss of {}", pct(fv)), "axp-c", Key::Dash), it(format!("CVaR {}", pct(fc)), "axp-c3", Key::Dash)];
    let (h, mut pl) = stem_frame(s, &r, &[-fv * 100.0, -fc * 100.0], &format!("{} VaR {}", pct(conf), pct(fv)), &items);
    pl.hline(0.0, "axp-c4");
    pl.hline(-fv * 100.0, "axp-c");
    pl.hline(-fc * 100.0, "axp-c3");
    for (i, v) in r.iter().enumerate() {
        let t = tail.contains(&i);
        pl.stem(i as f64 + 1.0, 0.0, v * 100.0, if t { "axp-c" } else { "axp-c4" }, if t { "axp-d" } else { "axp-d4" });
    }
    pl.axes("observation", "return (%)");
    fig(s, "stems", "Historical value at risk on the observed returns",
        &format!("The historical returns {} as stems: the model returns the {} value at risk, a loss of {}, and the \
                  average loss beyond it (CVaR) {}; the {} return(s) in the tail are highlighted.", list_text(&r),
                 pct(conf), if pv.is_some() { fm(var) } else { pct(var) }, if pv.is_some() { fm(cvar) } else { pct(cvar) },
                 fm(nt)), finish(h, pl), 340.0)
}

fn var_density(s: &Solved) -> Option<Figure> {
    let (conf, pv) = (n(s, "confidence")?, n(s, "portfolio_value")?);
    let (var, cvar) = (s.out("var")?, s.out("cvar")?);
    let ps: Vec<f64> = (0..120).map(|i| 1.0 - 0.5 * (2e-4f64).powf(i as f64 / 119.0)).collect();
    let mut q = vec![];
    for p in &ps {
        if let Some(l) = again1(s, &[("confidence", *p)], "var") {
            q.push((*p, l));
        }
    }
    if q.len() < 10 {
        return None;
    }
    let dens: Vec<(f64, f64)> = q.windows(2).filter(|w| w[1].1 > w[0].1)
        .map(|w| ((w[0].1 + w[1].1) / 2.0, (w[1].0 - w[0].0) / (w[1].1 - w[0].1))).collect();
    if dens.len() < 5 {
        return None;
    }
    let xs: Vec<f64> = dens.iter().map(|d| d.0).chain([var, cvar]).collect();
    let ys: Vec<f64> = dens.iter().map(|d| d.1).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("loss density", "axp-c", Key::Line), it(format!("beyond the {} VaR", pct(conf)), "axp-s", Key::Box),
                 it(format!("CVaR {}", fm(cvar)), "axp-c3", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("VaR {}", fm(var)), &items, range(&xs, 0.04, false), range(&ys, 0.08, true), false, false);
    let mut tail: Vec<(f64, f64)> = dens.iter().copied().filter(|d| d.0 >= var).collect();
    if let Some(f) = tail.first().copied() {
        tail.insert(0, (var, f.1));
        tail.insert(0, (var, 0.0));
        let last = tail[tail.len() - 1].0;
        tail.push((last, 0.0));
        pl.fill(&tail, "axp-s");
    }
    pl.line(&dens, "axp-c");
    pl.vline(cvar, "axp-c3");
    let at = dens.iter().min_by(|a, b| (a.0 - var).abs().partial_cmp(&(b.0 - var).abs()).unwrap())?.1;
    pl.mark(var, at, &format!("VaR {}", fm(var)));
    pl.axes("loss over the horizon", "probability density");
    fig(s, "density", "Value at risk and expected shortfall on the loss distribution",
        &format!("The distribution of the loss on a {} portfolio over {} period(s) (mean return {}, standard deviation \
                  {} per period), above its median: the density is drawn from the VaR the model returns at {} \
                  confidence levels between 0.5 and 0.9999 (probability differences divided by loss differences). \
                  Shaded: losses beyond the {} VaR = {}; the expected loss in that tail, CVaR = {}, is also returned.",
                 fm(pv), fm(n(s, "horizon")?), fm(n(s, "mean")?), fm(n(s, "std_dev")?), q.len(), pct(conf), fm(var), fm(cvar)),
        finish(h, pl), 340.0)
}

fn drawdown(s: &Solved) -> Option<Figure> {
    let (pr, md) = (s.vec("prices")?, s.out("max_drawdown")?);
    if pr.len() < 2 || pr.len() > 150 {
        return None;
    }
    let mut dd = vec![(0.0, 0.0)];
    for kk in 1..pr.len() {
        let v = if kk == pr.len() - 1 { md } else { again_in(s, &[("prices", lst(&pr[..=kk]))], "max_drawdown")? };
        dd.push((kk as f64, -v * 100.0));
    }
    let nn = pr.len() as f64;
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("maximum drawdown {}", pct(md)));
    let mut p1 = panel(36.0, 196.0, (-0.3, nn - 0.7), range(&pr, 0.1, false), false, false, 1);
    let pts = crate::figures::indexed(&pr, 0.0);
    p1.line(&pts, "axp-c3");
    for (x, y) in &pts {
        p1.dot(*x, *y, "axp-d3", 3.5);
    }
    p1.axes("", "price");
    let mut p2 = panel(236.0, 398.0, (-0.3, nn - 0.7), range(&dd.iter().map(|q| q.1).collect::<Vec<_>>(), 0.1, true), false, false, 2);
    let mut area = dd.clone();
    area.push((nn - 1.0, 0.0));
    p2.fill(&area, "axp-s");
    p2.line(&dd, "axp-c");
    let worst = dd.iter().copied().fold((0.0, 0.0), |a, b| if b.1 < a.1 { b } else { a });
    p2.mark(worst.0, worst.1, &format!("maximum {}", pct(md)));
    p2.axes("observation", "largest fall so far (%)");
    let body = h + &p1.s + &p2.s;
    fig(s, "drawdown", "Drawdown of a price series",
        &format!("Top: the price series {}. Bottom: the largest fall from a previous peak seen so far, a run of the \
                  model on the prices up to each observation; marked: the maximum drawdown of the whole series, {}.",
                 list_text(&pr), pct(md)), body, 450.0)
}

fn info_bars(s: &Solved) -> Option<Figure> {
    let (p, b) = (s.vec("portfolio_returns")?, s.vec("benchmark_returns")?);
    let (ir, te, ma) = (s.out("information_ratio")?, s.out("tracking_error")?, s.out("mean_active_return")?);
    if p.len() != b.len() || p.len() > 60 {
        return None;
    }
    let nn = p.len();
    let act: Vec<f64> = p.iter().zip(&b).map(|(x, y)| (x - y) * 100.0).collect();
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("information ratio {}", fm(ir)));
    let lg = [it("portfolio", "axp-b", Key::Box), it("benchmark", "axp-b4", Key::Box), it("active return", "axp-b2", Key::Box),
              it("mean +/- tracking error", "axp-s3", Key::Box)];
    let top = crate::figures::legend(&mut h, &lg, 40.0) + 6.0;
    let all1: Vec<f64> = p100(&p).into_iter().chain(p100(&b)).chain([0.0]).collect();
    let mut p1 = panel(top, top + 150.0, (-0.6, nn as f64 - 0.4), range(&all1, 0.12, true), false, false, 1);
    bars(&mut p1, nn, &[(p100(&p), "axp-b"), (p100(&b), "axp-b4")], false, false);
    p1.hline(0.0, "axp-c4");
    cat_labels(&mut p1, &(1..=nn).map(|t| t.to_string()).collect::<Vec<_>>());
    p1.axes_with("", "return (%)", false, true);
    let all2: Vec<f64> = act.iter().copied().chain([0.0, (ma + te) * 100.0, (ma - te) * 100.0]).collect();
    let mut p2 = panel(top + 190.0, top + 342.0, (-0.6, nn as f64 - 0.4), range(&all2, 0.12, true), false, false, 2);
    p2.rect(-0.6, nn as f64 - 0.4, (ma - te) * 100.0, (ma + te) * 100.0, "axp-s3");
    bars(&mut p2, nn, &[(act.clone(), "axp-b2")], false, false);
    p2.line(&[(-0.6, ma * 100.0), (nn as f64 - 0.4, ma * 100.0)], "axp-c3");
    p2.label(nn as f64 - 0.4, ma * 100.0, -6.0, "end", "axp-vt", &format!("mean {}", pct(ma)));
    p2.hline(0.0, "axp-c4");
    cat_labels(&mut p2, &(1..=nn).map(|t| t.to_string()).collect::<Vec<_>>());
    p2.axes_with("period", "active return (%)", false, true);
    let body = h + &p1.s + &p2.s;
    fig(s, "bars", "Information ratio: active returns against the benchmark",
        &format!("Top: portfolio returns {} and benchmark returns {}. Bottom: their difference each period with the \
                  mean active return {} and the tracking error {} returned by the model (the band); information ratio = \
                  {}.", list_text(&p), list_text(&b), pct(ma), pct(te), fm(ir)), body, top + 396.0)
}

fn scenario_bars(s: &Solved) -> Option<Figure> {
    let (pr, r) = (s.vec("probabilities")?, s.vec("returns")?);
    let (er, sd) = (s.out("expected_return")?, s.out("std_dev")?);
    if pr.len() != r.len() || pr.len() > 30 {
        return None;
    }
    let total: f64 = pr.iter().sum();
    let all: Vec<f64> = p100(&r).into_iter().chain([0.0, (er + sd) * 100.0, (er - sd) * 100.0]).collect();
    let mut h = String::new();
    let items = [it("scenario return (width: probability)", "axp-b", Key::Box), it(format!("expected {}", pct(er)), "axp-c2", Key::Line),
                 it("+/- one standard deviation", "axp-s2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("expected {}, sd {}", pct(er), pct(sd)), &items, (0.0, total), range(&all, 0.12, true), false, false);
    pl.rect(0.0, total, (er - sd) * 100.0, (er + sd) * 100.0, "axp-s2");
    let mut x = 0.0;
    for (p, v) in pr.iter().zip(&r) {
        pl.rect(x + total * 0.005, x + p - total * 0.005, 0.0, v * 100.0, if *v >= 0.0 { "axp-b" } else { "axp-b3" });
        pl.label(x + p / 2.0, v * 100.0, if *v >= 0.0 { -6.0 } else { 15.0 }, "middle", "axp-vt", &format!("{} (p {})", pct(*v), fm(*p)));
        x += p;
    }
    pl.line(&[(0.0, er * 100.0), (total, er * 100.0)], "axp-c2");
    pl.hline(0.0, "axp-c4");
    pl.axes("cumulative probability", "return (%)");
    fig(s, "bars", "Scenario returns and their expected value",
        &format!("Returns {} in {} scenarios with probabilities {} (bar width grows with the probability): the model \
                  returns the expected return {} and the standard deviation {} (band).",
                 r.iter().map(|v| pct(*v)).collect::<Vec<_>>().join(", "), r.len(), list_text(&pr), pct(er), pct(sd)),
        finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ portfolios
fn port_point(s: &Solved, w: &[f64], er: &[f64], cov: &[Vec<f64>]) -> Option<(f64, f64)> {
    let a = s.solve_other("portfolio_return_risk", &[("weights", lst(w)), ("expected_returns", lst(er)),
                                                     ("covariance", Input::Json(serde_json::json!(cov)))])?;
    Some((k::num_of(&a, "std_dev")? * 100.0, k::num_of(&a, "expected_return")? * 100.0))
}

fn unit(nn: usize, i: usize) -> Vec<f64> {
    (0..nn).map(|j| if i == j { 1.0 } else { 0.0 }).collect()
}

fn portfolio_frontier(s: &Solved) -> Option<Figure> {
    let (w, er, cov) = (s.vec("weights")?, s.vec("expected_returns")?, s.mat("covariance")?);
    let (r, v, sd) = (s.out("expected_return")?, s.out("variance")?, s.out("std_dev")?);
    let nn = w.len();
    if nn < 2 || nn > 8 {
        return None;
    }
    let mut curves: Vec<Vec<(f64, f64)>> = vec![];
    if nn == 2 {
        curves.push(k::span(0.0, 1.0, 51, false).iter().filter_map(|a| port_point(s, &[*a, 1.0 - a], &er, &cov)).collect());
    } else {
        for i in 0..nn {
            let u = unit(nn, i);
            curves.push(k::span(0.0, 1.0, 11, false).iter().filter_map(|t| {
                let m: Vec<f64> = w.iter().zip(&u).map(|(a, b)| a * (1.0 - t) + b * t).collect();
                port_point(s, &m, &er, &cov)
            }).collect());
        }
    }
    let assets: Vec<(f64, f64)> = (0..nn).filter_map(|i| port_point(s, &unit(nn, i), &er, &cov)).collect();
    if curves.iter().all(|c| c.len() < 2) {
        return None;
    }
    let xs: Vec<f64> = curves.iter().flatten().chain(&assets).map(|p| p.0).chain([0.0, sd * 100.0]).collect();
    let ys: Vec<f64> = curves.iter().flatten().chain(&assets).map(|p| p.1).chain([r * 100.0]).collect();
    let mut h = String::new();
    let items = [it(if nn == 2 { "every mix of the two assets" } else { "mixes towards each asset" }, "axp-c", Key::Line),
                 it("each asset alone", "axp-d3", Key::Dot)];
    let mut pl = chart(&mut h, 340.0, &format!("return {}, sd {}", pct(r), pct(sd)), &items, range(&xs, 0.06, true), range(&ys, 0.1, false), false, false);
    for c in &curves {
        pl.line(c, "axp-c");
    }
    for (i, a) in assets.iter().enumerate() {
        pl.dot(a.0, a.1, "axp-d3", 5.0);
        pl.label(a.0, a.1, -8.0, "middle", "axp-lt", &format!("asset {}", i + 1));
    }
    pl.mark(sd * 100.0, r * 100.0, &format!("weights {}", list_text(&w)));
    pl.axes("standard deviation (%)", "expected return (%)");
    fig(s, "frontier", "A portfolio on the risk-return curve of its assets",
        &format!("Expected return and standard deviation of portfolios of {} assets (expected returns {}) at {} \
                  weights, each a run of the model. Marked: weights {}: return {}, standard deviation {} (variance {}).",
                 nn, list_text(&er), curves.iter().map(|c| c.len()).sum::<usize>(), list_text(&w), pct(r), pct(sd), fm(v)),
        finish(h, pl), 340.0)
}

fn tangency_frontier(s: &Solved) -> Option<Figure> {
    let (er, cov, rf) = (s.vec("expected_returns")?, s.mat("covariance")?, n(s, "risk_free")?);
    let (w, r, sd, sh) = (s.out_list("weights")?, s.out("expected_return")?, s.out("std_dev")?, s.out("sharpe")?);
    let nn = er.len();
    if nn < 2 || nn > 8 {
        return None;
    }
    let curve: Vec<(f64, f64)> = if nn == 2 {
        k::span(-0.2, 1.2, 57, false).iter().filter_map(|a| port_point(s, &[*a, 1.0 - a], &er, &cov)).collect()
    } else { vec![] };
    let assets: Vec<(f64, f64)> = (0..nn).filter_map(|i| port_point(s, &unit(nn, i), &er, &cov)).collect();
    let xs: Vec<f64> = curve.iter().chain(&assets).map(|p| p.0).chain([0.0, sd * 100.0]).collect();
    let xr = range(&xs, 0.08, true);
    let cml_end = (xr.1, rf * 100.0 + sh * xr.1);
    let ys: Vec<f64> = curve.iter().chain(&assets).map(|p| p.1).chain([rf * 100.0, r * 100.0]).collect();
    let mut h = String::new();
    let items = [it("mixes of the assets", "axp-c3", Key::Line), it("capital market line", "axp-c", Key::Line), it("each asset alone", "axp-d3", Key::Dot)];
    let mut pl = chart(&mut h, 340.0, &format!("Sharpe ratio {}", fm(sh)), &items, xr, range(&ys, 0.12, false), false, false);
    pl.line(&curve, "axp-c3");
    pl.line(&[(0.0, rf * 100.0), (sd * 100.0, r * 100.0), cml_end], "axp-c");
    for (i, a) in assets.iter().enumerate() {
        pl.dot(a.0, a.1, "axp-d3", 5.0);
        pl.label(a.0, a.1, 16.0, "middle", "axp-lt", &format!("asset {}", i + 1));
    }
    pl.dot(0.0, rf * 100.0, "axp-d2", 5.0);
    pl.label(0.0, rf * 100.0, 16.0, "start", "axp-lt", &format!(" risk-free {}", pct(rf)));
    pl.mark(sd * 100.0, r * 100.0, &format!("tangency {}", pct(r)));
    pl.axes("standard deviation (%)", "expected return (%)");
    fig(s, "frontier", "Tangency portfolio and the capital market line",
        &format!("Risk and return of mixes of the assets (expected returns {}; portfolio runs of the model) and the \
                  line from the risk-free rate {} through the tangency portfolio the model returns (weights {}; return \
                  {}, standard deviation {}, Sharpe ratio {}); the line is drawn through those two returned points.",
                 list_text(&er), pct(rf), list_text(&w), pct(r), pct(sd), fm(sh)), finish(h, pl), 340.0)
}

fn two_curve(s: &Solved, rho: Option<f64>) -> Vec<(f64, f64)> {
    k::span(0.0, 1.0, 51, false).iter().filter_map(|w| {
        let mut set = vec![("weight_1", *w)];
        if let Some(r) = rho {
            set.push(("correlation", r));
        }
        let a = again(s, &set)?;
        Some((k::num_of(&a, "std_dev")? * 100.0, k::num_of(&a, "expected_return")? * 100.0))
    }).collect()
}

fn two_asset_frontier(s: &Solved) -> Option<Figure> {
    let (w1, r1, r2, s1, s2, rho) = (n(s, "weight_1")?, n(s, "return_1")?, n(s, "return_2")?, n(s, "sd_1")?, n(s, "sd_2")?, n(s, "correlation")?);
    let (r, sd) = (s.out("expected_return")?, s.out("std_dev")?);
    let c = two_curve(s, None);
    if !ok2(&c) {
        return None;
    }
    let wm = other(s, "min_variance_two_asset", &[("sd_1", s1), ("sd_2", s2), ("correlation", rho)], "weight_1");
    let mv = wm.and_then(|w| again(s, &[("weight_1", w)]).and_then(|a| Some((k::num_of(&a, "std_dev")? * 100.0, k::num_of(&a, "expected_return")? * 100.0))));
    let xs: Vec<f64> = c.iter().map(|p| p.0).chain([0.0]).collect();
    let ys: Vec<f64> = c.iter().map(|p| p.1).collect();
    let mut h = String::new();
    let mut items = vec![it("every mix of the two assets", "axp-c", Key::Line)];
    if mv.is_some() {
        items.push(it("minimum-variance mix", "axp-d3", Key::Dot));
    }
    let mut pl = chart(&mut h, 340.0, &format!("w1 = {}: {} at sd {}", fm(w1), pct(r), pct(sd)), &items, range(&xs, 0.06, true), range(&ys, 0.12, false), false, false);
    pl.line(&c, "axp-c");
    pl.dot(s1 * 100.0, r1 * 100.0, "axp-d4", 5.0);
    pl.dot(s2 * 100.0, r2 * 100.0, "axp-d4", 5.0);
    pl.label(s1 * 100.0, r1 * 100.0, 17.0, "middle", "axp-lt", "asset 1");
    pl.label(s2 * 100.0, r2 * 100.0, -8.0, "middle", "axp-lt", "asset 2");
    if let Some((x, y)) = mv {
        pl.dot(x, y, "axp-d3", 5.5);
    }
    pl.mark(sd * 100.0, r * 100.0, &format!("w1 = {}", fm(w1)));
    pl.axes("standard deviation (%)", "expected return (%)");
    fig(s, "frontier", "Two-asset portfolios: the risk-return curve",
        &format!("Risk and return of every mix of two assets (returns {} and {}, standard deviations {} and {}, \
                  correlation {}), computed by the model at 51 weights. The mix w1 = {} gives {} at a standard deviation \
                  of {}{}.", pct(r1), pct(r2), pct(s1), pct(s2), fm(rho), fm(w1), pct(r), pct(sd),
                 match (wm, mv) {
                     (Some(w), Some((x, y))) => format!("; marked: the minimum-variance mix from the model, w1 = {}, \
                                                         standard deviation {} %, return {} %", fm(w), fm(x), fm(y)),
                     _ => String::new(),
                 }), finish(h, pl), 340.0)
}

fn two_asset_family(s: &Solved) -> Option<Figure> {
    let (r1, r2, s1, s2, rho) = (n(s, "return_1")?, n(s, "return_2")?, n(s, "sd_1")?, n(s, "sd_2")?, n(s, "correlation")?);
    let mut rhos = vec![-1.0, rho, if (rho - 0.6).abs() < 1e-9 { 0.3 } else { 0.6 }, 1.0];
    rhos.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rhos.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    let classes = ["axp-c2", "axp-c", "axp-c3", "axp-c4"];
    let curves: Vec<(f64, Vec<(f64, f64)>)> = rhos.iter().map(|r| (*r, two_curve(s, Some(*r)))).filter(|c| c.1.len() >= 2).collect();
    if curves.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.0)).chain([0.0]).collect();
    let ys: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.1)).collect();
    let mut h = String::new();
    let items: Vec<_> = curves.iter().enumerate().map(|(i, (r, _))| it(format!("correlation {}{}", fm(*r), if (*r - rho).abs() < 1e-12 { " (given)" } else { "" }),
                                                                       classes[i % 4], Key::Line)).collect();
    let mut pl = chart(&mut h, 340.0, "diversification", &items, range(&xs, 0.06, true), range(&ys, 0.12, false), false, false);
    for (i, (_, c)) in curves.iter().enumerate() {
        pl.line(c, classes[i % 4]);
    }
    pl.dot(s1 * 100.0, r1 * 100.0, "axp-d4", 5.0);
    pl.dot(s2 * 100.0, r2 * 100.0, "axp-d4", 5.0);
    pl.label(s1 * 100.0, r1 * 100.0, 17.0, "middle", "axp-lt", "asset 1");
    pl.label(s2 * 100.0, r2 * 100.0, -8.0, "middle", "axp-lt", "asset 2");
    pl.axes("standard deviation (%)", "expected return (%)");
    fig(s, "frontier-2", "Diversification: the two-asset curve for several correlations",
        &format!("All mixes of two assets (returns {} and {}, standard deviations {} and {}) for correlations {} (51 \
                  weights each, runs of the model): the lower the correlation, the more risk diversification removes; \
                  at -1 a riskless mix exists, at +1 the mixes lie on a straight line. Dots: each asset alone.",
                 pct(r1), pct(r2), pct(s1), pct(s2), curves.iter().map(|c| fm(c.0)).collect::<Vec<_>>().join(", ")),
        finish(h, pl), 340.0)
}

fn gmv_panels(s: &Solved, with_sd: bool) -> Option<(String, Vec<f64>, Vec<f64>)> {
    let (cov, w) = (s.mat("covariance")?, s.out_list("weights")?);
    let nn = cov.len();
    if nn < 2 || nn > 8 || w.len() != nn {
        return None;
    }
    let sds: Vec<f64> = if with_sd {
        (0..nn).map(|i| port_point(s, &unit(nn, i), &vec![0.0; nn], &cov).map(|p| p.0 / 100.0)).collect::<Option<Vec<_>>>()?
    } else { vec![] };
    let mut m = vec![];
    for i in 0..nn {
        let mut row = cov[i].clone();
        row.push(f64::NAN);
        row.push(w[i]);
        if with_sd {
            row.push(f64::NAN);
            row.push(sds[i]);
        }
        m.push(row);
    }
    let rows: Vec<String> = (1..=nn).map(|i| i.to_string()).collect();
    let mut cols: Vec<String> = (1..=nn).map(|i| i.to_string()).collect();
    cols.push(String::new());
    cols.push("w".into());
    if with_sd {
        cols.push(String::new());
        cols.push("sd".into());
    }
    let header = if with_sd { "covariance | weights | each asset's sd" } else { "covariance (input) | weights (returned)" };
    let body = k::heat(&m, &rows, &cols, header);
    (!body.is_empty()).then_some((body, w, sds))
}

fn gmv_heatmap(s: &Solved) -> Option<Figure> {
    let (body, w, sds) = gmv_panels(s, true)?;
    let sd = s.out("std_dev")?;
    fig(s, "heatmap", "Minimum-variance portfolio",
        &format!("The global minimum-variance portfolio of {} assets with the covariance matrix shown (left): the model \
                  returns weights {} (middle), a portfolio standard deviation of {}, below every single asset's ({}, each \
                  also computed by the model, right).", w.len(), list_text(&w), fm(sd), list_text(&sds)), body, 340.0)
}

fn gmv_matrix(s: &Solved) -> Option<Figure> {
    let (body, w, _) = gmv_panels(s, false)?;
    let (v, sd) = (s.out("variance")?, s.out("std_dev")?);
    fig(s, "matrix", "Global minimum-variance portfolio: the matrices",
        &format!("The covariance matrix (input) and the weights the model returns ({}) as heat maps, each cell shaded by \
                  its magnitude. Returned with them: variance = {}, standard deviation = {}.", list_text(&w), fm(v), fm(sd)),
        body, 340.0)
}

// ------------------------------------------------------------------------------------------------ markets and firms
/// A firm's profit-maximising output against a linear residual demand (the model's monopoly).
fn best_response(s: &Solved, a: f64, b: f64, c: f64, other_q: f64) -> Option<f64> {
    let q = other(s, "monopoly", &[("price_intercept", a - b * other_q), ("price_slope", b), ("marginal_cost", c)], "quantity")?;
    (q >= 0.0).then_some(q)
}

fn reaction(s: &Solved, stack: bool) -> Option<Figure> {
    let (a, b) = (n(s, "price_intercept")?, n(s, "price_slope")?);
    let (c1, c2) = if stack { (n(s, "cost_leader")?, n(s, "cost_follower")?) } else { (n(s, "cost_1")?, n(s, "cost_2")?) };
    let (q1, q2, p) = if stack { (s.out("q_leader")?, s.out("q_follower")?, s.out("price")?) } else { (s.out("q1")?, s.out("q2")?, s.out("price")?) };
    let (pr1, pr2) = if stack { (s.out("profit_leader")?, s.out("profit_follower")?) } else { (s.out("profit_1")?, s.out("profit_2")?) };
    if !(b > 0.0) {
        return None;
    }
    let qmax = ((a - c1.min(c2)) / b).max(q1.max(q2) * 1.2);
    let qs = k::span(0.0, qmax, 40, false);
    // firm 2's (the follower's) best response to firm 1, and firm 1's to firm 2
    let br2: Vec<(f64, f64)> = qs.iter().filter_map(|x| best_response(s, a, b, c2, *x).map(|y| (*x, y))).collect();
    let br1: Vec<(f64, f64)> = if stack { vec![] } else { qs.iter().filter_map(|y| best_response(s, a, b, c1, *y).map(|x| (x, *y))).collect() };
    if !ok2(&br2) {
        return None;
    }
    let cmp = if stack {
        let a_ = s.solve_other("cournot", &[("price_intercept", Input::Float(a)), ("price_slope", Input::Float(b)),
                                            ("cost_1", Input::Float(c1)), ("cost_2", Input::Float(c2))])?;
        (k::num_of(&a_, "q1")?, k::num_of(&a_, "q2")?)
    } else {
        let a_ = s.solve_other("stackelberg", &[("price_intercept", Input::Float(a)), ("price_slope", Input::Float(b)),
                                                ("cost_leader", Input::Float(c1)), ("cost_follower", Input::Float(c2))])?;
        (k::num_of(&a_, "q_leader")?, k::num_of(&a_, "q_follower")?)
    };
    let mut h = String::new();
    let items = if stack {
        vec![it("follower's best response", "axp-c3", Key::Line), it("simultaneous (Cournot) outcome", "axp-d4", Key::Dot)]
    } else {
        vec![it("firm 1's best response", "axp-c", Key::Line), it("firm 2's best response", "axp-c3", Key::Line),
             it("Stackelberg (firm 1 leads)", "axp-d4", Key::Dot)]
    };
    let mut pl = chart(&mut h, 340.0, &format!("price {}", fm(p)), &items, (0.0, qmax), (0.0, qmax), false, false);
    if !stack {
        pl.line(&br1, "axp-c");
    }
    pl.line(&br2, "axp-c3");
    pl.dot(cmp.0, cmp.1, "axp-d4", 5.5);
    pl.label(cmp.0, cmp.1, -9.0, "start", "axp-lt", &format!(" ({}, {})", fm(cmp.0), fm(cmp.1)));
    pl.mark(q1, q2, &format!("{} ({}, {})", if stack { "Stackelberg" } else { "Cournot" }, fm(q1), fm(q2)));
    pl.axes(if stack { "q1, output of the leader" } else { "q1, output of firm 1" }, if stack { "q2, output of the follower" } else { "q2, output of firm 2" });
    let (title, cap) = if stack {
        ("Stackelberg leader on the follower's best response", format!(
            "A leader (cost {}) and a follower (cost {}) facing P = {} - {} Q. The follower always plays its best \
             response (the line: {} runs of the model's monopoly output against the residual demand); the leader picks \
             the point on that line it likes best. The model returns q_leader = {}, q_follower = {}, price {}, profits \
             {} and {}. Dot: the simultaneous (Cournot) outcome ({}, {}), also from the model.", fm(c1), fm(c2), fm(a),
            fm(b), br2.len(), fm(q1), fm(q2), fm(p), fm(pr1), fm(pr2), fm(cmp.0), fm(cmp.1)))
    } else {
        ("Cournot equilibrium where the best responses cross", format!(
            "Two firms with inverse demand P = {} - {} (q1 + q2) and marginal costs {} and {}. Each best-response line \
             gives one firm's profit-maximising output for every output of the other (runs of the model's monopoly \
             output against the demand left over). They cross at the Cournot equilibrium the model returns, q1 = {}, \
             q2 = {}, price {}, profits {} and {}. Dot: the Stackelberg outcome with firm 1 leading ({}, {}), also \
             from the model.", fm(a), fm(b), fm(c1), fm(c2), fm(q1), fm(q2), fm(p), fm(pr1), fm(pr2), fm(cmp.0), fm(cmp.1)))
    };
    fig(s, "reaction", title, &cap, finish(h, pl), 340.0)
}

/// Demand Q = a - bP and supply Q = c + dP as (Q, P) lines over prices 0 .. pmax.
fn sd_lines(a: f64, b: f64, c: f64, d: f64, pmax: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let ps = k::span(0.0, pmax, 30, false);
    let dem: Vec<(f64, f64)> = ps.iter().map(|p| (a - b * p, *p)).filter(|q| q.0 >= 0.0).collect();
    let sup: Vec<(f64, f64)> = ps.iter().map(|p| (c + d * p, *p)).filter(|q| q.0 >= 0.0).collect();
    (dem, sup)
}

fn equilibrium(s: &Solved) -> Option<Figure> {
    let (a, b, c, d) = (n(s, "demand_intercept")?, n(s, "demand_slope")?, n(s, "supply_intercept")?, n(s, "supply_slope")?);
    let (p, q, cs, ps, ts) = (s.out("price")?, s.out("quantity")?, s.out("consumer_surplus")?, s.out("producer_surplus")?, s.out("total_surplus")?);
    if !(b > 0.0 && d > 0.0) {
        return None;
    }
    let choke = a / b;
    let pmax = choke.max(p * 1.3);
    let (dem, sup) = sd_lines(a, b, c, d, pmax);
    let mut h = String::new();
    let items = [it("demand", "axp-c", Key::Line), it("supply", "axp-c3", Key::Line), it(format!("consumer surplus {}", fm(cs)), "axp-s", Key::Box),
                 it(format!("producer surplus {}", fm(ps)), "axp-s3", Key::Box)];
    let qmax = dem.iter().chain(&sup).map(|x| x.0).fold(q, f64::max);
    let mut pl = chart(&mut h, 340.0, &format!("P* = {}, Q* = {}", fm(p), fm(q)), &items, (0.0, qmax * 1.05), (0.0, pmax * 1.05), false, false);
    pl.fill(&[(0.0, choke), (0.0, p), (q, p)], "axp-s");
    let p0 = (-c / d).max(0.0);
    if c > 0.0 {
        pl.fill(&[(0.0, 0.0), (c, 0.0), (q, p), (0.0, p)], "axp-s3");
    } else {
        pl.fill(&[(0.0, p0), (q, p), (0.0, p)], "axp-s3");
    }
    pl.line(&dem, "axp-c");
    pl.line(&sup, "axp-c3");
    pl.dash(&[(0.0, p), (q, p), (q, 0.0)], "axp-c4");
    pl.mark(q, p, &format!("Q = {}, P = {}", fm(q), fm(p)));
    pl.axes("quantity Q", "price P");
    fig(s, "equilibrium", "Supply and demand: equilibrium and surpluses",
        &format!("Linear demand Q = {} - {} P and supply Q = {} + {} P: the model returns the equilibrium P* = {}, Q* = \
                  {}, consumer surplus {} and producer surplus {} (the shaded areas; total {}).", fm(a), fm(b), fm(c),
                 fm(d), fm(p), fm(q), fm(cs), fm(ps), fm(ts)), finish(h, pl), 340.0)
}

fn demand_shift(s: &Solved) -> Option<Figure> {
    let (a, b, c, d) = (n(s, "demand_intercept")?, n(s, "demand_slope")?, n(s, "supply_intercept")?, n(s, "supply_slope")?);
    let (p, q) = (s.out("price")?, s.out("quantity")?);
    if !(b > 0.0 && d > 0.0) {
        return None;
    }
    let shifts = [a * 0.8, a, a * 1.2];
    let mut eq = vec![];
    for aa in &shifts {
        let r = again(s, &[("demand_intercept", *aa)]);
        eq.push(r.and_then(|r| Some((k::num_of(&r, "quantity")?, k::num_of(&r, "price")?))));
    }
    let pmax = shifts.iter().map(|aa| aa / b).fold(p * 1.3, f64::max);
    let classes = ["axp-c4", "axp-c", "axp-c2"];
    let mut h = String::new();
    let mut items: Vec<_> = shifts.iter().zip(classes).map(|(aa, cl)| it(format!("demand, intercept {}", fm(*aa)), cl, Key::Line)).collect();
    items.push(it("supply", "axp-c3", Key::Line));
    let mut qmax: f64 = q;
    let mut lines = vec![];
    for aa in &shifts {
        let (dem, sup) = sd_lines(*aa, b, c, d, pmax);
        qmax = dem.iter().chain(&sup).map(|x| x.0).fold(qmax, f64::max);
        lines.push((dem, sup));
    }
    let mut pl = chart(&mut h, 340.0, &format!("equilibrium ({}, {})", fm(q), fm(p)), &items, (0.0, qmax * 1.05), (0.0, pmax * 1.05), false, false);
    for (i, (dem, _)) in lines.iter().enumerate() {
        pl.line(dem, classes[i]);
    }
    pl.line(&lines[1].1, "axp-c3");
    for (i, e) in eq.iter().enumerate() {
        if let Some((x, y)) = e {
            if i == 1 {
                pl.mark(*x, *y, &format!("({}, {})", fm(*x), fm(*y)));
            } else {
                pl.dot(*x, *y, "axp-d4", 5.0);
            }
        }
    }
    pl.axes("quantity Q", "price P");
    fig(s, "shift", "A demand shift moves the equilibrium along supply",
        &format!("Supply Q = {} + {} P with demand shifted to intercepts {} (slope {}): the equilibrium the model \
                  returns for each, (Q*, P*) = {}. A rise in demand raises both price and quantity, moving along the \
                  supply line.", fm(c), fm(d), shifts.iter().map(|x| fm(*x)).collect::<Vec<_>>().join(", "), fm(b),
                 eq.iter().map(|e| e.map(|(x, y)| format!("({}, {})", fm(x), fm(y))).unwrap_or_else(|| "none".into())).collect::<Vec<_>>().join("; ")),
        finish(h, pl), 340.0)
}

fn tax_wedge(s: &Solved) -> Option<Figure> {
    let (a, b, c, d, t) = (n(s, "demand_intercept")?, n(s, "demand_slope")?, n(s, "supply_intercept")?, n(s, "supply_slope")?, n(s, "tax")?);
    let (q, pb, ps, rev, dwl, sh) = (s.out("quantity")?, s.out("buyer_price")?, s.out("seller_price")?, s.out("tax_revenue")?, s.out("deadweight_loss")?, s.out("buyer_share")?);
    if !(b > 0.0 && d > 0.0) {
        return None;
    }
    let e = s.solve_other("market_equilibrium", &[("demand_intercept", Input::Float(a)), ("demand_slope", Input::Float(b)),
                                                  ("supply_intercept", Input::Float(c)), ("supply_slope", Input::Float(d))])?;
    let (q0, p0) = (k::num_of(&e, "quantity")?, k::num_of(&e, "price")?);
    let pmax = (a / b).max(pb * 1.3);
    let (dem, sup) = sd_lines(a, b, c, d, pmax);
    let taxed: Vec<(f64, f64)> = sup.iter().map(|(x, y)| (*x, y + t)).collect();
    let qmax = dem.iter().chain(&sup).map(|x| x.0).fold(q0, f64::max);
    let mut h = String::new();
    let items = [it("demand", "axp-c", Key::Line), it("supply", "axp-c3", Key::Line), it("supply + tax", "axp-c3", Key::Dash),
                 it(format!("tax revenue {}", fm(rev)), "axp-s2", Key::Box), it(format!("deadweight loss {}", fm(dwl)), "axp-s", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("tax {}: buyers pay {}, sellers get {}", fm(t), fm(pb), fm(ps)), &items,
                       (0.0, qmax * 1.05), (0.0, (pmax + t) * 1.02), false, false);
    pl.fill(&[(0.0, ps), (q, ps), (q, pb), (0.0, pb)], "axp-s2");
    pl.fill(&[(q, ps), (q, pb), (q0, p0)], "axp-s");
    pl.line(&dem, "axp-c");
    pl.line(&sup, "axp-c3");
    pl.dash(&taxed, "axp-c3");
    pl.mark(q, pb, &format!("Q = {}, buyers {}", fm(q), fm(pb)));
    pl.dot(q, ps, "axp-d3", 5.0);
    pl.label(q, ps, 16.0, "start", "axp-lt", &format!(" sellers {}", fm(ps)));
    pl.dot(q0, p0, "axp-d4", 4.5);
    pl.axes("quantity Q", "price P");
    fig(s, "wedge", "A per-unit tax: the wedge, its revenue and the deadweight loss",
        &format!("Demand Q = {} - {} P and supply Q = {} + {} P with a tax of {} per unit (the dashed supply is shifted \
                  up by the tax). The model returns the taxed quantity {}, the buyers' price {} and the sellers' price {} \
                  (buyers bear {} of the tax), the tax revenue {} (rectangle) and the deadweight loss {} (triangle up \
                  to the untaxed equilibrium {}, {}, also from the model).", fm(a), fm(b), fm(c), fm(d), fm(t), fm(q),
                 fm(pb), fm(ps), pct(sh), fm(rev), fm(dwl), fm(q0), fm(p0)), finish(h, pl), 340.0)
}

fn monopoly_diagram(s: &Solved) -> Option<Figure> {
    let (a, b, mc) = (n(s, "price_intercept")?, n(s, "price_slope")?, n(s, "marginal_cost")?);
    let cq = n(s, "cost_quadratic").unwrap_or(0.0);
    let (q, p, pf, li, dwl) = (s.out("quantity")?, s.out("price")?, s.out("profit")?, s.out("lerner_index")?, s.out("deadweight_loss")?);
    if !(b > 0.0 && q > 0.0) {
        return None;
    }
    let mcq = |x: f64| mc + 2.0 * cq * x;
    let mcm = mcq(q);
    // the competitive quantity from the returned deadweight loss (a triangle of height P - MC at the monopoly output)
    let qc = if p > mcm { q + 2.0 * dwl / (p - mcm) } else { q };
    let pc = a - b * qc;
    let ac = p - pf / q;
    let qmax = (a / b).max(qc * 1.1);
    let dem: Vec<(f64, f64)> = vec![(0.0, a), (a / b, 0.0)];
    let mr: Vec<(f64, f64)> = vec![(0.0, a), (a / (2.0 * b), 0.0)];
    let mcl: Vec<(f64, f64)> = k::span(0.0, qmax, 20, false).iter().map(|x| (*x, mcq(*x))).collect();
    let mut h = String::new();
    let items = [it("demand", "axp-c", Key::Line), it("marginal revenue", "axp-c3", Key::Dash), it("marginal cost", "axp-c2", Key::Line),
                 it(format!("profit {}", fm(pf)), "axp-s2", Key::Box), it(format!("deadweight loss {}", fm(dwl)), "axp-s", Key::Box)];
    let ys: Vec<f64> = vec![a, 0.0, p, ac, mcm];
    let mut pl = chart(&mut h, 340.0, &format!("Q = {}, P = {}, Lerner {}", fm(q), fm(p), fm(li)), &items, (0.0, qmax), range(&ys, 0.06, true), false, false);
    pl.fill(&[(0.0, ac), (q, ac), (q, p), (0.0, p)], "axp-s2");
    pl.fill(&[(q, p), (q, mcm), (qc, pc)], "axp-s");
    pl.line(&dem, "axp-c");
    pl.dash(&mr, "axp-c3");
    pl.line(&mcl, "axp-c2");
    pl.dash(&[(q, 0.0), (q, p)], "axp-c4");
    pl.mark(q, p, &format!("Q = {}, P = {}", fm(q), fm(p)));
    pl.axes("quantity Q", "price, cost");
    fig(s, "diagram", "Monopoly: output, price, profit and deadweight loss",
        &format!("A monopolist facing demand P = {} - {} Q with marginal cost {}{} (lines from the stated inputs; \
                  marginal revenue {} - {} Q). The model returns the output {} where marginal revenue meets marginal \
                  cost, the price {}, the profit {} (rectangle), the deadweight loss {} (triangle out to the \
                  competitive quantity {}) and the Lerner index {}.", fm(a), fm(b), fm(mc),
                 if cq != 0.0 { format!(" + {} Q", fm(2.0 * cq)) } else { String::new() }, fm(a), fm(2.0 * b), fm(q), fm(p),
                 fm(pf), fm(dwl), fm(qc), fm(li)), finish(h, pl), 340.0)
}

fn budget(s: &Solved) -> Option<Figure> {
    let (al, be, inc, px, py) = (n(s, "alpha")?, n(s, "beta")?, n(s, "income")?, n(s, "price_x")?, n(s, "price_y")?);
    let (x, y, u, shx) = (s.out("x")?, s.out("y")?, s.out("utility")?, s.out("share_x")?);
    if !(px > 0.0 && py > 0.0 && inc > 0.0 && al + be > 0.0) {
        return None;
    }
    // a bundle on the indifference curve of utility `level`: the model's optimum at another price of x, with the income
    // scaled until the utility it returns is that level (checked)
    let on_curve = |level: f64, f: f64| -> Option<(f64, f64)> {
        let a1 = again(s, &[("price_x", px * f)])?;
        let u1 = k::num_of(&a1, "utility")?;
        let scale = (level / u1).powf(1.0 / (al + be));
        let a2 = again(s, &[("price_x", px * f), ("income", inc * scale)])?;
        let u2 = k::num_of(&a2, "utility")?;
        agrees(u2, level).then_some((k::num_of(&a2, "x")?, k::num_of(&a2, "y")?))
    };
    let fs = k::span(0.15, 7.0, 15, true);
    let levels = [u * 0.75, u, u * 1.25];
    let classes = ["axp-c4", "axp-c", "axp-c4"];
    let curves: Vec<Vec<(f64, f64)>> = levels.iter().map(|l| fs.iter().filter_map(|f| on_curve(*l, *f)).collect()).collect();
    if curves[1].len() < 3 {
        return None;
    }
    let (xmax, ymax) = (inc / px * 1.15, inc / py * 1.3);
    let mut h = String::new();
    let items = [it(format!("budget {} x + {} y = {}", fm(px), fm(py), fm(inc)), "axp-c3", Key::Line), it("affordable bundles", "axp-s3", Key::Box),
                 it("indifference curves", "axp-c", Key::Line)];
    let mut pl = chart(&mut h, 340.0, &format!("optimum ({}, {}), utility {}", fm(x), fm(y), fm(u)), &items, (0.0, xmax), (0.0, ymax), false, false);
    pl.fill(&[(0.0, 0.0), (inc / px, 0.0), (0.0, inc / py)], "axp-s3");
    pl.line(&[(0.0, inc / py), (inc / px, 0.0)], "axp-c3");
    for (c, cl) in curves.iter().zip(classes) {
        pl.line(c, cl);
    }
    pl.mark(x, y, &format!("optimum ({}, {})", fm(x), fm(y)));
    pl.axes("x, quantity of good x", "y, quantity of good y");
    fig(s, "budget", "Utility-maximising bundle on the budget line",
        &format!("Cobb-Douglas preferences U = x^{} y^{} with income {} and prices {} and {}: the budget line from the \
                  stated inputs and the indifference curve through the optimum the model returns, x = {}, y = {}, \
                  utility {} (each curve point is the model's optimum at another price of x with the income set so the \
                  returned utility is that level; curves for the utility and +/- 25 %); the budget share of x is {}.",
                 fm(al), fm(be), fm(inc), fm(px), fm(py), fm(x), fm(y), fm(u), fm(shx)), finish(h, pl), 340.0)
}

fn break_even_chart(s: &Solved) -> Option<Figure> {
    let (f, p, v) = (n(s, "fixed_costs")?, n(s, "price")?, n(s, "variable_cost")?);
    let (q, rev, cm, cmr) = (s.out("quantity")?, s.out("revenue")?, s.out("contribution_margin")?, s.out("contribution_margin_ratio")?);
    if !(q > 0.0) {
        return None;
    }
    let qmax = q * 2.0;
    let qb = if p > v { f / (p - v) } else { q };
    let mut h = String::new();
    let items = [it(format!("revenue {} Q", fm(p)), "axp-c2", Key::Line), it(format!("total cost {} + {} Q", fm(f), fm(v)), "axp-c", Key::Line),
                 it("loss", "axp-s", Key::Box), it("profit", "axp-s2", Key::Box)];
    let ys = [0.0, p * qmax, f + v * qmax];
    let mut pl = chart(&mut h, 340.0, &format!("break-even {} units", fm(q)), &items, (0.0, qmax), range(&ys, 0.04, true), false, false);
    pl.fill(&[(0.0, 0.0), (0.0, f), (qb, p * qb)], "axp-s");
    pl.fill(&[(qb, p * qb), (qmax, p * qmax), (qmax, f + v * qmax)], "axp-s2");
    pl.line(&[(0.0, 0.0), (qmax, p * qmax)], "axp-c2");
    pl.line(&[(0.0, f), (qmax, f + v * qmax)], "axp-c");
    pl.hline(f, "axp-c4");
    pl.label(0.0, f, -6.0, "start", "axp-lt", &format!(" fixed costs {}", fm(f)));
    pl.mark(q, rev, &format!("{} units, revenue {}", fm(q), fm(rev)));
    pl.axes("Q, units sold", "amount");
    fig(s, "chart", "Break-even chart: revenue against total cost",
        &format!("Revenue at a price of {} and total cost with fixed costs {} and a variable cost of {} per unit (lines \
                  from the stated inputs): the model returns the quantity {} units (revenue {}, contribution margin {} \
                  per unit, ratio {}).", fm(p), fm(f), fm(v), fm(q), fm(rev), fm(cm), fm(cmr)), finish(h, pl), 340.0)
}

fn lorenz(s: &Solved) -> Option<Figure> {
    let (inc, g) = (s.vec("incomes")?, s.out("gini")?);
    if inc.len() < 2 || inc.iter().any(|x| *x < 0.0) {
        return None;
    }
    let mut v = inc.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let tot: f64 = v.iter().sum();
    if !(tot > 0.0) {
        return None;
    }
    let nn = v.len() as f64;
    let mut pts = vec![(0.0, 0.0)];
    let mut acc = 0.0;
    for (i, x) in v.iter().enumerate() {
        acc += x;
        pts.push(((i + 1) as f64 / nn, acc / tot));
    }
    let mut h = String::new();
    let items = [it("Lorenz curve", "axp-c", Key::Line), it("line of equality", "axp-c4", Key::Dash), it("gap to equality", "axp-s", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("Gini = {}", fm(g)), &items, (0.0, 1.0), (0.0, 1.0), false, false);
    let mut area = pts.clone();
    area.reverse();
    let mut poly = vec![(0.0, 0.0), (1.0, 1.0)];
    poly.extend(area.into_iter().skip(1));
    pl.fill(&poly, "axp-s");
    pl.dash(&[(0.0, 0.0), (1.0, 1.0)], "axp-c4");
    pl.line(&pts, "axp-c");
    for (x, y) in &pts {
        pl.dot(*x, *y, "axp-d", 3.5);
    }
    pl.axes("share of people (poorest first)", "share of income");
    fig(s, "lorenz", "Lorenz curve and the Gini coefficient",
        &format!("The Lorenz curve of the incomes {}: the cumulative income share of the poorest k people (from the \
                  sorted incomes). The Gini coefficient the model returns, {}, measures the shaded gap between the curve \
                  and the line of equality.", list_text(&inc), fm(g)), finish(h, pl), 340.0)
}

fn hhi_bars(s: &Solved) -> Option<Figure> {
    let (sh, hhi, eqf) = (s.vec("market_shares")?, s.out("hhi")?, s.out("equivalent_firms")?);
    let cr4 = s.out("cr4");
    if sh.is_empty() || sh.len() > 40 {
        return None;
    }
    let eqs: Vec<f64> = (1..=10).map(|m| again_in(s, &[("market_shares", lst(&vec![100.0 / m as f64; m]))], "hhi").unwrap_or(f64::NAN)).collect();
    if eqs.iter().filter(|x| x.is_finite()).count() < 3 {
        return None;
    }
    let nn = sh.len();
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("HHI = {}", fm(hhi)));
    text(&mut h, 96.0, 40.0, "start", "axp-lt", "market share of each firm (%)");
    let mut p1 = panel(50.0, 190.0, (-0.6, nn as f64 - 0.4), range(&sh.iter().copied().chain([0.0]).collect::<Vec<_>>(), 0.15, true), false, false, 1);
    bars(&mut p1, nn, &[(sh.clone(), "axp-b")], false, nn <= 12);
    cat_labels(&mut p1, &(1..=nn).map(|t| t.to_string()).collect::<Vec<_>>());
    p1.axes_with("", "share (%)", false, true);
    text(&mut p1.s, 96.0, 238.0, "start", "axp-lt", "index of markets of 1 to 10 equal-sized firms");
    let all: Vec<f64> = eqs.iter().copied().filter(|x| x.is_finite()).chain([0.0, hhi]).collect();
    let mut p2 = panel(250.0, 408.0, (0.4, 10.6), range(&all, 0.08, true), false, false, 2);
    for (i, v) in eqs.iter().enumerate() {
        if v.is_finite() {
            p2.rect(i as f64 + 1.0 - 0.32, i as f64 + 1.0 + 0.32, 0.0, *v, "axp-b4");
        }
    }
    p2.hline(hhi, "axp-c");
    p2.mark(eqf, hhi, &format!("this market: {} equal firms", fm(eqf)));
    p2.axes("number of equal firms", "HHI");
    let body = h + &p1.s + &p2.s;
    fig(s, "bars", "Herfindahl-Hirschman index of a market",
        &format!("Top: the market shares {}. Bottom: the Herfindahl-Hirschman index of markets of 1 to 10 equal-sized \
                  firms (runs of the model) and this market's index, {}, which the model also expresses as {} \
                  equivalent equal firms{}.", list_text(&sh), fm(hhi), fm(eqf),
                 cr4.map(|c| format!(" (four-firm concentration {})", fm(c))).unwrap_or_default()), body, 470.0)
}

fn gdp_bars(s: &Solved) -> Option<Figure> {
    let (bp, bq) = (s.vec("base_prices")?, s.vec("base_quantities")?);
    let (nom, real, defl, infl, rg) = (s.out("nominal_gdp")?, s.out("real_gdp")?, s.out("deflator")?, s.out("inflation")?, s.out("real_growth")?);
    let base = again_in(s, &[("current_prices", lst(&bp)), ("current_quantities", lst(&bq))], "nominal_gdp")?;
    let vals = [base, real, nom];
    let mut h = String::new();
    let items = [it("base year", "axp-b4", Key::Box), it("current year, base prices (real)", "axp-b2", Key::Box), it("current year, current prices (nominal)", "axp-b", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("deflator {}", fm(defl)), &items, 3, range(&[0.0, nom, real, base], 0.12, true));
    let classes = ["axp-b4", "axp-b2", "axp-b"];
    for (i, v) in vals.iter().enumerate() {
        pl.rect(i as f64 - 0.32, i as f64 + 0.32, 0.0, *v, classes[i]);
        pl.label(i as f64, *v, -6.0, "middle", "axp-vt", &fm(*v));
    }
    cat_labels(&mut pl, &["base year".into(), "real".into(), "nominal".into()]);
    pl.axes_with("", "GDP", false, true);
    fig(s, "bars", "Nominal and real GDP and the deflator",
        &format!("GDP with base prices {}, base quantities {}, current prices {} and current quantities {}: base-year \
                  GDP {} (a run of the model with the base year as the current one), real GDP {} and nominal GDP {} as \
                  returned. Real growth {}; the deflator {} = 100 x nominal / real (inflation {}).", list_text(&bp),
                 list_text(&bq), list_text(&s.vec("current_prices")?), list_text(&s.vec("current_quantities")?), fm(base),
                 fm(real), fm(nom), pct(rg), fm(defl), pct(infl)), finish(h, pl), 340.0)
}

fn index_bars(s: &Solved) -> Option<Figure> {
    let (l, pa, f) = (s.out("laspeyres")?, s.out("paasche")?, s.out("fisher")?);
    let vals = [l, pa, f];
    let lo = (vals.iter().copied().fold(100.0, f64::min) * 0.95).floor();
    let hi = vals.iter().copied().fold(100.0, f64::max);
    let mut h = String::new();
    let items = [it("base = 100", "axp-c4", Key::Dash)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("Fisher {}", fm(f)), &items, 3, (lo, hi + (hi - lo) * 0.15));
    let classes = ["axp-b", "axp-b3", "axp-b2"];
    for (i, v) in vals.iter().enumerate() {
        pl.rect(i as f64 - 0.32, i as f64 + 0.32, lo, *v, classes[i]);
        pl.label(i as f64, *v, -6.0, "middle", "axp-vt", &fm(*v));
    }
    pl.hline(100.0, "axp-c4");
    cat_labels(&mut pl, &["Laspeyres".into(), "Paasche".into(), "Fisher".into()]);
    pl.axes_with("", "price index", false, true);
    fig(s, "bars", "Laspeyres, Paasche and Fisher price indices",
        &format!("Price indices for base prices {}, base quantities {}, current prices {} and current quantities {}, as \
                  returned by the model: Laspeyres {} (base quantities), Paasche {} (current quantities) and Fisher {}, \
                  their geometric mean. The axis starts at {}.", list_text(&s.vec("base_prices")?),
                 list_text(&s.vec("base_quantities")?), list_text(&s.vec("current_prices")?),
                 list_text(&s.vec("current_quantities")?), fm(l), fm(pa), fm(f), fm(lo)), finish(h, pl), 340.0)
}
