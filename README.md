# FactoryScript 🏭

[![Cargo tests](https://github.com/jaxsonp/factory-script/actions/workflows/rust.yml/badge.svg)](https://github.com/jaxsonp/factory-script/actions/workflows/rust.yml)
[![Documentation Status](https://readthedocs.org/projects/factoryscript/badge/?version=latest)](https://factoryscript.readthedocs.io/en/latest/?badge=latest)

FactoryScript is an interpreted, graph-based esolang themed around factories, inspired by the modern manufacturing process. In a nutshell, FactoryScript programs (factories) are graphs where the nodes (stations) are connected by Unicode box characters (conveyor belts).

This repository contains:

- `docs/`: Documentation files
- `examples/`: Directory containing some FactoryScript code examples
- `interpreter/`: Cargo package containing the canonical FactoryScript interpreter

## Language Overview

_For the complete reference, check out the [full documentation](https://factoryscript.readthedocs.io/en/latest/)_

In its most basic form, a FactoryScript program is simply a graph. Little chunks of data called _pallets_ move around the graph on _conveyor belts_ to and from different nodes, which are called _stations_.

**Pallets** hold morsels of data, such as integers, boolean values, strings, and so on.

**Conveyor belts** are represented using contiguous Unicode [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_characters). The beginning end of a conveyor belt is drawn with double line characters (`║`, `═`, `╗`, etc) while the rest of the belt is drawn with single line characters (`│`, `─`, `┐`, etc).

**Stations** in general are represented using square brackets with non-whitespace identifiers in between, such as `[println]`, `[>=]`, or `[exit]` (Literal assignment is an exception, using curly brackets instead, such as `{3}` or `{false}`). Depending on the type, a station consumes a certain number of input pallets, performs an operation, then optionally produces an output pallet.

Text that is not a station or a conveyor belt is treated as a comment, being ignored by the interpreter. Below is an annotated hello world program.

```text
spawns an empty    assigns it the string
pallet             literal "hello world"
  v                 v
[main]═──{"hello world"}═──[println]
                              ^
                          prints the pallets value
```

```sh
$ factory examples/hello_world.factory
hello world
$
```

FactoryScript is unopinionated about layout, it is possible to reverse the order...

```text
[println]─═{"hello world"}─═[start]
```

... or even make the conveyor belts as unnecessarily convoluted as you want (this does not affect runtime performance).

```text
[start]═─{"hello world"} [println]
┌────────╝               └───────────────────────────────────┐
│  ┌┐  ┌┐       ┌┐ ┌┐          ┌┐  ┌┐              ┌┐    ┌┐  │
│  ││  ││ ┌───┐ ││ ││          ││  ││        ┌┐    ││    ││  │
│  │└──┘│ │ # │ ││ ││ ┌────┐   ││┌┐││ ┌────┐ │└──┐ ││ ┌──┘│  │
│  │┌──┐│ │┌──┘ ││ ││ │ /\ │   ││││││ │ /\ │ │┌─┐│ ││ │| |│  │
│  ││  ││ │└──┐ ││ ││ │ \/ │   │└┘└┘│ │ \/ │ ││ └┘ ││ │|_|│  │
│  ││  └┘ └┐┌─┘ ││ ││ └┐┌──┘   └┐┌──┘ └┐┌──┘ ││    ││ └┐┌─┘  │
└──┘└──────┘└───┘└─┘└──┘└───────┘└─────┘└────┘└────┘└──┘└────┘
```

## Build Instructions

Requires Git and Cargo. First clone and cd into the repository:

```sh
git clone https://github.com/jaxsonp/factory-script.git && cd FactoryScript/
```

### To build interpreter:

```sh
cargo build --release --bin factory
```
