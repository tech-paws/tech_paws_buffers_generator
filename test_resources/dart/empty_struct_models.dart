class Empty {
  const Empty();
}

class EmptyBuffersFactory implements BuffersFactory<Empty> {
  const EmptyBuffersFactory();

  @override
  Empty createDefault() => const Empty();
}
