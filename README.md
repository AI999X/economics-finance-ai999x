# Economics and Finance (economics-finance-ai999x)

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

Economics and Finance covers time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics: 105 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Economics and Finance desktop app: Effective annual rate, with its worked example and live scene](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `economics-finance` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#try) |
| **Python** 3.10+ | `pip install economics-finance-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Economics and Finance desktop app: Effective annual rate, with its worked example and live scene](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero-dark.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

[![Cournot duopoly equilibrium: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/see-1.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

*Cournot duopoly equilibrium*

[![Binomial option price (Cox-Ross-Rubinstein): the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/see-2.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

*Binomial option price (Cox-Ross-Rubinstein)*

[![Correlation and covariance of two series: the live scene in the Economics and Finance desktop app](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/see-3.webp)](https://www.ai999x.com/models/economics-finance-ai999x/)

*Correlation and covariance of two series*

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
