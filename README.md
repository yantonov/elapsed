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
1 year 2 days (368 days)

elapsed since 2020-01-03 2021-01-02
11 months 29 days (364 days)

elapsed since 2020-01-03 2021-01-31
11 months 58 days (393 days)

elapsed since 2020-01-03 2021-02-01
1 year 28 days (394 days)
```

To understand behaviour the following scenario may help.  
Let's suppose you met your friend sometime in the past (date 1).  
And you meet him/her again, for example today (date 2).  
You want to calculate how many days have you not seen each other (between date 1 and date 2).

Moreover, when you consider months, only complete months between dates matters.

*Update*: to simplify understanding the total number of days strictly between the given dates is also outputed.
