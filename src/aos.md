# AOS VXL Format

## File format
Binary, as an array of `BYTE`s. No endianness issues.

Headerless.

Stores the `Collumn` data for each of the 2D map squares. Left-to-right, Top-to-bottom;

Example:  
**First `Collumn`:** (0,0)  
**511th `Collumn`:** (511,0)  
**512th `Collumn`:** (0,1)  
**513th `Collumn`:** (1,1)


### Validation Algorythm
- Check if first 4 bytes are a valid `Span Descriptor`.

## `Collumn` data

**Variable number of `Span`s.**

Consists of repetitions of the `Span` format.  
**Last `Span`is special : ends with `Solid` voxels extending down to z=63.** 

## `Span` data

**Variable `Run`s of voxel data**  

Encoded using **explicit positions (heights)** rather then lenghts.  

Consists of the folowing `Run`s of voxels **in order** unless it's the *last `Span`*:
- 0 or more `Open` voxels.
- 1 or more `Colored` voxels (Top Colored Run).
- 0 or more `Solid` voxels.
- 0 or more `Colored` voxels (Bottom Colored Run).

Last `Span` consists of: 
- 1 or more `Open` voxels.
- 1 or more `Colored` voxels (Top Colored Run).
- *All remaining voxels till `z=63` are **`Solid`***


Stored Top-to-Bottom, (`z=0` to `z=63`).

### Heights
Measured starting from `z=0` (highest position on sky), to `z=63` (lowest position, **must always be `Solid`**, behaves as water).  

`z=62` is Indistructible, lowest non-water ground.

### Data definition

**START of Header**  
At byte offset 0, Length of `Span` data. (N*4 bytes **including** header) [N]  
*Note: `N=0` defines the last `Span` of the `Collumn`*  
At byte offset 1, Starting height of Top Colored Run [S]  
At byte offset 2, Ending height of Top Colored Run [E]  
At byte offset 3, Starting height of Air Run [A]  
*Note: First span ignores value and assumes `A=0`*  
**END of Header**  
**START of Data**  
*Note: Variable `i` is defined as `0..(N-1)`, encoding first colors of Top Colored Run, then encoding colors of Bottom Colored Run*  
At byte offset `(4 + i*4)`, Blue channel for `i`th voxel. [b]  
At byte offset `(5 + i*4)`, Green channel for `i`th voxel. [g]  
At byte offset `(6 + i*4)`, Red channel for `i`th voxel. [r]  
At byte offset `(7 + i*4)`, Alpha channel for `i`th voxel. [a]  
**END of Data**  

Next byte after the list of colors, located at byte offset `(8 + N*4)` is the first byte of the next `Span`.  

### Useful formulas

**Constants**:  
`K` is `E - S + 1`  
`Z` is `(N - 1) - K` or 0, If `N=0` [Length of Bottom Colored Run]  
`M` is `A` at **Next `Span`** or 64, If `N=0`  

#### Determining storage space
Span: N  
Last Span: `4 * (1 + (E - S + 1))`  

#### `Open` voxel run
Start: A  
End: `S-1`  
Length: `S-A`  

#### Top Colored Run
Start: S  
End: E  
Length: `E - S + 1`  

#### Bottom Colored Run
Start: `M - Z`  
End: `M + 1`  
Length: `Z`  

#### `Solid` voxel run
Start: `E + 1`  
End: `M - Z - 1`  
Length: `M - Z - (E + 1)`  

#### `Solid`

## Surface Voxel Rule
**Supply colors for hidden blocks**

## Map Structure

2D Array of `Collumn`s of `voxels`.  
2D Array is `width * height` in size; Current format both are `512`.

Each `Collumn` is stored independently of every other `Collumn`.

### Spawning
Spawn locations are implicit on this version of the file format.

- Blue at `(0,128)` to `(256,384)`
- Red at `(384,128)` to `(512,384)`

### Voxels

Kinds of voxels:
- Open (Represent Air)
- Solid (Represent solid space, **are not** visible)
- Colored/Surface (Are visible, and as such must be drawn with a particular color)
