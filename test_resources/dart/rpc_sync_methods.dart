class ExamplesSyncRpcClient {
  ExamplesSyncRpcClient(this._scheduler);

  final TechPawsRuntimeRpcScheduler _scheduler;
  static const _scopeId = '4de616f8-12c5-4d2c-8d48-9c5fb038991f';

  void disconnect() {
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

  double sum({
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

    final result = _scheduler.syncRead<double>(
      _scopeId,
      3,
      TechPawsRuntimeRpcMethodBuffer.client,
      (reader) {
        final status = reader.readInt8();

        if (status != TechPawsRpcBufferStatus.hasData.value) {
          throw StateError('No data');
        }

        return reader.readDouble();
      },
    );

    return result;
  }
}
