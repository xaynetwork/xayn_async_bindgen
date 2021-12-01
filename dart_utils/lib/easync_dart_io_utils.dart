/// Support for doing something awesome.
///
/// More dartdocs go here.
library easync_dart_io_utils;

import 'dart:async' show Completer, Future;
import 'dart:ffi';

import 'dart:isolate' show ReceivePort;

const int _magicTag = -6504203682518908873;

class FfiCompleterRegistry {
  static int _idGen = 0;
  static final _registry = <int, _FfiCompleter>{};
  static final _port = _setup();

  FfiCompleterRegistry._();

  static FfiSetup newCompleter<T>() {
    final completerId = _nextId();
    final ffiCompleter = _FfiCompleterImpl(
      completer: Completer(),
      portId: _port.sendPort.nativePort,
      completerId: completerId,
    );
    _registry[completerId] = ffiCompleter;
    return ffiCompleter;
  }

  static int _nextId() {
    assert(_idGen < 0x1fffffffffffff);
    return _idGen++;
  }

  static ReceivePort _setup() {
    final port = ReceivePort("ffiCompleter");
    _startHandlingCompletions(port);
    return port;
  }

  static void _startHandlingCompletions(ReceivePort port) async {
    await for (final msg in port) {
      try {
        _handleMessage(msg);
      } catch (e) {
        //TODO log error
      }
    }
  }

  static void _handleMessage(Object? msg) {
    if (msg is List && msg[0] == _magicTag && msg.length >= 3) {
      assert(msg[1] is int);
      final completer = _takeCompleter(msg[1] as int);
      final result = msg[2];
      if (result is int) {
        completer.complete(result);
      } else if (result is String) {
        completer.completeError(FutureCanceled(result));
      } else {
        completer.completeError(
            FutureCanceled('unexpected result msg ${msg.toString()}'));
      }
    }
    throw ArgumentError(
        'expected well formed async bindgen response, got: ${msg.toString()}');
  }

  static _FfiCompleter _takeCompleter(int id) {
    final completer = _registry.remove(id);
    if (completer == null) {
      throw StateError('no completer registered for completer id');
    }
    return completer;
  }
}

class FutureCanceled implements Exception {
  final String _msg;

  FutureCanceled(this._msg);

  @override
  String toString() => 'Rust Future was canceled: $_msg';
}

abstract class _FfiCompleter {
  void complete(int handle);
  void completeError(Object error);
}

abstract class FfiSetup<T> {
  set extractor(T Function(int)? extractor);
  int get portId;
  int get completerId;
  Future<T> get future;
}

class _FfiCompleterImpl<T> implements _FfiCompleter, FfiSetup<T> {
  bool extractorNotSet = true;
  T Function(int)? _extractor;
  final Completer<T> _completer;
  final int _portId;
  final int _completerId;

  _FfiCompleterImpl({
    required Completer<T> completer,
    required int portId,
    required int completerId,
  })  : _completer = completer,
        _portId = portId,
        _completerId = completerId;

  @override
  int get portId => _portId;
  @override
  int get completerId => _completerId;
  @override
  set extractor(T Function(int)? extractor) {
    if (extractorNotSet) {
      extractorNotSet = false;
      _extractor = extractor;
    } else {
      throw StateError('extractor already set');
    }
  }

  @override
  Future<T> get future => _completer.future;

  @override
  void complete(int handle) {
    final extractor = _extractor;
    // while this method should never be called twice,
    // we still want to make sure the extractor is definitely
    // not called twice.
    _extractor = null;
    if (extractor == null) {
      throw StateError('extractor is null: extractorNotSet=$extractorNotSet');
    }
    final val = extractor(handle);
    _completer.complete(val);
  }

  @override
  void completeError(Object error) {
    _extractor = null;
    _completer.completeError(error);
  }
}
