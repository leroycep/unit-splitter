# Unit Splitter

Unit splitter is an application that takes an inventory of unit numbers and
a list of requests for units, and then produces a list of filled requests. It
currently has a web interface (https://utils.geemili.xyz/unit-splitter/) and a
command line interface.

## Usage

In the inventory field, we can put a list of units:

```
EVAL=57-58,60-72,74-75,77-78,80-93,95-97,99,101-113
CTRL=21-40
```

These units are separated into two groups, `EVAL` and `CTRL`. Following the
equals sign is the list of units. The units can be simple numbers separated by
commas, like `1,2,3` or `5000,5004`. Unit numbers that are contiguous can be
shortened to simply the first and last unit numbers, separated by a hyphen
(`1-50`).

And in the requests field:

```
A: 10, 5
B: 17, 2
```

Here we have two requests, each requesting units from `EVAL` and `CTRL`.
Request `A` wants 10 units from `EVAL`, and `5` units from `CTRL`. Request `B`
wants 17 units from `EVAL` and 2 from `CTRL`.

You should something like the following in the Output section:

|                    |                                         |
| -----------------: | :-------------------------------------- |
|              **A** | EVAL=57-58,60-67, CTRL=21-25            |
|              **B** | EVAL=68-72,74-75,77-78,80, CTRL=26-30   |
| **Leftover Units** | EVAL=81-93,95-97,99,101-113, CTRL=31-40 |

Nice!

## Project Layout

`./core` contains the "business" logic of the application, including parsing the
inventory and requests string and splitting the units.

`./web` is a frontend built using WebAssembly. It uses the [`yew`][] crate and is
currently the most supported.

`./cli` is a version that can be run from the command line, built using the
excellent [`quicli`][] crate.

The `./gtk` frontend is deprecated. It was built using `gtk` and the [`relm`][]
crate, but I couldn't get it to act the way I wanted it to.

[`yew`]: https://github.com/DenisKolodin/yew
[`quicli`]: https://killercup.github.io/quicli/
[`relm`]: https://github.com/antoyo/relm

## Motivation

I wanted to build something for the web using Rust, and this idea had been 
bounching around in my head as a small project that would be useful to somebody
other than me.

My day job as a reliability technician involves working with a lot of units
like this, and more than once mistakes have been made splitting the units
between tests. I decided that I wanted to make a program that would do the
splitting automatically, and thus we are here.
