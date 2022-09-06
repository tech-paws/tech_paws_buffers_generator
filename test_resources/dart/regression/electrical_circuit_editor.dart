class __editor_state_rpc_args__ {
  UIEvent event;

  __editor_state_rpc_args__({
    required this.event,
  });
}

class __editor_state_rpc_args__IntoBuffers implements IntoBuffers<__editor_state_rpc_args__> {
  const __editor_state_rpc_args__IntoBuffers();

  @override
  __editor_state_rpc_args__ read(BytesReader reader) {
    final event = const UIEventIntoBuffers().read(reader);

    return __editor_state_rpc_args__(
      event: event,
    );
  }

  @override
  void write(BytesWriter writer, __editor_state_rpc_args__ model) {
    const UIEventIntoBuffers().write(writer, model.event);
  }

  @override
  void skip(BytesReader reader, int count) {
    const UIEventIntoBuffers().skip(reader, 1);

    for (int i = 0; i < count; i += 1) {
    }
  }
}

class ElecticalCircuitEditorRpcClient implements RpcClient {
  ElecticalCircuitEditorRpcClient(
    this._scheduler, {
    required this.editorStateClientAddress,
    required this.editorStateServerAddress,
  });

  final TechPawsRuntimeChannelScheduler _scheduler;
  StreamController<UIState>? _readEditorStateStream;
  final _readEditorStateTasks = <TechPawsRuntimeChannelReadTask>[];
  final int editorStateClientAddress;
  final int editorStateServerAddress;

  void disconnect() {
    for (final task in _readEditorStateTasks) _scheduler.disconnect(task);

    _readEditorStateStream?.close();
  }

  Stream<UIState> readEditorState() {
    if (_readEditorStateStream != null) {
      return _readEditorStateStream!.stream;
    }

    _readEditorStateStream = StreamController<UIState>.broadcast();

    final task = _scheduler.read(editorStateClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readEditorStateStream!.add(const UIStateIntoBuffers().read(reader));
      }
    });

    _readEditorStateTasks.add(task);
    return _readEditorStateStream!.stream;
  }

  Stream<UIState> readEditorStateEmplace(UIState model) {
    if (_readEditorStateStream != null) {
      return _readEditorStateStream!.stream;
    }

    _readEditorStateStream = StreamController<UIState>.broadcast();

    final task = _scheduler.read(editorStateClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        const UIStateEmplaceToBuffers().read(reader, model);
        _readEditorStateStream!.add(model);
      }
    });

    _readEditorStateTasks.add(task);
    return _readEditorStateStream!.stream;
  }

  void writeEditorState({
    required UIEvent event,
  }) {
    final args = __editor_state_rpc_args__(
      event: event,
    );

    _scheduler.write(editorStateServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
      const __editor_state_rpc_args__IntoBuffers().write(writer, args);
    });
  }

  Future<UIState> editorState({
    required UIEvent event,
  }) {
    writeEditorState(
      event: event,
    );

    final completer = Completer<UIState>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(editorStateClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        completer.complete(const UIStateIntoBuffers().read(reader));
        _scheduler.disconnect(task);
        _readEditorStateTasks.remove(task);
      }
    });

    _readEditorStateTasks.add(task);
    return completer.future;
  }
}
