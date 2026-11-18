# Economics and Finance (economics-finance-ai999x)

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

Economics and Finance covers time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics: 105 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Economics and Finance desktop app: Loan payment (amortizing loan), with its worked example and live scene](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-economics-finance-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-economics-finance-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `economics-finance` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install economics-finance-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Economics and Finance desktop app: every scene and chart is drawn from the computed answer.

[![Economics and Finance desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/clip.mp4)*

| | |
|---|---|
| [![Loan payment (amortizing loan): the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-1.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-loan_payment)<br>Loan payment (amortizing loan) | [![Internal rate of return: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-2.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-irr)<br>Internal rate of return |
| [![Black-Scholes option prices: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-3.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-black_scholes)<br>Black-Scholes option prices | [![Bond price from its yield: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-4.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-bond_price)<br>Bond price from its yield |
| [![Market equilibrium and surplus (linear curves): the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-5.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-market_equilibrium)<br>Market equilibrium and surplus (linear curves) | [![Standard-cost variances: materials price and quantity, labour rate and efficiency: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-6.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-direct_cost_variances)<br>Standard-cost variances: materials price and quantity, labour rate and efficiency |
| [![Endowment insurance: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-7.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-endowment_insurance)<br>Endowment insurance | [![Credit card finance charge (average daily balance): the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-8.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-credit_card_finance_charge)<br>Credit card finance charge (average daily balance) |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Loan payment (amortizing loan): a chart in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-1.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-loan_payment)<br>Loan payment (amortizing loan) | [![Internal rate of return: a chart in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-2.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-irr)<br>Internal rate of return | [![Black-Scholes option prices: a chart in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-3.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-black_scholes)<br>Black-Scholes option prices | [![Bond price from its yield: a chart in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-4.webp)](https://www.ai999x.com/models/economics-finance-ai999x/#cap-bond_price)<br>Bond price from its yield |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import economics_finance_ai999x as economics_finance

a = economics_finance.ask(
    "Find the discounted payback period with rate = 10%, cash flows = "
    "[-1000, 300, 400, 500, 200]."
)
print(a.outcome, a.values)
# SOLVED {'discounted_payback': 3.154000000000001}

a = economics_finance.ask(
    "Find the compound annual growth rate with begin value = 1000, end "
    "value = 2000, years = 5."
)
print(a.outcome, a.values)
# SOLVED {'cagr': 0.1486983549970351, 'total_growth': 1.0}

a = economics_finance.ask(
    "Find the life expectancy with mortality rates = [0.1, 0.3, 0.6, "
    "1]."
)
print(a.outcome, a.values)
# SOLVED {'curtate': 1.782, 'complete': 2.282}

a = economics_finance.ask(
    "Find the equivalent annual annuity with rate = 10%, cash flows = "
    "[-1000, 300, 400, 500]."
)
print(a.outcome, a.values)
# SOLVED {'eaa': -8.459214501510571, 'npv': -21.036814425244188}

a = economics_finance.ask(
    "Find the maximum drawdown with prices = [100, 120, 90, 110, 80, "
    "130]."
)
print(a.outcome, a.values)
# SOLVED {'max_drawdown': 0.33333333333333337}

a = economics_finance.ask(
    "Find the compound interest with principal = 1000, rate = 5%, "
    "years = 2."
)
print(a.outcome, a.values)
# SOLVED {'amount': 1102.5, 'interest': 102.5}

a = economics_finance.ask(
    "Find the convexity with face = 1000, coupon rate = 5%, years = "
    "10, ytm = 6%, frequency = 2."
)
print(a.outcome, a.values)
# SOLVED {'convexity': 71.78539801290005, 'price': 925.6126256977227}

a = economics_finance.ask(
    "Find the internal rate of return with cash flows = [-1000, 300, "
    "400, 500]."
)
print(a.outcome, a.values)
# SOLVED {'irr': 0.08896339469334996}

a = economics_finance.ask(
    "Find the Durbin-Watson statistic with residuals = [0.5, -0.3, "
    "0.2, -0.4, 0.1]."
)
print(a.outcome, a.values)
# SOLVED {'durbin_watson': 2.7272727272727275}

a = economics_finance.ask(
    "Find the amount with continuous compounding with principal = "
    "1000, rate = 5%, years = 10."
)
print(a.outcome, a.values)
# SOLVED {'amount': 1648.7212707001281, 'interest': 648.7212707001281}

a = economics_finance.ask(
    "Find the forward rate with s1 = 4%, t1 = 1, s2 = 5%, t2 = 2."
)
print(a.outcome, a.values)
# SOLVED {'forward_rate': 0.060096153846153744}

a = economics_finance.ask(
    "Find the number of payments with present value = 10000, payment = "
    "500, rate = 1%."
)
print(a.outcome, a.values)
# SOLVED {'periods': 22.425741878036437}

a = economics_finance.ask(
    "Find the cross-price elasticity with p1 = 2, q1 = 100, p2 = 2.5, "
    "q2 = 120."
)
print(a.outcome, a.values)
# SOLVED {'elasticity': 0.8181818181818182, 'relationship': 'substitutes'}

a = economics_finance.ask("Find the doubling time with rate = 6%.")
print(a.outcome, a.values)
# SOLVED {'periods': 11.895661045941875, 'rule_of_72': 12.0}

a = economics_finance.ask(
    "Find the temporary annuity value with mortality rates = [0.01, "
    "0.02, 0.03], interest rate = 4%, term = 3, payment = 1000."
)
print(a.outcome, a.values)
# SOLVED {'apv': 2848.9275147928993}
```

## Command line: 15 examples

Each command below was run with `economics-finance` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ economics-finance solve financial_ratios sales=500000 net_income=40000 total_assets=400000 total_equity=160000 current_assets=150000 current_liabilities=100000 inventory=60000
current_ratio = 1.5
quick_ratio = 0.9
debt_ratio = 0.6
debt_to_equity = 1.5
net_profit_margin = 0.08
asset_turnover = 1.25

$ economics-finance solve endowment_insurance "mortality_rates=[0.01, 0.02, 0.03]" interest_rate=0.04 term=3 benefit=1000
apv = 890.4258648
term_part = 53.79672565
pure_endowment = 836.6291392

$ economics-finance solve bond_price_change face=1000 coupon_rate=0.05 years=10 ytm=0.06 yield_change=0.01 frequency=2
duration_estimate = -70.94863318
duration_convexity_estimate = -67.62635965
actual_change = -67.73665872
new_price = 857.875967
price = 925.6126257

$ economics-finance solve discounted_payback rate=0.1 "cash_flows=[-1000, 300, 400, 500, 200]"
discounted_payback = 3.154

$ economics-finance solve credit_card_finance_charge opening_balance=1500 apr=0.18 billing_days=30 rate_basis=daily "transaction_days=[11, 21]" "transaction_amounts=[-500, 200]"
average_daily_balance = 1233.333333
finance_charge = 18.24657534
cycle_rate = 0.01479452055
closing_balance = 1200
new_balance = 1218.246575

$ economics-finance solve log_log_elasticity "prices=[1, 2, 4, 8]" "quantities=[100, 50, 25, 12.5]"
elasticity = -1
intercept = 4.605170186
r_squared = 1

$ economics-finance solve comparative_advantage a_good_x=6 a_good_y=3 b_good_x=1 b_good_y=2 basis=output
opportunity_cost_x_a = 0.5
opportunity_cost_y_a = 2
opportunity_cost_x_b = 2
opportunity_cost_y_b = 0.5
comparative_advantage_x = country A
comparative_advantage_y = country B

$ economics-finance solve loan_cumulative_interest principal=200000 annual_rate=0.06 years=30 payments_made=12
cumulative_interest = 11933.18918
principal_repaid = 2456.023425

$ economics-finance solve inflation_adjustment "price_levels=[100, 103, 107.12]" amount=1000
total_inflation = 0.0712
average_inflation = 0.03498792263
real_value = 933.5324869

$ economics-finance solve break_even fixed_costs=10000 price=50 variable_cost=30
quantity = 500
revenue = 25000
contribution_margin = 20
contribution_margin_ratio = 0.4

$ economics-finance solve mean_returns "returns=[0.1, -0.05, 0.2]"
arithmetic_mean = 0.08333333333
geometric_mean = 0.07836515339
cumulative_return = 0.254

$ economics-finance solve historical_var "returns=[-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015]" confidence=0.9
var = 0.02
cvar = 0.0275
tail_observations = 2

$ economics-finance solve capm risk_free=0.03 beta=1.2 market_return=0.09
expected_return = 0.102
market_risk_premium = 0.06
risk_premium = 0.072

$ economics-finance solve binomial_option spot=100 strike=100 time=1 rate=0.05 volatility=0.2 steps=3
price = 11.04387109
delta = 0.6140855606
up_factor = 1.122400902
probability_up = 0.5437765964

$ economics-finance solve cagr begin_value=1000 end_value=2000 years=5
cagr = 0.148698355
total_growth = 1
```

## Rust library: 15 examples

The crate `economics-finance-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `economics_finance_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/economics-finance/examples/readme.rs`: `cd rust && cargo run --release --example readme -p economics-finance-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&economics_finance_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "financial_ratios", "inputs": {"sales": "500000", "net_income": "40000", "total_assets": "400000", "total_equity": "160000", "current_assets": "150000", "current_liabilities": "100000", "inventory": "60000"}}));
// SOLVED
// current_ratio = 1.5
// quick_ratio = 0.9
// debt_ratio = 0.6
// debt_to_equity = 1.5
// net_profit_margin = 0.08
// asset_turnover = 1.25

show(json!({"key": "endowment_insurance", "inputs": {"mortality_rates": [0.01, 0.02, 0.03], "interest_rate": "0.04", "term": "3", "benefit": "1000"}}));
// SOLVED
// apv = 890.4258648156573
// term_part = 53.796725648611734
// pure_endowment = 836.6291391670457

show(json!({"key": "bond_price_change", "inputs": {"face": "1000", "coupon_rate": "0.05", "years": "10", "ytm": "0.06", "yield_change": "0.01", "frequency": "2"}}));
// SOLVED
// duration_estimate = -70.94863318371567
// duration_convexity_estimate = -67.62635964664184
// actual_change = -67.73665871724529
// new_price = 857.8759669804774
// price = 925.6126256977227

show(json!({"key": "discounted_payback", "inputs": {"rate": "0.1", "cash_flows": [-1000, 300, 400, 500, 200]}}));
// SOLVED
// discounted_payback = 3.154000000000001

show(json!({"key": "credit_card_finance_charge", "inputs": {"opening_balance": "1500", "apr": "0.18", "billing_days": "30", "rate_basis": "daily", "transaction_days": [11, 21], "transaction_amounts": [-500, 200]}}));
// SOLVED
// average_daily_balance = 1233.3333333333333
// finance_charge = 18.24657534246575
// cycle_rate = 0.014794520547945203
// closing_balance = 1200.0
// new_balance = 1218.2465753424658

show(json!({"key": "log_log_elasticity", "inputs": {"prices": [1, 2, 4, 8], "quantities": [100, 50, 25, 12.5]}}));
// SOLVED
// elasticity = -1.0000000000000002
// intercept = 4.605170185988092
// r_squared = 1.0

show(json!({"key": "comparative_advantage", "inputs": {"a_good_x": "6", "a_good_y": "3", "b_good_x": "1", "b_good_y": "2", "basis": "output"}}));
// SOLVED
// opportunity_cost_x_a = 0.5
// opportunity_cost_y_a = 2.0
// opportunity_cost_x_b = 2.0
// opportunity_cost_y_b = 0.5
// comparative_advantage_x = "country A"
// comparative_advantage_y = "country B"

show(json!({"key": "loan_cumulative_interest", "inputs": {"principal": "200000", "annual_rate": "0.06", "years": "30", "payments_made": "12"}}));
// SOLVED
// cumulative_interest = 11933.189179112655
// principal_repaid = 2456.0234245535103

show(json!({"key": "inflation_adjustment", "inputs": {"price_levels": [100, 103, 107.12], "amount": "1000"}}));
// SOLVED
// total_inflation = 0.07120000000000015
// average_inflation = 0.03498792263484907
// real_value = 933.5324869305451

show(json!({"key": "break_even", "inputs": {"fixed_costs": "10000", "price": "50", "variable_cost": "30"}}));
// SOLVED
// quantity = 500.0
// revenue = 25000.0
// contribution_margin = 20.0
// contribution_margin_ratio = 0.4

show(json!({"key": "mean_returns", "inputs": {"returns": [0.1, -0.05, 0.2]}}));
// SOLVED
// arithmetic_mean = 0.08333333333333333
// geometric_mean = 0.07836515339093597
// cumulative_return = 0.2540000000000002

show(json!({"key": "historical_var", "inputs": {"returns": [-0.02, 0.01, -0.035, 0.004, 0.012, -0.01, 0.02, -0.005, 0.007, 0.001, -0.015], "confidence": "0.9"}}));
// SOLVED
// var = 0.02
// cvar = 0.027500000000000004
// tail_observations = 2

show(json!({"key": "capm", "inputs": {"risk_free": "0.03", "beta": "1.2", "market_return": "0.09"}}));
// SOLVED
// expected_return = 0.102
// market_risk_premium = 0.06
// risk_premium = 0.072

show(json!({"key": "binomial_option", "inputs": {"spot": "100", "strike": "100", "time": "1", "rate": "0.05", "volatility": "0.2", "steps": "3"}}));
// SOLVED
// price = 11.043871091951113
// delta = 0.6140855606115926
// up_factor = 1.1224009024456676
// probability_up = 0.5437765963610321

show(json!({"key": "cagr", "inputs": {"begin_value": "1000", "end_value": "2000", "years": "5"}}));
// SOLVED
// cagr = 0.1486983549970351
// total_growth = 1.0
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Economics and Finance on ai999x.com](https://www.ai999x.com/models/economics-finance-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/economics-finance-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/economics-finance-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `economics-finance` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/economics-finance`) and its data (`rust/models/economics_finance/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p economics-finance-ai999x
./target/release/economics-finance --help
cargo test -p economics-finance-ai999x
```
