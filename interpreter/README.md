# FactoryScript Interpreter

```
Usage: factory [OPTIONS] [FILE]

Arguments:
  [FILE]  Conveyor program to execute

Options:
  -b, --benchmark   Print benchmarking information after completion
  -d, --debug...    Increase debug logging level, can be supplied multiple times
      --no-color    Disable colored terminal output
  -h, --help        Print help
  -V, --version     Print version
```

## Debug levels

| Level | Description                 |
| :---: | --------------------------- |
|   1   | Show options                |
|   2   | Show interpreter progress   |
|   3   | Show preprocessor output    |
|   4   | Verbose preprocessor output |
