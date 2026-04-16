# Algae - Another layout generator

Logical keyboard layout generator. Finds *all* keyboard layouts (or rather sets
of columns) with (d)sfbs below a chosen threshold. For low thresholds does its
job nearly instantly. Should work well as a way to filter out layouts on which
to run slower analyzers. Ones that could for example take sfb distance into
account and make sure sane pinky columns exist.

This is hopefully last rewrite of my generator. This time I cut everything I
could out of it. I could have made it shorter, but I've decided to also try
using 0 dependencies, which came out surprisingly well (did not result in
a disaster).

Edit: started adding basic dependencies back. While my own parser worked the
code was pretty bad. Replaced it with serde_json instead.

While this generator *can* calculate weights of bigrams by itself, I strongly
recommend supplying your own weights. I made sure the calculated weights aren't
super bad, but don't expect more.

## Example usage

With cargo:

```sh
cargo run --release -- --precalculated --file monkeyracer_sfbs.json --cutoff 0.006
```

Without `--file` the weights / corpus is loaded through stdio. Without
`--precalculated` generator calcualtes weights based on provided text as `4 *
sfbs + dsfbs`. `--alphabet` sets (if none is present) or overrides characters
placed on a keyboard. `--preset-left` and `--preset-right` fix displayed
columns and constrain the search up front, so they reduce the amount of work
instead of just filtering results afterward.

Example:

```sh
cargo run --release -- --precalculated --file monkeyracer_sfbs.json --cutoff 0.006 --preset-left "b n x" --preset-right "a u . | o e i | ' / ,"
```

`--preset-left` expects 3 characters in top-to-bottom order for the displayed
leftmost column, so `--preset-left "bnx"` is equivalent to
`--preset-left "b n x"`.

`--preset-right` expects 9 characters in row-major order for the rightmost
3x3 block. Whitespace and `|` separators are ignored, so the example above is
equivalent to `--preset-right "au.oei'/,"`.
Both presets constrain the search up front, instead of filtering finished
layouts afterward. Because Algae fixes key order inside a column by character
frequency, each displayed 3-key column must be ordered with the most frequent
key in the middle, then the top key, then the bottom key.

## Theory

To make exhaustive search feasible we are using two optimizations.

First, we are exploiting various symmetries. We don't distinguish between
layouts with columns swapped or with keys swapped inside a column.

Second, instead of trying to go through all possible layouts ony by one, we look
at groups of layouts instead. Specifically, we place characters on a keyboard
in order of frequency untill we get complete layout or we cross the cutoff, at
which point the whole group gets pruned. Because it's impossible for a layout
to have less sfbs after adding another letter to it, we can be sure that all
layouts we skip are above the cutoff point.

## Fun stats

Number of column sets below percentage of sfbs on monkeyracer corpus. Weights
taken from [cmini repo](https://github.com/Apsu/cmini/). Percentages are
normalized to bigrams containing only letters found on the layout.

|  %  | column sets |
|-----|-------------|
| 0.5 |          70 |
| 0.6 |       14638 |
| 0.7 |      410645 |
| 0.8 |     5584638 |
| 0.9 |    48430687 |
| 1.0 |   299938617 |
