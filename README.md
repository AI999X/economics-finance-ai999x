# economics-finance-ai999x: offline, verified economics and finance solver with step-by-step answers

[![tests](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/economics-finance-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/economics-finance-ai999x)](https://pypi.org/project/economics-finance-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/economics-finance-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/economics-finance-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/economics-finance-ai999x/)** · **[Guides](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/economics-finance-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Economics & Finance** · Tasks: Question Answering

economics-finance-ai999x is an offline, deterministic economics and finance solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers economics and finance questions with verified results from 105 certified capabilities, gives 453 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**economics-finance-ai999x** reads a question in plain English (or a typed call), picks a certified economics and finance capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** economics-finance-ai999x comes as the AI999X desktop app and as the command-line program `economics-finance` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Economics and Finance desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-economics-finance-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-economics-finance-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`economics-finance`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install economics-finance-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Economics and Finance** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Economics and Finance among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-economics-finance-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-economics-finance-1.5.0-linux-x86_64.tar.gz
./ai999x-economics-finance-1.5.0-linux-x86_64/ai999x-economics-finance
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
economics-finance --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
economics-finance --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ economics-finance ask "Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30."
Read as: Loan payment (amortizing loan)
payment = 1199.10105
number_of_payments = 360
total_paid = 431676.3781
total_interest = 231676.3781
  by: Loan payment (amortizing loan)  (PMT = P i / (1 - (1 + i)^-n), i = annual rate / m, n = years x m)
  annual_rate = 0.06
  principal = 200000
  years = 30
  assumes: principal > 0; 0 <= annual_rate < 1; years x payments_per_year a whole number, years at most 1000
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ economics-finance info loan_payment
Loan payment (amortizing loan)
  PMT = P i / (1 - (1 + i)^-n), i = annual rate / m, n = years x m
  in   principal amount borrowed [1]
  in   annual_rate nominal annual interest rate (decimal), compounded at the payment frequency [1]
  in   years    term of the loan in years [1]
  in   payments_per_year payments per year (12 = monthly, default) (integer) [1]
  out  payment  level payment per period [1]
  out  number_of_payments number of payments n = years x m [1]
  out  total_paid payment x n [1]
  out  total_interest total paid - principal [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ economics-finance solve loan_payment principal=200000.0 annual_rate=0.06 years=30.0
payment = 1199.10105
number_of_payments = 360
total_paid = 431676.3781
total_interest = 231676.3781
  by: Loan payment (amortizing loan)  (PMT = P i / (1 - (1 + i)^-n), i = annual rate / m, n = years x m)
  principal = 200000
  annual_rate = 0.06
  years = 30
  assumes: principal > 0; 0 <= annual_rate < 1; years x payments_per_year a whole number, years at most 1000
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | economics-finance batch

`cases.jsonl`:

```json
{"key": "loan_payment", "inputs": {"principal": 200000.0, "annual_rate": 0.06, "years": 30.0}}
{"key": "irr", "inputs": {"cash_flows": "[-1000, 300, 400, 500]"}}
```

```console
$ economics-finance batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":1199.1010503055138,"unit":"","solved_for":"payment","si":[["principal",200000.0],["annual_rate",0.06],["years",30.0]],"values":[["payment",1199.1010503055138,""],["number_of_payments",360,""],["total_paid",431676.378109985,""],["total_interest",231676.37810998497,""]],"target":"payment","phrase":"level payment per period","assumptions":["principal > 0","0 <= annual_rate < 1","years x payments_per_year a whole number, years at most 1000"],"name":"Loan payment (amortizing loan)","equation":"PMT = P i / (1 - (1 + i)^-n), i = annual rate / m, n = years x m"}
{"outcome":"SOLVED","message":"","value":0.08896339469334996,"unit":"","solved_for":"irr","values":[["irr",0.08896339469334996,""]],"target":"irr","phrase":"internal rate of return per period","assumptions":["the cash flows change sign exactly once (otherwise the IRR is not unique and none is given)"],"name":"Internal rate of return","equation":"bisection on v = 1/(1 + r) for the unique positive root of sum CFt v^t, Newton polishing"}
```

### In your browser

Open [Economics and Finance on ai999x.com](https://www.ai999x.com/models/economics-finance-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `economics_finance_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install economics-finance-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Financial ratio analysis with the DuPont return on equity

Liquidity (current, quick), solvency (debt ratio, debt to equity), efficiency (asset turnover) and profitability (net margin, ROA, ROE) ratios from balance-sheet and income-statement items, with ROE broken into its DuPont factors: margin x asset turnover x equity multiplier.

Command line:

```console
$ economics-finance solve financial_ratios sales=500000 net_income=40000 total_assets=400000 total_equity=160000 current_assets=150000 current_liabilities=100000 inventory=60000
current_ratio = 1.5
quick_ratio = 0.9
debt_ratio = 0.6
debt_to_equity = 1.5
net_profit_margin = 0.08
asset_turnover = 1.25
equity_multiplier = 2.5
roa = 0.1
roe = 0.25
  by: Financial ratio analysis with the DuPont return on equity  (ratios of the stated items; liabilities = assets - equity; ROE = (NI/S)(S/TA)(TA/E))
  sales = 500000
  net_income = 40000
  total_assets = 400000
  total_equity = 160000
  current_assets = 150000
  current_liabilities = 100000
  inventory = 60000
  assumes: sales, total assets, equity, current liabilities > 0; equity <= total assets; inventory <= current assets <= total assets; current liabilities <= total assets - equity
```

Python:

```python
import economics_finance_ai999x as m

a = m.solve("financial_ratios", sales="500000", net_income="40000", total_assets="400000", total_equity="160000", current_assets="150000", current_liabilities="100000", inventory="60000")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#cap-financial_ratios) and press **Compute in your browser**.

### Inventory costing: FIFO, LIFO and weighted average

Cost of goods sold and ending inventory under FIFO, LIFO and the weighted average cost (periodic system) from the cost layers of the period and the units sold, with the gross margin when a selling price is given.

Command line:

```console
$ economics-finance solve inventory_costing "layer_units=[100, 200, 150]" "layer_costs=[10, 12, 14]" units_sold=300 selling_price=20
units_available = 450
cost_of_goods_available = 5500
fifo_cogs = 3400
fifo_ending_inventory = 2100
lifo_cogs = 3900
lifo_ending_inventory = 1600
average_unit_cost = 12.22222222
average_cogs = 3666.666667
average_ending_inventory = 1833.333333
sales_revenue = 6000
fifo_gross_margin = 2600
lifo_gross_margin = 2100
average_gross_margin = 2333.333333
  by: Inventory costing: FIFO, LIFO and weighted average  (periodic system: FIFO takes the units sold from the first layers, LIFO from the last; weighted average cost = sum(units x cost) / sum(units); ending inventory = the units left at their costs)
  units_sold = 300
  selling_price = 20
  assumes: one unit cost per layer; units of each layer > 0; unit costs >= 0; 0 <= units sold <= units available; selling price >= 0
```

Python:

```python
import economics_finance_ai999x as m

a = m.solve("inventory_costing", layer_units=[100, 200, 150], layer_costs=[10, 12, 14], units_sold="300", selling_price="20")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#cap-inventory_costing) and press **Compute in your browser**.

### Overhead variances: variable spending and efficiency, fixed budget and volume

The four overhead variances of a standard costing system with overhead applied on hours: variable overhead spending and efficiency, fixed overhead budget (spending) and volume, and the total over- or under-applied overhead, each marked favourable or unfavourable.

Command line:

```console
$ economics-finance solve overhead_cost_variances variable_overhead_rate=2 actual_variable_overhead=9000 actual_hours=4200 standard_hours=4000 budgeted_fixed_overhead=12000 actual_fixed_overhead=12500 denominator_hours=5000
variable_spending_variance = 600
variable_efficiency_variance = 400
fixed_budget_variance = 500
fixed_volume_variance = 2400
total_overhead_variance = 3900
fixed_overhead_rate = 2.4
applied_overhead = 17600
variable_spending_type = unfavourable
variable_efficiency_type = unfavourable
fixed_budget_type = unfavourable
fixed_volume_type = unfavourable
total_type = unfavourable
  by: Overhead variances: variable spending and efficiency, fixed budget and volume  (spending = AVOH - AH VR; efficiency = VR (AH - SH); budget = AFOH - BFOH; volume = BFOH (DH - SH)/DH; total = AVOH + AFOH - SH (VR + BFOH/DH))
  variable_overhead_rate = 2
  actual_variable_overhead = 9000
  actual_hours = 4200
  standard_hours = 4000
  budgeted_fixed_overhead = 12000
  actual_fixed_overhead = 12500
  denominator_hours = 5000
  assumes: amounts, rates and hours >= 0; denominator hours > 0
```

Python:

```python
import economics_finance_ai999x as m

a = m.solve("overhead_cost_variances", variable_overhead_rate="2", actual_variable_overhead="9000", actual_hours="4200", standard_hours="4000", budgeted_fixed_overhead="12000", actual_fixed_overhead="12500", denominator_hours="5000")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#cap-overhead_cost_variances) and press **Compute in your browser**.

### Standard-cost variances: materials price and quantity, labour rate and efficiency

Price (labour: rate) and quantity (labour: efficiency) variances of direct materials or direct labour from standard and actual prices and quantities, each marked favourable or unfavourable.

Command line:

```console
$ economics-finance solve direct_cost_variances standard_price=4 standard_quantity=2000 actual_price=4.2 actual_quantity=2100
price_variance = 420
quantity_variance = 400
total_variance = 820
price_variance_type = unfavourable
quantity_variance_type = unfavourable
total_variance_type = unfavourable
standard_cost_allowed = 8000
  by: Standard-cost variances: materials price and quantity, labour rate and efficiency  (PV = QP (AP - SP), QV = SP (AQ - SQ), total = PV + QV; positive = unfavourable)
  standard_price = 4
  standard_quantity = 2000
  actual_price = 4.2
  actual_quantity = 2100
  assumes: standard price and standard quantity > 0; actual price, actual quantity and quantity purchased >= 0
```

Python:

```python
import economics_finance_ai999x as m

a = m.solve("direct_cost_variances", standard_price="4", standard_quantity="2000", actual_price="4.2", actual_quantity="2100")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/economics-finance-ai999x/#cap-direct_cost_variances) and press **Compute in your browser**.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/economics-finance-ai999x/blob/main/docs/economics_finance_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/economics-finance-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified economics and finance capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 453 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `economics-finance` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics.

## Model overview

| Property | Value |
|---|---|
| Package | `economics-finance-ai999x` 1.6.0 (import `economics_finance_ai999x`) |
| Type | Deterministic neuro-symbolic economics and finance engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 105 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 453 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `economics-finance-ai999x "<question>"`; for the native program use `economics-finance` (above). The outputs above were produced by running this exact code on economics-finance-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

**Standard-cost variances: materials price and quantity, labour rate and efficiency** · _Accounting_

> Find the materials price and quantity variances with standard price = 4, standard quantity = 2000, actual price = 4.2, actual quantity = 2100.

→ **price_variance = 420, quantity_variance = 400, total_variance = 820, price_variance_type = unfavourable, quantity_variance_type = unfavourable, total_variance_type = unfavourable, standard_cost_allowed = 8000**

<details><summary>Steps</summary>

- Standard-cost variances: materials price and quantity, labour rate and efficiency: standard_price = 4, standard_quantity = 2000, actual_price = 4.2, actual_quantity = 2100 → price_variance = 420, quantity_variance = 400, total_variance = 820, price_variance_type = unfavourable, quantity_variance_type = unfavourable, total_variance_type = unfavourable, standard_cost_allowed = 8000

</details>

## Figures

Every point was computed by `economics-finance-ai999x`.

**Call option payoff: profit and loss at expiry**

![Call option payoff: profit and loss at expiry, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/black-scholes-payoff.svg)

_Profit and loss at expiry of a European call (strike 100) bought for the Black–Scholes price the package returns, 10.4506 (rate 5 %, volatility 20 %, 1 year): the payoff is the package's value with 10⁻⁶ year left, at 101 prices, minus the premium. The loss (shaded) is capped at the premium; profit starts above the break-even price S ≈ 110.451 (strike + premium = 110.451). Dashed: value before expiry minus the premium._

**Bond price against yield, with duration and convexity**

![Bond price against yield, with duration and convexity, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/bond-price-change-curve.svg)

_Price of a 10-year bond (face 1000, coupon 5 %, 2 payments a year) against its yield, with the duration (tangent) and duration-plus-convexity approximations, all computed by the package for yield changes of ±4 points. Modified duration 7.66505; marked: yield = 6 %, price = 925.613._

**Supply and demand: equilibrium and surpluses**

![Supply and demand: equilibrium and surpluses, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/market-equilibrium-equilibrium.svg)

_Linear demand Q = 100 − 2P and supply Q = -20 + 4P: the package returns the equilibrium P* = 20, Q* = 60, consumer surplus 900 and producer surplus 450 (the shaded triangles; total 1350). Marked: Q = 60, P = 20._

**Loan amortisation: interest and principal in each payment**

![Loan amortisation: interest and principal in each payment, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/loan-payment-split-schedule.svg)

_How the monthly payment of 1199.1 on a loan of 200000 at 6 % over 30 years splits into interest (lower part) and principal (upper part): the first payment of each year, as computed by the package. Year 1: interest 1000, principal 199.101; principal is the larger part from year 20 (interest 578.324). Total interest over the loan: 231676._

**Two-asset portfolios: the risk–return curve**

![Two-asset portfolios: the risk–return curve, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/two-asset-portfolio-frontier.svg)

_Risk and return of every mix of two assets (returns 10 % and 15 %, standard deviations 20 % and 30 %, correlation 0.1), computed by the package at 51 weights. The mix w₁ = 0.6 gives 12 % at σ = 17.7989 %; marked: the minimum-variance mix w₁ = 0.711864, σ = 17.3791 %, return 11.4407 %._

**Minimum-variance portfolio: covariance and weights**

![Minimum-variance portfolio: covariance and weights, computed by economics-finance-ai999x](https://huggingface.co/AI999X/economics-finance-ai999x/resolve/main/figures/global-min-variance-heatmap.svg)

_The global minimum-variance portfolio of three assets with the covariance matrix shown (left): the package returns weights [0.547506, 0.190409, 0.262085] (right), a portfolio standard deviation of 0.160198, below every single asset's (0.2, 0.3, 0.25, each also computed by the package)._

## Capabilities

105 certified procedures. A few of them:

- **Procedures:** Arithmetic and geometric mean return; Beta of an asset from return series; Binomial option price (Cox-Ross-Rubinstein); Black-Scholes Greeks; Black-Scholes option prices; Bond price change from duration and convexity; Bond price from its yield; Break-even and target-profit volume; Capital asset pricing model; Comparative advantage and opportunity cost; Compound annual growth rate; Compound interest; Constant elasticity from a log-log regression; Continuous compounding; Convexity of a bond; Correlation and covariance of two series; Cost curves, best output and shutdown of a competitive firm; Cournot duopoly equilibrium; Credit card finance charge (average daily balance); Cross-price elasticity of demand (midpoint); Cumulative interest of a loan; Discounted payback period; Dividend discount model (Gordon growth); Doubling time …

`economics_finance_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified economics and finance capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{economics_finance_ai999x_1_6_0,
  title  = {economics-finance-ai999x: an offline, verified economics and finance solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/economics-finance-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/economics-finance-ai999x/discussions or visit https://ai999x.com.

## Support the project

economics-finance-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/economics-finance-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/economics-finance-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `economics-finance` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/economics-finance`) and its data (`rust/models/economics_finance/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p economics-finance-ai999x
./target/release/economics-finance --help
cargo test -p economics-finance-ai999x
```
