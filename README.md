Paste ``src.rs`` into your library. Use ``String`` for the return error type.

```
pub fn error() -> Result<(), String> {
    err!("{} Error message.", src!())
}
```

which is the equivalent of 

```
pub fn error() -> Result<(), String> {
  let msg = format!("{} Error message.", src!());
  Err(msg)
}
```
