# for_loop_with_new vs for_loop_with_vec_capacity vs iterator
## 1 word benchmarking

```shell
for_loop_encoding_vec_new
time:   [117.36 ns 117.57 ns 117.83 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

for_loop_encoding_vec_with_capacity
time:   [115.21 ns 115.64 ns 116.09 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

iterator_encoding       
time:   [121.07 ns 121.36 ns 121.68 ns]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
```

## 1000 words benchmarking

```shell
for_loop_encoding_vec_new
time:   [109.43 µs 109.84 µs 110.25 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild

for_loop_encoding_vec_with_capacity
time:   [106.90 µs 107.86 µs 109.23 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

iterator_encoding       
time:   [122.76 µs 123.29 µs 123.84 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

# for_loop vs iterator
## 1 word benchmarking

### First run

```shell
for_loop_encoding      
time:   [117.89 ns 118.07 ns 118.29 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

iterator_encoding
time:   [121.02 ns 121.21 ns 121.41 ns]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
```

### Second run

```shell
for_loop_encoding       
time:   [119.14 ns 119.48 ns 119.88 ns]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

iterator_encoding
time:   [121.60 ns 121.90 ns 122.24 ns]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
```


## 1000 words benchmarking

### First run

```shell
for_loop_encoding       
time:   [108.26 µs 108.96 µs 109.76 µs]  
Found 1 outliers among 100 measurements (1.00%)  
  1 (1.00%) high severe

iterator_encoding       
time:   [119.25 µs 120.67 µs 122.50 µs]  
Found 9 outliers among 100 measurements (9.00%)  
  5 (5.00%) high mild  
  4 (4.00%) high severe
```

### Second run

```shell
for_loop_encoding       
time:   [108.08 µs 109.24 µs 110.72 µs]  
Found 2 outliers among 100 measurements (2.00%)  
  2 (2.00%) high severe  

iterator_encoding       
time:   [118.01 µs 119.41 µs 121.22 µs]  
Found 2 outliers among 100 measurements (2.00%)  
  2 (2.00%) high severe  
```


### Third run

```shell
for_loop_encoding       
time:   [107.06 µs 107.44 µs 107.87 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

iterator_encoding       
time:   [116.08 µs 116.61 µs 117.17 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
```