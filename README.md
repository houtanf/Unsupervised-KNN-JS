# Unsupervised-KNN-JS

[![Build Status][travis-img]][travis-url]

Algorithm for fetching the k nearest neighbors of an input vector through distance calculations.

Algorithm computations are implemented in Rust for high perfromance and easy parallelism.

## Features

  * Parallelized distance computations
  * Fast native system processing
  * Out of the box JS support for Linux and OSX (Waiting on Windows)
      * For now Windows users will need to install Rust to build required native components
      * Pure JS implementation of the package might be made in the future for Windows users

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
      * The current algorithms natively supported include:
          ```javascript
            'euclidean'
          ```
  1. K-Value
      * The amount of closest neighbors to the target point to return
      * So if k = 2, the 2 closests neoghbors to the target vector will be returned.
  1. Neighbors
      * This is a list of objects where each object represents a neigbor or point
      * Each object should have a label and vector field as such:
        ```javascript
        {
          label: 'name or id',
          vector: [1, 3, 4.5, -4]
        }
        ```
      * The following is a valid list of neighbors:
        ```javascript
        const neighbors = [
          { label: 'some name', vector: [1, 2, 4, 5] },
          { label: 'name 2', vector: [14, 4, 13, 2] },
          { label: 'another name', vector: [4, 4, 4, 5] },
        ]
        ```
  1. Target
      * This is the vector for which to find the closest or most similar points to
      * This should be a list of numbers

These parameters were given this specific order based which paramaters would be most important for currying.

### Return

The function returns a list of objects representing the closest points to the target.

Each object has a label field for identification and a distance field which represents it's difference from the target.
```javascript
[
  { label: 'some name', distance: 1.4142135623730951 },
  { label: 'another name', distance: 3.872983346207417 }
]
```

This list is ordered in ascending order based on the distance field in each object.

## Future Features

  * Plans to implement use of custom distance functions implemented by the user.
  * Addition of more distance functions
  * Windows support (hopefully)
  * Parallel computations across multiple targets


[travis-img]: https://flat.badgen.net//travis/houtanf/unsupervised-knn-js/master?
[travis-url]: https://travis-ci.org/houtanf/Unsupervised-KNN-JS
