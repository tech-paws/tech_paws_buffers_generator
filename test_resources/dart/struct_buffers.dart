class EmptyEmplaceToBuffers implements EmplaceToBuffers<Empty> {
  const EmptyEmplaceToBuffers();

  @override
  void read(BytesReader reader, Empty model) {
  }

  @override
  void write(BytesWriter writer, Empty model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class EmptyIntoBuffers implements IntoBuffers<Empty> {
  const EmptyIntoBuffers();

  @override
  Empty read(BytesReader reader) {

    return Empty(
    );
  }

  @override
  void write(BytesWriter writer, Empty model) {
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
    }
  }
}

class ViewDataEmplaceToBuffers implements EmplaceToBuffers<ViewData> {
  const ViewDataEmplaceToBuffers();

  @override
  void read(BytesReader reader, ViewData model) {
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
  void write(BytesWriter writer, ViewData model) {
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

class ViewDataIntoBuffers implements IntoBuffers<ViewData> {
  const ViewDataIntoBuffers();

  @override
  ViewData read(BytesReader reader) {
    final deltaTime = reader.readFloat();
    final viewWidth = reader.readFloat();
    final viewHeight = reader.readFloat();
    final touchStartX = reader.readFloat();
    final touchStartY = reader.readFloat();
    final lastTouchX = reader.readFloat();
    final lastTouchY = reader.readFloat();
    final touchX = reader.readFloat();
    final touchY = reader.readFloat();

    return ViewData(
      deltaTime: deltaTime,
      viewWidth: viewWidth,
      viewHeight: viewHeight,
      touchStartX: touchStartX,
      touchStartY: touchStartY,
      lastTouchX: lastTouchX,
      lastTouchY: lastTouchY,
      touchX: touchX,
      touchY: touchY,
    );
  }

  @override
  void write(BytesWriter writer, ViewData model) {
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

class TestEmplaceToBuffers implements EmplaceToBuffers<Test> {
  const TestEmplaceToBuffers();

  @override
  void read(BytesReader reader, Test model) {
    model.touchX = reader.readFloat();
    model.touchY = reader.readFloat();
    const TouchStatusEmplaceToBuffers().read(reader, model.touchStatus);
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
    const TouchStatusEmplaceToBuffers().write(writer, model.touchStatus);
  }

  @override
  void skip(BytesReader reader, int count) {
    const TouchStatusEmplaceToBuffers().skip(reader, count);

    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
    }
  }
}

class TestIntoBuffers implements IntoBuffers<Test> {
  const TestIntoBuffers();

  @override
  Test read(BytesReader reader) {
    final touchX = reader.readFloat();
    final touchY = reader.readFloat();
    final touchStatus = const TouchStatusIntoBuffers().read(reader);

    return Test(
      touchX: touchX,
      touchY: touchY,
      touchStatus: touchStatus,
    );
  }

  @override
  void write(BytesWriter writer, Test model) {
    writer.writeFloat(model.touchX);
    writer.writeFloat(model.touchY);
    const TouchStatusIntoBuffers().write(writer, model.touchStatus);
  }

  @override
  void skip(BytesReader reader, int count) {
    const TouchStatusIntoBuffers().skip(reader, count);

    for (int i = 0; i < count; i += 1) {
      reader.readFloat();
      reader.readFloat();
    }
  }
}

class GenericTypeEmplaceToBuffers implements EmplaceToBuffers<GenericType> {
  const GenericTypeEmplaceToBuffers();

  @override
  void read(BytesReader reader, GenericType model) {
    const ListEmplaceToBuffers<Test>().read(reader, model.items);
    const LinearTableEmplaceToBuffers<double, Test>().read(reader, model.table);
  }

  @override
  void write(BytesWriter writer, GenericType model) {
    const ListEmplaceToBuffers<Test>().write(writer, model.items);
    const LinearTableEmplaceToBuffers<double, Test>().write(writer, model.table);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      const ListEmplaceToBuffers<Test>().read(reader, count);
      const LinearTableEmplaceToBuffers<double, Test>().read(reader, count);
    }
  }
}

class GenericTypeIntoBuffers implements IntoBuffers<GenericType> {
  const GenericTypeIntoBuffers();

  @override
  GenericType read(BytesReader reader) {
    final items = const ListIntoBuffers<Test>().read(reader);
    final table = const LinearTableIntoBuffers<double, Test>().read(reader);

    return GenericType(
      items: items,
      table: table,
    );
  }

  @override
  void write(BytesWriter writer, GenericType model) {
    const ListIntoBuffers<Test>().write(writer, model.items);
    const LinearTableIntoBuffers<double, Test>().write(writer, model.table);
  }

  @override
  void skip(BytesReader reader, int count) {
    for (int i = 0; i < count; i += 1) {
      const ListIntoBuffers<Test>().read(reader, count);
      const LinearTableIntoBuffers<double, Test>().read(reader, count);
    }
  }
}
