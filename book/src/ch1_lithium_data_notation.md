# Lithium Data Notation

Lithium Data Notation (LDN) is a modified subset of Rich Hickey's [Extensible Data Notation
(EDN)](http://edn-format.org/) format without syntax for vectors, maps, sets, floating-point
numbers, and tagged elements.

## Formal Description

The following grammar is written in Extended Backus–Naur form (EBNF) and describes the LDN format.
EBNF is itself formally described in the [ISO/IEC
14977(E)](http://standards.iso.org/ittf/PubliclyAvailableStandards/s026153_ISO_IEC_14977_1996(E).zip)
standard.

A more succinct description of EBNF can be found on
[Wikipedia](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form).

```
{{#include data/ldn.ebnf}}
```
