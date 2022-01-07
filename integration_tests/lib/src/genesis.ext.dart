import 'dart:ffi' show NativeApi;

import 'package:async_bindgen_dart_utils/async_bindgen_dart_utils.dart'
    show CouldNotInitializeDartApiError, FfiCompleterRegistry;
// ignore: always_use_package_imports
import './genesis.ffigen.dart' show IntegrationTestsFfi;

class Api2 {
  final IntegrationTestsFfi _inner;

  Api2(this._inner) {
    final status =
        _inner.async_bindgen_dart_init_api__api2(NativeApi.initializeApiDLData);
    if (status != 1) {
      throw CouldNotInitializeDartApiError();
    }
  }

  Future<int> getTheByte() {
    final setup = FfiCompleterRegistry.newCompleter(
      extractor: _inner.async_bindgen_dart_r__api2__get_the_byte,
    );
    final callOk = _inner.async_bindgen_dart_c__api2__get_the_byte(
      setup.portId,
      setup.completerId,
    );
    if (callOk == 0) {
      //TODO
      throw Exception('failed to setup callbacks');
    }
    return setup.future;
  }
}

class AsyncApi {
  final IntegrationTestsFfi _inner;

  AsyncApi(this._inner) {
    final status = _inner
        .async_bindgen_dart_init_api__async_api(NativeApi.initializeApiDLData);
    if (status != 1) {
      throw CouldNotInitializeDartApiError();
    }
  }

  Future<int> add(
    int x,
    int y,
  ) {
    final setup = FfiCompleterRegistry.newCompleter(
      extractor: _inner.async_bindgen_dart_r__async_api__add,
    );
    final callOk = _inner.async_bindgen_dart_c__async_api__add(
      x,
      y,
      setup.portId,
      setup.completerId,
    );
    if (callOk == 0) {
      //TODO
      throw Exception('failed to setup callbacks');
    }
    return setup.future;
  }

  Future<int> sub(
    int x,
    int y,
  ) {
    final setup = FfiCompleterRegistry.newCompleter(
      extractor: _inner.async_bindgen_dart_r__async_api__sub,
    );
    final callOk = _inner.async_bindgen_dart_c__async_api__sub(
      x,
      y,
      setup.portId,
      setup.completerId,
    );
    if (callOk == 0) {
      //TODO
      throw Exception('failed to setup callbacks');
    }
    return setup.future;
  }
}
