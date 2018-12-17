# Lithium Data Notation

Lithium Data Notation (LDN) is a subset of Rich Hickey's Extensible Data Notation (EDN) format
without direct syntactic support for vectors, maps, sets, and (for now) floating-point numbers.

## Formal Description

The following grammar, in Extended Backus–Naur form (EBNF), describes the Lithium Data Notation
format:

```ebnf
{{#include data/ldn.ebnf}}
```
