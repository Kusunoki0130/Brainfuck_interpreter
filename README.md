This is a Brainfuck interpreter implemented by Rust.

### How to use

```
.\bfru {your .bf file path}
```

#### example

```brainfuck
// hello.bf
+++++ +++++
[
	>+++++++
	>++++++++++
	>+++
	>+
	<<<<-
]
>++.
>+.+++++++..+++.
>++.
<<+++++++++++++++.
>.+++.------.--------.
>+.
>.
```

In console

```shell
.\bfru hello.bf
```

The output is

```
Ok(['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!', '\n'])
```

---

### Compile error

There are two kind of errors when compiling:

1. `LeftBracketError` : Left bracket is not closed.
2. `RightBracketError` : Right bracket is not expected. 

#### example

```brainfuck
// hello.bf
+++++ +++++
[
	>+++++++
	>++++++++++
	>+++
	>+
	<<<<-
	[
]
>++.
>+.+++++++..+++.
>++.
<<+++++++++++++++.
>.+++.------.--------.
>+.
>.
```

In console

```shell
.\bfru hello.bf
```

The output is

```
Err(CompileError { line: 2, col: 0, error_type: LeftBracketError })
```

---

### TODO

1. Runtime error
2. compiler optimization
3. JIT

