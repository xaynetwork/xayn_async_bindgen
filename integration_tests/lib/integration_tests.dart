/// Support for doing something awesome.
///
/// More dartdocs go here.
library integration_tests;

import 'dart:ffi' show NativeApi;
import 'package:integration_tests/src/ext.ffigen.dart';
import 'package:integration_tests/src/load_lib.dart' show ffi;


Future<int> add(int l, int r) {
  return ffi.add(l,r);
}

/// ffi bool as dart bool
///
/// ffigen might depending on factors outside of it's version
/// sometimes generate a bool returning function an sometimes an
/// integer returning function.
bool ffiBool(Object val) {
  if (val is int) {
    assert(val == 1 || val == 0);
    return val == 1;
  }
  assert(val is bool);
  return val as bool;
}

Future<void> initialize() async {
  if (ffiBool(ffi.init_dart_api_dl(NativeApi.initializeApiDLData))) {
    return;
  }
  throw Exception('failed to initialize');
}
