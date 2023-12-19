# Advent of Code 2023 Solutions

[![Build Status](https://github.com/akaritakai/AdventOfCode2023/actions/workflows/main.yml/badge.svg)](https://github.com/akaritakai/AdventOfCode2023/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/akaritakai/AdventOfCode2023.svg)](https://codecov.io/gh/akaritakai/AdventOfCode2023)
![Stars](https://img.shields.io/badge/stars%20⭐-34-yellow)
![Days Completed](https://img.shields.io/badge/days%20completed-17-green)

This repo contains my Advent of Code 2023 solutions in Rust. After providing it with your puzzle inputs (or your
session token), running the program will print out the answers to all days of the puzzle. A Docker image is provided to
ensure compatibility with machines that do not want to install dependencies.

The goal of this repo is to provide fast, highly tested, and easy-to-use solutions.

This repo may see changes in the future to improve runtime. If you have any suggestions, issues running the code, or
find a correctness error: please open an issue or pull request.

### Example output:
```
Day 01 Part 1: 55002
Day 01 Part 2: 55093
Day 02 Part 1: 1734
Day 02 Part 2: 70387
Day 03 Part 1: 556367
Day 03 Part 2: 89471771
Day 04 Part 1: 33950
Day 04 Part 2: 14814534
Day 05 Part 1: 1181555926
Day 05 Part 2: 37806486
Day 06 Part 1: 275724
Day 06 Part 2: 37286485
Day 07 Part 1: 251106089
Day 07 Part 2: 249620106
Day 08 Part 1: 14257
Day 08 Part 2: 16187743689077
Day 09 Part 1: 1684566095
Day 09 Part 2: 1136
Day 10 Part 1: 6733
Day 10 Part 2: 435
Day 11 Part 1: 9957702
Day 11 Part 2: 512240933238
Day 12 Part 1: 7541
Day 12 Part 2: 17485169859432
Day 13 Part 1: 27202
Day 13 Part 2: 41566
Day 14 Part 1: 113486
Day 14 Part 2: 104409
Day 15 Part 1: 511215
Day 15 Part 2: 236057
Day 16 Part 1: 7482
Day 16 Part 2: 7896
Day 17 Part 1: 758
Day 17 Part 2: 892
```

## Docker Instructions

1. Follow the instructions below for providing your puzzle input.
2. Run `docker build -t aoc2023 .`
3. Run `docker run --rm --name aoc2023-run aoc2023`

## Providing Your Puzzle Input

There are two supported methods for inputting your puzzle data into this application.

### Automatic Puzzle Fetcher (via Session Cookie)

First, get your cookie session data.

You will need to log into the Advent of Code website and then inspect your cookies.
If you are using Chrome, you can follow the directions [here](https://developers.google.com/web/tools/chrome-devtools/storage/cookies).

You will be looking for a cookie called `session`. It will contain a long sequence of hexadecimal digits.

Place that data into a file called `cookie.txt` in the project directory.

The application will use that data to automatically fetch your puzzle input for each day.

### Manual Input

This code will also look in a particular location on your local machine for puzzle input.

In the project directory, it will check a directory called `puzzle`.
Within that directory it will expect Day 1's input to be in a file called `01`, Day 2's input to be in a file called `02`, etc.

You can find your puzzle input for a given day by logging into the Advent of Code website and then navigating to the URL
for that puzzle's input.

The URL for your puzzle input will be at:
```
https://adventofcode.com/2023/day/${DAY}/input
```
where `${DAY}` is the day number of the puzzle.

As an example, Day 1's input is at https://adventofcode.com/2023/day/1/input,
Day 2's input is at https://adventofcode.com/2023/day/2/input, etc.
