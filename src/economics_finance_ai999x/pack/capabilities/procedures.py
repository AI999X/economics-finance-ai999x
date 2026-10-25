"""Economics and finance procedures: deterministic algorithms with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
worked examples, an INDEPENDENT reference computation (used only by certification, calling numpy-financial, QuantLib,
scipy, statsmodels, sympy or mpmath, or a different method) and a deliberately wrong variant (certification shows that
the checks reject it). The shipped code (run, verify, parse) is plain Python.

Conventions, stated once and repeated in each procedure:
  * rates are decimals (0.05) or written with a percent sign (5%); an interest rate, yield, growth or inflation rate
    must lie strictly between -100% and 100% (a value such as 8 is refused: write 8%);
  * a "period" is the payment or compounding period; `rate` per period unless the input says "annual";
  * an ordinary annuity pays at the END of each period, an annuity due at the BEGINNING;
  * cash flows are a list [CF0, CF1, ..., CFn] with CF0 at time 0 (now) and CFt at the end of period t;
  * bonds are priced on a coupon date (no accrued interest), the yield is a nominal annual rate compounded at the
    coupon frequency;
  * options are European on an asset with a continuous dividend yield; time in years, volatility and rates annual,
    continuously compounded;
  * life tables are annual: q_x, q_x+1, ... (the probability of dying within the year), the benefit of an insurance is
    paid at the end of the year of death, a life annuity due pays at the start of each year while alive.
This is quantitative education, not financial advice.

Questions are read in one documented form: a phrase naming the capability, then `name = value` pairs (lists in
brackets: `cash flows = [-1000, 300, 400, 500]`; matrices as `[[0.04, 0.006], [0.006, 0.09]]`). A number that is not
given as `name = value`, or a name the capability does not take, makes the question unrecognised (never guessed).
"""
from __future__ import annotations

import math
import re
import statistics
from fractions import Fraction
from functools import lru_cache

PROCEDURES: dict = {}
_ALIASES: dict = {}
_TRIGGERS: dict = {}
_SYN: dict = {}


# --- input checks -----------------------------------------------------------------------------------------------
def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(name, v):
    _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), f"{name} must be a finite number")
    return float(v)


def _pos(name, v):
    v = _fin(name, v)
    _need(v > 0, f"{name} must be positive")
    return v


def _nonneg(name, v):
    v = _fin(name, v)
    _need(v >= 0, f"{name} must not be negative")
    return v


def _rate(name, v, lo=-1.0, hi=1.0):
    v = _fin(name, v)
    _need(lo < v < hi, f"{name} must be a decimal rate between {lo:g} and {hi:g} (write 5% or 0.05)")
    return v


def _prob(name, v, lo=0.0, hi=1.0):
    v = _fin(name, v)
    _need(lo <= v <= hi, f"{name} must lie between {lo:g} and {hi:g}")
    return v


def _int(name, v, lo=1, hi=None):
    _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and float(v) == int(v),
          f"{name} must be a whole number")
    v = int(v)
    _need(v >= lo, f"{name} must be at least {lo}")
    _need(hi is None or v <= hi, f"{name} must be at most {hi}")
    return v


def _vec(name, xs, n_min=1):
    _need(isinstance(xs, (list, tuple)) and len(xs) >= n_min, f"{name} needs at least {n_min} values")
    return [_fin(name, x) for x in xs]


def _periods_of(years, per_year, name="years"):
    n = years * per_year
    _need(abs(n - round(n)) <= 1e-9 * max(1.0, abs(n)) and round(n) >= 1,
          f"{name} times the number of payments per year must be a whole number of periods (at least 1)")
    return int(round(n))


def _timing(t):
    _need(t in ("ordinary", "due"), "timing is 'ordinary' (end of period) or 'due' (beginning of period)")
    return 1 if t == "due" else 0


# --- verification: outputs compared with a recomputation (relative to max(1, |value|)) -------------------------
def _residual(got, want) -> float:
    worst = 0.0
    for k, w in want.items():
        if k not in got:
            return math.inf
        o = got[k]
        if isinstance(w, str):
            if o != w:
                return math.inf
            continue
        pairs = list(zip(o, w)) if isinstance(w, (list, tuple)) else [(o, w)]
        if isinstance(w, (list, tuple)) and (not isinstance(o, (list, tuple)) or len(o) != len(w)):
            return math.inf
        for a, b in pairs:
            if isinstance(a, bool) or not isinstance(a, (int, float)) or not math.isfinite(a):
                return math.inf
            # relative with certification's tiny floor: with max(1, |b|) a sinking-fund payment of 1e-20 (360
            # periods at 15 %) "agreed" with any wrong value below 1e-9 (2026-10-23)
            d = abs(a - b)
            worst = max(worst, 0.0 if d <= 1e-15 else d / max(abs(a), abs(b)))
    return worst


def _verifier(fn):
    def verify(inputs, outputs):
        return _residual(outputs, fn(**inputs))
    return verify


# --- reading questions ----------------------------------------------------------------------------------------
_CONNECT = {"with", "and", "where", "given", "at", "for", "if", "when", "using", "let", "is", "are", "has", "having",
            "assuming", "then", "also", "plus", "a", "an", "the", "of", "its", "to", "from", "by", "in", "on", "as"}
_NUMBER = re.compile(r"[$€£]?\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?)(\s*%)?")
_WORD = re.compile(r"[a-z][a-z-]*")


def _norm(question: str) -> str:
    q = question.lower().replace("−", "-").replace("–", "-").replace("’", "'").replace("×", "x")
    return re.sub(r"\s+", " ", q).strip()


def _value_at(text, j):
    """(value text, kind, end) of the value written at text[j:], or None."""
    while j < len(text) and text[j] == " ":
        j += 1
    if j >= len(text):
        return None
    if text[j] == "[":
        depth = 0
        for k in range(j, len(text)):
            depth += {"[": 1, "]": -1}.get(text[k], 0)
            if depth == 0:
                return text[j:k + 1], "list", k + 1
        return None
    m = _NUMBER.match(text, j)
    if m:
        return m.group(0), "number", m.end()
    m = _WORD.match(text, j)
    if m:
        return m.group(0), "word", m.end()
    return None


def _pairs(text):
    """Every `name = value` of a question: (segment before '=', value, kind) and the text that is not a pair."""
    pairs, rest, pos = [], [], 0
    for m in re.finditer("=", text):
        p = m.start()
        if p < pos:
            return None
        seg = text[pos:p]
        cut = max(seg.rfind(c) for c in ",;:(")
        rest.append(seg[:cut + 1])
        got = _value_at(text, p + 1)
        if got is None:
            return None
        value, kind, end = got
        if end < len(text) and (text[end].isalnum() or text[end] in "_[="):
            return None
        pairs.append((seg[cut + 1:].strip(), value, kind))
        pos = end
    rest.append(text[pos:])
    return pairs, rest


def _trigger_text(text):
    """The question without its assignments' names and values (a name such as 'number of periods' must not decide
    which capability is asked)."""
    got = _pairs(text)
    if got is None:
        return text
    pairs, rest = got
    pieces = list(rest)
    for i, (seg, _v, _k) in enumerate(pairs):
        words = list(re.finditer(r"[a-z0-9_'%$.-]+", seg))
        keep = 0
        for w in words:
            if w.group(0) in _CONNECT:
                keep = w.end()
        pieces[i] = pieces[i] + " " + seg[:keep]
    return " ".join(pieces)


@lru_cache(maxsize=256)
def _winners(text):
    t = _trigger_text(text)
    best, keys = 0, []
    for key, pats in _TRIGGERS.items():
        n = max((len(m.group(0)) for p in pats for m in [re.search(p, t)] if m), default=0)
        if n > best:
            best, keys = n, [key]
        elif n == best and n > 0:
            keys.append(key)
    return tuple(keys)


def _list_value(v):
    inner = v.strip()[1:-1]
    if "%" not in inner or "[" in inner:
        return v
    try:
        return [float(t[:-1]) / 100 if t.endswith("%") else float(t) for t in re.split(r"[,\s;]+", inner.strip()) if t]
    except ValueError:
        return v


def _read(key, text):
    got = _pairs(text)
    if got is None:
        return None
    pairs, rest = got
    aliases, by = _ALIASES[key], {i["symbol"]: i for i in PROCEDURES[key]["inputs"]}
    out, left = {}, list(rest)
    for seg, value, kind in pairs:
        sym = None
        for alias, s in aliases:
            if seg.endswith(alias) and (len(seg) == len(alias) or seg[-len(alias) - 1] == " "):
                before = seg[:-len(alias)].strip()
                words = re.findall(r"[a-z0-9_'%$.-]+", before)
                if not words or words[-1] in _CONNECT:
                    sym = s
                    left.append(before)
                    break
        if sym is None or sym in out:
            return None                                   # a name the capability does not take (or given twice)
        spec = by[sym]
        t = spec.get("type", "number")
        if t == "choice":
            if kind != "word":
                return None
            out[sym] = _SYN.get(key, {}).get(sym, {}).get(value, value)
        elif t in ("vector", "matrix"):
            if kind != "list":
                return None
            out[sym] = _list_value(value) if t == "vector" else value
        else:
            if kind != "number":
                return None
            m = _NUMBER.match(value)
            x = float(m.group(1))
            out[sym] = x / 100 if m.group(2) else x
    if re.search(r"[\d=\[\]]", " ".join(left)):
        return None                                       # a number that is not given as name = value: never guessed
    return out


def _parser(key):
    def parse(question):
        if not isinstance(question, str):
            return None
        text = _norm(question)
        if key not in _winners(text):
            return None
        return _read(key, text)
    return parse


def _phrase(p):
    """'net present value' -> a regex with word boundaries and flexible spaces or hyphens."""
    return r"\b" + re.sub(r"[ -]+", r"[\\s-]+", p) + r"\b"


# --- declaring a procedure ------------------------------------------------------------------------------------------
def _i(sym, meaning, *aliases, type=None, required=True, choices=None, syn=None, **limits):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if type:
        d["type"] = type
    if choices:
        d["choices"] = list(choices)
    d.update(limits)
    d["_aliases"] = list(aliases) + [sym.replace("_", " ")]
    if syn:
        d["_syn"] = syn
    return d


def _o(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, conditions, method, latex, run, reference, wrong,
         rand, triggers, templates, examples, questions, check=None):
    al = []
    syn = {}
    for i in inputs:
        for a in i.pop("_aliases"):
            al.append((a.lower(), i["symbol"]))
        if "_syn" in i:
            syn[i["symbol"]] = i.pop("_syn")
    _ALIASES[key] = sorted(set(al), key=lambda t: -len(t[0]))
    _TRIGGERS[key] = [re.compile(t[3:] if t.startswith("re:") else _phrase(t)) for t in triggers]
    if syn:
        _SYN[key] = syn
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": conditions, "method": method, "latex": latex, "run": run,
        "verify": _verifier(check or run), "parse": _parser(key), "reference": reference, "wrong": wrong,
        "random_inputs": rand, "templates": templates, "examples": examples, "questions": questions,
        "keywords": [t for t in triggers if not t.startswith("re:")]}


def _u(rng, a, b, nd=4):
    return round(rng.uniform(a, b), nd)


_TIMING = dict(type="choice", choices=["ordinary", "due"], required=False,
               syn={"end": "ordinary", "arrears": "ordinary", "immediate": "ordinary", "beginning": "due",
                    "begin": "due", "advance": "due", "start": "due"})


def _hnpv(rate, cfs):
    """Sum of CFt / (1 + rate)^t by Horner's rule in v = 1/(1 + rate)."""
    v, acc = 1.0 / (1.0 + rate), 0.0
    for c in reversed(cfs):
        acc = acc * v + c
    return acc


def _sign_changes(cfs):
    s = [c for c in cfs if c != 0]
    return sum(1 for a, b in zip(s, s[1:]) if (a > 0) != (b > 0))


def _irr(cfs):
    """The unique internal rate of return of cash flows that change sign exactly once (bisection in v = 1/(1+r),
    then Newton polishing)."""
    k = _sign_changes(cfs)
    _need(k >= 1, "the cash flows never change sign: there is no internal rate of return")
    _need(k == 1, f"the cash flows change sign {k} times: the internal rate of return is not unique, so none is given")
    nz = [c for c in cfs if c != 0]
    first = nz[0] > 0
    g = lambda v: sum(c * v ** t for t, c in enumerate(cfs))
    lo, hi = 0.0, 1.0
    while (g(hi) > 0) == first:
        lo, hi = hi, hi * 2
        _need(hi < 2.0 ** 60, "the internal rate of return is too close to -100%")
    for _ in range(300):
        mid = (lo + hi) / 2
        if mid in (lo, hi):
            break
        if (g(mid) > 0) == first:
            lo = mid
        else:
            hi = mid
    r = 1.0 / ((lo + hi) / 2) - 1.0
    for _ in range(3):
        f = _hnpv(r, cfs)
        d = sum(-t * c * (1.0 + r) ** (-t - 1) for t, c in enumerate(cfs))
        if d == 0 or not math.isfinite(f / d):
            break
        nr = r - f / d
        if not (nr > -1 and abs(_hnpv(nr, cfs)) <= abs(f)):
            break
        r = nr
    return r


def _npv_scale(rate, cfs):
    return sum(abs(c) * (1.0 + rate) ** (-t) for t, c in enumerate(cfs)) or 1.0


# =====================================================================================================================
# TIME VALUE OF MONEY
# =====================================================================================================================
def simple_interest(principal, rate, years):
    P, r, t = _pos("principal", principal), _rate("rate", rate, lo=-1e-300), _nonneg("years", years)
    I = P * r * t
    return {"interest": I, "amount": P + I}


def _ref_simple(principal, rate, years):
    from scipy.integrate import quad                       # interest accrues at the constant speed P*r per year
    I = quad(lambda s: principal * rate, 0, years)[0]
    return {"interest": I, "amount": principal + I}


_add("simple_interest", name="Simple interest", topic="time value of money / interest",
     purpose="Interest and final amount when interest is earned on the principal only: I = P r t.",
     inputs=[_i("principal", "amount invested or borrowed", "p", "present value", "pv", "amount"),
             _i("rate", "annual simple interest rate (decimal; 5% = 0.05)", "r", "interest rate", "annual rate"),
             _i("years", "time in years", "t", "time", "time in years")],
     outputs=[_o("interest", "simple interest earned P r t"), _o("amount", "principal plus interest")],
     primary="interest", conditions=["principal > 0", "0 <= rate < 1", "years >= 0"],
     method="I = P r t, A = P + I", latex=r"I = P\,r\,t,\quad A = P(1 + r t)", run=simple_interest,
     reference=_ref_simple,
     wrong=lambda principal, rate, years: {"interest": principal * ((1 + rate) ** years - 1),
                                           "amount": principal * (1 + rate) ** years},
     rand=lambda rng: {"principal": _u(rng, 100, 50000, 2), "rate": _u(rng, 0.01, 0.15),
                       "years": rng.choice([_u(rng, 0.25, 10, 2), rng.randint(1, 30)])},
     triggers=["simple interest"],
     templates=["Find the simple interest with principal = 1000, rate = 5%, years = 3."],
     examples=[{"inputs": {"principal": 1000, "rate": 0.05, "years": 3}, "outputs": {"interest": 150.0, "amount": 1150.0}},
               {"inputs": {"principal": 2500, "rate": 0.04, "years": 0.5}, "outputs": {"interest": 50.0, "amount": 2550.0}}],
     questions=[{"q": "Find the simple interest with principal = 1000, rate = 5%, years = 3.", "outputs": {"interest": 150.0, "amount": 1150.0}},
                {"q": "What is the simple interest with principal = $2500, rate = 4%, time = 0.5?", "outputs": {"interest": 50.0}},
                {"q": "Find the simple interest with principal = 1000, rate = 5%.", "outcome": "NEEDS_INPUT"},
                {"q": "Find the simple interest with principal = 1000, rate = 5, years = 3.", "outcome": "INVALID_INPUT"},
                {"q": "Find the simple interest on 1000 at 5% for 3 years.", "outcome": "UNRECOGNIZED_FORMAT"}])


def compound_interest(principal, rate, years, periods_per_year=1):
    P, r, t = _pos("principal", principal), _rate("rate", rate), _nonneg("years", years)
    m = _int("periods_per_year", periods_per_year, 1, 100000)
    A = P * (1 + r / m) ** (m * t)
    return {"amount": A, "interest": A - P}


def _ref_compound(principal, rate, years, periods_per_year=1):
    import numpy_financial as npf
    A = float(npf.fv(rate / periods_per_year, periods_per_year * years, 0, -principal))
    return {"amount": A, "interest": A - principal}


_add("compound_interest", name="Compound interest", topic="time value of money / interest",
     purpose="Future amount of a sum invested at a nominal annual rate compounded m times a year: A = P (1 + r/m)^(m t).",
     inputs=[_i("principal", "amount invested now", "p", "present value", "pv", "amount"),
             _i("rate", "nominal annual interest rate (decimal; 5% = 0.05)", "r", "interest rate", "annual rate", "nominal rate"),
             _i("years", "time in years", "t", "time"),
             _i("periods_per_year", "compounding periods per year (1 annual, 12 monthly, 365 daily)", "m", "n",
                "compounding periods per year", "compounding frequency", "compoundings per year", type="integer",
                required=False, min=1)],
     outputs=[_o("amount", "future amount A"), _o("interest", "compound interest earned A - P")],
     primary="amount", conditions=["principal > 0", "-1 < rate < 1", "years >= 0", "periods_per_year a whole number >= 1"],
     method="A = P (1 + r/m)^(m t)", latex=r"A = P\left(1 + \frac{r}{m}\right)^{m t}", run=compound_interest,
     reference=_ref_compound,
     wrong=lambda principal, rate, years, periods_per_year=1: {"amount": principal * (1 + rate) ** years,
                                                              "interest": principal * ((1 + rate) ** years - 1)},
     rand=lambda rng: {"principal": _u(rng, 100, 100000, 2), "rate": _u(rng, 0.005, 0.2), "years": rng.randint(1, 40),
                       "periods_per_year": rng.choice([1, 2, 4, 12, 12, 365])},
     triggers=["compound interest", "compounded amount"],
     templates=["Find the compound interest with principal = 1000, rate = 5%, years = 10, periods per year = 12."],
     examples=[{"inputs": {"principal": 1000, "rate": 0.05, "years": 2, "periods_per_year": 1},
                "outputs": {"amount": 1102.5, "interest": 102.5}},
               {"inputs": {"principal": 1000, "rate": 0.05, "years": 10, "periods_per_year": 12}, "outputs": {'amount': 1647.00949769028, 'interest': 647.0094976902801}}],
     questions=[{"q": "Find the compound interest with principal = 1000, rate = 5%, years = 2.", "outputs": {"amount": 1102.5}},
                {"q": "Find the compound interest with principal = 1000, rate = 5%, years = 10, periods per year = 12.", "outputs": {'amount': 1647.00949769028, 'interest': 647.0094976902801}},
                {"q": "Find the compound interest with principal = 1000, rate = 5%, years = 10, periods per year = 2.5.", "outcome": "INVALID_INPUT"},
                {"q": "Find the compound interest with principal = 1000, years = 10.", "outcome": "NEEDS_INPUT"}])


def continuous_compounding(principal, rate, years):
    P, r, t = _pos("principal", principal), _rate("rate", rate), _nonneg("years", years)
    A = P * math.exp(r * t)
    return {"amount": A, "interest": A - P}


def _ref_continuous(principal, rate, years):
    import mpmath                                          # integrate dA/dt = r A (Taylor-series ODE solver)
    with mpmath.workdps(30):
        A = float(mpmath.odefun(lambda s, y: rate * y, 0, mpmath.mpf(principal))(years))
    return {"amount": A, "interest": A - principal}


_add("continuous_compounding", name="Continuous compounding", topic="time value of money / interest",
     purpose="Future amount of a sum growing at a continuously compounded annual rate: A = P e^(r t).",
     inputs=[_i("principal", "amount invested now", "p", "present value", "pv", "amount"),
             _i("rate", "continuously compounded annual rate (decimal)", "r", "interest rate", "annual rate"),
             _i("years", "time in years", "t", "time")],
     outputs=[_o("amount", "future amount P e^(r t)"), _o("interest", "interest earned")],
     primary="amount", conditions=["principal > 0", "-1 < rate < 1", "years >= 0"],
     method="A = P exp(r t)", latex=r"A = P e^{r t}", run=continuous_compounding, reference=_ref_continuous,
     wrong=lambda principal, rate, years: {"amount": principal * (1 + rate) ** years, "interest": principal * ((1 + rate) ** years - 1)},
     rand=lambda rng: {"principal": _u(rng, 100, 100000, 2), "rate": _u(rng, 0.005, 0.2), "years": _u(rng, 0.5, 30, 2)},
     triggers=["continuous compounding", "compounded continuously", "continuously compounded"],
     templates=["Find the amount with continuous compounding with principal = 1000, rate = 5%, years = 10."],
     examples=[{"inputs": {"principal": 1000, "rate": 0.05, "years": 10}, "outputs": {"amount": 1648.7212707001282}},
               {"inputs": {"principal": 500, "rate": 0.08, "years": 2.5}, "outputs": {'amount': 610.701379080085, 'interest': 110.70137908008496}}],
     questions=[{"q": "Find the amount with continuous compounding with principal = 1000, rate = 5%, years = 10.",
                 "outputs": {"amount": 1648.7212707001282}},
                {"q": "What is the amount compounded continuously with principal = 500, rate = 8%, years = 2.5?", "outputs": {'amount': 610.701379080085, 'interest': 110.70137908008496}},
                {"q": "Find the amount with continuous compounding with principal = 0, rate = 5%, years = 10.", "outcome": "INVALID_INPUT"}])


def effective_annual_rate(rate, periods_per_year):
    r, m = _rate("rate", rate), _int("periods_per_year", periods_per_year, 1, 100000)
    return {"effective_rate": (1 + r / m) ** m - 1, "periodic_rate": r / m}


def _ref_ear(rate, periods_per_year):
    import numpy_financial as npf
    return {"effective_rate": float(npf.fv(rate / periods_per_year, periods_per_year, 0, -1)) - 1}


_add("effective_annual_rate", name="Effective annual rate", topic="time value of money / interest rates",
     purpose="The effective annual rate (annual percentage yield) of a nominal rate compounded m times a year.",
     inputs=[_i("rate", "nominal annual rate (decimal)", "r", "nominal rate", "nominal annual rate", "annual rate", "apr", "stated rate"),
             _i("periods_per_year", "compounding periods per year", "m", "n", "compounding periods per year",
                "compounding frequency", "compoundings per year", type="integer", min=1)],
     outputs=[_o("effective_rate", "effective annual rate (1 + r/m)^m - 1"), _o("periodic_rate", "rate per period r/m")],
     primary="effective_rate", conditions=["-1 < rate < 1", "periods_per_year a whole number >= 1"],
     method="EAR = (1 + r/m)^m - 1", latex=r"\mathrm{EAR} = \left(1 + \frac{r}{m}\right)^{m} - 1",
     run=effective_annual_rate, reference=_ref_ear,
     wrong=lambda rate, periods_per_year: {"effective_rate": rate, "periodic_rate": rate / periods_per_year},
     rand=lambda rng: {"rate": _u(rng, 0.005, 0.3), "periods_per_year": rng.choice([2, 4, 12, 52, 365])},
     triggers=["effective annual rate", "effective rate", "annual percentage yield", "effective annual yield"],
     templates=["Find the effective annual rate with rate = 6%, periods per year = 12."],
     examples=[{"inputs": {"rate": 0.06, "periods_per_year": 12}, "outputs": {'effective_rate': 0.06167781186449828, 'periodic_rate': 0.005}},
               {"inputs": {"rate": 0.1, "periods_per_year": 2}, "outputs": {"effective_rate": 0.1025}}],
     questions=[{"q": "Find the effective annual rate with rate = 6%, periods per year = 12.", "outputs": {'effective_rate': 0.06167781186449828, 'periodic_rate': 0.005}},
                {"q": "What is the annual percentage yield with nominal rate = 10%, compounding frequency = 2?",
                 "outputs": {"effective_rate": 0.1025}},
                {"q": "Find the effective annual rate with rate = 6%, periods per year = 0.", "outcome": "INVALID_INPUT"}])


def nominal_rate(effective_rate, periods_per_year):
    e, m = _rate("effective_rate", effective_rate), _int("periods_per_year", periods_per_year, 1, 100000)
    p = (1 + e) ** (1 / m) - 1
    return {"nominal_rate": m * p, "periodic_rate": p}


def _ref_nominal(effective_rate, periods_per_year):
    import numpy_financial as npf
    from scipy.optimize import brentq
    m = periods_per_year
    r = brentq(lambda x: float(npf.fv(x / m, m, 0, -1)) - 1 - effective_rate, -0.99 * m, 2.0 * m, xtol=1e-16, rtol=1e-15)
    return {"nominal_rate": r}


_add("nominal_rate", name="Nominal rate from an effective rate", topic="time value of money / interest rates",
     purpose="The nominal annual rate (APR) compounded m times a year that gives a stated effective annual rate.",
     inputs=[_i("effective_rate", "effective annual rate (decimal)", "ear", "effective annual rate", "effective rate", "apy"),
             _i("periods_per_year", "compounding periods per year", "m", "n", "compounding periods per year",
                "compounding frequency", type="integer", min=1)],
     outputs=[_o("nominal_rate", "nominal annual rate m((1 + EAR)^(1/m) - 1)"), _o("periodic_rate", "rate per period")],
     primary="nominal_rate", conditions=["-1 < effective_rate < 1", "periods_per_year a whole number >= 1"],
     method="r = m ((1 + EAR)^(1/m) - 1)", latex=r"r = m\left[(1 + \mathrm{EAR})^{1/m} - 1\right]",
     run=nominal_rate, reference=_ref_nominal,
     wrong=lambda effective_rate, periods_per_year: {"nominal_rate": (1 + effective_rate) ** (1 / periods_per_year) - 1,
                                                     "periodic_rate": (1 + effective_rate) ** (1 / periods_per_year) - 1},
     rand=lambda rng: {"effective_rate": _u(rng, 0.005, 0.3), "periods_per_year": rng.choice([2, 4, 12, 52, 365])},
     triggers=["nominal annual rate", "nominal rate", "annual percentage rate", "apr"],
     templates=["Find the nominal annual rate with effective rate = 8%, periods per year = 12."],
     examples=[{"inputs": {"effective_rate": 0.1025, "periods_per_year": 2}, "outputs": {"nominal_rate": 0.1}},
               {"inputs": {"effective_rate": 0.08, "periods_per_year": 12}, "outputs": {'nominal_rate': 0.07720836132004116, 'periodic_rate': 0.00643403011000343}}],
     questions=[{"q": "Find the nominal annual rate with effective rate = 8%, periods per year = 12.", "outputs": {'nominal_rate': 0.07720836132004116, 'periodic_rate': 0.00643403011000343}},
                {"q": "What APR with ear = 10.25%, m = 2?", "outputs": {"nominal_rate": 0.1}}])


def present_value(future_value, rate, periods):
    F, r, n = _fin("future_value", future_value), _rate("rate", rate), _nonneg("periods", periods)
    d = (1 + r) ** (-n)
    return {"present_value": F * d, "discount_factor": d}


def _ref_pv(future_value, rate, periods):
    import numpy_financial as npf
    return {"present_value": -float(npf.pv(rate, periods, 0, future_value))}


_add("present_value", name="Present value of a single sum", topic="time value of money / discounting",
     purpose="What a single amount received after n periods is worth today at a rate per period: PV = FV / (1 + r)^n.",
     inputs=[_i("future_value", "amount received in the future", "fv", "f", "future amount", "amount"),
             _i("rate", "discount rate per period (decimal)", "r", "i", "discount rate", "interest rate"),
             _i("periods", "number of periods until the amount is received", "n", "t", "number of periods", "years")],
     outputs=[_o("present_value", "value today FV (1 + r)^-n"), _o("discount_factor", "(1 + r)^-n")],
     primary="present_value", conditions=["-1 < rate < 1", "periods >= 0"],
     method="PV = FV (1 + r)^-n", latex=r"PV = \frac{FV}{(1 + r)^{n}}", run=present_value, reference=_ref_pv,
     wrong=lambda future_value, rate, periods: {"present_value": future_value / (1 + rate * periods),
                                                "discount_factor": 1 / (1 + rate * periods)},
     rand=lambda rng: {"future_value": _u(rng, 100, 100000, 2), "rate": _u(rng, 0.005, 0.2), "periods": rng.randint(1, 40)},
     triggers=["present value", "discounted value"],
     templates=["Find the present value with future value = 1000, rate = 5%, periods = 10."],
     examples=[{"inputs": {"future_value": 1102.5, "rate": 0.05, "periods": 2}, "outputs": {"present_value": 1000.0}},
               {"inputs": {"future_value": 1000, "rate": 0.05, "periods": 10}, "outputs": {'present_value': 613.9132535407591, 'discount_factor': 0.6139132535407591}}],
     questions=[{"q": "Find the present value with future value = 1000, rate = 5%, periods = 10.", "outputs": {'present_value': 613.9132535407591, 'discount_factor': 0.6139132535407591}},
                {"q": "What is the present value with fv = 1102.5, discount rate = 5%, years = 2?", "outputs": {"present_value": 1000.0}},
                {"q": "Find the present value with future value = 1000, rate = 5%, periods = -2.", "outcome": "INVALID_INPUT"}])


def _annuity_fv(p, r, n, due):
    f = n if r == 0 else ((1 + r) ** n - 1) / r
    return p * f * ((1 + r) if due else 1)


def _annuity_pv(p, r, n, due):
    f = n if r == 0 else (1 - (1 + r) ** (-n)) / r
    return p * f * ((1 + r) if due else 1)


def future_value_annuity(payment, rate, periods, timing="ordinary"):
    p, r, n, due = _fin("payment", payment), _rate("rate", rate), _int("periods", periods, 1, 100000), _timing(timing)
    return {"future_value": _annuity_fv(p, r, n, due)}


def present_value_annuity(payment, rate, periods, timing="ordinary"):
    p, r, n, due = _fin("payment", payment), _rate("rate", rate), _int("periods", periods, 1, 100000), _timing(timing)
    return {"present_value": _annuity_pv(p, r, n, due)}


def _when(timing):
    return "begin" if timing == "due" else "end"


_ANN_IN = [_i("payment", "payment per period", "pmt", "payment per period", "periodic payment", "c"),
           _i("rate", "interest rate per period (decimal)", "r", "i", "interest rate", "rate per period"),
           _i("periods", "number of payments", "n", "number of periods", "number of payments", "payments"),
           _i("timing", "'ordinary' (end of each period, default) or 'due' (beginning)", "payment timing", "type", **_TIMING)]

_add("future_value_annuity", name="Future value of an annuity", topic="time value of money / annuities",
     purpose="The accumulated value after n periods of equal payments, ordinary (end of period) or due (beginning).",
     inputs=[dict(i, _aliases=list(i["_aliases"])) for i in _ANN_IN],
     outputs=[_o("future_value", "accumulated value at the end of period n")], primary="future_value",
     conditions=["-1 < rate < 1", "periods a whole number >= 1", "timing ordinary or due"],
     method="FV = PMT ((1 + r)^n - 1)/r, times (1 + r) for an annuity due (PMT n when r = 0)",
     latex=r"FV = PMT\,\frac{(1+r)^{n} - 1}{r}\;(\times (1 + r)\ \text{if due})", run=future_value_annuity,
     reference=lambda payment, rate, periods, timing="ordinary": {
         "future_value": float(__import__("numpy_financial").fv(rate, periods, -payment, 0, when=_when(timing)))},
     wrong=lambda payment, rate, periods, timing="ordinary": {"future_value": _annuity_fv(payment, rate, periods, timing != "due")},
     rand=lambda rng: {"payment": _u(rng, 10, 5000, 2), "rate": _u(rng, 0.001, 0.15), "periods": rng.randint(1, 360),
                       "timing": rng.choice(["ordinary", "due"])},
     triggers=["re:\\bfuture value of (?:an? )?(?:ordinary )?annuity(?: due)?\\b", "annuity future value",
               "future value of an ordinary annuity", "future value of a series of payments"],
     templates=["Find the future value of an annuity with payment = 100, rate = 5%, periods = 10.",
                "Find the future value of an annuity due with payment = 100, rate = 5%, periods = 10, timing = due."],
     examples=[{"inputs": {"payment": 100, "rate": 0.05, "periods": 10}, "outputs": {'future_value': 1257.789253554884}},
               {"inputs": {"payment": 100, "rate": 0.05, "periods": 10, "timing": "due"}, "outputs": {'future_value': 1320.6787162326282}},
               {"inputs": {"payment": 100, "rate": 0.0, "periods": 12}, "outputs": {"future_value": 1200.0}}],
     questions=[{"q": "Find the future value of an annuity with payment = 100, rate = 5%, periods = 10.", "outputs": {'future_value': 1257.789253554884}},
                {"q": "Find the future value of an annuity due with payment = 100, rate = 5%, periods = 10, timing = due.", "outputs": {'future_value': 1320.6787162326282}},
                {"q": "Find the future value of an annuity with payment = 100, rate = 5%, periods = 10, timing = sometimes.", "outcome": "INVALID_INPUT"},
                {"q": "Find the future value of an annuity with payment = 100, periods = 10.", "outcome": "NEEDS_INPUT"}])

_add("present_value_annuity", name="Present value of an annuity", topic="time value of money / annuities",
     purpose="The value today of n equal payments, ordinary (end of period) or due (beginning).",
     inputs=[dict(i, _aliases=list(i["_aliases"])) for i in _ANN_IN],
     outputs=[_o("present_value", "value today of the payments")], primary="present_value",
     conditions=["-1 < rate < 1", "periods a whole number >= 1", "timing ordinary or due"],
     method="PV = PMT (1 - (1 + r)^-n)/r, times (1 + r) for an annuity due (PMT n when r = 0)",
     latex=r"PV = PMT\,\frac{1 - (1+r)^{-n}}{r}\;(\times (1 + r)\ \text{if due})", run=present_value_annuity,
     reference=lambda payment, rate, periods, timing="ordinary": {
         "present_value": -float(__import__("numpy_financial").pv(rate, periods, payment, 0, when=_when(timing)))},
     wrong=lambda payment, rate, periods, timing="ordinary": {"present_value": _annuity_pv(payment, rate, periods, timing != "due")},
     rand=lambda rng: {"payment": _u(rng, 10, 5000, 2), "rate": _u(rng, 0.001, 0.15), "periods": rng.randint(1, 360),
                       "timing": rng.choice(["ordinary", "due"])},
     triggers=["re:\\bpresent value of (?:an? )?(?:ordinary )?annuity(?: due)?\\b", "annuity present value",
               "present value of an ordinary annuity", "present value of a series of payments"],
     templates=["Find the present value of an annuity with payment = 100, rate = 5%, periods = 10.",
                "Find the present value of an annuity due with payment = 100, rate = 5%, periods = 10, timing = due."],
     examples=[{"inputs": {"payment": 100, "rate": 0.05, "periods": 10}, "outputs": {'present_value': 772.1734929184818}},
               {"inputs": {"payment": 100, "rate": 0.05, "periods": 10, "timing": "due"}, "outputs": {'present_value': 810.7821675644059}},
               {"inputs": {"payment": 250, "rate": 0.0, "periods": 4}, "outputs": {"present_value": 1000.0}}],
     questions=[{"q": "Find the present value of an annuity with payment = 100, rate = 5%, periods = 10.", "outputs": {'present_value': 772.1734929184818}},
                {"q": "Find the present value of an annuity due with payment = 100, rate = 5%, periods = 10, timing = beginning.", "outputs": {'present_value': 810.7821675644059}},
                {"q": "Find the present value of an annuity with payment = 100, rate = 5%, periods = 10.5.", "outcome": "INVALID_INPUT"}])


def loan_payment(principal, annual_rate, years, payments_per_year=12):
    P, r = _pos("principal", principal), _rate("annual_rate", annual_rate, lo=-1e-300)
    m = _int("payments_per_year", payments_per_year, 1, 365)
    n = _periods_of(_pos("years", years), m)
    i = r / m
    pmt = P / n if i == 0 else P * i / (1 - (1 + i) ** (-n))
    return {"payment": pmt, "number_of_payments": n, "total_paid": pmt * n, "total_interest": pmt * n - P}


def _ref_loan(principal, annual_rate, years, payments_per_year=12):
    import numpy_financial as npf
    n = round(years * payments_per_year)
    p = float(npf.pmt(annual_rate / payments_per_year, n, -principal))
    return {"payment": p, "total_paid": p * n, "total_interest": p * n - principal}


_LOAN_IN = [_i("principal", "amount borrowed", "loan amount", "loan", "amount borrowed", "p", "amount"),
            _i("annual_rate", "nominal annual interest rate (decimal), compounded at the payment frequency", "rate",
               "annual rate", "interest rate", "annual interest rate", "apr", "r"),
            _i("years", "term of the loan in years", "term", "t", "loan term", "term in years"),
            _i("payments_per_year", "payments per year (12 = monthly, default)", "m", "payment frequency",
               "payments a year", type="integer", required=False, min=1)]


def _loan_inputs(extra=()):
    return [dict(i, _aliases=list(i["_aliases"])) for i in _LOAN_IN[:3]] + list(extra) + \
        [dict(_LOAN_IN[3], _aliases=list(_LOAN_IN[3]["_aliases"]))]


def _rand_loan(rng):
    return {"principal": float(rng.choice([rng.randint(1, 500) * 1000, _u(rng, 1000, 50000, 2)])),
            "annual_rate": _u(rng, 0.01, 0.15), "years": rng.choice([1, 2, 3, 5, 10, 15, 20, 25, 30]),
            "payments_per_year": rng.choice([12, 12, 12, 4, 2, 1])}


_add("loan_payment", name="Loan payment (amortizing loan)", topic="loans and annuities / amortization",
     purpose="The level payment that repays a loan over its term, with the total paid and the total interest.",
     inputs=_loan_inputs(),
     outputs=[_o("payment", "level payment per period"), _o("number_of_payments", "number of payments n = years x m"),
              _o("total_paid", "payment x n"), _o("total_interest", "total paid - principal")],
     primary="payment", conditions=["principal > 0", "0 <= annual_rate < 1", "years x payments_per_year a whole number"],
     method="PMT = P i / (1 - (1 + i)^-n), i = annual rate / m, n = years x m", latex=r"PMT = \frac{P\,i}{1 - (1+i)^{-n}}",
     run=loan_payment, reference=_ref_loan,
     wrong=lambda principal, annual_rate, years, payments_per_year=12: (lambda n: (lambda p: {
         "payment": p, "number_of_payments": n, "total_paid": p * n, "total_interest": p * n - principal})(
         principal * annual_rate / (1 - (1 + annual_rate) ** (-n))))(round(years * payments_per_year)),
     rand=_rand_loan,
     triggers=["loan payment", "mortgage payment", "monthly payment", "payment on a loan", "payment on a mortgage",
               "amortizing loan"],
     templates=["Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.",
                "Find the loan payment with principal = 10000, annual rate = 8%, years = 5, payments per year = 4."],
     examples=[{"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30}, "outputs": {"payment": 1199.1010503055138}},
               {"inputs": {"principal": 10000, "annual_rate": 0.08, "years": 5, "payments_per_year": 4}, "outputs": {'payment': 611.5671812529034, 'number_of_payments': 20, 'total_paid': 12231.343625058069, 'total_interest': 2231.343625058069}},
               {"inputs": {"principal": 12000, "annual_rate": 0.0, "years": 1}, "outputs": {"payment": 1000.0}}],
     questions=[{"q": "Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.",
                 "outputs": {"payment": 1199.1010503055138}},
                {"q": "Find the loan payment with principal = 10000, annual rate = 8%, years = 5, payments per year = 4.", "outputs": {'payment': 611.5671812529034, 'number_of_payments': 20, 'total_paid': 12231.343625058069, 'total_interest': 2231.343625058069}},
                {"q": "Find the mortgage payment with loan amount = 300000, interest rate = 7%, term = 15.", "outputs": {'payment': 2696.4848125572744, 'number_of_payments': 180, 'total_paid': 485367.2662603094, 'total_interest': 185367.26626030938}},
                {"q": "Find the loan payment with principal = 10000, annual rate = 8%, years = 2.55.", "outcome": "INVALID_INPUT"},
                {"q": "Find the loan payment with principal = 10000, years = 5.", "outcome": "NEEDS_INPUT"}])


def sinking_fund(future_value, rate, periods, timing="ordinary"):
    F, r, n, due = _pos("future_value", future_value), _rate("rate", rate), _int("periods", periods, 1, 100000), _timing(timing)
    return {"payment": F / _annuity_fv(1.0, r, n, due)}


_add("sinking_fund", name="Sinking fund payment", topic="time value of money / annuities",
     purpose="The equal payment per period that accumulates to a target amount after n periods.",
     inputs=[_i("future_value", "target amount", "fv", "target", "target amount", "goal"),
             _i("rate", "interest rate per period (decimal)", "r", "i", "interest rate", "rate per period"),
             _i("periods", "number of payments", "n", "number of periods", "number of payments"),
             _i("timing", "'ordinary' (end of each period, default) or 'due' (beginning)", "payment timing", **_TIMING)],
     outputs=[_o("payment", "payment per period")], primary="payment",
     conditions=["future_value > 0", "-1 < rate < 1", "periods a whole number >= 1"],
     method="PMT = FV r / ((1 + r)^n - 1), divided by (1 + r) for payments at the beginning",
     latex=r"PMT = \frac{FV\,r}{(1 + r)^{n} - 1}", run=sinking_fund,
     reference=lambda future_value, rate, periods, timing="ordinary": {
         "payment": float(__import__("numpy_financial").pmt(rate, periods, 0, -future_value, when=_when(timing)))},
     wrong=lambda future_value, rate, periods, timing="ordinary": {"payment": future_value * rate / (1 + rate) ** periods},
     rand=lambda rng: {"future_value": _u(rng, 1000, 1000000, 2), "rate": _u(rng, 0.001, 0.15), "periods": rng.randint(1, 360),
                       "timing": rng.choice(["ordinary", "due"])},
     triggers=["sinking fund", "payment to accumulate", "savings needed to reach"],
     templates=["Find the sinking fund payment with future value = 10000, rate = 4%, periods = 5."],
     examples=[{"inputs": {"future_value": 10000, "rate": 0.04, "periods": 5}, "outputs": {'payment': 1846.2711349303381}},
               {"inputs": {"future_value": 1200, "rate": 0.0, "periods": 12}, "outputs": {"payment": 100.0}}],
     questions=[{"q": "Find the sinking fund payment with future value = 10000, rate = 4%, periods = 5.", "outputs": {'payment': 1846.2711349303381}},
                {"q": "Find the sinking fund payment with target = 10000, rate = 4%, periods = 5, timing = due.", "outputs": {'payment': 1775.2607066637868}}])


def number_of_periods(present_value, payment, rate, timing="ordinary"):
    pv, p, r, due = _pos("present_value", present_value), _pos("payment", payment), _rate("rate", rate, lo=0.0), _timing(timing)
    eff = p * (1 + r) if due else p
    _need(eff > pv * r, "the payment does not even cover the interest: the loan is never repaid")
    return {"periods": -math.log(1 - pv * r / eff) / math.log(1 + r)}


_add("number_of_periods", name="Number of payments to repay a loan", topic="loans and annuities / amortization",
     purpose="How many periodic payments repay a present value at a rate per period (a fractional answer means the "
             "last payment is smaller).",
     inputs=[_i("present_value", "amount borrowed (present value)", "pv", "principal", "loan amount", "loan"),
             _i("payment", "payment per period", "pmt", "periodic payment", "payment per period"),
             _i("rate", "interest rate per period (decimal)", "r", "i", "interest rate", "rate per period"),
             _i("timing", "'ordinary' (end of each period, default) or 'due' (beginning)", "payment timing", **_TIMING)],
     outputs=[_o("periods", "number of periods n (may be fractional)")], primary="periods",
     conditions=["present_value > 0", "payment > 0", "0 < rate < 1", "payment larger than the interest of one period"],
     method="n = -ln(1 - PV r / PMT) / ln(1 + r) (PMT (1 + r) for an annuity due)",
     latex=r"n = \frac{-\ln\left(1 - \frac{PV\,r}{PMT}\right)}{\ln(1 + r)}", run=number_of_periods,
     reference=lambda present_value, payment, rate, timing="ordinary": {
         "periods": float(__import__("numpy_financial").nper(rate, -payment, present_value, 0, when=_when(timing)))},
     wrong=lambda present_value, payment, rate, timing="ordinary": {
         "periods": -math.log(1 - present_value * rate / (payment * ((1 + rate) if timing == "due" else 1))) / rate},
     rand=lambda rng: (lambda pv, r: {"present_value": pv, "rate": r, "payment": round(pv * r * rng.uniform(1.05, 4) + 1, 2),
                                      "timing": rng.choice(["ordinary", "due"])})(_u(rng, 1000, 300000, 2), _u(rng, 0.002, 0.12)),
     triggers=["number of payments", "number of periods", "how many payments", "how many periods", "periods to repay",
               "payments to repay", "time to repay"],
     templates=["Find the number of payments with present value = 10000, payment = 500, rate = 1%."],
     examples=[{"inputs": {"present_value": 10000, "payment": 500, "rate": 0.01}, "outputs": {'periods': 22.425741878036437}},
               {"inputs": {"present_value": 10000, "payment": 500, "rate": 0.01, "timing": "due"}, "outputs": {'periods': 22.177288918846447}}],
     questions=[{"q": "Find the number of payments with present value = 10000, payment = 500, rate = 1%.", "outputs": {'periods': 22.425741878036437}},
                {"q": "How many payments with loan amount = 10000, payment = 50, rate = 1%?", "outcome": "INVALID_INPUT"}])


def annuity_rate(present_value, payment, periods, timing="ordinary"):
    pv, p, n, due = _pos("present_value", present_value), _pos("payment", payment), _int("periods", periods, 1, 100000), _timing(timing)
    def f(r):
        try:
            return _annuity_pv(p, r, n, due) - pv
        except OverflowError:
            return math.inf
    lo, hi = -0.99, 0.99
    _need(f(lo) > 0 > f(hi), "no rate between -99% and 99% per period gives this present value")
    for _ in range(200):
        mid = (lo + hi) / 2
        if mid in (lo, hi):
            break
        if f(mid) > 0:
            lo = mid
        else:
            hi = mid
    return {"rate": (lo + hi) / 2}


def _ref_annuity_rate(present_value, payment, periods, timing="ordinary"):
    import numpy_financial as npf
    from scipy.optimize import brentq
    w = _when(timing)
    return {"rate": brentq(lambda r: float(npf.pv(r, periods, -payment, 0, when=w)) - present_value, -0.5, 0.99,
                           xtol=1e-16, rtol=1e-15, maxiter=500)}


_add("annuity_rate", name="Interest rate of an annuity", topic="time value of money / annuities",
     purpose="The rate per period at which n equal payments are worth a given present value (bisection; the rate is "
             "unique because the present value falls as the rate rises).",
     inputs=[_i("present_value", "present value (amount borrowed or price paid)", "pv", "principal", "loan amount", "price"),
             _i("payment", "payment per period", "pmt", "periodic payment", "payment per period"),
             _i("periods", "number of payments", "n", "number of periods", "number of payments"),
             _i("timing", "'ordinary' (end of each period, default) or 'due' (beginning)", "payment timing", **_TIMING)],
     outputs=[_o("rate", "interest rate per period")], primary="rate",
     conditions=["present_value > 0", "payment > 0", "periods a whole number >= 1", "the rate lies between -99% and 99%"],
     method="bisection on PMT (1 - (1 + r)^-n)/r = PV", latex=r"PV = PMT\,\frac{1 - (1+r)^{-n}}{r}",
     run=annuity_rate, reference=_ref_annuity_rate,
     wrong=lambda present_value, payment, periods, timing="ordinary": annuity_rate(
         present_value, payment, periods, "ordinary" if timing == "due" else "due"),
     rand=lambda rng: (lambda p, r, n, t: {"payment": p, "periods": n, "timing": t,
                                           "present_value": round(_annuity_pv(p, r, n, t == "due"), 2)})(
         _u(rng, 50, 5000, 2), _u(rng, 0.002, 0.2), rng.randint(2, 360), rng.choice(["ordinary", "due"])),
     triggers=["rate of an annuity", "interest rate of an annuity", "annuity rate", "implied interest rate",
               "rate per period of an annuity"],
     templates=["Find the interest rate of an annuity with present value = 1000, payment = 263.80, periods = 4."],
     examples=[{"inputs": {"present_value": 1000, "payment": 263.8, "periods": 4}, "outputs": {'rate': 0.021844019077514466}},
               {"inputs": {"present_value": 386.0867, "payment": 100, "periods": 5, "timing": "ordinary"}, "outputs": {'rate': 0.09286908941404498}}],
     questions=[{"q": "Find the interest rate of an annuity with present value = 1000, payment = 263.80, periods = 4.", "outputs": {'rate': 0.021844019077514466}},
                {"q": "Find the interest rate of an annuity with present value = 1000, payment = 1, periods = 1.", "outcome": "INVALID_INPUT"}])


def perpetuity(payment, rate, timing="ordinary"):
    p, r, due = _fin("payment", payment), _rate("rate", rate, lo=0.0), _timing(timing)
    return {"present_value": p / r * ((1 + r) if due else 1)}


def _ref_perpetuity(payment, rate, timing="ordinary"):
    import mpmath
    with mpmath.workdps(30):
        start = 0 if timing == "due" else 1
        return {"present_value": float(mpmath.nsum(lambda k: payment / (1 + mpmath.mpf(rate)) ** k, [start, mpmath.inf]))}


_add("perpetuity", name="Present value of a perpetuity", topic="time value of money / perpetuities",
     purpose="The value today of an equal payment every period forever: PV = C / r (C / r + C if payments start now).",
     inputs=[_i("payment", "payment per period", "c", "pmt", "cash flow", "payment per period"),
             _i("rate", "discount rate per period (decimal)", "r", "i", "discount rate", "interest rate"),
             _i("timing", "'ordinary' (first payment in one period, default) or 'due' (first payment now)", "payment timing", **_TIMING)],
     outputs=[_o("present_value", "value today")], primary="present_value", conditions=["0 < rate < 1"],
     method="PV = C / r, times (1 + r) for a perpetuity due", latex=r"PV = \frac{C}{r}", run=perpetuity,
     reference=_ref_perpetuity,
     wrong=lambda payment, rate, timing="ordinary": {"present_value": payment / rate * (1 if timing == "due" else 1 + rate)},
     rand=lambda rng: {"payment": _u(rng, 1, 10000, 2), "rate": _u(rng, 0.01, 0.3), "timing": rng.choice(["ordinary", "due"])},
     triggers=["re:\\b(?:present value of (?:an? )?)?perpetuity(?: due)?\\b"],
     templates=["Find the present value of a perpetuity with payment = 100, rate = 5%."],
     examples=[{"inputs": {"payment": 100, "rate": 0.05}, "outputs": {"present_value": 2000.0}},
               {"inputs": {"payment": 100, "rate": 0.05, "timing": "due"}, "outputs": {"present_value": 2100.0}}],
     questions=[{"q": "Find the present value of a perpetuity with payment = 100, rate = 5%.", "outputs": {"present_value": 2000.0}},
                {"q": "Find the present value of a perpetuity due with payment = 100, rate = 5%, timing = due.", "outputs": {"present_value": 2100.0}},
                {"q": "Find the present value of a perpetuity with payment = 100, rate = 0.", "outcome": "INVALID_INPUT"}])


def growing_perpetuity(first_payment, rate, growth):
    c, r, g = _fin("first_payment", first_payment), _rate("rate", rate, lo=0.0), _rate("growth", growth)
    _need(r > g, "the discount rate must exceed the growth rate (otherwise the value is infinite)")
    return {"present_value": c / (r - g)}


def _ref_growing_perpetuity(first_payment, rate, growth):
    import mpmath
    with mpmath.workdps(30):
        r, g = mpmath.mpf(rate), mpmath.mpf(growth)
        return {"present_value": float(mpmath.nsum(lambda k: first_payment * (1 + g) ** (k - 1) / (1 + r) ** k, [1, mpmath.inf]))}


_add("growing_perpetuity", name="Present value of a growing perpetuity", topic="time value of money / perpetuities",
     purpose="The value today of payments that start at C one period from now and grow at g per period forever: C / (r - g).",
     inputs=[_i("first_payment", "payment one period from now", "c1", "c", "first cash flow", "payment", "next payment"),
             _i("rate", "discount rate per period (decimal)", "r", "discount rate", "required return"),
             _i("growth", "growth rate per period (decimal)", "g", "growth rate")],
     outputs=[_o("present_value", "value today C1 / (r - g)")], primary="present_value", conditions=["0 < rate < 1", "growth < rate"],
     method="PV = C1 / (r - g)", latex=r"PV = \frac{C_1}{r - g}", run=growing_perpetuity, reference=_ref_growing_perpetuity,
     wrong=lambda first_payment, rate, growth: {"present_value": first_payment * (1 + growth) / (rate - growth)},
     rand=lambda rng: (lambda r: {"first_payment": _u(rng, 1, 10000, 2), "rate": r, "growth": round(r - rng.uniform(0.02, 0.15), 4)})(
         _u(rng, 0.05, 0.3)),
     triggers=["growing perpetuity"],
     templates=["Find the present value of a growing perpetuity with first payment = 100, rate = 8%, growth = 3%."],
     examples=[{"inputs": {"first_payment": 100, "rate": 0.08, "growth": 0.03}, "outputs": {"present_value": 2000.0}},
               {"inputs": {"first_payment": 50, "rate": 0.1, "growth": -0.02}, "outputs": {'present_value': 416.66666666666663}}],
     questions=[{"q": "Find the present value of a growing perpetuity with first payment = 100, rate = 8%, growth = 3%.",
                 "outputs": {"present_value": 2000.0}},
                {"q": "Find the present value of a growing perpetuity with first payment = 100, rate = 3%, growth = 8%.", "outcome": "INVALID_INPUT"}])


def growing_annuity(first_payment, rate, growth, periods):
    c, r, g, n = _fin("first_payment", first_payment), _rate("rate", rate), _rate("growth", growth), _int("periods", periods, 1, 100000)
    pv = c * n / (1 + r) if abs(r - g) < 1e-15 else c / (r - g) * (1 - ((1 + g) / (1 + r)) ** n)
    return {"present_value": pv, "future_value": pv * (1 + r) ** n}


def _ref_growing_annuity(first_payment, rate, growth, periods):
    import numpy as np
    k = np.arange(1, periods + 1, dtype=float)
    flows = first_payment * (1 + growth) ** (k - 1)
    return {"present_value": float(np.sum(flows / (1 + rate) ** k)),
            "future_value": float(np.sum(flows * (1 + rate) ** (periods - k)))}


_add("growing_annuity", name="Present and future value of a growing annuity", topic="time value of money / annuities",
     purpose="n payments at the end of each period, the first C1, each growing at g per period.",
     inputs=[_i("first_payment", "first payment (end of period 1)", "c1", "c", "first cash flow", "payment"),
             _i("rate", "discount rate per period (decimal)", "r", "discount rate", "interest rate"),
             _i("growth", "growth rate per period (decimal)", "g", "growth rate"),
             _i("periods", "number of payments", "n", "number of periods", "number of payments", type="integer", min=1)],
     outputs=[_o("present_value", "value today"), _o("future_value", "value at the end of period n")], primary="present_value",
     conditions=["-1 < rate < 1", "-1 < growth < 1", "periods a whole number >= 1"],
     method="PV = C1/(r - g) (1 - ((1 + g)/(1 + r))^n) (n C1/(1 + r) when r = g); FV = PV (1 + r)^n",
     latex=r"PV = \frac{C_1}{r - g}\left[1 - \left(\frac{1+g}{1+r}\right)^{n}\right]", run=growing_annuity,
     reference=_ref_growing_annuity,
     wrong=lambda first_payment, rate, growth, periods: (lambda pv: {"present_value": pv, "future_value": pv * (1 + rate) ** periods})(
         first_payment / (rate - growth) * (1 - ((1 + growth) / (1 + rate)) ** (periods - 1)) if rate != growth else first_payment * periods),
     rand=lambda rng: {"first_payment": _u(rng, 10, 10000, 2), "rate": _u(rng, 0.01, 0.2), "growth": _u(rng, -0.05, 0.15),
                       "periods": rng.randint(2, 60)},
     triggers=["growing annuity"],
     templates=["Find the present value of a growing annuity with first payment = 1000, rate = 8%, growth = 3%, periods = 10."],
     examples=[{"inputs": {"first_payment": 1000, "rate": 0.08, "growth": 0.03, "periods": 10}, "outputs": {'present_value': 7550.133691149122, 'future_value': 16300.172358573303}},
               {"inputs": {"first_payment": 100, "rate": 0.05, "growth": 0.05, "periods": 3}, "outputs": {'present_value': 285.7142857142857, 'future_value': 330.75000000000006}}],
     questions=[{"q": "Find the present value of a growing annuity with first payment = 1000, rate = 8%, growth = 3%, periods = 10.", "outputs": {'present_value': 7550.133691149122, 'future_value': 16300.172358573303}}])


def doubling_time(rate):
    r = _rate("rate", rate, lo=0.0)
    return {"periods": math.log(2) / math.log(1 + r), "rule_of_72": 72 / (100 * r)}


_add("doubling_time", name="Doubling time", topic="time value of money / interest",
     purpose="How many periods an amount compounding at r per period takes to double, with the rule-of-72 estimate.",
     inputs=[_i("rate", "interest or growth rate per period (decimal)", "r", "interest rate", "growth rate", "annual rate")],
     outputs=[_o("periods", "exact doubling time ln 2 / ln(1 + r)"), _o("rule_of_72", "rule-of-72 estimate 72 / (100 r)")],
     primary="periods", conditions=["0 < rate < 1"], method="n = ln 2 / ln(1 + r)",
     latex=r"n = \frac{\ln 2}{\ln(1 + r)}", run=doubling_time,
     reference=lambda rate: {"periods": float(__import__("numpy_financial").nper(rate, 0, -1, 2)), "rule_of_72": 72 / (100 * rate)},
     wrong=lambda rate: {"periods": math.log(2) / rate, "rule_of_72": 72 / (100 * rate)},
     rand=lambda rng: {"rate": _u(rng, 0.005, 0.5)},
     triggers=["doubling time", "time to double", "years to double", "periods to double", "rule of seventy-two",
               "how long to double"],
     templates=["Find the doubling time with rate = 6%."],
     examples=[{"inputs": {"rate": 0.06}, "outputs": {'periods': 11.895661045941875, 'rule_of_72': 12.0}}, {"inputs": {"rate": 1 / 9}, "outputs": {'periods': 6.578813478960581, 'rule_of_72': 6.48}}],
     questions=[{"q": "Find the doubling time with rate = 6%.", "outputs": {'periods': 11.895661045941875, 'rule_of_72': 12.0}},
                {"q": "Find the doubling time with rate = -3%.", "outcome": "INVALID_INPUT"}])


def cagr(begin_value, end_value, years):
    b, e, t = _pos("begin_value", begin_value), _pos("end_value", end_value), _pos("years", years)
    return {"cagr": (e / b) ** (1 / t) - 1, "total_growth": e / b - 1}


def _ref_cagr(begin_value, end_value, years):
    import numpy_financial as npf
    from scipy.optimize import brentq
    return {"cagr": brentq(lambda x: float(npf.fv(x, years, 0, -begin_value)) - end_value, -0.999999, 1000,
                           xtol=1e-16, rtol=1e-15, maxiter=1000)}


_add("cagr", name="Compound annual growth rate", topic="time value of money / growth rates",
     purpose="The constant annual growth rate that takes a beginning value to an ending value over a number of years.",
     inputs=[_i("begin_value", "value at the start", "beginning value", "initial value", "start value", "start"),
             _i("end_value", "value at the end", "ending value", "final value", "end"),
             _i("years", "number of years", "t", "n", "periods")],
     outputs=[_o("cagr", "compound annual growth rate (end/begin)^(1/t) - 1"), _o("total_growth", "end/begin - 1")],
     primary="cagr", conditions=["begin_value > 0", "end_value > 0", "years > 0"], method="CAGR = (end/begin)^(1/t) - 1",
     latex=r"\mathrm{CAGR} = \left(\frac{V_{end}}{V_{begin}}\right)^{1/t} - 1", run=cagr, reference=_ref_cagr,
     wrong=lambda begin_value, end_value, years: {"cagr": (end_value / begin_value - 1) / years, "total_growth": end_value / begin_value - 1},
     rand=lambda rng: {"begin_value": _u(rng, 100, 100000, 2), "end_value": _u(rng, 50, 500000, 2), "years": rng.randint(2, 30)},
     triggers=["compound annual growth rate", "cagr"],
     templates=["Find the compound annual growth rate with begin value = 1000, end value = 2000, years = 5."],
     examples=[{"inputs": {"begin_value": 1000, "end_value": 1210, "years": 2}, "outputs": {"cagr": 0.1}},
               {"inputs": {"begin_value": 1000, "end_value": 2000, "years": 5}, "outputs": {'cagr': 0.1486983549970351, 'total_growth': 1.0}}],
     questions=[{"q": "Find the compound annual growth rate with begin value = 1000, end value = 2000, years = 5.", "outputs": {'cagr': 0.1486983549970351, 'total_growth': 1.0}},
                {"q": "Find the CAGR with initial value = 1000, final value = 1210, years = 2.", "outputs": {"cagr": 0.1}},
                {"q": "Find the CAGR with initial value = 0, final value = 1210, years = 2.", "outcome": "INVALID_INPUT"}])


def _loan_state(principal, annual_rate, years, payments_per_year):
    P, r = _pos("principal", principal), _rate("annual_rate", annual_rate, lo=-1e-300)
    m = _int("payments_per_year", payments_per_year, 1, 365)
    n = _periods_of(_pos("years", years), m)
    i = r / m
    pmt = P / n if i == 0 else P * i / (1 - (1 + i) ** (-n))
    return P, i, n, pmt


def _balance(P, i, pmt, k):
    return P - pmt * k if i == 0 else P * (1 + i) ** k - pmt * ((1 + i) ** k - 1) / i


def loan_balance(principal, annual_rate, years, payments_made, payments_per_year=12):
    P, i, n, pmt = _loan_state(principal, annual_rate, years, payments_per_year)
    k = _int("payments_made", payments_made, 0, n)
    return {"balance": max(0.0, _balance(P, i, pmt, k)) if k < n else 0.0, "payment": pmt}


def _ref_balance(principal, annual_rate, years, payments_made, payments_per_year=12):
    import numpy_financial as npf
    n, i = round(years * payments_per_year), annual_rate / payments_per_year
    p = float(npf.pmt(i, n, principal))
    b = -float(npf.fv(i, payments_made, p, principal)) if payments_made < n else 0.0
    return {"balance": b, "payment": -p}


def _rand_loan_k(rng, lo=0):
    x = _rand_loan(rng)
    n = round(x["years"] * x["payments_per_year"])
    x["payments_made"] = rng.randint(max(lo, 0), max(lo, n - 1))
    return x


_add("loan_balance", name="Remaining balance of a loan", topic="loans and annuities / amortization",
     purpose="The balance still owed on an amortizing loan after k level payments.",
     inputs=_loan_inputs([_i("payments_made", "number of payments already made", "k", "payments made", "number of payments made",
                             "after payments", type="integer", min=0)]),
     outputs=[_o("balance", "balance owed after k payments"), _o("payment", "level payment per period")], primary="balance",
     conditions=["principal > 0", "0 <= annual_rate < 1", "0 <= payments_made <= number of payments"],
     method="B_k = P (1 + i)^k - PMT ((1 + i)^k - 1)/i", latex=r"B_k = P(1+i)^{k} - PMT\,\frac{(1+i)^{k} - 1}{i}",
     run=loan_balance, reference=_ref_balance,
     wrong=lambda principal, annual_rate, years, payments_made, payments_per_year=12: (
         lambda s: {"balance": s[0] - s[3] * payments_made, "payment": s[3]})(_loan_state(principal, annual_rate, years, payments_per_year)),
     rand=lambda rng: _rand_loan_k(rng, 1),
     triggers=["remaining balance", "outstanding balance", "loan balance", "balance owed", "balance remaining"],
     templates=["Find the remaining balance with principal = 200000, annual rate = 6%, years = 30, payments made = 60."],
     examples=[{"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payments_made": 60}, "outputs": {'balance': 186108.7136456388, 'payment': 1199.1010503055138}},
               {"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payments_made": 360}, "outputs": {"balance": 0.0}},
               {"inputs": {"principal": 12000, "annual_rate": 0.0, "years": 1, "payments_made": 3}, "outputs": {"balance": 9000.0}}],
     questions=[{"q": "Find the remaining balance with principal = 200000, annual rate = 6%, years = 30, payments made = 60.", "outputs": {'balance': 186108.7136456388, 'payment': 1199.1010503055138}},
                {"q": "Find the remaining balance with principal = 200000, annual rate = 6%, years = 30, payments made = 400.", "outcome": "INVALID_INPUT"}])


def loan_payment_split(principal, annual_rate, years, payment_number, payments_per_year=12):
    P, i, n, pmt = _loan_state(principal, annual_rate, years, payments_per_year)
    k = _int("payment_number", payment_number, 1, n)
    interest = _balance(P, i, pmt, k - 1) * i
    return {"interest_part": interest, "principal_part": pmt - interest, "payment": pmt}


def _ref_split(principal, annual_rate, years, payment_number, payments_per_year=12):
    import numpy_financial as npf
    n, i = round(years * payments_per_year), annual_rate / payments_per_year
    return {"interest_part": -float(npf.ipmt(i, payment_number, n, principal)),
            "principal_part": -float(npf.ppmt(i, payment_number, n, principal))}


_add("loan_payment_split", name="Interest and principal in a loan payment", topic="loans and annuities / amortization",
     purpose="How payment number k of an amortizing loan splits into interest and principal repayment.",
     inputs=_loan_inputs([_i("payment_number", "which payment (1 = the first)", "k", "payment number", "period", "payment",
                             type="integer", min=1)]),
     outputs=[_o("interest_part", "interest in payment k"), _o("principal_part", "principal repaid by payment k"),
              _o("payment", "level payment")], primary="interest_part",
     conditions=["principal > 0", "0 <= annual_rate < 1", "1 <= payment_number <= number of payments"],
     method="interest = i B_(k-1), principal = PMT - interest", latex=r"I_k = i\,B_{k-1},\quad P_k = PMT - I_k",
     run=loan_payment_split, reference=_ref_split,
     wrong=lambda principal, annual_rate, years, payment_number, payments_per_year=12: (
         lambda s: {"interest_part": s[0] * s[1], "principal_part": s[3] - s[0] * s[1], "payment": s[3]})(
         _loan_state(principal, annual_rate, years, payments_per_year)),
     rand=lambda rng: (lambda x: dict({k: v for k, v in x.items() if k != "payments_made"}, payment_number=x["payments_made"] + 1))(
         _rand_loan_k(rng, 1)),
     triggers=["interest portion", "principal portion", "interest part", "principal part", "interest and principal",
               "split of payment", "interest in payment"],
     templates=["Find the interest and principal in payment number = 12 with principal = 200000, annual rate = 6%, years = 30."],
     examples=[{"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payment_number": 1},
                "outputs": {"interest_part": 1000.0, "principal_part": 199.10105030551382}},
               {"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payment_number": 12}, "outputs": {'interest_part': 988.7715304763783, 'principal_part': 210.3295198291355, 'payment': 1199.1010503055138}}],
     questions=[{"q": "Find the interest and principal in payment number = 12 with principal = 200000, annual rate = 6%, years = 30.", "outputs": {'interest_part': 988.7715304763783, 'principal_part': 210.3295198291355, 'payment': 1199.1010503055138}},
                {"q": "Find the interest portion with principal = 200000, annual rate = 6%, years = 30, payment number = 1.",
                 "outputs": {"interest_part": 1000.0}},
                {"q": "Find the interest portion with principal = 200000, annual rate = 6%, years = 30, payment number = 0.", "outcome": "INVALID_INPUT"}])


def loan_cumulative_interest(principal, annual_rate, years, payments_made, payments_per_year=12):
    P, i, n, pmt = _loan_state(principal, annual_rate, years, payments_per_year)
    k = _int("payments_made", payments_made, 1, n)
    repaid = P - (_balance(P, i, pmt, k) if k < n else 0.0)
    return {"cumulative_interest": pmt * k - repaid, "principal_repaid": repaid}


def _ref_cum(principal, annual_rate, years, payments_made, payments_per_year=12):
    import numpy as np
    import numpy_financial as npf
    n, i = round(years * payments_per_year), annual_rate / payments_per_year
    per = np.arange(1, payments_made + 1)
    return {"cumulative_interest": -float(np.sum(npf.ipmt(i, per, n, principal))),
            "principal_repaid": -float(np.sum(npf.ppmt(i, per, n, principal)))}


_add("loan_cumulative_interest", name="Cumulative interest of a loan", topic="loans and annuities / amortization",
     purpose="Total interest paid and principal repaid over the first k payments of an amortizing loan.",
     inputs=_loan_inputs([_i("payments_made", "number of payments counted (the first k)", "k", "payments made",
                             "number of payments made", "first payments", type="integer", min=1)]),
     outputs=[_o("cumulative_interest", "interest paid in payments 1..k"), _o("principal_repaid", "principal repaid in payments 1..k")],
     primary="cumulative_interest", conditions=["principal > 0", "0 <= annual_rate < 1", "1 <= payments_made <= number of payments"],
     method="principal repaid = P - B_k; interest = k PMT - (P - B_k)", latex=r"\sum_{j=1}^{k} I_j = k\,PMT - (P - B_k)",
     run=loan_cumulative_interest, reference=_ref_cum,
     wrong=lambda principal, annual_rate, years, payments_made, payments_per_year=12: (
         lambda s: {"cumulative_interest": payments_made * s[0] * s[1], "principal_repaid": payments_made * (s[3] - s[0] * s[1])})(
         _loan_state(principal, annual_rate, years, payments_per_year)),
     rand=lambda rng: _rand_loan_k(rng, 1),
     triggers=["cumulative interest", "total interest paid in the first", "interest paid over the first", "interest paid in the first"],
     templates=["Find the cumulative interest with principal = 200000, annual rate = 6%, years = 30, payments made = 12."],
     examples=[{"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payments_made": 12}, "outputs": {'cumulative_interest': 11933.189179112655, 'principal_repaid': 2456.0234245535103}},
               {"inputs": {"principal": 200000, "annual_rate": 0.06, "years": 30, "payments_made": 1}, "outputs": {"cumulative_interest": 1000.0}}],
     questions=[{"q": "Find the cumulative interest with principal = 200000, annual rate = 6%, years = 30, payments made = 12.", "outputs": {'cumulative_interest': 11933.189179112655, 'principal_repaid': 2456.0234245535103}}])


# =====================================================================================================================
# CAPITAL BUDGETING
# =====================================================================================================================
_CF = dict(type="vector", min_length=2)


def npv(rate, cash_flows):
    r, c = _rate("rate", rate), _vec("cash_flows", cash_flows, 2)
    return {"npv": _hnpv(r, c)}


def _check_npv(rate, cash_flows):
    return {"npv": sum(c / (1 + rate) ** t for t, c in enumerate(cash_flows))}


_add("npv", name="Net present value", topic="capital budgeting / discounted cash flow",
     purpose="NPV of cash flows [CF0, CF1, ..., CFn] (CF0 now, CFt at the end of period t) at a discount rate per period.",
     inputs=[_i("rate", "discount rate per period (decimal)", "r", "discount rate", "cost of capital", "required return", "hurdle rate"),
             _i("cash_flows", "cash flows [CF0, CF1, ..., CFn], CF0 at time 0", "cash flow", "flows", "cf", "cfs", **_CF)],
     outputs=[_o("npv", "net present value")], primary="npv", conditions=["-1 < rate < 1", "at least two cash flows"],
     method="NPV = sum CFt / (1 + r)^t, t = 0..n", latex=r"NPV = \sum_{t=0}^{n} \frac{CF_t}{(1 + r)^{t}}", run=npv,
     check=_check_npv, reference=lambda rate, cash_flows: {"npv": float(__import__("numpy_financial").npv(rate, cash_flows))},
     wrong=lambda rate, cash_flows: {"npv": sum(c / (1 + rate) ** (t + 1) for t, c in enumerate(cash_flows))},
     rand=lambda rng: {"rate": _u(rng, 0.01, 0.25),
                       "cash_flows": [-_u(rng, 500, 100000, 2)] + [_u(rng, -2000, 40000, 2) for _ in range(rng.randint(1, 15))]},
     triggers=["net present value", "npv"],
     templates=["Find the net present value with rate = 10%, cash flows = [-1000, 300, 400, 500]."],
     examples=[{"inputs": {"rate": 0.1, "cash_flows": "-1000, 300, 400, 500"}, "outputs": {'npv': -21.036814425244188}},
               {"inputs": {"rate": 0.0, "cash_flows": [-100, 50, 60]}, "outputs": {"npv": 10.0}}],
     questions=[{"q": "Find the net present value with rate = 10%, cash flows = [-1000, 300, 400, 500].", "outputs": {'npv': -21.036814425244188}},
                {"q": "What is the NPV with discount rate = 0%, cash flows = [-100, 50, 60]?", "outputs": {"npv": 10.0}},
                {"q": "Find the NPV with rate = 10%, cash flows = [-1000].", "outcome": "INVALID_INPUT"},
                {"q": "Find the NPV with rate = 10%, cash flows = -1000, 300, 400.", "outcome": "UNRECOGNIZED_FORMAT"},
                {"q": "Find the NPV with cash flows = [-1000, 300, 400, 500].", "outcome": "NEEDS_INPUT"}])


def irr(cash_flows):
    return {"irr": _irr(_vec("cash_flows", cash_flows, 2))}


def _verify_irr(inputs, outputs):
    """Not a recomputation: the NPV at the reported rate must vanish (relative to the size of the discounted flows)."""
    r = outputs.get("irr")
    if not isinstance(r, (int, float)) or not math.isfinite(r) or r <= -1:
        return math.inf
    c = inputs["cash_flows"]
    return abs(_hnpv(r, c)) / _npv_scale(r, c)


_add("irr", name="Internal rate of return", topic="capital budgeting / discounted cash flow",
     purpose="The discount rate at which the NPV of [CF0, ..., CFn] is zero; given only when it is unique (the cash "
             "flows change sign exactly once).",
     inputs=[_i("cash_flows", "cash flows [CF0, CF1, ..., CFn], CF0 at time 0", "cash flow", "flows", "cf", "cfs", **_CF)],
     outputs=[_o("irr", "internal rate of return per period")], primary="irr",
     conditions=["the cash flows change sign exactly once (otherwise the IRR is not unique and none is given)"],
     method="bisection on v = 1/(1 + r) for the unique positive root of sum CFt v^t, Newton polishing",
     latex=r"\sum_{t=0}^{n} \frac{CF_t}{(1 + \mathrm{IRR})^{t}} = 0", run=irr,
     reference=lambda cash_flows: {"irr": float(__import__("numpy_financial").irr(cash_flows))},
     wrong=lambda cash_flows: {"irr": (sum(cash_flows[1:]) / -cash_flows[0] - 1) / (len(cash_flows) - 1)},
     rand=lambda rng: {"cash_flows": [-_u(rng, 500, 50000, 2)] + [rng.choice([0.0, _u(rng, 50, 20000, 2), _u(rng, 50, 20000, 2)])
                                                               for _ in range(rng.randint(1, 12))] + [_u(rng, 50, 20000, 2)]},
     triggers=["internal rate of return", "irr"],
     templates=["Find the internal rate of return with cash flows = [-1000, 300, 400, 500]."],
     examples=[{"inputs": {"cash_flows": "-1000, 300, 400, 500"}, "outputs": {'irr': 0.08896339469334996}},
               {"inputs": {"cash_flows": [-100, 110]}, "outputs": {"irr": 0.1}},
               {"inputs": {"cash_flows": [-1000, 0, 1210]}, "outputs": {"irr": 0.1}}],
     questions=[{"q": "Find the internal rate of return with cash flows = [-1000, 300, 400, 500].", "outputs": {'irr': 0.08896339469334996}},
                {"q": "What is the IRR of the cash flows = [-100, 110]?", "outputs": {"irr": 0.1}},
                {"q": "Find the IRR with cash flows = [-100, 230, -132].", "outcome": "INVALID_INPUT"},
                {"q": "Find the IRR with cash flows = [100, 200, 300].", "outcome": "INVALID_INPUT"}])
PROCEDURES["irr"]["verify"] = _verify_irr


def mirr(cash_flows, finance_rate, reinvest_rate):
    c = _vec("cash_flows", cash_flows, 2)
    f, rr = _rate("finance_rate", finance_rate), _rate("reinvest_rate", reinvest_rate)
    _need(any(x > 0 for x in c) and any(x < 0 for x in c), "the cash flows need at least one inflow and one outflow")
    n = len(c) - 1
    fv_pos = sum(x * (1 + rr) ** (n - t) for t, x in enumerate(c) if x > 0)
    pv_neg = -sum(x / (1 + f) ** t for t, x in enumerate(c) if x < 0)
    return {"mirr": (fv_pos / pv_neg) ** (1 / n) - 1, "terminal_value": fv_pos, "pv_outflows": pv_neg}


_add("mirr", name="Modified internal rate of return", topic="capital budgeting / discounted cash flow",
     purpose="MIRR: outflows discounted at the finance rate, inflows compounded to the end at the reinvestment rate.",
     inputs=[_i("cash_flows", "cash flows [CF0, ..., CFn], CF0 at time 0", "cash flow", "flows", "cf", **_CF),
             _i("finance_rate", "finance rate for outflows (decimal)", "finance rate", "cost of capital", "borrowing rate"),
             _i("reinvest_rate", "reinvestment rate for inflows (decimal)", "reinvestment rate", "reinvest rate")],
     outputs=[_o("mirr", "modified IRR per period"), _o("terminal_value", "inflows compounded to period n"),
              _o("pv_outflows", "present value of the outflows")], primary="mirr",
     conditions=["at least one inflow and one outflow", "-1 < rates < 1"],
     method="MIRR = (FV of inflows at the reinvestment rate / PV of outflows at the finance rate)^(1/n) - 1",
     latex=r"\mathrm{MIRR} = \left(\frac{FV_{+}}{PV_{-}}\right)^{1/n} - 1", run=mirr,
     reference=lambda cash_flows, finance_rate, reinvest_rate: {
         "mirr": float(__import__("numpy_financial").mirr(cash_flows, finance_rate, reinvest_rate))},
     wrong=lambda cash_flows, finance_rate, reinvest_rate: (lambda o: dict(o, mirr=(o["terminal_value"] / o["pv_outflows"]) ** (
         1 / len(cash_flows)) - 1))(mirr(cash_flows, finance_rate, reinvest_rate)),
     rand=lambda rng: {"cash_flows": [-_u(rng, 500, 50000, 2)] + [_u(rng, -3000, 20000, 2) for _ in range(rng.randint(1, 12))]
                       + [_u(rng, 100, 20000, 2)], "finance_rate": _u(rng, 0.01, 0.2), "reinvest_rate": _u(rng, 0.01, 0.2)},
     triggers=["modified internal rate of return", "mirr"],
     templates=["Find the modified internal rate of return with cash flows = [-1000, 300, 400, 500], finance rate = 10%, reinvestment rate = 12%."],
     examples=[{"inputs": {"cash_flows": "-1000, 300, 400, 500", "finance_rate": 0.1, "reinvest_rate": 0.12}, "outputs": {'mirr': 0.09815669244631553, 'terminal_value': 1324.3200000000002, 'pv_outflows': 1000.0}},
               {"inputs": {"cash_flows": [-100, 0, 121], "finance_rate": 0.05, "reinvest_rate": 0.05}, "outputs": {"mirr": 0.1}}],
     questions=[{"q": "Find the modified internal rate of return with cash flows = [-1000, 300, 400, 500], finance rate = 10%, reinvestment rate = 12%.",
                 "outputs": {'mirr': 0.09815669244631553, 'terminal_value': 1324.3200000000002, 'pv_outflows': 1000.0}},
                {"q": "Find the MIRR with cash flows = [100, 200], finance rate = 10%, reinvestment rate = 12%.", "outcome": "INVALID_INPUT"}])


def _payback(flows):
    _need(flows[0] < 0, "the first cash flow (the investment at time 0) must be negative")
    _need(all(x > 0 for x in flows[1:]), "the payback period is defined here for cash flows after time 0 that are all positive")
    cum = flows[0]
    for t in range(1, len(flows)):
        prev, cum = cum, cum + flows[t]
        if cum >= 0:
            return t - 1 + (-prev) / flows[t]
    raise ValueError("the investment is not recovered within the cash flows given")


def payback_period(cash_flows):
    return {"payback": _payback(_vec("cash_flows", cash_flows, 2))}


def _ref_payback(flows):
    import numpy as np
    cum = np.cumsum(np.asarray(flows, dtype=float))
    return float(np.interp(0.0, cum, np.arange(len(flows), dtype=float)))


def _rand_payback(rng):
    c0 = _u(rng, 500, 50000, 2)
    flows = [-c0]
    while sum(flows) < 0 or rng.random() < 0.3:
        flows.append(_u(rng, 0.05 * c0, 0.6 * c0, 2))
    return flows


_add("payback_period", name="Payback period", topic="capital budgeting / payback",
     purpose="Time until the cumulative cash flow turns non-negative, interpolating linearly within the period "
             "(cash arriving evenly during the period).",
     inputs=[_i("cash_flows", "cash flows [CF0 < 0, CF1, ..., CFn > 0]", "cash flow", "flows", "cf", **_CF)],
     outputs=[_o("payback", "payback period in periods")], primary="payback",
     conditions=["CF0 < 0", "every later cash flow > 0", "the investment is recovered within the flows"],
     method="first t with cumulative flow >= 0; payback = (t - 1) + (unrecovered at t - 1) / CFt",
     latex=r"\text{payback} = (t-1) + \frac{-\sum_{s<t} CF_s}{CF_t}", run=payback_period,
     reference=lambda cash_flows: {"payback": _ref_payback(cash_flows)},
     wrong=lambda cash_flows: {"payback": float(math.ceil(_payback(cash_flows) - 1e-12))},
     rand=lambda rng: {"cash_flows": _rand_payback(rng)},
     triggers=["payback period", "payback"],
     templates=["Find the payback period with cash flows = [-1000, 300, 400, 500]."],
     examples=[{"inputs": {"cash_flows": "-1000, 300, 400, 500"}, "outputs": {"payback": 2.6}},
               {"inputs": {"cash_flows": [-100, 50, 50, 50]}, "outputs": {"payback": 2.0}}],
     questions=[{"q": "Find the payback period with cash flows = [-1000, 300, 400, 500].", "outputs": {"payback": 2.6}},
                {"q": "Find the payback period with cash flows = [-1000, 300, 400].", "outcome": "INVALID_INPUT"}])


def discounted_payback(rate, cash_flows):
    r, c = _rate("rate", rate), _vec("cash_flows", cash_flows, 2)
    return {"discounted_payback": _payback([x / (1 + r) ** t for t, x in enumerate(c)])}


def _ref_dpayback(rate, cash_flows):
    import numpy as np
    c = np.asarray(cash_flows, dtype=float)
    return {"discounted_payback": _ref_payback(c * (1 + rate) ** -np.arange(len(c), dtype=float))}


def _rand_dpayback(rng):
    r = _u(rng, 0.01, 0.2)
    c0 = _u(rng, 500, 50000, 2)
    flows = [-c0]
    while sum(x / (1 + r) ** t for t, x in enumerate(flows)) < 0 or rng.random() < 0.3:
        flows.append(_u(rng, 0.05 * c0, 0.6 * c0, 2))
    return {"rate": r, "cash_flows": flows}


_add("discounted_payback", name="Discounted payback period", topic="capital budgeting / payback",
     purpose="Time until the cumulative DISCOUNTED cash flow turns non-negative (linear within the period).",
     inputs=[_i("rate", "discount rate per period (decimal)", "r", "discount rate", "cost of capital"),
             _i("cash_flows", "cash flows [CF0 < 0, CF1, ..., CFn > 0]", "cash flow", "flows", "cf", **_CF)],
     outputs=[_o("discounted_payback", "discounted payback period in periods")], primary="discounted_payback",
     conditions=["CF0 < 0", "every later cash flow > 0", "recovered within the flows", "-1 < rate < 1"],
     method="payback of the discounted flows CFt / (1 + r)^t", latex=r"\sum_{s \le t} \frac{CF_s}{(1+r)^{s}} \ge 0",
     run=discounted_payback, reference=_ref_dpayback,
     wrong=lambda rate, cash_flows: {"discounted_payback": _payback(cash_flows)},
     rand=_rand_dpayback,
     triggers=["discounted payback period", "discounted payback"],
     templates=["Find the discounted payback period with rate = 10%, cash flows = [-1000, 300, 400, 500, 200]."],
     examples=[{"inputs": {"rate": 0.1, "cash_flows": "-1000, 300, 400, 500, 200"}, "outputs": {'discounted_payback': 3.154000000000001}},
               {"inputs": {"rate": 0.0, "cash_flows": [-100, 50, 50, 50]}, "outputs": {"discounted_payback": 2.0}}],
     questions=[{"q": "Find the discounted payback period with rate = 10%, cash flows = [-1000, 300, 400, 500, 200].", "outputs": {'discounted_payback': 3.154000000000001}},
                {"q": "Find the discounted payback period with rate = 10%, cash flows = [-1000, 300, 400, 500].", "outcome": "INVALID_INPUT"}])


def profitability_index(rate, cash_flows):
    r, c = _rate("rate", rate), _vec("cash_flows", cash_flows, 2)
    _need(c[0] < 0, "the first cash flow (the investment at time 0) must be negative")
    pv = _hnpv(r, [0.0] + c[1:])
    return {"profitability_index": pv / -c[0], "pv_future": pv, "npv": pv + c[0]}


_add("profitability_index", name="Profitability index", topic="capital budgeting / discounted cash flow",
     purpose="PV of the future cash flows divided by the initial investment (benefit-cost ratio).",
     inputs=[_i("rate", "discount rate per period (decimal)", "r", "discount rate", "cost of capital", "required return"),
             _i("cash_flows", "cash flows [CF0 < 0, CF1, ..., CFn]", "cash flow", "flows", "cf", **_CF)],
     outputs=[_o("profitability_index", "PV(CF1..CFn) / |CF0|"), _o("pv_future", "PV of CF1..CFn"), _o("npv", "net present value")],
     primary="profitability_index", conditions=["CF0 < 0", "-1 < rate < 1"], method="PI = sum_{t>=1} CFt/(1+r)^t / (-CF0)",
     latex=r"PI = \frac{\sum_{t=1}^{n} CF_t (1+r)^{-t}}{-CF_0}", run=profitability_index,
     reference=lambda rate, cash_flows: (lambda v: {"profitability_index": (v - cash_flows[0]) / -cash_flows[0],
                                                    "npv": v})(float(__import__("numpy_financial").npv(rate, cash_flows))),
     wrong=lambda rate, cash_flows: (lambda o: dict(o, profitability_index=o["npv"] / -cash_flows[0]))(profitability_index(rate, cash_flows)),
     rand=lambda rng: {"rate": _u(rng, 0.01, 0.25),
                       "cash_flows": [-_u(rng, 500, 100000, 2)] + [_u(rng, 0, 40000, 2) for _ in range(rng.randint(1, 12))]},
     triggers=["profitability index", "benefit-cost ratio", "benefit cost ratio"],
     templates=["Find the profitability index with rate = 10%, cash flows = [-1000, 300, 400, 500]."],
     examples=[{"inputs": {"rate": 0.1, "cash_flows": "-1000, 300, 400, 500"}, "outputs": {'profitability_index': 0.9789631855747558, 'pv_future': 978.9631855747558, 'npv': -21.036814425244188}},
               {"inputs": {"rate": 0.0, "cash_flows": [-100, 60, 60]}, "outputs": {"profitability_index": 1.2}}],
     questions=[{"q": "Find the profitability index with rate = 10%, cash flows = [-1000, 300, 400, 500].", "outputs": {'profitability_index': 0.9789631855747558, 'pv_future': 978.9631855747558, 'npv': -21.036814425244188}},
                {"q": "Find the profitability index with rate = 10%, cash flows = [1000, 300, 400, 500].", "outcome": "INVALID_INPUT"}])


def equivalent_annual_annuity(rate, cash_flows):
    r, c = _rate("rate", rate), _vec("cash_flows", cash_flows, 2)
    n, v = len(c) - 1, _hnpv(r, c)
    return {"eaa": v / n if r == 0 else v * r / (1 - (1 + r) ** (-n)), "npv": v}


_add("equivalent_annual_annuity", name="Equivalent annual annuity", topic="capital budgeting / discounted cash flow",
     purpose="The level payment over the project's life with the same NPV (for comparing projects of unequal lives).",
     inputs=[_i("rate", "discount rate per period (decimal)", "r", "discount rate", "cost of capital"),
             _i("cash_flows", "cash flows [CF0, CF1, ..., CFn]", "cash flow", "flows", "cf", **_CF)],
     outputs=[_o("eaa", "equivalent annual annuity NPV r / (1 - (1 + r)^-n)"), _o("npv", "net present value")], primary="eaa",
     conditions=["-1 < rate < 1", "at least two cash flows"], method="EAA = NPV r / (1 - (1 + r)^-n)",
     latex=r"EAA = \frac{NPV\,r}{1 - (1 + r)^{-n}}", run=equivalent_annual_annuity,
     reference=lambda rate, cash_flows: (lambda npf: (lambda v: {"eaa": float(npf.pmt(rate, len(cash_flows) - 1, -v)), "npv": v})(
         float(npf.npv(rate, cash_flows))))(__import__("numpy_financial")),
     wrong=lambda rate, cash_flows: (lambda v: {"eaa": v / (len(cash_flows) - 1), "npv": v})(_hnpv(rate, cash_flows)),
     rand=lambda rng: {"rate": _u(rng, 0.01, 0.25),
                       "cash_flows": [-_u(rng, 500, 100000, 2)] + [_u(rng, 0, 40000, 2) for _ in range(rng.randint(1, 12))]},
     triggers=["equivalent annual annuity", "equivalent annual value", "equivalent annual cost"],
     templates=["Find the equivalent annual annuity with rate = 10%, cash flows = [-1000, 300, 400, 500]."],
     examples=[{"inputs": {"rate": 0.1, "cash_flows": "-1000, 300, 400, 500"}, "outputs": {'eaa': -8.459214501510571, 'npv': -21.036814425244188}},
               {"inputs": {"rate": 0.0, "cash_flows": [-100, 70, 70]}, "outputs": {"eaa": 20.0}}],
     questions=[{"q": "Find the equivalent annual annuity with rate = 10%, cash flows = [-1000, 300, 400, 500].", "outputs": {'eaa': -8.459214501510571, 'npv': -21.036814425244188}}])


def crossover_rate(cash_flows_a, cash_flows_b):
    a, b = _vec("cash_flows_a", cash_flows_a, 2), _vec("cash_flows_b", cash_flows_b, 2)
    _need(len(a) == len(b), "the two projects need cash flows over the same periods (lists of equal length)")
    d = [x - y for x, y in zip(a, b)]
    return {"crossover_rate": _irr(d), "npv_at_crossover": _hnpv(_irr(d), a)}


def _verify_crossover(inputs, outputs):
    r = outputs.get("crossover_rate")
    if not isinstance(r, (int, float)) or not math.isfinite(r) or r <= -1:
        return math.inf
    a, b = inputs["cash_flows_a"], inputs["cash_flows_b"]
    d = [x - y for x, y in zip(a, b)]
    return max(abs(_hnpv(r, d)) / _npv_scale(r, d),
               abs(outputs.get("npv_at_crossover", math.inf) - _hnpv(r, a)) / max(1.0, _npv_scale(r, a)))


def _rand_crossover(rng):
    n = rng.randint(2, 8)
    a = [-_u(rng, 1000, 50000, 2)] + [_u(rng, 100, 20000, 2) for _ in range(n)]
    d = [-_u(rng, 100, 20000, 2)] + [_u(rng, 10, 8000, 2) for _ in range(n)]
    return {"cash_flows_a": [x + y for x, y in zip(a, d)], "cash_flows_b": a}


_add("crossover_rate", name="NPV crossover rate of two projects", topic="capital budgeting / project comparison",
     purpose="The discount rate at which two projects have equal NPV (the IRR of the difference A - B); given only "
             "when it is unique.",
     inputs=[_i("cash_flows_a", "cash flows of project A [CF0, ..., CFn]", "project a", "flows a", "cash flows of project a", "a", **_CF),
             _i("cash_flows_b", "cash flows of project B, same periods", "project b", "flows b", "cash flows of project b", "b", **_CF)],
     outputs=[_o("crossover_rate", "rate at which NPV(A) = NPV(B)"), _o("npv_at_crossover", "the common NPV at that rate")],
     primary="crossover_rate", conditions=["equal lengths", "the difference A - B changes sign exactly once"],
     method="IRR of the incremental cash flows A - B", latex=r"\sum_t \frac{CF^A_t - CF^B_t}{(1 + r^{*})^{t}} = 0",
     run=crossover_rate,
     reference=lambda cash_flows_a, cash_flows_b: {"crossover_rate": float(__import__("numpy_financial").irr(
         [x - y for x, y in zip(cash_flows_a, cash_flows_b)]))},
     wrong=lambda cash_flows_a, cash_flows_b: (lambda r: {"crossover_rate": r, "npv_at_crossover": _hnpv(r, cash_flows_a)})(
         (_irr(cash_flows_a) + _irr(cash_flows_b)) / 2),
     rand=_rand_crossover,
     triggers=["crossover rate", "npv crossover", "fisher intersection"],
     templates=["Find the crossover rate with cash flows a = [-1000, 100, 1200], cash flows b = [-1000, 700, 500]."],
     examples=[{"inputs": {"cash_flows_a": "-1000, 100, 1200", "cash_flows_b": "-1000, 700, 500"}, "outputs": {'crossover_rate': 0.16666666666666652, 'npv_at_crossover': -32.65306122448965}}],
     questions=[{"q": "Find the crossover rate with cash flows a = [-1000, 100, 1200], cash flows b = [-1000, 700, 500].", "outputs": {'crossover_rate': 0.16666666666666652, 'npv_at_crossover': -32.65306122448965}},
                {"q": "Find the crossover rate with project a = [-1000, 100, 1200], project b = [-1000, 700].", "outcome": "INVALID_INPUT"}])
PROCEDURES["crossover_rate"]["verify"] = _verify_crossover


# =====================================================================================================================
# BONDS (priced on a coupon date; yield = nominal annual rate compounded at the coupon frequency)
# =====================================================================================================================
def _bond(face, coupon_rate, years, frequency):
    F = _pos("face", face)
    c = _fin("coupon_rate", coupon_rate)
    _need(0 <= c < 1, "coupon_rate must be a decimal between 0 and 1 (write 5% or 0.05)")
    f = _int("frequency", frequency, 1, 12)
    _need(12 % f == 0, "frequency is 1, 2, 3, 4, 6 or 12 coupons a year")
    N = _periods_of(_pos("years", years), f)
    return F, c, f, N, F * c / f


def _bprice(F, cpn, N, j):
    """Price of N coupons cpn and the face F at a yield j per period."""
    if j == 0:
        return cpn * N + F
    v = (1 + j) ** (-N)
    return cpn * (1 - v) / j + F * v


def bond_price(face, coupon_rate, ytm, years, frequency=2):
    F, c, f, N, cpn = _bond(face, coupon_rate, years, frequency)
    y = _rate("ytm", ytm)
    p = _bprice(F, cpn, N, y / f)
    return {"price": p, "current_yield": F * c / p, "coupon": cpn}


_BOND_IN = [_i("face", "face (par) value repaid at maturity", "face value", "par value", "par", "f"),
            _i("coupon_rate", "annual coupon rate (decimal)", "coupon rate", "coupon", "c"),
            _i("years", "years to maturity (a whole number of coupon periods)", "maturity", "years to maturity", "term", "t", "n"),
            _i("frequency", "coupons per year (2 = semiannual, default)", "coupons per year", "payments per year", "m",
               type="integer", required=False, min=1)]


def _bond_inputs(extra_front=(), extra_back=()):
    base = [dict(i, _aliases=list(i["_aliases"])) for i in _BOND_IN]
    return list(extra_front) + base[:3] + list(extra_back) + base[3:]


def _rand_bond(rng):
    f = rng.choice([1, 2, 2, 2, 4, 12])
    return {"face": float(rng.choice([100, 1000, 1000, 5000])), "coupon_rate": rng.choice([0.0, _u(rng, 0.01, 0.12), _u(rng, 0.01, 0.12)]),
            "years": float(rng.randint(1, 30)), "frequency": f}


_add("bond_price", name="Bond price from its yield", topic="bonds / pricing",
     purpose="Price of a fixed-coupon bond on a coupon date from its yield to maturity, with the current yield.",
     inputs=_bond_inputs(extra_back=[_i("ytm", "yield to maturity, nominal annual, compounded at the coupon frequency",
                                        "yield", "yield to maturity", "y", "market yield", "required yield")]),
     outputs=[_o("price", "bond price"), _o("current_yield", "annual coupon / price"), _o("coupon", "coupon per period")],
     primary="price", conditions=["face > 0", "0 <= coupon_rate < 1", "-1 < ytm < 1", "years x frequency a whole number"],
     method="P = C (1 - (1 + y/m)^-N)/(y/m) + F (1 + y/m)^-N, C = F c / m, N = years x m",
     latex=r"P = \sum_{k=1}^{N} \frac{C}{(1 + y/m)^{k}} + \frac{F}{(1 + y/m)^{N}}", run=bond_price,
     reference=lambda face, coupon_rate, ytm, years, frequency=2: (lambda p: {"price": p, "current_yield": face * coupon_rate / p})(
         -float(__import__("numpy_financial").pv(ytm / frequency, round(years * frequency), face * coupon_rate / frequency, face))),
     wrong=lambda face, coupon_rate, ytm, years, frequency=2: (lambda p: {"price": p, "current_yield": face * coupon_rate / p,
                                                                           "coupon": face * coupon_rate / frequency})(
         _bprice(face, face * coupon_rate / frequency, round(years * frequency), ytm)),
     rand=lambda rng: dict(_rand_bond(rng), ytm=_u(rng, 0.005, 0.15)),
     triggers=["bond price", "price of a bond", "price of the bond", "bond valuation", "value of a bond"],
     templates=["Find the bond price with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2."],
     examples=[{"inputs": {"face": 1000, "coupon_rate": 0.05, "years": 10, "ytm": 0.06, "frequency": 2}, "outputs": {'price': 925.6126256977225, 'current_yield': 0.05401827785388108, 'coupon': 25.0}},
               {"inputs": {"face": 1000, "coupon_rate": 0.06, "years": 5, "ytm": 0.06, "frequency": 1}, "outputs": {"price": 1000.0}}],
     questions=[{"q": "Find the bond price with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2.", "outputs": {'price': 925.6126256977225, 'current_yield': 0.05401827785388108, 'coupon': 25.0}},
                {"q": "What is the price of a bond with par value = 1000, coupon rate = 6%, maturity = 5, yield = 6%, frequency = 1?",
                 "outputs": {"price": 1000.0}},
                {"q": "Find the bond price with face = 1000, coupon rate = 5%, years = 10.25, ytm = 6%, frequency = 2.", "outcome": "INVALID_INPUT"},
                {"q": "Find the bond price with face = 1000, coupon rate = 5%, years = 10.", "outcome": "NEEDS_INPUT"}])


def bond_ytm(price, face, coupon_rate, years, frequency=2):
    F, c, f, N, cpn = _bond(face, coupon_rate, years, frequency)
    P = _pos("price", price)

    def g(j):
        try:
            return _bprice(F, cpn, N, j) - P
        except OverflowError:
            return math.inf
    lo, hi = -0.5, 1.0
    _need(g(lo) > 0 > g(hi), "no yield between -50% and 100% per period gives this price")
    for _ in range(300):
        mid = (lo + hi) / 2
        if mid in (lo, hi):
            break
        if g(mid) > 0:
            lo = mid
        else:
            hi = mid
    j = (lo + hi) / 2
    return {"ytm": j * f, "current_yield": F * c / P}


def _verify_ytm(inputs, outputs):
    y = outputs.get("ytm")
    if not isinstance(y, (int, float)) or not math.isfinite(y):
        return math.inf
    F, c, f = inputs["face"], inputs["coupon_rate"], inputs.get("frequency", 2)
    N = round(inputs["years"] * f)
    P = inputs["price"]
    try:
        r1 = abs(_bprice(F, F * c / f, N, y / f) - P) / max(1.0, P)
    except (OverflowError, ZeroDivisionError):
        return math.inf
    return max(r1, _residual(outputs, {"current_yield": F * c / P}))


def _ref_ytm(price, face, coupon_rate, years, frequency=2):
    import numpy_financial as npf
    from scipy.optimize import brentq
    N, cpn = round(years * frequency), face * coupon_rate / frequency
    j = brentq(lambda x: -float(npf.pv(x, N, cpn, face)) - price, -0.5, 1.0, xtol=1e-16, rtol=1e-15, maxiter=500)
    return {"ytm": j * frequency}


_add("bond_ytm", name="Yield to maturity of a bond", topic="bonds / yields",
     purpose="The yield to maturity (nominal annual, compounded at the coupon frequency) that equates the bond's "
             "price to the present value of its coupons and face value.",
     inputs=_bond_inputs(extra_front=[_i("price", "bond price (clean = dirty on a coupon date)", "bond price", "market price", "p")]),
     outputs=[_o("ytm", "yield to maturity, nominal annual"), _o("current_yield", "annual coupon / price")], primary="ytm",
     conditions=["price > 0", "face > 0", "0 <= coupon_rate < 1", "years x frequency a whole number",
                 "a yield between -50% and 100% per period"],
     method="bisection on P(y) = price (P falls as y rises, so the yield is unique)",
     latex=r"\text{price} = \sum_{k=1}^{N} \frac{C}{(1 + y/m)^{k}} + \frac{F}{(1 + y/m)^{N}}", run=bond_ytm, reference=_ref_ytm,
     wrong=lambda price, face, coupon_rate, years, frequency=2: {
         "ytm": (face * coupon_rate + (face - price) / years) / ((face + price) / 2), "current_yield": face * coupon_rate / price},
     rand=lambda rng: (lambda b, y: dict(b, price=round(_bprice(b["face"], b["face"] * b["coupon_rate"] / b["frequency"],
                                                                round(b["years"] * b["frequency"]), y / b["frequency"]), 4)))(
         _rand_bond(rng), _u(rng, 0.005, 0.15)),
     triggers=["yield to maturity", "ytm", "bond yield", "yield of a bond", "yield of the bond"],
     templates=["Find the yield to maturity with price = 925.61, face = 1000, coupon rate = 5%, years = 10, frequency = 2."],
     examples=[{"inputs": {"price": 1000, "face": 1000, "coupon_rate": 0.06, "years": 5, "frequency": 2}, "outputs": {"ytm": 0.06}},
               {"inputs": {"price": 925.61, "face": 1000, "coupon_rate": 0.05, "years": 10, "frequency": 2}, "outputs": {'ytm': 0.06000037008496918, 'current_yield': 0.05401843108868746}}],
     questions=[{"q": "Find the yield to maturity with price = 925.61, face = 1000, coupon rate = 5%, years = 10, frequency = 2.", "outputs": {'ytm': 0.06000037008496918, 'current_yield': 0.05401843108868746}},
                {"q": "What is the YTM with price = 1000, par = 1000, coupon = 6%, maturity = 5?", "outputs": {"ytm": 0.06}},
                {"q": "Find the yield to maturity with price = 10, face = 1000, coupon rate = 5%, years = 10.", "outcome": "INVALID_INPUT"}])
PROCEDURES["bond_ytm"]["verify"] = _verify_ytm


def _mp_bond_price(face, coupon_rate, years, frequency):
    import mpmath
    N = round(years * frequency)
    cpn = mpmath.mpf(face) * mpmath.mpf(coupon_rate) / frequency

    def P(y):
        j = y / frequency
        return sum(cpn / (1 + j) ** k for k in range(1, N + 1)) + mpmath.mpf(face) / (1 + j) ** N
    return P


def bond_duration(face, coupon_rate, ytm, years, frequency=2):
    F, c, f, N, cpn = _bond(face, coupon_rate, years, frequency)
    y = _rate("ytm", ytm)
    v = 1 / (1 + y / f)
    pv = [cpn * v ** k for k in range(1, N + 1)]
    pv[-1] += F * v ** N
    P = math.fsum(pv)
    mac = math.fsum(k / f * x for k, x in enumerate(pv, start=1)) / P
    return {"macaulay_duration": mac, "modified_duration": mac / (1 + y / f), "price": P}


def _ref_duration(face, coupon_rate, ytm, years, frequency=2):
    import mpmath                                          # duration from the derivative of the price, not a weighted time
    with mpmath.workdps(40):
        P = _mp_bond_price(face, coupon_rate, years, frequency)
        p0, d = P(mpmath.mpf(ytm)), mpmath.diff(P, mpmath.mpf(ytm))
        mod = -d / p0
        return {"macaulay_duration": float(mod * (1 + mpmath.mpf(ytm) / frequency)), "modified_duration": float(mod), "price": float(p0)}


_add("bond_duration", name="Macaulay and modified duration of a bond", topic="bonds / interest-rate risk",
     purpose="The Macaulay duration (PV-weighted average time to the cash flows, years) and the modified duration "
             "(the price's sensitivity -dP/dy / P) of a fixed-coupon bond.",
     inputs=_bond_inputs(extra_back=[_i("ytm", "yield to maturity, nominal annual, compounded at the coupon frequency",
                                        "yield", "yield to maturity", "y")]),
     outputs=[_o("macaulay_duration", "Macaulay duration in years"), _o("modified_duration", "modified duration = Macaulay / (1 + y/m)"),
              _o("price", "bond price")], primary="macaulay_duration",
     conditions=["face > 0", "0 <= coupon_rate < 1", "-1 < ytm < 1", "years x frequency a whole number"],
     method="D_mac = sum (k/m) PV_k / P; D_mod = D_mac / (1 + y/m)",
     latex=r"D_{mac} = \frac{1}{P}\sum_{k=1}^{N} \frac{k}{m}\,\frac{CF_k}{(1 + y/m)^{k}},\quad D_{mod} = \frac{D_{mac}}{1 + y/m}",
     run=bond_duration, reference=_ref_duration,
     wrong=lambda face, coupon_rate, ytm, years, frequency=2: (lambda o: dict(o, macaulay_duration=o["modified_duration"]))(
         bond_duration(face, coupon_rate, ytm, years, frequency)),
     rand=lambda rng: dict(_rand_bond(rng), ytm=_u(rng, 0.005, 0.15)),
     triggers=["macaulay duration", "modified duration", "duration of a bond", "duration of the bond", "bond duration"],
     templates=["Find the Macaulay duration with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2."],
     examples=[{"inputs": {"face": 1000, "coupon_rate": 0.05, "years": 10, "ytm": 0.06, "frequency": 2}, "outputs": {'macaulay_duration': 7.894997340182342, 'modified_duration': 7.665045961342079, 'price': 925.6126256977227}},
               {"inputs": {"face": 1000, "coupon_rate": 0.0, "years": 7, "ytm": 0.05, "frequency": 1}, "outputs": {"macaulay_duration": 7.0}}],
     questions=[{"q": "Find the Macaulay duration with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2.", "outputs": {'macaulay_duration': 7.894997340182342, 'modified_duration': 7.665045961342079, 'price': 925.6126256977227}},
                {"q": "Find the modified duration with face = 1000, coupon rate = 0%, years = 7, yield = 5%, frequency = 1.",
                 "outputs": {"macaulay_duration": 7.0, "modified_duration": 6.666666666666667}}])


def bond_convexity(face, coupon_rate, ytm, years, frequency=2):
    F, c, f, N, cpn = _bond(face, coupon_rate, years, frequency)
    y = _rate("ytm", ytm)
    v = 1 / (1 + y / f)
    cf = [cpn] * N
    cf[-1] += F
    P = math.fsum(x * v ** k for k, x in enumerate(cf, start=1))
    conv = math.fsum(x * k * (k + 1) * v ** (k + 2) for k, x in enumerate(cf, start=1)) / (f * f * P)
    return {"convexity": conv, "price": P}


def _ref_convexity(face, coupon_rate, ytm, years, frequency=2):
    import mpmath
    with mpmath.workdps(40):
        P = _mp_bond_price(face, coupon_rate, years, frequency)
        y = mpmath.mpf(ytm)
        return {"convexity": float(mpmath.diff(P, y, 2) / P(y)), "price": float(P(y))}


_add("bond_convexity", name="Convexity of a bond", topic="bonds / interest-rate risk",
     purpose="The convexity (1/P) d^2P/dy^2 of a fixed-coupon bond, in years squared.",
     inputs=_bond_inputs(extra_back=[_i("ytm", "yield to maturity, nominal annual, compounded at the coupon frequency",
                                        "yield", "yield to maturity", "y")]),
     outputs=[_o("convexity", "convexity (1/P) d2P/dy2, years^2"), _o("price", "bond price")], primary="convexity",
     conditions=["face > 0", "0 <= coupon_rate < 1", "-1 < ytm < 1", "years x frequency a whole number"],
     method="C = sum CF_k k (k + 1) (1 + y/m)^-(k+2) / (m^2 P)",
     latex=r"C = \frac{1}{P\,m^{2}} \sum_{k=1}^{N} \frac{k(k+1)\,CF_k}{(1 + y/m)^{k+2}}", run=bond_convexity, reference=_ref_convexity,
     wrong=lambda face, coupon_rate, ytm, years, frequency=2: (lambda o: dict(o, convexity=math.fsum(
         x * (k / frequency) ** 2 * (1 + ytm / frequency) ** (-k) for k, x in enumerate(
             [face * coupon_rate / frequency] * (round(years * frequency) - 1) + [face * coupon_rate / frequency + face], start=1)) / o["price"]))(
         bond_convexity(face, coupon_rate, ytm, years, frequency)),
     rand=lambda rng: dict(_rand_bond(rng), ytm=_u(rng, 0.005, 0.15)),
     triggers=["convexity of a bond", "convexity of the bond", "bond convexity", "convexity"],
     templates=["Find the convexity with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2."],
     examples=[{"inputs": {"face": 1000, "coupon_rate": 0.05, "years": 10, "ytm": 0.06, "frequency": 2}, "outputs": {'convexity': 71.78539801290005, 'price': 925.6126256977227}},
               {"inputs": {"face": 100, "coupon_rate": 0.0, "years": 5, "ytm": 0.05, "frequency": 1}, "outputs": {'convexity': 27.21088435374149, 'price': 78.35261664684589}}],
     questions=[{"q": "Find the convexity with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2.", "outputs": {'convexity': 71.78539801290005, 'price': 925.6126256977227}}])


_ZC = {"annual": 1, "semiannual": 2, "quarterly": 4, "monthly": 12, "continuous": 0}


def zero_coupon_bond(face, ytm, years, compounding="annual"):
    F, y, t = _pos("face", face), _rate("ytm", ytm), _nonneg("years", years)
    _need(compounding in _ZC, "compounding is annual, semiannual, quarterly, monthly or continuous")
    m = _ZC[compounding]
    d = math.exp(-y * t) if m == 0 else (1 + y / m) ** (-m * t)
    return {"price": F * d, "discount_factor": d}


def _ref_zero(face, ytm, years, compounding="annual"):
    import QuantLib as ql
    freq = {1: ql.Annual, 2: ql.Semiannual, 4: ql.Quarterly, 12: ql.Monthly}
    m = _ZC[compounding]
    ir = ql.InterestRate(ytm, ql.Actual365Fixed(), ql.Continuous if m == 0 else ql.Compounded, ql.Annual if m == 0 else freq[m])
    d = ir.discountFactor(float(years))
    return {"price": face * d, "discount_factor": d}


_add("zero_coupon_bond", name="Price of a zero-coupon bond", topic="bonds / pricing",
     purpose="Price of a bond that pays only its face value at maturity, at a yield with a stated compounding.",
     inputs=[_i("face", "face value paid at maturity", "face value", "par value", "par", "f"),
             _i("ytm", "yield (decimal), with the stated compounding", "yield", "yield to maturity", "y", "rate", "spot rate"),
             _i("years", "years to maturity", "maturity", "t", "time"),
             _i("compounding", "annual (default), semiannual, quarterly, monthly or continuous", "compounding frequency",
                type="choice", choices=list(_ZC), required=False,
                syn={"semi-annual": "semiannual", "semi-annually": "semiannual", "semiannually": "semiannual", "annually": "annual",
                     "yearly": "annual", "continuously": "continuous", "quarter": "quarterly", "month": "monthly"})],
     outputs=[_o("price", "price F x discount factor"), _o("discount_factor", "(1 + y/m)^(-m t) or e^(-y t)")], primary="price",
     conditions=["face > 0", "-1 < ytm < 1", "years >= 0"],
     method="P = F (1 + y/m)^(-m t), or F e^(-y t) when continuous", latex=r"P = \frac{F}{(1 + y/m)^{m t}}",
     run=zero_coupon_bond, reference=_ref_zero,
     wrong=lambda face, ytm, years, compounding="annual": {"price": face / (1 + ytm * years), "discount_factor": 1 / (1 + ytm * years)},
     rand=lambda rng: {"face": float(rng.choice([100, 1000, 10000])), "ytm": _u(rng, 0.005, 0.15), "years": _u(rng, 0.5, 30, 2),
                       "compounding": rng.choice(list(_ZC))},
     triggers=["zero-coupon bond", "zero coupon bond", "discount bond", "pure discount bond"],
     templates=["Find the price of a zero-coupon bond with face = 1000, ytm = 5%, years = 10, compounding = semiannual."],
     examples=[{"inputs": {"face": 1000, "ytm": 0.05, "years": 10, "compounding": "semiannual"}, "outputs": {'price': 610.2709428588307, 'discount_factor': 0.6102709428588308}},
               {"inputs": {"face": 1000, "ytm": 0.05, "years": 2}, "outputs": {'price': 907.0294784580498, 'discount_factor': 0.9070294784580498}},
               {"inputs": {"face": 1000, "ytm": 0.05, "years": 10, "compounding": "continuous"}, "outputs": {'price': 606.5306597126335, 'discount_factor': 0.6065306597126334}}],
     questions=[{"q": "Find the price of a zero-coupon bond with face = 1000, ytm = 5%, years = 10, compounding = semiannual.", "outputs": {'price': 610.2709428588307, 'discount_factor': 0.6102709428588308}},
                {"q": "Find the price of a zero coupon bond with face = 1000, yield = 5%, years = 10, compounding = continuously.", "outputs": {'price': 606.5306597126335, 'discount_factor': 0.6065306597126334}},
                {"q": "Find the price of a zero coupon bond with face = 1000, yield = 5%, years = 10, compounding = weekly.", "outcome": "INVALID_INPUT"}])


def bond_price_change(face, coupon_rate, ytm, years, yield_change, frequency=2):
    d = bond_duration(face, coupon_rate, ytm, years, frequency)
    cv = bond_convexity(face, coupon_rate, ytm, years, frequency)["convexity"]
    dy = _fin("yield_change", yield_change)
    _need(abs(dy) < 0.5 and -1 < ytm + dy < 1, "yield_change must keep the yield between -100% and 100% (and |change| < 50%)")
    P, D = d["price"], d["modified_duration"]
    new = bond_price(face, coupon_rate, ytm + dy, years, frequency)["price"]
    return {"duration_estimate": -D * P * dy, "duration_convexity_estimate": -D * P * dy + 0.5 * cv * P * dy * dy,
            "actual_change": new - P, "new_price": new, "price": P}


def _ref_price_change(face, coupon_rate, ytm, years, yield_change, frequency=2):
    import mpmath
    import numpy_financial as npf
    N, cpn = round(years * frequency), face * coupon_rate / frequency
    with mpmath.workdps(40):
        P = _mp_bond_price(face, coupon_rate, years, frequency)
        y = mpmath.mpf(ytm)
        p0, d1, d2 = P(y), mpmath.diff(P, y), mpmath.diff(P, y, 2)
        dy = mpmath.mpf(yield_change)
        new = -float(npf.pv((ytm + yield_change) / frequency, N, cpn, face))
        return {"duration_estimate": float(d1 * dy), "duration_convexity_estimate": float(d1 * dy + d2 * dy * dy / 2),
                "actual_change": new - float(p0), "new_price": new}


_add("bond_price_change", name="Bond price change from duration and convexity", topic="bonds / interest-rate risk",
     purpose="The price change of a bond for a change in its yield: duration estimate, duration-plus-convexity "
             "estimate and the exact change by repricing.",
     inputs=_bond_inputs(extra_back=[_i("ytm", "current yield to maturity, nominal annual", "yield", "yield to maturity", "y"),
                                     _i("yield_change", "change in the yield (decimal; +1% = 0.01, 25 basis points = 0.0025)",
                                        "change in yield", "yield change", "delta y", "dy", "rate change")]),
     outputs=[_o("duration_estimate", "-D_mod P dy"), _o("duration_convexity_estimate", "-D_mod P dy + C P dy^2 / 2"),
              _o("actual_change", "exact change by repricing"), _o("new_price", "price at the new yield"), _o("price", "price now")],
     primary="duration_convexity_estimate",
     conditions=["face > 0", "0 <= coupon_rate < 1", "the old and new yields between -100% and 100%", "|yield_change| < 50%"],
     method="dP ~ -D_mod P dy + (1/2) C P dy^2; exact change by repricing at y + dy",
     latex=r"\Delta P \approx -D_{mod}\,P\,\Delta y + \tfrac{1}{2}\,C\,P\,(\Delta y)^{2}", run=bond_price_change,
     reference=_ref_price_change,
     wrong=lambda face, coupon_rate, ytm, years, yield_change, frequency=2: (lambda o: dict(
         o, duration_convexity_estimate=o["duration_estimate"] + 0.5 * (o["duration_convexity_estimate"] - o["duration_estimate"]) * 2 * 2))(
         bond_price_change(face, coupon_rate, ytm, years, yield_change, frequency)),
     rand=lambda rng: dict(_rand_bond(rng), ytm=_u(rng, 0.01, 0.15), yield_change=rng.choice([-1, 1]) * _u(rng, 0.0005, 0.03)),
     triggers=["price change of a bond", "bond price change", "change in bond price", "change in the price of the bond",
               "duration and convexity", "price change of the bond"],
     templates=["Find the bond price change with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, yield change = 1%, frequency = 2."],
     examples=[{"inputs": {"face": 1000, "coupon_rate": 0.05, "years": 10, "ytm": 0.06, "yield_change": 0.01, "frequency": 2}, "outputs": {'duration_estimate': -70.94863318371567, 'duration_convexity_estimate': -67.62635964664184, 'actual_change': -67.73665871724529, 'new_price': 857.8759669804774, 'price': 925.6126256977227}}],
     questions=[{"q": "Find the bond price change with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, yield change = 1%, frequency = 2.",
                 "outputs": {'duration_estimate': -70.94863318371567, 'duration_convexity_estimate': -67.62635964664184, 'actual_change': -67.73665871724529, 'new_price': 857.8759669804774, 'price': 925.6126256977227}}])


def forward_rate(spot_rate_1, maturity_1, spot_rate_2, maturity_2):
    s1, t1 = _rate("spot_rate_1", spot_rate_1), _nonneg("maturity_1", maturity_1)
    s2, t2 = _rate("spot_rate_2", spot_rate_2), _pos("maturity_2", maturity_2)
    _need(t2 > t1, "maturity_2 must be later than maturity_1")
    return {"forward_rate": ((1 + s2) ** t2 / (1 + s1) ** t1) ** (1 / (t2 - t1)) - 1}


def _ref_forward(spot_rate_1, maturity_1, spot_rate_2, maturity_2):
    from scipy.optimize import brentq                     # no arbitrage: invest to t2, or to t1 and roll at the forward rate
    t1, t2 = maturity_1, maturity_2
    return {"forward_rate": brentq(lambda f: (1 + spot_rate_1) ** t1 * (1 + f) ** (t2 - t1) - (1 + spot_rate_2) ** t2,
                                   -0.99, 5.0, xtol=1e-16, rtol=1e-15, maxiter=500)}


_add("forward_rate", name="Implied forward rate from spot rates", topic="bonds / term structure",
     purpose="The annually compounded forward rate between t1 and t2 implied by the spot rates for t1 and t2.",
     inputs=[_i("spot_rate_1", "spot rate to maturity t1 (annual compounding)", "s1", "first spot rate", "short rate", "r1"),
             _i("maturity_1", "first maturity t1 in years", "t1", "first maturity"),
             _i("spot_rate_2", "spot rate to maturity t2 (annual compounding)", "s2", "second spot rate", "long rate", "r2"),
             _i("maturity_2", "second maturity t2 > t1 in years", "t2", "second maturity")],
     outputs=[_o("forward_rate", "forward rate f(t1, t2), annual compounding")], primary="forward_rate",
     conditions=["-1 < spot rates < 1", "0 <= t1 < t2"],
     method="(1 + s2)^t2 = (1 + s1)^t1 (1 + f)^(t2 - t1)",
     latex=r"f_{t_1,t_2} = \left[\frac{(1+s_2)^{t_2}}{(1+s_1)^{t_1}}\right]^{1/(t_2 - t_1)} - 1", run=forward_rate,
     reference=_ref_forward,
     wrong=lambda spot_rate_1, maturity_1, spot_rate_2, maturity_2: {
         "forward_rate": (spot_rate_2 * maturity_2 - spot_rate_1 * maturity_1) / (maturity_2 - maturity_1)},
     rand=lambda rng: (lambda t1: {"spot_rate_1": _u(rng, 0.005, 0.1), "maturity_1": t1, "spot_rate_2": _u(rng, 0.005, 0.12),
                                   "maturity_2": t1 + rng.randint(1, 10)})(float(rng.randint(1, 10))),
     triggers=["forward rate", "implied forward rate"],
     templates=["Find the forward rate with s1 = 4%, t1 = 1, s2 = 5%, t2 = 2."],
     examples=[{"inputs": {"spot_rate_1": 0.04, "maturity_1": 1, "spot_rate_2": 0.05, "maturity_2": 2}, "outputs": {'forward_rate': 0.060096153846153744}},
               {"inputs": {"spot_rate_1": 0.05, "maturity_1": 2, "spot_rate_2": 0.05, "maturity_2": 5}, "outputs": {"forward_rate": 0.05}}],
     questions=[{"q": "Find the forward rate with s1 = 4%, t1 = 1, s2 = 5%, t2 = 2.", "outputs": {'forward_rate': 0.060096153846153744}},
                {"q": "Find the forward rate with s1 = 4%, t1 = 2, s2 = 5%, t2 = 1.", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# STOCKS AND THE COST OF CAPITAL
# =====================================================================================================================
def _d1(dividend, next_dividend, growth):
    _need((dividend is None) != (next_dividend is None),
          "give either the dividend just paid (D0) or the next dividend (D1), not both")
    return _fin("next_dividend", next_dividend) if next_dividend is not None else _fin("dividend", dividend) * (1 + growth)


def gordon_growth(required_return, growth, dividend=None, next_dividend=None):
    r, g = _rate("required_return", required_return, lo=0.0), _rate("growth", growth)
    _need(r > g, "the required return must exceed the growth rate (otherwise the value is infinite)")
    D1 = _d1(dividend, next_dividend, g)
    return {"price": D1 / (r - g), "next_dividend": D1, "dividend_yield": r - g}


def _ref_gordon(required_return, growth, dividend=None, next_dividend=None):
    import mpmath
    D1 = next_dividend if next_dividend is not None else dividend * (1 + growth)
    with mpmath.workdps(30):
        r, g = mpmath.mpf(required_return), mpmath.mpf(growth)
        return {"price": float(mpmath.nsum(lambda k: D1 * (1 + g) ** (k - 1) / (1 + r) ** k, [1, mpmath.inf])), "next_dividend": D1}


_DIV_IN = [_i("dividend", "dividend just paid D0 (give this OR next_dividend)", "d0", "current dividend", "last dividend",
              "dividend just paid", "recent dividend", required=False),
           _i("next_dividend", "dividend expected one year from now D1 (give this OR dividend)", "d1", "next dividend",
              "expected dividend", "next year's dividend", required=False)]


def _rand_div(rng):
    return rng.choice([{"dividend": _u(rng, 0.1, 10, 2)}, {"next_dividend": _u(rng, 0.1, 10, 2)}])


_add("gordon_growth", name="Dividend discount model (Gordon growth)", topic="stocks / equity valuation",
     purpose="Value of a share whose dividend grows at a constant rate forever: P0 = D1 / (r - g).",
     inputs=[_i("required_return", "required return on equity (decimal)", "r", "required rate of return", "cost of equity",
                "discount rate", "ke")] + [_i("growth", "constant dividend growth rate (decimal)", "g", "growth rate",
                                              "dividend growth rate")] + [dict(i, _aliases=list(i["_aliases"])) for i in _DIV_IN],
     outputs=[_o("price", "share value P0 = D1 / (r - g)"), _o("next_dividend", "D1 = D0 (1 + g)"), _o("dividend_yield", "D1 / P0 = r - g")],
     primary="price", conditions=["0 < required_return < 1", "growth < required_return", "exactly one of dividend (D0), next_dividend (D1)"],
     method="P0 = D0 (1 + g) / (r - g)", latex=r"P_0 = \frac{D_1}{r - g} = \frac{D_0 (1 + g)}{r - g}", run=gordon_growth,
     reference=_ref_gordon,
     wrong=lambda required_return, growth, dividend=None, next_dividend=None: (lambda d: {
         "price": d / (required_return - growth), "next_dividend": d, "dividend_yield": required_return - growth})(
         dividend if dividend is not None else next_dividend * (1 + growth)),
     rand=lambda rng: (lambda r: dict(_rand_div(rng), required_return=r, growth=round(r - rng.uniform(0.02, 0.12), 4)))(_u(rng, 0.05, 0.25)),
     triggers=["gordon growth model", "gordon growth", "dividend discount model", "constant growth model", "gordon model", "ddm"],
     templates=["Find the share price with the Gordon growth model with dividend = 2, required return = 10%, growth = 4%."],
     examples=[{"inputs": {"dividend": 2, "required_return": 0.1, "growth": 0.04}, "outputs": {'price': 34.666666666666664, 'next_dividend': 2.08, 'dividend_yield': 0.060000000000000005}},
               {"inputs": {"next_dividend": 3, "required_return": 0.12, "growth": 0.02}, "outputs": {"price": 30.0}}],
     questions=[{"q": "Find the share price with the Gordon growth model with dividend = 2, required return = 10%, growth = 4%.", "outputs": {'price': 34.666666666666664, 'next_dividend': 2.08, 'dividend_yield': 0.060000000000000005}},
                {"q": "Use the dividend discount model with next dividend = 3, required return = 12%, growth = 2%.", "outputs": {"price": 30.0}},
                {"q": "Use the dividend discount model with next dividend = 3, required return = 2%, growth = 12%.", "outcome": "INVALID_INPUT"},
                {"q": "Use the dividend discount model with dividend = 2, next dividend = 3, required return = 12%, growth = 2%.", "outcome": "INVALID_INPUT"}])


def two_stage_ddm(dividend, high_growth, high_growth_years, stable_growth, required_return):
    D0 = _pos("dividend", dividend)
    g1, g2 = _rate("high_growth", high_growth), _rate("stable_growth", stable_growth)
    n = _int("high_growth_years", high_growth_years, 1, 200)
    r = _rate("required_return", required_return, lo=0.0)
    _need(r > g2, "the required return must exceed the stable growth rate")
    pv1 = math.fsum(D0 * (1 + g1) ** k / (1 + r) ** k for k in range(1, n + 1))
    tv = D0 * (1 + g1) ** n * (1 + g2) / (r - g2)
    pvt = tv / (1 + r) ** n
    return {"price": pv1 + pvt, "terminal_value": tv, "pv_terminal": pvt, "pv_high_growth_dividends": pv1}


def _ref_two_stage(dividend, high_growth, high_growth_years, stable_growth, required_return):
    import mpmath                                          # every dividend of the infinite stream, discounted and summed
    n = high_growth_years
    with mpmath.workdps(30):
        g1, g2, r, D0 = (mpmath.mpf(x) for x in (high_growth, stable_growth, required_return, dividend))
        Dn = D0 * (1 + g1) ** n
        head = mpmath.fsum(D0 * (1 + g1) ** k / (1 + r) ** k for k in range(1, n + 1))
        tail = mpmath.nsum(lambda k: Dn * (1 + g2) ** (k - n) / (1 + r) ** k, [n + 1, mpmath.inf])
        return {"price": float(head + tail), "pv_high_growth_dividends": float(head), "pv_terminal": float(tail)}


_add("two_stage_ddm", name="Two-stage dividend discount model", topic="stocks / equity valuation",
     purpose="Share value when dividends grow at a high rate for n years and at a stable rate forever after.",
     inputs=[_i("dividend", "dividend just paid D0", "d0", "current dividend", "last dividend"),
             _i("high_growth", "growth rate in the first stage (decimal)", "g1", "high growth rate", "initial growth rate", "first stage growth"),
             _i("high_growth_years", "number of years of high growth", "n", "years of high growth", "high growth period", type="integer", min=1),
             _i("stable_growth", "growth rate forever after (decimal)", "g2", "stable growth rate", "long-term growth", "terminal growth"),
             _i("required_return", "required return on equity (decimal)", "r", "required rate of return", "cost of equity", "discount rate")],
     outputs=[_o("price", "share value"), _o("terminal_value", "value at year n of the dividends after n"),
              _o("pv_terminal", "present value of the terminal value"), _o("pv_high_growth_dividends", "present value of D1..Dn")],
     primary="price", conditions=["dividend > 0", "stable_growth < required_return", "high_growth_years a whole number >= 1"],
     method="P0 = sum_{k=1}^{n} D0 (1+g1)^k / (1+r)^k + [D_n (1 + g2)/(r - g2)] / (1 + r)^n",
     latex=r"P_0 = \sum_{k=1}^{n} \frac{D_0(1+g_1)^{k}}{(1+r)^{k}} + \frac{D_{n}(1+g_2)}{(r - g_2)(1+r)^{n}}", run=two_stage_ddm,
     reference=_ref_two_stage,
     wrong=lambda dividend, high_growth, high_growth_years, stable_growth, required_return: (lambda o: dict(
         o, price=o["pv_high_growth_dividends"] + o["pv_terminal"] / (1 + stable_growth)))(
         two_stage_ddm(dividend, high_growth, high_growth_years, stable_growth, required_return)),
     rand=lambda rng: (lambda r: {"dividend": _u(rng, 0.1, 10, 2), "high_growth": _u(rng, 0.05, 0.3), "high_growth_years": rng.randint(1, 15),
                                  "stable_growth": round(r - rng.uniform(0.02, 0.1), 4), "required_return": r})(_u(rng, 0.06, 0.2)),
     triggers=["two-stage dividend discount model", "two-stage ddm", "two-stage growth model", "two stage dividend"],
     templates=["Value the share with the two-stage dividend discount model with dividend = 2, high growth = 15%, "
                "high growth years = 5, stable growth = 4%, required return = 10%."],
     examples=[{"inputs": {"dividend": 2, "high_growth": 0.15, "high_growth_years": 5, "stable_growth": 0.04, "required_return": 0.1},
                "outputs": {'price': 54.74416165790129, 'terminal_value': 69.72704916666663, 'pv_terminal': 43.295011621577395, 'pv_high_growth_dividends': 11.449150036323893}}],
     questions=[{"q": "Value the share with the two-stage dividend discount model with dividend = 2, high growth = 15%, "
                      "high growth years = 5, stable growth = 4%, required return = 10%.", "outputs": {'price': 54.74416165790129, 'terminal_value': 69.72704916666663, 'pv_terminal': 43.295011621577395, 'pv_high_growth_dividends': 11.449150036323893}},
                {"q": "Value the share with the two-stage DDM with dividend = 2, g1 = 15%, n = 5, g2 = 12%, r = 10%.", "outcome": "INVALID_INPUT"}])


def gordon_implied_return(price, growth, dividend=None, next_dividend=None):
    P, g = _pos("price", price), _rate("growth", growth)
    D1 = _d1(dividend, next_dividend, g)
    _need(D1 > 0, "the dividend must be positive")
    r = D1 / P + g
    _need(-1 < r < 1, "the implied required return is outside -100%..100%")
    return {"required_return": r, "dividend_yield": D1 / P, "next_dividend": D1}


def _ref_implied_return(price, growth, dividend=None, next_dividend=None):
    from scipy.optimize import brentq
    D1 = next_dividend if next_dividend is not None else dividend * (1 + growth)
    return {"required_return": brentq(lambda r: D1 / (r - growth) - price, growth + 1e-12, 50, xtol=1e-16, rtol=1e-15, maxiter=1000)}


_add("gordon_implied_return", name="Required return implied by the Gordon model", topic="stocks / cost of equity",
     purpose="The return investors require given a share price and a constant dividend growth rate: r = D1 / P0 + g.",
     inputs=[_i("price", "current share price", "p0", "share price", "stock price", "market price"),
             _i("growth", "constant dividend growth rate (decimal)", "g", "growth rate", "dividend growth rate")]
     + [dict(i, _aliases=list(i["_aliases"])) for i in _DIV_IN],
     outputs=[_o("required_return", "implied required return D1/P0 + g"), _o("dividend_yield", "D1 / P0"), _o("next_dividend", "D1")],
     primary="required_return", conditions=["price > 0", "exactly one of dividend (D0), next_dividend (D1)", "dividend > 0"],
     method="r = D1 / P0 + g", latex=r"r = \frac{D_1}{P_0} + g", run=gordon_implied_return, reference=_ref_implied_return,
     wrong=lambda price, growth, dividend=None, next_dividend=None: (lambda d0: {
         "required_return": d0 / price + growth, "dividend_yield": d0 / price, "next_dividend": d0})(
         dividend if dividend is not None else next_dividend / (1 + growth)),
     rand=lambda rng: dict(_rand_div(rng), price=_u(rng, 5, 200, 2), growth=_u(rng, -0.02, 0.1)),
     triggers=["implied required return", "implied cost of equity", "required return implied", "implied return"],
     templates=["Find the implied required return with price = 50, dividend = 2, growth = 4%."],
     examples=[{"inputs": {"price": 50, "dividend": 2, "growth": 0.04}, "outputs": {'required_return': 0.0816, 'dividend_yield': 0.0416, 'next_dividend': 2.08}},
               {"inputs": {"price": 40, "next_dividend": 2, "growth": 0.05}, "outputs": {"required_return": 0.1}}],
     questions=[{"q": "Find the implied required return with price = 50, dividend = 2, growth = 4%.", "outputs": {'required_return': 0.0816, 'dividend_yield': 0.0416, 'next_dividend': 2.08}},
                {"q": "Find the implied cost of equity with share price = 40, next dividend = 2, growth = 5%.", "outputs": {"required_return": 0.1}}])


def capm(risk_free, beta, market_return):
    rf, b, rm = _rate("risk_free", risk_free), _fin("beta", beta), _rate("market_return", market_return)
    return {"expected_return": rf + b * (rm - rf), "market_risk_premium": rm - rf, "risk_premium": b * (rm - rf)}


def _ref_capm(risk_free, beta, market_return):
    import numpy as np                                     # the security market line through (0, rf) and (1, rm), evaluated at beta
    line = np.polyfit([0.0, 1.0], [risk_free, market_return], 1)
    return {"expected_return": float(np.polyval(line, beta)), "market_risk_premium": float(line[0])}


_add("capm", name="Capital asset pricing model", topic="stocks / asset pricing",
     purpose="Expected return of an asset from its beta: E[R] = rf + beta (E[Rm] - rf) (the security market line).",
     inputs=[_i("risk_free", "risk-free rate (decimal)", "rf", "risk-free rate", "risk free rate"),
             _i("beta", "beta of the asset", "b", "asset beta", "stock beta"),
             _i("market_return", "expected market return (decimal)", "rm", "expected market return", "market rate of return")],
     outputs=[_o("expected_return", "expected (required) return"), _o("market_risk_premium", "rm - rf"),
              _o("risk_premium", "beta (rm - rf)")], primary="expected_return",
     conditions=["-1 < risk_free < 1", "-1 < market_return < 1"], method="E[R] = rf + beta (rm - rf)",
     latex=r"E[R_i] = r_f + \beta_i\,(E[R_m] - r_f)", run=capm, reference=_ref_capm,
     wrong=lambda risk_free, beta, market_return: {"expected_return": risk_free + beta * market_return,
                                                   "market_risk_premium": market_return - risk_free, "risk_premium": beta * market_return},
     rand=lambda rng: {"risk_free": _u(rng, 0.0, 0.06), "beta": _u(rng, -0.5, 2.5), "market_return": _u(rng, 0.03, 0.15)},
     triggers=["capm", "capital asset pricing model", "security market line"],
     templates=["Find the expected return with CAPM with risk-free rate = 3%, beta = 1.2, market return = 9%."],
     examples=[{"inputs": {"risk_free": 0.03, "beta": 1.2, "market_return": 0.09}, "outputs": {"expected_return": 0.102}},
               {"inputs": {"risk_free": 0.02, "beta": 0.0, "market_return": 0.1}, "outputs": {"expected_return": 0.02}}],
     questions=[{"q": "Find the expected return with CAPM with risk-free rate = 3%, beta = 1.2, market return = 9%.", "outputs": {"expected_return": 0.102}},
                {"q": "Use the capital asset pricing model with rf = 2%, beta = 0.8.", "outcome": "NEEDS_INPUT"}])


def wacc(equity_value, debt_value, cost_of_equity, cost_of_debt, tax_rate):
    E, D = _nonneg("equity_value", equity_value), _nonneg("debt_value", debt_value)
    _need(E + D > 0, "the firm needs a positive total value")
    re_, rd = _rate("cost_of_equity", cost_of_equity), _rate("cost_of_debt", cost_of_debt)
    t = _prob("tax_rate", tax_rate, 0.0, 0.99)
    V = E + D
    return {"wacc": E / V * re_ + D / V * rd * (1 - t), "after_tax_cost_of_debt": rd * (1 - t),
            "equity_weight": E / V, "debt_weight": D / V}


_add("wacc", name="Weighted average cost of capital", topic="stocks / cost of capital",
     purpose="WACC = E/V re + D/V rd (1 - t): the after-tax cost of a firm's capital at market-value weights.",
     inputs=[_i("equity_value", "market value of equity", "e", "equity", "market value of equity"),
             _i("debt_value", "market value of debt", "d", "debt", "market value of debt"),
             _i("cost_of_equity", "cost of equity (decimal)", "re", "ke", "cost of equity"),
             _i("cost_of_debt", "pre-tax cost of debt (decimal)", "rd", "kd", "cost of debt", "pre-tax cost of debt"),
             _i("tax_rate", "corporate tax rate (decimal)", "t", "tax rate", "corporate tax rate", "tc")],
     outputs=[_o("wacc", "weighted average cost of capital"), _o("after_tax_cost_of_debt", "rd (1 - t)"),
              _o("equity_weight", "E / V"), _o("debt_weight", "D / V")], primary="wacc",
     conditions=["equity_value, debt_value >= 0 and not both 0", "0 <= tax_rate < 0.99"],
     method="WACC = E/V re + D/V rd (1 - t)", latex=r"\mathrm{WACC} = \frac{E}{V} r_e + \frac{D}{V} r_d (1 - t)", run=wacc,
     reference=lambda equity_value, debt_value, cost_of_equity, cost_of_debt, tax_rate: {"wacc": float(__import__("numpy").average(
         [cost_of_equity, cost_of_debt * (1 - tax_rate)], weights=[equity_value, debt_value]))},
     wrong=lambda equity_value, debt_value, cost_of_equity, cost_of_debt, tax_rate: (lambda o: dict(
         o, wacc=o["equity_weight"] * cost_of_equity + o["debt_weight"] * cost_of_debt))(
         wacc(equity_value, debt_value, cost_of_equity, cost_of_debt, tax_rate)),
     rand=lambda rng: {"equity_value": _u(rng, 1e5, 1e9, 0), "debt_value": _u(rng, 1e5, 1e9, 0), "cost_of_equity": _u(rng, 0.06, 0.2),
                       "cost_of_debt": _u(rng, 0.02, 0.1), "tax_rate": _u(rng, 0.1, 0.4)},
     triggers=["weighted average cost of capital", "wacc"],
     templates=["Find the WACC with equity value = 600, debt value = 400, cost of equity = 12%, cost of debt = 6%, tax rate = 25%."],
     examples=[{"inputs": {"equity_value": 600, "debt_value": 400, "cost_of_equity": 0.12, "cost_of_debt": 0.06, "tax_rate": 0.25},
                "outputs": {"wacc": 0.09}}],
     questions=[{"q": "Find the WACC with equity value = 600, debt value = 400, cost of equity = 12%, cost of debt = 6%, tax rate = 25%.",
                 "outputs": {"wacc": 0.09}},
                {"q": "Find the WACC with equity value = 0, debt value = 0, cost of equity = 12%, cost of debt = 6%, tax rate = 25%.",
                 "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# RETURNS AND PORTFOLIO THEORY
# =====================================================================================================================
def _returns(name, xs, n_min=2, hi=1.0):
    xs = _vec(name, xs, n_min)
    _need(all(-1 < x < hi for x in xs), f"{name}: each return is a decimal between -100% and {hi:.0%} (write 5% or 0.05)")
    return xs


def _mean(xs):
    return math.fsum(xs) / len(xs)


def _cov(xs, ys, ddof=1):
    mx, my = _mean(xs), _mean(ys)
    return math.fsum((x - mx) * (y - my) for x, y in zip(xs, ys)) / (len(xs) - ddof)


def _sd(xs, ddof=1):
    return math.sqrt(_cov(xs, xs, ddof))


def _pair(a, b, na, nb, n_min=3):
    a, b = _returns(na, a, n_min), _returns(nb, b, n_min)
    _need(len(a) == len(b), f"{na} and {nb} must have the same number of observations")
    return a, b


def _rand_pair(rng, n_lo=8, n_hi=60):
    n = rng.randint(n_lo, n_hi)
    beta, alpha = rng.uniform(-0.5, 2.0), rng.uniform(-0.01, 0.01)
    m = [round(rng.gauss(0.008, 0.045), 5) for _ in range(n)]
    a = [round(max(-0.9, min(0.9, alpha + beta * x + rng.gauss(0, 0.03))), 5) for x in m]
    return a, m


def beta_regression(asset_returns, market_returns):
    y, x = _pair(asset_returns, market_returns, "asset_returns", "market_returns")
    sxx, syy, sxy = _cov(x, x), _cov(y, y), _cov(x, y)
    _need(sxx > 0, "the market returns must not all be equal")
    _need(syy > 0, "the asset returns must not all be equal")
    b = sxy / sxx
    return {"beta": b, "alpha": _mean(y) - b * _mean(x), "correlation": sxy / math.sqrt(sxx * syy), "r_squared": sxy * sxy / (sxx * syy)}


def _ref_beta(asset_returns, market_returns):
    import numpy as np
    import statsmodels.api as sm
    fit = sm.OLS(np.asarray(asset_returns), sm.add_constant(np.asarray(market_returns))).fit()
    return {"alpha": float(fit.params[0]), "beta": float(fit.params[1]), "r_squared": float(fit.rsquared),
            "correlation": float(np.corrcoef(asset_returns, market_returns)[0, 1])}


_add("beta_regression", name="Beta of an asset from return series", topic="portfolio theory / systematic risk",
     purpose="Beta (and alpha, correlation, R^2) of an asset from paired asset and market returns: the slope of the "
             "ordinary least-squares regression of asset on market returns.",
     inputs=[_i("asset_returns", "returns of the asset (decimals), one per period", "asset", "stock returns", "returns", "y", type="vector", min_length=3),
             _i("market_returns", "returns of the market in the same periods", "market", "index returns", "benchmark returns", "x",
                type="vector", min_length=3)],
     outputs=[_o("beta", "cov(asset, market) / var(market)"), _o("alpha", "intercept mean(asset) - beta mean(market)"),
              _o("correlation", "correlation of asset and market"), _o("r_squared", "share of variance explained")], primary="beta",
     conditions=["same number of observations, at least 3", "returns between -100% and 100%", "neither series constant"],
     method="beta = S_xy / S_xx (sample covariance over sample variance)", latex=r"\beta = \frac{\mathrm{Cov}(R_i, R_m)}{\mathrm{Var}(R_m)}",
     run=beta_regression, reference=_ref_beta,
     wrong=lambda asset_returns, market_returns: (lambda o: dict(o, beta=_cov(asset_returns, market_returns) / _cov(asset_returns, asset_returns)))(
         beta_regression(asset_returns, market_returns)),
     rand=lambda rng: (lambda p: {"asset_returns": p[0], "market_returns": p[1]})(_rand_pair(rng)),
     triggers=["beta of", "beta from", "estimate beta", "estimate the beta", "calculate beta", "regression beta", "stock beta"],
     templates=["Estimate the beta with asset returns = [0.02, -0.01, 0.03, 0.015, -0.02], market returns = [0.015, -0.005, 0.02, 0.01, -0.015]."],
     examples=[{"inputs": {"asset_returns": "0.02, -0.01, 0.03, 0.015, -0.02", "market_returns": "0.015, -0.005, 0.02, 0.01, -0.015"},
                "outputs": {'beta': 1.4411764705882353, 'alpha': -0.00020588235294117727, 'correlation': 0.9959020462162099, 'r_squared': 0.9918208856576339}},
               {"inputs": {"asset_returns": [0.02, 0.04, 0.06], "market_returns": [0.01, 0.02, 0.03]}, "outputs": {"beta": 2.0, "alpha": 0.0}}],
     questions=[{"q": "Estimate the beta with asset returns = [0.02, -0.01, 0.03, 0.015, -0.02], market returns = [0.015, -0.005, 0.02, 0.01, -0.015].",
                 "outputs": {'beta': 1.4411764705882353, 'alpha': -0.00020588235294117727, 'correlation': 0.9959020462162099, 'r_squared': 0.9918208856576339}},
                {"q": "Estimate the beta with asset returns = [2%, 4%, 6%], market returns = [1%, 2%, 3%].", "outputs": {"beta": 2.0}},
                {"q": "Estimate the beta with asset returns = [0.02, 0.04, 0.06], market returns = [0.01, 0.01, 0.01].", "outcome": "INVALID_INPUT"},
                {"q": "Estimate the beta with asset returns = [0.02, 0.04, 0.06], market returns = [0.01, 0.02].", "outcome": "INVALID_INPUT"}])


def sharpe_ratio(returns, risk_free=0.0, periods_per_year=1):
    r = _returns("returns", returns)
    rf = _rate("risk_free", risk_free)
    m = _int("periods_per_year", periods_per_year, 1, 100000)
    s = _sd(r)
    _need(s > 0, "the returns must not all be equal (zero standard deviation)")
    sh = (_mean(r) - rf) / s
    return {"sharpe": sh, "annualized_sharpe": sh * math.sqrt(m), "mean_excess_return": _mean(r) - rf, "std_dev": s}


_add("sharpe_ratio", name="Sharpe ratio", topic="portfolio theory / performance measures",
     purpose="Mean excess return per unit of total risk, from a return series (sample standard deviation, n - 1); "
             "annualized by sqrt(periods per year).",
     inputs=[_i("returns", "periodic returns (decimals)", "portfolio returns", "fund returns", "r", type="vector", min_length=2),
             _i("risk_free", "risk-free rate per period (decimal, default 0)", "rf", "risk-free rate", "risk free rate", required=False),
             _i("periods_per_year", "periods per year for annualizing (12 monthly, 252 daily; default 1)", "m", "periods a year",
                type="integer", required=False, min=1)],
     outputs=[_o("sharpe", "(mean - rf) / sd per period"), _o("annualized_sharpe", "sharpe x sqrt(periods per year)"),
              _o("mean_excess_return", "mean - rf"), _o("std_dev", "sample standard deviation")], primary="sharpe",
     conditions=["at least 2 returns, not all equal", "returns between -100% and 100%"],
     method="S = (mean(R) - rf) / s(R), s with n - 1", latex=r"S = \frac{\bar R - r_f}{s_R}", run=sharpe_ratio,
     reference=lambda returns, risk_free=0.0, periods_per_year=1: (lambda np: (lambda a: {
         "sharpe": float((a.mean() - risk_free) / a.std(ddof=1)), "std_dev": float(a.std(ddof=1)),
         "annualized_sharpe": float((a.mean() - risk_free) / a.std(ddof=1) * np.sqrt(periods_per_year))})(np.asarray(returns)))(__import__("numpy")),
     wrong=lambda returns, risk_free=0.0, periods_per_year=1: (lambda o: dict(o, sharpe=o["mean_excess_return"] / _sd(returns, 0)))(
         sharpe_ratio(returns, risk_free, periods_per_year)),
     rand=lambda rng: {"returns": [round(rng.gauss(0.01, 0.04), 5) for _ in range(rng.randint(5, 60))], "risk_free": _u(rng, 0, 0.005, 5),
                       "periods_per_year": rng.choice([1, 12, 52, 252])},
     triggers=["sharpe ratio", "sharpe"],
     templates=["Find the Sharpe ratio with returns = [0.05, 0.02, -0.01, 0.04], risk-free rate = 0.5%."],
     examples=[{"inputs": {"returns": "0.05, 0.02, -0.01, 0.04", "risk_free": 0.005}, "outputs": {'sharpe': 0.7559289460184544, 'annualized_sharpe': 0.7559289460184544, 'mean_excess_return': 0.02, 'std_dev': 0.026457513110645908}},
               {"inputs": {"returns": [0.01, 0.03], "risk_free": 0.0, "periods_per_year": 12}, "outputs": {'sharpe': 1.4142135623730951, 'annualized_sharpe': 4.898979485566357, 'mean_excess_return': 0.02, 'std_dev': 0.01414213562373095}}],
     questions=[{"q": "Find the Sharpe ratio with returns = [0.05, 0.02, -0.01, 0.04], risk-free rate = 0.5%.", "outputs": {'sharpe': 0.7559289460184544, 'annualized_sharpe': 0.7559289460184544, 'mean_excess_return': 0.02, 'std_dev': 0.026457513110645908}},
                {"q": "Find the Sharpe ratio with returns = [0.02, 0.02, 0.02].", "outcome": "INVALID_INPUT"}])


def sortino_ratio(returns, target=0.0):
    r = _returns("returns", returns)
    T = _rate("target", target)
    dd = math.sqrt(math.fsum(min(0.0, x - T) ** 2 for x in r) / len(r))
    _need(dd > 0, "no return is below the target: the downside deviation is zero")
    return {"sortino": (_mean(r) - T) / dd, "downside_deviation": dd, "mean_return": _mean(r)}


_add("sortino_ratio", name="Sortino ratio", topic="portfolio theory / performance measures",
     purpose="Mean return above a target per unit of downside deviation sqrt(mean(min(0, R - T)^2)) over ALL "
             "observations.",
     inputs=[_i("returns", "periodic returns (decimals)", "portfolio returns", "r", type="vector", min_length=2),
             _i("target", "target (minimum acceptable) return per period (default 0)", "mar", "minimum acceptable return",
                "target return", "t", required=False)],
     outputs=[_o("sortino", "(mean - T) / downside deviation"), _o("downside_deviation", "sqrt(sum min(0, R - T)^2 / n)"),
              _o("mean_return", "mean return")], primary="sortino",
     conditions=["at least 2 returns", "at least one return below the target"],
     method="Sortino = (mean(R) - T) / sqrt((1/n) sum min(0, R - T)^2)",
     latex=r"\mathrm{Sortino} = \frac{\bar R - T}{\sqrt{\frac{1}{n}\sum_i \min(0, R_i - T)^{2}}}", run=sortino_ratio,
     reference=lambda returns, target=0.0: (lambda np: (lambda a: (lambda dd: {"sortino": float((a.mean() - target) / dd),
                                                                               "downside_deviation": float(dd)})(
         np.sqrt(np.mean(np.minimum(0.0, a - target) ** 2))))(np.asarray(returns)))(__import__("numpy")),
     wrong=lambda returns, target=0.0: (lambda o: dict(o, sortino=(o["mean_return"] - target) / _sd(returns)))(sortino_ratio(returns, target)),
     rand=lambda rng: {"returns": [round(rng.gauss(0.01, 0.04), 5) for _ in range(rng.randint(5, 60))], "target": _u(rng, -0.005, 0.01, 5)},
     triggers=["sortino ratio", "sortino"],
     templates=["Find the Sortino ratio with returns = [0.05, 0.02, -0.01, 0.04, -0.03], target = 0."],
     examples=[{"inputs": {"returns": "0.05, 0.02, -0.01, 0.04, -0.03", "target": 0.0}, "outputs": {'sortino': 0.9899494936611667, 'downside_deviation': 0.01414213562373095, 'mean_return': 0.014000000000000002}}],
     questions=[{"q": "Find the Sortino ratio with returns = [0.05, 0.02, -0.01, 0.04, -0.03], target = 0.", "outputs": {'sortino': 0.9899494936611667, 'downside_deviation': 0.01414213562373095, 'mean_return': 0.014000000000000002}},
                {"q": "Find the Sortino ratio with returns = [0.05, 0.02, 0.04].", "outcome": "INVALID_INPUT"}])


def treynor_jensen(portfolio_returns, market_returns, risk_free=0.0):
    p, m = _pair(portfolio_returns, market_returns, "portfolio_returns", "market_returns")
    rf = _rate("risk_free", risk_free)
    sxx = _cov(m, m)
    _need(sxx > 0, "the market returns must not all be equal")
    b = _cov(p, m) / sxx
    _need(abs(b) > 1e-9, "beta is zero: the Treynor ratio is undefined")
    mp, mm = _mean(p), _mean(m)
    return {"beta": b, "treynor": (mp - rf) / b, "jensen_alpha": mp - (rf + b * (mm - rf)), "mean_portfolio_return": mp,
            "mean_market_return": mm}


def _ref_treynor(portfolio_returns, market_returns, risk_free=0.0):
    import numpy as np
    import statsmodels.api as sm                           # regression of EXCESS returns: alpha is the intercept
    ex_p, ex_m = np.asarray(portfolio_returns) - risk_free, np.asarray(market_returns) - risk_free
    fit = sm.OLS(ex_p, sm.add_constant(ex_m)).fit()
    b = float(fit.params[1])
    return {"beta": b, "jensen_alpha": float(fit.params[0]), "treynor": float(ex_p.mean() / b)}


_add("treynor_jensen", name="Treynor ratio and Jensen's alpha", topic="portfolio theory / performance measures",
     purpose="Excess return per unit of beta (Treynor) and the return above the CAPM prediction (Jensen's alpha), "
             "from paired portfolio and market returns and a constant risk-free rate per period.",
     inputs=[_i("portfolio_returns", "portfolio returns (decimals) per period", "portfolio", "fund returns", "returns", type="vector", min_length=3),
             _i("market_returns", "market returns in the same periods", "market", "benchmark returns", "index returns", type="vector", min_length=3),
             _i("risk_free", "risk-free rate per period (decimal, default 0)", "rf", "risk-free rate", "risk free rate", required=False)],
     outputs=[_o("beta", "beta of the portfolio"), _o("treynor", "(mean Rp - rf) / beta"),
              _o("jensen_alpha", "mean Rp - [rf + beta (mean Rm - rf)]"), _o("mean_portfolio_return", "mean Rp"),
              _o("mean_market_return", "mean Rm")], primary="treynor",
     conditions=["same number of observations, at least 3", "market returns not constant", "beta not 0"],
     method="beta = S_pm / S_mm; Treynor = (mean Rp - rf)/beta; alpha = mean Rp - rf - beta (mean Rm - rf)",
     latex=r"T = \frac{\bar R_p - r_f}{\beta_p},\quad \alpha_J = \bar R_p - [r_f + \beta_p(\bar R_m - r_f)]", run=treynor_jensen,
     reference=_ref_treynor,
     wrong=lambda portfolio_returns, market_returns, risk_free=0.0: (lambda o: dict(
         o, treynor=(o["mean_portfolio_return"] - risk_free) / _sd(portfolio_returns)))(treynor_jensen(portfolio_returns, market_returns, risk_free)),
     rand=lambda rng: (lambda pr: {"portfolio_returns": pr[0], "market_returns": pr[1], "risk_free": _u(rng, 0, 0.004, 5)})(_rand_pair(rng)),
     triggers=["treynor ratio", "jensen's alpha", "jensen alpha", "jensens alpha", "treynor"],
     templates=["Find the Treynor ratio and Jensen's alpha with portfolio returns = [0.03, -0.01, 0.04, 0.02], "
                "market returns = [0.02, -0.02, 0.03, 0.01], risk-free rate = 0.2%."],
     examples=[{"inputs": {"portfolio_returns": "0.03, -0.01, 0.04, 0.02", "market_returns": "0.02, -0.02, 0.03, 0.01", "risk_free": 0.002},
                "outputs": {'beta': 1.0000000000000002, 'treynor': 0.018, 'jensen_alpha': 0.009999999999999998, 'mean_portfolio_return': 0.02, 'mean_market_return': 0.01}}],
     questions=[{"q": "Find the Treynor ratio and Jensen's alpha with portfolio returns = [0.03, -0.01, 0.04, 0.02], "
                      "market returns = [0.02, -0.02, 0.03, 0.01], risk-free rate = 0.2%.", "outputs": {'beta': 1.0000000000000002, 'treynor': 0.018, 'jensen_alpha': 0.009999999999999998, 'mean_portfolio_return': 0.02, 'mean_market_return': 0.01}}])


def information_ratio(portfolio_returns, benchmark_returns):
    p, b = _pair(portfolio_returns, benchmark_returns, "portfolio_returns", "benchmark_returns", 2)
    a = [x - y for x, y in zip(p, b)]
    te = _sd(a)
    _need(te > 0, "the active returns are constant: the tracking error is zero")
    return {"information_ratio": _mean(a) / te, "tracking_error": te, "mean_active_return": _mean(a)}


_add("information_ratio", name="Information ratio and tracking error", topic="portfolio theory / performance measures",
     purpose="Mean active return (portfolio - benchmark) per unit of tracking error (sample sd of active returns).",
     inputs=[_i("portfolio_returns", "portfolio returns (decimals)", "portfolio", "fund returns", "returns", type="vector", min_length=2),
             _i("benchmark_returns", "benchmark returns in the same periods", "benchmark", "index returns", type="vector", min_length=2)],
     outputs=[_o("information_ratio", "mean active / tracking error"), _o("tracking_error", "sample sd of active returns"),
              _o("mean_active_return", "mean of portfolio - benchmark")], primary="information_ratio",
     conditions=["same number of observations, at least 2", "active returns not constant"],
     method="IR = mean(Rp - Rb) / s(Rp - Rb)", latex=r"IR = \frac{\overline{R_p - R_b}}{s(R_p - R_b)}", run=information_ratio,
     reference=lambda portfolio_returns, benchmark_returns: (lambda a: {"information_ratio": float(a.mean() / a.std(ddof=1)),
                                                                        "tracking_error": float(a.std(ddof=1))})(
         __import__("numpy").asarray(portfolio_returns) - __import__("numpy").asarray(benchmark_returns)),
     wrong=lambda portfolio_returns, benchmark_returns: (lambda o: dict(o, tracking_error=_sd([x - y for x, y in zip(
         portfolio_returns, benchmark_returns)], 0)))(information_ratio(portfolio_returns, benchmark_returns)),
     rand=lambda rng: (lambda pr: {"portfolio_returns": pr[0], "benchmark_returns": pr[1]})(_rand_pair(rng, 4, 60)),
     triggers=["information ratio", "tracking error"],
     templates=["Find the information ratio with portfolio returns = [0.03, 0.01, 0.04, 0.02], benchmark returns = [0.02, 0.015, 0.03, 0.01]."],
     examples=[{"inputs": {"portfolio_returns": "0.03, 0.01, 0.04, 0.02", "benchmark_returns": "0.02, 0.015, 0.03, 0.01"}, "outputs": {'information_ratio': 0.8333333333333334, 'tracking_error': 0.0075, 'mean_active_return': 0.00625}}],
     questions=[{"q": "Find the information ratio with portfolio returns = [0.03, 0.01, 0.04, 0.02], benchmark returns = [0.02, 0.015, 0.03, 0.01].",
                 "outputs": {'information_ratio': 0.8333333333333334, 'tracking_error': 0.0075, 'mean_active_return': 0.00625}}])


def _square(name, A, n=None):
    _need(isinstance(A, (list, tuple)) and len(A) >= 1 and all(isinstance(r, (list, tuple)) for r in A), f"{name} must be a matrix")
    k = len(A)
    _need(all(len(r) == k for r in A), f"{name} must be square")
    _need(n is None or k == n, f"{name} must be {n} x {n} (one row and column per asset)")
    M = [[_fin(name, x) for x in r] for r in A]
    scale = max(abs(x) for r in M for x in r) or 1.0
    _need(all(abs(M[i][j] - M[j][i]) <= 1e-12 * scale for i in range(k) for j in range(k)), f"{name} must be symmetric")
    _need(all(M[i][i] >= 0 for i in range(k)), f"{name}: variances on the diagonal must not be negative")
    return M


def _cholesky(M):
    """Lower-triangular L with L L' = M, or None if M is not positive definite."""
    n = len(M)
    L = [[0.0] * n for _ in range(n)]
    tol = 1e-12 * max(M[i][i] for i in range(n))
    for i in range(n):
        for j in range(i + 1):
            s = M[i][j] - math.fsum(L[i][k] * L[j][k] for k in range(j))
            if i == j:
                if s <= tol:
                    return None
                L[i][i] = math.sqrt(s)
            else:
                L[i][j] = s / L[j][j]
    return L


def _chol_solve(L, b):
    n = len(L)
    y = [0.0] * n
    for i in range(n):
        y[i] = (b[i] - math.fsum(L[i][k] * y[k] for k in range(i))) / L[i][i]
    x = [0.0] * n
    for i in reversed(range(n)):
        x[i] = (y[i] - math.fsum(L[k][i] * x[k] for k in range(i + 1, n))) / L[i][i]
    return x


def _quad(w, S):
    return math.fsum(w[i] * S[i][j] * w[j] for i in range(len(w)) for j in range(len(w)))


def _rand_cov(rng, n):
    sd = [rng.uniform(0.05, 0.5) for _ in range(n)]
    A = [[rng.gauss(0, 1) for _ in range(n)] for _ in range(n)]
    C = [[sum(A[i][k] * A[j][k] for k in range(n)) + (n if i == j else 0) for j in range(n)] for i in range(n)]
    R = [[C[i][j] / math.sqrt(C[i][i] * C[j][j]) for j in range(n)] for i in range(n)]
    S = [[round(R[i][j] * sd[i] * sd[j], 6) for j in range(n)] for i in range(n)]
    for i in range(n):
        for j in range(i):
            S[i][j] = S[j][i]
    return S


def _rand_weights(rng, n):
    u = [rng.uniform(0.05, 1) for _ in range(n)]
    w = [round(x / sum(u), 4) for x in u]
    w[-1] = round(1 - sum(w[:-1]), 10)
    return w


def portfolio_return_risk(weights, expected_returns, covariance):
    w = _vec("weights", weights, 1)
    mu = _returns("expected_returns", expected_returns, 1)
    _need(len(w) == len(mu), "weights and expected_returns need one entry per asset")
    S = _square("covariance", covariance, len(w))
    _need(abs(math.fsum(w) - 1) <= 1e-9, "the weights must add up to 1")
    var = _quad(w, S)
    _need(var >= -1e-15, "the covariance matrix is not positive semidefinite (negative portfolio variance)")
    var = max(var, 0.0)
    return {"expected_return": math.fsum(a * b for a, b in zip(w, mu)), "variance": var, "std_dev": math.sqrt(var)}


def _ref_portfolio(weights, expected_returns, covariance):
    import numpy as np
    w, S = np.asarray(weights), np.asarray(covariance)
    v = float(w @ S @ w)
    return {"expected_return": float(w @ np.asarray(expected_returns)), "variance": v, "std_dev": float(np.sqrt(v))}


def _rand_portfolio(rng):
    n = rng.randint(2, 6)
    return {"weights": _rand_weights(rng, n), "expected_returns": [_u(rng, 0.01, 0.2) for _ in range(n)], "covariance": _rand_cov(rng, n)}


_add("portfolio_return_risk", name="Portfolio expected return and risk", topic="portfolio theory / mean-variance",
     purpose="Expected return w'mu, variance w'Sigma w and standard deviation of a portfolio from its weights, "
             "expected returns and covariance matrix.",
     inputs=[_i("weights", "portfolio weights, adding up to 1 (negative = short)", "w", "portfolio weights", type="vector", min_length=1),
             _i("expected_returns", "expected return of each asset (decimals)", "mu", "returns", "expected return", "means", type="vector", min_length=1),
             _i("covariance", "covariance matrix of the returns [[s11, s12], [s21, s22]]", "covariance matrix", "cov", "sigma", type="matrix")],
     outputs=[_o("expected_return", "w' mu"), _o("variance", "w' Sigma w"), _o("std_dev", "sqrt(variance)")], primary="std_dev",
     conditions=["one weight, return and row per asset", "weights add up to 1", "covariance symmetric, positive semidefinite"],
     method="E = sum w_i mu_i; Var = sum_i sum_j w_i w_j sigma_ij", latex=r"E[R_p] = w^{\top}\mu,\quad \sigma_p^{2} = w^{\top}\Sigma\,w",
     run=portfolio_return_risk, reference=_ref_portfolio,
     wrong=lambda weights, expected_returns, covariance: (lambda o: (lambda v: dict(o, variance=v, std_dev=math.sqrt(v)))(
         math.fsum(w * w * covariance[i][i] for i, w in enumerate(weights))))(portfolio_return_risk(weights, expected_returns, covariance)),
     rand=_rand_portfolio,
     triggers=["portfolio expected return and risk", "portfolio variance", "portfolio risk", "portfolio standard deviation",
               "expected return and variance of the portfolio", "expected return of the portfolio"],
     templates=["Find the portfolio variance with weights = [0.6, 0.4], expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.006, 0.09]]."],
     examples=[{"inputs": {"weights": "0.6, 0.4", "expected_returns": "0.1, 0.15", "covariance": "[[0.04, 0.006], [0.006, 0.09]]"},
                "outputs": {'expected_return': 0.12, 'variance': 0.03168, 'std_dev': 0.1779887636902959}},
               {"inputs": {"weights": [1.0], "expected_returns": [0.08], "covariance": [[0.04]]}, "outputs": {"std_dev": 0.2, "expected_return": 0.08}}],
     questions=[{"q": "Find the portfolio variance with weights = [0.6, 0.4], expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.006, 0.09]].",
                 "outputs": {'expected_return': 0.12, 'variance': 0.03168, 'std_dev': 0.1779887636902959}},
                {"q": "Find the portfolio variance with weights = [0.6, 0.6], expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.006, 0.09]].",
                 "outcome": "INVALID_INPUT"},
                {"q": "Find the portfolio variance with weights = [0.6, 0.4], expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.007, 0.09]].",
                 "outcome": "INVALID_INPUT"}])


def two_asset_portfolio(weight_1, return_1, return_2, sd_1, sd_2, correlation):
    w = _fin("weight_1", weight_1)
    r1, r2 = _returns("return_1", [return_1], 1)[0], _returns("return_2", [return_2], 1)[0]
    s1, s2 = _nonneg("sd_1", sd_1), _nonneg("sd_2", sd_2)
    rho = _prob("correlation", correlation, -1.0, 1.0)
    var = w * w * s1 * s1 + (1 - w) ** 2 * s2 * s2 + 2 * w * (1 - w) * rho * s1 * s2
    var = max(var, 0.0)
    return {"expected_return": w * r1 + (1 - w) * r2, "std_dev": math.sqrt(var), "variance": var, "weight_2": 1 - w}


def _ref_two_asset(weight_1, return_1, return_2, sd_1, sd_2, correlation):
    import numpy as np
    w = np.array([weight_1, 1 - weight_1])
    D = np.diag([sd_1, sd_2])
    S = D @ np.array([[1.0, correlation], [correlation, 1.0]]) @ D
    v = max(float(w @ S @ w), 0.0)
    return {"expected_return": float(w @ np.array([return_1, return_2])), "variance": v, "std_dev": float(np.sqrt(v))}


_add("two_asset_portfolio", name="Two-asset portfolio return and risk", topic="portfolio theory / mean-variance",
     purpose="Expected return and standard deviation of a portfolio of two assets from the weight, means, standard "
             "deviations and correlation.",
     inputs=[_i("weight_1", "weight of asset 1 (asset 2 gets 1 - w1)", "w1", "weight of asset 1", "weight"),
             _i("return_1", "expected return of asset 1 (decimal)", "r1", "expected return 1", "mean 1"),
             _i("return_2", "expected return of asset 2 (decimal)", "r2", "expected return 2", "mean 2"),
             _i("sd_1", "standard deviation of asset 1 (decimal)", "sd1", "sigma1", "s1", "standard deviation 1", "volatility 1"),
             _i("sd_2", "standard deviation of asset 2 (decimal)", "sd2", "sigma2", "s2", "standard deviation 2", "volatility 2"),
             _i("correlation", "correlation between the two assets (-1..1)", "rho", "correlation coefficient")],
     outputs=[_o("expected_return", "w1 r1 + w2 r2"), _o("std_dev", "portfolio standard deviation"), _o("variance", "portfolio variance"),
              _o("weight_2", "1 - w1")], primary="std_dev", conditions=["-1 <= correlation <= 1", "standard deviations >= 0"],
     method="var = w1^2 s1^2 + w2^2 s2^2 + 2 w1 w2 rho s1 s2",
     latex=r"\sigma_p^{2} = w_1^{2}\sigma_1^{2} + w_2^{2}\sigma_2^{2} + 2 w_1 w_2 \rho\,\sigma_1\sigma_2", run=two_asset_portfolio,
     reference=_ref_two_asset,
     wrong=lambda weight_1, return_1, return_2, sd_1, sd_2, correlation: (lambda o: dict(
         o, std_dev=abs(weight_1 * sd_1 + (1 - weight_1) * sd_2)))(two_asset_portfolio(weight_1, return_1, return_2, sd_1, sd_2, correlation)),
     rand=lambda rng: {"weight_1": _u(rng, -0.3, 1.3), "return_1": _u(rng, 0.0, 0.2), "return_2": _u(rng, 0.0, 0.2),
                       "sd_1": _u(rng, 0.05, 0.5), "sd_2": _u(rng, 0.05, 0.5), "correlation": _u(rng, -0.95, 0.95)},
     triggers=["two-asset portfolio", "two asset portfolio", "portfolio of two assets", "portfolio of two stocks"],
     templates=["Find the two-asset portfolio return and risk with w1 = 0.6, r1 = 10%, r2 = 15%, sd1 = 20%, sd2 = 30%, correlation = 0.1."],
     examples=[{"inputs": {"weight_1": 0.6, "return_1": 0.1, "return_2": 0.15, "sd_1": 0.2, "sd_2": 0.3, "correlation": 0.1}, "outputs": {'expected_return': 0.12, 'std_dev': 0.1779887636902959, 'variance': 0.03168, 'weight_2': 0.4}},
               {"inputs": {"weight_1": 0.5, "return_1": 0.1, "return_2": 0.1, "sd_1": 0.2, "sd_2": 0.2, "correlation": -1.0},
                "outputs": {"std_dev": 0.0, "expected_return": 0.1}}],
     questions=[{"q": "Find the two-asset portfolio return and risk with w1 = 0.6, r1 = 10%, r2 = 15%, sd1 = 20%, sd2 = 30%, correlation = 0.1.",
                 "outputs": {'expected_return': 0.12, 'std_dev': 0.1779887636902959, 'variance': 0.03168, 'weight_2': 0.4}},
                {"q": "Find the two-asset portfolio return and risk with w1 = 0.6, r1 = 10%, r2 = 15%, sd1 = 20%, sd2 = 30%, correlation = 1.5.",
                 "outcome": "INVALID_INPUT"}])


def min_variance_two_asset(sd_1, sd_2, correlation):
    s1, s2 = _pos("sd_1", sd_1), _pos("sd_2", sd_2)
    rho = _prob("correlation", correlation, -1.0, 1.0)
    den = s1 * s1 + s2 * s2 - 2 * rho * s1 * s2
    _need(den > 1e-12 * (s1 * s1 + s2 * s2), "the two assets are perfectly correlated with equal risk: every mix has the same variance")
    w1 = (s2 * s2 - rho * s1 * s2) / den
    var = max(w1 * w1 * s1 * s1 + (1 - w1) ** 2 * s2 * s2 + 2 * w1 * (1 - w1) * rho * s1 * s2, 0.0)
    return {"weight_1": w1, "weight_2": 1 - w1, "std_dev": math.sqrt(var)}


def _ref_minvar2(sd_1, sd_2, correlation):
    import numpy as np                                     # Lagrange conditions of min w'Sw subject to w1 + w2 = 1
    S = np.array([[sd_1 ** 2, correlation * sd_1 * sd_2], [correlation * sd_1 * sd_2, sd_2 ** 2]])
    K = np.block([[2 * S, np.ones((2, 1))], [np.ones((1, 2)), np.zeros((1, 1))]])
    sol = np.linalg.solve(K, np.array([0.0, 0.0, 1.0]))
    w = sol[:2]
    return {"weight_1": float(w[0]), "weight_2": float(w[1]), "std_dev": float(np.sqrt(max(w @ S @ w, 0.0)))}


_add("min_variance_two_asset", name="Minimum-variance portfolio of two assets", topic="portfolio theory / mean-variance",
     purpose="The weights of two assets that minimise the portfolio variance (short sales allowed), and that variance's sd.",
     inputs=[_i("sd_1", "standard deviation of asset 1 (decimal)", "sd1", "sigma1", "s1", "standard deviation 1", "volatility 1"),
             _i("sd_2", "standard deviation of asset 2 (decimal)", "sd2", "sigma2", "s2", "standard deviation 2", "volatility 2"),
             _i("correlation", "correlation between the assets (-1..1)", "rho", "correlation coefficient")],
     outputs=[_o("weight_1", "w1 = (s2^2 - rho s1 s2) / (s1^2 + s2^2 - 2 rho s1 s2)"), _o("weight_2", "1 - w1"),
              _o("std_dev", "standard deviation of the minimum-variance portfolio")], primary="weight_1",
     conditions=["sd_1, sd_2 > 0", "-1 <= correlation <= 1", "not (correlation = 1 and sd_1 = sd_2)"],
     method="w1 = (s2^2 - rho s1 s2) / (s1^2 + s2^2 - 2 rho s1 s2)",
     latex=r"w_1^{*} = \frac{\sigma_2^{2} - \rho\sigma_1\sigma_2}{\sigma_1^{2} + \sigma_2^{2} - 2\rho\sigma_1\sigma_2}",
     run=min_variance_two_asset, reference=_ref_minvar2,
     wrong=lambda sd_1, sd_2, correlation: (lambda w: {"weight_1": w, "weight_2": 1 - w, "std_dev": math.sqrt(
         max(w * w * sd_1 ** 2 + (1 - w) ** 2 * sd_2 ** 2 + 2 * w * (1 - w) * correlation * sd_1 * sd_2, 0.0))})(sd_2 ** 2 / (sd_1 ** 2 + sd_2 ** 2)),
     rand=lambda rng: {"sd_1": _u(rng, 0.05, 0.5), "sd_2": _u(rng, 0.05, 0.5), "correlation": _u(rng, -0.95, 0.95)},
     triggers=["minimum-variance portfolio of two assets", "minimum variance weights", "minimum-variance weights",
               "minimum variance portfolio of two", "two-asset minimum variance"],
     templates=["Find the minimum variance weights with sd1 = 20%, sd2 = 30%, correlation = 0.1."],
     examples=[{"inputs": {"sd_1": 0.2, "sd_2": 0.3, "correlation": 0.1}, "outputs": {'weight_1': 0.7118644067796609, 'weight_2': 0.2881355932203391, 'std_dev': 0.17379122483747123}},
               {"inputs": {"sd_1": 0.2, "sd_2": 0.2, "correlation": 0.3}, "outputs": {"weight_1": 0.5}}],
     questions=[{"q": "Find the minimum variance weights with sd1 = 20%, sd2 = 30%, correlation = 0.1.", "outputs": {'weight_1': 0.7118644067796609, 'weight_2': 0.2881355932203391, 'std_dev': 0.17379122483747123}},
                {"q": "Find the minimum variance weights with sd1 = 20%, sd2 = 20%, correlation = 1.", "outcome": "INVALID_INPUT"}])


def global_min_variance(covariance):
    S = _square("covariance", covariance)
    L = _cholesky(S)
    _need(L is not None, "the covariance matrix must be positive definite (no riskless combination of the assets)")
    x = _chol_solve(L, [1.0] * len(S))
    w = [v / math.fsum(x) for v in x]
    var = _quad(w, S)
    return {"weights": w, "variance": var, "std_dev": math.sqrt(max(var, 0.0))}


def _ref_gmv(covariance):
    import numpy as np                                     # Lagrange (KKT) system of min w'Sw subject to sum w = 1
    S = np.asarray(covariance, dtype=float)
    n = len(S)
    K = np.block([[2 * S, np.ones((n, 1))], [np.ones((1, n)), np.zeros((1, 1))]])
    w = np.linalg.solve(K, np.r_[np.zeros(n), 1.0])[:n]
    v = float(w @ S @ w)
    return {"weights": [float(x) for x in w], "variance": v, "std_dev": float(np.sqrt(v))}


_add("global_min_variance", name="Global minimum-variance portfolio", topic="portfolio theory / mean-variance",
     purpose="Weights (adding up to 1, short sales allowed) of the portfolio with the smallest variance, from a "
             "covariance matrix: w = Sigma^-1 1 / (1' Sigma^-1 1).",
     inputs=[_i("covariance", "covariance matrix of the asset returns", "covariance matrix", "cov", "sigma", type="matrix")],
     outputs=[_o("weights", "minimum-variance weights (list)"), _o("variance", "portfolio variance"), _o("std_dev", "portfolio sd")],
     primary="std_dev", conditions=["covariance symmetric and positive definite"],
     method="Cholesky solve of Sigma x = 1, w = x / sum(x)", latex=r"w^{*} = \frac{\Sigma^{-1}\mathbf{1}}{\mathbf{1}^{\top}\Sigma^{-1}\mathbf{1}}",
     run=global_min_variance, reference=_ref_gmv,
     wrong=lambda covariance: (lambda iv: (lambda w: {"weights": w, "variance": _quad(w, covariance),
                                                      "std_dev": math.sqrt(_quad(w, covariance))})([x / sum(iv) for x in iv]))(
         [1 / covariance[i][i] for i in range(len(covariance))]),
     rand=lambda rng: {"covariance": _rand_cov(rng, rng.randint(2, 6))},
     triggers=["global minimum-variance portfolio", "global minimum variance portfolio", "minimum-variance portfolio",
               "minimum variance portfolio", "gmv portfolio"],
     templates=["Find the global minimum variance portfolio with covariance = [[0.04, 0.006, 0.01], [0.006, 0.09, 0.02], [0.01, 0.02, 0.0625]]."],
     examples=[{"inputs": {"covariance": "[[0.04, 0.006, 0.01], [0.006, 0.09, 0.02], [0.01, 0.02, 0.0625]]"}, "outputs": {'weights': [0.5475060905244262, 0.19040902679830743, 0.2620848826772663], 'variance': 0.025663546608539554, 'std_dev': 0.16019846006918903}},
               {"inputs": {"covariance": [[0.04, 0.0], [0.0, 0.04]]}, "outputs": {"weights": [0.5, 0.5], "variance": 0.02}}],
     questions=[{"q": "Find the global minimum variance portfolio with covariance = [[0.04, 0.006, 0.01], [0.006, 0.09, 0.02], [0.01, 0.02, 0.0625]].",
                 "outputs": {'weights': [0.5475060905244262, 0.19040902679830743, 0.2620848826772663], 'variance': 0.025663546608539554, 'std_dev': 0.16019846006918903}},
                {"q": "Find the global minimum variance portfolio with covariance = [[0.04, 0.04], [0.04, 0.04]].", "outcome": "INVALID_INPUT"}])


def tangency_portfolio(expected_returns, covariance, risk_free):
    mu = _returns("expected_returns", expected_returns, 1)
    S = _square("covariance", covariance, len(mu))
    rf = _rate("risk_free", risk_free)
    L = _cholesky(S)
    _need(L is not None, "the covariance matrix must be positive definite")
    z = _chol_solve(L, [m - rf for m in mu])
    tot = math.fsum(z)
    _need(tot > 1e-12 * max(abs(v) for v in z), "no portfolio of positive total weight has the highest Sharpe ratio "
                                                 "(the excess returns do not favour a long position)")
    w = [v / tot for v in z]
    er, sd = math.fsum(a * b for a, b in zip(w, mu)), math.sqrt(max(_quad(w, S), 0.0))
    return {"weights": w, "expected_return": er, "std_dev": sd, "sharpe": (er - rf) / sd}


def _ref_tangency(expected_returns, covariance, risk_free):
    import numpy as np                                     # min w'Sw subject to w'(mu - rf) = 1, then rescale to sum 1
    S, e = np.asarray(covariance, dtype=float), np.asarray(expected_returns) - risk_free
    n = len(S)
    K = np.block([[2 * S, e.reshape(n, 1)], [e.reshape(1, n), np.zeros((1, 1))]])
    v = np.linalg.solve(K, np.r_[np.zeros(n), 1.0])[:n]
    w = v / v.sum()
    er, sd = float(w @ np.asarray(expected_returns)), float(np.sqrt(w @ S @ w))
    return {"weights": [float(x) for x in w], "expected_return": er, "std_dev": sd, "sharpe": (er - risk_free) / sd}


_add("tangency_portfolio", name="Tangency (maximum Sharpe ratio) portfolio", topic="portfolio theory / mean-variance",
     purpose="Weights of the risky portfolio with the highest Sharpe ratio (short sales allowed): w proportional to "
             "Sigma^-1 (mu - rf).",
     inputs=[_i("expected_returns", "expected returns of the assets (decimals)", "mu", "returns", "expected return", type="vector", min_length=1),
             _i("covariance", "covariance matrix of the returns", "covariance matrix", "cov", "sigma", type="matrix"),
             _i("risk_free", "risk-free rate (decimal)", "rf", "risk-free rate", "risk free rate")],
     outputs=[_o("weights", "tangency weights (list, add up to 1)"), _o("expected_return", "portfolio expected return"),
              _o("std_dev", "portfolio sd"), _o("sharpe", "(E - rf) / sd")], primary="sharpe",
     conditions=["covariance positive definite", "1' Sigma^-1 (mu - rf) > 0"],
     method="z = Sigma^-1 (mu - rf) by Cholesky, w = z / sum(z)",
     latex=r"w^{*} = \frac{\Sigma^{-1}(\mu - r_f\mathbf{1})}{\mathbf{1}^{\top}\Sigma^{-1}(\mu - r_f\mathbf{1})}", run=tangency_portfolio,
     reference=_ref_tangency,
     wrong=lambda expected_returns, covariance, risk_free: (lambda z: (lambda w: (lambda er, sd: {
         "weights": w, "expected_return": er, "std_dev": sd, "sharpe": (er - risk_free) / sd})(
         sum(a * b for a, b in zip(w, expected_returns)), math.sqrt(_quad(w, covariance))))([v / sum(z) for v in z]))(
         [(m - risk_free) / covariance[i][i] for i, m in enumerate(expected_returns)]),
     rand=lambda rng: (lambda n, rf: {"expected_returns": [round(rf + rng.uniform(0.01, 0.15), 4) for _ in range(n)],
                                      "covariance": _rand_cov(rng, n), "risk_free": rf})(rng.randint(2, 5), _u(rng, 0.0, 0.05)),
     triggers=["tangency portfolio", "maximum sharpe ratio portfolio", "max sharpe portfolio", "optimal risky portfolio",
               "maximum sharpe portfolio"],
     templates=["Find the tangency portfolio with expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.006, 0.09]], risk-free rate = 3%."],
     examples=[{"inputs": {"expected_returns": "0.1, 0.15", "covariance": "[[0.04, 0.006], [0.006, 0.09]]", "risk_free": 0.03},
                "outputs": {'weights': [0.5602409638554217, 0.4397590361445783], 'expected_return': 0.1219879518072289, 'std_dev': 0.1814281419927021, 'sharpe': 0.5070214069156321}}],
     questions=[{"q": "Find the tangency portfolio with expected returns = [0.1, 0.15], covariance = [[0.04, 0.006], [0.006, 0.09]], risk-free rate = 3%.",
                 "outputs": {'weights': [0.5602409638554217, 0.4397590361445783], 'expected_return': 0.1219879518072289, 'std_dev': 0.1814281419927021, 'sharpe': 0.5070214069156321}},
                {"q": "Find the tangency portfolio with expected returns = [0.01, 0.02], covariance = [[0.04, 0.006], [0.006, 0.09]], risk-free rate = 5%.",
                 "outcome": "INVALID_INPUT"}])


def mean_returns(returns):
    r = _returns("returns", returns, 1)
    g = math.exp(math.fsum(math.log1p(x) for x in r) / len(r)) - 1
    return {"arithmetic_mean": _mean(r), "geometric_mean": g, "cumulative_return": (1 + g) ** len(r) - 1}


_add("mean_returns", name="Arithmetic and geometric mean return", topic="portfolio theory / returns",
     purpose="Arithmetic mean, geometric (compound) mean and cumulative return of a series of periodic returns.",
     inputs=[_i("returns", "periodic returns (decimals)", "r", "annual returns", "yearly returns", "monthly returns", type="vector", min_length=1)],
     outputs=[_o("arithmetic_mean", "sum R / n"), _o("geometric_mean", "(prod (1 + R))^(1/n) - 1"),
              _o("cumulative_return", "prod (1 + R) - 1")], primary="geometric_mean", conditions=["each return between -100% and 100%"],
     method="geometric mean = exp(mean ln(1 + R)) - 1", latex=r"\bar R_G = \left[\prod_{t=1}^{n}(1 + R_t)\right]^{1/n} - 1",
     run=mean_returns,
     reference=lambda returns: (lambda np, st: (lambda a: {"arithmetic_mean": float(a.mean()), "geometric_mean": float(st.gmean(1 + a)) - 1,
                                                           "cumulative_return": float(np.prod(1 + a)) - 1})(np.asarray(returns)))(
         __import__("numpy"), __import__("scipy.stats", fromlist=["gmean"])),
     wrong=lambda returns: (lambda o: dict(o, geometric_mean=o["arithmetic_mean"] - _cov(returns, returns, 0) / 2))(mean_returns(returns)),
     rand=lambda rng: {"returns": [round(rng.gauss(0.06, 0.15), 4) for _ in range(rng.randint(2, 30))]},
     triggers=["geometric mean return", "arithmetic and geometric mean", "geometric average return", "mean return",
               "average return", "compound average return"],
     templates=["Find the geometric mean return with returns = [10%, -5%, 20%]."],
     examples=[{"inputs": {"returns": "0.1, -0.05, 0.2"}, "outputs": {'arithmetic_mean': 0.08333333333333333, 'geometric_mean': 0.07836515339093597, 'cumulative_return': 0.2540000000000002}},
               {"inputs": {"returns": [0.5, -0.5]}, "outputs": {"arithmetic_mean": 0.0, "cumulative_return": -0.25}}],
     questions=[{"q": "Find the geometric mean return with returns = [10%, -5%, 20%].", "outputs": {'arithmetic_mean': 0.08333333333333333, 'geometric_mean': 0.07836515339093597, 'cumulative_return': 0.2540000000000002}},
                {"q": "Find the geometric mean return with returns = [10%, -150%, 20%].", "outcome": "INVALID_INPUT"}])


def scenario_return(probabilities, returns):
    p, r = _vec("probabilities", probabilities, 1), _vec("returns", returns, 1)
    _need(len(p) == len(r), "one probability per scenario return")
    _need(all(x >= 0 for x in p) and abs(math.fsum(p) - 1) <= 1e-9, "the probabilities must be non-negative and add up to 1")
    _need(all(x > -1 for x in r), "a return cannot be below -100%")
    e = math.fsum(a * b for a, b in zip(p, r))
    var = math.fsum(a * (b - e) ** 2 for a, b in zip(p, r))
    return {"expected_return": e, "variance": var, "std_dev": math.sqrt(var)}


def _rand_probs(rng, n):
    u = [rng.randint(1, 20) for _ in range(n)]
    p = [round(x / sum(u), 6) for x in u]
    p[-1] = round(1 - sum(p[:-1]), 12)
    return p


_add("scenario_return", name="Expected return and risk from scenarios", topic="portfolio theory / returns",
     purpose="Expected return, variance and standard deviation of an asset whose return depends on scenarios with "
             "given probabilities.",
     inputs=[_i("probabilities", "probability of each scenario (adding up to 1)", "probs", "p", "probability", type="vector", min_length=1),
             _i("returns", "return in each scenario (decimals)", "r", "scenario returns", "outcomes", type="vector", min_length=1)],
     outputs=[_o("expected_return", "sum p R"), _o("variance", "sum p (R - E)^2"), _o("std_dev", "sqrt(variance)")],
     primary="expected_return", conditions=["one probability per return", "probabilities >= 0 adding up to 1"],
     method="E = sum p_i R_i; Var = sum p_i (R_i - E)^2", latex=r"E[R] = \sum_i p_i R_i,\quad \sigma^{2} = \sum_i p_i (R_i - E[R])^{2}",
     run=scenario_return,
     reference=lambda probabilities, returns: (lambda np: (lambda e: {"expected_return": e, "variance": float(np.average(
         (np.asarray(returns) - e) ** 2, weights=probabilities))})(float(np.average(returns, weights=probabilities))))(__import__("numpy")),
     wrong=lambda probabilities, returns: (lambda o: dict(o, variance=_cov(returns, returns, 0)))(scenario_return(probabilities, returns)),
     rand=lambda rng: (lambda n: {"probabilities": _rand_probs(rng, n), "returns": [_u(rng, -0.5, 0.8) for _ in range(n)]})(rng.randint(2, 7)),
     triggers=["expected return from scenarios", "scenario analysis", "expected return of the scenarios", "probability-weighted return",
               "expected return and standard deviation of the scenarios"],
     templates=["Find the expected return from scenarios with probabilities = [0.3, 0.5, 0.2], returns = [0.2, 0.1, -0.15]."],
     examples=[{"inputs": {"probabilities": "0.3, 0.5, 0.2", "returns": "0.2, 0.1, -0.15"}, "outputs": {'expected_return': 0.08, 'variance': 0.0151, 'std_dev': 0.12288205727444508}}],
     questions=[{"q": "Find the expected return from scenarios with probabilities = [0.3, 0.5, 0.2], returns = [0.2, 0.1, -0.15].", "outputs": {'expected_return': 0.08, 'variance': 0.0151, 'std_dev': 0.12288205727444508}},
                {"q": "Find the expected return from scenarios with probabilities = [0.3, 0.5, 0.3], returns = [0.2, 0.1, -0.15].",
                 "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# RISK MEASURES
# =====================================================================================================================
def _confidence(c):
    c = _fin("confidence", c)
    _need(0.5 < c < 1, "confidence is a decimal between 0.5 and 1 (95% = 0.95)")
    return c


def historical_var(returns, confidence, portfolio_value=1.0):
    r = _returns("returns", returns, 2)
    a = _confidence(confidence)
    V = _pos("portfolio_value", portfolio_value)
    k = math.ceil(len(r) * (1 - Fraction(repr(a))))       # the stated decimal, not its binary rounding
    s = sorted(r)
    return {"var": -s[k - 1] * V, "cvar": -math.fsum(s[:k]) / k * V, "tail_observations": k}


def _ref_hvar(returns, confidence, portfolio_value=1.0):
    import numpy as np
    a = np.asarray(returns)
    q = float(np.quantile(a, 1 - confidence, method="inverted_cdf"))
    return {"var": -q * portfolio_value, "cvar": -float(a[a <= q].mean()) * portfolio_value}


def _rand_hvar(rng):
    n = rng.choice([23, 47, 61, 97, 101, 103, 199, 251, 257, 499])
    return {"returns": [round(rng.gauss(0.0005, 0.015), 6) for _ in range(n)], "confidence": rng.choice([0.9, 0.95, 0.975, 0.99]),
            "portfolio_value": rng.choice([1.0, _u(rng, 1e4, 1e7, 0)])}


_add("historical_var", name="Historical value at risk and expected shortfall", topic="risk measures / value at risk",
     purpose="VaR and CVaR (expected shortfall) by historical simulation: with k = ceil(n (1 - confidence)), VaR is "
             "minus the k-th smallest return and CVaR minus the mean of the k smallest, times the portfolio value.",
     inputs=[_i("returns", "historical returns (decimals)", "r", "historical returns", "daily returns", type="vector", min_length=2),
             _i("confidence", "confidence level (0.95 = 95%)", "confidence level", "alpha", "level"),
             _i("portfolio_value", "portfolio value (default 1: VaR as a fraction)", "value", "position", "portfolio", required=False)],
     outputs=[_o("var", "value at risk (a positive number is a loss)"), _o("cvar", "expected shortfall (mean loss beyond VaR)"),
              _o("tail_observations", "k, the number of returns in the tail")], primary="var",
     conditions=["at least 2 returns", "0.5 < confidence < 1", "portfolio_value > 0"],
     method="k = ceil(n (1 - c)); VaR = -R_(k); CVaR = -(1/k) sum_{i<=k} R_(i)",
     latex=r"\mathrm{VaR}_c = -R_{(k)},\quad \mathrm{CVaR}_c = -\frac{1}{k}\sum_{i=1}^{k} R_{(i)},\quad k = \lceil n(1-c)\rceil",
     run=historical_var, reference=_ref_hvar,
     wrong=lambda returns, confidence, portfolio_value=1.0: (lambda o: dict(o, var=-sorted(returns)[min(o["tail_observations"], len(returns) - 1)]
                                                                         * portfolio_value))(historical_var(returns, confidence, portfolio_value)),
     rand=_rand_hvar,
     triggers=["historical var", "historical value at risk", "historical simulation var", "historical expected shortfall",
               "historical cvar"],
     templates=["Find the historical VaR with returns = [-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015], confidence = 90%."],
     examples=[{"inputs": {"returns": "-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015", "confidence": 0.9},
                "outputs": {'var': 0.02, 'cvar': 0.027500000000000004, 'tail_observations': 2}},
               {"inputs": {"returns": [0.01, -0.02, 0.03, -0.04, 0.05, -0.01, 0.02, 0.0, 0.01, -0.03], "confidence": 0.9, "portfolio_value": 1000},
                "outputs": {"var": 40.0, "cvar": 40.0}}],
     questions=[{"q": "Find the historical VaR with returns = [-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015], confidence = 90%.",
                 "outputs": {'var': 0.02, 'cvar': 0.027500000000000004, 'tail_observations': 2}},
                {"q": "Find the historical VaR with returns = [-0.02, 0.01, -0.035], confidence = 40%.", "outcome": "INVALID_INPUT"}])


def parametric_var(mean, std_dev, confidence, horizon=1.0, portfolio_value=1.0):
    mu, s = _rate("mean", mean), _pos("std_dev", std_dev)
    _need(s < 5, "std_dev is a decimal (20% = 0.2)")
    a, h, V = _confidence(confidence), _pos("horizon", horizon), _pos("portfolio_value", portfolio_value)
    z = statistics.NormalDist().inv_cdf(a)
    sh, mh = s * math.sqrt(h), mu * h
    phi = math.exp(-z * z / 2) / math.sqrt(2 * math.pi)
    return {"var": V * (z * sh - mh), "cvar": V * (sh * phi / (1 - a) - mh), "z": z}


def _ref_pvar(mean, std_dev, confidence, horizon=1.0, portfolio_value=1.0):
    import numpy as np
    from scipy import stats
    from scipy.integrate import quad                       # expected shortfall by integrating the lower tail
    m, s = mean * horizon, std_dev * np.sqrt(horizon)
    q = stats.norm.ppf(1 - confidence, loc=m, scale=s)
    tail = quad(lambda x: x * stats.norm.pdf(x, m, s), m - 40 * s, q, epsabs=1e-15, epsrel=1e-13, limit=200)[0] / (1 - confidence)
    return {"var": -q * portfolio_value, "cvar": -tail * portfolio_value}


_add("parametric_var", name="Parametric (normal) value at risk and expected shortfall", topic="risk measures / value at risk",
     purpose="VaR and CVaR of normally distributed returns with a mean and standard deviation per period, scaled to a "
             "horizon by mean x h and sd x sqrt(h).",
     inputs=[_i("mean", "mean return per period (decimal)", "mu", "expected return", "mean return"),
             _i("std_dev", "standard deviation of returns per period (decimal)", "sigma", "sd", "volatility", "standard deviation"),
             _i("confidence", "confidence level (0.95 = 95%)", "confidence level", "alpha", "level"),
             _i("horizon", "horizon in periods (default 1)", "h", "holding period", "days", required=False),
             _i("portfolio_value", "portfolio value (default 1)", "value", "position", "portfolio", required=False)],
     outputs=[_o("var", "value at risk V (z sd sqrt(h) - mean h)"), _o("cvar", "expected shortfall V (sd sqrt(h) phi(z)/(1 - c) - mean h)"),
              _o("z", "standard normal quantile at the confidence level")], primary="var",
     conditions=["0 < std_dev < 5", "0.5 < confidence < 1", "horizon > 0", "returns normally distributed"],
     method="VaR = V (z_c sigma sqrt(h) - mu h); ES = V (sigma sqrt(h) phi(z_c) / (1 - c) - mu h)",
     latex=r"\mathrm{VaR} = V(z_c\,\sigma\sqrt{h} - \mu h),\quad \mathrm{ES} = V\left(\frac{\sigma\sqrt{h}\,\varphi(z_c)}{1 - c} - \mu h\right)",
     run=parametric_var, reference=_ref_pvar,
     wrong=lambda mean, std_dev, confidence, horizon=1.0, portfolio_value=1.0: (lambda o: dict(
         o, var=portfolio_value * (o["z"] * std_dev * math.sqrt(horizon) + mean * horizon)))(parametric_var(mean, std_dev, confidence, horizon, portfolio_value)),
     rand=lambda rng: {"mean": rng.choice([-1, 1]) * _u(rng, 0.0002, 0.01, 5), "std_dev": _u(rng, 0.005, 0.05, 5),
                       "confidence": rng.choice([0.9, 0.95, 0.975, 0.99]), "horizon": float(rng.choice([1, 1, 5, 10, 20])),
                       "portfolio_value": rng.choice([1.0, _u(rng, 1e4, 1e7, 0)])},
     triggers=["parametric var", "parametric value at risk", "normal var", "variance-covariance var", "delta-normal var",
               "parametric expected shortfall"],
     templates=["Find the parametric VaR with mean = 0.05%, std dev = 1.2%, confidence = 99%, horizon = 10, portfolio value = 1000000."],
     examples=[{"inputs": {"mean": 0.0005, "std_dev": 0.012, "confidence": 0.99, "horizon": 10, "portfolio_value": 1000000}, "outputs": {'var': 83278.69494231466, 'cvar': 96137.76866275152, 'z': 2.3263478740408408}},
               {"inputs": {"mean": 0.0, "std_dev": 0.02, "confidence": 0.95}, "outputs": {'var': 0.03289707253902943, 'cvar': 0.04125425615014858, 'z': 1.6448536269514715}}],
     questions=[{"q": "Find the parametric VaR with mean = 0.05%, std dev = 1.2%, confidence = 99%, horizon = 10, portfolio value = 1000000.",
                 "outputs": {'var': 83278.69494231466, 'cvar': 96137.76866275152, 'z': 2.3263478740408408}},
                {"q": "Find the parametric VaR with mean = 0, sigma = 2%, confidence = 95%.", "outputs": {'var': 0.03289707253902943, 'cvar': 0.04125425615014858, 'z': 1.6448536269514715}},
                {"q": "Find the parametric VaR with mean = 0, sigma = 20, confidence = 95%.", "outcome": "INVALID_INPUT"}])


def max_drawdown(prices):
    p = _vec("prices", prices, 2)
    _need(all(x > 0 for x in p), "prices must be positive")
    peak, mdd = p[0], 0.0
    for x in p:
        peak = max(peak, x)
        mdd = max(mdd, 1 - x / peak)
    return {"max_drawdown": mdd}


_add("max_drawdown", name="Maximum drawdown", topic="risk measures / drawdown",
     purpose="The largest fall from a running peak to a later trough, as a fraction of the peak.",
     inputs=[_i("prices", "price or portfolio value series in time order", "values", "price series", "portfolio values", "nav",
                type="vector", min_length=2)],
     outputs=[_o("max_drawdown", "max over t of 1 - P_t / max_{s<=t} P_s")], primary="max_drawdown", conditions=["prices > 0", "at least 2"],
     method="running peak; drawdown 1 - P/peak; maximum over time", latex=r"\mathrm{MDD} = \max_{t}\left(1 - \frac{P_t}{\max_{s\le t} P_s}\right)",
     run=max_drawdown,
     reference=lambda prices: (lambda np: (lambda a: {"max_drawdown": float(np.max(1 - a / np.maximum.accumulate(a)))})(np.asarray(prices)))(
         __import__("numpy")),
     wrong=lambda prices: {"max_drawdown": max(max(0.0, 1 - b / a) for a, b in zip(prices, prices[1:]))},
     rand=lambda rng: {"prices": (lambda n: [round(100 * math.exp(sum(rng.gauss(0.0, 0.03) for _ in range(k + 1))), 4) for k in range(n)])(
         rng.randint(15, 80))},
     triggers=["maximum drawdown", "max drawdown", "drawdown"],
     templates=["Find the maximum drawdown with prices = [100, 120, 90, 110, 80, 130]."],
     examples=[{"inputs": {"prices": "100, 120, 90, 110, 80, 130"}, "outputs": {"max_drawdown": 1 / 3}},
               {"inputs": {"prices": [100, 101, 102]}, "outputs": {"max_drawdown": 0.0}}],
     questions=[{"q": "Find the maximum drawdown with prices = [100, 120, 90, 110, 80, 130].", "outputs": {"max_drawdown": 1 / 3}},
                {"q": "Find the maximum drawdown with prices = [100, -5, 90].", "outcome": "INVALID_INPUT"}])


def portfolio_var(weights, covariance, confidence, portfolio_value=1.0, horizon=1.0):
    w = _vec("weights", weights, 1)
    S = _square("covariance", covariance, len(w))
    a, V, h = _confidence(confidence), _pos("portfolio_value", portfolio_value), _pos("horizon", horizon)
    var = _quad(w, S)
    _need(var >= -1e-15, "the covariance matrix is not positive semidefinite")
    z = statistics.NormalDist().inv_cdf(a)
    sd = math.sqrt(max(var, 0.0))
    undiv = z * math.sqrt(h) * V * math.fsum(abs(x) * math.sqrt(S[i][i]) for i, x in enumerate(w))
    pv = z * sd * math.sqrt(h) * V
    return {"var": pv, "portfolio_sd": sd, "undiversified_var": undiv, "diversification_benefit": undiv - pv}


def _ref_portfolio_var(weights, covariance, confidence, portfolio_value=1.0, horizon=1.0):
    import numpy as np
    from scipy import stats
    w, S = np.asarray(weights), np.asarray(covariance)
    z = stats.norm.ppf(confidence)
    sd = float(np.sqrt(w @ S @ w))
    return {"var": z * sd * np.sqrt(horizon) * portfolio_value, "portfolio_sd": sd,
            "undiversified_var": float(z * np.sqrt(horizon) * portfolio_value * np.sum(np.abs(w) * np.sqrt(np.diag(S))))}


_add("portfolio_var", name="Portfolio value at risk (variance-covariance)", topic="risk measures / value at risk",
     purpose="Zero-mean normal VaR of a portfolio from its weights and the covariance matrix of periodic returns, with "
             "the undiversified VaR (sum of the assets' own VaRs) and the diversification benefit.",
     inputs=[_i("weights", "portfolio weights (fractions of the portfolio value)", "w", "portfolio weights", type="vector", min_length=1),
             _i("covariance", "covariance matrix of periodic returns", "covariance matrix", "cov", "sigma", type="matrix"),
             _i("confidence", "confidence level (0.99 = 99%)", "confidence level", "alpha", "level"),
             _i("portfolio_value", "portfolio value (default 1)", "value", "position", "portfolio", required=False),
             _i("horizon", "horizon in periods (default 1)", "h", "holding period", "days", required=False)],
     outputs=[_o("var", "z sqrt(w'Sigma w) sqrt(h) V"), _o("portfolio_sd", "sqrt(w'Sigma w) per period"),
              _o("undiversified_var", "z sqrt(h) V sum |w_i| sigma_i"), _o("diversification_benefit", "undiversified - portfolio VaR")],
     primary="var", conditions=["one weight per row of the covariance", "covariance symmetric positive semidefinite", "0.5 < confidence < 1"],
     method="VaR = z_c sqrt(w' Sigma w) sqrt(h) V (mean return taken as 0)", latex=r"\mathrm{VaR} = z_c\sqrt{w^{\top}\Sigma w}\,\sqrt{h}\,V",
     run=portfolio_var, reference=_ref_portfolio_var,
     wrong=lambda weights, covariance, confidence, portfolio_value=1.0, horizon=1.0: (lambda o: dict(o, var=o["undiversified_var"]))(
         portfolio_var(weights, covariance, confidence, portfolio_value, horizon)),
     rand=lambda rng: (lambda n: {"weights": _rand_weights(rng, n), "covariance": [[x / 250 for x in r] for r in _rand_cov(rng, n)],
                                  "confidence": rng.choice([0.95, 0.99]), "portfolio_value": _u(rng, 1e4, 1e7, 0),
                                  "horizon": float(rng.choice([1, 10]))})(rng.randint(2, 5)),
     triggers=["portfolio var", "portfolio value at risk", "var of the portfolio", "value at risk of the portfolio"],
     templates=["Find the portfolio VaR with weights = [0.6, 0.4], covariance = [[0.0004, 0.00006], [0.00006, 0.0009]], confidence = 99%, portfolio value = 1000000."],
     examples=[{"inputs": {"weights": "0.6, 0.4", "covariance": "[[0.0004, 0.00006], [0.00006, 0.0009]]", "confidence": 0.99,
                           "portfolio_value": 1000000}, "outputs": {'var': 41406.378201407744, 'portfolio_sd': 0.01779887636902959, 'undiversified_var': 55832.348976980174, 'diversification_benefit': 14425.97077557243}}],
     questions=[{"q": "Find the portfolio VaR with weights = [0.6, 0.4], covariance = [[0.0004, 0.00006], [0.00006, 0.0009]], confidence = 99%, portfolio value = 1000000.",
                 "outputs": {'var': 41406.378201407744, 'portfolio_sd': 0.01779887636902959, 'undiversified_var': 55832.348976980174, 'diversification_benefit': 14425.97077557243}}])


# =====================================================================================================================
# OPTIONS AND FORWARDS (European; continuous dividend yield; time in years; annual continuously compounded rates)
# =====================================================================================================================
def _N(x):
    return 0.5 * math.erfc(-x / math.sqrt(2))


def _phi(x):
    return math.exp(-x * x / 2) / math.sqrt(2 * math.pi)


def _opt(spot, strike, time, rate, volatility, dividend_yield):
    S, K, T = _pos("spot", spot), _pos("strike", strike), _pos("time", time)
    r, q = _rate("rate", rate), _rate("dividend_yield", dividend_yield)
    s = _pos("volatility", volatility)
    _need(s < 5, "volatility is a decimal (20% = 0.2)")
    return S, K, T, r, s, q


def _bs(S, K, T, r, s, q):
    sq = s * math.sqrt(T)
    d1 = (math.log(S / K) + (r - q + s * s / 2) * T) / sq
    d2 = d1 - sq
    dq, dr = math.exp(-q * T), math.exp(-r * T)
    return d1, d2, S * dq * _N(d1) - K * dr * _N(d2), K * dr * _N(-d2) - S * dq * _N(-d1)


def black_scholes(spot, strike, time, rate, volatility, dividend_yield=0.0):
    d1, d2, c, p = _bs(*_opt(spot, strike, time, rate, volatility, dividend_yield))
    return {"call": c, "put": p, "d1": d1, "d2": d2}


def _ql_black(kind, S, K, T, r, sd, q):
    import QuantLib as ql
    return ql.blackFormula(ql.Option.Call if kind == "call" else ql.Option.Put, K, S * math.exp((r - q) * T), sd, math.exp(-r * T))


_OPT_IN = [_i("spot", "price of the underlying now", "s", "s0", "stock price", "spot price", "underlying price"),
           _i("strike", "strike (exercise) price", "k", "strike price", "exercise price", "x"),
           _i("time", "time to expiry in years", "t", "maturity", "time to maturity", "time to expiry", "years", "expiry"),
           _i("rate", "risk-free rate, continuously compounded (decimal)", "r", "risk-free rate", "risk free rate", "interest rate"),
           _i("volatility", "volatility of the underlying, annual (decimal)", "sigma", "vol", "annual volatility"),
           _i("dividend_yield", "continuous dividend yield (decimal, default 0)", "q", "dividend yield", "yield", required=False)]


def _opt_inputs(skip=(), extra=()):
    return [dict(i, _aliases=list(i["_aliases"])) for i in _OPT_IN if i["symbol"] not in skip] + list(extra)


def _rand_opt(rng):
    S = _u(rng, 20, 200, 2)
    return {"spot": S, "strike": round(S * rng.uniform(0.75, 1.3), 2), "time": _u(rng, 0.1, 3, 3), "rate": _u(rng, 0.0, 0.08),
            "volatility": _u(rng, 0.1, 0.6), "dividend_yield": rng.choice([0.0, _u(rng, 0.005, 0.05)])}


_add("black_scholes", name="Black-Scholes option prices", topic="stocks and options / option pricing",
     purpose="Prices of European call and put options on an asset with a continuous dividend yield (Black-Scholes-Merton).",
     inputs=_opt_inputs(),
     outputs=[_o("call", "European call price"), _o("put", "European put price"), _o("d1", "d1"), _o("d2", "d2 = d1 - sigma sqrt(T)")],
     primary="call", conditions=["spot, strike, time > 0", "0 < volatility < 5", "European exercise", "lognormal prices"],
     method="C = S e^(-qT) N(d1) - K e^(-rT) N(d2); P = K e^(-rT) N(-d2) - S e^(-qT) N(-d1)",
     latex=r"C = S e^{-qT}N(d_1) - K e^{-rT}N(d_2),\quad d_{1} = \frac{\ln(S/K) + (r - q + \sigma^{2}/2)T}{\sigma\sqrt{T}}",
     run=black_scholes,
     reference=lambda spot, strike, time, rate, volatility, dividend_yield=0.0: {
         k: _ql_black(k, spot, strike, time, rate, volatility * math.sqrt(time), dividend_yield) for k in ("call", "put")},
     wrong=lambda spot, strike, time, rate, volatility, dividend_yield=0.0: (lambda d1: (lambda d2: {
         "call": spot * math.exp(-dividend_yield * time) * _N(d1) - strike * math.exp(-rate * time) * _N(d2),
         "put": strike * math.exp(-rate * time) * _N(-d2) - spot * math.exp(-dividend_yield * time) * _N(-d1), "d1": d1, "d2": d2})(
         d1 - volatility * math.sqrt(time)))((math.log(spot / strike) + (rate - dividend_yield) * time) / (volatility * math.sqrt(time))),
     rand=_rand_opt,
     triggers=["black-scholes", "black scholes", "black-scholes-merton", "european call", "european put", "european option"],
     templates=["Price the option with Black-Scholes with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%."],
     examples=[{"inputs": {"spot": 100, "strike": 100, "time": 1, "rate": 0.05, "volatility": 0.2}, "outputs": {"call": 10.450583572185565}},
               {"inputs": {"spot": 50, "strike": 55, "time": 0.5, "rate": 0.03, "volatility": 0.3, "dividend_yield": 0.02}, "outputs": {'call': 2.4289056001374334, 'put': 7.10757059084748, 'd1': -0.3196602531541147, 'd2': -0.531792287510079}}],
     questions=[{"q": "Price the option with Black-Scholes with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%.",
                 "outputs": {"call": 10.450583572185565}},
                {"q": "Find the Black-Scholes price with s = 50, k = 55, t = 0.5, r = 3%, sigma = 30%, q = 2%.", "outputs": {'call': 2.4289056001374334, 'put': 7.10757059084748, 'd1': -0.3196602531541147, 'd2': -0.531792287510079}},
                {"q": "Find the Black-Scholes price with s = 50, k = 55, t = 0, r = 3%, sigma = 30%.", "outcome": "INVALID_INPUT"},
                {"q": "Find the Black-Scholes price with s = 50, k = 55, t = 0.5, r = 3%.", "outcome": "NEEDS_INPUT"}])


def bs_greeks(spot, strike, time, rate, volatility, dividend_yield=0.0):
    S, K, T, r, s, q = _opt(spot, strike, time, rate, volatility, dividend_yield)
    d1, d2, _c, _p = _bs(S, K, T, r, s, q)
    dq, dr, sq = math.exp(-q * T), math.exp(-r * T), math.sqrt(T)
    common = -S * dq * _phi(d1) * s / (2 * sq)
    return {"call_delta": dq * _N(d1), "put_delta": -dq * _N(-d1), "gamma": dq * _phi(d1) / (S * s * sq),
            "vega": S * dq * _phi(d1) * sq,
            "call_theta": common - r * K * dr * _N(d2) + q * S * dq * _N(d1),
            "put_theta": common + r * K * dr * _N(-d2) - q * S * dq * _N(-d1),
            "call_rho": K * T * dr * _N(d2), "put_rho": -K * T * dr * _N(-d2)}


def _ref_greeks(spot, strike, time, rate, volatility, dividend_yield=0.0):
    import mpmath                                          # derivatives of the price itself, numerically, at 40 digits
    with mpmath.workdps(40):
        def price(kind, S, T, r, s):
            d1 = (mpmath.log(S / strike) + (r - dividend_yield + s * s / 2) * T) / (s * mpmath.sqrt(T))
            d2 = d1 - s * mpmath.sqrt(T)
            if kind == "call":
                return S * mpmath.exp(-dividend_yield * T) * mpmath.ncdf(d1) - strike * mpmath.exp(-r * T) * mpmath.ncdf(d2)
            return strike * mpmath.exp(-r * T) * mpmath.ncdf(-d2) - S * mpmath.exp(-dividend_yield * T) * mpmath.ncdf(-d1)
        S, T, r, s = (mpmath.mpf(x) for x in (spot, time, rate, volatility))
        out = {}
        for k in ("call", "put"):
            out[k + "_delta"] = float(mpmath.diff(lambda x: price(k, x, T, r, s), S))
            out[k + "_theta"] = float(-mpmath.diff(lambda x: price(k, S, x, r, s), T))
            out[k + "_rho"] = float(mpmath.diff(lambda x: price(k, S, T, x, s), r))
        out["gamma"] = float(mpmath.diff(lambda x: price("call", x, T, r, s), S, 2))
        out["vega"] = float(mpmath.diff(lambda x: price("call", S, T, r, x), s))
        return out


_add("bs_greeks", name="Black-Scholes Greeks", topic="stocks and options / option risk",
     purpose="Delta, gamma, vega, theta and rho of European calls and puts (Black-Scholes-Merton, continuous dividend "
             "yield). Vega per 1.00 of volatility, theta per year, rho per 1.00 of rate.",
     inputs=_opt_inputs(),
     outputs=[_o("call_delta", "dC/dS"), _o("put_delta", "dP/dS"), _o("gamma", "d2V/dS2 (call = put)"), _o("vega", "dV/dsigma (call = put)"),
              _o("call_theta", "-dC/dT per year"), _o("put_theta", "-dP/dT per year"), _o("call_rho", "dC/dr"), _o("put_rho", "dP/dr")],
     primary="call_delta", conditions=["spot, strike, time > 0", "0 < volatility < 5"],
     method="closed-form Greeks of the Black-Scholes-Merton formula",
     latex=r"\Delta_C = e^{-qT}N(d_1),\ \Gamma = \frac{e^{-qT}\varphi(d_1)}{S\sigma\sqrt{T}},\ \nu = S e^{-qT}\varphi(d_1)\sqrt{T}",
     run=bs_greeks, reference=_ref_greeks,
     wrong=lambda spot, strike, time, rate, volatility, dividend_yield=0.0: (lambda o: dict(
         o, call_delta=_N(_bs(spot, strike, time, rate, volatility, dividend_yield)[1])))(bs_greeks(spot, strike, time, rate, volatility, dividend_yield)),
     rand=_rand_opt,
     triggers=["greeks", "option greeks", "delta and gamma", "black-scholes greeks", "delta of the option", "vega of the option"],
     templates=["Find the option greeks with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%."],
     examples=[{"inputs": {"spot": 100, "strike": 100, "time": 1, "rate": 0.05, "volatility": 0.2}, "outputs": {'call_delta': 0.6368306511756191, 'put_delta': -0.3631693488243809, 'gamma': 0.018762017345846895, 'vega': 37.52403469169379, 'call_theta': -6.414027546438197, 'put_theta': -1.6578804239346265, 'call_rho': 53.232481545376345, 'put_rho': -41.89046090469506}},
               {"inputs": {"spot": 50, "strike": 55, "time": 0.5, "rate": 0.03, "volatility": 0.3, "dividend_yield": 0.02}, "outputs": {'call_delta': 0.37088548560094126, 'put_delta': -0.6191643481482269, 'gamma': 0.035383612747802204, 'vega': 13.268854780425828, 'call_theta': -4.093232008924096, 'put_theta': -3.45784714232821, 'call_rho': 8.057684339954815, 'put_rho': -19.03289399912941}}],
     questions=[{"q": "Find the option greeks with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%.", "outputs": {'call_delta': 0.6368306511756191, 'put_delta': -0.3631693488243809, 'gamma': 0.018762017345846895, 'vega': 37.52403469169379, 'call_theta': -6.414027546438197, 'put_theta': -1.6578804239346265, 'call_rho': 53.232481545376345, 'put_rho': -41.89046090469506}}])


def implied_volatility(option_price, spot, strike, time, rate, dividend_yield=0.0, option_type="call"):
    S, K, T, r, _s, q = _opt(spot, strike, time, rate, 0.2, dividend_yield)
    p = _pos("option_price", option_price)
    _need(option_type in ("call", "put"), "option_type is call or put")
    f = lambda s: _bs(S, K, T, r, s, q)[2 if option_type == "call" else 3]
    lo, hi = 1e-4, 5.0
    _need(f(lo) < p < f(hi), "the price is outside the range Black-Scholes can produce (volatility 0.01% to 500%): "
                             "it violates the no-arbitrage bounds or implies an extreme volatility")
    for _ in range(200):
        mid = (lo + hi) / 2
        if mid in (lo, hi):
            break
        if f(mid) < p:
            lo = mid
        else:
            hi = mid
    sig = (lo + hi) / 2
    vega = S * math.exp(-q * T) * _phi(_bs(S, K, T, r, sig, q)[0]) * math.sqrt(T)
    _need(vega > 1e-4 * S, "the option is too far from the money: its price barely depends on volatility, so the implied "
                           "volatility is not well determined")
    return {"volatility": sig, "vega": vega}


def _verify_iv(inputs, outputs):
    s = outputs.get("volatility")
    if not isinstance(s, (int, float)) or not math.isfinite(s) or s <= 0:
        return math.inf
    S, K, T, r = inputs["spot"], inputs["strike"], inputs["time"], inputs["rate"]
    q = inputs.get("dividend_yield", 0.0)
    v = _bs(S, K, T, r, s, q)[2 if inputs.get("option_type", "call") == "call" else 3]
    return abs(v - inputs["option_price"]) / max(1.0, inputs["option_price"])


def _ref_iv(option_price, spot, strike, time, rate, dividend_yield=0.0, option_type="call"):
    from scipy.optimize import brentq
    sd = brentq(lambda x: _ql_black(option_type, spot, strike, time, rate, x, dividend_yield) - option_price,
                1e-4 * math.sqrt(time), 5 * math.sqrt(time), xtol=1e-16, rtol=1e-15, maxiter=500)
    return {"volatility": sd / math.sqrt(time)}


def _rand_iv(rng):
    x = _rand_opt(rng)
    x["strike"] = round(x["spot"] * rng.uniform(0.85, 1.15), 2)
    x["option_type"] = rng.choice(["call", "put"])
    S, K, T, r, s, q = x["spot"], x["strike"], x["time"], x["rate"], x.pop("volatility"), x["dividend_yield"]
    x["option_price"] = round(_bs(S, K, T, r, s, q)[2 if x["option_type"] == "call" else 3], 6)
    return x


_add("implied_volatility", name="Implied volatility", topic="stocks and options / option pricing",
     purpose="The volatility at which the Black-Scholes price of a European call or put equals its market price.",
     inputs=[_i("option_price", "market price of the option", "price", "option premium", "premium", "market price", "c")]
     + _opt_inputs(skip=("volatility",), extra=[_i("option_type", "call (default) or put", "type", "option", type="choice",
                                                    choices=["call", "put"], required=False, syn={"calls": "call", "puts": "put"})]),
     outputs=[_o("volatility", "implied volatility, annual"), _o("vega", "vega at that volatility")], primary="volatility",
     conditions=["price within the no-arbitrage bounds", "implied volatility between 0.01% and 500%", "not too far from the money"],
     method="bisection on sigma (the price rises with sigma, so the implied volatility is unique)",
     latex=r"C_{BS}(\sigma_{imp}) = C_{market}", run=implied_volatility, reference=_ref_iv,
     wrong=lambda option_price, spot, strike, time, rate, dividend_yield=0.0, option_type="call": {
         "volatility": option_price / spot * math.sqrt(2 * math.pi / time), "vega": 0.0},
     rand=_rand_iv,
     triggers=["implied volatility", "implied vol"],
     templates=["Find the implied volatility with option price = 10.45, spot = 100, strike = 100, time = 1, rate = 5%."],
     examples=[{"inputs": {"option_price": 10.450583572185565, "spot": 100, "strike": 100, "time": 1, "rate": 0.05}, "outputs": {"volatility": 0.2}},
               {"inputs": {"option_price": 6.5, "spot": 50, "strike": 55, "time": 0.5, "rate": 0.03, "option_type": "put"}, "outputs": {'volatility': 0.2777406200919259, 'vega': 13.439981088667267}}],
     questions=[{"q": "Find the implied volatility with option price = 10.45, spot = 100, strike = 100, time = 1, rate = 5%.", "outputs": {'volatility': 0.19998444801094378, 'vega': 37.52388148158723}},
                {"q": "Find the implied volatility with option price = 6.5, s = 50, k = 55, t = 0.5, r = 3%, type = put.", "outputs": {'volatility': 0.2777406200919259, 'vega': 13.439981088667267}},
                {"q": "Find the implied volatility with option price = 120, spot = 100, strike = 100, time = 1, rate = 5%.", "outcome": "INVALID_INPUT"}])
PROCEDURES["implied_volatility"]["verify"] = _verify_iv


def put_call_parity(call_price, spot, strike, time, rate, dividend_yield=0.0):
    S, K, T, r, _s, q = _opt(spot, strike, time, rate, 0.2, dividend_yield)
    C = _pos("call_price", call_price)
    Sq, Kr = S * math.exp(-q * T), K * math.exp(-r * T)
    _need(max(0.0, Sq - Kr) < C < Sq, "the call price violates the no-arbitrage bounds max(0, S e^-qT - K e^-rT) < C < S e^-qT")
    return {"put_price": C - Sq + Kr, "pv_strike": Kr, "pv_spot": Sq}


def _ref_parity(call_price, spot, strike, time, rate, dividend_yield=0.0):
    from scipy.optimize import brentq                     # any volatility reproducing the call gives the put
    sd = brentq(lambda x: _ql_black("call", spot, strike, time, rate, x, dividend_yield) - call_price, 1e-6, 20,
                xtol=1e-16, rtol=1e-15, maxiter=1000)
    return {"put_price": _ql_black("put", spot, strike, time, rate, sd, dividend_yield)}


_add("put_call_parity", name="Put-call parity", topic="stocks and options / arbitrage",
     purpose="The price of a European put from the call with the same strike and expiry: P = C - S e^(-qT) + K e^(-rT).",
     inputs=[_i("call_price", "price of the European call", "call", "c", "call premium")] + _opt_inputs(skip=("volatility",)),
     outputs=[_o("put_price", "European put price"), _o("pv_strike", "K e^(-rT)"), _o("pv_spot", "S e^(-qT)")], primary="put_price",
     conditions=["European options, same strike and expiry", "call within the no-arbitrage bounds"],
     method="P = C - S e^(-qT) + K e^(-rT)", latex=r"C + K e^{-rT} = P + S e^{-qT}", run=put_call_parity, reference=_ref_parity,
     wrong=lambda call_price, spot, strike, time, rate, dividend_yield=0.0: (lambda o: dict(o, put_price=call_price - spot + strike))(
         put_call_parity(call_price, spot, strike, time, rate, dividend_yield)),
     rand=lambda rng: (lambda x: dict({k: v for k, v in x.items() if k != "volatility"}, call_price=round(_bs(
         x["spot"], x["strike"], x["time"], x["rate"], x["volatility"], x["dividend_yield"])[2], 6)))(_rand_opt(rng)),
     triggers=["put-call parity", "put call parity"],
     templates=["Use put-call parity with call price = 10.45, spot = 100, strike = 100, time = 1, rate = 5%."],
     examples=[{"inputs": {"call_price": 10.450583572185565, "spot": 100, "strike": 100, "time": 1, "rate": 0.05}, "outputs": {'put_price': 5.573526022256971, 'pv_strike': 95.1229424500714, 'pv_spot': 100.0}}],
     questions=[{"q": "Use put-call parity with call price = 10.45, spot = 100, strike = 100, time = 1, rate = 5%.", "outputs": {'put_price': 5.572942450071409, 'pv_strike': 95.1229424500714, 'pv_spot': 100.0}},
                {"q": "Use put-call parity with call price = 150, spot = 100, strike = 100, time = 1, rate = 5%.", "outcome": "INVALID_INPUT"}])


def binomial_option(spot, strike, time, rate, volatility, steps, option_type="call", dividend_yield=0.0):
    S, K, T, r, s, q = _opt(spot, strike, time, rate, volatility, dividend_yield)
    n = _int("steps", steps, 1, 1000)
    _need(option_type in ("call", "put"), "option_type is call or put")
    dt = T / n
    u = math.exp(s * math.sqrt(dt))
    d = 1 / u
    p = (math.exp((r - q) * dt) - d) / (u - d)
    _need(0 < p < 1, "the risk-neutral probability is outside 0..1: use more steps")
    disc = math.exp(-r * dt)
    pay = (lambda x: max(x - K, 0.0)) if option_type == "call" else (lambda x: max(K - x, 0.0))
    v = [pay(S * u ** j * d ** (n - j)) for j in range(n + 1)]
    vu = vd = None
    for step in range(n, 0, -1):
        if step == 1:
            vd, vu = v[0], v[1]
        v = [disc * (p * v[j + 1] + (1 - p) * v[j]) for j in range(step)]
    return {"price": v[0], "delta": (vu - vd) / (S * u - S * d), "up_factor": u, "probability_up": p}


def _ref_binomial(spot, strike, time, rate, volatility, steps, option_type="call", dividend_yield=0.0):
    import numpy as np
    from scipy.stats import binom                          # the terminal distribution summed directly, no backward induction
    dt = time / steps
    u = math.exp(volatility * math.sqrt(dt))
    d = 1 / u
    p = (math.exp((rate - dividend_yield) * dt) - d) / (u - d)

    def value(S0, n, t):
        j = np.arange(n + 1)
        ST = S0 * u ** j * d ** (n - j)
        pay = np.maximum(ST - strike, 0) if option_type == "call" else np.maximum(strike - ST, 0)
        return math.exp(-rate * t) * float(np.sum(binom.pmf(j, n, p) * pay))
    vu, vd = value(spot * u, steps - 1, time - dt), value(spot * d, steps - 1, time - dt)
    return {"price": value(spot, steps, time), "delta": (vu - vd) / (spot * (u - d))}


_add("binomial_option", name="Binomial option price (Cox-Ross-Rubinstein)", topic="stocks and options / option pricing",
     purpose="European call or put price on an n-step Cox-Ross-Rubinstein tree (u = e^(sigma sqrt(dt)), d = 1/u), with "
             "the delta of the first step.",
     inputs=_opt_inputs(skip=("dividend_yield",), extra=[
         _i("steps", "number of time steps of the tree", "n", "number of steps", "periods", type="integer", min=1),
         _i("option_type", "call (default) or put", "type", "option", type="choice", choices=["call", "put"], required=False,
            syn={"calls": "call", "puts": "put"}),
         _i("dividend_yield", "continuous dividend yield (decimal, default 0)", "q", "dividend yield", required=False)]),
     outputs=[_o("price", "option price at time 0"), _o("delta", "(V_u - V_d) / (S u - S d)"), _o("up_factor", "u"),
              _o("probability_up", "risk-neutral p = (e^((r-q)dt) - d)/(u - d)")], primary="price",
     conditions=["European exercise", "1 <= steps <= 1000", "0 < p < 1"],
     method="backward induction V = e^(-r dt) (p V_up + (1 - p) V_down) from the payoffs at expiry",
     latex=r"u = e^{\sigma\sqrt{\Delta t}},\ d = 1/u,\ p = \frac{e^{(r-q)\Delta t} - d}{u - d},\ V = e^{-r\Delta t}[pV_u + (1-p)V_d]",
     run=binomial_option, reference=_ref_binomial,
     wrong=lambda spot, strike, time, rate, volatility, steps, option_type="call", dividend_yield=0.0: (lambda o: dict(
         o, price=binomial_option(spot, strike, time, rate, volatility, steps, option_type, dividend_yield)["price"] * math.exp(-rate * time / steps)))(
         binomial_option(spot, strike, time, rate, volatility, steps, option_type, dividend_yield)),
     rand=lambda rng: dict(_rand_opt(rng), steps=rng.choice([1, 2, 3, 5, 10, 50, 100, 200]), option_type=rng.choice(["call", "put"])),
     triggers=["binomial option", "binomial tree", "binomial model", "cox-ross-rubinstein", "crr tree", "binomial price"],
     templates=["Price the option with a binomial tree with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%, steps = 3."],
     examples=[{"inputs": {"spot": 100, "strike": 100, "time": 1, "rate": 0.05, "volatility": 0.2, "steps": 3}, "outputs": {'price': 11.043871091951113, 'delta': 0.6140855606115926, 'up_factor': 1.1224009024456676, 'probability_up': 0.5437765963610321}},
               {"inputs": {"spot": 100, "strike": 100, "time": 1, "rate": 0.05, "volatility": 0.2, "steps": 200, "option_type": "put"}, "outputs": {'price': 5.563533709930569, 'delta': -0.3633287895891145, 'up_factor': 1.0142426086996437, 'probability_up': 0.5053041700719276}}],
     questions=[{"q": "Price the option with a binomial tree with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%, steps = 3.",
                 "outputs": {'price': 11.043871091951113, 'delta': 0.6140855606115926, 'up_factor': 1.1224009024456676, 'probability_up': 0.5437765963610321}},
                {"q": "Price the option with a binomial tree with spot = 100, strike = 100, time = 1, rate = 5%, volatility = 20%, steps = 2.5.",
                 "outcome": "INVALID_INPUT"}])


def forward_price(spot, rate, time, dividend_yield=0.0, storage_cost=0.0):
    S, T = _pos("spot", spot), _nonneg("time", time)
    r, q, u = _rate("rate", rate), _rate("dividend_yield", dividend_yield), _rate("storage_cost", storage_cost)
    return {"forward": S * math.exp((r + u - q) * T), "cost_of_carry": r + u - q}


def _ref_forward_price(spot, rate, time, dividend_yield=0.0, storage_cost=0.0):
    import QuantLib as ql
    df = lambda x: ql.InterestRate(x, ql.Actual365Fixed(), ql.Continuous, ql.Annual).discountFactor(float(time))
    return {"forward": spot * df(dividend_yield - storage_cost) / df(rate)}


_add("forward_price", name="Forward price with cost of carry", topic="stocks and options / forwards",
     purpose="No-arbitrage forward price of an asset with a continuous income yield q and storage cost u: S e^((r + u - q) T).",
     inputs=[_i("spot", "spot price now", "s", "s0", "spot price", "price"),
             _i("rate", "risk-free rate, continuously compounded (decimal)", "r", "risk-free rate", "interest rate"),
             _i("time", "time to delivery in years", "t", "maturity", "years", "delivery"),
             _i("dividend_yield", "continuous income or convenience yield (decimal, default 0)", "q", "dividend yield", "income yield",
                "convenience yield", "foreign rate", required=False),
             _i("storage_cost", "continuous storage cost rate (decimal, default 0)", "u", "storage cost rate", required=False)],
     outputs=[_o("forward", "forward price"), _o("cost_of_carry", "r + u - q")], primary="forward",
     conditions=["spot > 0", "time >= 0", "rates as continuously compounded decimals"],
     method="F = S exp((r + u - q) T)", latex=r"F_0 = S_0\,e^{(r + u - q)T}", run=forward_price, reference=_ref_forward_price,
     wrong=lambda spot, rate, time, dividend_yield=0.0, storage_cost=0.0: {
         "forward": spot * (1 + (rate + storage_cost - dividend_yield) * time), "cost_of_carry": rate + storage_cost - dividend_yield},
     rand=lambda rng: {"spot": _u(rng, 10, 2000, 2), "rate": _u(rng, 0.0, 0.08), "time": _u(rng, 0.1, 5, 3),
                       "dividend_yield": rng.choice([0.0, _u(rng, 0.005, 0.06)]), "storage_cost": rng.choice([0.0, _u(rng, 0.005, 0.03)])},
     triggers=["forward price", "futures price", "cost of carry"],
     templates=["Find the forward price with spot = 100, rate = 5%, time = 0.5, dividend yield = 2%."],
     examples=[{"inputs": {"spot": 100, "rate": 0.05, "time": 0.5, "dividend_yield": 0.02}, "outputs": {'forward': 101.51130646157189, 'cost_of_carry': 0.030000000000000002}},
               {"inputs": {"spot": 100, "rate": 0.05, "time": 0}, "outputs": {"forward": 100.0}}],
     questions=[{"q": "Find the forward price with spot = 100, rate = 5%, time = 0.5, dividend yield = 2%.", "outputs": {'forward': 101.51130646157189, 'cost_of_carry': 0.030000000000000002}},
                {"q": "Find the futures price with spot = 1800, r = 4%, t = 1, storage cost = 1%.", "outputs": {'forward': 1892.2879734768435, 'cost_of_carry': 0.05}}])


def forward_price_dividends(spot, rate, time, dividends, dividend_times):
    S, T, r = _pos("spot", spot), _pos("time", time), _rate("rate", rate)
    D, t = _vec("dividends", dividends, 1), _vec("dividend_times", dividend_times, 1)
    _need(len(D) == len(t), "one time per dividend")
    _need(all(0 < x <= T for x in t), "each dividend time must lie in (0, time]")
    pv = math.fsum(d * math.exp(-r * x) for d, x in zip(D, t))
    _need(pv < S, "the dividends are worth more than the asset")
    return {"forward": (S - pv) * math.exp(r * T), "pv_dividends": pv}


_add("forward_price_dividends", name="Forward price with known dividends", topic="stocks and options / forwards",
     purpose="Forward price of a stock paying known cash dividends before delivery: F = (S - PV(dividends)) e^(rT).",
     inputs=[_i("spot", "stock price now", "s", "s0", "spot price", "price"),
             _i("rate", "risk-free rate, continuously compounded (decimal)", "r", "risk-free rate", "interest rate"),
             _i("time", "time to delivery in years", "t", "maturity", "years"),
             _i("dividends", "cash dividends (list)", "dividend", "d", "cash dividends", type="vector", min_length=1),
             _i("dividend_times", "time of each dividend in years (list)", "times", "dividend dates", "ex-dividend times", type="vector", min_length=1)],
     outputs=[_o("forward", "forward price"), _o("pv_dividends", "present value of the dividends")], primary="forward",
     conditions=["0 < each dividend time <= time", "PV(dividends) < spot"],
     method="F = (S - sum D_i e^(-r t_i)) e^(rT)", latex=r"F_0 = \left(S_0 - \sum_i D_i e^{-r t_i}\right)e^{rT}", run=forward_price_dividends,
     reference=lambda spot, rate, time, dividends, dividend_times: (lambda np: {
         "forward": float(spot * np.exp(rate * time) - np.sum(np.asarray(dividends) * np.exp(rate * (time - np.asarray(dividend_times)))))})(
         __import__("numpy")),
     wrong=lambda spot, rate, time, dividends, dividend_times: {"forward": (spot - sum(dividends)) * math.exp(rate * time),
                                                                "pv_dividends": sum(dividends)},
     rand=lambda rng: (lambda T, k: {"spot": _u(rng, 20, 500, 2), "rate": _u(rng, 0.005, 0.08), "time": T,
                                     "dividends": [_u(rng, 0.1, 3, 2) for _ in range(k)],
                                     "dividend_times": sorted(_u(rng, 0.01, T, 4) for _ in range(k))})(_u(rng, 0.25, 3, 3), rng.randint(1, 6)),
     triggers=["forward price with dividends", "forward price of a dividend-paying stock", "forward with known dividends",
               "forward price with known dividends"],
     templates=["Find the forward price with dividends with spot = 50, rate = 5%, time = 1, dividends = [0.75, 0.75], dividend times = [0.25, 0.75]."],
     examples=[{"inputs": {"spot": 50, "rate": 0.05, "time": 1, "dividends": "0.75, 0.75", "dividend_times": "0.25, 0.75"}, "outputs": {'forward': 51.02546198233436, 'pv_dividends': 1.4630791636610274}}],
     questions=[{"q": "Find the forward price with dividends with spot = 50, rate = 5%, time = 1, dividends = [0.75, 0.75], dividend times = [0.25, 0.75].",
                 "outputs": {'forward': 51.02546198233436, 'pv_dividends': 1.4630791636610274}},
                {"q": "Find the forward price with dividends with spot = 50, rate = 5%, time = 1, dividends = [0.75], dividend times = [1.5].",
                 "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# ACTUARIAL MATHEMATICS (annual life table q_x, q_x+1, ...; benefit at the end of the year of death)
# =====================================================================================================================
def _qtab(q, terminal):
    q = _vec("mortality_rates", q, 1)
    _need(all(0 <= x <= 1 for x in q), "each mortality rate q is a probability between 0 and 1")
    _need(not terminal or q[-1] == 1, "the table must end with q = 1 (no one survives past its last age)")
    return q


def _surv(q):
    p = [1.0]
    for x in q:
        p.append(p[-1] * (1 - x))
    return p


def _v(i):
    return 1 / (1 + _rate("interest_rate", i))


def _rand_q(rng, terminal=True):
    x0, a, b = rng.randint(20, 85), rng.uniform(2e-5, 1e-4), rng.uniform(0.07, 0.11)
    q = [round(min(0.99, a * math.exp(b * x)), 6) for x in range(x0, 111)]
    if terminal:
        q[-1] = 1.0
    return q


_QIN = _i("mortality_rates", "one-year death probabilities q_x, q_x+1, ... from the current age", "q", "qx", "mortality",
          "mortality table", "life table", type="vector", min_length=1)
_IIN = _i("interest_rate", "annual effective interest rate (decimal)", "i", "interest", "rate", "discount rate")


def _qin():
    return dict(_QIN, _aliases=list(_QIN["_aliases"]))


def _iin():
    return dict(_IIN, _aliases=list(_IIN["_aliases"]))


def survival_probability(mortality_rates, years, deferred_years=1):
    q = _qtab(mortality_rates, False)
    t = _int("years", years, 0, len(q))
    u = _int("deferred_years", deferred_years, 1, len(q) - t) if t < len(q) else _int("deferred_years", deferred_years, 1, 0)
    p = _surv(q)
    return {"survival": p[t], "death_probability": 1 - p[t], "deferred_death_probability": p[t] - p[t + u]}


def _ref_survival(mortality_rates, years, deferred_years=1):
    import numpy as np                                     # a cohort of 100000 lives, deaths subtracted year by year
    l = np.empty(len(mortality_rates) + 1)
    l[0] = 1e5
    for k, x in enumerate(mortality_rates):
        l[k + 1] = l[k] - l[k] * x
    return {"survival": l[years] / 1e5, "death_probability": (l[0] - l[years]) / 1e5,
            "deferred_death_probability": (l[years] - l[years + deferred_years]) / 1e5}


_add("survival_probability", name="Survival and death probabilities from a life table", topic="actuarial mathematics / life tables",
     purpose="t p_x (survive t years), t q_x (die within t years) and t|u q_x (survive t years, then die within the "
             "next u years) from one-year mortality rates.",
     inputs=[_qin(), _i("years", "number of years t", "t", "n", "survival years", type="integer", min=0),
             _i("deferred_years", "u, the length of the death window after t (default 1)", "u", "then within", type="integer",
                required=False, min=1)],
     outputs=[_o("survival", "t p_x = prod (1 - q)"), _o("death_probability", "t q_x = 1 - t p_x"),
              _o("deferred_death_probability", "t|u q_x = t p_x - (t+u) p_x")], primary="survival",
     conditions=["0 <= q <= 1", "t + u <= number of rates"], method="t p_x = prod_{k<t} (1 - q_{x+k})",
     latex=r"{}_{t}p_x = \prod_{k=0}^{t-1}(1 - q_{x+k}),\quad {}_{t|u}q_x = {}_{t}p_x - {}_{t+u}p_x", run=survival_probability,
     reference=_ref_survival,
     wrong=lambda mortality_rates, years, deferred_years=1: (lambda o: dict(o, survival=1 - sum(mortality_rates[:years])))(
         survival_probability(mortality_rates, years, deferred_years)),
     rand=lambda rng: (lambda q: {"mortality_rates": q, "years": rng.randint(2, len(q) - 1), "deferred_years": 1})(_rand_q(rng, False)),
     triggers=["survival probability", "probability of surviving", "probability of survival", "life table probability"],
     templates=["Find the survival probability with mortality rates = [0.01, 0.02, 0.03, 0.05], years = 3."],
     examples=[{"inputs": {"mortality_rates": "0.01, 0.02, 0.03, 0.05", "years": 3}, "outputs": {'survival': 0.9410939999999999, 'death_probability': 0.058906000000000125, 'deferred_death_probability': 0.047054700000000005}},
               {"inputs": {"mortality_rates": [0.1, 0.2], "years": 1, "deferred_years": 1}, "outputs": {"survival": 0.9, "deferred_death_probability": 0.18}}],
     questions=[{"q": "Find the survival probability with mortality rates = [0.01, 0.02, 0.03, 0.05], years = 3.", "outputs": {'survival': 0.9410939999999999, 'death_probability': 0.058906000000000125, 'deferred_death_probability': 0.047054700000000005}},
                {"q": "Find the survival probability with q = [0.01, 0.02], t = 5.", "outcome": "INVALID_INPUT"}])


def life_expectancy(mortality_rates):
    q = _qtab(mortality_rates, True)
    e = math.fsum(_surv(q)[1:])
    return {"curtate": e, "complete": e + 0.5}


_add("life_expectancy", name="Life expectancy from a life table", topic="actuarial mathematics / life tables",
     purpose="Curtate expectation of life e_x = sum_{k>=1} k p_x (whole years lived) and the complete expectation "
             "e_x + 1/2 (deaths uniform within each year).",
     inputs=[_qin()],
     outputs=[_o("curtate", "curtate expectation of life e_x"), _o("complete", "complete expectation e_x + 0.5 (UDD)")], primary="curtate",
     conditions=["the table ends with q = 1"], method="e_x = sum_{k=1}^{n} k p_x", latex=r"e_x = \sum_{k=1}^{\infty} {}_{k}p_x",
     run=life_expectancy,
     reference=lambda mortality_rates: (lambda np: (lambda p: (lambda e: {"curtate": e, "complete": e + 0.5})(
         float(np.sum(np.arange(len(mortality_rates)) * p[:-1] * np.asarray(mortality_rates)))))(
         np.concatenate([[1.0], np.cumprod(1 - np.asarray(mortality_rates))])))(__import__("numpy")),
     wrong=lambda mortality_rates: (lambda e: {"curtate": e, "complete": e + 0.5})(math.fsum(_surv(mortality_rates))),
     rand=lambda rng: {"mortality_rates": _rand_q(rng)},
     triggers=["life expectancy", "expectation of life", "expected future lifetime"],
     templates=["Find the life expectancy with mortality rates = [0.1, 0.3, 0.6, 1]."],
     examples=[{"inputs": {"mortality_rates": "0.1, 0.3, 0.6, 1"}, "outputs": {'curtate': 1.782, 'complete': 2.282}}, {"inputs": {"mortality_rates": [1.0]}, "outputs": {"curtate": 0.0}}],
     questions=[{"q": "Find the life expectancy with mortality rates = [0.1, 0.3, 0.6, 1].", "outputs": {'curtate': 1.782, 'complete': 2.282}},
                {"q": "Find the life expectancy with mortality rates = [0.1, 0.3, 0.6].", "outcome": "INVALID_INPUT"}])


def _A(q, v, n):
    p = _surv(q)
    return math.fsum(v ** (k + 1) * p[k] * q[k] for k in range(n))


def _recur_A(q, v, n, end=0.0):
    """Backward recursion A_k = v q_k + v p_k A_{k+1} (a different computation from the direct sum)."""
    A = end
    for k in reversed(range(n)):
        A = v * q[k] + v * (1 - q[k]) * A
    return A


def whole_life_insurance(mortality_rates, interest_rate, benefit=1.0):
    q, v, B = _qtab(mortality_rates, True), _v(interest_rate), _pos("benefit", benefit)
    return {"apv": B * _A(q, v, len(q))}


_BIN = _i("benefit", "benefit amount (default 1)", "sum insured", "death benefit", "face amount", "b", required=False)


def _bin():
    return dict(_BIN, _aliases=list(_BIN["_aliases"]))


_add("whole_life_insurance", name="Whole life insurance (net single premium)", topic="actuarial mathematics / life insurance",
     purpose="Actuarial present value A_x of a benefit paid at the end of the year of death, whenever it occurs.",
     inputs=[_qin(), _iin(), _bin()],
     outputs=[_o("apv", "B A_x = B sum v^(k+1) k p_x q_x+k")], primary="apv", conditions=["the table ends with q = 1", "-1 < interest_rate < 1"],
     method="A_x = sum_k v^(k+1) k p_x q_(x+k)", latex=r"A_x = \sum_{k=0}^{\infty} v^{k+1}\,{}_{k}p_x\,q_{x+k}", run=whole_life_insurance,
     reference=lambda mortality_rates, interest_rate, benefit=1.0: {"apv": benefit * _recur_A(mortality_rates, 1 / (1 + interest_rate), len(mortality_rates))},
     wrong=lambda mortality_rates, interest_rate, benefit=1.0: {"apv": benefit * _A(mortality_rates, 1 / (1 + interest_rate), len(mortality_rates)) * (1 + interest_rate)},
     rand=lambda rng: {"mortality_rates": _rand_q(rng), "interest_rate": _u(rng, 0.01, 0.08), "benefit": float(rng.choice([1, 1000, 100000]))},
     triggers=["whole life insurance", "whole-life insurance", "net single premium for whole life"],
     templates=["Find the whole life insurance value with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, benefit = 1000."],
     examples=[{"inputs": {"mortality_rates": "0.1, 0.3, 0.6, 1", "interest_rate": 0.05, "benefit": 1000}, "outputs": {'apv': 873.9876903142208}},
               {"inputs": {"mortality_rates": [1.0], "interest_rate": 0.25}, "outputs": {"apv": 0.8}}],
     questions=[{"q": "Find the whole life insurance value with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, benefit = 1000.", "outputs": {'apv': 873.9876903142208}}])


def term_insurance(mortality_rates, interest_rate, term, benefit=1.0):
    q, v, B = _qtab(mortality_rates, False), _v(interest_rate), _pos("benefit", benefit)
    n = _int("term", term, 1, len(q))
    return {"apv": B * _A(q, v, n)}


_TIN = _i("term", "term n in years", "n", "years", "term in years", "duration", type="integer", min=1)


def _tin():
    return dict(_TIN, _aliases=list(_TIN["_aliases"]))


_add("term_insurance", name="Term life insurance (net single premium)", topic="actuarial mathematics / life insurance",
     purpose="Actuarial present value of a benefit paid at the end of the year of death if death occurs within n years.",
     inputs=[_qin(), _iin(), _tin(), _bin()],
     outputs=[_o("apv", "B A^1_{x:n} = B sum_{k<n} v^(k+1) k p_x q_x+k")], primary="apv", conditions=["1 <= term <= number of rates"],
     method="A^1_{x:n} = sum_{k=0}^{n-1} v^(k+1) k p_x q_(x+k)", latex=r"A^{1}_{x:\overline{n}|} = \sum_{k=0}^{n-1} v^{k+1}\,{}_{k}p_x\,q_{x+k}",
     run=term_insurance,
     reference=lambda mortality_rates, interest_rate, term, benefit=1.0: {"apv": benefit * _recur_A(mortality_rates, 1 / (1 + interest_rate), term)},
     wrong=lambda mortality_rates, interest_rate, term, benefit=1.0: {"apv": benefit * _A(mortality_rates, 1 / (1 + interest_rate), term) * (1 + interest_rate)},
     rand=lambda rng: (lambda q: {"mortality_rates": q, "interest_rate": _u(rng, 0.01, 0.08), "term": rng.randint(1, len(q)),
                                  "benefit": float(rng.choice([1, 1000, 100000]))})(_rand_q(rng, False)),
     triggers=["term insurance", "term life insurance", "term assurance"],
     templates=["Find the term insurance value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, benefit = 100000."],
     examples=[{"inputs": {"mortality_rates": "0.01, 0.02, 0.03", "interest_rate": 0.04, "term": 3, "benefit": 100000}, "outputs": {'apv': 5379.672564861173}}],
     questions=[{"q": "Find the term insurance value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, benefit = 100000.", "outputs": {'apv': 5379.672564861173}},
                {"q": "Find the term insurance value with q = [0.01, 0.02], i = 4%, n = 3.", "outcome": "INVALID_INPUT"}])


def _a_due(q, v, n):
    p = _surv(q)
    return math.fsum(v ** k * p[k] for k in range(n))


def life_annuity_due(mortality_rates, interest_rate, payment=1.0):
    q, v, P = _qtab(mortality_rates, True), _v(interest_rate), _pos("payment", payment)
    return {"apv": P * _a_due(q, v, len(q))}


_PIN = _i("payment", "annual payment (default 1)", "annual payment", "annuity payment", "amount", required=False)


def _pin():
    return dict(_PIN, _aliases=list(_PIN["_aliases"]))


_add("life_annuity_due", name="Whole life annuity-due", topic="actuarial mathematics / life annuities",
     purpose="Actuarial present value of a payment at the start of every year while the person is alive.",
     inputs=[_qin(), _iin(), _pin()],
     outputs=[_o("apv", "P a-due_x = P sum v^k k p_x")], primary="apv", conditions=["the table ends with q = 1", "-1 < interest_rate < 1"],
     method="a-due_x = sum_k v^k k p_x", latex=r"\ddot a_x = \sum_{k=0}^{\infty} v^{k}\,{}_{k}p_x", run=life_annuity_due,
     reference=lambda mortality_rates, interest_rate, payment=1.0: {"apv": payment * (1 - _A(mortality_rates, 1 / (1 + interest_rate), len(
         mortality_rates))) / (interest_rate / (1 + interest_rate))},
     wrong=lambda mortality_rates, interest_rate, payment=1.0: {"apv": payment * (_a_due(mortality_rates, 1 / (1 + interest_rate), len(mortality_rates)) - 1)},
     rand=lambda rng: {"mortality_rates": _rand_q(rng), "interest_rate": _u(rng, 0.01, 0.08), "payment": float(rng.choice([1, 1000, 12000]))},
     triggers=["life annuity-due", "life annuity due", "whole life annuity", "life annuity"],
     templates=["Find the life annuity-due value with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, payment = 1000."],
     examples=[{"inputs": {"mortality_rates": "0.1, 0.3, 0.6, 1", "interest_rate": 0.05, "payment": 1000}, "outputs": {'apv': 2646.2585034013605}},
               {"inputs": {"mortality_rates": [1.0], "interest_rate": 0.05}, "outputs": {"apv": 1.0}}],
     questions=[{"q": "Find the life annuity-due value with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, payment = 1000.", "outputs": {'apv': 2646.2585034013605}}])


def temporary_annuity_due(mortality_rates, interest_rate, term, payment=1.0):
    q, v, P = _qtab(mortality_rates, False), _v(interest_rate), _pos("payment", payment)
    n = _int("term", term, 1, len(q))
    return {"apv": P * _a_due(q, v, n)}


_add("temporary_annuity_due", name="Temporary life annuity-due", topic="actuarial mathematics / life annuities",
     purpose="Actuarial present value of up to n annual payments at the start of each year while alive.",
     inputs=[_qin(), _iin(), _tin(), _pin()],
     outputs=[_o("apv", "P sum_{k<n} v^k k p_x")], primary="apv", conditions=["1 <= term <= number of rates"],
     method="a-due_{x:n} = sum_{k=0}^{n-1} v^k k p_x", latex=r"\ddot a_{x:\overline{n}|} = \sum_{k=0}^{n-1} v^{k}\,{}_{k}p_x",
     run=temporary_annuity_due,
     reference=lambda mortality_rates, interest_rate, term, payment=1.0: (lambda v: payment and {"apv": payment * (1 - (
         _recur_A(mortality_rates, v, term, 1.0))) / (1 - v)})(1 / (1 + interest_rate)),
     wrong=lambda mortality_rates, interest_rate, term, payment=1.0: (lambda v, p: {"apv": payment * math.fsum(v ** k * p[k] for k in range(1, term + 1))})(
         1 / (1 + interest_rate), _surv(mortality_rates)),
     rand=lambda rng: (lambda q: {"mortality_rates": q, "interest_rate": _u(rng, 0.01, 0.08), "term": rng.randint(1, len(q)),
                                  "payment": float(rng.choice([1, 1000]))})(_rand_q(rng, False)),
     triggers=["temporary annuity", "temporary life annuity", "temporary annuity-due", "n-year life annuity"],
     templates=["Find the temporary annuity value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, payment = 1000."],
     examples=[{"inputs": {"mortality_rates": "0.01, 0.02, 0.03", "interest_rate": 0.04, "term": 3, "payment": 1000}, "outputs": {'apv': 2848.9275147928993}}],
     questions=[{"q": "Find the temporary annuity value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, payment = 1000.",
                 "outputs": {'apv': 2848.9275147928993}}])


def net_premium_whole_life(mortality_rates, interest_rate, benefit=1.0):
    q, v, B = _qtab(mortality_rates, True), _v(interest_rate), _pos("benefit", benefit)
    A, a = _A(q, v, len(q)), _a_due(q, v, len(q))
    return {"premium": B * A / a, "insurance_apv": B * A, "annuity_due": a}


def _ref_net_premium(mortality_rates, interest_rate, benefit=1.0):
    v = 1 / (1 + interest_rate)
    a = 0.0                                                # a-due by backward recursion a_k = 1 + v p_k a_{k+1}
    for x in reversed(mortality_rates):
        a = 1 + v * (1 - x) * a
    return {"premium": benefit * (1 / a - (1 - v)), "annuity_due": a}


_add("net_premium_whole_life", name="Net annual premium for whole life insurance", topic="actuarial mathematics / premiums",
     purpose="The level annual net premium, paid at the start of each year while alive, for a whole life benefit "
             "(equivalence principle): P = B A_x / a-due_x.",
     inputs=[_qin(), _iin(), _bin()],
     outputs=[_o("premium", "net annual premium B A_x / a-due_x"), _o("insurance_apv", "B A_x"), _o("annuity_due", "a-due_x")],
     primary="premium", conditions=["the table ends with q = 1", "0 < interest_rate < 1"],
     method="equivalence principle: P a-due_x = B A_x", latex=r"P = B\,\frac{A_x}{\ddot a_x}", run=net_premium_whole_life,
     reference=_ref_net_premium,
     wrong=lambda mortality_rates, interest_rate, benefit=1.0: (lambda o: dict(o, premium=o["insurance_apv"] / (o["annuity_due"] - 1)
                                                                                if o["annuity_due"] > 1 else 0.0))(
         net_premium_whole_life(mortality_rates, interest_rate, benefit)),
     rand=lambda rng: {"mortality_rates": _rand_q(rng), "interest_rate": _u(rng, 0.01, 0.08), "benefit": float(rng.choice([1, 1000, 100000]))},
     triggers=["net premium", "net annual premium", "benefit premium", "level annual premium"],
     templates=["Find the net annual premium with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, benefit = 1000."],
     examples=[{"inputs": {"mortality_rates": "0.1, 0.3, 0.6, 1", "interest_rate": 0.05, "benefit": 1000}, "outputs": {'premium': 330.27298322928135, 'insurance_apv': 873.9876903142208, 'annuity_due': 2.6462585034013606}}],
     questions=[{"q": "Find the net annual premium with mortality rates = [0.1, 0.3, 0.6, 1], interest rate = 5%, benefit = 1000.", "outputs": {'premium': 330.27298322928135, 'insurance_apv': 873.9876903142208, 'annuity_due': 2.6462585034013606}}])


def endowment_insurance(mortality_rates, interest_rate, term, benefit=1.0):
    q, v, B = _qtab(mortality_rates, False), _v(interest_rate), _pos("benefit", benefit)
    n = _int("term", term, 1, len(q))
    t, pe = _A(q, v, n), v ** n * _surv(q)[n]
    return {"apv": B * (t + pe), "term_part": B * t, "pure_endowment": B * pe}


_add("endowment_insurance", name="Endowment insurance", topic="actuarial mathematics / life insurance",
     purpose="Actuarial present value of a benefit paid at the end of the year of death within n years, or at year n "
             "if alive then (term insurance + pure endowment).",
     inputs=[_qin(), _iin(), _tin(), _bin()],
     outputs=[_o("apv", "B (A^1_{x:n} + nE_x)"), _o("term_part", "B A^1_{x:n}"), _o("pure_endowment", "B v^n n p_x")], primary="apv",
     conditions=["1 <= term <= number of rates"], method="A_{x:n} = A^1_{x:n} + v^n n p_x",
     latex=r"A_{x:\overline{n}|} = A^{1}_{x:\overline{n}|} + v^{n}\,{}_{n}p_x", run=endowment_insurance,
     reference=lambda mortality_rates, interest_rate, term, benefit=1.0: (lambda v: {
         "apv": benefit * _recur_A(mortality_rates, v, term, 1.0), "term_part": benefit * _recur_A(mortality_rates, v, term)})(1 / (1 + interest_rate)),
     wrong=lambda mortality_rates, interest_rate, term, benefit=1.0: (lambda o: dict(o, apv=o["term_part"] + benefit * _surv(mortality_rates)[term]))(
         endowment_insurance(mortality_rates, interest_rate, term, benefit)),
     rand=lambda rng: (lambda q: {"mortality_rates": q, "interest_rate": _u(rng, 0.01, 0.08), "term": rng.randint(1, len(q)),
                                  "benefit": float(rng.choice([1, 1000, 100000]))})(_rand_q(rng, False)),
     triggers=["endowment insurance", "endowment assurance", "endowment policy"],
     templates=["Find the endowment insurance value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, benefit = 1000."],
     examples=[{"inputs": {"mortality_rates": "0.01, 0.02, 0.03", "interest_rate": 0.04, "term": 3, "benefit": 1000}, "outputs": {'apv': 890.4258648156573, 'term_part': 53.796725648611734, 'pure_endowment': 836.6291391670457}}],
     questions=[{"q": "Find the endowment insurance value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, benefit = 1000.",
                 "outputs": {'apv': 890.4258648156573, 'term_part': 53.796725648611734, 'pure_endowment': 836.6291391670457}}])


# =====================================================================================================================
# MICROECONOMICS
# =====================================================================================================================
def _arc(x1, q1, x2, q2, nx, nq):
    a, b, c, d = _pos(nx + "_1", x1), _pos(nq + "_1", q1), _pos(nx + "_2", x2), _pos(nq + "_2", q2)
    _need(a != c, f"the two {nx.replace('_', ' ')}s must differ")
    e = ((d - b) / ((b + d) / 2)) / ((c - a) / ((a + c) / 2))
    ex = (Fraction(d) - Fraction(b)) / (Fraction(b) + Fraction(d)) / ((Fraction(c) - Fraction(a)) / (Fraction(a) + Fraction(c)))
    return e, ex


def _ref_arc(x1, q1, x2, q2):
    import numpy as np                                     # slope of the line through the two points, at the midpoint
    slope = np.polyfit([x1, x2], [q1, q2], 1)[0]
    return float(slope * ((x1 + x2) / 2) / ((q1 + q2) / 2))


def _rand_arc(rng, nx, nq):
    x1 = _u(rng, 1, 100, 2)
    return {nx + "_1": x1, nq + "_1": _u(rng, 10, 1000, 1), nx + "_2": round(x1 * rng.uniform(1.05, 1.6), 2), nq + "_2": _u(rng, 10, 1000, 1)}


def arc_price_elasticity(price_1, quantity_1, price_2, quantity_2):
    e, ex = _arc(price_1, quantity_1, price_2, quantity_2, "price", "quantity")
    cls = "perfectly inelastic" if ex == 0 else ("unit elastic" if abs(ex) == 1 else ("elastic" if abs(ex) > 1 else "inelastic"))
    return {"elasticity": e, "abs_elasticity": abs(e), "classification": cls}


_add("arc_price_elasticity", name="Price elasticity of demand (midpoint)", topic="microeconomics / elasticity",
     purpose="Arc (midpoint) price elasticity of demand between two price-quantity points, and whether demand is "
             "elastic, inelastic or unit elastic there.",
     inputs=[_i("price_1", "first price", "p1", "initial price", "old price"), _i("quantity_1", "quantity demanded at price 1", "q1", "initial quantity", "old quantity"),
             _i("price_2", "second price", "p2", "new price", "final price"), _i("quantity_2", "quantity demanded at price 2", "q2", "new quantity", "final quantity")],
     outputs=[_o("elasticity", "(dQ / mean Q) / (dP / mean P)"), _o("abs_elasticity", "|elasticity|"),
              _o("classification", "elastic, inelastic, unit elastic or perfectly inelastic", "text")], primary="elasticity",
     conditions=["prices and quantities > 0", "the two prices differ"],
     method="midpoint formula: percentage changes relative to the averages",
     latex=r"E = \frac{(Q_2 - Q_1)/[(Q_1 + Q_2)/2]}{(P_2 - P_1)/[(P_1 + P_2)/2]}", run=arc_price_elasticity,
     reference=lambda price_1, quantity_1, price_2, quantity_2: {"elasticity": _ref_arc(price_1, quantity_1, price_2, quantity_2)},
     wrong=lambda price_1, quantity_1, price_2, quantity_2: (lambda e: {"elasticity": e, "abs_elasticity": abs(e), "classification": "?"})(
         ((quantity_2 - quantity_1) / quantity_1) / ((price_2 - price_1) / price_1)),
     rand=lambda rng: _rand_arc(rng, "price", "quantity"),
     triggers=["price elasticity of demand", "price elasticity", "arc elasticity", "midpoint elasticity", "elasticity of demand"],
     templates=["Find the price elasticity of demand with p1 = 10, q1 = 100, p2 = 12, q2 = 80."],
     examples=[{"inputs": {"price_1": 10, "quantity_1": 100, "price_2": 12, "quantity_2": 80}, "outputs": {'elasticity': -1.222222222222222, 'abs_elasticity': 1.222222222222222}},
               {"inputs": {"price_1": 10, "quantity_1": 12, "price_2": 12, "quantity_2": 10}, "outputs": {"elasticity": -1.0, "classification": "unit elastic"}}],
     questions=[{"q": "Find the price elasticity of demand with p1 = 10, q1 = 100, p2 = 12, q2 = 80.", "outputs": {'elasticity': -1.222222222222222, 'abs_elasticity': 1.222222222222222}},
                {"q": "Find the price elasticity of demand with p1 = 10, q1 = 100, p2 = 10, q2 = 80.", "outcome": "INVALID_INPUT"}])


def income_elasticity(income_1, quantity_1, income_2, quantity_2):
    e, ex = _arc(income_1, quantity_1, income_2, quantity_2, "income", "quantity")
    cls = "inferior good" if ex < 0 else ("neutral" if ex == 0 else ("normal good (luxury)" if ex > 1 else "normal good (necessity)"))
    return {"elasticity": e, "classification": cls}


_add("income_elasticity", name="Income elasticity of demand (midpoint)", topic="microeconomics / elasticity",
     purpose="Midpoint income elasticity of demand and the good's type (inferior, necessity, luxury).",
     inputs=[_i("income_1", "first income", "i1", "m1", "initial income", "old income"), _i("quantity_1", "quantity at income 1", "q1", "initial quantity"),
             _i("income_2", "second income", "i2", "m2", "new income"), _i("quantity_2", "quantity at income 2", "q2", "new quantity")],
     outputs=[_o("elasticity", "(dQ / mean Q) / (dI / mean I)"), _o("classification", "inferior, necessity or luxury", "text")],
     primary="elasticity", conditions=["incomes and quantities > 0", "the incomes differ"], method="midpoint formula",
     latex=r"E_I = \frac{(Q_2 - Q_1)/\bar Q}{(I_2 - I_1)/\bar I}", run=income_elasticity,
     reference=lambda income_1, quantity_1, income_2, quantity_2: {"elasticity": _ref_arc(income_1, quantity_1, income_2, quantity_2)},
     wrong=lambda income_1, quantity_1, income_2, quantity_2: {"elasticity": ((quantity_2 - quantity_1) / quantity_1) / ((income_2 - income_1) / income_1),
                                                               "classification": "?"},
     rand=lambda rng: _rand_arc(rng, "income", "quantity"),
     triggers=["income elasticity", "income elasticity of demand"],
     templates=["Find the income elasticity with income 1 = 40000, q1 = 10, income 2 = 50000, q2 = 13."],
     examples=[{"inputs": {"income_1": 40000, "quantity_1": 10, "income_2": 50000, "quantity_2": 13}, "outputs": {'elasticity': 1.173913043478261}}],
     questions=[{"q": "Find the income elasticity with income 1 = 40000, q1 = 10, income 2 = 50000, q2 = 13.", "outputs": {'elasticity': 1.173913043478261}}])


def cross_price_elasticity(price_other_1, quantity_1, price_other_2, quantity_2):
    e, ex = _arc(price_other_1, quantity_1, price_other_2, quantity_2, "price_other", "quantity")
    return {"elasticity": e, "relationship": "substitutes" if ex > 0 else ("complements" if ex < 0 else "independent")}


_add("cross_price_elasticity", name="Cross-price elasticity of demand (midpoint)", topic="microeconomics / elasticity",
     purpose="Midpoint cross-price elasticity: response of the quantity of good A to the price of good B; positive = "
             "substitutes, negative = complements.",
     inputs=[_i("price_other_1", "first price of the other good", "price 1", "p1", "price of b 1", "other price 1", "pb1"),
             _i("quantity_1", "quantity of this good at that price", "q1", "quantity of a 1", "qa1"),
             _i("price_other_2", "second price of the other good", "price 2", "p2", "price of b 2", "other price 2", "pb2"),
             _i("quantity_2", "quantity of this good at the second price", "q2", "quantity of a 2", "qa2")],
     outputs=[_o("elasticity", "(dQa / mean Qa) / (dPb / mean Pb)"), _o("relationship", "substitutes, complements or independent", "text")],
     primary="elasticity", conditions=["prices and quantities > 0", "the prices differ"], method="midpoint formula",
     latex=r"E_{AB} = \frac{\Delta Q_A/\bar Q_A}{\Delta P_B/\bar P_B}", run=cross_price_elasticity,
     reference=lambda price_other_1, quantity_1, price_other_2, quantity_2: {"elasticity": _ref_arc(price_other_1, quantity_1, price_other_2, quantity_2)},
     wrong=lambda price_other_1, quantity_1, price_other_2, quantity_2: {
         "elasticity": ((quantity_2 - quantity_1) / quantity_1) / ((price_other_2 - price_other_1) / price_other_1), "relationship": "?"},
     rand=lambda rng: _rand_arc(rng, "price_other", "quantity"),
     triggers=["cross-price elasticity", "cross price elasticity", "cross elasticity", "cross-price elasticity of demand",
               "cross price elasticity of demand", "cross elasticity of demand"],
     templates=["Find the cross-price elasticity with p1 = 2, q1 = 100, p2 = 2.5, q2 = 120."],
     examples=[{"inputs": {"price_other_1": 2, "quantity_1": 100, "price_other_2": 2.5, "quantity_2": 120}, "outputs": {'elasticity': 0.8181818181818182}}],
     questions=[{"q": "Find the cross-price elasticity with p1 = 2, q1 = 100, p2 = 2.5, q2 = 120.", "outputs": {'elasticity': 0.8181818181818182}},
                {"q": "Find the cross-price elasticity of demand with price 1 = 2, q1 = 100, price 2 = 2.5, q2 = 80.", "outputs": {'elasticity': -1.0}}])


def point_elasticity(demand_intercept, demand_slope, price):
    a, b, P = _pos("demand_intercept", demand_intercept), _pos("demand_slope", demand_slope), _pos("price", price)
    Q = a - b * P
    _need(Q > 0, "the price is at or above the choke price a/b: nothing is demanded")
    return {"quantity": Q, "elasticity": -b * P / Q, "revenue": P * Q, "revenue_maximizing_price": a / (2 * b)}


def _ref_point(demand_intercept, demand_slope, price):
    import sympy as sp                                     # symbolic derivative dQ/dP times P/Q
    P = sp.Symbol("P")
    R = lambda v: sp.Rational(Fraction(v).numerator, Fraction(v).denominator)
    Q = R(demand_intercept) - R(demand_slope) * P
    p0 = R(price)
    rev = P * Q
    return {"quantity": float(Q.subs(P, p0)), "elasticity": float((sp.diff(Q, P) * P / Q).subs(P, p0)),
            "revenue": float(rev.subs(P, p0)), "revenue_maximizing_price": float(sp.solve(sp.diff(rev, P), P)[0])}


_add("point_elasticity", name="Point elasticity of a linear demand curve", topic="microeconomics / elasticity",
     purpose="Quantity, point price elasticity and revenue on the linear demand curve Q = a - b P at a price, and the "
             "revenue-maximising price.",
     inputs=[_i("demand_intercept", "a in Q = a - bP", "a", "intercept", "quantity intercept"),
             _i("demand_slope", "b in Q = a - bP (b > 0)", "b", "slope"),
             _i("price", "price at which to evaluate", "p")],
     outputs=[_o("quantity", "Q = a - bP"), _o("elasticity", "-b P / Q"), _o("revenue", "P Q"), _o("revenue_maximizing_price", "a / (2b)")],
     primary="elasticity", conditions=["a, b, P > 0", "Q = a - bP > 0"], method="E = (dQ/dP)(P/Q) = -b P / (a - bP)",
     latex=r"E = \frac{dQ}{dP}\frac{P}{Q} = \frac{-bP}{a - bP}", run=point_elasticity, reference=_ref_point,
     wrong=lambda demand_intercept, demand_slope, price: (lambda o: dict(o, elasticity=-demand_slope * o["quantity"] / price))(
         point_elasticity(demand_intercept, demand_slope, price)),
     rand=lambda rng: (lambda a, b: {"demand_intercept": a, "demand_slope": b, "price": round(a / b * rng.uniform(0.05, 0.95), 3)})(
         _u(rng, 50, 5000, 1), _u(rng, 0.5, 50, 2)),
     triggers=["point elasticity", "point price elasticity", "point elasticity of demand", "point price elasticity of demand",
               "elasticity of linear demand", "elasticity on a linear demand"],
     templates=["Find the point elasticity with demand intercept = 100, demand slope = 2, price = 20."],
     examples=[{"inputs": {"demand_intercept": 100, "demand_slope": 2, "price": 20}, "outputs": {"quantity": 60.0, "elasticity": -2 / 3,
                                                                                               "revenue": 1200.0, "revenue_maximizing_price": 25.0}}],
     questions=[{"q": "Find the point elasticity with demand intercept = 100, demand slope = 2, price = 20.", "outputs": {"elasticity": -2 / 3}},
                {"q": "Find the point elasticity with a = 100, b = 2, price = 60.", "outcome": "INVALID_INPUT"}])


_MKT_IN = [_i("demand_intercept", "a in Qd = a - bP", "a", "demand intercept"),
           _i("demand_slope", "b in Qd = a - bP (b > 0)", "b", "demand slope"),
           _i("supply_intercept", "c in Qs = c + dP", "c", "supply intercept"),
           _i("supply_slope", "d in Qs = c + dP (d > 0)", "d", "supply slope")]


def _mkt():
    return [dict(i, _aliases=list(i["_aliases"])) for i in _MKT_IN]


def _ps(c, d, P):
    """Producer surplus: area above the supply curve (at prices >= 0) and below P."""
    return c * P + d * P * P / 2 if c >= 0 else d * (P + c / d) ** 2 / 2


def market_equilibrium(demand_intercept, demand_slope, supply_intercept, supply_slope):
    a, b = _pos("demand_intercept", demand_intercept), _pos("demand_slope", demand_slope)
    c, d = _fin("supply_intercept", supply_intercept), _pos("supply_slope", supply_slope)
    P = (a - c) / (b + d)
    Q = a - b * P
    _need(P > 0 and Q > 0, "the curves do not cross at a positive price and quantity")
    cs, ps = Q * Q / (2 * b), _ps(c, d, P)
    return {"price": P, "quantity": Q, "consumer_surplus": cs, "producer_surplus": ps, "total_surplus": cs + ps}


def _ref_mkt(demand_intercept, demand_slope, supply_intercept, supply_slope):
    import numpy as np
    from scipy.integrate import quad                       # surpluses as areas in quantity space
    a, b, c, d = demand_intercept, demand_slope, supply_intercept, supply_slope
    Q, P = np.linalg.solve(np.array([[1.0, b], [1.0, -d]]), np.array([a, c]))
    cs = quad(lambda q: (a - q) / b - P, 0, Q, epsabs=1e-14, epsrel=1e-14)[0]
    pts = [c] if 0 < c < Q else None
    ps = quad(lambda q: P - max(0.0, (q - c) / d), 0, Q, points=pts, epsabs=1e-14, epsrel=1e-14)[0]
    return {"price": float(P), "quantity": float(Q), "consumer_surplus": cs, "producer_surplus": ps}


def _rand_mkt(rng):
    return {"demand_intercept": _u(rng, 100, 2000, 1), "demand_slope": _u(rng, 0.5, 20, 2),
            "supply_intercept": _u(rng, -200, 80, 1), "supply_slope": _u(rng, 0.5, 20, 2)}


_add("market_equilibrium", name="Market equilibrium and surplus (linear curves)", topic="microeconomics / supply and demand",
     purpose="Equilibrium price and quantity of Qd = a - bP and Qs = c + dP, with consumer, producer and total surplus "
             "(producer surplus counted at prices >= 0).",
     inputs=_mkt(),
     outputs=[_o("price", "equilibrium price (a - c)/(b + d)"), _o("quantity", "equilibrium quantity"), _o("consumer_surplus", "Q^2 / (2b)"),
              _o("producer_surplus", "area above supply, below P"), _o("total_surplus", "CS + PS")], primary="price",
     conditions=["b, d > 0", "positive equilibrium price and quantity"], method="a - bP = c + dP; surpluses are triangle/trapezoid areas",
     latex=r"P^{*} = \frac{a - c}{b + d},\quad CS = \frac{Q^{*2}}{2b}", run=market_equilibrium, reference=_ref_mkt,
     wrong=lambda demand_intercept, demand_slope, supply_intercept, supply_slope: (lambda o: dict(
         o, consumer_surplus=(demand_intercept - o["price"]) * o["quantity"] / 2))(
         market_equilibrium(demand_intercept, demand_slope, supply_intercept, supply_slope)),
     rand=_rand_mkt,
     triggers=["market equilibrium", "equilibrium price", "equilibrium quantity", "consumer surplus", "producer surplus"],
     templates=["Find the market equilibrium with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4."],
     examples=[{"inputs": {"demand_intercept": 100, "demand_slope": 2, "supply_intercept": -20, "supply_slope": 4},
                "outputs": {"price": 20.0, "quantity": 60.0, "consumer_surplus": 900.0, "producer_surplus": 450.0}},
               {"inputs": {"demand_intercept": 100, "demand_slope": 2, "supply_intercept": 10, "supply_slope": 1}, "outputs": {'price': 30.0, 'quantity': 40.0, 'consumer_surplus': 400.0, 'producer_surplus': 750.0, 'total_surplus': 1150.0}}],
     questions=[{"q": "Find the market equilibrium with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4.",
                 "outputs": {"price": 20.0, "quantity": 60.0}},
                {"q": "Find the equilibrium price with a = 100, b = 2, c = 200, d = 4.", "outcome": "INVALID_INPUT"}])


def per_unit_tax(demand_intercept, demand_slope, supply_intercept, supply_slope, tax):
    e = market_equilibrium(demand_intercept, demand_slope, supply_intercept, supply_slope)
    t = _pos("tax", tax)
    a, b, c, d = demand_intercept, demand_slope, supply_intercept, supply_slope
    Pb = (a - c + d * t) / (b + d)
    Qt, Ps = a - b * Pb, Pb - t
    _need(Qt > 0 and Ps > 0, "the tax is so large that the market closes (no positive quantity or seller price)")
    return {"quantity": Qt, "buyer_price": Pb, "seller_price": Ps, "tax_revenue": t * Qt,
            "deadweight_loss": t * (e["quantity"] - Qt) / 2, "buyer_share": (Pb - e["price"]) / t}


def _ref_tax(demand_intercept, demand_slope, supply_intercept, supply_slope, tax):
    import numpy as np
    from scipy.integrate import quad
    a, b, c, d = demand_intercept, demand_slope, supply_intercept, supply_slope
    Q0, P0 = np.linalg.solve(np.array([[1.0, b], [1.0, -d]]), np.array([a, c]))
    Qt, Pb = np.linalg.solve(np.array([[1.0, b], [1.0, -d]]), np.array([a, c - d * tax]))
    dwl = quad(lambda q: (a - q) / b - (q - c) / d, Qt, Q0, epsabs=1e-14, epsrel=1e-14)[0]
    return {"quantity": float(Qt), "buyer_price": float(Pb), "seller_price": float(Pb - tax), "tax_revenue": float(tax * Qt),
            "deadweight_loss": dwl, "buyer_share": float((Pb - P0) / tax)}


_add("per_unit_tax", name="Per-unit tax incidence (linear curves)", topic="microeconomics / taxation",
     purpose="Effect of a per-unit tax on Qd = a - bP, Qs = c + dP: quantity, price paid by buyers and kept by sellers, "
             "tax revenue, deadweight loss and the buyers' share of the burden.",
     inputs=_mkt() + [_i("tax", "tax per unit", "t", "per-unit tax", "excise tax", "unit tax")],
     outputs=[_o("quantity", "quantity with the tax"), _o("buyer_price", "price paid by buyers"), _o("seller_price", "buyer price - tax"),
              _o("tax_revenue", "t Q"), _o("deadweight_loss", "t (Q0 - Qt) / 2"), _o("buyer_share", "share of the tax borne by buyers d/(b+d)")],
     primary="deadweight_loss", conditions=["b, d > 0", "tax > 0", "positive quantity and seller price with the tax"],
     method="a - b Pb = c + d (Pb - t)", latex=r"P_b = \frac{a - c + d\,t}{b + d},\quad DWL = \tfrac12\,t\,(Q_0 - Q_t)", run=per_unit_tax,
     reference=_ref_tax,
     wrong=lambda demand_intercept, demand_slope, supply_intercept, supply_slope, tax: (lambda o: dict(o, deadweight_loss=2 * o["deadweight_loss"]))(
         per_unit_tax(demand_intercept, demand_slope, supply_intercept, supply_slope, tax)),
     rand=lambda rng: (lambda m: dict(m, tax=round((m["demand_intercept"] - m["supply_intercept"]) / (m["demand_slope"] + m["supply_slope"])
                                                   * rng.uniform(0.02, 0.4), 3)))(_rand_mkt(rng)),
     triggers=["per-unit tax", "tax incidence", "excise tax", "deadweight loss of a tax", "unit tax"],
     templates=["Find the tax incidence with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4, tax = 3."],
     examples=[{"inputs": {"demand_intercept": 100, "demand_slope": 2, "supply_intercept": -20, "supply_slope": 4, "tax": 3},
                "outputs": {"quantity": 56.0, "buyer_price": 22.0, "seller_price": 19.0, "tax_revenue": 168.0, "deadweight_loss": 6.0}}],
     questions=[{"q": "Find the tax incidence with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4, tax = 3.",
                 "outputs": {"quantity": 56.0, "deadweight_loss": 6.0}},
                {"q": "Find the tax incidence with a = 100, b = 2, c = -20, d = 4, tax = 50.", "outcome": "INVALID_INPUT"}])


_INV_IN = [_i("price_intercept", "A in the inverse demand P = A - B Q", "a", "demand intercept", "choke price"),
           _i("price_slope", "B in P = A - B Q (B > 0)", "b", "demand slope", "slope")]


def _inv():
    return [dict(i, _aliases=list(i["_aliases"])) for i in _INV_IN]


def monopoly(price_intercept, price_slope, marginal_cost, fixed_cost=0.0, cost_quadratic=0.0):
    A, B = _pos("price_intercept", price_intercept), _pos("price_slope", price_slope)
    c, F, e = _nonneg("marginal_cost", marginal_cost), _nonneg("fixed_cost", fixed_cost), _nonneg("cost_quadratic", cost_quadratic)
    _need(A > c, "the demand intercept must exceed marginal cost (otherwise nothing is produced)")
    Q = (A - c) / (2 * B + 2 * e)
    P = A - B * Q
    mc = c + 2 * e * Q
    Qc = (A - c) / (B + 2 * e)
    return {"quantity": Q, "price": P, "profit": P * Q - (F + c * Q + e * Q * Q), "lerner_index": (P - mc) / P,
            "deadweight_loss": (B + 2 * e) * (Qc - Q) ** 2 / 2}


def _ref_monopoly(price_intercept, price_slope, marginal_cost, fixed_cost=0.0, cost_quadratic=0.0):
    import sympy as sp                                     # maximise profit symbolically; DWL by integration
    R = lambda v: sp.Rational(Fraction(v).numerator, Fraction(v).denominator)
    A, B, c, F, e = (R(v) for v in (price_intercept, price_slope, marginal_cost, fixed_cost, cost_quadratic))
    q = sp.Symbol("q", positive=True)
    profit = (A - B * q) * q - (F + c * q + e * q ** 2)
    Q = sp.solve(sp.diff(profit, q), q)[0]
    Qc = sp.solve(sp.Eq(A - B * q, c + 2 * e * q), q)[0]
    P = A - B * Q
    return {"quantity": float(Q), "price": float(P), "profit": float(profit.subs(q, Q)),
            "lerner_index": float((P - (c + 2 * e * Q)) / P),
            "deadweight_loss": float(sp.integrate(A - B * q - c - 2 * e * q, (q, Q, Qc)))}


_add("monopoly", name="Monopoly profit maximization", topic="microeconomics / market structure",
     purpose="Profit-maximising quantity and price of a monopoly facing P = A - BQ with cost F + cQ + eQ^2, its profit, "
             "Lerner index and the deadweight loss against marginal-cost pricing.",
     inputs=_inv() + [_i("marginal_cost", "c, the constant part of marginal cost", "mc", "c", "unit cost", "variable cost"),
                      _i("fixed_cost", "fixed cost F (default 0)", "f", "fixed costs", required=False),
                      _i("cost_quadratic", "e in the cost F + cQ + eQ^2 (default 0)", "e", "quadratic cost", required=False)],
     outputs=[_o("quantity", "Q* where MR = MC"), _o("price", "A - B Q*"), _o("profit", "P Q - cost"), _o("lerner_index", "(P - MC)/P"),
              _o("deadweight_loss", "loss against P = MC")], primary="quantity",
     conditions=["A > c >= 0", "B > 0", "F, e >= 0"], method="MR = A - 2BQ = MC = c + 2eQ",
     latex=r"Q^{*} = \frac{A - c}{2(B + e)},\quad P^{*} = A - BQ^{*}", run=monopoly, reference=_ref_monopoly,
     wrong=lambda price_intercept, price_slope, marginal_cost, fixed_cost=0.0, cost_quadratic=0.0: (lambda o: dict(
         o, quantity=(price_intercept - marginal_cost) / (price_slope + 2 * cost_quadratic)))(
         monopoly(price_intercept, price_slope, marginal_cost, fixed_cost, cost_quadratic)),
     rand=lambda rng: (lambda A: {"price_intercept": A, "price_slope": _u(rng, 0.1, 10, 3), "marginal_cost": round(A * rng.uniform(0, 0.8), 2),
                                  "fixed_cost": rng.choice([0.0, _u(rng, 10, 5000, 1)]), "cost_quadratic": rng.choice([0.0, _u(rng, 0.01, 2, 3)])})(
         _u(rng, 20, 500, 1)),
     triggers=["monopoly", "monopolist", "profit-maximizing monopoly", "monopoly profit"],
     templates=["Find the monopoly price and quantity with price intercept = 100, price slope = 2, marginal cost = 20."],
     examples=[{"inputs": {"price_intercept": 100, "price_slope": 2, "marginal_cost": 20},
                "outputs": {"quantity": 20.0, "price": 60.0, "profit": 800.0, "deadweight_loss": 400.0}},
               {"inputs": {"price_intercept": 100, "price_slope": 1, "marginal_cost": 10, "fixed_cost": 200, "cost_quadratic": 0.5}, "outputs": {'quantity': 30.0, 'price': 70.0, 'profit': 1150.0, 'lerner_index': 0.42857142857142855, 'deadweight_loss': 225.0}}],
     questions=[{"q": "Find the monopoly price and quantity with price intercept = 100, price slope = 2, marginal cost = 20.",
                 "outputs": {"quantity": 20.0, "price": 60.0}},
                {"q": "Find the monopoly price and quantity with a = 100, b = 2, mc = 120.", "outcome": "INVALID_INPUT"}])


def cournot(price_intercept, price_slope, cost_1, cost_2):
    A, B = _pos("price_intercept", price_intercept), _pos("price_slope", price_slope)
    c1, c2 = _nonneg("cost_1", cost_1), _nonneg("cost_2", cost_2)
    q1, q2 = (A - 2 * c1 + c2) / (3 * B), (A - 2 * c2 + c1) / (3 * B)
    _need(q1 > 0 and q2 > 0, "one firm would not produce: the costs are too different for an interior Cournot equilibrium")
    P = A - B * (q1 + q2)
    return {"q1": q1, "q2": q2, "price": P, "profit_1": (P - c1) * q1, "profit_2": (P - c2) * q2}


def _ref_duopoly(A, B, c1, c2, leader):
    import sympy as sp
    R = lambda v: sp.Rational(Fraction(v).numerator, Fraction(v).denominator)
    A, B, c1, c2 = R(A), R(B), R(c1), R(c2)
    q1, q2 = sp.symbols("q1 q2")
    P = A - B * (q1 + q2)
    if leader:                                             # follower's best response, then the leader optimises against it
        br = sp.solve(sp.diff((P - c2) * q2, q2), q2)[0]
        s1 = sp.solve(sp.diff(((P - c1) * q1).subs(q2, br), q1), q1)[0]
        sol = {q1: s1, q2: br.subs(q1, s1)}
    else:
        sol = sp.solve([sp.diff((P - c1) * q1, q1), sp.diff((P - c2) * q2, q2)], [q1, q2])
    p = P.subs(sol)
    return float(sol[q1]), float(sol[q2]), float(p), float((p - c1) * sol[q1]), float((p - c2) * sol[q2])


def _rand_duo(rng):
    A = _u(rng, 50, 500, 1)
    return {"price_intercept": A, "price_slope": _u(rng, 0.1, 5, 3), "c1": round(A * rng.uniform(0, 0.3), 2), "c2": round(A * rng.uniform(0, 0.3), 2)}


_add("cournot", name="Cournot duopoly equilibrium", topic="microeconomics / oligopoly",
     purpose="Quantities, price and profits of two firms choosing quantities simultaneously against P = A - B(q1 + q2) "
             "with constant marginal costs.",
     inputs=_inv() + [_i("cost_1", "marginal cost of firm 1", "c1", "mc1", "cost of firm 1"), _i("cost_2", "marginal cost of firm 2", "c2", "mc2", "cost of firm 2")],
     outputs=[_o("q1", "(A - 2c1 + c2)/(3B)"), _o("q2", "(A - 2c2 + c1)/(3B)"), _o("price", "A - B(q1 + q2)"),
              _o("profit_1", "(P - c1) q1"), _o("profit_2", "(P - c2) q2")], primary="q1",
     conditions=["B > 0", "both equilibrium quantities positive"], method="intersection of the two best-response functions",
     latex=r"q_i^{*} = \frac{A - 2c_i + c_j}{3B}", run=cournot,
     reference=lambda price_intercept, price_slope, cost_1, cost_2: dict(zip(("q1", "q2", "price", "profit_1", "profit_2"),
                                                                           _ref_duopoly(price_intercept, price_slope, cost_1, cost_2, False))),
     wrong=lambda price_intercept, price_slope, cost_1, cost_2: (lambda o: dict(o, q1=(price_intercept - cost_1) / (2 * price_slope)))(
         cournot(price_intercept, price_slope, cost_1, cost_2)),
     rand=lambda rng: (lambda x: {"price_intercept": x["price_intercept"], "price_slope": x["price_slope"], "cost_1": x["c1"], "cost_2": x["c2"]})(_rand_duo(rng)),
     triggers=["cournot", "cournot duopoly", "cournot equilibrium", "cournot competition"],
     templates=["Find the Cournot equilibrium with price intercept = 100, price slope = 1, c1 = 10, c2 = 10."],
     examples=[{"inputs": {"price_intercept": 100, "price_slope": 1, "cost_1": 10, "cost_2": 10}, "outputs": {"q1": 30.0, "q2": 30.0, "price": 40.0, "profit_1": 900.0}}],
     questions=[{"q": "Find the Cournot equilibrium with price intercept = 100, price slope = 1, c1 = 10, c2 = 10.", "outputs": {"q1": 30.0, "price": 40.0}},
                {"q": "Find the Cournot equilibrium with a = 100, b = 1, c1 = 10, c2 = 90.", "outcome": "INVALID_INPUT"}])


def stackelberg(price_intercept, price_slope, cost_leader, cost_follower):
    A, B = _pos("price_intercept", price_intercept), _pos("price_slope", price_slope)
    cl, cf = _nonneg("cost_leader", cost_leader), _nonneg("cost_follower", cost_follower)
    ql = (A - 2 * cl + cf) / (2 * B)
    qf = (A - cf - B * ql) / (2 * B)
    _need(ql > 0 and qf > 0, "one firm would not produce: no interior Stackelberg equilibrium")
    P = A - B * (ql + qf)
    return {"q_leader": ql, "q_follower": qf, "price": P, "profit_leader": (P - cl) * ql, "profit_follower": (P - cf) * qf}


_add("stackelberg", name="Stackelberg duopoly equilibrium", topic="microeconomics / oligopoly",
     purpose="Quantities, price and profits when a leader chooses its quantity first and the follower best-responds, "
             "against P = A - B(qL + qF) with constant marginal costs.",
     inputs=_inv() + [_i("cost_leader", "marginal cost of the leader", "cl", "c1", "leader cost", "mc leader"),
                      _i("cost_follower", "marginal cost of the follower", "cf", "c2", "follower cost", "mc follower")],
     outputs=[_o("q_leader", "(A - 2cL + cF)/(2B)"), _o("q_follower", "(A - cF - B qL)/(2B)"), _o("price", "market price"),
              _o("profit_leader", "leader's profit"), _o("profit_follower", "follower's profit")], primary="q_leader",
     conditions=["B > 0", "both quantities positive"], method="backward induction: follower's best response, then the leader's optimum",
     latex=r"q_L^{*} = \frac{A - 2c_L + c_F}{2B},\quad q_F^{*} = \frac{A - c_F - Bq_L^{*}}{2B}", run=stackelberg,
     reference=lambda price_intercept, price_slope, cost_leader, cost_follower: dict(zip(
         ("q_leader", "q_follower", "price", "profit_leader", "profit_follower"), _ref_duopoly(price_intercept, price_slope, cost_leader, cost_follower, True))),
     wrong=lambda price_intercept, price_slope, cost_leader, cost_follower: (lambda o: dict(
         o, q_leader=(price_intercept - 2 * cost_leader + cost_follower) / (3 * price_slope)))(stackelberg(price_intercept, price_slope, cost_leader, cost_follower)),
     rand=lambda rng: (lambda x: {"price_intercept": x["price_intercept"], "price_slope": x["price_slope"], "cost_leader": x["c1"], "cost_follower": x["c2"]})(
         _rand_duo(rng)),
     triggers=["stackelberg", "stackelberg duopoly", "stackelberg equilibrium", "stackelberg leader"],
     templates=["Find the Stackelberg equilibrium with price intercept = 100, price slope = 1, cost leader = 10, cost follower = 10."],
     examples=[{"inputs": {"price_intercept": 100, "price_slope": 1, "cost_leader": 10, "cost_follower": 10},
                "outputs": {"q_leader": 45.0, "q_follower": 22.5, "price": 32.5}}],
     questions=[{"q": "Find the Stackelberg equilibrium with price intercept = 100, price slope = 1, cost leader = 10, cost follower = 10.",
                 "outputs": {"q_leader": 45.0, "q_follower": 22.5}}])


def break_even(fixed_costs, price, variable_cost, target_profit=0.0):
    F, p, v = _nonneg("fixed_costs", fixed_costs), _pos("price", price), _nonneg("variable_cost", variable_cost)
    T = _fin("target_profit", target_profit)
    _need(p > v, "the price must exceed the variable cost per unit (no positive contribution margin)")
    _need(F + T >= 0, "fixed costs plus the target profit must not be negative")
    Q = (F + T) / (p - v)
    return {"quantity": Q, "revenue": p * Q, "contribution_margin": p - v, "contribution_margin_ratio": (p - v) / p}


_add("break_even", name="Break-even and target-profit volume", topic="microeconomics / cost-volume-profit",
     purpose="Units to sell to cover fixed costs (and reach a target profit): Q = (F + target) / (p - v).",
     inputs=[_i("fixed_costs", "total fixed costs", "f", "fixed cost", "fc"), _i("price", "selling price per unit", "p", "unit price", "selling price"),
             _i("variable_cost", "variable cost per unit", "v", "vc", "unit variable cost", "variable cost per unit"),
             _i("target_profit", "target profit (default 0: break-even)", "profit target", "desired profit", required=False)],
     outputs=[_o("quantity", "units (F + target)/(p - v)"), _o("revenue", "p Q"), _o("contribution_margin", "p - v per unit"),
              _o("contribution_margin_ratio", "(p - v)/p")], primary="quantity", conditions=["price > variable cost", "fixed costs >= 0"],
     method="Q = (F + target) / (p - v)", latex=r"Q_{BE} = \frac{F + \pi_{target}}{p - v}", run=break_even,
     reference=None,
     wrong=lambda fixed_costs, price, variable_cost, target_profit=0.0: (lambda o: dict(o, quantity=(fixed_costs + target_profit) / price))(
         break_even(fixed_costs, price, variable_cost, target_profit)),
     rand=lambda rng: (lambda p: {"fixed_costs": _u(rng, 100, 1e6, 1), "price": p, "variable_cost": round(p * rng.uniform(0, 0.9), 2),
                                  "target_profit": rng.choice([0.0, _u(rng, 100, 1e5, 1)])})(_u(rng, 1, 500, 2)),
     triggers=["break-even", "break even", "breakeven", "target profit"],
     templates=["Find the break-even quantity with fixed costs = 10000, price = 50, variable cost = 30."],
     examples=[{"inputs": {"fixed_costs": 10000, "price": 50, "variable_cost": 30}, "outputs": {"quantity": 500.0, "revenue": 25000.0}},
               {"inputs": {"fixed_costs": 10000, "price": 50, "variable_cost": 30, "target_profit": 4000}, "outputs": {"quantity": 700.0}}],
     questions=[{"q": "Find the break-even quantity with fixed costs = 10000, price = 50, variable cost = 30.", "outputs": {"quantity": 500.0}},
                {"q": "Find the break-even quantity with fixed costs = 10000, price = 30, variable cost = 50.", "outcome": "INVALID_INPUT"}])
PROCEDURES["break_even"]["reference"] = lambda fixed_costs, price, variable_cost, target_profit=0.0: {
    "quantity": __import__("scipy.optimize", fromlist=["brentq"]).brentq(
        lambda q: (price - variable_cost) * q - fixed_costs - target_profit, 0.0, 2 * (fixed_costs + target_profit) / (price - variable_cost) + 1,
        xtol=1e-14, rtol=1e-15, maxiter=1000)}


def cobb_douglas_demand(alpha, beta, income, price_x, price_y):
    a, b, M = _pos("alpha", alpha), _pos("beta", beta), _pos("income", income)
    px, py = _pos("price_x", price_x), _pos("price_y", price_y)
    x, y = a / (a + b) * M / px, b / (a + b) * M / py
    return {"x": x, "y": y, "utility": x ** a * y ** b, "share_x": a / (a + b)}


def _ref_cd(alpha, beta, income, price_x, price_y):
    import sympy as sp                                     # maximise a ln x + b ln((M - px x)/py) symbolically
    R = lambda v: sp.Rational(Fraction(v).numerator, Fraction(v).denominator)
    a, b, M, px, py = (R(v) for v in (alpha, beta, income, price_x, price_y))
    x = sp.Symbol("x", positive=True)
    xs = sp.solve(sp.diff(a * sp.log(x) + b * sp.log((M - px * x) / py), x), x)[0]
    ys = (M - px * xs) / py
    return {"x": float(xs), "y": float(ys)}


_add("cobb_douglas_demand", name="Utility-maximising demand (Cobb-Douglas)", topic="microeconomics / consumer choice",
     purpose="The bundle that maximises U = x^alpha y^beta subject to px x + py y = M.",
     inputs=[_i("alpha", "exponent of good x", "a"), _i("beta", "exponent of good y", "b"),
             _i("income", "income (budget)", "m", "budget", "i"), _i("price_x", "price of good x", "px", "price of x"),
             _i("price_y", "price of good y", "py", "price of y")],
     outputs=[_o("x", "alpha/(alpha+beta) M/px"), _o("y", "beta/(alpha+beta) M/py"), _o("utility", "x^alpha y^beta"),
              _o("share_x", "budget share of x")], primary="x", conditions=["alpha, beta, income, prices > 0"],
     method="tangency MRS = px/py with the budget line", latex=r"x^{*} = \frac{\alpha}{\alpha+\beta}\frac{M}{p_x},\quad y^{*} = \frac{\beta}{\alpha+\beta}\frac{M}{p_y}",
     run=cobb_douglas_demand, reference=_ref_cd,
     wrong=lambda alpha, beta, income, price_x, price_y: (lambda o: dict(o, x=alpha * income / price_x))(
         cobb_douglas_demand(alpha, beta, income, price_x, price_y)),
     rand=lambda rng: {"alpha": _u(rng, 0.1, 2, 3), "beta": _u(rng, 0.1, 2, 3), "income": _u(rng, 100, 1e5, 1),
                       "price_x": _u(rng, 0.5, 100, 2), "price_y": _u(rng, 0.5, 100, 2)},
     triggers=["cobb-douglas", "cobb douglas", "utility maximization", "utility-maximizing bundle", "optimal bundle"],
     templates=["Find the Cobb-Douglas optimal bundle with alpha = 0.3, beta = 0.7, income = 1000, px = 10, py = 20."],
     examples=[{"inputs": {"alpha": 0.3, "beta": 0.7, "income": 1000, "price_x": 10, "price_y": 20}, "outputs": {"x": 30.0, "y": 35.0}}],
     questions=[{"q": "Find the Cobb-Douglas optimal bundle with alpha = 0.3, beta = 0.7, income = 1000, px = 10, py = 20.", "outputs": {"x": 30.0, "y": 35.0}},
                {"q": "Find the Cobb-Douglas optimal bundle with alpha = 0.3, beta = 0.7, income = 1000, px = 0, py = 20.", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# MACROECONOMICS
# =====================================================================================================================
def _baskets(p0, q0, p1, q1):
    p0, q0, p1, q1 = (_vec(n, v, 1) for n, v in (("base_prices", p0), ("base_quantities", q0), ("current_prices", p1), ("current_quantities", q1)))
    _need(len(p0) == len(q0) == len(p1) == len(q1), "one price and quantity per good in each year")
    _need(all(x > 0 for x in p0 + p1) and all(x >= 0 for x in q0 + q1), "prices must be positive and quantities non-negative")
    _need(sum(q0) > 0 and sum(q1) > 0, "each year needs a positive quantity")
    return p0, q0, p1, q1


def _dot(a, b):
    return math.fsum(x * y for x, y in zip(a, b))


_BASK_IN = [_i("base_prices", "prices in the base year", "p0", "base year prices", type="vector", min_length=1),
            _i("base_quantities", "quantities in the base year", "q0", "base year quantities", type="vector", min_length=1),
            _i("current_prices", "prices in the current year", "p1", "current year prices", type="vector", min_length=1),
            _i("current_quantities", "quantities in the current year", "q1", "current year quantities", type="vector", min_length=1)]


def _bask():
    return [dict(i, _aliases=list(i["_aliases"])) for i in _BASK_IN]


def _rand_bask(rng):
    n = rng.randint(2, 8)
    p0 = [_u(rng, 0.5, 100, 2) for _ in range(n)]
    return {"base_prices": p0, "base_quantities": [_u(rng, 1, 1000, 1) for _ in range(n)],
            "current_prices": [round(p * rng.uniform(0.8, 1.4), 2) for p in p0], "current_quantities": [_u(rng, 1, 1000, 1) for _ in range(n)]}


def gdp_deflator(base_prices, base_quantities, current_prices, current_quantities):
    p0, q0, p1, q1 = _baskets(base_prices, base_quantities, current_prices, current_quantities)
    nom, real, real0 = _dot(p1, q1), _dot(p0, q1), _dot(p0, q0)
    d = 100 * nom / real
    return {"nominal_gdp": nom, "real_gdp": real, "deflator": d, "inflation": d / 100 - 1, "real_growth": real / real0 - 1}


def _ref_deflator(base_prices, base_quantities, current_prices, current_quantities):
    import numpy as np                                     # deflator as the harmonic mean of price relatives (current-value weights)
    p0, q0, p1, q1 = (np.asarray(v, dtype=float) for v in (base_prices, base_quantities, current_prices, current_quantities))
    w = p1 * q1 / np.sum(p1 * q1)
    d = 100 / np.sum(w * p0 / p1)
    nom = float(np.sum(p1 * q1))
    return {"deflator": float(d), "nominal_gdp": nom, "real_gdp": nom / float(d) * 100, "real_growth": nom / float(d) * 100 / float(np.sum(p0 * q0)) - 1}


_add("gdp_deflator", name="Nominal GDP, real GDP and the GDP deflator", topic="macroeconomics / national accounts",
     purpose="Nominal GDP (current prices), real GDP (base-year prices), the GDP deflator, inflation since the base year "
             "and real growth, from the prices and quantities of the goods produced.",
     inputs=_bask(),
     outputs=[_o("nominal_gdp", "sum p1 q1"), _o("real_gdp", "sum p0 q1"), _o("deflator", "100 x nominal / real"),
              _o("inflation", "deflator/100 - 1 (since the base year)"), _o("real_growth", "real GDP / base-year GDP - 1")], primary="deflator",
     conditions=["one price and quantity per good", "prices > 0"], method="deflator = 100 sum p1 q1 / sum p0 q1",
     latex=r"\text{Deflator} = 100\,\frac{\sum p_1 q_1}{\sum p_0 q_1}", run=gdp_deflator, reference=_ref_deflator,
     wrong=lambda base_prices, base_quantities, current_prices, current_quantities: (lambda o: dict(
         o, deflator=100 * _dot(current_prices, base_quantities) / _dot(base_prices, base_quantities)))(
         gdp_deflator(base_prices, base_quantities, current_prices, current_quantities)),
     rand=_rand_bask,
     triggers=["gdp deflator", "real gdp", "nominal gdp"],
     templates=["Find the GDP deflator with base prices = [2, 5], base quantities = [100, 50], current prices = [3, 6], current quantities = [120, 60]."],
     examples=[{"inputs": {"base_prices": "2, 5", "base_quantities": "100, 50", "current_prices": "3, 6", "current_quantities": "120, 60"},
                "outputs": {"nominal_gdp": 720.0, "real_gdp": 540.0}}],
     questions=[{"q": "Find the GDP deflator with base prices = [2, 5], base quantities = [100, 50], current prices = [3, 6], current quantities = [120, 60].",
                 "outputs": {"nominal_gdp": 720.0, "real_gdp": 540.0}},
                {"q": "Find the GDP deflator with base prices = [2, 5], base quantities = [100, 50], current prices = [3], current quantities = [120, 60].",
                 "outcome": "INVALID_INPUT"}])


def price_indices(base_prices, base_quantities, current_prices, current_quantities):
    p0, q0, p1, q1 = _baskets(base_prices, base_quantities, current_prices, current_quantities)
    L, P = 100 * _dot(p1, q0) / _dot(p0, q0), 100 * _dot(p1, q1) / _dot(p0, q1)
    return {"laspeyres": L, "paasche": P, "fisher": math.sqrt(L * P)}


def _ref_indices(base_prices, base_quantities, current_prices, current_quantities):
    import numpy as np                                     # weighted means of price relatives
    p0, q0, p1, q1 = (np.asarray(v, dtype=float) for v in (base_prices, base_quantities, current_prices, current_quantities))
    L = 100 * np.sum(p0 * q0 / np.sum(p0 * q0) * (p1 / p0))
    P = 100 / np.sum(p1 * q1 / np.sum(p1 * q1) * (p0 / p1))
    return {"laspeyres": float(L), "paasche": float(P), "fisher": float(np.sqrt(L * P))}


_add("price_indices", name="Laspeyres, Paasche and Fisher price indices", topic="macroeconomics / price indices",
     purpose="Price indices (base = 100) of a basket: Laspeyres (base-year quantities, the CPI method), Paasche "
             "(current quantities) and Fisher (their geometric mean).",
     inputs=_bask(),
     outputs=[_o("laspeyres", "100 sum p1 q0 / sum p0 q0"), _o("paasche", "100 sum p1 q1 / sum p0 q1"), _o("fisher", "sqrt(L P)")],
     primary="laspeyres", conditions=["one price and quantity per good", "prices > 0"], method="ratios of basket costs",
     latex=r"L = 100\frac{\sum p_1 q_0}{\sum p_0 q_0},\quad P = 100\frac{\sum p_1 q_1}{\sum p_0 q_1},\quad F = \sqrt{LP}", run=price_indices,
     reference=_ref_indices,
     wrong=lambda base_prices, base_quantities, current_prices, current_quantities: (lambda o: dict(
         o, laspeyres=100 * _dot(current_prices, base_quantities) / _dot(base_prices, current_quantities)))(
         price_indices(base_prices, base_quantities, current_prices, current_quantities)),
     rand=_rand_bask,
     triggers=["laspeyres", "paasche", "fisher index", "fisher price index", "price index", "consumer price index", "cpi"],
     templates=["Find the Laspeyres and Paasche price index with base prices = [2, 5], base quantities = [100, 50], current prices = [3, 6], current quantities = [120, 60]."],
     examples=[{"inputs": {"base_prices": "2, 5", "base_quantities": "100, 50", "current_prices": "3, 6", "current_quantities": "150, 40"},
                "outputs": {"laspeyres": 133.33333333333334, "paasche": 138.0}}],
     questions=[{"q": "Find the Laspeyres and Paasche price index with base prices = [2, 5], base quantities = [100, 50], current prices = [3, 6], current quantities = [150, 40].",
                 "outputs": {"laspeyres": 133.33333333333334, "paasche": 138.0}}])


def fisher_real_rate(nominal_rate, inflation_rate):
    i, pi = _rate("nominal_rate", nominal_rate), _rate("inflation_rate", inflation_rate)
    return {"real_rate": (1 + i) / (1 + pi) - 1, "approximate_real_rate": i - pi}


_add("fisher_real_rate", name="Real interest rate (Fisher equation)", topic="macroeconomics / interest and inflation",
     purpose="The exact real rate (1 + i)/(1 + pi) - 1 and the approximation i - pi.",
     inputs=[_i("nominal_rate", "nominal interest rate (decimal)", "i", "nominal interest rate", "nominal"),
             _i("inflation_rate", "inflation rate (decimal)", "inflation", "pi", "expected inflation")],
     outputs=[_o("real_rate", "(1 + i)/(1 + pi) - 1"), _o("approximate_real_rate", "i - pi")], primary="real_rate",
     conditions=["-1 < rates < 1"], method="(1 + i) = (1 + r)(1 + pi)", latex=r"1 + i = (1 + r)(1 + \pi)", run=fisher_real_rate,
     reference=lambda nominal_rate, inflation_rate: {"real_rate": __import__("scipy.optimize", fromlist=["brentq"]).brentq(
         lambda r: (1 + r) * (1 + inflation_rate) - (1 + nominal_rate), -0.999, 100, xtol=1e-16, rtol=1e-15, maxiter=500)},
     wrong=lambda nominal_rate, inflation_rate: {"real_rate": (1 + nominal_rate) * (1 + inflation_rate) - 1, "approximate_real_rate": nominal_rate - inflation_rate},
     rand=lambda rng: {"nominal_rate": _u(rng, 0.0, 0.2), "inflation_rate": _u(rng, -0.02, 0.15)},
     triggers=["real interest rate", "fisher equation", "real rate", "fisher effect"],
     templates=["Find the real interest rate with nominal rate = 8%, inflation = 3%."],
     examples=[{"inputs": {"nominal_rate": 0.08, "inflation_rate": 0.03}, "outputs": {'real_rate': 0.04854368932038833, 'approximate_real_rate': 0.05}},
               {"inputs": {"nominal_rate": 0.1, "inflation_rate": 0.1}, "outputs": {"real_rate": 0.0}}],
     questions=[{"q": "Find the real interest rate with nominal rate = 8%, inflation = 3%.", "outputs": {'real_rate': 0.04854368932038833, 'approximate_real_rate': 0.05}},
                {"q": "Find the real interest rate with nominal rate = 8, inflation = 3.", "outcome": "INVALID_INPUT"}])


def inflation_adjustment(price_levels, amount=None):
    p = _vec("price_levels", price_levels, 2)
    _need(all(x > 0 for x in p), "price levels must be positive")
    n = len(p) - 1
    out = {"total_inflation": p[-1] / p[0] - 1, "average_inflation": (p[-1] / p[0]) ** (1 / n) - 1}
    if amount is not None:
        out["real_value"] = _fin("amount", amount) * p[0] / p[-1]
    return out


def _ref_inflation(price_levels, amount=None):
    import numpy as np
    from scipy.stats import gmean
    rel = np.asarray(price_levels[1:]) / np.asarray(price_levels[:-1])
    out = {"total_inflation": float(np.prod(rel)) - 1, "average_inflation": float(gmean(rel)) - 1}
    if amount is not None:
        out["real_value"] = amount / float(np.prod(rel))
    return out


_add("inflation_adjustment", name="Inflation from a price index series", topic="macroeconomics / interest and inflation",
     purpose="Total and average annual (geometric) inflation from consecutive price levels (CPI), and the value of an "
             "amount in the last period's money expressed in first-period money.",
     inputs=[_i("price_levels", "price index levels in consecutive periods (e.g. CPI each year)", "cpi", "price index", "index levels",
                "cpi values", type="vector", min_length=2),
             _i("amount", "an amount in last-period money to deflate (optional)", "nominal amount", "value", required=False)],
     outputs=[_o("total_inflation", "P_last / P_first - 1"), _o("average_inflation", "(P_last/P_first)^(1/n) - 1"),
              _o("real_value", "amount x P_first / P_last")], primary="average_inflation", conditions=["price levels > 0"],
     method="geometric mean of the period-to-period price relatives", latex=r"\bar\pi = \left(\frac{P_n}{P_0}\right)^{1/n} - 1",
     run=inflation_adjustment, reference=_ref_inflation,
     wrong=lambda price_levels, amount=None: (lambda o: dict(o, average_inflation=sum(b / a - 1 for a, b in zip(price_levels, price_levels[1:])) / (len(price_levels) - 1)))(
         inflation_adjustment(price_levels, amount)),
     rand=lambda rng: {"price_levels": (lambda n: [round(100 * math.prod(1 + rng.uniform(-0.02, 0.12) for _ in range(k)), 3) for k in range(n)])(
         rng.randint(3, 15)), "amount": _u(rng, 100, 1e5, 2)},
     triggers=["average inflation", "inflation rate from", "inflation from the cpi", "inflation adjustment", "adjust for inflation", "cumulative inflation"],
     templates=["Find the average inflation with cpi = [100, 103, 107.12], amount = 1000."],
     examples=[{"inputs": {"price_levels": "100, 103, 107.12", "amount": 1000}, "outputs": {'total_inflation': 0.07120000000000015, 'average_inflation': 0.03498792263484907, 'real_value': 933.5324869305451}},
               {"inputs": {"price_levels": [100, 121]}, "outputs": {"average_inflation": 0.21, "total_inflation": 0.21}}],
     questions=[{"q": "Find the average inflation with cpi = [100, 103, 107.12], amount = 1000.", "outputs": {'total_inflation': 0.07120000000000015, 'average_inflation': 0.03498792263484907, 'real_value': 933.5324869305451}},
                {"q": "Find the average inflation with cpi = [100].", "outcome": "INVALID_INPUT"}])


def money_multiplier(reserve_ratio, currency_ratio=0.0, monetary_base=None):
    rr, c = _prob("reserve_ratio", reserve_ratio), _nonneg("currency_ratio", currency_ratio)
    _need(rr > 0, "the reserve ratio must be positive")
    m = (1 + c) / (rr + c)
    out = {"simple_multiplier": 1 / rr, "money_multiplier": m}
    if monetary_base is not None:
        out["money_supply"] = m * _pos("monetary_base", monetary_base)
    return out


def _ref_money(reserve_ratio, currency_ratio=0.0, monetary_base=None):
    import mpmath
    import numpy as np                                     # M = C + D, B = C + R, R = rr D, C = c D, solved with B = 1
    rr, c = reserve_ratio, currency_ratio
    K = np.array([[1, -1, -1, 0], [0, 1, 0, 1], [0, 0, -rr, 1], [0, 1, -c, 0]], dtype=float)
    M = float(np.linalg.solve(K, np.array([0.0, 1.0, 0.0, 0.0]))[0])
    out = {"money_multiplier": M, "simple_multiplier": float(mpmath.nsum(lambda k: (1 - mpmath.mpf(rr)) ** k, [0, mpmath.inf]))}
    if monetary_base is not None:
        out["money_supply"] = M * monetary_base
    return out


_add("money_multiplier", name="Money multiplier", topic="macroeconomics / money and banking",
     purpose="Simple deposit multiplier 1/rr and the money multiplier (1 + c)/(rr + c) with a currency-deposit ratio c, "
             "and the money supply from the monetary base.",
     inputs=[_i("reserve_ratio", "reserve ratio (decimal)", "rr", "required reserve ratio", "reserve requirement"),
             _i("currency_ratio", "currency-to-deposit ratio (default 0)", "c", "currency deposit ratio", "currency-deposit ratio", required=False),
             _i("monetary_base", "monetary base (optional)", "base", "b", "high-powered money", required=False)],
     outputs=[_o("simple_multiplier", "1/rr"), _o("money_multiplier", "(1 + c)/(rr + c)"), _o("money_supply", "multiplier x base")],
     primary="money_multiplier", conditions=["0 < reserve_ratio <= 1", "currency_ratio >= 0"],
     method="m = (1 + c)/(rr + c)", latex=r"m = \frac{1 + c}{rr + c}", run=money_multiplier, reference=_ref_money,
     wrong=lambda reserve_ratio, currency_ratio=0.0, monetary_base=None: (lambda o: dict(o, money_multiplier=1 / (reserve_ratio + currency_ratio)))(
         money_multiplier(reserve_ratio, currency_ratio, monetary_base)),
     rand=lambda rng: {"reserve_ratio": _u(rng, 0.02, 0.3), "currency_ratio": _u(rng, 0.05, 0.5), "monetary_base": _u(rng, 100, 1e6, 1)},
     triggers=["money multiplier", "deposit multiplier", "money supply"],
     templates=["Find the money multiplier with reserve ratio = 10%, currency ratio = 20%, monetary base = 1000."],
     examples=[{"inputs": {"reserve_ratio": 0.1, "currency_ratio": 0.2, "monetary_base": 1000}, "outputs": {'simple_multiplier': 10.0, 'money_multiplier': 3.999999999999999, 'money_supply': 3999.999999999999}},
               {"inputs": {"reserve_ratio": 0.2}, "outputs": {"simple_multiplier": 5.0, "money_multiplier": 5.0}}],
     questions=[{"q": "Find the money multiplier with reserve ratio = 10%, currency ratio = 20%, monetary base = 1000.", "outputs": {'simple_multiplier': 10.0, 'money_multiplier': 3.999999999999999, 'money_supply': 3999.999999999999}},
                {"q": "Find the money multiplier with reserve ratio = 0.", "outcome": "INVALID_INPUT"}])


def spending_multiplier(mpc, tax_rate=0.0, import_propensity=0.0, spending_change=None):
    b = _fin("mpc", mpc)
    _need(0 < b < 1, "the marginal propensity to consume is between 0 and 1")
    t, m = _prob("tax_rate", tax_rate, 0, 0.99), _prob("import_propensity", import_propensity, 0, 0.99)
    rho = b * (1 - t) - m
    _need(0 <= rho < 1, "the induced-spending ratio mpc(1 - t) - m must lie in [0, 1)")
    k = 1 / (1 - rho)
    out = {"spending_multiplier": k, "tax_multiplier": -b * k}
    if spending_change is not None:
        out["output_change"] = k * _fin("spending_change", spending_change)
    return out


def _ref_spending(mpc, tax_rate=0.0, import_propensity=0.0, spending_change=None):
    import mpmath                                          # sum of the rounds of induced spending
    rho = mpmath.mpf(mpc) * (1 - mpmath.mpf(tax_rate)) - mpmath.mpf(import_propensity)
    k = float(mpmath.nsum(lambda j: rho ** j, [0, mpmath.inf]))
    out = {"spending_multiplier": k, "tax_multiplier": -mpc * k}
    if spending_change is not None:
        out["output_change"] = k * spending_change
    return out


_add("spending_multiplier", name="Keynesian spending multiplier", topic="macroeconomics / aggregate demand",
     purpose="Government-spending multiplier 1/(1 - mpc(1 - t) + m), the lump-sum tax multiplier and the change in "
             "output from a change in spending.",
     inputs=[_i("mpc", "marginal propensity to consume", "marginal propensity to consume", "b"),
             _i("tax_rate", "proportional income tax rate (default 0)", "t", "tax", required=False),
             _i("import_propensity", "marginal propensity to import (default 0)", "mpi", "m", "marginal propensity to import", required=False),
             _i("spending_change", "change in government spending (optional)", "change in spending", "delta g", "dg", "spending increase", required=False)],
     outputs=[_o("spending_multiplier", "1/(1 - mpc(1 - t) + m)"), _o("tax_multiplier", "-mpc x multiplier"),
              _o("output_change", "multiplier x spending change")], primary="spending_multiplier",
     conditions=["0 < mpc < 1", "0 <= mpc(1 - t) - m < 1"], method="k = 1/(1 - mpc(1 - t) + m)",
     latex=r"k = \frac{1}{1 - c(1 - t) + m}", run=spending_multiplier, reference=_ref_spending,
     wrong=lambda mpc, tax_rate=0.0, import_propensity=0.0, spending_change=None: (lambda o: dict(o, spending_multiplier=1 / (1 - mpc)))(
         spending_multiplier(mpc, tax_rate, import_propensity, spending_change)),
     rand=lambda rng: {"mpc": _u(rng, 0.5, 0.95), "tax_rate": _u(rng, 0.1, 0.4), "import_propensity": _u(rng, 0.0, 0.2),
                       "spending_change": _u(rng, 1, 1000, 1)},
     triggers=["spending multiplier", "keynesian multiplier", "government spending multiplier", "expenditure multiplier", "fiscal multiplier"],
     templates=["Find the spending multiplier with mpc = 0.8, tax rate = 25%, mpi = 0.1, spending change = 100."],
     examples=[{"inputs": {"mpc": 0.8, "tax_rate": 0.25, "import_propensity": 0.1, "spending_change": 100}, "outputs": {'spending_multiplier': 2.0000000000000004, 'tax_multiplier': -1.6000000000000005, 'output_change': 200.00000000000006}},
               {"inputs": {"mpc": 0.75}, "outputs": {"spending_multiplier": 4.0, "tax_multiplier": -3.0}}],
     questions=[{"q": "Find the spending multiplier with mpc = 0.8, tax rate = 25%, mpi = 0.1, spending change = 100.", "outputs": {'spending_multiplier': 2.0000000000000004, 'tax_multiplier': -1.6000000000000005, 'output_change': 200.00000000000006}},
                {"q": "Find the spending multiplier with mpc = 1.2.", "outcome": "INVALID_INPUT"}])


def solow_steady_state(saving_rate, population_growth, depreciation, alpha, technology_growth=0.0, productivity=1.0):
    s, a = _fin("saving_rate", saving_rate), _fin("alpha", alpha)
    _need(0 < s < 1 and 0 < a < 1, "saving_rate and alpha are decimals between 0 and 1")
    n, d, g = _rate("population_growth", population_growth), _rate("depreciation", depreciation, lo=0.0), _rate("technology_growth", technology_growth)
    A = _pos("productivity", productivity)
    br = n + g + d
    _need(br > 0, "population growth + technology growth + depreciation must be positive")
    k = (s * A / br) ** (1 / (1 - a))
    y = A * k ** a
    return {"capital": k, "output": y, "consumption": (1 - s) * y, "golden_rule_saving": a,
            "golden_rule_capital": (a * A / br) ** (1 / (1 - a))}


def _ref_solow(saving_rate, population_growth, depreciation, alpha, technology_growth=0.0, productivity=1.0):
    from scipy.optimize import brentq                      # roots of the steady-state and golden-rule conditions
    br, A = population_growth + technology_growth + depreciation, productivity
    k = brentq(lambda x: saving_rate * A * x ** alpha - br * x, 1e-9, 1e9, xtol=1e-15, rtol=1e-15, maxiter=2000)
    kg = brentq(lambda x: alpha * A * x ** (alpha - 1) - br, 1e-9, 1e9, xtol=1e-15, rtol=1e-15, maxiter=2000)
    return {"capital": k, "output": A * k ** alpha, "golden_rule_capital": kg}


_add("solow_steady_state", name="Solow model steady state", topic="macroeconomics / growth",
     purpose="Steady-state capital, output and consumption per effective worker in the Solow model with y = A k^alpha, "
             "and the golden-rule saving rate and capital.",
     inputs=[_i("saving_rate", "saving rate s (decimal)", "s", "savings rate", "saving"),
             _i("population_growth", "population growth n (decimal)", "n", "labor force growth", "labour force growth"),
             _i("depreciation", "depreciation rate (decimal)", "delta", "depreciation rate", "d"),
             _i("alpha", "capital share alpha in y = A k^alpha", "a", "capital share"),
             _i("technology_growth", "technology growth g (default 0)", "g", "technological progress", required=False),
             _i("productivity", "total factor productivity A (default 1)", "tfp", "technology level", required=False)],
     outputs=[_o("capital", "k* = (sA/(n + g + delta))^(1/(1 - alpha))"), _o("output", "y* = A k*^alpha"), _o("consumption", "(1 - s) y*"),
              _o("golden_rule_saving", "alpha"), _o("golden_rule_capital", "(alpha A/(n + g + delta))^(1/(1 - alpha))")], primary="capital",
     conditions=["0 < s, alpha < 1", "n + g + delta > 0"], method="s A k^alpha = (n + g + delta) k",
     latex=r"k^{*} = \left(\frac{sA}{n + g + \delta}\right)^{\frac{1}{1-\alpha}}", run=solow_steady_state, reference=_ref_solow,
     wrong=lambda saving_rate, population_growth, depreciation, alpha, technology_growth=0.0, productivity=1.0: (lambda o: dict(
         o, capital=(saving_rate * productivity / (population_growth + technology_growth + depreciation)) ** (1 / alpha)))(
         solow_steady_state(saving_rate, population_growth, depreciation, alpha, technology_growth, productivity)),
     rand=lambda rng: {"saving_rate": _u(rng, 0.05, 0.5), "population_growth": _u(rng, 0.0, 0.04), "depreciation": _u(rng, 0.02, 0.1),
                       "alpha": _u(rng, 0.2, 0.5), "technology_growth": _u(rng, 0.0, 0.03), "productivity": _u(rng, 0.5, 3)},
     triggers=["solow", "solow model", "solow steady state", "steady state capital", "solow growth model"],
     templates=["Find the Solow steady state with saving rate = 20%, population growth = 1%, depreciation = 5%, alpha = 0.3."],
     examples=[{"inputs": {"saving_rate": 0.2, "population_growth": 0.01, "depreciation": 0.05, "alpha": 0.3}, "outputs": {'capital': 5.5843115041821845, 'output': 1.6752934512546553, 'consumption': 1.3402347610037244, 'golden_rule_saving': 0.3, 'golden_rule_capital': 9.966176578193439}},
               {"inputs": {"saving_rate": 0.25, "population_growth": 0.0, "depreciation": 0.1, "alpha": 0.5}, "outputs": {"capital": 6.25, "output": 2.5}}],
     questions=[{"q": "Find the Solow steady state with saving rate = 20%, population growth = 1%, depreciation = 5%, alpha = 0.3.", "outputs": {'capital': 5.5843115041821845, 'output': 1.6752934512546553, 'consumption': 1.3402347610037244, 'golden_rule_saving': 0.3, 'golden_rule_capital': 9.966176578193439}},
                {"q": "Find the Solow steady state with s = 20%, n = 1%, delta = 5%, alpha = 1.3.", "outcome": "INVALID_INPUT"}])


# =====================================================================================================================
# ECONOMETRICS AND DISTRIBUTION
# =====================================================================================================================
def ols_simple(x, y):
    X, Y = _vec("x", x, 3), _vec("y", y, 3)
    _need(len(X) == len(Y), "x and y need the same number of observations")
    sxx, syy, sxy = _cov(X, X), _cov(Y, Y), _cov(X, Y)
    _need(sxx > 0, "x must not be constant")
    _need(syy > 0, "y must not be constant")
    b = sxy / sxx
    a = _mean(Y) - b * _mean(X)
    ssr = math.fsum((yv - a - b * xv) ** 2 for xv, yv in zip(X, Y))
    n = len(X)
    return {"slope": b, "intercept": a, "r_squared": 1 - ssr / (syy * (n - 1)),
            "slope_std_error": math.sqrt(ssr / (n - 2) / (sxx * (n - 1))), "n": n}


def _ref_ols(x, y):
    import numpy as np
    import statsmodels.api as sm
    f = sm.OLS(np.asarray(y), sm.add_constant(np.asarray(x))).fit()
    return {"intercept": float(f.params[0]), "slope": float(f.params[1]), "r_squared": float(f.rsquared), "slope_std_error": float(f.bse[1])}


def _rand_xy(rng):
    n = rng.randint(5, 50)
    b, a = rng.uniform(-5, 5), rng.uniform(-50, 50)
    x = [_u(rng, 0, 100, 3) for _ in range(n)]
    return {"x": x, "y": [round(a + b * v + rng.gauss(0, 10), 3) for v in x]}


_add("ols_simple", name="Simple linear regression (OLS)", topic="econometrics / regression",
     purpose="Ordinary least squares fit y = a + b x: slope, intercept, R^2 and the standard error of the slope.",
     inputs=[_i("x", "explanatory variable values", "x values", "regressor", "independent variable", type="vector", min_length=3),
             _i("y", "dependent variable values", "y values", "dependent variable", "response", type="vector", min_length=3)],
     outputs=[_o("slope", "b = S_xy / S_xx"), _o("intercept", "a = mean y - b mean x"), _o("r_squared", "1 - SSR/SST"),
              _o("slope_std_error", "sqrt(SSR/(n - 2) / S_xx)"), _o("n", "observations")], primary="slope",
     conditions=["same number of x and y, at least 3", "neither variable constant"], method="least squares normal equations",
     latex=r"\hat\beta = \frac{\sum(x - \bar x)(y - \bar y)}{\sum (x - \bar x)^{2}},\quad \hat\alpha = \bar y - \hat\beta\bar x", run=ols_simple,
     reference=_ref_ols,
     wrong=lambda x, y: (lambda o: dict(o, slope=o["slope"] * len(x) / (len(x) - 1)))(ols_simple(x, y)),
     rand=_rand_xy,
     triggers=["linear regression", "ols regression", "least squares regression", "regression line", "simple regression", "ols"],
     templates=["Fit a linear regression with x = [1, 2, 3, 4, 5], y = [2.1, 3.9, 6.2, 7.8, 10.1]."],
     examples=[{"inputs": {"x": "1, 2, 3, 4, 5", "y": "2.1, 3.9, 6.2, 7.8, 10.1"}, "outputs": {'slope': 1.9899999999999998, 'intercept': 0.0500000000000016, 'r_squared': 0.9973053289009771, 'slope_std_error': 0.05972157622389639, 'n': 5}},
               {"inputs": {"x": [1, 2, 3], "y": [3, 5, 7]}, "outputs": {"slope": 2.0, "intercept": 1.0, "r_squared": 1.0}}],
     questions=[{"q": "Fit a linear regression with x = [1, 2, 3, 4, 5], y = [2.1, 3.9, 6.2, 7.8, 10.1].", "outputs": {'slope': 1.9899999999999998, 'intercept': 0.0500000000000016, 'r_squared': 0.9973053289009771, 'slope_std_error': 0.05972157622389639, 'n': 5}},
                {"q": "Fit a linear regression with x = [1, 1, 1], y = [3, 5, 7].", "outcome": "INVALID_INPUT"}])


def multiple_regression(X, y):
    rows, Y = X, _vec("y", y, 3)
    _need(isinstance(rows, (list, tuple)) and len(rows) == len(Y), "X needs one row per observation of y")
    k = len(rows[0])
    n = len(Y)
    _need(n > k + 1, "more observations than coefficients are needed (n > k + 1)")
    cols = [[1.0] * n] + [[_fin("X", r[j]) for r in rows] for j in range(k)]
    # modified Gram-Schmidt QR of [1, X]
    Q, R = [], [[0.0] * (k + 1) for _ in range(k + 1)]
    for j, v in enumerate(cols):
        v = list(v)
        for i, q in enumerate(Q):
            R[i][j] = math.fsum(a * b for a, b in zip(q, v))
            v = [a - R[i][j] * b for a, b in zip(v, q)]
        R[j][j] = math.sqrt(math.fsum(a * a for a in v))
        _need(R[j][j] > 1e-10 * max(1.0, math.sqrt(math.fsum(a * a for a in cols[j]))), "the columns of X are (nearly) collinear")
        Q.append([a / R[j][j] for a in v])
    qty = [math.fsum(a * b for a, b in zip(q, Y)) for q in Q]
    beta = [0.0] * (k + 1)
    for i in reversed(range(k + 1)):
        beta[i] = (qty[i] - math.fsum(R[i][j] * beta[j] for j in range(i + 1, k + 1))) / R[i][i]
    my = _mean(Y)
    sst = math.fsum((v - my) ** 2 for v in Y)
    _need(sst > 0, "y must not be constant")
    ssr = math.fsum((Y[t] - math.fsum(beta[j] * cols[j][t] for j in range(k + 1))) ** 2 for t in range(n))
    r2 = 1 - ssr / sst
    return {"coefficients": beta, "r_squared": r2, "adj_r_squared": 1 - (1 - r2) * (n - 1) / (n - k - 1)}


def _ref_mreg(X, y):
    import numpy as np
    import statsmodels.api as sm
    f = sm.OLS(np.asarray(y, dtype=float), sm.add_constant(np.asarray(X, dtype=float), has_constant="add")).fit()
    return {"coefficients": [float(v) for v in f.params], "r_squared": float(f.rsquared), "adj_r_squared": float(f.rsquared_adj)}


def _rand_mreg(rng):
    n, k = rng.randint(8, 40), rng.randint(1, 4)
    b = [rng.uniform(-5, 5) for _ in range(k + 1)]
    X = [[_u(rng, -10, 10, 3) for _ in range(k)] for _ in range(n)]
    return {"X": X, "y": [round(b[0] + sum(bj * xj for bj, xj in zip(b[1:], r)) + rng.gauss(0, 2), 3) for r in X]}


_add("multiple_regression", name="Multiple linear regression (OLS)", topic="econometrics / regression",
     purpose="Least-squares coefficients of y = b0 + b1 x1 + ... + bk xk (intercept added), R^2 and adjusted R^2.",
     inputs=[_i("X", "regressor matrix, one row per observation, one column per variable (no constant column)", "x", "regressors",
                "design matrix", type="matrix"),
             _i("y", "dependent variable, one value per row of X", "y values", "dependent variable", "response", type="vector", min_length=3)],
     outputs=[_o("coefficients", "[b0 (intercept), b1, ..., bk]"), _o("r_squared", "1 - SSR/SST"), _o("adj_r_squared", "1 - (1 - R^2)(n - 1)/(n - k - 1)")],
     primary="r_squared", conditions=["one row of X per value of y", "n > k + 1", "columns not collinear", "y not constant"],
     method="QR decomposition (modified Gram-Schmidt) of [1, X], back substitution",
     latex=r"\hat\beta = (X^{\top}X)^{-1}X^{\top}y", run=multiple_regression, reference=_ref_mreg,
     wrong=lambda X, y: (lambda o: dict(o, r_squared=1 - (1 - o["r_squared"]) * len(y) / (len(y) - 1)))(multiple_regression(X, y)),
     rand=_rand_mreg,
     triggers=["multiple regression", "multiple linear regression", "multivariate regression"],
     templates=["Fit a multiple regression with X = [[1, 2], [2, 1], [3, 4], [4, 3], [5, 6]], y = [5, 5, 10, 10, 15]."],
     examples=[{"inputs": {"X": "[[1, 2], [2, 1], [3, 4], [4, 3], [5, 6]]", "y": "5, 5, 10, 10, 15"}, "outputs": {'coefficients': [1.250000000000001, 1.2499999999999973, 1.2500000000000024], 'r_squared': 1.0, 'adj_r_squared': 1.0}},
               {"inputs": {"X": [[1], [2], [3], [4]], "y": [3, 5, 7, 9]}, "outputs": {"coefficients": [1.0, 2.0], "r_squared": 1.0}}],
     questions=[{"q": "Fit a multiple regression with X = [[1, 2], [2, 1], [3, 4], [4, 3], [5, 6]], y = [5, 5, 10, 10, 15].", "outputs": {'coefficients': [1.250000000000001, 1.2499999999999973, 1.2500000000000024], 'r_squared': 1.0, 'adj_r_squared': 1.0}},
                {"q": "Fit a multiple regression with X = [[1, 2], [2, 4], [3, 6], [4, 8], [5, 10]], y = [5, 5, 10, 10, 15].", "outcome": "INVALID_INPUT"}])


def log_log_elasticity(prices, quantities):
    p, q = _vec("prices", prices, 3), _vec("quantities", quantities, 3)
    _need(len(p) == len(q), "one quantity per price")
    _need(all(v > 0 for v in p + q), "prices and quantities must be positive (logarithms)")
    o = ols_simple([math.log(v) for v in p], [math.log(v) for v in q])
    return {"elasticity": o["slope"], "intercept": o["intercept"], "r_squared": o["r_squared"]}


_add("log_log_elasticity", name="Constant elasticity from a log-log regression", topic="econometrics / demand estimation",
     purpose="The price elasticity estimated as the slope of ln Q on ln P (constant-elasticity demand Q = A P^e).",
     inputs=[_i("prices", "observed prices", "p", "price", type="vector", min_length=3),
             _i("quantities", "quantities at those prices", "q", "quantity", "sales", type="vector", min_length=3)],
     outputs=[_o("elasticity", "slope of ln Q on ln P"), _o("intercept", "ln A"), _o("r_squared", "R^2 of the log-log fit")],
     primary="elasticity", conditions=["positive values", "prices not all equal"], method="OLS of ln Q on ln P",
     latex=r"\ln Q = \ln A + \varepsilon \ln P", run=log_log_elasticity,
     reference=lambda prices, quantities: (lambda r: {"elasticity": r["slope"], "intercept": r["intercept"], "r_squared": r["r_squared"]})(
         _ref_ols([math.log(v) for v in prices], [math.log(v) for v in quantities])),
     wrong=lambda prices, quantities: (lambda o: dict(o, elasticity=ols_simple(prices, quantities)["slope"] * _mean(prices) / _mean(quantities)))(
         log_log_elasticity(prices, quantities)),
     rand=lambda rng: (lambda e, A, n: (lambda p: {"prices": p, "quantities": [round(A * v ** e * math.exp(rng.gauss(0, 0.1)), 4) for v in p]})(
         [_u(rng, 1, 50, 2) for _ in range(n)]))(rng.uniform(-3, -0.2), rng.uniform(100, 5000), rng.randint(5, 40)),
     triggers=["log-log regression", "log log regression", "constant elasticity", "elasticity from data", "estimate the elasticity"],
     templates=["Estimate the elasticity from data with prices = [1, 2, 4, 8], quantities = [100, 50, 25, 12.5]."],
     examples=[{"inputs": {"prices": "1, 2, 4, 8", "quantities": "100, 50, 25, 12.5"}, "outputs": {"elasticity": -1.0, "r_squared": 1.0}},
               {"inputs": {"prices": [2, 3, 5, 7], "quantities": [90, 70, 50, 41]}, "outputs": {'elasticity': -0.6322080250613804, 'intercept': 4.9385960690744515, 'r_squared': 0.9996466499533456}}],
     questions=[{"q": "Estimate the elasticity from data with prices = [1, 2, 4, 8], quantities = [100, 50, 25, 12.5].", "outputs": {"elasticity": -1.0}},
                {"q": "Estimate the elasticity from data with prices = [1, 2, 4], quantities = [100, 0, 25].", "outcome": "INVALID_INPUT"}])


def gini_coefficient(incomes):
    x = _vec("incomes", incomes, 2)
    _need(all(v >= 0 for v in x) and sum(x) > 0, "incomes must be non-negative with a positive total")
    s, n = sorted(x), len(x)
    return {"gini": math.fsum((2 * i - n - 1) * v for i, v in enumerate(s, start=1)) / (n * math.fsum(s))}


_add("gini_coefficient", name="Gini coefficient", topic="microeconomics / inequality",
     purpose="Gini coefficient of a list of incomes (population formula: mean absolute difference / twice the mean; "
             "0 = equality, (n-1)/n = one person has everything).",
     inputs=[_i("incomes", "incomes (or wealth) of the people or households", "income", "wealth", "values", type="vector", min_length=2)],
     outputs=[_o("gini", "sum (2i - n - 1) x_(i) / (n sum x)")], primary="gini", conditions=["incomes >= 0 with a positive total"],
     method="sorted-rank formula", latex=r"G = \frac{\sum_i \sum_j |x_i - x_j|}{2 n^{2} \bar x}", run=gini_coefficient,
     reference=lambda incomes: (lambda np: (lambda a: {"gini": float(np.abs(a[:, None] - a[None, :]).sum() / (2 * len(a) ** 2 * a.mean()))})(
         np.asarray(incomes, dtype=float)))(__import__("numpy")),
     wrong=lambda incomes: {"gini": math.fsum((2 * i - len(incomes) - 1) * v for i, v in enumerate(incomes, start=1)) / (len(incomes) * math.fsum(incomes))},
     rand=lambda rng: {"incomes": [round(rng.lognormvariate(10, 0.8), 2) for _ in range(rng.randint(3, 60))]},
     triggers=["gini coefficient", "gini index", "gini"],
     templates=["Find the Gini coefficient with incomes = [20000, 35000, 50000, 120000]."],
     examples=[{"inputs": {"incomes": "20000, 35000, 50000, 120000"}, "outputs": {'gini': 0.35}},
               {"inputs": {"incomes": [5, 5, 5, 5]}, "outputs": {"gini": 0.0}}, {"inputs": {"incomes": [0, 0, 0, 10]}, "outputs": {"gini": 0.75}}],
     questions=[{"q": "Find the Gini coefficient with incomes = [20000, 35000, 50000, 120000].", "outputs": {'gini': 0.35}},
                {"q": "Find the Gini coefficient with incomes = [0, 0].", "outcome": "INVALID_INPUT"}])


def herfindahl_index(market_shares):
    x = _vec("market_shares", market_shares, 1)
    _need(all(v >= 0 for v in x) and sum(x) > 0, "shares (or sales) must be non-negative with a positive total")
    tot = math.fsum(x)
    s = [v / tot for v in x]
    h = math.fsum(v * v for v in s)
    n = len(s)
    return {"hhi": 10000 * h, "hhi_fraction": h, "cr4": 100 * math.fsum(sorted(s, reverse=True)[:4]), "equivalent_firms": 1 / h}


_add("herfindahl_index", name="Herfindahl-Hirschman index and concentration ratio", topic="microeconomics / market structure",
     purpose="Market concentration from the firms' shares or sales (normalized to shares): HHI on the 0-10,000 scale, "
             "the four-firm concentration ratio and the equivalent number of equal-sized firms.",
     inputs=[_i("market_shares", "market shares (in %) or sales of each firm", "shares", "sales", "market share", "firm sales",
                type="vector", min_length=1)],
     outputs=[_o("hhi", "10,000 x sum of squared shares"), _o("hhi_fraction", "sum of squared shares (0-1)"),
              _o("cr4", "combined share of the four largest firms, %"), _o("equivalent_firms", "1 / HHI (fraction)")], primary="hhi",
     conditions=["shares or sales >= 0 with a positive total"], method="normalize to shares s_i; HHI = sum s_i^2",
     latex=r"HHI = 10\,000\sum_i s_i^{2}", run=herfindahl_index,
     reference=lambda market_shares: (lambda np: (lambda s: {"hhi_fraction": float(1 / len(s) + len(s) * np.var(s)),
                                                             "hhi": float(10000 * (1 / len(s) + len(s) * np.var(s))),
                                                             "cr4": float(100 * np.sort(s)[::-1][:4].sum())})(
         np.asarray(market_shares, dtype=float) / np.sum(market_shares)))(__import__("numpy")),
     wrong=lambda market_shares: (lambda o: dict(o, hhi=o["hhi"] / 100))(herfindahl_index(market_shares)),
     rand=lambda rng: {"market_shares": [_u(rng, 0.5, 40, 2) for _ in range(rng.randint(2, 15))]},
     triggers=["herfindahl", "herfindahl-hirschman index", "hhi", "concentration ratio", "market concentration"],
     templates=["Find the Herfindahl index with market shares = [40, 30, 20, 10]."],
     examples=[{"inputs": {"market_shares": "40, 30, 20, 10"}, "outputs": {"hhi": 3000.0, "cr4": 100.0}},
               {"inputs": {"market_shares": [1, 1, 1, 1, 1]}, "outputs": {"hhi": 2000.0, "cr4": 80.0, "equivalent_firms": 5.0}}],
     questions=[{"q": "Find the Herfindahl index with market shares = [40, 30, 20, 10].", "outputs": {"hhi": 3000.0}},
                {"q": "Find the HHI with sales = [0, 0, 0].", "outcome": "INVALID_INPUT"}])


def durbin_watson(residuals):
    e = _vec("residuals", residuals, 2)
    den = math.fsum(v * v for v in e)
    _need(den > 0, "the residuals are all zero")
    return {"durbin_watson": math.fsum((b - a) ** 2 for a, b in zip(e, e[1:])) / den}


_add("durbin_watson", name="Durbin-Watson statistic", topic="econometrics / diagnostics",
     purpose="Durbin-Watson statistic of regression residuals in time order (about 2: no first-order autocorrelation; "
             "toward 0: positive; toward 4: negative).",
     inputs=[_i("residuals", "regression residuals in time order", "e", "errors", "residual", type="vector", min_length=2)],
     outputs=[_o("durbin_watson", "sum (e_t - e_t-1)^2 / sum e_t^2")], primary="durbin_watson", conditions=["residuals not all zero"],
     method="DW = sum_{t>=2} (e_t - e_{t-1})^2 / sum_t e_t^2", latex=r"DW = \frac{\sum_{t=2}^{n}(e_t - e_{t-1})^{2}}{\sum_{t=1}^{n} e_t^{2}}",
     run=durbin_watson,
     reference=lambda residuals: {"durbin_watson": float(__import__("statsmodels.stats.stattools", fromlist=["durbin_watson"]).durbin_watson(
         __import__("numpy").asarray(residuals, dtype=float)))},
     wrong=lambda residuals: {"durbin_watson": math.fsum((b - a) ** 2 for a, b in zip(residuals, residuals[1:])) / math.fsum(v * v for v in residuals[1:])},
     rand=lambda rng: {"residuals": [round(rng.gauss(0, 1), 4) for _ in range(rng.randint(5, 60))]},
     triggers=["durbin-watson", "durbin watson"],
     templates=["Find the Durbin-Watson statistic with residuals = [0.5, -0.3, 0.2, -0.4, 0.1]."],
     examples=[{"inputs": {"residuals": "0.5, -0.3, 0.2, -0.4, 0.1"}, "outputs": {'durbin_watson': 2.7272727272727275}}, {"inputs": {"residuals": [1, 1]}, "outputs": {"durbin_watson": 0.0}}],
     questions=[{"q": "Find the Durbin-Watson statistic with residuals = [0.5, -0.3, 0.2, -0.4, 0.1].", "outputs": {'durbin_watson': 2.7272727272727275}},
                {"q": "Find the Durbin-Watson statistic with residuals = [0, 0, 0].", "outcome": "INVALID_INPUT"}])


def correlation(x, y):
    X, Y = _vec("x", x, 3), _vec("y", y, 3)
    _need(len(X) == len(Y), "x and y need the same number of observations")
    sxx, syy = _cov(X, X), _cov(Y, Y)
    _need(sxx > 0 and syy > 0, "neither variable may be constant")
    c = _cov(X, Y)
    r = c / math.sqrt(sxx * syy)
    return {"correlation": r, "covariance": c, "r_squared": r * r}


_add("correlation", name="Correlation and covariance of two series", topic="econometrics / association",
     purpose="Pearson correlation and sample covariance (n - 1) of two paired series, such as the returns of two assets.",
     inputs=[_i("x", "first series", "x values", "series 1", "returns 1", type="vector", min_length=3),
             _i("y", "second series", "y values", "series 2", "returns 2", type="vector", min_length=3)],
     outputs=[_o("correlation", "Pearson r"), _o("covariance", "sample covariance"), _o("r_squared", "r^2")], primary="correlation",
     conditions=["same length, at least 3", "neither series constant"], method="r = S_xy / sqrt(S_xx S_yy)",
     latex=r"r = \frac{\sum (x - \bar x)(y - \bar y)}{\sqrt{\sum (x-\bar x)^{2}\sum (y-\bar y)^{2}}}", run=correlation,
     reference=lambda x, y: (lambda st, np: {"correlation": float(st.pearsonr(x, y)[0]), "covariance": float(np.cov(x, y, ddof=1)[0, 1])})(
         __import__("scipy.stats", fromlist=["pearsonr"]), __import__("numpy")),
     wrong=lambda x, y: (lambda o: dict(o, covariance=_cov(x, y, 0)))(correlation(x, y)),
     rand=_rand_xy,
     triggers=["correlation", "correlation coefficient", "covariance", "pearson correlation"],
     templates=["Find the correlation with x = [1, 2, 3, 4, 5], y = [2, 4, 5, 4, 5]."],
     examples=[{"inputs": {"x": "1, 2, 3, 4, 5", "y": "2, 4, 5, 4, 5"}, "outputs": {'correlation': 0.7745966692414834, 'covariance': 1.5, 'r_squared': 0.6000000000000001}},
               {"inputs": {"x": [1, 2, 3], "y": [6, 4, 2]}, "outputs": {"correlation": -1.0, "covariance": -2.0}}],
     questions=[{"q": "Find the correlation with x = [1, 2, 3, 4, 5], y = [2, 4, 5, 4, 5].", "outputs": {'correlation': 0.7745966692414834, 'covariance': 1.5, 'r_squared': 0.6000000000000001}},
                {"q": "Find the correlation with x = [1, 1, 1], y = [6, 4, 2].", "outcome": "INVALID_INPUT"}])
