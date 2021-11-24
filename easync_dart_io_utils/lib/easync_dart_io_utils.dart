/// Support for doing something awesome.
///
/// More dartdocs go here.
library easync_dart_io_utils;

import 'dart:async';
import 'dart:ffi';

import 'dart:isolate';

class CompleterAndFuture<T> {
  final Future<T> future;
  final int portId;
  final int completerId;

  CompleterAndFuture(this.future, this.portId, this.completerId);
}

class FfiCompleterRegistry {
  static int _idGen = 0;
  static final _registry = <int, Completer>{};
  static final _port = _setup();

  FfiCompleterRegistry._();

  static CompleterAndFuture newCompleter<T>() {
    final completerId = _nextId();
    final completer = Completer<T>();
    _registry[completerId] = completer;

    return CompleterAndFuture(
        completer.future, _port.sendPort.nativePort, completerId);
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
    if (msg is int) {
      _takeCompleter(msg).completeError(FutureCanceled());
      return;
    }
    if (msg is! List || msg.length != 2 || msg[0] is! int) {
      throw ArgumentError(
          'expected msg like: [<completerId>, <data?], got: ${msg.toString()}');
    }
    _takeCompleter(msg[0]).complete(msg[1]);
  }

  static Completer _takeCompleter(int id) {
    final completer = _registry.remove(id);
    if (completer == null) {
      throw StateError('no completer registered for completer id');
    }
    return completer;
  }
}

class FutureCanceled implements Exception {
  @override
  String toString() => 'Rust Future was canceled';
}
