A long time ago in a galaxy far, far away...

... JSON was the talk of the town. Its grammar [fit on a business card](Readme/json-grammar.jpg),
and everything was good.

... Everybody used JSON, for data exchange and for configuration—configuration files were small,
and everything was good.

But we grew tired of using JSON. We were upset that it was unfit for a task it was not designed for
 And everything wasn't so good anymore.

But we did not fix JSON. Some persevered, some moved to new languages. But these languages grew
deranged. We did not just fix the flaws of JSON, but we overstepped.

I want not 63 types of strings, nor turing-completeness, I want JSON with added convenience.

And Goff will make everything good.

# /ɡɒθ/

A configuration langugae.

## Syntax

```
-- This is a comment
-- It is the only type of comment.
-- Multiline comments can span multiple lines using multiple comments.

-- Goff has types:
: Network
server   = 'example.com' -- String
useProxy = no            -- Boolean
timeout  = 5             -- Integer
proxy    = Nothing       -- Nothing

: Developer
revision = 6.66 -- Real
license  = '
As long as you retain this notice you can do whatever you want with this stuff. If we meet some day,
and you think this stuff is worth it, you can buy me a beer in return.
'                                   -- String (multiline!)
workDays = [ 'Monday'
           , 'Tuesday'
           , 'Wednesday']           -- Lists
hoursWorked = ( 'Monday'    -> 8.
              , 'Tuesday'   -> 7.5
              , 'Wednesday' -> 7.5) -- Map

-- What follows is Goff's lone 'magic' feature, Functions. They are not turing complete, and more
-- play the role of templates.
-- Functions are not represented in the final deserialised data, but may be used anywhere in a Goff
-- document to reduce boilerplate.

+ smallServer                      -- The Function name
| ip, supportsIpv6, bandwidthLimit -- These are fields that must be present when invoking the
                                   -- Function
cpus = 4                           -- These fields are applied automatically
location = 'us-east-1'

+ largeServer
| ip, supportsIpv6
cpus = 8
location = 'us-east-2'

: Server Info
HTTPCache = largeServer (ip -> '100.100.100.100', supportsIpv6 -> yes)
Seedbox   = smallServer (ip -> '200.200.200.200', supportsIpv6 -> no, bandwidthLimit -> Nothing)

```

## Internals (Conformance)

### Validity

All Goff documents that do not strictly conform to the following standards are **invalid**.
Upon encountering an invalid file, the parser implementation should cease parsing and not return any
form of deserialised data. The parser implementation should take adequate meaures to report the
error.

### Non-Type Representation

Non-types are characterised by not being in itself present as dynamic data in the parser's completed
output.

#### Key

A Key is composed of one or more valid, non-whitespace UTF-8 characters.
Keys are used as the constant name for their assigned data.
Keys in Goff are case-insensitive. If the parser is deserialising to a type with dynamic keys,
like a map, keys should be normalised by lowercasing them.

Keys are always followed by zero or more spaces, an equals sign, zero or more spaces and zero or
one newlines, and a value.

#### Struct

A Struct is represented by a line beginning with `:`, zero or more spaces, and one or more valid
UTF-8 characters. Structs are similar to namespaces, in viewing,

```
: Network
server = 'example.com'
```

is perhaps better understood as `network.server = 'example.com'`.

If Keys are present without an associated Struct, they are placed into the global namespace.
Given this contents of the file `config.gf`:

```
server = 'example.com'

: Network
server = 'example.com'
```

The following may be produced:

```
struct Config {
  server: String,
  network: Network,
}

struct Network {
  server: String,
}

...

Config {
  server: "example.com",
  network: Network {
    server: "example.com",
  },
}
```

Struct names may contain spaces, which are replaced with underscores in code. As with Keys, Struct
names should also be normalised by lowercasing them.

Many languages have `struct` types that may be used to represent Structs, or may have similar
constructs under the names like `data`.

#### Comment

A comment begins when the parser encounters two consecutive hyphens outside of a String.
The parser should unconditionally ignore the rest of the line.

### Type Representation

Types are characterised by being valid data to assign a key's value to.
This means that types are only in a valid position following a key.

#### String

A String is composed of zero or more valid UTF-8 characters, surrounded by one single quote on each
side.

When the parser encounters a single quote in a valid position, it should continue to read
until it encounters a second unescaped single quote. This means following it through newlines.

When a backslash is encountered in a String, the parser should identify the following character and
check if it is a valid Goff escape sequence. If it is, the escape sequence should be represented in
the output data as that language's equivalent of the escape sequence. Otherwise, it should be
ignored.

The first newline of a String should be stripped.

When available, parsers should deserialise Strings to their language's string type: `str`, `string`.
In languages without string types, strings are often represented by an array of characters.

### Boolean

A boolean is in one of two states represented by one of two atoms:

- yes
- no

In languages with boolean types (`true`, `false`), Boolean is equivalent.
In languages without boolean types, they should be represented by that language's idiom for
representing true and false values, usually 1 and 0 respectively.

### Integer

An Integer is a whole number, i.e. a number lacking a fractional segment.

The capacity of the Integer is dependant on the type used to represent that key in code. Integers
with a value exceeding the capacity of its key type constitutes an invalid Goff file.

### Real

A Real is a number with a fractional segument.

In definition, a Real may contain irrational numbers, though a parser may safely assume that it will
never encounter a complete Goff file containing an infinite sequence of numbers.

Additionally, Reals may by mathematical definition also match whole numbers. Integer should always
be prioritised over Real.

A Real must contain a period to denote its fractional segment. This means that 8.0 may be
represented by either `8.` or `8.0`, but not `8`.

The capacity of the Real is dependant on the type used to represent that key in code. Reals
with a value exceeding the capacity of its key type constitutes an invalid Goff file.

### List

A List is a collection of zero or more values. They are represented by an opening square bracket
followed by zero or more segments, and a closing square bracket.

A segment is composed of zero or more whitespaces or newlines, a value, zero or more whitespaces or
newlines, and a comma. The trailing comma may be omitted if the given segment is the last segment
of the List.

All values in a List must be the same type.

This definition means all of the following are valid Lists:

```
foo = ['bar',]
bar = [         


                                                                                     6




              ,

]
baz = [[[[[[[,],],],]]]]
```

But the following is not:

```
bar = [, 5,] -- A comma without an associated value
```

Lists are often represented in programming languages by types named `list`, `array`, or `vector`.

### Map

A Map is a collection of zero or more keys with an associated value, represented by an open
parentheses and a segment.

A segment is composed of zero or more whitespaces or newlines, a key (represented by what would be
an otherwise valid value), zero or more whitespaces or newlines, a hyphen symbol directly followed
by a right angle bracket (`->`), zero or more whitespaces or newlines, a value, zero or more
whitespaces or newlines, and a comma.

All keys in a list must be the same type, as must all of the values, but they may be different from
each other.

A Map is often represented in programming languages by types named `map`, `dict`, or `hash`.

### Nothing

A Nothing represents a lack of data. It is technically equivalent to not providing the key at all. 

A Nothing is equivalent in many languages by types named `null` or `nil`. In languages lacking null
types, they may be represented by an enum under names like `Nothing` or `None`. In languages
demanding explicitly nullable types, members that are not explicitly marked as nullable but are
represented by Nothing in the Goff file constitute an invalid Goff file.