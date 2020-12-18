# dmap

`dmap` is a tool for mapping directories and files and allowing easy ways to compare them.


# Commands

## Map

DetailsMap a directory and save the result as a `.dmap` file.

* Syntax: `dmap map [path] [output]`
* Args: 
    * `path` - the path of the directory to map.
    * `output` - the path to save the output to.
* Notes:
    * N/A

## Compare

Map a directory, save the results and compare with a previous map.

* Syntax: `dmap compare [path] [map]`
* Args:
    * `path` - the path of the directory to map.
    * `map` - the path of the map to compare results with.

## Diff

Compare the difference between 2 existing maps

* Syntax: `dmap diff [map1] [map2]`
* Args:
    * `map1` - the path to map 1.
    * `map2` - the path to map 2.