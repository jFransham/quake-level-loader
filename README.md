# Quake 3 level viewer in Rust

This is mostly an attempt to wrangle the pointer-arithmetic-based, low-memory-
footprint-oriented BSP file format into something more ideomatically Rustic.
Includes decent texture caching (although work is duplicated if multiple copies
of the same texture are requested simultaneously, this can definitely be
improved) and super fast initial parsing speeds (before transformation - thanks
[Nom](https://github.com/Geal/nom)!). Currently, transformation of the BSP tree
into an ideomatic data structure is by far the biggest performance hit.

Note: if you're thinking of using this in a game project: don't. It isn't ready
and if you want to make a 3D game in Piston then you are more than foolhardy
enough to write your own god-damn BSP loader. This currently panics on basically
any semantic problem with the BSP file (completely nonsensical file data fails
gracefully, but cyclical nodegraphs, illegal array indexing, etc. will cause the
transform function to explode without warning) and this isn't even a library
(yet). Having said that, anything you find here is
[Unlicense](http://unlicense.org)'d and you are welcome to base your crazy ideas
upon it.
