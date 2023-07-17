# `traitlib`

A rust lib for some util traits.

# Docs

First it re-exports all traits on `std`.

## `ByteSerialize`

For types that can be serialized into bytes.

### `byte_serialize`

The function for serialization into bytes.

## `StrSerialize`

For types that can be serialized into an String.

### `str_serialize`

The function for serialization into an String.

## `JSONSerialize`

For types that can be serialized into JSON.

### `json_serialize`

The function for serialization into JSON.

## `XMLSerialize`

For types that can be serialized into XML.

### `xml_serialize`

The function for serialization into XML.

## `Sortable`

For types that can be sorted.

### `sort_ascending`

Sort in ascending order.

### `sort_descending`

Sort in descending order.

## `Validate`

For types that can be checked if they're valid or not.

### `is_valid`

Check if the object is valid.

## `Execute`

For types that can be executed.

### `exec`

Execute the object.

## `AsyncExecute`

Same as `Execute` but asynchronously.

### `async_exec`

Execute the object asynchronously.

## `Encrypt`

For types that can be encrypted.

### `Output`

The type for the encrypted output.

### `encrypt`

The encrypting function.

### `decrypt`

The decrypting function.

## `Merge`

For objcets that can be merged with another instance.

### `merge`

The merging function.

## `Stream`, our most complex type at the time of writing

For an stream, which implements `Read` and `Write`.

### `copy_to`

Read data from this `Stream` and writes that data to another `Stream`.

### `skip`

Read and discard some bytes from this `Stream`.

### `read_until`

Read bytes from this `Stream` until an specified delimiter is reached.

### `read_line`

Read bytes from this `Stream` until a `\n` is reached.

### `write_flush`

`write` but also `flush`es automatically.

### `write_from_iterator`

Writes data from an iterator to this `Stream`.

### `write_string`

Writes a String to this `Stream`.

## That's not the end

More documentation is comming soon.