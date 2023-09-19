# multiple level specification

## Level0: abstract specification and generate all states

1. Write abstract specification.

2. Use [StateStore](https://github.com/scuptio/TLAPlusCommunityModules/blob/master/modules/StateStore.tla) module to dump all states of a model checking result.


## Level1: Concrete specification

1. Write concrete specification;
2. Choose a valid initialized state that read from StateStore, start from this state and run model checking.


## TODO

Use distributed system to faster testing progress.

Develop a framework to deal with this.