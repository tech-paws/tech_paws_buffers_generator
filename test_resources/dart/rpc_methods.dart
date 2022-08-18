class __print_hello_world_rpc_args__ {
  const __print_hello_world_rpc_args__();
}

class __print_hello_world_rpc_args__IntoBuffers implements IntoBuffers<__print_hello_world_rpc_args__> {
  const __print_hello_world_rpc_args__IntoBuffers();

  @override
  __print_hello_world_rpc_args__ read(BytesReader reader) {

    return __print_hello_world_rpc_args__(
    );
  }

  @override
  void write(BytesWriter writer, __print_hello_world_rpc_args__ model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class __hello_world_rpc_args__ {
  const __hello_world_rpc_args__();
}

class __hello_world_rpc_args__IntoBuffers implements IntoBuffers<__hello_world_rpc_args__> {
  const __hello_world_rpc_args__IntoBuffers();

  @override
  __hello_world_rpc_args__ read(BytesReader reader) {

    return __hello_world_rpc_args__(
    );
  }

  @override
  void write(BytesWriter writer, __hello_world_rpc_args__ model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class __say_hello_rpc_args__ {
  String name;

  __say_hello_rpc_args__({
    required this.name,
  });
}

class __say_hello_rpc_args__IntoBuffers implements IntoBuffers<__say_hello_rpc_args__> {
  const __say_hello_rpc_args__IntoBuffers();

  @override
  __say_hello_rpc_args__ read(BytesReader reader) {
    final name = const StringIntoBuffers().read(reader);

    return __say_hello_rpc_args__(
      name: name,
    );
  }

  @override
  void write(BytesWriter writer, __say_hello_rpc_args__ model) {
    const StringIntoBuffers().write(writer, model.name);
  }

  @override
  void skip(BytesReader reader, int count) {
    const StringIntoBuffers().skip(reader, 1);

    for (int i = 0; i < count; i += 1) {
    }
  }
}

class __sum_rpc_args__ {
  int a;
  double b;
  double c;

  __sum_rpc_args__({
    required this.a,
    required this.b,
    required this.c,
  });
}

class __sum_rpc_args__IntoBuffers implements IntoBuffers<__sum_rpc_args__> {
  const __sum_rpc_args__IntoBuffers();

  @override
  __sum_rpc_args__ read(BytesReader reader) {
    final a = reader.readInt32();
    final b = reader.readFloat();
    final c = reader.readDouble();

    return __sum_rpc_args__(
      a: a,
      b: b,
      c: c,
    );
  }

  @override
  void write(BytesWriter writer, __sum_rpc_args__ model) {
    writer.writeInt32(model.a);
    writer.writeFloat(model.b);
    writer.writeDouble(model.c);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readInt32();
      reader.readFloat();
      reader.readDouble();
    }
  }
}

class TestRpcClient implements RpcClient {
  final TechPawsRuntimeChannelScheduler _scheduler;

  StreamController<void>? _readPrintHelloWorldStream;
  StreamController<String>? _readHelloWorldStream;
  StreamController<String>? _readSayHelloStream;
  StreamController<void>? _readSumStream;

  final _readPrintHelloWorldTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readHelloWorldTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readSayHelloTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readSumTasks = <TechPawsRuntimeChannelReadTask>[];

  TestRpcClient(this._scheduler);

  void disconnect() {
    for (final task in _readPrintHelloWorldTasks) _scheduler.disconnect(task);
    for (final task in _readHelloWorldTasks) _scheduler.disconnect(task);
    for (final task in _readSayHelloTasks) _scheduler.disconnect(task);
    for (final task in _readSumTasks) _scheduler.disconnect(task);

    _readPrintHelloWorldStream?.close();
    _readHelloWorldStream?.close();
    _readSayHelloStream?.close();
    _readSumStream?.close();
  }

  Stream<void> readPrintHelloWorld() {
    if (_readPrintHelloWorldStream != null) {
      return _readPrintHelloWorldStream!.stream;
    }

    _readPrintHelloWorldStream = StreamController<void>.broadcast();

    final task = _scheduler.read(kPrintHelloWorldClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readPrintHelloWorldStream!.add(null);
      }
    });

    _readPrintHelloWorldTasks.add(task);
    return _readPrintHelloWorldStream!.stream;
  }

  void writePrintHelloWorld() {
    _scheduler.write(kPrintHelloWorldServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
    });
  }

  Future<void> printHelloWorld() {
    writePrintHelloWorld();
    final completer = Completer<void>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(kPrintHelloWorldClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        completer.complete();
        _scheduler.disconnect(task);
        _readPrintHelloWorldTasks.remove(task);
      }
    });

    _readPrintHelloWorldTasks.add(task);
    return completer.future;
  }

  Stream<String> readHelloWorld() {
    if (_readHelloWorldStream != null) {
      return _readHelloWorldStream!.stream;
    }

    _readHelloWorldStream = StreamController<String>.broadcast();

    final task = _scheduler.read(kHelloWorldClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readHelloWorldStream!.add(const StringIntoBuffers().read(reader));
      }
    });

    _readHelloWorldTasks.add(task);
    return _readHelloWorldStream!.stream;
  }

  Stream<String> readHelloWorldEmplace(String model) {
    if (_readHelloWorldStream != null) {
      return _readHelloWorldStream!.stream;
    }

    _readHelloWorldStream = StreamController<String>.broadcast();

    final task = _scheduler.read(kHelloWorldClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        model = const StringIntoBuffers().read(reader);
        _readHelloWorldStream!.add(model);
      }
    });

    _readHelloWorldTasks.add(task);
    return _readHelloWorldStream!.stream;
  }

  void writeHelloWorld() {
    _scheduler.write(kHelloWorldServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
    });
  }

  Future<String> helloWorld() {
    writeHelloWorld();
    final completer = Completer<String>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(kHelloWorldClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        completer.complete(const StringIntoBuffers().read(reader));
        _scheduler.disconnect(task);
        _readHelloWorldTasks.remove(task);
      }
    });

    _readHelloWorldTasks.add(task);
    return completer.future;
  }

  Stream<String> readSayHello() {
    if (_readSayHelloStream != null) {
      return _readSayHelloStream!.stream;
    }

    _readSayHelloStream = StreamController<String>.broadcast();

    final task = _scheduler.read(kSayHelloClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readSayHelloStream!.add(const StringIntoBuffers().read(reader));
      }
    });

    _readSayHelloTasks.add(task);
    return _readSayHelloStream!.stream;
  }

  Stream<String> readSayHelloEmplace(String model) {
    if (_readSayHelloStream != null) {
      return _readSayHelloStream!.stream;
    }

    _readSayHelloStream = StreamController<String>.broadcast();

    final task = _scheduler.read(kSayHelloClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        model = const StringIntoBuffers().read(reader);
        _readSayHelloStream!.add(model);
      }
    });

    _readSayHelloTasks.add(task);
    return _readSayHelloStream!.stream;
  }

  void writeSayHello({
    required String name,
  }) {
    final args = __say_hello_rpc_args__(
      name: name,
    );

    _scheduler.write(kSayHelloServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
      const __say_hello_rpc_args__IntoBuffers().write(writer, args);
    });
  }

  Future<String> sayHello({
    required String name,
  }) {
    writeSayHello(
      name: name,
    );

    final completer = Completer<String>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(kSayHelloClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        completer.complete(const StringIntoBuffers().read(reader));
        _scheduler.disconnect(task);
        _readSayHelloTasks.remove(task);
      }
    });

    _readSayHelloTasks.add(task);
    return completer.future;
  }

  Stream<void> readSum() {
    if (_readSumStream != null) {
      return _readSumStream!.stream;
    }

    _readSumStream = StreamController<void>.broadcast();

    final task = _scheduler.read(kSumClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readSumStream!.add(null);
      }
    });

    _readSumTasks.add(task);
    return _readSumStream!.stream;
  }

  void writeSum({
    required int a,
    required double b,
    required double c,
  }) {
    final args = __sum_rpc_args__(
      a: a,
      b: b,
      c: c,
    );

    _scheduler.write(kSumServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
      const __sum_rpc_args__IntoBuffers().write(writer, args);
    });
  }

  Future<void> sum({
    required int a,
    required double b,
    required double c,
  }) {
    writeSum(
      a: a,
      b: b,
      c: c,
    );

    final completer = Completer<void>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(kSumClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        completer.complete();
        _scheduler.disconnect(task);
        _readSumTasks.remove(task);
      }
    });

    _readSumTasks.add(task);
    return completer.future;
  }
}
