# aos-vxl
Rust crate that reads and writes Ace of Spades VXL maps.

The internal representation, `data::VXL` is perfectly equivalent to the documentation found on [piqueserver](https://www.piqueserver.org/aosprotocol/mapformat.html), or on the [aos.md](src/aos.md) file.  
(I had to do my own notes since the original spec was all over the place and was a bit confusing).

Readers and Writers are provided and follow this contract:

`read . write = id` (checked with a selection of common maps from a github repo, this is the `vxl` submodule. Since the Sha512 matches we have bit-perfect identity.)

The APIs are not yet public since I haven't found a neat abstraction for them.  
The main idea is that you should be able to represent a `VXL` internally in whatever way you desire (e.g. a `Vec<Vec<Vec<Voxel>>>`) as long as you implement `From<VXL>` and `Into<VXL>`.
Work on that is yet to be done. 