# Unsupervised-KNN-JS

[![Build Status][travis-img]][travis-url]
[![Code Coverage][coveralls-img]][coveralls-url]
[![Version][npm-version]][npm-url]
[![Code Size][code-size]][github-url]
[![License][license-img]][license-url]

Node.JS package for fetching the k nearest neighbors of an input vector through distance calculations.

Algorithm computations are implemented in Rust for high perfromance and easy parallelism.

## Table of Contents

<!-- TOC -->

 - [Features](#features)
 - [Install](#install)
 - [Loading](#loading)
 - [Use Example](#example)
 - [Usage](#usage)
    - [Parameters](#parameters)
    - [Return](#return)
 - [Distance Comparisons](#distance-comparisons)
 - [Future Features](#future-features)
 - [Changes](#changes)

<!-- /TOC -->


## Features

  * Parallelized distance computations
  * Fast native system processing
  * Several popular distance metrics
  * Out of the box JavaScript support on Linux, OSX, and Windows
  * Support for Node 8, 9, 10, 11, and 12

## Install

```sh
$ npm i unsupervised-knn-js
```

## Loading

```javascript
const { knn } = require('unsupervised-knn-js')
```


## Example

```javascript
> const { knn } = require('unsupervised-knn-js')

> const neighbors = [
  { label: 'some name', vector: [1, 2, 4, 5] },
  { label: 'name 2', vector: [14, 4, 13, 2] },
  { label: 'another name', vector: [4, 4, 4, 5] },
]
> const target = [1, 2, 3, 4]
> const algo = 'euclidean'
> const k = 2

> knn(algo, k, neighbors, target)
[
  { label: 'some name', distance: 1.4142135623730951 },
  { label: 'another name', distance: 3.872983346207417 }
]
> 
```

## Usage


### Parameters
The knn function takes 4 parameters:

  1. Algorithm String
      * This is the algorithm which computes distances between the target and all neighbors
      * The current algorithms natively supported are:
          ```javascript
            'euclidean'  // L2 Norm Difference
            'cosine'     // Cosine Distance
            'mae'        // Mean-Absolute-Error
            'mse'        // Mean-Squared-Error 
            'manhattan'  // Sum of Absolute Differences
            'ssd'        // Sum of Squared Differences
            'chebyshev'  // L-Infinite Norm Difference
            'canberra'   // Weighted Manhatten Distance
            'hamming'    // Sum of Binary Differences
            'L3'         // L3 Norm Difference
            'L4'         // L4 Norm Difference
            'L5'         // L5 Norm Difference
            'L10'        // L10 Norm Difference
            'pearson'    // Pearson Correlation Distance
          ```
  1. K-Value
      * The amount of closest neighbors to the target point to return
      * So if k = 2, the 2 closests neighbors to the target vector will be returned.
  1. Neighbors
      * This is an array of objects where each object represents a neighbor or point
      * Each object should have a label and vector field as such:
        ```javascript
        {
          label: 'name or id',
          vector: [1, 3, 4.5, -4]
        }
        ```
      * The following is a valid array of neighbors:
        ```javascript
        const neighbors = [
          { label: 'some name', vector: [1, 2, 4, 5] },
          { label: 'name 2', vector: [14, 4, 13, 2] },
          { label: 'another name', vector: [4, 4, 4, 5] },
        ]
        ```
  1. Target
      * This is the vector for which to find the closest or most similar points to
      * This should be a array of numbers


### Return

The function returns an array of objects representing the closest points to the target.

Each object has a label field for identification and a distance field which represents it's difference from the target.
```javascript
[
  { label: 'some name', distance: 1.4142135623730951 },
  { label: 'another name', distance: 3.872983346207417 }
]
```

This list is ordered in ascending order based on the distance field in each object.


## Distance Comparisons

Here is an example of the same data run against different distance functions

```javascript
> const { knn } = require('unsupervised-knn-js')
> const neighbors = [
  { label: 'some name', vector: [1, 2, 4, 5] },
  { label: 'another name', vector: [4, 4, 4, 5] },
  { label: 'name 3', vector: [14, 4, 13, 2] },
]
> const target = [1, 2, 3, 4]

> // Euclidean
> knn('euclidean', 3, neighbors, target)
[
  { label: 'some name', distance: 1.4142135623730951 },
  { label: 'another name', distance: 3.872983346207417 },
  { label: 'name 3', distance: 16.64331697709324 }
]

> // Cosine
> knn('cosine', 3, neighbors, target)
[
  { label: 'some name', distance: 0.003993481192393733 },
  { label: 'another name', distance: 0.059777545024485734 },
  { label: 'name 3', distance: 0.35796589482505503 }
]

> // Mean-Absolute-Error 
> knn('mae', 3, neighbors, target)
[
  { label: 'some name', distance: 0.5 },
  { label: 'another name', distance: 1.75 },
  { label: 'name 2', distance: 6.75 }
]

> // Mean-Squared-Error
> knn('mse', 3, neighbors, target)
[
  { label: 'some name', distance: 0.5 },
  { label: 'another name', distance: 3.75 },
  { label: 'name 3', distance: 69.25 }
]

> // Manhattan
> knn('manhattan', 3, neighbors, target)
[
  { label: 'some name', distance: 2 },
  { label: 'another name', distance: 7 },
  { label: 'name 3', distance: 27 }
]

> // Sum of Squared Differences
> knn('ssd', 3, neighbors, target)
[
  { label: 'some name', distance: 2 },
  { label: 'another name', distance: 15 },
  { label: 'name 2', distance: 277 }
]

> // Chebyshev
> knn('chebyshev', 3, neighbors, target)
[
  { label: 'some name', distance: 1 },
  { label: 'another name', distance: 3 },
  { label: 'name 3', distance: 13 }
]

> // Canberra
> knn('canberra', 3, neighbors, target)
[
  { label: 'some name', distance: 0.25396825396825395 },
  { label: 'another name', distance: 1.1873015873015873 },
  { label: 'name 3', distance: 2.158333333333333 }
]

> // Hamming
> knn('hamming', 3, neighbors, target)
[
  { label: 'some name', distance: 2 },
  { label: 'another name', distance: 4 },
  { label: 'name 3', distance: 4 }
]

> // L3 Norm Difference
> knn('L3', 3, neighbors, target)
[
  { label: 'some name', distance: 1.2599210498948732 },
  { label: 'another name', distance: 3.332221851645953 },
  { label: 'name 3', distance: 14.756054203376182 }
]

> // L4 Norm Difference
> knn('L4', 3, neighbors, target)
[
  { label: 'some name', distance: 1.189207115002721 },
  { label: 'another name', distance: 3.1543421455299043 },
  { label: 'name 3', distance: 14.016098305349052 }
]

> // L5 Norm Difference
> knn('L5', 3, neighbors, target)
[
  { label: 'some name', distance: 1.148698354997035 },
  { label: 'another name', distance: 3.0796116495812957 },
  { label: 'name 3', distance: 13.635466232760923 }
]

> // L10 Norm Difference
> knn('L10', 3, neighbors, target)
[
  { label: 'some name', distance: 1.0717734625362931 },
  { label: 'another name', distance: 3.0051723058500506 },
  { label: 'name 2', distance: 13.091355843137347 }
]

> // Pearson Correlation Distance
> knn('pearson', 3, neighbors, target)
[
  { label: 'some name', distance: 0.010050506338833642 },
  { label: 'another name', distance: 0.2254033307585166 },
  { label: 'name 3', distance: 1.5685785754425927 }
]
```


## Future Features

  * Plans to implement use of custom distance functions passed in by the user
  * Even more native distance functions
  * Parallel computations across multiple targets

  Ideas and suggestions are welcome!


## Changes

  For changes please see the [Changelog][changelog-url]



[travis-img]: https://img.shields.io/travis/houtanf/unsupervised-knn-js?style=for-the-badge
[travis-url]: https://travis-ci.org/houtanf/Unsupervised-KNN-JS
[license-img]: https://img.shields.io/github/license/houtanf/unsupervised-knn-js?style=for-the-badge
[license-url]: https://github.com/houtanf/Unsupervised-KNN-JS/blob/master/LICENSE
[npm-version]: https://img.shields.io/npm/v/unsupervised-knn-js?color=red&style=for-the-badge
[code-size]: https://img.shields.io/github/languages/code-size/houtanf/unsupervised-knn-js?style=for-the-badge
[npm-url]: https://www.npmjs.com/package/unsupervised-knn-js
[github-url]: https://github.com/houtanf/Unsupervised-KNN-JS
[changelog-url]: https://github.com/houtanf/Unsupervised-KNN-JS/blob/master/CHANGELOG.md
[coveralls-img]: https://img.shields.io/coveralls/github/houtanf/Unsupervised-KNN-JS?style=for-the-badge
[coveralls-url]: https://coveralls.io/github/houtanf/Unsupervised-KNN-JS?branch=master