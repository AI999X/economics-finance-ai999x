# economics-finance-ai999x — offline, verified economics and finance solver with step-by-step answers

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

**[Install](https://pypi.org/project/economics-finance-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

Field: **Economics & Finance** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

economics-finance-ai999x is an offline, deterministic Python solver that answers economics and finance questions with verified results from 96 certified capabilities, gives 384 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**economics-finance-ai999x** is a compact, deterministic economics and finance engine for Python. It reads a question in plain English (or a typed call), picks a certified economics and finance capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified economics and finance capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 384 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics.

## Model overview

| Property | Value |
|---|---|
| Package | `economics-finance-ai999x` 1.0.1 (import `economics_finance_ai999x`) |
| Type | Deterministic neuro-symbolic economics and finance engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 96 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 384 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install economics-finance-ai999x
```

```python
import economics_finance_ai999x as economics_finance

economics_finance.ask("Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.").show()
# -> SOLVED — payment = 1199.1, number_of_payments = 360, total_paid = 431676, total_interest = 231676  (loan_payment)
economics_finance.ask("Find the internal rate of return with cash flows = [-1000, 300, 400, 500].").show()
# -> SOLVED — irr = 0.0889634  (irr)
economics_finance.solve("black_scholes", spot=100, strike=100, time=1, rate=0.05, volatility=0.2).show()
# -> SOLVED — call = 10.4506, put = 5.57353, d1 = 0.35, d2 = 0.15  (black_scholes)
economics_finance.ask("Find the bond price with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2.").show()
# -> SOLVED — price = 925.613, current_yield = 0.0540183, coupon = 25  (bond_price)
economics_finance.ask("Find the market equilibrium with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4.").show()
# -> SOLVED — price = 20, quantity = 60, consumer_surplus = 900, producer_surplus = 450  (market_equilibrium)
economics_finance.ask("Who created you?").message
# -> economics-finance-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `economics-finance-ai999x "<question>"`, `economics-finance-ai999x --capabilities`. The outputs above were produced by running this exact code on economics-finance-ai999x 1.0.1.

## Find the right capability, and draw it

`economics_finance.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "economics-finance-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import economics_finance_ai999x as economics_finance

economics_finance.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `economics-finance-ai999x`; the answer is the one it returned, with the steps it took.

**Loan payment (amortizing loan)** · _Loans and annuities_

> Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.

→ **payment = 1199.10105, number_of_payments = 360, total_paid = 431676.3781, total_interest = 231676.3781**

<details><summary>Steps</summary>

- Loan payment (amortizing loan): principal = 200000, annual_rate = 0.06, years = 30 → payment = 1199.10105, number_of_payments = 360, total_paid = 431676.3781, total_interest = 231676.3781

</details>

**Internal rate of return** · _Capital budgeting_

> Find the internal rate of return with cash flows = [-1000, 300, 400, 500].

→ **irr = 0.08896339469**

<details><summary>Steps</summary>

- Internal rate of return: cash_flows = [-1000, 300, 400, 500] → irr = 0.08896339469

</details>

**Black-Scholes option prices** · _Stocks and options_

`m.solve("black_scholes", spot=100, strike=100, time=1, rate=0.05, volatility=0.2)`

→ **call = 10.45058357, put = 5.573526022, d1 = 0.35, d2 = 0.15**

<details><summary>Steps</summary>

- Black-Scholes option prices: spot = 100, strike = 100, time = 1, rate = 0.05, volatility = 0.2 → call = 10.45058357, put = 5.573526022, d1 = 0.35, d2 = 0.15

</details>

**Bond price from its yield** · _Bonds_

> Find the bond price with face = 1000, coupon rate = 5%, years = 10, ytm = 6%, frequency = 2.

→ **price = 925.6126257, current_yield = 0.05401827785, coupon = 25**

<details><summary>Steps</summary>

- Bond price from its yield: face = 1000, coupon_rate = 0.05, years = 10, ytm = 0.06, frequency = 2 → price = 925.6126257, current_yield = 0.05401827785, coupon = 25

</details>

**Market equilibrium and surplus (linear curves)** · _Microeconomics_

> Find the market equilibrium with demand intercept = 100, demand slope = 2, supply intercept = -20, supply slope = 4.

→ **price = 20, quantity = 60, consumer_surplus = 900, producer_surplus = 450, total_surplus = 1350**

<details><summary>Steps</summary>

- Market equilibrium and surplus (linear curves): demand_intercept = 100, demand_slope = 2, supply_intercept = -20, supply_slope = 4 → price = 20, quantity = 60, consumer_surplus = 900, producer_surplus = 450, total_surplus = 1350

</details>

**Endowment insurance** · _Actuarial mathematics_

> Find the endowment insurance value with mortality rates = [0.01, 0.02, 0.03], interest rate = 4%, term = 3, benefit = 1000.

→ **apv = 890.4258648, term_part = 53.79672565, pure_endowment = 836.6291392**

<details><summary>Steps</summary>

- Endowment insurance: mortality_rates = [0.01, 0.02, 0.03], interest_rate = 0.04, term = 3, benefit = 1000 → apv = 890.4258648, term_part = 53.79672565, pure_endowment = 836.6291392

</details>

## Figures

Every point was computed by `economics-finance-ai999x`.

**Call option payoff: profit and loss at expiry**

![Call option payoff: profit and loss at expiry — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/black-scholes-payoff.svg)

_Profit and loss at expiry of a European call (strike 100) bought for the Black–Scholes price the package returns, 10.4506 (rate 5 %, volatility 20 %, 1 year): the payoff is the package's value with 10⁻⁶ year left, at 101 prices, minus the premium. The loss (shaded) is capped at the premium; profit starts above the break-even price S ≈ 110.451 (strike + premium = 110.451). Dashed: value before expiry minus the premium._

**Bond price against yield, with duration and convexity**

![Bond price against yield, with duration and convexity — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/bond-price-change-curve.svg)

_Price of a 10-year bond (face 1000, coupon 5 %, 2 payments a year) against its yield, with the duration (tangent) and duration-plus-convexity approximations, all computed by the package for yield changes of ±4 points. Modified duration 7.66505; marked: yield = 6 %, price = 925.613._

**Supply and demand: equilibrium and surpluses**

![Supply and demand: equilibrium and surpluses — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/market-equilibrium-equilibrium.svg)

_Linear demand Q = 100 − 2P and supply Q = -20 + 4P: the package returns the equilibrium P* = 20, Q* = 60, consumer surplus 900 and producer surplus 450 (the shaded triangles; total 1350). Marked: Q = 60, P = 20._

**Loan amortisation: interest and principal in each payment**

![Loan amortisation: interest and principal in each payment — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/loan-payment-split-schedule.svg)

_How the monthly payment of 1199.1 on a loan of 200000 at 6 % over 30 years splits into interest (lower part) and principal (upper part): the first payment of each year, as computed by the package. Year 1: interest 1000, principal 199.101; principal is the larger part from year 20 (interest 578.324). Total interest over the loan: 231676._

**Two-asset portfolios: the risk–return curve**

![Two-asset portfolios: the risk–return curve — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/two-asset-portfolio-frontier.svg)

_Risk and return of every mix of two assets (returns 10 % and 15 %, standard deviations 20 % and 30 %, correlation 0.1), computed by the package at 51 weights. The mix w₁ = 0.6 gives 12 % at σ = 17.7989 %; marked: the minimum-variance mix w₁ = 0.711864, σ = 17.3791 %, return 11.4407 %._

**Minimum-variance portfolio: covariance and weights**

![Minimum-variance portfolio: covariance and weights — computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/global-min-variance-heatmap.svg)

_The global minimum-variance portfolio of three assets with the covariance matrix shown (left): the package returns weights [0.547506, 0.190409, 0.262085] (right), a portfolio standard deviation of 0.160198, below every single asset's (0.2, 0.3, 0.25, each also computed by the package)._

## Capabilities

96 certified procedures. A few of them:

- **Procedures:** Arithmetic and geometric mean return; Beta of an asset from return series; Binomial option price (Cox-Ross-Rubinstein); Black-Scholes Greeks; Black-Scholes option prices; Bond price change from duration and convexity; Bond price from its yield; Break-even and target-profit volume; Capital asset pricing model; Compound annual growth rate; Compound interest; Constant elasticity from a log-log regression; Continuous compounding; Convexity of a bond; Correlation and covariance of two series; Cournot duopoly equilibrium; Cross-price elasticity of demand (midpoint); Cumulative interest of a loan; Discounted payback period; Dividend discount model (Gordon growth); Doubling time; Durbin-Watson statistic; Effective annual rate; Endowment insurance …

`economics_finance_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified economics and finance capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{economics_finance_ai999x_1_0_1,
  title  = {economics-finance-ai999x: an offline, verified economics and finance solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
  url    = {https://huggingface.co/AI999X/economics-finance-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/economics-finance-ai999x/discussions or visit https://ai999x.com.

## Support the project

economics-finance-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/economics-finance-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/economics-finance-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/onnx).
