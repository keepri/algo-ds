# Algorithms and Data Structures

## Sorting
- bubble sort
- quick sort
- insertion sort

## Searching
- linear search
- binary search
- two crystal balls to find breaking point index

## Recursion
- maze walker

| # | # | # | # | S | # | # | # | # | # |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| # |   |   | # |   |   |   |   |   | # |
| # | # |   | # |   | # | # | # |   | # |
| # |   |   | # |   |   | # |   |   | # |
| # |   | # | # | # | # | # |   |   | # |
| # |   |   |   |   |   |   |   |   | # |
| # | E | # | # | # | # | # | # | # | # |

##### Legend:
- S - start
- E - end
- \# - wall
- i - visited
- • - path

### Optional
`cargo run <usize> <bool>` arguments
- `usize` - custom sized Vec used for sort/search, default is 25000
- `bool` - disable logging, default is `true`
