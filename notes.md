


Future<int> asyncAdd(int l, int r) async {
    final ffiFuture = resolver.newFfiFuture<int>();
    final ok = ffi.easync_dart_sw__async_add(l, r, ffiFuture.nativeCompleter);

    if (ok != 0) {
        throw StateError("dart_api_dl not initialized");
    }

    return ffiFuture.future;
}

struct DartNativeCompleter {
    port: Dart_Port,
    id: i64
}

fn easync_dart_sw__async_add(l: i64, r:i64, port_id: i64, completer_id: i64) -> isize {
    let completer = PreparedCompleter::new(port_id, completer_id)?;
    spawn(completer.bind_future(async_add(l, r)))
    0
}


// pattern :

async fn <name>(<args>*) -> <ret>

=>

fn easync_<lang>_sw__<name>(<args>*, <lang_overhead_params>*) -> <status> {
    if let Ok(completer) = <langCompleter>::new(<lang_overhead_params>*) {
        sapwn(completer.bind_future(<name>(<args>*)));
        0
    } else {
        -1
    }
}







------------

Awesome realization:

spawn(Future<Output=T>) => JoinHandle<T>

So by polling "through dart" we can have
exactly the same API for sync/non-sync and
can sidestep CObject...