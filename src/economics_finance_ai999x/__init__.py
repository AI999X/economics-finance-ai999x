"""economics-finance-ai999x — offline economics and finance answers with their formula, steps and verification.

    import economics_finance_ai999x as m
    m.ask("<a question in words>").show()
    m.formulas()                       # every certified capability

96 certified capabilities: time value of money (simple, compound and continuous interest, effective and nominal rates,
present and future values, annuities ordinary and due, perpetuities, growing annuities, CAGR), loans (payment,
remaining balance, interest and principal split, cumulative interest), capital budgeting (NPV, IRR when unique, MIRR,
payback, profitability index, equivalent annual annuity, crossover rate), bonds (price, yield to maturity, duration,
convexity, zero-coupon, price change, forward rates), stocks and options (Gordon and two-stage dividend discount,
CAPM, WACC, Black-Scholes prices and Greeks, implied volatility, put-call parity, binomial trees, forward prices),
portfolio theory (beta, Sharpe, Sortino, Treynor, Jensen, information ratio, portfolio risk, minimum-variance and
tangency portfolios), risk measures (historical and parametric VaR and expected shortfall, portfolio VaR, maximum
drawdown), actuarial mathematics (survival, life expectancy, life insurance, annuities, net premiums), microeconomics
(elasticities, equilibrium and surplus, tax incidence, monopoly, Cournot, Stackelberg, consumer choice, break-even,
Gini, Herfindahl), macroeconomics (GDP deflator, price indices, Fisher equation, inflation, money and spending
multipliers, Solow model) and econometrics (OLS, multiple regression, log-log elasticity, correlation, Durbin-Watson).

    m.ask("Find the monthly payment on a loan with principal = 200000, annual rate = 6%, years = 30.").show()
    m.solve("black_scholes", spot=100, strike=100, time=1, rate=0.05, volatility=0.2).values   # call 10.4506, put 5.5735

Questions are read in one documented form: a phrase naming the capability, then `name = value` pairs (rates as 0.05 or
5%, lists in brackets). Conventions (payment timing, compounding, day count) are stated by each capability. This is
quantitative education, not financial advice. Built by the AI999X model machine from the same engine as every
domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("economics_finance", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
