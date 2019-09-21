# Unsupervised-KNN-JS

[![Build Status][travis-img]][travis-url]
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
 - [Future Features](#future-features)
 - [Changes](#changes)

<!-- /TOC -->


## Features

  * Parallelized distance computations
  * Fast native system processing
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
            'euclidean' // L2 Norm Difference
            'cosine'    // Cosine Distance
            'mse'       // Mean-Squared-Error 
            'manhattan' // Sum of Absolute Difference
            'chebyshev' //L-Infinite Norm Difference
            'canberra' // Weighted Manhatten Distance
            'hamming' // Binary Difference
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


## Future Features

  * Plans to implement use of custom distance functions passed in by the user
  * More native distance functions
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