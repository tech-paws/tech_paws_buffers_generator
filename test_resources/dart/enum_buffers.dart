class MyEnumIdleIntoBuffers implements IntoBuffers<MyEnumIdle> {
  const MyEnumIdleIntoBuffers();

  @override
  MyEnumIdle read(BytesReader reader) {

    return MyEnumIdle(
    );
  }

  @override
  void write(BytesWriter writer, MyEnumIdle model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class MyEnumMoveIntoBuffers implements IntoBuffers<MyEnumMove> {
  const MyEnumMoveIntoBuffers();

  @override
  MyEnumMove read(BytesReader reader) {
    final x = reader.readDouble();
    final y = reader.readDouble();

    return MyEnumMove(
      x: x;
      y: y;
    );
  }

  @override
  void write(BytesWriter writer, MyEnumMove model) {
    writer.writeDouble(model.x);
    writer.writeDouble(model.y);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readDouble();
      reader.readDouble();
    }
  }
}

class MyEnumUpdateIntoBuffers implements IntoBuffers<MyEnumUpdate> {
  const MyEnumUpdateIntoBuffers();

  @override
  MyEnumUpdate read(BytesReader reader) {
    final v0 = reader.readDouble();
    final v1 = reader.readDouble();
    final v2 = const StringIntoBuffers().read(reader);

    return MyEnumUpdate(
      v0: v0;
      v1: v1;
      v2: v2;
    );
  }

  @override
  void write(BytesWriter writer, MyEnumUpdate model) {
    writer.writeDouble(model.v0);
    writer.writeDouble(model.v1);
    const StringIntoBuffers().write(writer, model.v2);
  }

  @override
  void skip(BytesReader reader, int count) {
    const StringIntoBuffers().skip(reader, count);

    for (int i = 0; i < count; i += 1) {
      reader.readDouble();
      reader.readDouble();
    }
  }
}

class MyEnumIntoBuffers implements IntoBuffers<MyEnum> {
  const MyEnumIntoBuffers();

  @override
  void read(BytesReader reader) {
    final value = reader.readInt32();

    switch (value) {
      case 1:
        return const MyEnumIdleIntoBuffers().read(reader);

      case 2:
        return const MyEnumMoveIntoBuffers().read(reader);

      case 3:
        return const MyEnumUpdateIntoBuffers().read(reader);

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnum model) {
    switch (model.runtimeType) {
      case MyEnumIdle:
        writer.writeInt32(1);
        const MyEnumIdleIntoBuffers().write(writer, model.idle);
        return;

      case MyEnumMove:
        writer.writeInt32(2);
        const MyEnumMoveIntoBuffers().write(writer, model.move);
        return;

      case MyEnumUpdate:
        writer.writeInt32(3);
        const MyEnumUpdateIntoBuffers().write(writer, model.update);
        return;

      default:
        throw StateError('Unsupported enum type: ${model.runtimeType}');
    }
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      final value = reader.readInt32();

      switch (value) {
        case 1:
          const MyEnumIdleIntoBuffers().skip(reader, 1);
          continue;

        case 2:
          const MyEnumMoveIntoBuffers().skip(reader, 1);
          continue;

        case 3:
          const MyEnumUpdateIntoBuffers().skip(reader, 1);
          continue;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}

class MyEnumIdleEmplaceToBuffers implements EmplaceToBuffers<MyEnumIdle> {
  const MyEnumIdleEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumIdle model) {
  }

  @override
  void write(BytesWriter writer, MyEnumIdle model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class MyEnumMoveEmplaceToBuffers implements EmplaceToBuffers<MyEnumMove> {
  const MyEnumMoveEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumMove model) {
    model.x = reader.readDouble();
    model.y = reader.readDouble();
  }

  @override
  void write(BytesWriter writer, MyEnumMove model) {
    writer.writeDouble(model.x);
    writer.writeDouble(model.y);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readDouble();
      reader.readDouble();
    }
  }
}

class MyEnumUpdateEmplaceToBuffers implements EmplaceToBuffers<MyEnumUpdate> {
  const MyEnumUpdateEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumUpdate model) {
    model.v0 = reader.readDouble();
    model.v1 = reader.readDouble();
    const StringEmplaceToBuffers().read(reader, model.v2);
  }

  @override
  void write(BytesWriter writer, MyEnumUpdate model) {
    writer.writeDouble(model.v0);
    writer.writeDouble(model.v1);
    const StringEmplaceToBuffers().write(writer, model.v2);
  }

  @override
  void skip(BytesReader reader, int count) {
    const StringEmplaceToBuffers().skip(reader, count);

    for (int i = 0; i < count; i += 1) {
      reader.readDouble();
      reader.readDouble();
    }
  }
}

class MyEnumEmplaceToBuffers implements EmplaceToBuffers<MyEnumUnion> {
  const MyEnumEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumUnion model) {
    final value = reader.readInt32();

    switch (value) {
      case 1:
        model.value = MyEnumValue.idle;
        const MyEnumIdleEmplaceToBuffers().read(reader, model.idle);
        return;

      case 2:
        model.value = MyEnumValue.move;
        const MyEnumMoveEmplaceToBuffers().read(reader, model.move);
        return;

      case 3:
        model.value = MyEnumValue.update;
        const MyEnumUpdateEmplaceToBuffers().read(reader, model.update);
        return;

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnumUnion model) {
    switch (model.value) {
      case MyEnumValue.idle:
        writer.writeInt32(1);
        const MyEnumIdleEmplaceToBuffers().write(writer, model.idle);
        return;

      case MyEnumValue.move:
        writer.writeInt32(2);
        const MyEnumMoveEmplaceToBuffers().write(writer, model.move);
        return;

      case MyEnumValue.update:
        writer.writeInt32(3);
        const MyEnumUpdateEmplaceToBuffers().write(writer, model.update);
        return;
    }
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      final value = reader.readInt32();

      switch (value) {
        case 1:
          const MyEnumIdleEmplaceToBuffers().skip(reader, 1);
          continue;

        case 2:
          const MyEnumMoveEmplaceToBuffers().skip(reader, 1);
          continue;

        case 3:
          const MyEnumUpdateEmplaceToBuffers().skip(reader, 1);
          continue;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}
