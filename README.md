[![Build Actions Status](https://github.com/yantonov/elapsed/workflows/ci/badge.svg)](https://github.com/yantonov/elapsed/actions)

Calculates number of full days, months, years passed since the given day.

Examples:
```
elapsed since 2020-12-01 2020-12-01
0 days

elapsed since 2020-12-01 2020-12-02
0 days

elapsed since 2020-12-01 2020-12-03
1 day

elapsed since 2020-05-04 2020-05-10
5 days

elapsed since 2019-12-31 2021-01-03
1 year 2 days

elapsed since 2020-01-03 2021-01-02
11 months 29 days

elapsed since 2020-01-03 2021-01-31
11 months 58 days

elapsed since 2020-01-03 2021-02-01
1 year 28 days
```
