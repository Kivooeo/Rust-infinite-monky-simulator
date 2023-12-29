# Rust-infinite-monky-simulator

### Background:
Ever had a 4 am epiphany about the [Infinite Monkey Theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)? That was me, wondering if this proverbial monkey could actually type anything. I got curious about how long it might take to generate something as simple as the minimal code to successfully execute `cargo run`. I couldn't wait — I tried for about 4-5 hours before I hit my limit.

### Motivation:
Maybe you've pondered the idea of completely random generation using a good old monkey and wondered how long it would take to recreate your favorite phrase using [pseudorandom number generation](https://en.wikipedia.org/wiki/Pseudorandom_number_generator).

### How it looks -- Examles:

#### Log file:
```log
[Times:  0] ::: [Attemp at this time: 1      ] ::: [Current text: f            ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 3      ] ::: [Current text: fn           ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 42     ] ::: [Current text: fn           ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 2938   ] ::: [Current text: fn m         ] ::: [Current time: 29/12/2023 03:01:49]
[Times:  0] ::: [Attemp at this time: 77939  ] ::: [Current text: fn ma        ] ::: [Current time: 29/12/2023 03:01:59]
[Times:  0] ::: [Attemp at this time: 103591 ] ::: [Current text: fn mai       ] ::: [Current time: 29/12/2023 03:02:02]
```

#### Terminal output:
```bash
attemps: 34076 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34077 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34078 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34079 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34080 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34081 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34082 ::: max: fn ::: time: 29/12/2023 03:04:18
```

### Important things:
* To generate your own phrases, modify the constants within the source code.
* You can also alter the path and name of the log file.
* Remember to change the alphabet of unique symbols used in your desired phrase. You can utilize the built-in function `create_alphabet` for this purpose.

### Contributing:
#### I'll be thrilled if you:

* aim to enhance the code's quality and readability
* propose new and interesting functionalities


### Conclusion:
---
Created more as a playful endeavor than a serious project, this repository is inspired by the whimsical notion of an infinite monkey. There's no profound scientific pursuit here, but rather room for creativity, experimentation, and amusement. Let's have some fun, and perhaps amidst this lighthearted process, someone might stumble upon something intriguing or inspiring!
