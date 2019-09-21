# Changelog

  - [v2.3.0](#2.3.0)
  - [v2.2.2](#2.2.2)

# Changes


## [2.3.0] - 2019-09-21 <a name="2.3.0"></a>
### Added
  - Added Mean Square Error distance function
  - Added Manhattan distance function
  - Added Chebyshev distance function
  - Added Canberra distance function
  - Added Hamming distance function
  - Added new distance functions to test pipeline
  - Added new distance functions to readme
  - Added support for node 8 & 9
### Changes
  - Generalized rust knn pipeline function signatures


## [2.2.2] - 2019-09-20 <a name="2.2.2"></a>
### Added
  - Added support for Node 10 & 12
### Fixed
 - Fixed Windows and Mac users not able to download correct system binaries
 - Added `neon clean` step to npm prepublish to avoid packing system specific binaries on publish