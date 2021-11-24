
pub mod opaque;
pub mod platform;
pub mod spawn;


// auapi auapi-utils
// example with dart native
// example with
//

/*
  aupai => easyn-utils

  example with dart native
    - link against dart_api_dl.c/dart_api_dl.h
    - INIT: call the init function from dart_api_dl.h with the data from
      NativPort.... (see dart_api_dl.h)
    - then we can use the function exposed by dart_api_dl.h
    - implement platform callback mechanism for dart using
       Dart_PostCObject

*/