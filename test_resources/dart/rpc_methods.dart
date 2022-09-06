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

class __trigger_rpc_args__ {
  const __trigger_rpc_args__();
}

class __trigger_rpc_args__IntoBuffers implements IntoBuffers<__trigger_rpc_args__> {
  const __trigger_rpc_args__IntoBuffers();

  @override
  __trigger_rpc_args__ read(BytesReader reader) {

    return __trigger_rpc_args__(
    );
  }

  @override
  void write(BytesWriter writer, __trigger_rpc_args__ model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class __theme_rpc_args__ {
  const __theme_rpc_args__();
}

class __theme_rpc_args__IntoBuffers implements IntoBuffers<__theme_rpc_args__> {
  const __theme_rpc_args__IntoBuffers();

  @override
  __theme_rpc_args__ read(BytesReader reader) {

    return __theme_rpc_args__(
    );
  }

  @override
  void write(BytesWriter writer, __theme_rpc_args__ model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class TestRpcClient implements RpcClient {
  TestRpcClient(
    this._scheduler, {
    required this.printHelloWorldClientAddress,
    required this.printHelloWorldServerAddress,
    required this.helloWorldClientAddress,
    required this.helloWorldServerAddress,
    required this.sayHelloClientAddress,
    required this.sayHelloServerAddress,
    required this.sumClientAddress,
    required this.sumServerAddress,
    required this.triggerClientAddress,
    required this.themeClientAddress,
  });

  final TechPawsRuntimeChannelScheduler _scheduler;

  StreamController<void>? _readPrintHelloWorldStream;
  StreamController<String>? _readHelloWorldStream;
  StreamController<String>? _readSayHelloStream;
  StreamController<void>? _readSumStream;
  StreamController<void>? _readTriggerStream;
  StreamController<Theme>? _readThemeStream;

  final _readPrintHelloWorldTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readHelloWorldTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readSayHelloTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readSumTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readTriggerTasks = <TechPawsRuntimeChannelReadTask>[];
  final _readThemeTasks = <TechPawsRuntimeChannelReadTask>[];

  final int printHelloWorldClientAddress;
  final int printHelloWorldServerAddress;
  final int helloWorldClientAddress;
  final int helloWorldServerAddress;
  final int sayHelloClientAddress;
  final int sayHelloServerAddress;
  final int sumClientAddress;
  final int sumServerAddress;
  final int triggerClientAddress;
  final int themeClientAddress;

  void disconnect() {
    for (final task in _readPrintHelloWorldTasks) _scheduler.disconnect(task);
    for (final task in _readHelloWorldTasks) _scheduler.disconnect(task);
    for (final task in _readSayHelloTasks) _scheduler.disconnect(task);
    for (final task in _readSumTasks) _scheduler.disconnect(task);
    for (final task in _readTriggerTasks) _scheduler.disconnect(task);
    for (final task in _readThemeTasks) _scheduler.disconnect(task);

    _readPrintHelloWorldStream?.close();
    _readHelloWorldStream?.close();
    _readSayHelloStream?.close();
    _readSumStream?.close();
    _readTriggerStream?.close();
    _readThemeStream?.close();
  }

  Stream<void> readPrintHelloWorld() {
    if (_readPrintHelloWorldStream != null) {
      return _readPrintHelloWorldStream!.stream;
    }

    _readPrintHelloWorldStream = StreamController<void>.broadcast();

    final task = _scheduler.read(printHelloWorldClientAddress, (reader) {
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
    _scheduler.write(printHelloWorldServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
    });
  }

  Future<void> printHelloWorld() {
    writePrintHelloWorld();
    final completer = Completer<void>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(printHelloWorldClientAddress, (reader) {
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

    final task = _scheduler.read(helloWorldClientAddress, (reader) {
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

    final task = _scheduler.read(helloWorldClientAddress, (reader) {
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
    _scheduler.write(helloWorldServerAddress, (writer) {
      writer.clear();
      writer.writeInt8(kStatusReceivedData);
    });
  }

  Future<String> helloWorld() {
    writeHelloWorld();
    final completer = Completer<String>();

    late TechPawsRuntimeChannelReadTask task;
    task = _scheduler.read(helloWorldClientAddress, (reader) {
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

    final task = _scheduler.read(sayHelloClientAddress, (reader) {
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

    final task = _scheduler.read(sayHelloClientAddress, (reader) {
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

    _scheduler.write(sayHelloServerAddress, (writer) {
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
    task = _scheduler.read(sayHelloClientAddress, (reader) {
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

    final task = _scheduler.read(sumClientAddress, (reader) {
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

    _scheduler.write(sumServerAddress, (writer) {
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
    task = _scheduler.read(sumClientAddress, (reader) {
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

  Stream<void> readTrigger() {
    if (_readTriggerStream != null) {
      return _readTriggerStream!.stream;
    }

    _readTriggerStream = StreamController<void>.broadcast();

    final task = _scheduler.read(triggerClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readTriggerStream!.add(null);
      }
    });

    _readTriggerTasks.add(task);
    return _readTriggerStream!.stream;
  }


  Stream<Theme> readTheme() {
    if (_readThemeStream != null) {
      return _readThemeStream!.stream;
    }

    _readThemeStream = StreamController<Theme>.broadcast();

    final task = _scheduler.read(themeClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        _readThemeStream!.add(const ThemeIntoBuffers().read(reader));
      }
    });

    _readThemeTasks.add(task);
    return _readThemeStream!.stream;
  }

  Stream<Theme> readThemeEmplace(Theme model) {
    if (_readThemeStream != null) {
      return _readThemeStream!.stream;
    }

    _readThemeStream = StreamController<Theme>.broadcast();

    final task = _scheduler.read(themeClientAddress, (reader) {
      reader.reset();
      final status = reader.readInt8();

      if (status == kStatusReceivedData) {
        const ThemeEmplaceToBuffers().read(reader, model);
        _readThemeStream!.add(model);
      }
    });

    _readThemeTasks.add(task);
    return _readThemeStream!.stream;
  }

}
