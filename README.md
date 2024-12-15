# FactoryScript 🏭

[![Cargo tests](https://github.com/jaxsonp/FactoryScript/actions/workflows/rust.yml/badge.svg)](https://github.com/jaxsonp/FactoryScript/actions/workflows/rust.yml)
[![Documentation Status](https://readthedocs.org/projects/factoryscript/badge/?version=latest)](https://factoryscript.readthedocs.io/en/latest/?badge=latest)

The world's number one M.O.P. (Manufacturing Oriented Programming) language, FactoryScript is a dynamically typed, interpreted programming language inspired by the magnificence of the industrial revolution.

This repository contains:

- `docs/`: Documentation files (Incomplete)
- `examples/`: Directory containing some FactoryScript code examples
- `interpreter/`: Cargo package containing the canonical FactoryScript Interpreter

### Table of Contents

- [Language Overview](#language-overview)
  - [Syntax](#syntax)
  - [Examples](#examples)
- [Build Instructions](#build-instructions)

## Language Overview

FactoryScript was motivated by the elegance and efficiency of factories and the modern manufacturing process. At it's core, FactoryScript has three primary concepts. Pallets, which contain data, which are moved around by conveyor belts to different stations which perform operations on the pallets. FactoryScript code describes the layout and connections of these elements.

For the complete reference, check out the [full documentation](https://factoryscript.readthedocs.io/en/latest/) (incomplete).

**Stations:** In general, stations are defined with square brackets, and contain a single ASCII, non-whitespace identifier, such as `[println]`, `[>=]`, or `[exit]`. There is one exception to this syntax, assign stations. They are defined with curly brackets, and contain literals to be assigned to pallets. Examples include `{"abc"}`, `{true}`, and `{4.025}`

**Conveyor Belts:** Conveyor belts are represented using contiguous Unicode [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_characters). Conveyor belts are omni-directional, but must be attached on both ends to a station. The beginning end of a conveyor belt is drawn with double line characters (`║`, `═`, `╗`, etc) while the rest of the belt is drawn with single line characters (`│`, `─`, `┐`, etc).

Text that is not a station or a conveyor belt is treated as a comment, being ignored by the interpreter. Below is an annotated hello world program.

```text
spawns an empty   assigns it the string
pallet            literal "hello world"
  v                v
[main]═─{"hello world"}═─[println]
                            ^
                        prints the pallets value
```

```sh
$ factory examples/hello_world.factory
hello world
$
```

However, because FactoryScript is unopinionated about layout, it is possible to reverse the order...

```text
[println]─═{"hello world"}─═[main]
```

... or even make the conveyor belts as unnecessarily convoluted as you want (this does not affect runtime performance).

```text
[main]═─{"hello world"} [println]
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
git clone https://github.com/jaxsonp/FactoryScript.git && cd factory-script/`
```

### To build interpreter:

```sh
cargo build --release --bin factory
```
