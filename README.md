# EDMIPNG - Encode and Decode Messages In PNG

edmipng is cli app that allows you to store messages inside any png file.

## Usage

There are 4 commands available in edmipng: 
 - `encode <input file> <chunk type> <message> [output file]` - encode `message` inside chunk with `chunk type`, add this chunk to `input file` and save edited png to `output file` (or if it's not provided just edit `input file`)
 - `decode <input file> <chunk type>` - decode message from first chunk with `chunk type` inside `input file`
 - `remove <input file> <chunk type>` - remove first chunk with `chunk type` from `input file`
 - `print <input file>` - display all chunks that potentially can store encoded messages (meaning chunks which chunk type has first two letters lower case and third one upper case)

For example, let's say you have file `my_beautful_cat.png`.

First, let's store "Hello, world!" inside it:
`./edmipng encode my_beautiful_cat.png ruSt 'Hello, world!'`

Now, let's check if chunk was successfully saved:
`./edmipng print my_beautiful_cat.png`
If everything went well you should see:
```
Special chunk types inside file (private + ancillary):
ruSt
```

Let's decode the message:
`./edmipng decode my_beautiful_cat.png ruSt`
You should see:
```
ruSt: Hello, world!
```

Finally, remove the message and check if it was removed:
`./edmipng remove my_beautiful_cat.png ruSt`
`./edmipng print my_beautiful_cat.png`
After running those commands you should see:
```
Special chunk types inside file (private + ancillary):

```
meaning there are no more chunks with chunk type we're looking for.

## How does it work?
As you can see in [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), every png file consists of `chunks`. Each `chunk` has its `chunk type`, which is basically 4 ascii letters. We should focus on two of them:
 - first letter - it tells us if this chunk is critical or ancillary (meaning if it's required for properly displaying image)
 - second letter - it tells us if this chunk is public or private (meaning if chunk is special-purpose png chunk type, or some application-specific for its own use chunk)

The actual letter doesn't matter, but rather if it's lower or upper case. For example, `rust` means the chunk that is ancillary and private. On the other hand, `RUst` is the chunk that is critical and public - you get the idea. 
Actually, two previous chunk types I gave as example are both invalid, as specification says that third letter is reserved and therefore for current png version should always be uppercase (so they should be `ruSt` and `RUSt`).

Idea for the app is pretty simple - we take png file, load all its chunks and look for the ones that have chunk type with first two letters lower case and third one upper case. Then, we check their `data` part of the chunk and if it's valid string than we display it as our encoded message.