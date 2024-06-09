/**
* TODO: support JSON, JSONB []
* TODO: support GEOMITRY, GEOGRAPHY []
* TODO: support INTERVAL []
* FIX: parse types in conditions preperly strings '' for example [x]
* TODO: add procedural macro to select based on struct
* TODO: add support for parsing bytes sql value
* TODO: add support for native time
* */
mod condition;
mod group_by;
mod join;
mod order_by;
mod query;
mod select;
mod value;
