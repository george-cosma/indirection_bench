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

# After adding LUT:

Settings:

```rs
pub const MODULES: u32 = 10;
pub const FUEL: u32 = 10_000;
pub const JINX_MODULE_SIZE: usize = 200;
```

```py
indirect runtime        time:   [246.91 µs 247.73 µs 248.77 µs]
Found 10 outliers among 100 measurements (10.00%)
Memory usage: 16876 bytes

direct runtime          time:   [22.465 µs 22.500 µs 22.546 µs]
Found 16 outliers among 100 measurements (16.00%)
Memory usage: 13610 bytes

LUT 1                   time:   [50.703 µs 50.782 µs 50.904 µs]
Found 12 outliers among 100 measurements (12.00%)
Memory usage: 15228 bytes

LUT 2                   time:   [47.340 µs 47.381 µs 47.432 µs]
Found 15 outliers among 100 measurements (15.00%)
Memory usage: 15844 bytes

LUT 3                   time:   [52.329 µs 52.482 µs 52.703 µs]
Found 11 outliers among 100 measurements (11.00%)
Memory usage: 15084 bytes
```