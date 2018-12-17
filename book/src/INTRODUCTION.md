# Introduction

Lithium is, first and foremost, a _vision_ for an S-expression based programming environment. Said
environment will comprise an interpreted scripting language, a programming language, and other
domain specific languages all built on a shared and layered set of building blocks and abstractions.

Quoting Thomas A. Edison:

> None of my inventions came by accident. I see a worthwhile need to be met and I make trial after
> trial until it comes. What it boils down to is one percent inspiration and ninety-nine percent
> perspiration.

As such, Lithium will be mostly a collection of ideas taken from other programming languages,
scripting languages and data formats. I would like to thank all the original authors of all ideas
I'm essentially repackaging in a different way. I'm building on the shoulders of giants.

The initial idea of Lithium came around 2013 and the project changed scope (and implementation
language) several times during the years. I used it as an excuse to learn new programming languages
by implementing the parser for what I now call LDN (Lithium Data Notation, shamelessly _inspired_ by
[Extensible Data Notation](https://github.com/edn-format/edn)) in several programming languages. As
of today, said parser has been re-written in Common Lisp, Racket, OCaml, Haskell, C, Python, and
Rust.

The Python version, for example, was originally meant to become learning material to be used when
teaching others about Scheme, Lisp, and other S-expression based languages. That's a purpose that I
would like to preserve with the latest incarnation written in Rust, a language that is significantly
harder for people to learn, but that is gaining traction at an amazing pace and has a set of
concepts worth learning for programmers at every level of expertise.

As such, the Rust bootstrap interpreter and compiler will be written in a rather simple dialect of
Rust, without making use of its most advanced and Haskell-ish features, to maximize maintainability
and readability.

There is no pretense that this will be come a real, usable, programming language. It will probably
always remain a testbed for ideas and a learning platform to study programming language design and
implementation. I hope you will find it as useful as it was for me.
