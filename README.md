# Advent of code
This repository contains most of my solutions for Advent of Code challenges.

## Project structure
| Directory                   | Description                                            |
|-----------------------------|--------------------------------------------------------|
| `crates`                    | Extra utilities for setting tests etc. up              |
| `src/aoc{year}/`            | Solution for each day I was solving                    |
| `tests/aoc{year}`           | Tests for all days in that specific year               |
| `tests/fixtures/input/`     | Inputs for that specific day (or its test)             |
| `tests/fixtures/solution/`  | The correct values for me (so results can be asserted) |

## Test day macro
To make my life easier, I've crated a macro, that automatically imports
fixtures for selected day, and asserts the solution, so that everything
can be tested using `cargo test`.

### Parameters
The macro requires only two parameters, the year and the day, both of
those need to be literals, so integers, quoted strings etc.

```rust
use advent_of_code::{get_input, get_solution, AOCRunnable};
use aoc_macro::test_day;

test_day!(2023, 1);
```

The macro also takes an optional argument `test_only`, which only generates
the `run_dayX_test`, so if you don't have the solution yet, your tests don't
fail.

```rust
use advent_of_code::{get_input, get_solution, AOCRunnable};
use aoc_macro::test_day;

test_day!(2023, 1, test_only);
```

## License
All code in this repository is licensed under MIT or APACHE 2.0. Any
contribution made shall be dual licensed under those licenses.
