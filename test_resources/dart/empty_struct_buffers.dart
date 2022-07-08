class EmptyEmplaceToBuffers implements EmplaceToBuffers<Empty> {
  const EmptyEmplaceToBuffers();

  @override
  void read(BytesReader reader, Empty model) {}

  @override
  void write(BytesWriter writer, Empty model) {}

  @override
  void skip(BytesReader reader, int count) {}
}

class EmptyIntoToBuffers implements IntoToBuffers<Empty> {
  const EmptyIntoToBuffers();

  @override
  Empty read(BytesReader reader) {
    return Empty();
  }

  @override
  void write(BytesWriter writer, Empty model) {}

  @override
  void skip(BytesReader reader, int count) {}
}
