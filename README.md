# Economics and Finance (economics-finance-ai999x)

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

Economics and Finance covers time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics: 105 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Economics and Finance desktop app: Loan payment (amortizing loan), with its worked example and live scene](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-economics-finance-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-economics-finance-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `economics-finance` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
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

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Economics and Finance on ai999x.com](https://www.ai999x.com/models/economics-finance-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/economics-finance-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/economics-finance-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `economics-finance` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/economics-finance`) and its data (`rust/models/economics_finance/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p economics-finance-ai999x
./target/release/economics-finance --help
cargo test -p economics-finance-ai999x
```
