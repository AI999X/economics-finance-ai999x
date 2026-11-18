# Economics and Finance: the desktop app, user guide

**Economics and Finance**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Economics and Finance covers time value of money, loans and annuities, bonds, stocks and options, portfolio theory, risk measures, actuarial mathematics, microeconomics, macroeconomics and econometrics. Its desktop app holds all 105 calculations in 15 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-economics-finance-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-economics-finance-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-economics-finance-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-economics-finance.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-economics-finance-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-economics-finance-1.6.5-linux-x86_64.tar.gz
./ai999x-economics-finance-1.6.5-linux-x86_64/ai999x-economics-finance
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Economics and Finance app, opened on Loan payment (amortizing loan) with its worked example.](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero.webp)

*The Economics and Finance app, opened on Loan payment (amortizing loan) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Loans and annuities › Loan payment (amortizing loan) (or type part of its name in the search box). It opens with a worked example filled in: “Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.” The answer: payment = 1199.10105, number_of_payments = 360, total_paid = 431676.3781, total_interest = 231676.3781.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Loan payment (amortizing loan): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-1.webp)

*Loan payment (amortizing loan): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Loan payment (amortizing loan)
payment = 1199.10105
number_of_payments = 360
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Internal rate of return](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-2.webp)

*Internal rate of return*

![Black-Scholes option prices](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-3.webp)

*Black-Scholes option prices*

![Bond price from its yield](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-4.webp)

*Bond price from its yield*

![Market equilibrium and surplus (linear curves)](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-5.webp)

*Market equilibrium and surplus (linear curves)*

![Standard-cost variances: materials price and quantity, labour rate and efficiency](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-6.webp)

*Standard-cost variances: materials price and quantity, labour rate and efficiency*

![Endowment insurance](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-7.webp)

*Endowment insurance*

![Credit card finance charge (average daily balance)](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/scene-8.webp)

*Credit card finance charge (average daily balance)*

![Loan payment (amortizing loan): a chart, enlarged.](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-1.webp)

*Loan payment (amortizing loan): a chart, enlarged.*

![Internal rate of return: a chart, enlarged.](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/chart-2.webp)

*Internal rate of return: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Economics and Finance app in the dark theme.](https://www.ai999x.com/?ax_shot=economics-finance-ai999x/hero-dark.webp)

*The Economics and Finance app in the dark theme.*

## 7. Every calculation

105 calculations in 15 topics; each has its own entry on the model's page (https://www.ai999x.com/models/economics-finance-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Accounting | Financial ratio analysis with the DuPont return on equity, Inventory costing: FIFO, LIFO and weighted average, Overhead variances: variable spending and efficiency, fixed budget and volume, Standard-cost variances: materials price and quantity, labour rate and efficiency |
| Actuarial mathematics | Endowment insurance, Life expectancy from a life table, Net annual premium for whole life insurance, Survival and death probabilities from a life table, Temporary life annuity-due, Term life insurance (net single premium), Whole life annuity-due, Whole life insurance (net single premium) |
| Bonds | Bond price change from duration and convexity, Bond price from its yield, Convexity of a bond, Implied forward rate from spot rates, Macaulay and modified duration of a bond, Price of a zero-coupon bond, Yield to maturity of a bond |
| Capital budgeting | Discounted payback period, Equivalent annual annuity, Internal rate of return, Modified internal rate of return, Net present value, NPV crossover rate of two projects, Payback period, Profitability index |
| Consumer finance | Credit card finance charge (average daily balance), Income tax from progressive brackets |
| Econometrics | Constant elasticity from a log-log regression, Correlation and covariance of two series, Durbin-Watson statistic, Multiple linear regression (OLS), Simple linear regression (OLS) |
| International trade | Comparative advantage and opportunity cost |
| Loans and annuities | Cumulative interest of a loan, Interest and principal in a loan payment, Loan payment (amortizing loan), Number of payments to repay a loan, Remaining balance of a loan |
| Macroeconomics | Inflation from a price index series, Keynesian spending multiplier, Laspeyres, Paasche and Fisher price indices, Money multiplier, Nominal GDP, real GDP and the GDP deflator, Real interest rate (Fisher equation), Solow model steady state, Unemployment rate and labour-force participation |
| Microeconomics | Break-even and target-profit volume, Cost curves, best output and shutdown of a competitive firm, Cournot duopoly equilibrium, Cross-price elasticity of demand (midpoint), Gini coefficient, Herfindahl-Hirschman index and concentration ratio, Income elasticity of demand (midpoint), Market equilibrium and surplus (linear curves), Monopoly profit maximization, Per-unit tax incidence (linear curves), Point elasticity of a linear demand curve, Price elasticity of demand (midpoint), Stackelberg duopoly equilibrium, Utility-maximising demand (Cobb-Douglas) |
| Portfolio theory | Arithmetic and geometric mean return, Beta of an asset from return series, Expected return and risk from scenarios, Global minimum-variance portfolio, Information ratio and tracking error, Minimum-variance portfolio of two assets, Portfolio expected return and risk, Sharpe ratio, Sortino ratio, Tangency (maximum Sharpe ratio) portfolio, Treynor ratio and Jensen's alpha, Two-asset portfolio return and risk |
| Risk measures | Historical value at risk and expected shortfall, Maximum drawdown, Parametric (normal) value at risk and expected shortfall, Portfolio value at risk (variance-covariance) |
| Stocks | Capital asset pricing model, Dividend discount model (Gordon growth), Required return implied by the Gordon model, Two-stage dividend discount model, Weighted average cost of capital |
| Stocks and options | Binomial option price (Cox-Ross-Rubinstein), Black-Scholes Greeks, Black-Scholes option prices, Forward price with cost of carry, Forward price with known dividends, Implied volatility, Put-call parity |
| Time value of money | Compound annual growth rate, Compound interest, Continuous compounding, Doubling time, Effective annual rate, Future value of an annuity, Interest rate of an annuity, Nominal rate from an effective rate, Present and future value of a growing annuity, Present value of a growing perpetuity, Present value of a perpetuity, Present value of a single sum, Present value of an annuity, Simple interest, Sinking fund payment |

## 8. Other ways to use it

The same engine runs on the command line (`economics-finance`, its own guide), in Python (`pip install economics-finance-ai999x`, `import economics_finance_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/economics-finance-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/economics-finance-ai999x/
