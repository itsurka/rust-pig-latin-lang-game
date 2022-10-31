# Pig Latin Language Game

[Pig Latin Wiki](https://en.wikipedia.org/wiki/Pig_Latin)

Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

I made this application during study the book - [The Rust Programming Language](https://doc.rust-lang.org/book)

## Build and run
```shell
cargo run
```

Example: 

```text
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-pig-latin`
Input words: 
pig latin banana friends smile string eat omelet are
pig => ig-pay
latin => atin-lay
banana => anana-bay
friends => iends-fray
smile => ile-smay
string => ing-stray
eat => eat-hay
omelet => omelet-hay
are => are-hay
```