# Rust-infinite-monky-simulator

### Background:
One day at 4 am, I accidentally remembered [one theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem) and I wondered if this monky could really write `anything`, I wondered how long it would take such basic things as, for example, the minimum code to succesfuly execute `cargo run`, to be honest, I couldn’t wait, my attempts lasted 4-5 hours, that’s the maximum I could do.

### Motivation:
I think that someone might also be interested in finding out how long it will take for the [absolutely random generation](https://en.wikipedia.org/wiki/Pseudorandom_number_generator) of your phrase by a good monky.

### How it looks? Examles:

Log file:
```log
[Times:  0] ::: [Attemp at this time: 1                             ] ::: [Current text: f            ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 3                             ] ::: [Current text: fn           ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 42                            ] ::: [Current text: fn           ] ::: [Current time: 29/12/2023 03:01:48]
[Times:  0] ::: [Attemp at this time: 2938                          ] ::: [Current text: fn m         ] ::: [Current time: 29/12/2023 03:01:49]
[Times:  0] ::: [Attemp at this time: 77939                         ] ::: [Current text: fn ma        ] ::: [Current time: 29/12/2023 03:01:59]
[Times:  0] ::: [Attemp at this time: 103591                        ] ::: [Current text: fn mai       ] ::: [Current time: 29/12/2023 03:02:02]
```

Terminal output:
```bash
attemps: 34076 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34077 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34078 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34079 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34080 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34081 ::: max: fn ::: time: 29/12/2023 03:04:18
attemps: 34082 ::: max: fn ::: time: 29/12/2023 03:04:18
```

## Important things
* So, to use your phrases, you have to change consts inside the source code. 
* You can also change the path to log file and it name.
* Don't forget to change alphabet of unique symbols that contains in your answer phrase. (You can you built-in function `create_alphabet` for this)

### Contributing
##### I will be very happy

* if you want to make the code better or more readable
* if you offer new interesting and useful functionality
