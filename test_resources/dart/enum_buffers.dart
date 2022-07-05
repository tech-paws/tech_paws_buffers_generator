class MyEnumEmplaceToBuffers implements EmplaceToBuffers<MyEnumUnion> {
  const MyEnumEmplaceToBuffers()

  @override
  void read(BytesReader reader, MyEnumUnion model) {
    final value = reader.readInt32();

    switch (value) {
      case 1:
        model.value = MyEnumValue.idle;
        return;

      case 2:
        model.value = MyEnumValue.move;
        model.move.x = reader.readDouble();
        model.move.y = reader.readDouble();

        return;

      case 3:
        model.value = MyEnumValue.update;
        model.update.v0 = reader.readDouble();
        model.update.v1 = reader.readDouble();
        const StringEmplaceToBuffers().read(reader, update.v2);

        return;

      default:
        throw StateError('Unsupported enum value: $value');
    }
  }

  @override
  void write(BytesWriter writer, MyEnumUnion model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      final value = reader.readInt32();
    }
  }
}

class MyEnumIntoToBuffers implements IntoToBuffers<MyEnum> {
  const MyEnumIntoToBuffers()

  @override
  MyEnum read(BytesReader reader) {

  }

  @override
  void write(BytesWriter writer, MyEnum model) {

  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {

    }
  }
}
