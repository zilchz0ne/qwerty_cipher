# qwerty_cipher

During touch typing, if the hand is shifted from the home row position, the typing becomes "gibberh". This tool is to simulate that, and more importantly, it helps to get the pure text back from the gibberish text.

## Usage:
```bash
qwerty_cipher encrypt -l 2 -r 1 "hello world"
ciphertext = jt;;p rpy;g
cipherkey = 10111101010
```

```bash
qwerty_cipher decrypt -l 2 -r 1 -k 10111101010 "jt;;p rpy;g"
hello world
```

```bash
qwerty_cipher decrypt -l 2 -r 1 "jt;;p rpy;g"
plaintext left = ge~~~~w~r~d
plaintext right = h~llo ~o~l~
plaintext guaranteed = ~ello world
```
