# economics-finance-ai999x — offline, verified economics and finance solver with step-by-step answers

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

**[Install](https://pypi.org/project/economics-finance-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

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
| Package | `economics-finance-ai999x` 1.0.0 (import `economics_finance_ai999x`) |
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

Command line: `economics-finance-ai999x "<question>"`, `economics-finance-ai999x --capabilities`. The outputs above were produced by running this exact code on economics-finance-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{economics_finance_ai999x_1_0_0,
  title  = {economics-finance-ai999x: an offline, verified economics and finance solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
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
