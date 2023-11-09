# multiple level specification

## Level0: Abstract specification and generate all states

1. Write abstract specification.

2. Use [StateDB](https://github.com/scuptio/TLAPlusCommunityModules/blob/master/modules/StateDB.tla) module to dump all states of a model checking result.


## Level1: Concrete specification

1. Write concrete specification;
2. Choose a valid initialized state that read from StateDB, start from this state and run model checking.


## TODO

Use distributed system to faster testing progress.

Develop a framework to deal with this.