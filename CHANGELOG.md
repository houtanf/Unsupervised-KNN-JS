# Changelog

  - [v2.5.0](#2.5.0)
    - [v2.5.1](#2.5.1)
  - [v2.4.0](#2.4.0)
  - [v2.3.0](#2.3.0)
    - [v2.3.1](#2.3.1)
  - [v2.2.2](#2.2.2)

# Changes


## [2.5.1] - 2019-09-28 <a name="2.5.1"></a>
### Added
  - Support for Node 13

### Changed
  - Changed cloning of entire distance vector on iteration when extracting the k closest neighbors, to cloneing only the k closests for better performance
  - Updated Node dependencies
  - Updated Rust dependencies
    - Removed unused depricated Rust macro
  - Edited ReadMe


## [2.5.0] - 2019-09-28 <a name="2.5.0"></a>
### Added
  - Added L10, mean-absolute-error, and sum of squared difference distance metrics
  - Added new distance functions to test pipeline
  - Added new distances to readme
  - Added distance examples to readme

### Fixed
  - Added npm install neon and neon build --release steps to travis ci tasks to force building of native binaries even when the version is not increased (to prevent old native binaries from being downloaded).

## [2.4.0] - 2019-09-24 <a name="2.4.0"></a>
### Added
  - Added L3, L4, and L5 distance metrics
  - Added Pearson Correlation distance Metric
  - Added mean, std, and covariance vector functions
  - Added new distance functions to test pipeline
  - Added new distances to readme
  - Added distance examples to readme
  - Added npm build and build-test scripts

### Removed
  - Removed build.sh, run build + test using `npm run build-test`

### Changed
  - Changed powf to powi where possible for efficiency

## [2.3.1] - 2019-09-21 <a name="2.3.1"></a>
### Added
  - Added distance comparison examples to readme

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
