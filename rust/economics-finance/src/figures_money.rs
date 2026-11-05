//! Live figures of the time value of money, loans, bonds, capital budgeting and life contingencies: every value
//! drawn is an output of the model or a further solve of it (figures.rs helpers).

use crate::figures::{again, again1, again_in, agrees, bars, cat_chart, cat_labels, chart, disc, fig, finish, fm, it,
                     list_text, indexed, lst, n, other, other_in, panel, pct, range, lrange, surv, text, timing, Key};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "compound_interest" => vec![compound_growth(s)],
        "simple_interest" => vec![simple_bars(s)],
        "continuous_compounding" => vec![continuous_limit(s)],
        "cagr" => vec![cagr_growth(s)],
        "effective_annual_rate" => vec![ear_field(s), ear_stems(s)],
        "nominal_rate" => vec![nominal_stems(s)],
        "present_value_annuity" => vec![pv_annuity_bars(s)],
        "future_value_annuity" => vec![fv_annuity_bars(s)],
        "sinking_fund" => vec![sinking_bars(s)],
        "growing_annuity" => vec![growing_annuity_bars(s)],
        "growing_perpetuity" => vec![growing_perpetuity_limit(s)],
        "perpetuity" => vec![perpetuity_limit(s)],
        "loan_payment" => vec![loan_term_bars(s)],
        "loan_payment_split" => vec![loan_split_bars(s), loan_schedule(s)],
        "loan_cumulative_interest" => vec![loan_area(s)],
        "bond_price" => vec![bond_family(s), bond_timeline(s)],
        "bond_duration" => vec![duration_stems(s)],
        "bond_convexity" => vec![convexity_family(s)],
        "bond_price_change" => vec![price_change_bars(s)],
        "forward_rate" => vec![forward_rates(s)],
        "npv" => vec![npv_discounted(s)],
        "irr" => vec![irr_profile(s)],
        "crossover_rate" => vec![crossover_profile(s)],
        "payback_period" => vec![payback(s, false)],
        "discounted_payback" => vec![payback(s, true)],
        "equivalent_annual_annuity" => vec![eaa_bars(s)],
        "profitability_index" => vec![pi_bars(s)],
        "mirr" => vec![mirr_growth(s)],
        "two_stage_ddm" => vec![ddm_bars(s)],
        "wacc" => vec![wacc_bars(s)],
        "inflation_adjustment" => vec![inflation_path(s)],
        "mean_returns" => vec![mean_growth(s)],
        "term_insurance" | "whole_life_insurance" | "endowment_insurance" => vec![insurance_bars(s)],
        "life_annuity_due" | "temporary_annuity_due" => vec![annuity_due_bars(s)],
        "net_premium_whole_life" => vec![net_premium_bars(s)],
        "life_expectancy" => vec![expectancy_bars(s)],
        "survival_probability" => vec![survival_panels(s)],
        _ => vec![],
    }
}

fn pts_ok(v: &[(f64, f64)]) -> bool {
    v.len() >= 2
}

// ------------------------------------------------------------------------------------------------ interest
fn compound_growth(s: &Solved) -> Option<Figure> {
    let (p, r, t, m, amt) = (n(s, "principal")?, n(s, "rate")?, n(s, "years")?, n(s, "periods_per_year")?, s.out("amount")?);
    let tmax = if t > 30.0 { (t * 1.2).ceil() } else { 30.0 };
    let ts = k::span(0.0, tmax, 31, false);
    let set = |y: f64| [("principal", p), ("rate", r), ("years", y)];
    let simple: Vec<(f64, f64)> = ts.iter().filter_map(|y| other(s, "simple_interest", &set(*y), "amount").map(|a| (*y, a))).collect();
    let curve = |mm: f64| -> Vec<(f64, f64)> {
        ts.iter().filter_map(|y| other(s, "compound_interest", &[("principal", p), ("rate", r), ("years", *y), ("periods_per_year", mm)],
                                       "amount").map(|a| (*y, a))).collect()
    };
    let yearly = curve(1.0);
    let monthly = curve(12.0);
    let cont: Vec<(f64, f64)> = ts.iter().filter_map(|y| other(s, "continuous_compounding", &set(*y), "amount").map(|a| (*y, a))).collect();
    if !pts_ok(&yearly) {
        return None;
    }
    let all: Vec<f64> = [&simple, &yearly, &monthly, &cont].iter().flat_map(|v| v.iter().map(|q| q.1)).chain([amt]).collect();
    let (lo, hi) = range(&all, 0.05, false);
    let log = lo > 0.0 && hi / lo.max(1e-300) > 60.0;
    let yr = if log { lrange(&all, 1.15) } else { (lo, hi) };
    let mut h = String::new();
    let items = [it("simple interest", "axp-c4", Key::Line), it("compounded yearly", "axp-c3", Key::Line),
                 it("monthly", "axp-c2", Key::Line), it("continuously", "axp-c", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("amount after {} years: {}", fm(t), fm(amt)), &items, (0.0, tmax), yr, false, log);
    pl.line(&simple, "axp-c4");
    pl.line(&yearly, "axp-c3");
    pl.line(&monthly, "axp-c2");
    pl.dash(&cont, "axp-c");
    pl.mark(t, amt, &fm(amt));
    pl.axes("years", "amount");
    fig(s, "growth", "Compound against simple interest over time",
        &format!("{} at {} a year over {} years: simple interest, compounding once and 12 times a year and \
                  continuously, every point a run of the model (its simple-interest, compound-interest and \
                  continuous-compounding capabilities). Marked: {} years compounded {} time(s) a year, amount {}.",
                 fm(p), pct(r), fm(tmax), fm(t), fm(m), fm(amt)), finish(h, pl), 340.0)
}

fn simple_bars(s: &Solved) -> Option<Figure> {
    let (p, r, t, amt, int) = (n(s, "principal")?, n(s, "rate")?, n(s, "years")?, s.out("amount")?, s.out("interest")?);
    let nmax = (t.ceil().max(10.0)).min(40.0) as usize;
    let years: Vec<f64> = (1..=nmax).map(|y| y as f64).collect();
    let interest: Vec<f64> = years.iter().map(|y| again1(s, &[("years", *y)], "interest").unwrap_or(f64::NAN)).collect();
    let comp: Vec<(f64, f64)> = years.iter().filter_map(|y| other(s, "compound_interest",
        &[("principal", p), ("rate", r), ("years", *y), ("periods_per_year", 1.0)], "amount").map(|a| (*y, a))).collect();
    if interest.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let all: Vec<f64> = interest.iter().map(|i| p + i).chain(comp.iter().map(|c| c.1)).chain([0.0, amt]).collect();
    let mut h = String::new();
    let items = [it("principal", "axp-b4", Key::Box), it("simple interest", "axp-b", Key::Box),
                 it("compounded yearly", "axp-c3", Key::Line)];
    let mut pl = chart(&mut h, 340.0, &format!("{} years: interest {}, amount {}", fm(t), fm(int), fm(amt)), &items,
                       (0.4, nmax as f64 + 0.6), range(&all, 0.06, true), false, false);
    for (y, i) in years.iter().zip(&interest) {
        pl.rect(y - 0.32, y + 0.32, 0.0, p, "axp-b4");
        pl.rect(y - 0.32, y + 0.32, p, p + i, "axp-b");
    }
    pl.line(&comp, "axp-c3");
    pl.mark(t, amt, &fm(amt));
    pl.axes("year", "amount");
    fig(s, "bars", "Simple interest year by year, against compounding",
        &format!("{} at {} simple interest: principal plus the interest the model returns for each year 1 to {} \
                  (stacked), with yearly compounding (line, compound-interest runs) for comparison. Marked: {} years, \
                  amount {}, interest {}.", fm(p), pct(r), nmax, fm(t), fm(amt), fm(int)), finish(h, pl), 340.0)
}

fn continuous_limit(s: &Solved) -> Option<Figure> {
    let (p, r, t, amt) = (n(s, "principal")?, n(s, "rate")?, n(s, "years")?, s.out("amount")?);
    let ms = [1.0, 2.0, 4.0, 12.0, 52.0, 365.0];
    let v: Vec<(f64, f64)> = ms.iter().filter_map(|m| other(s, "compound_interest",
        &[("principal", p), ("rate", r), ("years", t), ("periods_per_year", *m)], "amount").map(|a| (*m, a))).collect();
    if v.len() < 3 {
        return None;
    }
    let all: Vec<f64> = v.iter().map(|q| q.1).chain([amt, p]).collect();
    let yr0 = range(&all, 0.08, false);
    let yr = (yr0.0, yr0.1 + (yr0.1 - yr0.0) * 0.2);
    let mut h = String::new();
    let items = [it("compounded m times a year", "axp-d", Key::Dot), it("continuous", "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("continuous {}", fm(amt)), &items, (0.6, 700.0), yr, true, false);
    let base = yr.0.max(p.min(yr.1));
    for (i, (m, a)) in v.iter().enumerate() {
        pl.stem(*m, base, *a, "axp-c", "axp-d");
        pl.label(*m, *a, if i % 2 == 0 { -9.0 } else { 16.0 }, "middle", "axp-vt", &fm(*a));
    }
    pl.hline(amt, "axp-c2");
    pl.axes("compounding periods per year", "amount");
    fig(s, "limit", "More frequent compounding approaches the continuous limit",
        &format!("{} at {} for {} years compounded 1, 2, 4, 12, 52 and 365 times a year (stems, each a \
                  compound-interest run of the model: {}) approach the continuously compounded amount the model \
                  returns, {} (dashed).", fm(p), pct(r), fm(t),
                 v.iter().map(|q| fm(q.1)).collect::<Vec<_>>().join(", "), fm(amt)), finish(h, pl), 340.0)
}

fn cagr_growth(s: &Solved) -> Option<Figure> {
    let (b, e, t, g) = (n(s, "begin_value")?, n(s, "end_value")?, n(s, "years")?, s.out("cagr")?);
    let tg = s.out("total_growth")?;
    let ts = k::span(0.0, t, 41, false);
    let path: Vec<(f64, f64)> = ts.iter().filter_map(|y| other(s, "compound_interest",
        &[("principal", b), ("rate", g), ("years", *y), ("periods_per_year", 1.0)], "amount").map(|a| (*y, a))).collect();
    if !pts_ok(&path) {
        return None;
    }
    let all: Vec<f64> = path.iter().map(|q| q.1).chain([b, e]).collect();
    let mut h = String::new();
    let items = [it(format!("compound growth at {} a year", pct(g)), "axp-c", Key::Line),
                 it("straight line between the values", "axp-c4", Key::Dash), it("stated values", "axp-d2", Key::Dot)];
    let mut pl = chart(&mut h, 340.0, &format!("CAGR {}", pct(g)), &items, (0.0, t), range(&all, 0.08, false), false, false);
    pl.dash(&[(0.0, b), (t, e)], "axp-c4");
    pl.line(&path, "axp-c");
    pl.dot(0.0, b, "axp-d2", 5.0);
    pl.dot(t, e, "axp-d2", 5.0);
    pl.label(0.0, b, -10.0, "start", "axp-vt", &fm(b));
    pl.mark(t, e, &fm(e));
    pl.axes("years", "value");
    fig(s, "growth", "Compound annual growth rate as a growth path",
        &format!("A value growing from {} to {} in {} years: the compound annual growth rate the model returns, {} \
                  (total growth {}), compounds {} into {} (curve: compound-interest runs at that rate); dashed: a \
                  straight line between the stated values.", fm(b), fm(e), fm(t), pct(g), pct(tg), fm(b), fm(e)),
        finish(h, pl), 340.0)
}

fn ear_field(s: &Solved) -> Option<Figure> {
    let (r, m, e) = (n(s, "rate")?, n(s, "periods_per_year")?, s.out("effective_rate")?);
    let rs = if r > 0.0 { k::span(r * 0.5, r * 1.5, 21, false) } else { k::span(0.01, 0.1, 21, false) };
    let ms: Vec<f64> = if m <= 24.0 {
        vec![1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 20.0, 24.0]
    } else {
        let mut v = vec![1.0, 2.0, 4.0, 12.0, 24.0, 52.0, 120.0, 365.0, m.round()];
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v.dedup();
        v
    };
    let mut vals = vec![];
    for rr in &rs {
        vals.push(ms.iter().map(|mm| again1(s, &[("rate", *rr), ("periods_per_year", *mm)], "effective_rate")).collect::<Vec<_>>());
    }
    let body = k::field(&rs, &ms, &vals, (r, m), "nominal annual rate", "periods per year", &format!("effective {}", pct(e)),
                        (false, m > 24.0));
    if body.is_empty() {
        return None;
    }
    fig(s, "field", "Effective annual rate over the nominal rate and the compounding frequency",
        &format!("The effective annual rate over the nominal rate from {} to {} and {} to {} compounding periods a \
                  year: each of the {} x {} cells is one run of the model, shaded by its value. Marked: the given \
                  rate {} compounded {} times a year, effective rate {}.", pct(rs[0]), pct(rs[rs.len() - 1]),
                 fm(ms[0]), fm(ms[ms.len() - 1]), rs.len(), ms.len(), pct(r), fm(m), pct(e)), body, 340.0)
}

fn ear_stems(s: &Solved) -> Option<Figure> {
    let (r, m, e) = (n(s, "rate")?, n(s, "periods_per_year")?, s.out("effective_rate")?);
    let mut ms = vec![1.0, 2.0, 4.0, 12.0, 52.0, 365.0];
    if !ms.contains(&m) {
        ms.push(m);
    }
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let v: Vec<(f64, f64)> = ms.iter().filter_map(|mm| again1(s, &[("periods_per_year", *mm)], "effective_rate").map(|x| (*mm, x * 100.0))).collect();
    let lim = other(s, "continuous_compounding", &[("principal", 1.0), ("rate", r), ("years", 1.0)], "interest");
    if v.len() < 3 {
        return None;
    }
    let all: Vec<f64> = v.iter().map(|q| q.1).chain([r * 100.0]).chain(lim.map(|l| l * 100.0)).collect();
    let yr = range(&all, 0.1, false);
    let mut h = String::new();
    let mut items = vec![it(format!("nominal {}", pct(r)), "axp-c4", Key::Dash)];
    if let Some(l) = lim {
        items.push(it(format!("continuous limit {}", pct(l)), "axp-c2", Key::Dash));
    }
    let mut pl = chart(&mut h, 340.0, &format!("m = {}: effective {}", fm(m), pct(e)), &items, (0.6, ms[ms.len() - 1] * 1.8),
                       yr, true, false);
    pl.hline(r * 100.0, "axp-c4");
    if let Some(l) = lim {
        pl.hline(l * 100.0, "axp-c2");
    }
    for (mm, x) in &v {
        pl.stem(*mm, yr.0, *x, "axp-c", "axp-d");
    }
    pl.mark(m, e * 100.0, &format!("m = {}", fm(m)));
    pl.axes("compounding periods per year", "effective annual rate (%)");
    fig(s, "stems", "Effective annual rate against compounding frequency",
        &format!("The effective annual rate of a nominal {} compounded {} times a year (each a run of the model), \
                  rising from the nominal rate towards the continuous limit{}. Marked: m = {}, {}.", pct(r),
                 ms.iter().map(|x| fm(*x)).collect::<Vec<_>>().join(", "),
                 lim.map(|l| format!(" (the model's continuous compounding of 1 for a year: {})", pct(l))).unwrap_or_default(),
                 fm(m), pct(e)), finish(h, pl), 340.0)
}

fn nominal_stems(s: &Solved) -> Option<Figure> {
    let (ef, m, nr) = (n(s, "effective_rate")?, n(s, "periods_per_year")?, s.out("nominal_rate")?);
    let mut ms = vec![1.0, 2.0, 4.0, 12.0, 52.0, 365.0];
    if !ms.contains(&m) {
        ms.push(m);
    }
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let v: Vec<(f64, f64)> = ms.iter().filter_map(|mm| again1(s, &[("periods_per_year", *mm)], "nominal_rate").map(|x| (*mm, x * 100.0))).collect();
    if v.len() < 3 {
        return None;
    }
    let all: Vec<f64> = v.iter().map(|q| q.1).chain([ef * 100.0]).collect();
    let yr = range(&all, 0.1, false);
    let mut h = String::new();
    let items = [it("nominal rate needed", "axp-d", Key::Dot), it(format!("effective {}", pct(ef)), "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("m = {}: nominal {}", fm(m), pct(nr)), &items, (0.6, ms[ms.len() - 1] * 1.8),
                       yr, true, false);
    pl.hline(ef * 100.0, "axp-c2");
    for (mm, x) in &v {
        pl.stem(*mm, yr.0, *x, "axp-c", "axp-d");
    }
    pl.mark(m, nr * 100.0, &format!("m = {}", fm(m)));
    pl.axes("compounding periods per year", "nominal annual rate (%)");
    fig(s, "stems", "Nominal rate needed for one effective rate",
        &format!("The nominal annual rate that gives an effective rate of {} when compounded {} times a year, each \
                  a run of the model: the more often interest compounds, the lower the quoted rate. Marked: m = {}, \
                  nominal {}.", pct(ef), ms.iter().map(|x| fm(*x)).collect::<Vec<_>>().join(", "), fm(m), pct(nr)),
        finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ annuities
/// A whole number of periods (at most `max`), else None.
fn whole(x: f64, max: usize) -> Option<usize> {
    let r = x.round();
    ((x - r).abs() < 1e-9 && r >= 1.0 && r <= max as f64).then_some(r as usize)
}

fn pv_annuity_bars(s: &Solved) -> Option<Figure> {
    let (pmt, r, np, pv) = (n(s, "payment")?, n(s, "rate")?, n(s, "periods")?, s.out("present_value")?);
    let nn = whole(np, 120)?;
    let due = timing(s) == "due";
    let pvs: Vec<f64> = (1..=nn).map(|t| disc(s, r, if due { t as f64 - 1.0 } else { t as f64 }).map(|d| d * pmt).unwrap_or(f64::NAN)).collect();
    let sum: f64 = pvs.iter().sum();
    if !agrees(sum, pv) {
        return None;
    }
    let mut h = String::new();
    let items = [it("payment", "axp-b4", Key::Box), it("its present value", "axp-b", Key::Box)];
    let all: Vec<f64> = pvs.iter().copied().chain([pmt, 0.0]).collect();
    let mut pl = cat_chart(&mut h, 340.0, &format!("sum of present values = {}", fm(pv)), &items, nn, range(&all, 0.1, true));
    bars(&mut pl, nn, &[(vec![pmt; nn], "axp-b4"), (pvs.clone(), "axp-b")], false, nn <= 6);
    cat_labels(&mut pl, &(1..=nn).map(|t| t.to_string()).collect::<Vec<_>>());
    pl.axes_with("payment number", "amount", false, true);
    fig(s, "bars", "Present value of an annuity, payment by payment",
        &format!("{} payments of {} at the {} of each period and the present value of each at {} (runs of the \
                  model's present value); they sum to the annuity's present value the model returns, {}.", nn, fm(pmt),
                 if due { "start" } else { "end" }, pct(r), fm(pv)), finish(h, pl), 340.0)
}

fn fv_annuity_bars(s: &Solved) -> Option<Figure> {
    let (pmt, r, np, fv) = (n(s, "payment")?, n(s, "rate")?, n(s, "periods")?, s.out("future_value")?);
    let nn = whole(np, 120)?;
    let bal: Vec<f64> = (1..=nn).map(|t| again1(s, &[("periods", t as f64)], "future_value").unwrap_or(f64::NAN)).collect();
    if bal.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let paid: Vec<f64> = (1..=nn).map(|t| pmt * t as f64).collect();
    let mut h = String::new();
    let items = [it("payments made", "axp-b4", Key::Box), it("interest earned", "axp-b", Key::Box)];
    let all: Vec<f64> = bal.iter().copied().chain(paid.iter().copied()).chain([0.0]).collect();
    let mut pl = chart(&mut h, 340.0, &format!("after {} payments: {}", nn, fm(fv)), &items, (0.4, nn as f64 + 0.6),
                       range(&all, 0.08, true), false, false);
    for t in 0..nn {
        let x = t as f64 + 1.0;
        pl.rect(x - 0.34, x + 0.34, 0.0, paid[t], "axp-b4");
        pl.rect(x - 0.34, x + 0.34, paid[t], bal[t], "axp-b");
    }
    pl.mark(nn as f64, fv, &fm(fv));
    pl.axes("period", "balance");
    fig(s, "bars", "Future value of an annuity: payments and interest",
        &format!("The balance after each of {} payments of {} at {} per period: the future value after k payments \
                  is a run of the model; it is split into the payments made (k x {}) and the interest (the rest). \
                  Marked: after {} payments, {}.", nn, fm(pmt), pct(r), fm(pmt), nn, fm(fv)), finish(h, pl), 340.0)
}

fn sinking_bars(s: &Solved) -> Option<Figure> {
    let (target, r, np, pmt) = (n(s, "future_value")?, n(s, "rate")?, n(s, "periods")?, s.out("payment")?);
    let nn = whole(np, 120)?;
    let tm = timing(s);
    let bal: Vec<f64> = (1..=nn).map(|t| other_in(s, "future_value_annuity", &[("payment", Input::Float(pmt)),
        ("rate", Input::Float(r)), ("periods", Input::Float(t as f64)), ("timing", Input::Text(tm.clone()))], "future_value")
        .unwrap_or(f64::NAN)).collect();
    if bal.iter().any(|x| !x.is_finite()) || !agrees(bal[nn - 1], target) {
        return None;
    }
    let mut h = String::new();
    let items = [it("balance after each payment", "axp-b", Key::Box), it(format!("target {}", fm(target)), "axp-c2", Key::Dash)];
    let all: Vec<f64> = bal.iter().copied().chain([0.0, target]).collect();
    let mut pl = chart(&mut h, 340.0, &format!("payment {} per period", fm(pmt)), &items, (0.4, nn as f64 + 0.6),
                       range(&all, 0.08, true), false, false);
    for (t, b) in bal.iter().enumerate() {
        let x = t as f64 + 1.0;
        pl.rect(x - 0.34, x + 0.34, 0.0, *b, "axp-b");
    }
    pl.hline(target, "axp-c2");
    pl.mark(nn as f64, target, &fm(target));
    pl.axes("period", "balance of the fund");
    fig(s, "bars", "Sinking fund reaching its target",
        &format!("A fund that must reach {} after {} periods at {}: the model returns the payment per period, {}; \
                  the balance after each payment (future-value-of-annuity runs with that payment) reaches the target \
                  exactly.", fm(target), nn, pct(r), fm(pmt)), finish(h, pl), 340.0)
}

fn growing_annuity_bars(s: &Solved) -> Option<Figure> {
    let (c, r, g, np, pv) = (n(s, "first_payment")?, n(s, "rate")?, n(s, "growth")?, n(s, "periods")?, s.out("present_value")?);
    let fv = s.out("future_value");
    let nn = whole(np, 150)?;
    let step = (nn as f64 / 30.0).ceil().max(1.0) as usize;
    let ks: Vec<usize> = (1..=nn).filter(|t| (t - 1) % step == 0 || *t == nn).collect();
    let grow: Vec<f64> = ks.iter().map(|t| again1(s, &[("periods", *t as f64)], "present_value").unwrap_or(f64::NAN)).collect();
    let level: Vec<f64> = ks.iter().map(|t| again1(s, &[("periods", *t as f64), ("growth", 0.0)], "present_value").unwrap_or(f64::NAN)).collect();
    if grow.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let mut h = String::new();
    let items = [it(format!("growing {} a period", pct(g)), "axp-b", Key::Box), it("level payments", "axp-b4", Key::Box)];
    let all: Vec<f64> = grow.iter().chain(level.iter()).copied().filter(|x| x.is_finite()).chain([0.0]).collect();
    let m = ks.len();
    let mut pl = cat_chart(&mut h, 340.0, &format!("all {} payments: {}", nn, fm(pv)), &items, m, range(&all, 0.08, true));
    bars(&mut pl, m, &[(grow.clone(), "axp-b"), (level.clone(), "axp-b4")], false, false);
    cat_labels(&mut pl, &ks.iter().map(|t| t.to_string()).collect::<Vec<_>>());
    pl.mark(m as f64 - 1.0 - 0.2, pv, &fm(pv));
    pl.axes_with("number of payments k", "present value of the first k", false, true);
    fig(s, "bars", "Growing annuity against level payments",
        &format!("Present value at {} of the first k payments of an annuity starting at {} and growing {} a period, \
                  beside level payments of {} (both runs of the model for k = 1 to {}). Marked: all {} payments, {}{}.",
                 pct(r), fm(c), pct(g), fm(c), nn, nn, fm(pv),
                 fv.map(|f| format!(" (future value {})", fm(f))).unwrap_or_default()), finish(h, pl), 340.0)
}

/// Periods N over which a ratio `q` per period falls to 1 %, for a limit chart's horizontal axis.
fn horizon(q: f64) -> f64 {
    if !(q > 0.0 && q < 1.0) {
        return 100.0;
    }
    (0.01f64.ln() / q.ln()).clamp(20.0, 500.0).ceil()
}

fn growing_perpetuity_limit(s: &Solved) -> Option<Figure> {
    let (c, r, g, pv) = (n(s, "first_payment")?, n(s, "rate")?, n(s, "growth")?, s.out("present_value")?);
    let nmax = horizon((1.0 + g) / (1.0 + r));
    let mut ns: Vec<f64> = k::span(1.0, nmax, 40, true).iter().map(|x| x.round()).collect();
    ns.dedup();
    let set = |nn: f64, gg: f64| [("first_payment", c), ("rate", r), ("growth", gg), ("periods", nn)];
    let grow: Vec<(f64, f64)> = ns.iter().filter_map(|nn| other(s, "growing_annuity", &set(*nn, g), "present_value").map(|v| (*nn, v))).collect();
    let level: Vec<(f64, f64)> = ns.iter().filter_map(|nn| other(s, "growing_annuity", &set(*nn, 0.0), "present_value").map(|v| (*nn, v))).collect();
    if !pts_ok(&grow) {
        return None;
    }
    let all: Vec<f64> = grow.iter().chain(level.iter()).map(|q| q.1).chain([0.0, pv]).collect();
    let mut h = String::new();
    let items = [it("first N growing payments", "axp-c", Key::Line), it("without growth", "axp-c4", Key::Dash),
                 it(format!("perpetuity {}", fm(pv)), "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("growing perpetuity = {}", fm(pv)), &items, (1.0, nmax), range(&all, 0.06, true),
                       true, false);
    pl.hline(pv, "axp-c2");
    pl.dash(&level, "axp-c4");
    pl.line(&grow, "axp-c");
    pl.axes("number of payments N", "present value");
    fig(s, "limit", "Growing perpetuity as the limit of growing annuities",
        &format!("Payments starting at {} and growing {} a period, discounted at {}: the present value of the first N \
                  (growing-annuity runs of the model, N from 1 to {}) approaches the growing perpetuity's value the \
                  model returns, {}; dashed: the same first payment without growth.", fm(c), pct(g), pct(r), fm(nmax),
                 fm(pv)), finish(h, pl), 340.0)
}

fn perpetuity_limit(s: &Solved) -> Option<Figure> {
    let (c, r, pv) = (n(s, "payment")?, n(s, "rate")?, s.out("present_value")?);
    let tm = timing(s);
    let nmax = horizon(1.0 / (1.0 + r));
    let mut ns: Vec<f64> = k::span(1.0, nmax, 45, false).iter().map(|x| x.round()).collect();
    ns.dedup();
    let v: Vec<(f64, f64)> = ns.iter().filter_map(|nn| other_in(s, "present_value_annuity", &[("payment", Input::Float(c)),
        ("rate", Input::Float(r)), ("periods", Input::Float(*nn)), ("timing", Input::Text(tm.clone()))], "present_value")
        .map(|x| (*nn, x))).collect();
    if !pts_ok(&v) {
        return None;
    }
    let all: Vec<f64> = v.iter().map(|q| q.1).chain([0.0, pv]).collect();
    let mut h = String::new();
    let items = [it("annuity of N payments", "axp-c", Key::Line), it(format!("perpetuity {}", fm(pv)), "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("perpetuity = {}", fm(pv)), &items, (0.0, nmax), range(&all, 0.06, true), false, false);
    pl.hline(pv, "axp-c2");
    pl.line(&v, "axp-c");
    pl.axes("number of payments N", "present value");
    fig(s, "limit", "A perpetuity as the limit of ever longer annuities",
        &format!("The present value at {} of the first N payments of {} (runs of the model's present value of an \
                  annuity, N up to {}) approaches the value of payments that never stop, which the model returns: {}.",
                 pct(r), fm(c), fm(nmax), fm(pv)), finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ loans
fn loan_term_bars(s: &Solved) -> Option<Figure> {
    let (pr, r, y, pmt, ti) = (n(s, "principal")?, n(s, "annual_rate")?, n(s, "years")?, s.out("payment")?, s.out("total_interest")?);
    let mut terms: Vec<f64> = if (10.0..=30.0).contains(&y) { vec![10.0, 15.0, 20.0, 25.0, 30.0] } else {
        [0.4, 0.6, 0.8, 1.0, 1.2].iter().map(|f| (y * f * 10.0).round() / 10.0).filter(|t| *t > 0.0).collect()
    };
    if !terms.iter().any(|t| (t - y).abs() < 1e-9) {
        terms.push(y);
    }
    terms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    terms.dedup();
    let mut pay = vec![];
    let mut int = vec![];
    for t in &terms {
        let a = again(s, &[("years", *t)])?;
        pay.push(k::num_of(&a, "payment")?);
        int.push(k::num_of(&a, "total_interest")?);
    }
    let m = terms.len();
    let tops: Vec<f64> = int.iter().map(|i| pr + i).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("principal", "axp-b4", Key::Box), it("total interest", "axp-b", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("{} years: {} a period, interest {}", fm(y), fm(pmt), fm(ti)), &items, m,
                           range(&tops, 0.14, true));
    bars(&mut pl, m, &[(vec![pr; m], "axp-b4"), (int.clone(), "axp-b")], true, false);
    for i in 0..m {
        pl.label(i as f64, pr + int[i], -6.0, "middle", "axp-vt", &format!("{}/period", fm(pay[i])));
    }
    cat_labels(&mut pl, &terms.iter().map(|t| format!("{} y", fm(*t))).collect::<Vec<_>>());
    pl.axes_with("term of the loan", "total paid", false, true);
    fig(s, "bars", "Loan term: payment against total interest",
        &format!("A loan of {} at {} repaid over {} years: the payment and the total interest returned by the model \
                  for each term (stacked on the principal). Longer terms lower the payment and raise the interest; \
                  {} years pays {} a period and {} interest in all.", fm(pr), pct(r),
                 terms.iter().map(|t| fm(*t)).collect::<Vec<_>>().join(", "), fm(y), fm(pmt), fm(ti)), finish(h, pl), 340.0)
}

fn loan_split_bars(s: &Solved) -> Option<Figure> {
    let (pr, r, y, k0) = (n(s, "principal")?, n(s, "annual_rate")?, n(s, "years")?, n(s, "payment_number")?);
    let mut rates: Vec<f64> = if r > 0.0 && r <= 0.2 { vec![0.02, 0.04, 0.06, 0.08, 0.10] } else {
        [0.4, 0.7, 1.0, 1.3, 1.6].iter().map(|f| r * f).collect()
    };
    if !rates.iter().any(|x| (x - r).abs() < 1e-12) {
        rates.push(r);
    }
    rates.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ip = vec![];
    let mut pp = vec![];
    for rr in &rates {
        let a = again(s, &[("annual_rate", *rr)])?;
        ip.push(k::num_of(&a, "interest_part")?);
        pp.push(k::num_of(&a, "principal_part")?);
    }
    let m = rates.len();
    let tops: Vec<f64> = ip.iter().zip(&pp).map(|(a, b)| a + b).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("interest", "axp-b", Key::Box), it("principal", "axp-b2", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("payment {} at {}", fm(k0), pct(r)), &items, m, range(&tops, 0.1, true));
    bars(&mut pl, m, &[(ip.clone(), "axp-b"), (pp.clone(), "axp-b2")], true, false);
    for i in 0..m {
        pl.label(i as f64, ip[i] + pp[i], -6.0, "middle", "axp-vt", &fm(ip[i] + pp[i]));
    }
    cat_labels(&mut pl, &rates.iter().map(|x| pct(*x)).collect::<Vec<_>>());
    pl.axes_with("annual rate", "payment", false, true);
    fig(s, "bars", "Interest and principal in one payment at several rates",
        &format!("How payment number {} on a loan of {} over {} years splits into interest and principal at annual \
                  rates {} (each a run of the model; the given rate is {}). The higher the rate, the larger the \
                  payment and the more of it is interest.", fm(k0), fm(pr), fm(y),
                 rates.iter().map(|x| pct(*x)).collect::<Vec<_>>().join(", "), pct(r)), finish(h, pl), 340.0)
}

fn loan_schedule(s: &Solved) -> Option<Figure> {
    let (pr, r, y, pmt) = (n(s, "principal")?, n(s, "annual_rate")?, n(s, "years")?, s.out("payment")?);
    let ppy = n(s, "payments_per_year").unwrap_or(12.0);
    let ny = y.floor() as usize;
    if ny < 1 || ny > 80 {
        return None;
    }
    let mut ip = vec![];
    let mut pp = vec![];
    for yy in 0..ny {
        let a = again(s, &[("payment_number", 1.0 + yy as f64 * ppy)])?;
        ip.push(k::num_of(&a, "interest_part")?);
        pp.push(k::num_of(&a, "principal_part")?);
    }
    let cross = (0..ny).find(|i| pp[*i] > ip[*i]);
    let tot = other(s, "loan_payment", &[("principal", pr), ("annual_rate", r), ("years", y), ("payments_per_year", ppy)], "total_interest");
    let mut h = String::new();
    let items = [it("interest", "axp-b", Key::Box), it("principal", "axp-b2", Key::Box)];
    let tops: Vec<f64> = ip.iter().zip(&pp).map(|(a, b)| a + b).chain([0.0]).collect();
    let head = match cross { Some(c) => format!("payment {}: principal larger from year {}", fm(pmt), c + 1), None => format!("payment {}", fm(pmt)) };
    let mut pl = chart(&mut h, 340.0, &head, &items, (0.4, ny as f64 + 0.6), range(&tops, 0.08, true), false, false);
    for i in 0..ny {
        let x = i as f64 + 1.0;
        let w = 0.36;
        pl.rect(x - w, x + w, 0.0, ip[i], "axp-b");
        pl.rect(x - w, x + w, ip[i], ip[i] + pp[i], "axp-b2");
    }
    if let Some(c) = cross {
        pl.vline(c as f64 + 0.5, "axp-c4");
    }
    pl.axes("year of the loan", "first payment of the year");
    fig(s, "schedule", "Loan amortisation: interest and principal in each payment",
        &format!("How the payment of {} on a loan of {} at {} over {} years splits into interest (lower part) and \
                  principal (upper part): the first payment of each year, as computed by the model. Year 1: interest \
                  {}, principal {}{}{}.", fm(pmt), fm(pr), pct(r), fm(y), fm(ip[0]), fm(pp[0]),
                 cross.map(|c| format!("; principal is the larger part from year {}", c + 1)).unwrap_or_default(),
                 tot.map(|t| format!("; total interest over the loan {}", fm(t))).unwrap_or_default()), finish(h, pl), 340.0)
}

fn loan_area(s: &Solved) -> Option<Figure> {
    let (pr, r, y, k0) = (n(s, "principal")?, n(s, "annual_rate")?, n(s, "years")?, n(s, "payments_made")?);
    let (ci, rep) = (s.out("cumulative_interest")?, s.out("principal_repaid")?);
    let ppy = n(s, "payments_per_year").unwrap_or(12.0);
    let total = (y * ppy).round();
    if !(total >= 1.0) {
        return None;
    }
    let step = (total / 60.0).ceil().max(1.0);
    let mut ks: Vec<f64> = vec![];
    let mut kk = step;
    while kk < total {
        ks.push(kk);
        kk += step;
    }
    ks.push(total);
    ks.push(k0);
    ks.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ks.dedup();
    let mut low = vec![(0.0, 0.0)];
    let mut up = vec![(0.0, 0.0)];
    for kv in &ks {
        let Some(a) = again(s, &[("payments_made", *kv)]) else { continue };
        let (Some(i), Some(p)) = (k::num_of(&a, "cumulative_interest"), k::num_of(&a, "principal_repaid")) else { continue };
        low.push((*kv, i));
        up.push((*kv, i + p));
    }
    if low.len() < 3 {
        return None;
    }
    let all: Vec<f64> = up.iter().map(|q| q.1).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("interest paid so far", "axp-s", Key::Box), it("principal repaid so far", "axp-s2", Key::Box)];
    let xmax = low.last().unwrap().0;
    let mut pl = chart(&mut h, 340.0, &format!("after {}: interest {}, principal {}", fm(k0), fm(ci), fm(rep)), &items,
                       (0.0, xmax), range(&all, 0.06, true), false, false);
    let mut a1 = low.clone();
    a1.push((xmax, 0.0));
    pl.fill(&a1, "axp-s");
    let mut a2 = up.clone();
    a2.extend(low.iter().rev().copied());
    pl.fill(&a2, "axp-s2");
    pl.line(&low, "axp-c");
    pl.line(&up, "axp-c2");
    pl.vline(k0, "axp-c4");
    pl.mark(k0, ci, &format!("interest {}", fm(ci)));
    pl.dot(k0, ci + rep, "axp-d2", 4.5);
    pl.axes("number of payments made", "paid so far");
    fig(s, "area", "Cumulative interest and principal repaid",
        &format!("What has been paid after each number of payments on a {} loan at {} over {} years, split into \
                  interest (lower) and principal (upper), both returned by the model every {} payments. Marked: after \
                  {} payments, interest {}, principal {}.", fm(pr), pct(r), fm(y), fm(step), fm(k0), fm(ci), fm(rep)),
        finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ bonds
fn bond_family(s: &Solved) -> Option<Figure> {
    let (face, cr, y, f, ytm, price) = (n(s, "face")?, n(s, "coupon_rate")?, n(s, "years")?, n(s, "frequency")?,
                                        n(s, "ytm")?, s.out("price")?);
    let ymax = (ytm * 1.6).max(0.15);
    let ys = k::span(0.005, ymax, 41, false);
    let coupons: Vec<f64> = if cr > 0.0 { vec![0.0, cr, 2.0 * cr] } else { vec![0.0, 0.05, 0.10] };
    let classes = ["axp-c3", "axp-c", "axp-c2"];
    let mut series = vec![];
    for c in &coupons {
        let pts: Vec<(f64, f64)> = ys.iter().filter_map(|yy| again1(s, &[("coupon_rate", *c), ("ytm", *yy)], "price").map(|p| (yy * 100.0, p))).collect();
        series.push(pts);
    }
    if series.iter().all(|v| v.len() < 2) {
        return None;
    }
    let all: Vec<f64> = series.iter().flatten().map(|q| q.1).chain([face, price, 0.0]).collect();
    let mut h = String::new();
    let mut items: Vec<_> = coupons.iter().zip(classes).map(|(c, cl)| it(format!("coupon {}", pct(*c)), cl, Key::Line)).collect();
    items.push(it(format!("par {}", fm(face)), "axp-c4", Key::Dash));
    let mut pl = chart(&mut h, 340.0, &format!("price {} at {}", fm(price), pct(ytm)), &items, (0.0, ymax * 100.0),
                       range(&all, 0.05, true), false, false);
    pl.hline(face, "axp-c4");
    for (v, cl) in series.iter().zip(classes) {
        pl.line(v, cl);
    }
    pl.mark(ytm * 100.0, price, &fm(price));
    pl.axes("yield to maturity (%)", "bond price");
    fig(s, "family", "Price-yield curves for three coupon rates",
        &format!("Prices of {}-year bonds (face {}, {} payment(s) a year) with coupons {} at yields from 0.5 % to {} \
                  (41 runs of the model each): a bond trades at par where its yield equals its coupon, and the \
                  zero-coupon bond is the most sensitive. Marked: yield {}, price {}.", fm(y), fm(face), fm(f),
                 coupons.iter().map(|c| pct(*c)).collect::<Vec<_>>().join(", "), pct(ymax), pct(ytm), fm(price)),
        finish(h, pl), 340.0)
}

fn bond_timeline(s: &Solved) -> Option<Figure> {
    let (face, cr, y, f, ytm, price, c) = (n(s, "face")?, n(s, "coupon_rate")?, n(s, "years")?, n(s, "frequency")?,
                                           n(s, "ytm")?, s.out("price")?, s.out("coupon")?);
    let np = whole(y * f, 120)?;
    let cy = s.out("current_yield");
    let mut flows = vec![-price];
    for t in 1..=np {
        flows.push(if t == np { c + face } else { c });
    }
    let mut h = String::new();
    let items = [it("paid", "axp-b", Key::Box), it("received", "axp-b2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("price {}, {} coupons of {}", fm(price), np, fm(c)), &items,
                       (-0.7, np as f64 + 0.7), range(&flows, 0.12, true), false, false);
    let w = (0.36f64).min(0.4);
    for (t, v) in flows.iter().enumerate() {
        pl.rect(t as f64 - w, t as f64 + w, 0.0, *v, if *v < 0.0 { "axp-b" } else { "axp-b2" });
    }
    pl.label(0.0, -price, 15.0, "start", "axp-vt", &format!("price {}", fm(price)));
    pl.label(np as f64, c + face, -6.0, "end", "axp-vt", &format!("coupon + face {}", fm(c + face)));
    if np >= 3 {
        pl.label((np as f64) / 2.0, c, -6.0, "middle", "axp-vt", &format!("coupon {}", fm(c)));
    }
    pl.hline(0.0, "axp-c4");
    pl.axes("payment number", "cash flow to the holder");
    fig(s, "timeline", "A bond's cash flows: price, coupons and face",
        &format!("The cash flows of a {}-year bond with face {} and a {} coupon paid {} time(s) a year, bought at the \
                  price the model returns for a yield of {}: {} paid, then {} coupons of {} (returned) and the face at \
                  maturity{}.", fm(y), fm(face), pct(cr), fm(f), pct(ytm), fm(price), np, fm(c),
                 cy.map(|x| format!(". Current yield {}", pct(x))).unwrap_or_default()), finish(h, pl), 340.0)
}

fn duration_stems(s: &Solved) -> Option<Figure> {
    let (face, cr, y, f, ytm) = (n(s, "face")?, n(s, "coupon_rate")?, n(s, "years")?, n(s, "frequency")?, n(s, "ytm")?);
    let (mac, md, price) = (s.out("macaulay_duration")?, s.out("modified_duration")?, s.out("price")?);
    let np = whole(y * f, 200)?;
    let c = other(s, "bond_price", &[("face", face), ("coupon_rate", cr), ("years", y), ("ytm", ytm), ("frequency", f)], "coupon")?;
    let mut pts = vec![];
    for t in 1..=np {
        let cf = if t == np { c + face } else { c };
        if cf <= 0.0 {
            continue;
        }
        pts.push((t as f64 / f, cf * disc(s, ytm / f, t as f64)?));
    }
    let sum: f64 = pts.iter().map(|q| q.1).sum();
    if pts.is_empty() || !agrees(sum, price) {
        return None;
    }
    let yr = lrange(&pts.iter().map(|q| q.1).collect::<Vec<_>>(), 2.0);
    let mut h = String::new();
    let items = [it("present value of each payment", "axp-d", Key::Dot), it(format!("Macaulay duration {} years", fm(mac)), "axp-c2", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("sum = price {}", fm(price)), &items, (0.0, y * 1.04), yr, false, true);
    for (t, v) in &pts {
        pl.stem(*t, yr.0, *v, "axp-c", "axp-d");
    }
    pl.vline(mac, "axp-c2");
    pl.note(4.0, 16.0, "start", "axp-mt", &format!("duration {}", fm(mac)));
    pl.axes("time of the payment (years)", "present value of the payment");
    fig(s, "stems", "Duration as the balance point of discounted payments",
        &format!("Each payment of a {}-year bond (coupon {} per period, face {}) at its present value at the yield {} \
                  (runs of the model's present value; they sum to the returned price {}). The Macaulay duration the \
                  model returns, {} years, is the present-value-weighted average time of the payments, the balance \
                  point of these stems (modified duration {}). Logarithmic value axis.", fm(y), fm(c), fm(face),
                 pct(ytm), fm(price), fm(mac), fm(md)), finish(h, pl), 340.0)
}

fn convexity_family(s: &Solved) -> Option<Figure> {
    let (y, ytm, cv) = (n(s, "years")?, n(s, "ytm")?, s.out("convexity")?);
    let (lo, hi) = if ytm > 0.0 { (ytm / 10.0, (ytm * 10.0).min(2.0)) } else { (0.006, 0.6) };
    let xs = k::span(lo, hi, 41, true);
    let vals = [y, (y / 2.0).max(0.5), y * 2.0];
    let classes = ["axp-c", "axp-c2", "axp-c3"];
    let mut series = vec![];
    for v in &vals {
        series.push(xs.iter().filter_map(|x| again1(s, &[("ytm", *x), ("years", *v)], "convexity").map(|c| (*x, c))).collect::<Vec<_>>());
    }
    if series[0].len() < 2 {
        return None;
    }
    let all: Vec<f64> = series.iter().flatten().map(|q| q.1).chain([cv, 0.0]).collect();
    let mut h = String::new();
    let items: Vec<_> = vals.iter().zip(classes).enumerate()
        .map(|(i, (v, cl))| it(format!("{} years{}", fm(*v), if i == 0 { " (given)" } else { "" }), cl, Key::Line)).collect();
    let mut pl = chart(&mut h, 340.0, &format!("convexity {}", fm(cv)), &items, (lo, hi), range(&all, 0.05, true), true, false);
    for (v, cl) in series.iter().zip(classes) {
        pl.line(v, cl);
    }
    pl.mark(ytm, cv, &fm(cv));
    pl.axes("yield to maturity", "convexity (years squared)");
    fig(s, "curve-2", "Convexity of a bond against its yield, for three maturities",
        &format!("Convexity against the yield to maturity from {} to {} on a logarithmic axis, one curve for each \
                  maturity: {} years; other inputs as given. 41 points per curve computed by the model; marked: yield \
                  {}, convexity {}.", fm(lo), fm(hi), vals.iter().map(|v| fm(*v)).collect::<Vec<_>>().join(", "),
                 fm(ytm), fm(cv)), finish(h, pl), 340.0)
}

fn price_change_bars(s: &Solved) -> Option<Figure> {
    let (y, ytm, dy, price) = (n(s, "years")?, n(s, "ytm")?, n(s, "yield_change")?, s.out("price")?);
    let d = if dy.abs() > 0.0 { dy.abs() } else { 0.01 };
    let moves = [-2.0 * d, -d, d, 2.0 * d];
    let mut de = vec![];
    let mut dc = vec![];
    let mut ac = vec![];
    for m in &moves {
        match again(s, &[("yield_change", *m)]) {
            Some(a) => {
                de.push(k::num_of(&a, "duration_estimate").unwrap_or(f64::NAN));
                dc.push(k::num_of(&a, "duration_convexity_estimate").unwrap_or(f64::NAN));
                ac.push(k::num_of(&a, "actual_change").unwrap_or(f64::NAN));
            }
            None => {
                de.push(f64::NAN);
                dc.push(f64::NAN);
                ac.push(f64::NAN);
            }
        }
    }
    if ac.iter().filter(|x| x.is_finite()).count() < 2 {
        return None;
    }
    let all: Vec<f64> = de.iter().chain(&dc).chain(&ac).copied().filter(|x| x.is_finite()).chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("duration estimate", "axp-b4", Key::Box), it("duration + convexity", "axp-b3", Key::Box),
                 it("actual change", "axp-b", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("price {} at {}", fm(price), pct(ytm)), &items, 4, range(&all, 0.15, true));
    bars(&mut pl, 4, &[(de.clone(), "axp-b4"), (dc.clone(), "axp-b3"), (ac.clone(), "axp-b")], false, false);
    for (i, a) in ac.iter().enumerate() {
        if a.is_finite() {
            let dyl = if *a >= 0.0 { -6.0 } else { 15.0 };
            pl.label(i as f64 + 0.27, *a, dyl, "middle", "axp-vt", &fm(*a));
        }
    }
    pl.hline(0.0, "axp-c4");
    cat_labels(&mut pl, &moves.iter().map(|m| format!("{}{}", if *m > 0.0 { "+" } else { "" }, fm(m * 100.0))).collect::<Vec<_>>());
    pl.axes_with("yield change (percentage points)", "price change", false, true);
    fig(s, "bars", "Duration and convexity estimates against the actual price change",
        &format!("Price change of a {}-year bond (yield {}, price {}) for yield moves of {} percentage points: the \
                  duration estimate, the duration-plus-convexity estimate and the actual change, all returned by the \
                  model. Duration alone overstates a fall and understates a rise; convexity closes most of the gap.",
                 fm(y), pct(ytm), fm(price), moves.iter().map(|m| fm(m * 100.0)).collect::<Vec<_>>().join(", ")),
        finish(h, pl), 340.0)
}

fn forward_rates(s: &Solved) -> Option<Figure> {
    let (r1, m1, r2, m2, fr) = (n(s, "spot_rate_1")?, n(s, "maturity_1")?, n(s, "spot_rate_2")?, n(s, "maturity_2")?,
                                s.out("forward_rate")?);
    if !(m2 > m1 && m1 > 0.0) {
        return None;
    }
    let all = [r1 * 100.0, r2 * 100.0, fr * 100.0];
    let mut h = String::new();
    let items = [it("spot rate to the first maturity", "axp-c3", Key::Line), it("spot rate to the second", "axp-c2", Key::Line),
                 it("implied forward rate", "axp-c", Key::Line)];
    let mut pl = chart(&mut h, 340.0, &format!("forward {}", pct(fr)), &items, (0.0, m2 * 1.05), range(&all, 0.25, false), false, false);
    pl.line(&[(0.0, r1 * 100.0), (m1, r1 * 100.0)], "axp-c3");
    pl.line(&[(0.0, r2 * 100.0), (m2, r2 * 100.0)], "axp-c2");
    pl.line(&[(m1, fr * 100.0), (m2, fr * 100.0)], "axp-c");
    pl.vline(m1, "axp-c4");
    pl.label(0.0, r1 * 100.0, -7.0, "start", "axp-vt", &pct(r1));
    pl.label(0.0, r2 * 100.0, if r2 > r1 { -7.0 } else { 16.0 }, "start", "axp-vt", &pct(r2));
    pl.mark((m1 + m2) / 2.0, fr * 100.0, &format!("forward {}", pct(fr)));
    pl.axes("maturity (years)", "rate (%)");
    fig(s, "rates", "Forward rate implied by two spot rates",
        &format!("Spot rates {} for {} year(s) and {} for {} years (stated): investing at the first and rolling over \
                  at the forward rate must equal investing at the second, which gives the forward rate the model \
                  returns for the period from {} to {} years, {}.", pct(r1), fm(m1), pct(r2), fm(m2), fm(m1), fm(m2),
                 pct(fr)), finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ capital budgeting
fn flows_of(s: &Solved, sym: &str) -> Option<Vec<f64>> {
    let v = s.vec(sym)?;
    (v.len() >= 2 && v.len() <= 120).then_some(v)
}

fn npv_discounted(s: &Solved) -> Option<Figure> {
    let (r, cf, npv) = (n(s, "rate")?, flows_of(s, "cash_flows")?, s.out("npv")?);
    let pv: Vec<f64> = cf.iter().enumerate().map(|(t, c)| disc(s, r, t as f64).map(|d| c * d).unwrap_or(f64::NAN)).collect();
    if !agrees(pv.iter().sum::<f64>(), npv) && (pv.iter().sum::<f64>() - npv).abs() > 1e-9 * cf.iter().map(|x| x.abs()).sum::<f64>() {
        return None;
    }
    let nn = cf.len();
    let all: Vec<f64> = cf.iter().chain(&pv).copied().chain([0.0]).collect();
    let mut h = String::new();
    let items = [it("cash flow", "axp-b4", Key::Box), it(format!("present value at {}", pct(r)), "axp-b", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("NPV = {}", fm(npv)), &items, nn, range(&all, 0.14, true));
    bars(&mut pl, nn, &[(cf.clone(), "axp-b4"), (pv.clone(), "axp-b")], false, false);
    if nn <= 12 {
        for (t, v) in pv.iter().enumerate() {
            pl.label(t as f64 + 0.2, *v, if *v >= 0.0 { -6.0 } else { 15.0 }, "middle", "axp-vt", &fm(*v));
        }
    }
    pl.hline(0.0, "axp-c4");
    cat_labels(&mut pl, &(0..nn).map(|t| t.to_string()).collect::<Vec<_>>());
    pl.axes_with("period", "amount", false, true);
    fig(s, "discounted", "Each cash flow and its present value",
        &format!("Cash flows {} (period 0 first) beside their present values at the discount rate {}, each a run of \
                  the model's present value; the net present value it returns, NPV = {}, is their sum.",
                 list_text(&cf), pct(r), fm(npv)), finish(h, pl), 340.0)
}

fn npv_curve(s: &Solved, cf: &[f64], rates: &[f64]) -> Vec<(f64, f64)> {
    rates.iter().filter_map(|r| other_in(s, "npv", &[("rate", Input::Float(*r)), ("cash_flows", lst(cf))], "npv").map(|v| (r * 100.0, v))).collect()
}

/// Cash flows as arrows on a time line in the band y0..y1 of the canvas (down: paid, up: received).
fn timeline_arrows(out: &mut String, cf: &[f64], y0: f64, y1: f64) {
    let mid = (y0 + y1) / 2.0;
    let big = cf.iter().map(|x| x.abs()).fold(0.0, f64::max).max(1e-300);
    let nn = cf.len();
    let dx = (408.0 / nn as f64).min(80.0);
    let x0 = 96.0 + (408.0 - dx * (nn as f64 - 1.0)) / 2.0;
    out.push_str(&format!("<path class=\"axp-k\" d=\"M96,{mid:.1} L504,{mid:.1}\"/>"));
    for (t, c) in cf.iter().enumerate() {
        let x = x0 + dx * t as f64;
        let len = (c.abs() / big) * (mid - y0 - 14.0);
        if len > 1.0 {
            let tip = if *c >= 0.0 { mid - len } else { mid + len };
            let (cls, hd) = if *c >= 0.0 { ("axp-c2", -1.0) } else { ("axp-c", 1.0) };
            out.push_str(&format!("<path class=\"{cls}\" d=\"M{x:.1},{mid:.1} L{x:.1},{tip:.1} M{:.1},{:.1} L{x:.1},{tip:.1} L{:.1},{:.1}\"/>",
                                  x - 4.0, tip - 6.0 * hd, x + 4.0, tip - 6.0 * hd));
        }
        if nn <= 14 {
            let ty = if *c >= 0.0 { mid - len - 6.0 } else { mid + len + 14.0 };
            text(out, x, ty.clamp(y0 + 2.0, y1 + 8.0), "middle", "axp-vt", &fm(*c));
            text(out, x, mid + if *c >= 0.0 { 15.0 } else { -6.0 }, "middle", "axp-t", &t.to_string());
        }
    }
}

fn irr_profile(s: &Solved) -> Option<Figure> {
    let (cf, irr) = (flows_of(s, "cash_flows")?, s.out("irr")?);
    let hi = (irr * 2.0).max(0.3).min(5.0);
    let lo = if irr < 0.0 { (irr * 1.5).max(-0.9) } else { 0.0 };
    let rates = k::span(lo, hi, 61, false);
    let pts = npv_curve(s, &cf, &rates);
    if !pts_ok(&pts) {
        return None;
    }
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("IRR = {}", pct(irr)));
    text(&mut h, 96.0, 40.0, "start", "axp-lt", "cash flows by period (down: paid, up: received)");
    timeline_arrows(&mut h, &cf, 48.0, 150.0);
    let all: Vec<f64> = pts.iter().map(|q| q.1).chain([0.0]).collect();
    let mut pl = panel(176.0, 408.0, (lo * 100.0, hi * 100.0), range(&all, 0.06, false), false, false, 0);
    pl.hline(0.0, "axp-c4");
    pl.line(&pts, "axp-c");
    pl.mark(irr * 100.0, 0.0, &format!("IRR {}", pct(irr)));
    pl.axes("discount rate (%)", "net present value");
    fig(s, "profile", "Internal rate of return on the NPV profile",
        &format!("Top: the cash flows {} as a timeline (arrows down: paid; up: received). Bottom: their net present \
                  value at 61 discount rates from {} to {}, each a run of the model's NPV. The NPV is zero at the \
                  internal rate of return the model returns; marked: IRR = {}.", list_text(&cf), pct(lo), pct(hi), pct(irr)),
        finish(h, pl), 470.0)
}

fn crossover_profile(s: &Solved) -> Option<Figure> {
    let (a, b, cr, nc) = (flows_of(s, "cash_flows_a")?, flows_of(s, "cash_flows_b")?, s.out("crossover_rate")?, s.out("npv_at_crossover")?);
    let hi = (cr * 1.8).max(0.3).min(5.0);
    let lo = if cr < 0.0 { (cr * 1.5).max(-0.9) } else { 0.0 };
    let rates = k::span(lo, hi, 61, false);
    let pa = npv_curve(s, &a, &rates);
    let pb = npv_curve(s, &b, &rates);
    if !pts_ok(&pa) || !pts_ok(&pb) {
        return None;
    }
    let ia = other_in(s, "irr", &[("cash_flows", lst(&a))], "irr");
    let ib = other_in(s, "irr", &[("cash_flows", lst(&b))], "irr");
    let all: Vec<f64> = pa.iter().chain(&pb).map(|q| q.1).chain([0.0, nc]).collect();
    let mut h = String::new();
    let items = [it(format!("project A: {}", list_text(&a)), "axp-c", Key::Line), it(format!("project B: {}", list_text(&b)), "axp-c3", Key::Line)];
    let mut pl = chart(&mut h, 340.0, &format!("crossover {}", pct(cr)), &items, (lo * 100.0, hi * 100.0), range(&all, 0.06, false), false, false);
    pl.hline(0.0, "axp-c4");
    pl.line(&pa, "axp-c");
    pl.line(&pb, "axp-c3");
    if let Some(x) = ia {
        pl.dot(x * 100.0, 0.0, "axp-d", 4.5);
    }
    if let Some(x) = ib {
        pl.dot(x * 100.0, 0.0, "axp-d3", 4.5);
    }
    pl.mark(cr * 100.0, nc, &format!("crossover {}", pct(cr)));
    pl.axes("discount rate (%)", "net present value");
    fig(s, "profile", "Two NPV profiles and their crossover rate",
        &format!("NPV profiles of two projects (A: {}; B: {}) at 61 discount rates, each a run of the model's NPV. \
                  Below the crossover rate one project is worth more, above it the other; marked: the crossover rate \
                  the model returns, {}, where both NPVs equal {}.{}", list_text(&a), list_text(&b), pct(cr), fm(nc),
                 match (ia, ib) {
                     (Some(x), Some(y)) => format!(" Dots on the axis: each project's IRR from the model, {} and {}.", pct(x), pct(y)),
                     _ => String::new(),
                 }), finish(h, pl), 340.0)
}

fn payback(s: &Solved, discounted: bool) -> Option<Figure> {
    let cf = flows_of(s, "cash_flows")?;
    let pb = s.out(if discounted { "discounted_payback" } else { "payback" })?;
    let r = if discounted { n(s, "rate")? } else { 0.0 };
    let nn = cf.len();
    let mut cum = vec![];
    let mut acc = 0.0;
    for c in &cf {
        acc += c;
        cum.push(acc);
    }
    let mut dcum = vec![];
    if discounted {
        let mut acc = 0.0;
        for (t, c) in cf.iter().enumerate() {
            acc += c * disc(s, r, t as f64)?;
            dcum.push(acc);
        }
    }
    let all: Vec<f64> = cf.iter().chain(&cum).chain(&dcum).copied().chain([0.0]).collect();
    let mut h = String::new();
    let items = if discounted {
        vec![it("cumulative cash flow", "axp-c4", Key::Dash), it(format!("cumulative present value at {}", pct(r)), "axp-c", Key::Line)]
    } else {
        vec![it("cash flow", "axp-b4", Key::Box), it("cumulative cash flow", "axp-c", Key::Line)]
    };
    let mut pl = chart(&mut h, 340.0, &format!("{}payback {} periods", if discounted { "discounted " } else { "" }, fm(pb)),
                       &items, (-0.5, nn as f64 - 0.5), range(&all, 0.08, false), false, false);
    pl.hline(0.0, "axp-c4");
    if discounted {
        pl.dash(&indexed(&cum, 0.0), "axp-c4");
        let d = indexed(&dcum, 0.0);
        pl.line(&d, "axp-c");
        for (x, y) in &d {
            pl.dot(*x, *y, "axp-d", 3.5);
        }
    } else {
        for (t, c) in cf.iter().enumerate() {
            pl.rect(t as f64 - 0.3, t as f64 + 0.3, 0.0, *c, "axp-b4");
        }
        let d = indexed(&cum, 0.0);
        pl.line(&d, "axp-c");
        for (x, y) in &d {
            pl.dot(*x, *y, "axp-d", 3.5);
        }
    }
    pl.mark(pb, 0.0, &format!("payback {}", fm(pb)));
    pl.axes("period", "cumulative amount");
    let (kind, title) = ("cumulative", if discounted { "Discounted payback: cumulative present value crossing zero" } else {
        "Payback: the cumulative cash flow crossing zero"
    });
    let cap = if discounted {
        format!("Cash flows {}: their running sum (dashed) and the running sum of their present values at {} (each \
                 present value a run of the model; the sums are added here). Discounting delays recovery; marked: the \
                 discounted payback period the model returns, {} periods.", list_text(&cf), pct(r), fm(pb))
    } else {
        format!("Cash flows {} as bars and their running sum (the cumulative cash flow) as a line: the investment is \
                 recovered where the line crosses zero. Marked: the payback period the model returns, {} periods \
                 (flows assumed spread evenly within a period).", list_text(&cf), fm(pb))
    };
    fig(s, kind, title, &cap, finish(h, pl), 340.0)
}

fn eaa_bars(s: &Solved) -> Option<Figure> {
    let (r, cf, eaa, npv) = (n(s, "rate")?, flows_of(s, "cash_flows")?, s.out("eaa")?, s.out("npv")?);
    let nn = cf.len();
    let level: Vec<f64> = (0..nn).map(|t| if t == 0 { f64::NAN } else { eaa }).collect();
    let all: Vec<f64> = cf.iter().copied().chain([eaa, 0.0]).collect();
    let mut h = String::new();
    let items = [it("the project's cash flows", "axp-b4", Key::Box), it(format!("equivalent level amount {}", fm(eaa)), "axp-b", Key::Box)];
    let mut pl = cat_chart(&mut h, 340.0, &format!("same NPV {} at {}", fm(npv), pct(r)), &items, nn, range(&all, 0.12, true));
    bars(&mut pl, nn, &[(cf.clone(), "axp-b4"), (level, "axp-b")], false, nn <= 6);
    pl.hline(0.0, "axp-c4");
    cat_labels(&mut pl, &(0..nn).map(|t| t.to_string()).collect::<Vec<_>>());
    pl.axes_with("period", "amount", false, true);
    fig(s, "bars", "Equivalent annual annuity of a project",
        &format!("The cash flows {} and the level amount per period with the same net present value at {}: the model \
                  returns NPV = {} and the equivalent annual annuity {} per period over {} periods.", list_text(&cf),
                 pct(r), fm(npv), fm(eaa), nn - 1), finish(h, pl), 340.0)
}

fn pi_bars(s: &Solved) -> Option<Figure> {
    let (r, cf, pi, pvf, npv) = (n(s, "rate")?, flows_of(s, "cash_flows")?, s.out("profitability_index")?, s.out("pv_future")?, s.out("npv")?);
    let mut pvs = vec![];
    for (t, c) in cf.iter().enumerate().skip(1) {
        pvs.push(c * disc(s, r, t as f64)?);
    }
    if !agrees(pvs.iter().sum(), pvf) {
        return None;
    }
    let outlay = -cf[0];
    let all: Vec<f64> = vec![outlay, pvf, 0.0, pvs.iter().filter(|x| **x > 0.0).sum()];
    let mut h = String::new();
    let items = [it("initial outlay", "axp-b4", Key::Box), it("present value of each later flow", "axp-b", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("PI = {}, NPV = {}", fm(pi), fm(npv)), &items, (-0.7, 1.7), range(&all, 0.12, true), false, false);
    pl.rect(-0.3, 0.3, 0.0, outlay, "axp-b4");
    pl.label(0.0, outlay, -6.0, "middle", "axp-vt", &fm(outlay));
    let classes = ["axp-b", "axp-b2", "axp-b3"];
    let mut base = 0.0;
    for (i, v) in pvs.iter().enumerate() {
        pl.rect(0.7, 1.3, base, base + v, classes[i % 3]);
        if pvs.len() <= 8 && v.abs() > 0.0 {
            pl.label(1.35, base + v / 2.0, 5.0, "start", "axp-lt", &format!("t={}: {}", i + 1, fm(*v)));
        }
        base += v;
    }
    pl.label(1.0, pvf, -6.0, "middle", "axp-vt", &fm(pvf));
    let lab = vec!["outlay".to_string(), "PV of later flows".to_string()];
    for (i, l) in lab.iter().enumerate() {
        let x = pl.x(i as f64);
        let y = pl.y1 + 19.0;
        text(&mut pl.s, x, y, "middle", "axp-t", l);
    }
    pl.axes_with("", "amount", false, true);
    fig(s, "bars", "Profitability index: what the flows are worth against the outlay",
        &format!("The initial outlay {} beside the present values at {} of the later flows {} (stacked, each a run of \
                  the model; they sum to the returned {}). Profitability index = PV / outlay = {}, NPV = {}, as returned.",
                 fm(outlay), pct(r), list_text(&cf[1..]), fm(pvf), fm(pi), fm(npv)), finish(h, pl), 340.0)
}

fn mirr_growth(s: &Solved) -> Option<Figure> {
    let (cf, rr, mirr, tv, pvo) = (flows_of(s, "cash_flows")?, n(s, "reinvest_rate")?, s.out("mirr")?, s.out("terminal_value")?, s.out("pv_outflows")?);
    let nn = cf.len() - 1;
    let mut parts = vec![];
    for (t, c) in cf.iter().enumerate() {
        if *c > 0.0 {
            let v = if t == nn { *c } else {
                other(s, "compound_interest", &[("principal", *c), ("rate", rr), ("years", (nn - t) as f64), ("periods_per_year", 1.0)], "amount")?
            };
            parts.push((t, v));
        }
    }
    if parts.is_empty() || !agrees(parts.iter().map(|p| p.1).sum(), tv) {
        return None;
    }
    let ts = k::span(0.0, nn as f64, 41, false);
    let curve: Vec<(f64, f64)> = ts.iter().filter_map(|t| other(s, "compound_interest",
        &[("principal", pvo), ("rate", mirr), ("years", *t), ("periods_per_year", 1.0)], "amount").map(|a| (*t, a))).collect();
    if !pts_ok(&curve) {
        return None;
    }
    let all: Vec<f64> = curve.iter().map(|q| q.1).chain([0.0, tv]).collect();
    let mut h = String::new();
    let items = [it(format!("outflows {} grown at the MIRR", fm(pvo)), "axp-c", Key::Line),
                 it(format!("inflows reinvested at {} to period {}", pct(rr), nn), "axp-b2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("MIRR = {}", pct(mirr)), &items, (-0.2, nn as f64 + 0.8), range(&all, 0.08, true), false, false);
    let classes = ["axp-b2", "axp-b3", "axp-b4"];
    let mut base = 0.0;
    let x = nn as f64 + 0.35;
    for (i, (t, v)) in parts.iter().enumerate() {
        pl.rect(x - 0.25, x + 0.25, base, base + v, classes[i % 3]);
        if parts.len() <= 6 {
            pl.label(x - 0.3, base + v / 2.0, 5.0, "end", "axp-lt", &format!("from t={}: {}", t, fm(*v)));
        }
        base += v;
    }
    pl.line(&curve, "axp-c");
    pl.mark(nn as f64, tv, &format!("terminal {}", fm(tv)));
    pl.axes("period", "value");
    fig(s, "growth", "MIRR: outflows grown to the terminal value of reinvested inflows",
        &format!("Modified IRR of the flows {}: each inflow is carried forward to period {} at the reinvestment rate \
                  {} (stacked bar, each a compound-interest run of the model; they sum to the returned terminal value \
                  {}), and the present value of the outflows {} grows at the returned MIRR = {} to exactly that \
                  terminal value (the curve, compound-interest runs).", list_text(&cf), nn, pct(rr), fm(tv), fm(pvo),
                 pct(mirr)), finish(h, pl), 340.0)
}

fn ddm_bars(s: &Solved) -> Option<Figure> {
    let (d0, g1, ny, g2, r) = (n(s, "dividend")?, n(s, "high_growth")?, n(s, "high_growth_years")?, n(s, "stable_growth")?, n(s, "required_return")?);
    let (p, tv, pvt, pvd) = (s.out("price")?, s.out("terminal_value")?, s.out("pv_terminal")?, s.out("pv_high_growth_dividends")?);
    let all = [p, tv, pvt, pvd, 0.0];
    let mut h = String::new();
    let items = [it("present value of the high-growth dividends", "axp-b", Key::Box), it("present value of the terminal value", "axp-b2", Key::Box),
                 it(format!("terminal value in year {}", fm(ny)), "axp-b4", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("price {}", fm(p)), &items, (-0.6, 3.6), range(&all, 0.12, true), false, false);
    pl.rect(-0.3, 0.3, 0.0, pvd, "axp-b");
    pl.rect(0.7, 1.3, 0.0, pvt, "axp-b2");
    pl.rect(1.7, 2.3, 0.0, pvd, "axp-b");
    pl.rect(1.7, 2.3, pvd, pvd + pvt, "axp-b2");
    pl.rect(2.7, 3.3, 0.0, tv, "axp-b4");
    for (x, v) in [(0.0, pvd), (1.0, pvt), (2.0, p), (3.0, tv)] {
        pl.label(x, v, -6.0, "middle", "axp-vt", &fm(v));
    }
    let labs = ["dividends (PV)", "terminal (PV)", "price", "terminal value"];
    for (i, l) in labs.iter().enumerate() {
        let x = pl.x(i as f64);
        let y = pl.y1 + 19.0;
        text(&mut pl.s, x, y, "middle", "axp-lt", l);
    }
    pl.axes_with("", "value per share", false, true);
    fig(s, "bars", "Two-stage dividend discount model: where the value comes from",
        &format!("A share paying {} now, growing {} for {} years and {} after, at a required return of {}: the model \
                  returns the terminal value in year {}, {}, its present value {}, the present value of the \
                  high-growth dividends {} and their sum, the price {}.", fm(d0), pct(g1), fm(ny), pct(g2), pct(r),
                 fm(ny), fm(tv), fm(pvt), fm(pvd), fm(p)), finish(h, pl), 340.0)
}

fn wacc_bars(s: &Solved) -> Option<Figure> {
    let (e, d, ke, kd, tx) = (n(s, "equity_value")?, n(s, "debt_value")?, n(s, "cost_of_equity")?, n(s, "cost_of_debt")?, n(s, "tax_rate")?);
    let (w, akd, we, wd) = (s.out("wacc")?, s.out("after_tax_cost_of_debt")?, s.out("equity_weight")?, s.out("debt_weight")?);
    let pe = we * ke * 100.0;
    let pd = wd * akd * 100.0;
    if !agrees(pe + pd, w * 100.0) && (pe + pd - w * 100.0).abs() > 1e-9 {
        return None;
    }
    let all = [ke * 100.0, kd * 100.0, w * 100.0, 0.0];
    let mut h = String::new();
    let items = [it("cost of the source", "axp-b4", Key::Box), it("equity weight x cost", "axp-b", Key::Box),
                 it("debt weight x after-tax cost", "axp-b2", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("WACC {}", pct(w)), &items, (-0.6, 2.6), range(&all, 0.15, true), false, false);
    pl.rect(-0.35, -0.02, 0.0, ke * 100.0, "axp-b4");
    pl.rect(0.02, 0.35, 0.0, pe, "axp-b");
    pl.rect(0.65, 0.98, 0.0, akd * 100.0, "axp-b4");
    pl.rect(1.02, 1.35, 0.0, pd, "axp-b2");
    pl.rect(1.7, 2.3, 0.0, pe, "axp-b");
    pl.rect(1.7, 2.3, pe, pe + pd, "axp-b2");
    pl.label(-0.18, ke * 100.0, -6.0, "middle", "axp-vt", &pct(ke));
    pl.label(0.82, akd * 100.0, -6.0, "middle", "axp-vt", &pct(akd));
    pl.label(0.18, pe, -6.0, "middle", "axp-vt", &fm(pe));
    pl.label(1.18, pd, -6.0, "middle", "axp-vt", &fm(pd));
    pl.label(2.0, w * 100.0, -6.0, "middle", "axp-vt", &pct(w));
    let labs = [format!("equity, weight {}", fm(we)), format!("debt, weight {}", fm(wd)), "WACC".to_string()];
    for (i, l) in labs.iter().enumerate() {
        let x = pl.x(i as f64);
        let y = pl.y1 + 19.0;
        text(&mut pl.s, x, y, "middle", "axp-lt", l);
    }
    pl.axes_with("", "rate (%)", false, true);
    fig(s, "bars", "Weighted average cost of capital from its parts",
        &format!("Equity {} at {} and debt {} at {} before tax (tax rate {}): the model returns the after-tax cost of \
                  debt {}, the weights {} and {} and the weighted average cost of capital {}, drawn as the sum of each \
                  weight x cost (right bar).", fm(e), pct(ke), fm(d), pct(kd), pct(tx), pct(akd), fm(we), fm(wd), pct(w)),
        finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ returns, prices
fn inflation_path(s: &Solved) -> Option<Figure> {
    let (pl_, amt) = (s.vec("price_levels")?, n(s, "amount")?);
    let (ti, ai, rv) = (s.out("total_inflation")?, s.out("average_inflation")?, s.out("real_value")?);
    if pl_.len() < 2 || pl_.len() > 120 {
        return None;
    }
    let mut pts = vec![(0.0, amt)];
    for kk in 1..pl_.len() {
        let v = again_in(s, &[("price_levels", lst(&pl_[..=kk]))], "real_value")?;
        pts.push((kk as f64, v));
    }
    let all: Vec<f64> = pts.iter().map(|q| q.1).collect();
    let mut h = String::new();
    let items = [it(format!("real value of {}", fm(amt)), "axp-c", Key::Line)];
    let mut pl = chart(&mut h, 340.0, &format!("total inflation {}", pct(ti)), &items, (-0.2, pl_.len() as f64 - 0.8),
                       range(&all, 0.15, false), false, false);
    pl.hline(amt, "axp-c4");
    pl.line(&pts, "axp-c");
    for (i, (x, y)) in pts.iter().enumerate() {
        pl.dot(*x, *y, "axp-d", 4.0);
        if pl_.len() <= 10 {
            pl.label(*x, *y, 18.0, "middle", "axp-lt", &format!("index {}", fm(pl_[i])));
        }
    }
    pl.mark((pl_.len() - 1) as f64, rv, &fm(rv));
    pl.axes("period", "purchasing power");
    fig(s, "path", "Purchasing power of an amount as prices rise",
        &format!("The purchasing power of {} as the price index moves through {}: the real value after each period is \
                  a run of the model on the price levels up to that period. Over the whole span: total inflation {}, \
                  average {} a period, real value {}.", fm(amt), list_text(&pl_), pct(ti), pct(ai), fm(rv)),
        finish(h, pl), 340.0)
}

fn mean_growth(s: &Solved) -> Option<Figure> {
    let (rs, am, gm, cum) = (s.vec("returns")?, s.out("arithmetic_mean")?, s.out("geometric_mean")?, s.out("cumulative_return")?);
    if rs.is_empty() || rs.len() > 120 {
        return None;
    }
    let nn = rs.len();
    let mut path = vec![(0.0, 1.0)];
    for kk in 1..=nn {
        let c = if kk == nn { cum } else { again_in(s, &[("returns", lst(&rs[..kk]))], "cumulative_return")? };
        path.push((kk as f64, 1.0 + c));
    }
    let grow = |r: f64| -> Vec<(f64, f64)> {
        (0..=nn).filter_map(|t| other(s, "compound_interest", &[("principal", 1.0), ("rate", r), ("years", t as f64), ("periods_per_year", 1.0)], "amount")
            .map(|a| (t as f64, a))).collect()
    };
    let (g, a) = (grow(gm), grow(am));
    let all: Vec<f64> = path.iter().chain(&g).chain(&a).map(|q| q.1).collect();
    let mut h = String::new();
    let items = [it("value of 1 invested", "axp-c", Key::Line), it(format!("geometric mean {}", pct(gm)), "axp-c2", Key::Dash),
                 it(format!("arithmetic mean {}", pct(am)), "axp-c3", Key::Dash)];
    let mut pl = chart(&mut h, 340.0, &format!("cumulative return {}", pct(cum)), &items, (0.0, nn as f64), range(&all, 0.1, false), false, false);
    pl.dash(&a, "axp-c3");
    pl.dash(&g, "axp-c2");
    pl.line(&path, "axp-c");
    for (x, y) in &path {
        pl.dot(*x, *y, "axp-d", 3.5);
    }
    pl.mark(nn as f64, 1.0 + cum, &fm(1.0 + cum));
    pl.axes("period", "value");
    fig(s, "growth", "Arithmetic against geometric mean return",
        &format!("1 invested through returns {}: the value after each period is 1 plus the cumulative return the model \
                  returns for the returns so far. Growth at the returned geometric mean {} ends at the same value; \
                  growth at the arithmetic mean {} overstates it (both compound-interest runs).", list_text(&rs),
                 pct(gm), pct(am)), finish(h, pl), 340.0)
}

// ------------------------------------------------------------------------------------------------ life contingencies
fn mort(s: &Solved) -> Option<Vec<f64>> {
    let q = s.vec("mortality_rates")?;
    (!q.is_empty() && q.len() <= 60).then_some(q)
}

fn insurance_bars(s: &Solved) -> Option<Figure> {
    let (q, i, b) = (mort(s)?, n(s, "interest_rate")?, n(s, "benefit")?);
    let endow = s.key == "endowment_insurance";
    let term = match n(s, "term") { Some(t) => whole(t, q.len())?, None => q.len() };
    let mut sv = vec![1.0];
    for t in 1..=term {
        sv.push(surv(s, &q, t)?);
    }
    let mut death = vec![];
    for t in 1..=term {
        death.push(b * (sv[t - 1] - sv[t]) * disc(s, i, t as f64)?);
    }
    let dsum: f64 = death.iter().sum();
    let (total, pe) = if endow {
        let (tp, pe, apv) = (s.out("term_part")?, s.out("pure_endowment")?, s.out("apv")?);
        if !agrees(dsum, tp) {
            return None;
        }
        (apv, Some(pe))
    } else {
        let apv = s.out("apv")?;
        if !agrees(dsum, apv) {
            return None;
        }
        (apv, None)
    };
    let mut h = String::new();
    let mut items = vec![it(format!("paid on death (sum {})", fm(dsum)), "axp-b", Key::Box)];
    if let Some(p) = pe {
        items.push(it(format!("paid on survival to year {} ({})", term, fm(p)), "axp-b2", Key::Box));
    }
    let tops: Vec<f64> = death.iter().copied().chain([0.0, death[term - 1] + pe.unwrap_or(0.0)]).collect();
    let mut pl = cat_chart(&mut h, 340.0, &format!("total {}", fm(total)), &items, term, range(&tops, 0.12, true));
    bars(&mut pl, term, &[(death.clone(), "axp-b")], false, term <= 10 && pe.is_none());
    if pe.is_some() && term <= 10 {
        for (t, v) in death.iter().enumerate().take(term - 1) {
            pl.label(t as f64, *v, -6.0, "middle", "axp-vt", &fm(*v));
        }
    }
    if let Some(p) = pe {
        pl.rect(term as f64 - 1.0 - 0.32, term as f64 - 1.0 + 0.32, death[term - 1], death[term - 1] + p, "axp-b2");
        pl.label(term as f64 - 1.0, death[term - 1] + p, -6.0, "middle", "axp-vt", &format!("total {}", fm(total)));
    }
    cat_labels(&mut pl, &(1..=term).map(|t| t.to_string()).collect::<Vec<_>>());
    pl.axes_with("year of the payment", "expected present value", false, true);
    let qtext = list_text(&q);
    let (title, cap) = match s.key {
        "endowment_insurance" => ("Endowment insurance: death benefits and the survival payment", format!(
            "An endowment of {} paid at the end of the year of death within {} years, or at year {} on survival (death \
             probabilities {}, interest {}). The death bars are benefit x death probability that year (differences of \
             the model's survival probabilities) x discount factor and sum to the returned term part {}; stacked on the \
             last year: the returned pure endowment {}. Total {}, as returned.", fm(b), term, term, qtext, pct(i),
            fm(dsum), fm(pe.unwrap_or(0.0)), fm(total))),
        "term_insurance" => ("Term insurance: expected present value year by year", format!(
            "A benefit of {} paid at the end of the year of death within {} years, for death probabilities {} and \
             interest {}. Each bar is the amount x the probability of payment that year (from the model's survival \
             probabilities) x the discount factor (from the model's present value); the bars sum to the returned \
             actuarial present value {}.", fm(b), term, qtext, pct(i), fm(total))),
        _ => ("Whole life insurance: expected present value year by year", format!(
            "A benefit of {} paid at the end of the year of death, for death probabilities {} and interest {}. Each bar \
             is the amount x the probability of payment that year (from the model's survival probabilities) x the \
             discount factor (from the model's present value); the bars sum to the returned actuarial present value \
             {}.", fm(b), qtext, pct(i), fm(total))),
    };
    fig(s, "bars", title, &cap, finish(h, pl), 340.0)
}

fn annuity_due_bars(s: &Solved) -> Option<Figure> {
    let (q, i, pmt, apv) = (mort(s)?, n(s, "interest_rate")?, n(s, "payment")?, s.out("apv")?);
    let term = match n(s, "term") { Some(t) => whole(t, q.len())?, None => q.len() };
    let mut v = vec![];
    for t in 0..term {
        v.push(pmt * surv(s, &q, t)? * disc(s, i, t as f64)?);
    }
    if !agrees(v.iter().sum(), apv) {
        return None;
    }
    let mut h = String::new();
    let items = [it("expected present value of the payment", "axp-b", Key::Box)];
    let all: Vec<f64> = v.iter().copied().chain([0.0]).collect();
    let mut pl = cat_chart(&mut h, 340.0, &format!("sum = {}", fm(apv)), &items, term, range(&all, 0.12, true));
    bars(&mut pl, term, &[(v.clone(), "axp-b")], false, term <= 12);
    cat_labels(&mut pl, &(0..term).map(|t| t.to_string()).collect::<Vec<_>>());
    pl.axes_with("time of the payment (years)", "expected present value", false, true);
    let temp = s.key == "temporary_annuity_due";
    fig(s, "bars", if temp { "Temporary annuity-due: expected present value of each payment" } else {
            "Life annuity-due: expected present value of each payment" },
        &format!("Payments of {} at the start of each {} while alive, for death probabilities {} and interest {}. Each \
                  bar is the amount x the probability of payment that year (from the model's survival probabilities) x \
                  the discount factor (from the model's present value); the bars sum to the returned actuarial present \
                  value {}.", fm(pmt), if temp { format!("of {} years", term) } else { "year".into() }, list_text(&q),
                 pct(i), fm(apv)), finish(h, pl), 340.0)
}

fn net_premium_bars(s: &Solved) -> Option<Figure> {
    let (q, i, b) = (mort(s)?, n(s, "interest_rate")?, n(s, "benefit")?);
    let (prem, ins, ad) = (s.out("premium")?, s.out("insurance_apv")?, s.out("annuity_due")?);
    let nn = q.len();
    let mut sv = vec![];
    for t in 0..=nn {
        sv.push(surv(s, &q, t)?);
    }
    let mut up = vec![];
    let mut down = vec![];
    for t in 0..nn {
        up.push(prem * sv[t] * disc(s, i, t as f64)?);
        down.push(-b * (sv[t] - sv[t + 1]) * disc(s, i, t as f64 + 1.0)?);
    }
    if !agrees(up.iter().sum(), ins) || !agrees(-down.iter().sum::<f64>(), ins) {
        return None;
    }
    let mut h = String::new();
    let items = [it("premium (expected PV)", "axp-b2", Key::Box), it("benefit (expected PV)", "axp-b", Key::Box)];
    let all: Vec<f64> = up.iter().chain(&down).copied().chain([0.0]).collect();
    let mut pl = chart(&mut h, 340.0, &format!("premium {}: both sides {}", fm(prem), fm(ins)), &items,
                       (-0.6, nn as f64 + 0.6), range(&all, 0.15, true), false, false);
    for t in 0..nn {
        pl.rect(t as f64 - 0.32, t as f64 + 0.32, 0.0, up[t], "axp-b2");
        pl.rect(t as f64 + 1.0 - 0.32, t as f64 + 1.0 + 0.32, 0.0, down[t], "axp-b");
        if nn <= 8 {
            pl.label(t as f64, up[t], -6.0, "middle", "axp-vt", &fm(up[t]));
            pl.label(t as f64 + 1.0, down[t], 15.0, "middle", "axp-vt", &fm(down[t]));
        }
    }
    pl.hline(0.0, "axp-c4");
    pl.axes("year", "expected present value");
    fig(s, "bars", "Net premium: expected premiums balance expected benefits",
        &format!("Whole life insurance of {} (death probabilities {}, interest {}) paid for by the net annual premium \
                  the model returns, {}. Up: expected present value of each premium (premium x survival x discount); \
                  down: of each year's benefit (benefit x death probability x discount); survival and discount factors \
                  are runs of the model. Both sum to the returned insurance value {} (= premium x annuity-due {}).",
                 fm(b), list_text(&q), pct(i), fm(prem), fm(ins), fm(ad)), finish(h, pl), 340.0)
}

fn expectancy_bars(s: &Solved) -> Option<Figure> {
    let (q, cur, comp) = (mort(s)?, s.out("curtate")?, s.out("complete")?);
    let nn = q.len();
    let mut sv = vec![];
    for t in 1..=nn {
        sv.push(surv(s, &q, t)?);
    }
    if !agrees(sv.iter().sum(), cur) && (sv.iter().sum::<f64>() - cur).abs() > 1e-9 {
        return None;
    }
    let mut h = String::new();
    let items = [it("probability of surviving t years", "axp-b", Key::Box)];
    let mut pl = chart(&mut h, 340.0, &format!("curtate {} years, complete {}", fm(cur), fm(comp)), &items,
                       (0.4, nn as f64 + 0.6), (0.0, 1.08), false, false);
    for (t, v) in sv.iter().enumerate() {
        let x = t as f64 + 1.0;
        pl.rect(x - 0.34, x + 0.34, 0.0, *v, "axp-b");
        if nn <= 14 {
            pl.label(x, *v, -6.0, "middle", "axp-vt", &fm(*v));
        }
    }
    pl.axes("t (years)", "survival probability");
    fig(s, "survival", "Life expectancy as the sum of survival probabilities",
        &format!("Survival for the one-year death probabilities {} (each value a run of the model's survival \
                  probability). The curtate life expectancy the model returns, {} years, is the sum of the bars (whole \
                  years lived); the complete expectancy {} adds half a year.", list_text(&q), fm(cur), fm(comp)),
        finish(h, pl), 340.0)
}

fn survival_panels(s: &Solved) -> Option<Figure> {
    let (q, years, sv, dp) = (mort(s)?, n(s, "years")?, s.out("survival")?, s.out("death_probability")?);
    let ty = years.round() as usize;
    let nn = q.len();
    let tmax = nn;
    let mut pts = vec![];
    for t in 0..=tmax {
        if let Some(x) = surv(s, &q, t) {
            pts.push((t as f64, x));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let mut h = String::new();
    text(&mut h, 96.0, 20.0, "start", "axp-mt", &format!("t = {}: survival {}, death {}", ty, fm(sv), fm(dp)));
    text(&mut h, 96.0, 40.0, "start", "axp-lt", "one-year death probability for each year from the current age");
    let qmax = q.iter().copied().fold(0.0, f64::max).max(1e-9);
    let mut top = panel(58.0, 172.0, (-0.5, tmax as f64 + 0.5), (0.0, qmax * 1.2), false, false, 1);
    for (t, x) in q.iter().enumerate() {
        top.stem(t as f64, 0.0, *x, "axp-c3", "axp-d3");
        if nn <= 12 {
            top.label(t as f64, *x, -7.0, "middle", "axp-vt", &fm(*x));
        }
    }
    top.axes_with("", "q", true, true);
    let mut bot = panel(220.0, 418.0, (-0.5, tmax as f64 + 0.5), (0.0, 1.05), false, false, 2);
    bot.line(&pts, "axp-c");
    for (x, y) in &pts {
        bot.dot(*x, *y, "axp-d", 3.5);
    }
    bot.mark(ty as f64, sv, &format!("t = {}: {}", ty, fm(sv)));
    bot.axes("t (years)", "survival probability");
    let body = h + &top.s + &bot.s;
    fig(s, "survival", "Survival over the years from the current age",
        &format!("Top: the one-year death probabilities {} for each year from the current age (stated). Bottom: the \
                  probability of surviving t years, a run of the model for t = 0 to {}. Marked: t = {}: survival {}, \
                  death within {} years {}.", list_text(&q), tmax, ty, fm(sv), ty, fm(dp)), body, 480.0)
}
