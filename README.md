PNG encoder and decoder in rust. Adds redundant chunks to the bytecode of a PNG file, which will not alter any visual properties of the image, but is still quite simple to decode.

Best used with a large number of chunks, so that there is more redundancy and less chance that someone will find your flag by opening the PNG file in a text editor. 
```
Possible Commands:
- pngenc encode <filepath> <chunk type> <message> [output file]
- pngenc decode <filepath> <chunk type>
- pngenc remove <filepath> <chunk type>
- pngenc print <filepath>
```