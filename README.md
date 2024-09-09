# EDMIPNG - Encode and Decode Messages In PNG

edmipng is cli app that allows you to store messages inside any png file.

## Usage

There are 4 commands available in edmipng: 
**using local files**:
 - `encode <input file> <chunk type> <message> [output file]` - encode `message` inside chunk with `chunk type`, add this chunk to `input file` and save edited png to `output file` (or if it's not provided just edit `input file`)
 - `decode <input file> <chunk type>` - decode message from first chunk with `chunk type` inside `input file`
 - `remove <input file> <chunk type> [output file]` - remove first chunk with `chunk type` from `input file` and save changes inside `output file` (or if it's not provided just edit `input file`)
 - `print <input file>` - display all chunks that potentially can store encoded messages (meaning chunks which chunk type has first two letters lower case and third one upper case) stored inside `<input file>`

**using http**:
 - `encode <url> <chunk type> <message> [output file]` - encode `message` inside chunk with `chunk type`, add this chunk to file downloaded from `url` and save edited png to `output file` (or if it's not provided create new file with name equals `<file_from_url_name>_<current_time_in_epoch>.png`)
 - `decode <url> <chunk type>` - decode message from first chunk with `chunk type` inside file downloaded from `url`
 - `remove <url> <chunk type> [output file]` - remove first chunk with `chunk type` from file downloaded from `url` and save changes inside `output file` (or if it's not provided create new file with name equals `<file_from_url_name>_<current_time_in_epoch>.png`)
  - `print <url>` - display all chunks that potentially can store encoded messages (meaning chunks which chunk type has first two letters lower case and third one upper case) stored inside file downloaded from `url`

To decide whether provided source is `<input file>` or `<url>` the simplest method is used - we firstly check if it points to any exisitng file. If so, we decide that it must be a path. Otherwise we try to convert it into `url`.

When file is automatically created (meaning `[output file]` is not provided) and source is `<url>` (both in `encode` and `remove`) you can manually set directory in which the file should be created by providing environment variable `EDMIPNG_DIR`.

## Example
Let's say you have file `my_beautful_cat.png`.

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

## Note on HTTP mode
As I was testing this feature it turned out that many services remove my custom chunks from uploaded png files - when I sent an image via [messenger](https://www.messenger.com/) it didn't have them. Same goes for uploading to [imgur](https://imgur.com/). However, when I sent file over [gmail](https://mail.google.com/) I was happy to see that chunks were still there. 
It makes this feature a little less useful than I wanted it to be, but for now I don't know if I can do anything about it. It's best usecase right now is probably making downloading png and encoding it one easy step instead of doing one after another.