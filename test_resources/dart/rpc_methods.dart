class TestRpcClient {
  TestRpcClient(this._scheduler) {
    _connectPrintHelloWorldAsync();
    _connectHelloWorldAsync();
    _connectSayHelloAsync();
    _connectSumAsync();
    _connectTrigger();
    _connectTheme();
    _connectTriggerAsync();
    _connectThemeAsync();
  }

  final TechPawsRuntimeRpcScheduler _scheduler;
  static const _scopeId = '4de616f8-12c5-4d2c-8d48-9c5fb038991f';

  int _lastPrintHelloWorldAsyncMethodId = 0;
  final _printHelloWorldAsyncCompleters = <int, Completer<void>>{};
  late final TechPawsRuntimeRpcReadTask _readPrintHelloWorldAsyncTask;

  int _lastHelloWorldAsyncMethodId = 0;
  final _helloWorldAsyncCompleters = <int, Completer<String>>{};
  late final TechPawsRuntimeRpcReadTask _readHelloWorldAsyncTask;

  int _lastSayHelloAsyncMethodId = 0;
  final _sayHelloAsyncCompleters = <int, Completer<String>>{};
  late final TechPawsRuntimeRpcReadTask _readSayHelloAsyncTask;

  int _lastSumAsyncMethodId = 0;
  final _sumAsyncCompleters = <int, Completer<double>>{};
  late final TechPawsRuntimeRpcReadTask _readSumAsyncTask;

  late final TechPawsRuntimeRpcReadTask _readTriggerTask;
  final _readTriggerStreamController = StreamController<void>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readThemeTask;
  final _readThemeStreamController = StreamController<Theme>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readTriggerAsyncTask;
  final _readTriggerAsyncStreamController = StreamController<void>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readThemeAsyncTask;
  final _readThemeAsyncStreamController = StreamController<Theme>.broadcast();

  Stream<void> get trigger => _readTriggerStreamController.stream;

  Stream<Theme> get theme => _readThemeStreamController.stream;

  Stream<void> get triggerAsync => _readTriggerAsyncStreamController.stream;

  Stream<Theme> get themeAsync => _readThemeAsyncStreamController.stream;

  void disconnect() {
    _scheduler.disconnect(_readPrintHelloWorldAsyncTask);
    _scheduler.disconnect(_readHelloWorldAsyncTask);
    _scheduler.disconnect(_readSayHelloAsyncTask);
    _scheduler.disconnect(_readSumAsyncTask);
    _scheduler.disconnect(_readTriggerTask);
    _readTriggerStreamController.close();
    _scheduler.disconnect(_readThemeTask);
    _readThemeStreamController.close();
    _scheduler.disconnect(_readTriggerAsyncTask);
    _readTriggerAsyncStreamController.close();
    _scheduler.disconnect(_readThemeAsyncTask);
    _readThemeAsyncStreamController.close();
  }

  void printHelloWorld() {
    _scheduler.syncWrite(
      _scopeId,
      0,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
      },
    );

    _scheduler.loopSyncGroup();

  }

  String helloWorld() {
    _scheduler.syncWrite(
      _scopeId,
      1,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
      },
    );

    _scheduler.loopSyncGroup();

    final result = _scheduler.syncRead<String>(
      _scopeId,
      1,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          throw StateError('No data');
        }

        return const StringIntoBuffers().read(reader);
      },
    );

    return result;
  }

  String sayHello({
    required String name,
  }) {
    _scheduler.syncWrite(
      _scopeId,
      2,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        const StringIntoBuffers().write(writer, name);
      },
    );

    _scheduler.loopSyncGroup();

    final result = _scheduler.syncRead<String>(
      _scopeId,
      2,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          throw StateError('No data');
        }

        return const StringIntoBuffers().read(reader);
      },
    );

    return result;
  }

  void sum({
    required int a,
    required double b,
    required double c,
  }) {
    _scheduler.syncWrite(
      _scopeId,
      3,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt32(a);
        writer.writeFloat(b);
        writer.writeDouble(c);
      },
    );

    _scheduler.loopSyncGroup();

  }

  void _connectPrintHelloWorldAsync() {
    _readPrintHelloWorldAsyncTask = _scheduler.read(
      _scopeId,
      4,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();

        _printHelloWorldAsyncCompleters[methodId]?.complete();
        _printHelloWorldAsyncCompleters.remove(methodId);
      },
    );
  }

  Future<void> printHelloWorldAsync() {
    final methodId = _lastPrintHelloWorldAsyncMethodId;
    _lastPrintHelloWorldAsyncMethodId = rotateMethodId(_lastPrintHelloWorldAsyncMethodId);

    _scheduler.write(
      _scopeId,
      4,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
      },
    );

    final completer = Completer<void>();
    _printHelloWorldAsyncCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectHelloWorldAsync() {
    _readHelloWorldAsyncTask = _scheduler.read(
      _scopeId,
      5,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = const StringIntoBuffers().read(reader);

        _helloWorldAsyncCompleters[methodId]?.complete(result);
        _helloWorldAsyncCompleters.remove(methodId);
      },
    );
  }

  Future<String> helloWorldAsync() {
    final methodId = _lastHelloWorldAsyncMethodId;
    _lastHelloWorldAsyncMethodId = rotateMethodId(_lastHelloWorldAsyncMethodId);

    _scheduler.write(
      _scopeId,
      5,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
      },
    );

    final completer = Completer<String>();
    _helloWorldAsyncCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectSayHelloAsync() {
    _readSayHelloAsyncTask = _scheduler.read(
      _scopeId,
      6,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = const StringIntoBuffers().read(reader);

        _sayHelloAsyncCompleters[methodId]?.complete(result);
        _sayHelloAsyncCompleters.remove(methodId);
      },
    );
  }

  Future<String> sayHelloAsync({
    required String name,
  }) {
    final methodId = _lastSayHelloAsyncMethodId;
    _lastSayHelloAsyncMethodId = rotateMethodId(_lastSayHelloAsyncMethodId);

    _scheduler.write(
      _scopeId,
      6,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
        const StringIntoBuffers().write(writer, name);
      },
    );

    final completer = Completer<String>();
    _sayHelloAsyncCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectSumAsync() {
    _readSumAsyncTask = _scheduler.read(
      _scopeId,
      7,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = reader.readDouble();

        _sumAsyncCompleters[methodId]?.complete(result);
        _sumAsyncCompleters.remove(methodId);
      },
    );
  }

  Future<double> sumAsync({
    required int a,
    required double b,
    required double c,
  }) {
    final methodId = _lastSumAsyncMethodId;
    _lastSumAsyncMethodId = rotateMethodId(_lastSumAsyncMethodId);

    _scheduler.write(
      _scopeId,
      7,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
        writer.writeInt32(a);
        writer.writeFloat(b);
        writer.writeDouble(c);
      },
    );

    final completer = Completer<double>();
    _sumAsyncCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectTrigger() {
    _readTriggerTask = _scheduler.read(
      _scopeId,
      8,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        _readTriggerStreamController.add(null);
      },
    );
  }

  void _connectTheme() {
    _readThemeTask = _scheduler.read(
      _scopeId,
      9,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final result = const ThemeIntoBuffers().read(reader);
        _readThemeStreamController.add(result);
      },
    );
  }

  void _connectTriggerAsync() {
    _readTriggerAsyncTask = _scheduler.read(
      _scopeId,
      10,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        _readTriggerAsyncStreamController.add(null);
      },
    );
  }

  void _connectThemeAsync() {
    _readThemeAsyncTask = _scheduler.read(
      _scopeId,
      11,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final result = const ThemeIntoBuffers().read(reader);
        _readThemeAsyncStreamController.add(result);
      },
    );
  }
}
