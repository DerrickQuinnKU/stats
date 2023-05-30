# Stats

Provides a command line interface for simple statistical analysis:
- Average
- Median
- Variance (sample)
- Standard Deviation (sample)
- Confidence interval for mean (90%, 95%)

## Installation

### Rust/Cargo (required)
Linux:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


### Install 
`cargo install --git https://github.com/DerrickQuinnKU/stats`

## Usage
```
stats --average 1.0 2.0 2.5
// 1.8333333
stats --median 1.0 2.0 2.5
// 2
stats --variance 1.0 2.0 2.5
// 0.5833333
stats --stdev 1.0 2.0 2.5
// 0.7637626
stats --error90 1.0 2.0 2.5
// 0.7253123
stats --error95 1.0 2.0 2.5
//0.86426288
```
To read from a file: `stats --error95 $(cat results.txt)`
