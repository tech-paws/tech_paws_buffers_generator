class ExamplesReadRpcClient {
  ExamplesReadRpcClient(this._scheduler) {
    _connectCounter();
    _connectTheme();
    _connectAsyncTrigger();
    _connectAsyncHelloRead();
  }

  final TechPawsRuntimeRpcScheduler _scheduler;
  static const _scopeId = '723ca727-6a66-43a7-bfcc-b8ad94eac9be';

  late final TechPawsRuntimeRpcReadTask _readCounterTask;
  final _readCounterStreamController = StreamController<void>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readThemeTask;
  final _readThemeStreamController = StreamController<String>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readAsyncTriggerTask;
  final _readAsyncTriggerStreamController = StreamController<void>.broadcast();

  late final TechPawsRuntimeRpcReadTask _readAsyncHelloReadTask;
  final _readAsyncHelloReadStreamController = StreamController<String>.broadcast();

  Stream<void> get counter => _readCounterStreamController.stream;

  Stream<String> get theme => _readThemeStreamController.stream;

  Stream<void> get asyncTrigger => _readAsyncTriggerStreamController.stream;

  Stream<String> get asyncHelloRead => _readAsyncHelloReadStreamController.stream;

  void disconnect() {
    _scheduler.disconnect(_readCounterTask);
    _readCounterStreamController.close();
    _scheduler.disconnect(_readThemeTask);
    _readThemeStreamController.close();
    _scheduler.disconnect(_readAsyncTriggerTask);
    _readAsyncTriggerStreamController.close();
    _scheduler.disconnect(_readAsyncHelloReadTask);
    _readAsyncHelloReadStreamController.close();
  }

  void _connectCounter() {
    _readCounterTask = _scheduler.read(
      _scopeId,
      0,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        _readCounterStreamController.add(null);
      },
    );
  }

  void _connectTheme() {
    _readThemeTask = _scheduler.read(
      _scopeId,
      1,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final result = const StringIntoBuffers().read(reader);
        _readThemeStreamController.add(result);
      },
    );
  }

  void _connectAsyncTrigger() {
    _readAsyncTriggerTask = _scheduler.read(
      _scopeId,
      2,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        _readAsyncTriggerStreamController.add(null);
      },
    );
  }

  void _connectAsyncHelloRead() {
    _readAsyncHelloReadTask = _scheduler.read(
      _scopeId,
      3,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          return;
        }

        final result = const StringIntoBuffers().read(reader);
        _readAsyncHelloReadStreamController.add(result);
      },
    );
  }
}
