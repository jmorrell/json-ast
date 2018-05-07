
# JSON -> AST

A JSON -> AST parser in Rust. You aren't supposed to be here yet.

## TODO:

Cases:

- object trailing comma

```
{
    "a": 1,
}
```

- object missing comma

```
{
    "a": 1
    "b": 2
}
```

- object unquoted strings

```
{
    foo: "bar"
}
```

- array trailing comma

```
[1,]
```

- array missing comma

```
[1, 2 3]

[
    { "foo": 1 }
    { "bar": 2 }
]
```

- invalid literal

```
["x", truth]
```

- strings with single quotes

```
['value']

{
    'foo': 'bar',
}
```

- illegal comments in JSON

```
{
    // this is a value for a
    "a": 1
    /* and this is another type of comment */
}
```

- unterminated string literal

```
{
    "a": "foo
}
```

- invalid number literal

```
[NaN]
[-012]
[- 1]
[Inf]
[0.e1]
```









