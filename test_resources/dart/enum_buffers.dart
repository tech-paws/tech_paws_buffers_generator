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

class MyEnumIntoBuffers implements IntoBuffers<MyEnum> {
  const MyEnumIntoBuffers();

  @override
  MyEnum read(BytesReader reader) {
    final value = reader.readInt32();

    switch (value) {
      case 1:
        final model = MyEnum();
        model.value = MyEnumValue.idle;
        const MyEnumIdleEmplaceToBuffers().read(reader, model.idle);
        return model;

      case 2:
        final model = MyEnum();
        model.value = MyEnumValue.move;
        const MyEnumMoveEmplaceToBuffers().read(reader, model.move);
        return model;

      case 3:
        final model = MyEnum();
        model.value = MyEnumValue.update;
        const MyEnumUpdateEmplaceToBuffers().read(reader, model.update);
        return model;

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnum model) {
    switch (model.runtimeType) {
      case MyEnumIdle:
        writer.writeInt32(1);
        const MyEnumIdleEmplaceToBuffers().write(writer, model.idle);
        return;

      case MyEnumMove:
        writer.writeInt32(2);
        const MyEnumMoveEmplaceToBuffers().write(writer, model.move);
        return;

      case MyEnumUpdate:
        writer.writeInt32(3);
        const MyEnumUpdateEmplaceToBuffers().write(writer, model.update);
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
          const MyEnumIdleEmplaceToBuffers().skip(reader, 1);
          break;

        case 2:
          const MyEnumMoveEmplaceToBuffers().skip(reader, 1);
          break;

        case 3:
          const MyEnumUpdateEmplaceToBuffers().skip(reader, 1);
          break;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}

class MyEnumEmplaceToBuffers implements EmplaceToBuffers<MyEnum> {
  const MyEnumEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnum model) {
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
  void write(BytesWriter writer, MyEnum model) {
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
          break;

        case 2:
          const MyEnumMoveEmplaceToBuffers().skip(reader, 1);
          break;

        case 3:
          const MyEnumUpdateEmplaceToBuffers().skip(reader, 1);
          break;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}

class MyEnumWithoutPositionsOption1EmplaceToBuffers implements EmplaceToBuffers<MyEnumWithoutPositionsOption1> {
  const MyEnumWithoutPositionsOption1EmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumWithoutPositionsOption1 model) {
    model.v0 = reader.readInt64();
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositionsOption1 model) {
    writer.writeInt64(model.v0);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readInt64();
    }
  }
}

class MyEnumWithoutPositionsOption2EmplaceToBuffers implements EmplaceToBuffers<MyEnumWithoutPositionsOption2> {
  const MyEnumWithoutPositionsOption2EmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumWithoutPositionsOption2 model) {
    const StringEmplaceToBuffers().read(reader, model.name);
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositionsOption2 model) {
    const StringEmplaceToBuffers().write(writer, model.name);
  }

  @override
  void skip(BytesReader reader, int count) {
    const StringEmplaceToBuffers().skip(reader, count);

    for (int i = 0; i < count; i += 1) {
    }
  }
}

class MyEnumWithoutPositionsOption3EmplaceToBuffers implements EmplaceToBuffers<MyEnumWithoutPositionsOption3> {
  const MyEnumWithoutPositionsOption3EmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumWithoutPositionsOption3 model) {
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositionsOption3 model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class MyEnumWithoutPositionsOption4EmplaceToBuffers implements EmplaceToBuffers<MyEnumWithoutPositionsOption4> {
  const MyEnumWithoutPositionsOption4EmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumWithoutPositionsOption4 model) {
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositionsOption4 model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class MyEnumWithoutPositionsIntoBuffers implements IntoBuffers<MyEnumWithoutPositions> {
  const MyEnumWithoutPositionsIntoBuffers();

  @override
  MyEnumWithoutPositions read(BytesReader reader) {
    final value = reader.readInt32();

    switch (value) {
      case 0:
        final model = MyEnumWithoutPositions();
        model.value = MyEnumWithoutPositionsValue.option1;
        const MyEnumWithoutPositionsOption1EmplaceToBuffers().read(reader, model.option1);
        return model;

      case 1:
        final model = MyEnumWithoutPositions();
        model.value = MyEnumWithoutPositionsValue.option2;
        const MyEnumWithoutPositionsOption2EmplaceToBuffers().read(reader, model.option2);
        return model;

      case 2:
        final model = MyEnumWithoutPositions();
        model.value = MyEnumWithoutPositionsValue.option3;
        const MyEnumWithoutPositionsOption3EmplaceToBuffers().read(reader, model.option3);
        return model;

      case 3:
        final model = MyEnumWithoutPositions();
        model.value = MyEnumWithoutPositionsValue.option4;
        const MyEnumWithoutPositionsOption4EmplaceToBuffers().read(reader, model.option4);
        return model;

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositions model) {
    switch (model.runtimeType) {
      case MyEnumWithoutPositionsOption1:
        writer.writeInt32(0);
        const MyEnumWithoutPositionsOption1EmplaceToBuffers().write(writer, model.option1);
        return;

      case MyEnumWithoutPositionsOption2:
        writer.writeInt32(1);
        const MyEnumWithoutPositionsOption2EmplaceToBuffers().write(writer, model.option2);
        return;

      case MyEnumWithoutPositionsOption3:
        writer.writeInt32(2);
        const MyEnumWithoutPositionsOption3EmplaceToBuffers().write(writer, model.option3);
        return;

      case MyEnumWithoutPositionsOption4:
        writer.writeInt32(3);
        const MyEnumWithoutPositionsOption4EmplaceToBuffers().write(writer, model.option4);
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
        case 0:
          const MyEnumWithoutPositionsOption1EmplaceToBuffers().skip(reader, 1);
          break;

        case 1:
          const MyEnumWithoutPositionsOption2EmplaceToBuffers().skip(reader, 1);
          break;

        case 2:
          const MyEnumWithoutPositionsOption3EmplaceToBuffers().skip(reader, 1);
          break;

        case 3:
          const MyEnumWithoutPositionsOption4EmplaceToBuffers().skip(reader, 1);
          break;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}

class MyEnumWithoutPositionsEmplaceToBuffers implements EmplaceToBuffers<MyEnumWithoutPositions> {
  const MyEnumWithoutPositionsEmplaceToBuffers();

  @override
  void read(BytesReader reader, MyEnumWithoutPositions model) {
    final value = reader.readInt32();

    switch (value) {
      case 0:
        model.value = MyEnumWithoutPositionsValue.option1;
        const MyEnumWithoutPositionsOption1EmplaceToBuffers().read(reader, model.option1);
        return;

      case 1:
        model.value = MyEnumWithoutPositionsValue.option2;
        const MyEnumWithoutPositionsOption2EmplaceToBuffers().read(reader, model.option2);
        return;

      case 2:
        model.value = MyEnumWithoutPositionsValue.option3;
        const MyEnumWithoutPositionsOption3EmplaceToBuffers().read(reader, model.option3);
        return;

      case 3:
        model.value = MyEnumWithoutPositionsValue.option4;
        const MyEnumWithoutPositionsOption4EmplaceToBuffers().read(reader, model.option4);
        return;

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnumWithoutPositions model) {
    switch (model.value) {
      case MyEnumWithoutPositionsValue.option1:
        writer.writeInt32(0);
        const MyEnumWithoutPositionsOption1EmplaceToBuffers().write(writer, model.option1);
        return;

      case MyEnumWithoutPositionsValue.option2:
        writer.writeInt32(1);
        const MyEnumWithoutPositionsOption2EmplaceToBuffers().write(writer, model.option2);
        return;

      case MyEnumWithoutPositionsValue.option3:
        writer.writeInt32(2);
        const MyEnumWithoutPositionsOption3EmplaceToBuffers().write(writer, model.option3);
        return;

      case MyEnumWithoutPositionsValue.option4:
        writer.writeInt32(3);
        const MyEnumWithoutPositionsOption4EmplaceToBuffers().write(writer, model.option4);
        return;
    }
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      final value = reader.readInt32();

      switch (value) {
        case 0:
          const MyEnumWithoutPositionsOption1EmplaceToBuffers().skip(reader, 1);
          break;

        case 1:
          const MyEnumWithoutPositionsOption2EmplaceToBuffers().skip(reader, 1);
          break;

        case 2:
          const MyEnumWithoutPositionsOption3EmplaceToBuffers().skip(reader, 1);
          break;

        case 3:
          const MyEnumWithoutPositionsOption4EmplaceToBuffers().skip(reader, 1);
          break;

        default:
          throw StateError('Unsupported enum value: $value');
      }
    }
  }
}
