# Unsupervised-KNN [![Build Status](https://travis-ci.org/houtanf/Unsupervised-KNN-JS.svg?branch=master)](https://travis-ci.org/houtanf/Unsupervised-KNN-JS)

Algorithm for fetching the k nearest neighbors of an input vector through distance calculations.

__Still in progress__

## Example

```javascript
> const { knn } = require('.')

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