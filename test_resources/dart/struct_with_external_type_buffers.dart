class TestEmplaceToBuffers implements EmplaceToBuffers<Test> {
  const TestEmplaceToBuffers()

  @override
  void read(BytesReader reader, Test model) {
    model.touchX = reader.readFloat();
    model.touchY = reader.readFloat();
    model.touchStatus = const TouchStatusIntoBuffers().read(reader);
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
    const TouchStatusIntoBuffers().write(writer, model.touchStatus)
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
      const TouchStatusIntoBuffers().read(reader);
    }
  }
}

class TestIntoToBuffers implements IntoToBuffers<Test> {
  const TestIntoToBuffers()

  @override
  Test read(BytesReader reader) {
    final touchX = reader.readFloat();
    final touchY = reader.readFloat();
    final touchStatus = const TouchStatusIntoBuffers().read(reader);

    return Test(
      touchX: touchX;
      touchY: touchY;
      touchStatus: touchStatus;
    );
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
    const TouchStatusIntoBuffers().write(writer, model.touchStatus)
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
      const TouchStatusIntoBuffers().read(reader);
    }
  }
}
