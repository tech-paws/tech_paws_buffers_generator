class TestEmplaceToBuffers implements EmplaceToBuffers<Test> {
  const TestEmplaceToBuffers()

  @override
  void read(BytesReader reader, Test model) {
    model.deltaTime = reader.readFloat();
    model.viewWidth = reader.readFloat();
    model.viewHeight = reader.readFloat();
    model.touchStartX = reader.readFloat();
    model.touchStartY = reader.readFloat();
    model.lastTouchX = reader.readFloat();
    model.lastTouchY = reader.readFloat();
    model.touchX = reader.readFloat();
    model.touchY = reader.readFloat();
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.deltaTime);
    writer.writeFloat(model.viewWidth);
    writer.writeFloat(model.viewHeight);
    writer.writeFloat(model.touchStartX);
    writer.writeFloat(model.touchStartY);
    writer.writeFloat(model.lastTouchX);
    writer.writeFloat(model.lastTouchY);
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
    }
  }
}

class TestIntoToBuffers implements IntoToBuffers<Test> {
  const TestIntoToBuffers()

  @override
  Test read(BytesReader reader) {
    final deltaTime = reader.readFloat();
    final viewWidth = reader.readFloat();
    final viewHeight = reader.readFloat();
    final touchStartX = reader.readFloat();
    final touchStartY = reader.readFloat();
    final lastTouchX = reader.readFloat();
    final lastTouchY = reader.readFloat();
    final touchX = reader.readFloat();
    final touchY = reader.readFloat();

    return Test(
      deltaTime: deltaTime;
      viewWidth: viewWidth;
      viewHeight: viewHeight;
      touchStartX: touchStartX;
      touchStartY: touchStartY;
      lastTouchX: lastTouchX;
      lastTouchY: lastTouchY;
      touchX: touchX;
      touchY: touchY;
    );
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.deltaTime);
    writer.writeFloat(model.viewWidth);
    writer.writeFloat(model.viewHeight);
    writer.writeFloat(model.touchStartX);
    writer.writeFloat(model.touchStartY);
    writer.writeFloat(model.lastTouchX);
    writer.writeFloat(model.lastTouchY);
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
      reader.readFloat();
    }
  }
}
