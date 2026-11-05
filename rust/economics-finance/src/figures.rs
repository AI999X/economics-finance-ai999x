//! Economics and Finance's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: cash-flow bars, price-yield families, option values, frontiers, supply and demand, survival bars, ...
//! (docs/FIGURE_PARITY.md). The capabilities are drawn in figures_money.rs (time value, loans, bonds, budgeting,
//! life contingencies) and figures_markets.rs (options, portfolios, markets, statistics); this file holds the plotting
//! helpers they share (a chart with axes, legend, curves, areas, bars and marked points) and the solve helpers.

use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::json;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v = crate::figures_money::live(s);
    v.extend(crate::figures_markets::live(s));
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ solves
/// A number input as read (a number or a whole number).
pub(crate) fn n(s: &Solved, sym: &str) -> Option<f64> {
    if let Some(x) = s.inp(sym) {
        return Some(x);
    }
    match s.val(sym)? {
        ax_engine::Val::Num(x) if x.is_finite() => Some(x),
        ax_engine::Val::Int(i) => Some(i as f64),
        _ => None,
    }
}

/// A list input as an input value.
pub(crate) fn lst(v: &[f64]) -> Input {
    Input::Json(json!(v))
}

/// A number as an input value for `sym` of capability `key` (a whole number where the capability asks for one).
pub(crate) fn arg(s: &Solved, key: &str, sym: &str, x: f64) -> Input {
    let int = s.p.procedure(key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.ty == "integer")
        .unwrap_or(false);
    if int { Input::Int(x.round() as i64) } else { Input::Float(x) }
}

/// Another capability solved with number inputs (whole numbers where it asks for them); one output.
pub(crate) fn other(s: &Solved, key: &str, set: &[(&str, f64)], out: &str) -> Option<f64> {
    let v: Vec<(&str, Input)> = set.iter().map(|(k_, x)| (*k_, arg(s, key, k_, *x))).collect();
    k::num_of(&s.solve_other(key, &v)?, out)
}

/// Another capability solved with any inputs (lists, choices); one output.
pub(crate) fn other_in(s: &Solved, key: &str, set: &[(&str, Input)], out: &str) -> Option<f64> {
    k::num_of(&s.solve_other(key, set)?, out)
}

/// This capability solved again with any inputs replaced; one output.
pub(crate) fn again_in(s: &Solved, set: &[(&str, Input)], out: &str) -> Option<f64> {
    k::num_of(&s.with_input(set)?, out)
}

/// The timing choice as given ("ordinary" when not given).
pub(crate) fn timing(s: &Solved) -> String {
    s.txt("timing").unwrap_or_else(|| "ordinary".into())
}

/// A legend entry.
pub(crate) fn it(t: impl Into<String>, class: &'static str, key: Key) -> (String, &'static str, Key) {
    (t.into(), class, key)
}

/// Two values agree to 1e-6 of their size (a drawing built from parts is shown only when the parts add up to the
/// model's own total).
pub(crate) fn agrees(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-6 * a.abs().max(b.abs()).max(1e-9)
}

/// This capability solved again with some number inputs replaced (whole numbers where it asks for them).
pub(crate) fn again(s: &Solved, set: &[(&str, f64)]) -> Option<Answer> {
    let v: Vec<(&str, Input)> = set.iter().map(|(k_, x)| (*k_, arg(s, s.key, k_, *x))).collect();
    s.with_input(&v)
}

/// One output of `again`.
pub(crate) fn again1(s: &Solved, set: &[(&str, f64)], out: &str) -> Option<f64> {
    k::num_of(&again(s, set)?, out)
}

/// The present value of 1 due after `t` periods at the rate `i` (the model's present value).
pub(crate) fn disc(s: &Solved, i: f64, t: f64) -> Option<f64> {
    other(s, "present_value", &[("future_value", 1.0), ("rate", i), ("periods", t)], "present_value")
}

/// The probability of surviving `t` years for the one-year death probabilities `q` (the model's survival).
pub(crate) fn surv(s: &Solved, q: &[f64], t: usize) -> Option<f64> {
    if t == 0 {
        return Some(1.0);
    }
    if let Some(a) = s.solve_other("survival_probability", &[("mortality_rates", lst(q)), ("years", Input::Int(t as i64))]) {
        return k::num_of(&a, "survival");
    }
    // the last year of the table: the survival to the year before less the probability of dying in it (both returned)
    let a = s.solve_other("survival_probability", &[("mortality_rates", lst(q)), ("years", Input::Int(t as i64 - 1))])?;
    Some(k::num_of(&a, "survival")? - k::num_of(&a, "deferred_death_probability")?)
}

// ------------------------------------------------------------------------------------------------ numbers as text
/// A number as text: 4 significant digits, whole numbers from 10 000 up written out (431676, not 4.317e+05).
pub(crate) fn fm(x: f64) -> String {
    let x = if x.abs() < 1e-12 { 0.0 } else { x };
    if x.abs() >= 1e4 && x.abs() < 1e10 {
        return format!("{:.0}", x);
    }
    k::num(x)
}

/// A fraction as a percentage ("6.168 %").
pub(crate) fn pct(x: f64) -> String {
    format!("{} %", fm(x * 100.0))
}

/// A list as text ("-1000, 300, 400").
pub(crate) fn list_text(v: &[f64]) -> String {
    let t: Vec<String> = v.iter().map(|x| fm(*x)).collect();
    if t.len() > 12 {
        format!("{}, ... ({} values)", t[..10].join(", "), t.len())
    } else {
        t.join(", ")
    }
}

pub(crate) fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Estimated width of a text in a class.
fn tw(t: &str, class: &str) -> f64 {
    let c = t.chars().count() as f64;
    c * match class {
        "axp-mt" => 9.2,
        "axp-t" | "axp-vt" => 8.4,
        _ => 7.6,
    }
}

/// A text kept inside a canvas `w` wide (moved across when it would leave it).
pub(crate) fn text(out: &mut String, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
    if t.is_empty() {
        return;
    }
    let w = tw(t, class);
    let left = match anchor {
        "middle" => x - w / 2.0,
        "end" => x - w,
        _ => x,
    };
    let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - (left + w) } else { 0.0 };
    out.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                          x + shift, esc(t)));
}

// ------------------------------------------------------------------------------------------------ the chart
/// A legend entry: its words, its class and how it is drawn (a line, a dashed line, a filled box, a dot).
pub(crate) enum Key {
    Line,
    Dash,
    Box,
    Dot,
}

/// A plot area with its axes: data coordinates to pixels, curves, areas, bars, marks and labels, clipped to the box.
pub(crate) struct Plot {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    xr: (f64, f64),
    yr: (f64, f64),
    lx: bool,
    ly: bool,
    clip: String,
    pub s: String,
}

fn fix_range(r: (f64, f64), log: bool) -> (f64, f64) {
    let (mut lo, mut hi) = r;
    if log {
        if !(hi > 0.0) {
            hi = 1.0;
        }
        if !(lo > 0.0) || lo >= hi {
            lo = hi / 10.0;
        }
        return (lo, hi);
    }
    if !(hi > lo) || !(hi - lo).is_finite() {
        let d = if lo == 0.0 || !lo.is_finite() { 1.0 } else { lo.abs() * 0.1 };
        let lo0 = if lo.is_finite() { lo } else { 0.0 };
        lo = lo0 - d;
        hi = lo0 + d;
    }
    (lo, hi)
}

/// The range of some values padded by a fraction of their span on each side (`zero`: the range includes 0).
pub(crate) fn range(v: &[f64], pad: f64, zero: bool) -> (f64, f64) {
    let mut lo = v.iter().copied().filter(|x| x.is_finite()).fold(f64::INFINITY, f64::min);
    let mut hi = v.iter().copied().filter(|x| x.is_finite()).fold(f64::NEG_INFINITY, f64::max);
    if !lo.is_finite() {
        return (0.0, 1.0);
    }
    if zero {
        lo = lo.min(0.0);
        hi = hi.max(0.0);
    }
    let d = if hi > lo { (hi - lo) * pad } else { lo.abs().max(1.0) * 0.1 };
    let lo2 = if zero && lo == 0.0 { 0.0 } else { lo - d };
    let hi2 = if zero && hi == 0.0 { 0.0 } else { hi + d };
    (lo2, hi2)
}

/// A logarithmic range padded by a factor.
pub(crate) fn lrange(v: &[f64], f: f64) -> (f64, f64) {
    let p: Vec<f64> = v.iter().copied().filter(|x| *x > 0.0 && x.is_finite()).collect();
    if p.is_empty() {
        return (0.1, 1.0);
    }
    let lo = p.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = p.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (lo / f, hi * f)
}

fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 14 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

fn log_ticks(lo: f64, hi: f64) -> Vec<f64> {
    let (e0, e1) = (lo.log10().ceil() as i32, hi.log10().floor() as i32);
    let mut v: Vec<f64> = (e0..=e1).map(|e| 10f64.powi(e)).collect();
    if v.len() > 7 {
        let step = (v.len() as f64 / 6.0).ceil() as usize;
        v = v.into_iter().step_by(step).collect();
    }
    if v.len() < 2 {
        // a span inside one decade: 1-2-5 steps
        v = vec![];
        let mut e = lo.log10().floor() as i32;
        while 10f64.powi(e) <= hi * 1.0001 && v.len() < 20 {
            for m in [1.0, 2.0, 5.0] {
                let t = m * 10f64.powi(e);
                if t >= lo * 0.9999 && t <= hi * 1.0001 {
                    v.push(t);
                }
            }
            e += 1;
        }
        if v.len() < 2 {
            v = nice_ticks(lo, hi, 4);
        }
    }
    v
}

fn tick_text(t: f64) -> String {
    fm(t)
}

impl Plot {
    /// A plot box (x0, x1, y0, y1 in pixels) over the data ranges; `lx`/`ly`: logarithmic axes; `id`: the clip number
    /// (one per plot of a figure).
    pub fn new(bx: (f64, f64, f64, f64), xr: (f64, f64), yr: (f64, f64), lx: bool, ly: bool, id: usize) -> Plot {
        let (x0, x1, y0, y1) = bx;
        let clip = format!("CLIPID{id}");
        let mut s = String::new();
        s.push_str(&format!("<clipPath id=\"{clip}\"><rect x=\"{x0:.1}\" y=\"{y0:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>",
                            x1 - x0, y1 - y0));
        Plot { x0, x1, y0, y1, xr: fix_range(xr, lx), yr: fix_range(yr, ly), lx, ly, clip, s }
    }
    pub fn x(&self, v: f64) -> f64 {
        let f = if self.lx { (v.max(1e-300).log10() - self.xr.0.log10()) / (self.xr.1.log10() - self.xr.0.log10()) } else {
            (v - self.xr.0) / (self.xr.1 - self.xr.0)
        };
        self.x0 + f * (self.x1 - self.x0)
    }
    pub fn y(&self, v: f64) -> f64 {
        let f = if self.ly { (v.max(1e-300).log10() - self.yr.0.log10()) / (self.yr.1.log10() - self.yr.0.log10()) } else {
            (v - self.yr.0) / (self.yr.1 - self.yr.0)
        };
        self.y1 - f * (self.y1 - self.y0)
    }
    /// A pixel point, kept near the box (a far value would make a huge number in the drawing).
    fn p(&self, x: f64, y: f64) -> (f64, f64) {
        (self.x(x).clamp(self.x0 - 400.0, self.x1 + 400.0), self.y(y).clamp(self.y0 - 400.0, self.y1 + 400.0))
    }
    /// Grid, tick labels, the frame and the axis titles.
    pub fn axes(&mut self, xlab: &str, ylab: &str) {
        self.axes_with(xlab, ylab, true, true);
    }
    pub fn axes_with(&mut self, xlab: &str, ylab: &str, xt: bool, yt: bool) {
        let mut o = String::new();
        if xt {
            let mut ts = if self.lx { log_ticks(self.xr.0, self.xr.1) } else { nice_ticks(self.xr.0, self.xr.1, 5) };
            // fewer ticks when their labels would touch
            let widest = ts.iter().map(|t| tw(&tick_text(*t), "axp-t")).fold(0.0, f64::max);
            while ts.len() > 2 {
                let gap = (self.x(ts[1]) - self.x(ts[0])).abs();
                if gap >= widest + 10.0 {
                    break;
                }
                ts = ts.into_iter().step_by(2).collect();
            }
            for t in ts {
                let x = self.x(t);
                o.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>",
                                    self.y0, self.y1));
                text(&mut o, x, self.y1 + 19.0, "middle", "axp-t", &tick_text(t));
            }
        }
        if yt {
            let ts = if self.ly { log_ticks(self.yr.0, self.yr.1) } else { nice_ticks(self.yr.0, self.yr.1, 4) };
            for t in ts {
                let y = self.y(t);
                o.push_str(&format!("<line class=\"axp-g\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\"/>",
                                    self.x0, self.x1));
                text(&mut o, self.x0 - 6.0, y + 5.0, "end", "axp-t", &tick_text(t));
            }
        }
        o.push_str(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", self.x0,
                            self.y0, self.x1 - self.x0, self.y1 - self.y0));
        if !xlab.is_empty() {
            text(&mut o, (self.x0 + self.x1) / 2.0, self.y1 + 42.0, "middle", "axp-l", xlab);
        }
        if !ylab.is_empty() {
            let t: String = if ylab.chars().count() > 34 { ylab.chars().take(33).collect::<String>() + "." } else { ylab.into() };
            o.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                                (self.y0 + self.y1) / 2.0, esc(&t)));
        }
        // the grid goes under what is drawn already
        self.s = o + &self.s;
    }
    fn clipped(&mut self, el: &str) {
        self.s.push_str(&format!("<g clip-path=\"url(#{})\">{el}</g>", self.clip));
    }
    fn d(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in pts {
            let ok = x.is_finite() && y.is_finite() && (!self.lx || *x > 0.0) && (!self.ly || *y > 0.0);
            if !ok {
                pen = false;
                continue;
            }
            let (px, py) = self.p(*x, *y);
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// A curve (a NaN point breaks it).
    pub fn line(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            self.clipped(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
        }
    }
    /// A dashed curve.
    pub fn dash(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            self.clipped(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"{d}\"/>"));
        }
    }
    /// A filled polygon (a shaded area).
    pub fn fill(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            self.clipped(&format!("<path class=\"{class}\" d=\"{d} Z\"/>"));
        }
    }
    /// A rectangle between two x and two y values (a bar).
    pub fn rect(&mut self, xa: f64, xb: f64, ya: f64, yb: f64, class: &str) {
        let (px0, py0) = self.p(xa.min(xb), ya.max(yb));
        let (px1, py1) = self.p(xa.max(xb), ya.min(yb));
        let (top, bot) = (py0.min(py1), py0.max(py1));
        self.clipped(&format!("<rect class=\"{class}\" x=\"{px0:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                              (px1 - px0).abs().max(1.0), (bot - top).max(0.6)));
    }
    /// A filled dot.
    pub fn dot(&mut self, x: f64, y: f64, class: &str, r: f64) {
        if !(x.is_finite() && y.is_finite()) {
            return;
        }
        let (px, py) = self.p(x, y);
        self.clipped(&format!("<circle class=\"{class}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r:.1}\"/>"));
    }
    /// A vertical stem from `base` to y with a dot.
    pub fn stem(&mut self, x: f64, base: f64, y: f64, class: &str, dot: &str) {
        let (px, pb) = self.p(x, base);
        let (_, py) = self.p(x, y);
        self.clipped(&format!("<line class=\"{class}\" x1=\"{px:.1}\" y1=\"{pb:.1}\" x2=\"{px:.1}\" y2=\"{py:.1}\"/>"));
        self.dot(x, y, dot, 4.0);
    }
    /// A dashed vertical or horizontal guide across the box.
    pub fn vline(&mut self, x: f64, class: &str) {
        let px = self.x(x);
        if px >= self.x0 - 0.5 && px <= self.x1 + 0.5 {
            self.s.push_str(&format!("<path class=\"{class}\" stroke-dasharray=\"5 4\" d=\"M{px:.1},{:.1} L{px:.1},{:.1}\"/>",
                                     self.y0, self.y1));
        }
    }
    pub fn hline(&mut self, y: f64, class: &str) {
        let py = self.y(y);
        if py >= self.y0 - 0.5 && py <= self.y1 + 0.5 {
            self.s.push_str(&format!("<path class=\"{class}\" stroke-dasharray=\"5 4\" d=\"M{:.1},{py:.1} L{:.1},{py:.1}\"/>",
                                     self.x0, self.x1));
        }
    }
    /// The visitor's point: a ring and its label (right of it when there is room, else left).
    pub fn mark(&mut self, x: f64, y: f64, label: &str) {
        if !(x.is_finite() && y.is_finite()) {
            return;
        }
        let (px, py) = (self.x(x).clamp(self.x0, self.x1), self.y(y).clamp(self.y0, self.y1));
        self.s.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
        let w = tw(label, "axp-mt");
        let (tx, anchor) = if px + 10.0 + w <= self.x1 { (px + 10.0, "start") } else if px - 10.0 - w >= self.x0 {
            (px - 10.0, "end")
        } else {
            ((px - w / 2.0).clamp(self.x0 + 2.0, (self.x1 - w - 2.0).max(self.x0 + 2.0)), "start")
        };
        let ty = if py - 10.0 < self.y0 + 14.0 { py + 22.0 } else { py - 10.0 };
        text(&mut self.s, tx, ty, anchor, "axp-mt", label);
    }
    /// A label at a data point, kept inside the box vertically.
    pub fn label(&mut self, x: f64, y: f64, dy: f64, anchor: &str, class: &str, t: &str) {
        if !(x.is_finite() && y.is_finite()) {
            return;
        }
        let px = self.x(x).clamp(self.x0, self.x1);
        let py = (self.y(y) + dy).clamp(self.y0 + 13.0, self.y1 - 4.0);
        text(&mut self.s, px, py, anchor, class, t);
    }
    /// A label at pixel position (relative to the box's top left).
    pub fn note(&mut self, dx: f64, dy: f64, anchor: &str, class: &str, t: &str) {
        let (x, y) = (self.x0 + dx, self.y0 + dy);
        text(&mut self.s, x, y, anchor, class, t);
    }
}

/// A legend flowing across rows from the left edge of the plot at height y; returns the height below the last row.
pub(crate) fn legend(out: &mut String, items: &[(String, &str, Key)], y: f64) -> f64 {
    let (mut x, mut y) = (96.0, y);
    for (t, class, key) in items {
        let w = tw(t, "axp-lt") + 26.0;
        if x + w > 512.0 && x > 96.0 {
            x = 96.0;
            y += 17.0;
        }
        match key {
            Key::Line => out.push_str(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", x, y - 4.0, x + 16.0, y - 4.0)),
            Key::Dash => out.push_str(&format!("<path class=\"{class}\" stroke-dasharray=\"5 3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                               x, y - 4.0, x + 16.0, y - 4.0)),
            Key::Box => out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"13\" height=\"10\"/>", y - 9.0)),
            Key::Dot => out.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/>", x + 7.0, y - 4.0)),
        }
        text(out, x + 20.0, y, "start", "axp-lt", t);
        x += w + 8.0;
    }
    y + 10.0
}

/// One chart in a canvas `h` tall: a header line, a legend, then the plot box below them (x 96 to 504, its bottom
/// 62 px above the canvas's). Returns the plot (draw on it, then `finish`).
pub(crate) fn chart(head: &mut String, h: f64, header: &str, items: &[(String, &str, Key)], xr: (f64, f64),
                    yr: (f64, f64), lx: bool, ly: bool) -> Plot {
    if !header.is_empty() {
        text(head, 96.0, 20.0, "start", "axp-mt", header);
    }
    let top = if items.is_empty() { 40.0 } else { legend(head, items, 40.0) + 4.0 };
    Plot::new((96.0, 504.0, top.max(44.0), h - 62.0), xr, yr, lx, ly, 0)
}

/// A plot in a panel of a taller canvas: its box from y0 to y1 (pixels), its own clip number.
pub(crate) fn panel(y0: f64, y1: f64, xr: (f64, f64), yr: (f64, f64), lx: bool, ly: bool, id: usize) -> Plot {
    Plot::new((96.0, 504.0, y0, y1), xr, yr, lx, ly, id)
}

/// Header, legend and plot joined into a figure body.
pub(crate) fn finish(head: String, p: Plot) -> String {
    head + &p.s
}

/// A 340-tall chart figure.
#[allow(clippy::too_many_arguments)]
pub(crate) fn fig(s: &Solved, kind: &str, title: &str, caption: &str, body: String, h: f64) -> Option<Figure> {
    if body.contains("NaN") || body.contains("inf\"") {
        return None;
    }
    Some(s.figure(kind, title, caption, &body, h))
}

/// Points of a series as (index from `start`, value).
pub(crate) fn indexed(v: &[f64], start: f64) -> Vec<(f64, f64)> {
    v.iter().enumerate().map(|(i, y)| (start + i as f64, *y)).collect()
}

/// Bars of one or more groups over categories in a plot, a group's bars side by side (`stack`: on top of each other).
pub(crate) fn bars(p: &mut Plot, n: usize, groups: &[(Vec<f64>, &str)], stack: bool, values: bool) {
    let ng = groups.len().max(1);
    for i in 0..n {
        let cx = i as f64;
        let mut pos = 0.0;
        let mut neg = 0.0;
        for (g, (vals, class)) in groups.iter().enumerate() {
            let Some(v) = vals.get(i).copied().filter(|v| v.is_finite()) else { continue };
            let (xa, xb, ya, yb) = if stack {
                let base = if v >= 0.0 { pos } else { neg };
                if v >= 0.0 { pos += v } else { neg += v }
                (cx - 0.32, cx + 0.32, base, base + v)
            } else {
                let w = 0.8 / ng as f64;
                let l = cx - 0.4 + w * g as f64;
                (l + 0.03, l + w - 0.03, 0.0, v)
            };
            p.rect(xa, xb, ya, yb, class);
            if values && n * ng <= 14 && !stack {
                let dy = if v >= 0.0 { -5.0 } else { 15.0 };
                p.label((xa + xb) / 2.0, yb, dy, "middle", "axp-bt", &fm(v));
            }
        }
    }
}

/// Category labels under bars at 0, 1, ... (every k-th when many).
pub(crate) fn cat_labels(p: &mut Plot, labels: &[String]) {
    let n = labels.len();
    let every = (n as f64 / 12.0).ceil().max(1.0) as usize;
    for (i, l) in labels.iter().enumerate() {
        if i % every == 0 {
            let x = p.x(i as f64);
            let y = p.y1 + 19.0;
            text(&mut p.s, x, y, "middle", "axp-t", l);
        }
    }
}

/// A plot for categories 0 .. n-1 (bars) over the value range `yr`.
pub(crate) fn cat_chart(head: &mut String, h: f64, header: &str, items: &[(String, &str, Key)], n: usize,
                        yr: (f64, f64)) -> Plot {
    chart(head, h, header, items, (-0.6, n as f64 - 0.4), yr, false, false)
}
