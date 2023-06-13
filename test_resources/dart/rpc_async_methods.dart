class ExamplesAsyncRpcClient {
  ExamplesAsyncRpcClient(this._scheduler) {
    _connectPrintHelloWorld();
    _connectHelloWorld();
    _connectSayHello();
    _connectSum();
  }

  final TechPawsRuntimeRpcScheduler _scheduler;
  static const _scopeId = '106c2228-ff3b-45c5-8a55-db9c0537f275';

  int _lastPrintHelloWorldMethodId = 0;
  final _printHelloWorldCompleters = <int, Completer<void>>{};
  late final TechPawsRuntimeRpcReadTask _readPrintHelloWorldTask;

  int _lastHelloWorldMethodId = 0;
  final _helloWorldCompleters = <int, Completer<String>>{};
  late final TechPawsRuntimeRpcReadTask _readHelloWorldTask;

  int _lastSayHelloMethodId = 0;
  final _sayHelloCompleters = <int, Completer<String>>{};
  late final TechPawsRuntimeRpcReadTask _readSayHelloTask;

  int _lastSumMethodId = 0;
  final _sumCompleters = <int, Completer<double>>{};
  late final TechPawsRuntimeRpcReadTask _readSumTask;

  void disconnect() {
    _scheduler.disconnect(_readPrintHelloWorldTask);
    _scheduler.disconnect(_readHelloWorldTask);
    _scheduler.disconnect(_readSayHelloTask);
    _scheduler.disconnect(_readSumTask);
  }

  void _connectPrintHelloWorld() {
    _readPrintHelloWorldTask = _scheduler.read(
      _scopeId,
      0,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();

        _printHelloWorldCompleters[methodId]?.complete();
        _printHelloWorldCompleters.remove(methodId);
      },
    );
  }

  Future<void> printHelloWorld() {
    final methodId = _lastPrintHelloWorldMethodId;
    _lastPrintHelloWorldMethodId = rotateMethodId(_lastPrintHelloWorldMethodId);

    _scheduler.write(
      _scopeId,
      0,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
      },
    );

    final completer = Completer<void>();
    _printHelloWorldCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectHelloWorld() {
    _readHelloWorldTask = _scheduler.read(
      _scopeId,
      1,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = const StringIntoBuffers().read(reader);

        _helloWorldCompleters[methodId]?.complete(result);
        _helloWorldCompleters.remove(methodId);
      },
    );
  }

  Future<String> helloWorld() {
    final methodId = _lastHelloWorldMethodId;
    _lastHelloWorldMethodId = rotateMethodId(_lastHelloWorldMethodId);

    _scheduler.write(
      _scopeId,
      1,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
      },
    );

    final completer = Completer<String>();
    _helloWorldCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectSayHello() {
    _readSayHelloTask = _scheduler.read(
      _scopeId,
      2,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = const StringIntoBuffers().read(reader);

        _sayHelloCompleters[methodId]?.complete(result);
        _sayHelloCompleters.remove(methodId);
      },
    );
  }

  Future<String> sayHello({
    required String name,
  }) {
    final methodId = _lastSayHelloMethodId;
    _lastSayHelloMethodId = rotateMethodId(_lastSayHelloMethodId);

    _scheduler.write(
      _scopeId,
      2,
      TechPawsRuntimeRpcMethodBuffer.server,
      (writer) {
        writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);
        writer.writeInt64(methodId);
        const StringIntoBuffers().write(writer, name);
      },
    );

    final completer = Completer<String>();
    _sayHelloCompleters[methodId] = completer;

    return completer.future;
  }

  void _connectSum() {
    _readSumTask = _scheduler.read(
      _scopeId,
      3,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final methodId = reader.readInt64();
        final result = reader.readDouble();

        _sumCompleters[methodId]?.complete(result);
        _sumCompleters.remove(methodId);
      },
    );
  }

  Future<double> sum({
    required int a,
    required double b,
    required double c,
  }) {
    final methodId = _lastSumMethodId;
    _lastSumMethodId = rotateMethodId(_lastSumMethodId);

    _scheduler.write(
      _scopeId,
      3,
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
    _sumCompleters[methodId] = completer;

    return completer.future;
  }
}
