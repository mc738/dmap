# dmap

`dmap` is a tool for mapping directories and files and allowing easy ways to compare them.


## Why?

Sometimes you need an easy way to compare directories and their content. `git` or other command line tools/scripts may not always be an option.

Also, because it was an interesting thing to build and I can definitely get some use of out it.

Some use cases include:

* Comparing directories on different computers.
* Comparing and monitoring a directory over time. 
* Auditing directories.

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
  

## Tests

A collection of tests are used to check enhancements/changes do not break anything and `dmap` provides consistence behaviour.

* The integration test suite can be found at [here](https://github.com/mc738/dmap_tests)

* Instructions on the settings it up can he found [here](https://github.com/mc738/dmap_tests/blob/master/SETUP.md)

Please feel free to contribute!