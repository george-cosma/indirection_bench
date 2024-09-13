# Indirection Benchmark

Using `HashMap`, MODULES = 10, FUEL = 10_000.

```py
indirect runtime        time:   [41.395 µs 41.707 µs 42.053 µs]
direct runtime          time:   [2.3294 µs 2.3401 µs 2.3520 µs]
```

Using `BTreesMap`, MODULES = 10, FUEL = 10_000.

```py
indirect runtime        time:   [21.897 µs 22.119 µs 22.418 µs]
                        change: [-47.140% -46.697% -46.152%] (p = 0.00 < 0.05)
                        Performance has improved.

direct runtime          time:   [2.2816 µs 2.2936 µs 2.3081 µs]
                        change: [-3.4029% -2.7465% -2.1216%] (p = 0.00 < 0.05)
                        Performance has improved.
```